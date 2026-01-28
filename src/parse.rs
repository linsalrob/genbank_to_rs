//! GenBank file parsing
//!
//! Parses GenBank format files into our internal Record model
//! Uses gb-io library with extensions for full qualifier support

use crate::model::*;
use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use gb_io::reader::SeqReader;
use gb_io::seq::{Feature as GbFeature, Seq};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Parse GenBank file into records
/// Automatically detects and decompresses gzip files based on magic number
pub fn parse_genbank_file(path: &Path) -> Result<Vec<Record>> {
    let mut file =
        File::open(path).with_context(|| format!("Failed to open GenBank file: {:?}", path))?;

    // Check for gzip magic number (0x1f, 0x8b)
    let mut magic = [0u8; 2];
    file.read_exact(&mut magic)
        .with_context(|| format!("Failed to read file header: {:?}", path))?;

    // Reopen file since we consumed bytes
    let file =
        File::open(path).with_context(|| format!("Failed to reopen GenBank file: {:?}", path))?;

    let mut records = Vec::new();

    if magic == [0x1f, 0x8b] {
        // File is gzip compressed
        tracing::info!("Detected gzip compressed file: {:?}", path);
        let decoder = GzDecoder::new(BufReader::new(file));
        let reader = SeqReader::new(decoder);

        for seq_result in reader {
            let seq = seq_result.context("Failed to parse GenBank record")?;
            let record = convert_seq_to_record(seq)?;
            records.push(record);
        }
    } else {
        // Regular uncompressed file
        tracing::info!("Reading uncompressed file: {:?}", path);
        let reader = SeqReader::new(BufReader::new(file));

        for seq_result in reader {
            let seq = seq_result.context("Failed to parse GenBank record")?;
            let record = convert_seq_to_record(seq)?;
            records.push(record);
        }
    }

    tracing::info!("Parsed {} record(s) from {:?}", records.len(), path);
    Ok(records)
}

/// Convert gb-io Seq to our Record model
fn convert_seq_to_record(seq: Seq) -> Result<Record> {
    let id = seq.name.clone().unwrap_or_else(|| "unknown".to_string());
    let description = seq.definition.clone().unwrap_or_default();

    // Extract sequence (convert Vec<u8> to String)
    let sequence = String::from_utf8_lossy(&seq.seq).to_string();
    let length = sequence.len();

    // Extract topology and molecule type
    let topology = Some(format!("{:?}", seq.topology));
    let molecule_type = seq.molecule_type.clone();

    // Parse organism from source
    let organism = extract_organism(&seq);

    // Convert features
    let mut features = Vec::new();
    for gb_feat in seq.features {
        if let Some(feature) = convert_feature(gb_feat)? {
            features.push(feature);
        }
    }

    Ok(Record {
        id,
        description,
        sequence,
        length,
        topology,
        molecule_type,
        organism,
        features,
    })
}

/// Extract organism from GenBank seq
fn extract_organism(seq: &Seq) -> Option<String> {
    // Try to find organism in features
    for feat in &seq.features {
        if feat.kind.as_ref() == "source" {
            for (key, value) in &feat.qualifiers {
                if key.as_ref() == "organism" {
                    if let Some(val) = value {
                        return Some(val.clone());
                    }
                }
            }
        }
    }
    None
}

/// Convert gb-io Feature to our Feature model
fn convert_feature(gb_feat: GbFeature) -> Result<Option<Feature>> {
    let feature_type = gb_feat.kind.as_ref().to_string();

    // Parse location
    let location = parse_gb_location(&gb_feat.location)?;

    // Parse qualifiers
    let qualifiers = parse_qualifiers(&gb_feat.qualifiers);

    Ok(Some(Feature {
        feature_type,
        location,
        qualifiers,
    }))
}

