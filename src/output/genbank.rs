//! GenBank format output (for --separate without other options)
//!
//! Writes separate GenBank files per sequence

use crate::cli::Args;
use crate::model::Record;
use anyhow::Result;

/// Write separate GenBank files (placeholder - not fully implemented)
pub fn write_separate_genbank(records: &[&Record], _args: &Args) -> Result<()> {
    for record in records {
        let path = format!("{}.gbk", record.id);
        tracing::info!("Would write separate GenBank file: {}", path);
        // TODO: Implement GenBank writing if needed
        // For now, this is a placeholder as the Python version uses BioPython's SeqIO.write
    }
    Ok(())
}
