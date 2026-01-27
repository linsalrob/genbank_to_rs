//! GenBank to Rust CLI Application
//!
//! Converts NCBI GenBank format files to various output formats
//! Rust port of the Python genbank_to tool

mod cli;
mod model;
mod parse;
mod output;
mod logging;
mod util;

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    // Parse command line arguments
    let mut args = cli::Args::parse();
    
    // Validate and apply auto-enabled flags
    args.validate()?;
    
    // Initialize logging
    logging::init_logging(&args.log, args.debug)?;
    
    tracing::info!("Starting genbank_to version {}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Input file: {:?}", args.genbank);
    
    // Parse GenBank file
    let records = parse::parse_genbank_file(&args.genbank)?;
    tracing::info!("Parsed {} record(s)", records.len());
    
    // Check if any output format is requested
    if !args.has_output() && !args.separate {
        tracing::warn!("No output format specified");
        eprintln!("Warning: No output format specified. Use -h for help.");
        return Ok(());
    }
    
    // Filter by seqid if specified
    let filtered_records: Vec<&model::Record> = if args.seqid.is_empty() {
        records.iter().collect()
    } else {
        tracing::info!("Filtering by sequence IDs: {:?}", args.seqid);
        records
            .iter()
            .filter(|r| args.seqid.contains(&r.id))
            .collect()
    };
    
    tracing::info!("Processing {} record(s) for output", filtered_records.len());
    
    // Write outputs
    output::write_outputs(&filtered_records, &args)?;
    
    tracing::info!("Completed successfully");
    Ok(())
}
