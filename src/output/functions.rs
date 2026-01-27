//! Functions table format output
//!
//! Maps to Python's genbank_to_functions function
//! Outputs: seq_id, protein_id, product (tab-separated)

use crate::cli::Args;
use crate::model::Record;
use crate::util::MaybeGzWriter;
use anyhow::Result;
use std::io::Write;
use std::path::Path;

/// Write functions table
pub fn write_functions(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let mut writer = MaybeGzWriter::new(path, args.zip)?;

    for record in records {
        write_functions_for_record(&mut writer, record, args)?;
    }

    writer.flush()?;
    tracing::info!("Wrote functions table to {:?}", path);
    Ok(())
}

fn write_functions_for_record<W: Write>(
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

        let protein_id = feature.feature_id(&record.id);
        let product = feature.product();

        writeln!(writer, "{}\t{}\t{}", record.id, protein_id, product)?;
    }

    Ok(())
}
