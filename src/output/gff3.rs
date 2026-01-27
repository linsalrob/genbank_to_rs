//! GFF3 format output
//!
//! Maps to Python's genbank_to_gff function

use crate::cli::Args;
use crate::model::Record;
use crate::util::{url_encode_gff, MaybeGzWriter};
use anyhow::Result;
use std::io::Write;
use std::path::Path;

/// Write GFF3 format
pub fn write_gff3(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let mut writer = MaybeGzWriter::new(path, args.zip)?;

    // Write GFF3 header
    writeln!(writer, "##gff-version 3")?;

    for record in records {
        write_gff3_for_record(&mut writer, record, args)?;
    }

    // Write FASTA section
    writeln!(writer, "##FASTA")?;
    for record in records {
        writeln!(writer, ">{}", record.id)?;
        writeln!(writer, "{}", record.sequence)?;
    }

    writer.flush()?;
    tracing::info!("Wrote GFF3 format to {:?}", path);
    Ok(())
}

fn write_gff3_for_record<W: Write>(writer: &mut W, record: &Record, args: &Args) -> Result<()> {
    // Write sequence-region directive
    writeln!(
        writer,
        "##sequence-region {} 1 {}",
        record.id, record.length
    )?;

    for feature in &record.features {
        if !args.pseudo && feature.is_pseudo() {
            continue;
        }

        let start = feature.location.start();
        let end = feature.location.end();
        let strand = feature.location.strand.to_char();

        // Frame calculation for CDS
        let frame = if feature.feature_type == "CDS" {
            feature.qualifiers.codon_start.map(|c| c - 1).unwrap_or(0)
        } else {
            0
        };

        // Build attributes
        let mut attributes = Vec::new();

        if let Some(ref protein_id) = feature.qualifiers.protein_id {
            attributes.push(format!("protein_id={}", url_encode_gff(protein_id)));
        }
        if let Some(ref locus_tag) = feature.qualifiers.locus_tag {
            attributes.push(format!("locus_tag={}", url_encode_gff(locus_tag)));
        }
        if let Some(ref gene) = feature.qualifiers.gene {
            attributes.push(format!("gene={}", url_encode_gff(gene)));
        }
        if let Some(ref product) = feature.qualifiers.product {
            attributes.push(format!("product={}", url_encode_gff(product)));
        }
        if feature.qualifiers.codon_start.is_some() {
            attributes.push(format!(
                "codon_start={}",
                feature.qualifiers.codon_start.unwrap()
            ));
        }
        if feature.qualifiers.pseudo {
            attributes.push("pseudo=true".to_string());
        }
        if !feature.qualifiers.db_xref.is_empty() {
            let xrefs = feature
                .qualifiers
                .db_xref
                .iter()
                .map(|x| url_encode_gff(x))
                .collect::<Vec<_>>()
                .join(",");
            attributes.push(format!("db_xref={}", xrefs));
        }

        let attr_string = attributes.join(";");

        writeln!(
            writer,
            "{}\tfeature\t{}\t{}\t{}\t.\t{}\t{}\t{}",
            record.id,
            feature.feature_type,
            start,
            end,
            strand,
            frame,
            attr_string
        )?;
    }

    Ok(())
}