/// Parse gb-io Location into our Location struct
fn parse_gb_location(loc: &gb_io::seq::Location) -> Result<Location> {
    use gb_io::seq::Location as GbLoc;

    let mut strand = Strand::Forward;
    let mut ranges = Vec::new();
    let mut is_join = false;

    match loc {
        GbLoc::Range((start, _), (end, _)) => {
            ranges.push((*start as usize + 1)..(*end as usize));
        }
        GbLoc::Complement(inner) => {
            strand = Strand::Reverse;
            match inner.as_ref() {
                GbLoc::Range((start, _), (end, _)) => {
                    ranges.push((*start as usize + 1)..(*end as usize));
                }
                GbLoc::Join(parts) => {
                    is_join = true;
                    for part in parts {
                        if let GbLoc::Range((start, _), (end, _)) = part {
                            ranges.push((*start as usize + 1)..(*end as usize));
                        }
                    }
                }
                _ => {}
            }
        }
        GbLoc::Join(parts) => {
            is_join = true;
            for part in parts {
                match part {
                    GbLoc::Range((start, _), (end, _)) => {
                        ranges.push((*start as usize + 1)..(*end as usize));
                    }
                    GbLoc::Complement(inner) => {
                        strand = Strand::Reverse;
                        if let GbLoc::Range((start, _), (end, _)) = inner.as_ref() {
                            ranges.push((*start as usize + 1)..(*end as usize));
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {
            // For other location types, use a default range
            ranges.push(1..1);
        }
    }

    Ok(Location {
        strand,
        ranges,
        is_join,
    })
}

/// Parse qualifiers from gb-io format
fn parse_qualifiers(quals: &[(gb_io::seq::QualifierKey, Option<String>)]) -> Qualifiers {
    let mut qualifiers = Qualifiers::default();

    for (key, value) in quals {
        let key_str = key.as_ref();
        let value_opt = value.clone();

        match key_str {
            "protein_id" => qualifiers.protein_id = value_opt,
            "locus_tag" => qualifiers.locus_tag = value_opt,
            "gene" => qualifiers.gene = value_opt,
            "product" => qualifiers.product = value_opt,
            "translation" => qualifiers.translation = value_opt,
            "codon_start" => {
                qualifiers.codon_start = value_opt.and_then(|v| v.parse().ok());
            }
            "transl_table" => {
                qualifiers.transl_table = value_opt.and_then(|v| v.parse().ok());
            }
            "pseudo" => qualifiers.pseudo = true,
            "note" => qualifiers.note = value_opt,
            "db_xref" => {
                if let Some(val) = value_opt {
                    qualifiers.db_xref.push(val);
                }
            }
            "inference" => {
                if let Some(val) = value_opt {
                    qualifiers.inference.push(val);
                }
            }
            "anticodon" => qualifiers.anticodon = value_opt,
            _ => {
                if let Some(val) = value_opt {
                    qualifiers
                        .other
                        .entry(key_str.to_string())
                        .or_insert_with(Vec::new)
                        .push(val);
                }
            }
        }
    }

    qualifiers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_qualifiers() {
        use gb_io::seq::QualifierKey;

        let quals = vec![
            (
                QualifierKey::from("protein_id"),
                Some("YP_001234".to_string()),
            ),
            (QualifierKey::from("locus_tag"), Some("ABC_001".to_string())),
        ];

        let parsed = parse_qualifiers(&quals);
        assert_eq!(parsed.protein_id, Some("YP_001234".to_string()));
        assert_eq!(parsed.locus_tag, Some("ABC_001".to_string()));
    }

    #[test]
    fn test_parse_uncompressed_genbank() {
        let path = Path::new("tests/data/input/NC_001417.gbk");
        if path.exists() {
            let records = parse_genbank_file(path).unwrap();
            assert!(!records.is_empty());
            assert_eq!(records[0].id, "NC_001417");
        }
    }

    #[test]
    fn test_parse_gzip_genbank() {
        let path = Path::new("tests/data/input/NC_001417.gbk.gz");
        if path.exists() {
            let records = parse_genbank_file(path).unwrap();
            assert!(!records.is_empty());
            assert_eq!(records[0].id, "NC_001417");
        }
    }

    #[test]
    fn test_gzip_and_uncompressed_produce_same_result() {
        let uncompressed_path = Path::new("tests/data/input/NC_001417.gbk");
        let gzip_path = Path::new("tests/data/input/NC_001417.gbk.gz");

        if uncompressed_path.exists() && gzip_path.exists() {
            let uncompressed_records = parse_genbank_file(uncompressed_path).unwrap();
            let gzip_records = parse_genbank_file(gzip_path).unwrap();

            assert_eq!(uncompressed_records.len(), gzip_records.len());
            assert_eq!(uncompressed_records[0].id, gzip_records[0].id);
            assert_eq!(
                uncompressed_records[0].sequence,
                gzip_records[0].sequence
            );
            assert_eq!(
                uncompressed_records[0].features.len(),
                gzip_records[0].features.len()
            );
        }
    }
}
