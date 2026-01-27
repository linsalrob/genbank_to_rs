//! PTT (Protein Table) format output
//!
//! Maps to Python's genbank_to_ptt function

use crate::cli::Args;
use crate::model::Record;
use crate::util::MaybeGzWriter;
use anyhow::Result;
use std::io::Write;
use std::path::Path;
use regex::Regex;

/// Write PTT format
pub fn write_ptt(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let mut writer = MaybeGzWriter::new(path, args.zip)?;
    
    for record in records {
        write_ptt_for_record(&mut writer, record, args)?;
    }
    
    writer.flush()?;
    tracing::info!("Wrote PTT format to {:?}", path);
    Ok(())
}

fn write_ptt_for_record<W: Write>(writer: &mut W, record: &Record, args: &Args) -> Result<()> {
    // PTT format columns (tab-separated):
    // Location | Strand | Length | GI | Gene | Protein_ID | COG | Product
    
    for feature in &record.features {
        if feature.feature_type != "CDS" {
            continue;
        }
        
        if !args.pseudo && feature.is_pseudo() {
            continue;
        }

        let start = feature.location.start();
        let end = feature.location.end();
        let location = format!("{}..{}", start, end);
        
        let strand = feature.location.strand.to_char();
        
        // Length calculation: (end - start) / 3 - 1 (protein length)
        let length = ((end - start) as f64 / 3.0 - 1.0).max(0.0);
        
        // Extract GI from db_xref
        let gi = extract_gi(&feature.qualifiers.db_xref);
        
        // Gene or location as fallback
        let gene = feature.qualifiers.gene
            .clone()
            .unwrap_or_else(|| format!("{}..{}", start, end));
        
        // Protein ID
        let protein_id = feature.feature_id(&record.id);
        
        // Extract COG from product
        let cog = extract_cog(feature.qualifiers.product.as_deref());
        
        // Product
        let product = feature.product();

        writeln!(
            writer,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            location, strand, length, gi, gene, protein_id, cog, product
        )?;
    }
    
    Ok(())
}

/// Extract GI number from db_xref list
fn extract_gi(db_xrefs: &[String]) -> String {
    let gi_regex = Regex::new(r"GI:(\d+)").unwrap();
    
    for xref in db_xrefs {
        if let Some(caps) = gi_regex.captures(xref) {
            if let Some(gi) = caps.get(1) {
                return gi.as_str().to_string();
            }
        }
    }
    
    "-".to_string()
}

/// Extract COG identifier from product string
fn extract_cog(product: Option<&str>) -> String {
    if let Some(prod) = product {
        let cog_regex = Regex::new(r"(COG\S+)").unwrap();
        if let Some(caps) = cog_regex.captures(prod) {
            if let Some(cog) = caps.get(1) {
                return cog.as_str().to_string();
            }
        }
    }
    "-".to_string()
}
