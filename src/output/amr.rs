//! AMRFinderPlus format output
//!
//! Maps to Python's genbank_to_amrfinder function
//! Creates three files: .gff, .faa, .fna

use crate::cli::Args;
use crate::model::Record;
use crate::util::{url_encode_gff, MaybeGzWriter};
use anyhow::Result;
use std::io::Write;
use std::path::Path;

/// Write AMR format (creates .gff, .faa, .fna files)
pub fn write_amr(records: &[&Record], base_path: &Path, args: &Args) -> Result<()> {
    let stem = base_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let parent = base_path.parent().unwrap_or_else(|| Path::new("."));

    let gff_path = parent.join(format!("{}.gff", stem));
    let faa_path = parent.join(format!("{}.faa", stem));
    let fna_path = parent.join(format!("{}.fna", stem));

    write_amr_gff(records, &gff_path, args)?;
    write_amr_faa(records, &faa_path, args)?;
    write_amr_fna(records, &fna_path, args)?;

    Ok(())
}

fn write_amr_gff(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let mut writer = MaybeGzWriter::new(path, args.zip)?;

    writeln!(writer, "##gff-version 3")?;

    for record in records {
        writeln!(
            writer,
            "##sequence-region {} 1 {}",
            record.id, record.length
        )?;

        for feature in &record.features {
            if feature.feature_type != "CDS" {
                continue;
            }

            if !args.pseudo && feature.is_pseudo() {
                continue;
            }

            let start = feature.location.start();
            let end = feature.location.end();
            let strand = feature.location.strand.to_char();
            let frame = feature.qualifiers.codon_start.map(|c| c - 1).unwrap_or(0);

            let mut attributes = Vec::new();

            if let Some(ref protein_id) = feature.qualifiers.protein_id {
                attributes.push(format!("protein_id={}", url_encode_gff(protein_id)));
                // Critical: Add Name field for AMRFinderPlus
                attributes.push(format!("Name={}", url_encode_gff(protein_id)));
            }
            if let Some(ref locus_tag) = feature.qualifiers.locus_tag {
                attributes.push(format!("locus_tag={}", url_encode_gff(locus_tag)));
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

            writeln!(
                writer,
                "{}\tfeature\t{}\t{}\t{}\t.\t{}\t{}\t{}",
                record.id,
                feature.feature_type,
                start,
                end,
                strand,
                frame,
                attributes.join(";")
            )?;
        }
    }

    writer.flush()?;
    tracing::info!("Wrote AMR GFF to {:?}", path);
    Ok(())
}

fn write_amr_faa(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    // Reuse FASTA amino acid writer
    crate::output::fasta::write_aminoacids(records, path, args)
}

fn write_amr_fna(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    // Reuse FASTA ORF writer
    crate::output::fasta::write_orfs(records, path, args)
}
