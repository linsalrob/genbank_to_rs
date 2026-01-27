//! Phage Finder format output
//!
//! Maps to Python's genbank_to_phage_finder function
//! Format: contig_id, contig_length, gene_id, start, end, function (tab-separated)

use crate::cli::Args;
use crate::model::Record;
use crate::util::MaybeGzWriter;
use anyhow::Result;
use std::io::Write;
use std::path::Path;

/// Write phage_finder format
pub fn write_phage_finder(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let mut writer = MaybeGzWriter::new(path, args.zip)?;

    for record in records {
        write_phage_finder_for_record(&mut writer, record, args)?;
    }

    writer.flush()?;
    tracing::info!("Wrote phage_finder format to {:?}", path);
    Ok(())
}

fn write_phage_finder_for_record<W: Write>(
    writer: &mut W,
    record: &Record,
    args: &Args,
) -> Result<()> {
    for feature in &record.features {
        if feature.feature_type != "CDS" {
            continue;
        }

        if !args.pseudo && feature.is_pseudo() {
            continue;
        }

        let gene_id = feature.feature_id(&record.id);
        let start = feature.location.start() - 1; // Convert to 0-based for phage_finder
        let end = feature.location.end();
        let function = feature.product();

        writeln!(
            writer,
            "{}\t{}\t{}\t{}\t{}\t{}",
            record.id, record.length, gene_id, start, end, function
        )?;
    }

    Ok(())
}
