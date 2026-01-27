//! FASTA format output (nucleotide, amino acids, ORFs)
//!
//! Maps to Python's genbank_to_fna, genbank_to_faa, genbank_to_orfs

use crate::cli::Args;
use crate::model::{Feature, Record, Strand};
use crate::util::{write_fasta, MaybeGzWriter};
use anyhow::Result;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Write nucleotide sequences
pub fn write_nucleotide(records: &[&Record], base_path: &Path, args: &Args) -> Result<()> {
    if args.separate {
        write_nucleotide_separate(records, base_path, args)
    } else {
        write_nucleotide_single(records, base_path, args)
    }
}

fn write_nucleotide_single(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let mut writer = MaybeGzWriter::new(path, args.zip)?;

    for record in records {
        let header = if record.description.is_empty() {
            record.id.clone()
        } else {
            format!("{} {}", record.id, record.description)
        };
        write_fasta(&mut writer, &header, &record.sequence)?;
    }

    writer.flush()?;
    tracing::info!("Wrote nucleotide sequences to {:?}", path);
    Ok(())
}

fn write_nucleotide_separate(records: &[&Record], base_path: &Path, args: &Args) -> Result<()> {
    for record in records {
        let path = build_separate_path(base_path, &record.id, "fna");
        let mut writer = MaybeGzWriter::new(&path, args.zip)?;

        let header = if record.description.is_empty() {
            record.id.clone()
        } else {
            format!("{} {}", record.id, record.description)
        };
        write_fasta(&mut writer, &header, &record.sequence)?;
        writer.flush()?;
        tracing::info!("Wrote nucleotide sequence to {:?}", path);
    }
    Ok(())
}

/// Write amino acid sequences
pub fn write_aminoacids(records: &[&Record], base_path: &Path, args: &Args) -> Result<()> {
    if args.separate {
        write_aminoacids_separate(records, base_path, args)
    } else {
        write_aminoacids_single(records, base_path, args)
    }
}

fn write_aminoacids_single(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let mut writer = MaybeGzWriter::new(path, args.zip)?;

    for record in records {
        for feature in &record.features {
            if feature.feature_type != "CDS" {
                continue;
            }

            // Skip pseudogenes unless --pseudo
            if !args.pseudo && feature.is_pseudo() {
                continue;
            }

            let aa_seq = get_amino_acid_sequence(feature, &record.sequence)?;
            let header = build_fasta_header(feature, record, args.complex);
            write_fasta(&mut writer, &header, &aa_seq)?;
        }
    }

    writer.flush()?;
    tracing::info!("Wrote amino acid sequences to {:?}", path);
    Ok(())
}

fn write_aminoacids_separate(records: &[&Record], base_path: &Path, args: &Args) -> Result<()> {
    for record in records {
        let path = build_separate_path(base_path, &record.id, "faa");
        let mut writer = MaybeGzWriter::new(&path, args.zip)?;

        for feature in &record.features {
            if feature.feature_type != "CDS" {
                continue;
            }

            if !args.pseudo && feature.is_pseudo() {
                continue;
            }

            let aa_seq = get_amino_acid_sequence(feature, &record.sequence)?;
            let header = build_fasta_header(feature, record, args.complex);
            write_fasta(&mut writer, &header, &aa_seq)?;
        }

        writer.flush()?;
        tracing::info!("Wrote amino acid sequences to {:?}", path);
    }
    Ok(())
}

/// Write ORF nucleotide sequences
pub fn write_orfs(records: &[&Record], base_path: &Path, args: &Args) -> Result<()> {
    if args.separate {
        write_orfs_separate(records, base_path, args)
    } else {
        write_orfs_single(records, base_path, args)
    }
}

fn write_orfs_single(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let mut writer = MaybeGzWriter::new(path, args.zip)?;

    for record in records {
        for feature in &record.features {
            if feature.feature_type != "CDS" {
                continue;
            }

            if !args.pseudo && feature.is_pseudo() {
                continue;
            }

            let orf_seq = feature.location.extract_sequence(&record.sequence);
            let header = build_fasta_header(feature, record, args.complex);
            write_fasta(&mut writer, &header, &orf_seq)?;
        }
    }

    writer.flush()?;
    tracing::info!("Wrote ORF sequences to {:?}", path);
    Ok(())
}

