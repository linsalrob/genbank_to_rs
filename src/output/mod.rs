//! Output format dispatching
//!
//! Coordinates writing records in various formats

pub mod amr;
pub mod bakta_json;
pub mod fasta;
pub mod functions;
pub mod genbank;
pub mod gff3;
pub mod phage_finder;
pub mod ptt;

use crate::cli::Args;
use crate::model::Record;
use anyhow::Result;

/// Write records in all requested output formats
pub fn write_outputs(records: &[&Record], args: &Args) -> Result<()> {
    tracing::info!("Writing outputs for {} record(s)", records.len());

    // Write nucleotide FASTA
    if let Some(ref path) = args.nucleotide {
        fasta::write_nucleotide(records, path, args)?;
    }

    // Write amino acid FASTA
    if let Some(ref path) = args.aminoacids {
        fasta::write_aminoacids(records, path, args)?;
    }

    // Write ORFs
    if let Some(ref path) = args.orfs {
        fasta::write_orfs(records, path, args)?;
    }

    // Write PTT
    if let Some(ref path) = args.ptt {
        ptt::write_ptt(records, path, args)?;
    }

    // Write functions
    if let Some(ref path) = args.functions {
        functions::write_functions(records, path, args)?;
    }

    // Write GFF3
    if let Some(ref path) = args.gff3 {
        gff3::write_gff3(records, path, args)?;
    }

    // Write AMR
    if let Some(ref path) = args.amr {
        amr::write_amr(records, path, args)?;
    }

    // Write phage_finder
    if let Some(ref path) = args.phage_finder {
        phage_finder::write_phage_finder(records, path, args)?;
    }

    // Write Bakta JSON
    if let Some(ref path) = args.bakta_json {
        bakta_json::write_bakta_json(records, path, args)?;
    }

    // Write separate GenBank files if --separate with no other output
    if args.separate && !args.has_output() {
        genbank::write_separate_genbank(records, args)?;
    }

    Ok(())
}