fn write_orfs_separate(records: &[&Record], base_path: &Path, args: &Args) -> Result<()> {
    for record in records {
        let path = build_separate_path(base_path, &record.id, "orfs");
        let mut writer = MaybeGzWriter::new(&path, args.zip)?;

        for feature in &record.features {
            if feature.feature_type != "CDS" {
                continue;
            }

            if !args.pseudo && feature.is_pseudo() {
                continue;
            }

            let orf_seq = feature.location.extract_sequence(&record.sequence);
            let header = build_fasta_header(feature, record, args.complex);
            write_fasta(&mut writer, &header, &orf_seq)?;
        }

        writer.flush()?;
        tracing::info!("Wrote ORF sequences to {:?}", path);
    }
    Ok(())
}

/// Build FASTA header (simple or complex)
fn build_fasta_header(feature: &Feature, record: &Record, complex: bool) -> String {
    let feature_id = feature.feature_id(&record.id);

    if !complex {
        return feature_id;
    }

    // Complex header format:
    // protein_id [seq_id] [organism] [seq_id_start_stop] product [db_xrefs]
    let organism = record.organism.as_deref().unwrap_or("");

    let start = feature.location.start();
    let end = feature.location.end();
    let location_str = if feature.location.strand == Strand::Forward {
        format!("{}_{}", start, end)
    } else {
        format!("{}_{}", end, start)
    };

    let product = feature.product();

    let db_xrefs = if feature.qualifiers.db_xref.is_empty() {
        String::new()
    } else {
        format!(" [{}]", feature.qualifiers.db_xref.join("; "))
    };

    let location_id = format!("{}_{}", record.id, location_str);

    format!(
        "{} [{}] [{}] [{}] {}{}",
        feature_id, record.id, organism, location_id, product, db_xrefs
    )
}

/// Get amino acid sequence from feature
fn get_amino_acid_sequence(feature: &Feature, record_seq: &str) -> Result<String> {
    // Use translation if provided
    if let Some(ref translation) = feature.qualifiers.translation {
        return Ok(translation.clone());
    }

    // Otherwise translate nucleotide sequence
    let nt_seq = feature.location.extract_sequence(record_seq);
    let table = feature.qualifiers.transl_table.unwrap_or(11);
    let frame = feature.qualifiers.codon_start.unwrap_or(1);

    translate_sequence(&nt_seq, table, frame)
}

/// Translate nucleotide sequence to amino acids
pub fn translate_sequence(nt_seq: &str, table: u8, codon_start: u8) -> Result<String> {
    // For now, use standard genetic code (table 11)
    // Skip to codon_start position (1-based)
    let start_pos = (codon_start - 1) as usize;
    let seq = &nt_seq[start_pos..];

    let mut aa_seq = String::new();
    let codons: Vec<&str> = seq
        .as_bytes()
        .chunks(3)
        .filter_map(|chunk| {
            if chunk.len() == 3 {
                std::str::from_utf8(chunk).ok()
            } else {
                None
            }
        })
        .collect();

    for codon in codons {
        let aa = translate_codon(codon, table);
        if aa == '*' {
            break; // Stop at stop codon
        }
        aa_seq.push(aa);
    }

    Ok(aa_seq)
}

/// Translate a single codon (standard genetic code table 11)
fn translate_codon(codon: &str, _table: u8) -> char {
    // Standard genetic code (table 11 - bacterial)
    match codon.to_uppercase().as_str() {
        "TTT" | "TTC" => 'F',
        "TTA" | "TTG" | "CTT" | "CTC" | "CTA" | "CTG" => 'L',
        "ATT" | "ATC" | "ATA" => 'I',
        "ATG" => 'M',
        "GTT" | "GTC" | "GTA" | "GTG" => 'V',
        "TCT" | "TCC" | "TCA" | "TCG" | "AGT" | "AGC" => 'S',
        "CCT" | "CCC" | "CCA" | "CCG" => 'P',
        "ACT" | "ACC" | "ACA" | "ACG" => 'T',
        "GCT" | "GCC" | "GCA" | "GCG" => 'A',
        "TAT" | "TAC" => 'Y',
        "TAA" | "TAG" | "TGA" => '*', // Stop codons
        "CAT" | "CAC" => 'H',
        "CAA" | "CAG" => 'Q',
        "AAT" | "AAC" => 'N',
        "AAA" | "AAG" => 'K',
        "GAT" | "GAC" => 'D',
        "GAA" | "GAG" => 'E',
        "TGT" | "TGC" => 'C',
        "TGG" => 'W',
        "CGT" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => 'R',
        "GGT" | "GGC" | "GGA" | "GGG" => 'G',
        _ => 'X', // Unknown
    }
}

/// Build path for separate file output
fn build_separate_path(base_path: &Path, seq_id: &str, extension: &str) -> PathBuf {
    let parent = base_path.parent().unwrap_or_else(|| Path::new("."));
    let stem = base_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    parent.join(format!("{}.{}.{}", stem, seq_id, extension))
}
