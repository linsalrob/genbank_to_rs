//! Bakta JSON format output
//!
//! Maps to Python's genbank_to_json.py module
//! Produces JSON compatible with Bakta annotation tool

use crate::cli::Args;
use crate::model::{Feature, Record};
use crate::util::MaybeGzWriter;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Write Bakta JSON format
pub fn write_bakta_json(records: &[&Record], path: &Path, args: &Args) -> Result<()> {
    let json_data = build_bakta_json(records, args)?;
    let json_str = serde_json::to_string_pretty(&json_data)?;

    let mut writer = MaybeGzWriter::new(path, args.zip)?;
    writer.write_all(json_str.as_bytes())?;
    writer.flush()?;

    tracing::info!("Wrote Bakta JSON to {:?}", path);
    Ok(())
}

fn build_bakta_json(records: &[&Record], args: &Args) -> Result<BaktaJson> {
    let genome = build_genome_metadata(records, args)?;
    let stats = build_genome_stats(records);
    let features = build_features(records, args);
    let sequences = build_sequences(records);

    Ok(BaktaJson {
        genome,
        stats,
        features,
        sequences,
        version: VersionInfo {
            genbank_to: VERSION.to_string(),
        },
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct BaktaJson {
    genome: GenomeMetadata,
    stats: GenomeStats,
    features: Vec<FeatureInfo>,
    sequences: Vec<SequenceInfo>,
    version: VersionInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenomeMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    genus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    species: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strain: Option<String>,
    complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    gram: Option<String>,
    translation_table: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenomeStats {
    no_sequences: usize,
    size: usize,
    gc: f64,
    n_ratio: f64,
    n50: usize,
    coding_ratio: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeatureInfo {
    #[serde(rename = "type")]
    feature_type: String,
    contig: String,
    start: usize,
    stop: usize,
    strand: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    gene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    db_xrefs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aa_hexdigest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frame: Option<u8>,
    hypothetical: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rbs_motif: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truncated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SequenceInfo {
    id: String,
    description: String,
    sequence: String,
    length: usize,
    complete: bool,
    #[serde(rename = "type")]
    seq_type: String,
    topology: String,
    simple_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VersionInfo {
    genbank_to: String,
}

fn build_genome_metadata(records: &[&Record], args: &Args) -> Result<GenomeMetadata> {
    let mut genus = None;
    let mut species = None;
    let mut strain = None;

    // Parse organism from first record
    if let Some(record) = records.first() {
        if let Some(ref organism) = record.organism {
            let parts: Vec<&str> = organism.split_whitespace().collect();
            if !parts.is_empty() {
                genus = Some(parts[0].to_string());
            }
            if parts.len() > 1 {
                species = Some(parts[1].to_string());
            }
            if parts.len() > 2 {
                strain = Some(parts[2..].join(" "));
            }
        }
    }

    // Determine completeness
    let complete = records.iter().any(|r| {
        r.topology.as_deref() == Some("Topology::Circular")
            || r.description.to_lowercase().contains("complete")
    });

    // Gram stain
    let gram = args.gram.clone();
    // Could implement genus-based inference here if gram is None

    Ok(GenomeMetadata {
        genus,
        species,
        strain,
        complete,
        gram,
        translation_table: args.translation_table,
    })
}

fn build_genome_stats(records: &[&Record]) -> GenomeStats {
    let no_sequences = records.len();
    let size: usize = records.iter().map(|r| r.length).sum();

    // Calculate GC content
    let (gc_count, at_count) = records.iter().fold((0, 0), |(gc, at), r| {
        let seq = r.sequence.to_uppercase();
        let g = seq.matches('G').count();
        let c = seq.matches('C').count();
        let a = seq.matches('A').count();
        let t = seq.matches('T').count();
        (gc + g + c, at + a + t)
    });

    let gc = if gc_count + at_count > 0 {
        gc_count as f64 / (gc_count + at_count) as f64
    } else {
        0.0
    };

    // Calculate N ratio
    let n_count: usize = records
        .iter()
        .map(|r| r.sequence.to_uppercase().matches('N').count())
        .sum();
    let n_ratio = if size > 0 {
        n_count as f64 / size as f64
    } else {
        0.0
    };

    // N50 calculation (simplified)
    let n50 = if no_sequences > 0 {
        let mut lengths: Vec<usize> = records.iter().map(|r| r.length).collect();
        lengths.sort_by(|a, b| b.cmp(a));
        let half_size = size / 2;
        let mut cumulative = 0;
        for &len in &lengths {
            cumulative += len;
            if cumulative >= half_size {
                return GenomeStats {
                    no_sequences,
                    size,
                    gc,
                    n_ratio,
                    n50: len,
                    coding_ratio: calculate_coding_ratio(records),
                };
            }
        }
        lengths[0]
    } else {
        0
    };

    GenomeStats {
        no_sequences,
        size,
        gc,
        n_ratio,
        n50,
        coding_ratio: calculate_coding_ratio(records),
    }
}

fn calculate_coding_ratio(records: &[&Record]) -> f64 {
    let total_bases: usize = records.iter().map(|r| r.length).sum();
    if total_bases == 0 {
        return 0.0;
    }

    let mut coding_bases = 0;
    for record in records {
        let mut coverage = vec![false; record.length];
        for feature in &record.features {
            if feature.feature_type == "CDS" && !feature.is_pseudo() {
                for range in &feature.location.ranges {
                    for pos in range.start..=range.end.min(record.length) {
                        if pos > 0 && pos <= record.length {
                            coverage[pos - 1] = true;
                        }
                    }
                }
            }
        }
        coding_bases += coverage.iter().filter(|&&covered| covered).count();
    }

    coding_bases as f64 / total_bases as f64
}

fn build_features(records: &[&Record], args: &Args) -> Vec<FeatureInfo> {
    let mut features = Vec::new();

    for record in records {
        for feature in &record.features {
            if feature.feature_type == "CDS" {
                if !args.pseudo && feature.is_pseudo() {
                    continue;
                }

                let nt_seq = feature.location.extract_sequence(&record.sequence);
                let aa_seq = get_aa_sequence(feature, &record.sequence);

                let aa_hexdigest = aa_seq
                    .as_ref()
                    .map(|aa| format!("{:x}", md5::compute(aa.as_bytes())));

                let start_type = if nt_seq.len() >= 3 {
                    Some(nt_seq[0..3].to_uppercase())
                } else {
                    None
                };

                let truncated = detect_truncation(&nt_seq, &aa_seq, feature);

                let hypothetical = feature.product().to_lowercase().contains("hypothetical");

                features.push(FeatureInfo {
                    feature_type: "CDS".to_string(),
                    contig: record.id.clone(),
                    start: feature.location.start(),
                    stop: feature.location.end(),
                    strand: feature.location.strand.to_int(),
                    gene: feature.qualifiers.gene.clone(),
                    locus: feature.qualifiers.locus_tag.clone(),
                    product: feature.qualifiers.product.clone(),
                    db_xrefs: feature.qualifiers.db_xref.clone(),
                    nt: Some(nt_seq),
                    aa: aa_seq,
                    aa_hexdigest,
                    frame: feature.qualifiers.codon_start.map(|c| c - 1),
                    hypothetical,
                    start_type,
                    rbs_motif: Some("NA".to_string()),
                    truncated,
                });
            }
        }
    }

    features
}

fn get_aa_sequence(feature: &Feature, record_seq: &str) -> Option<String> {
    if let Some(ref translation) = feature.qualifiers.translation {
        return Some(translation.clone());
    }

    let nt_seq = feature.location.extract_sequence(record_seq);
    let table = feature.qualifiers.transl_table.unwrap_or(11);
    let frame = feature.qualifiers.codon_start.unwrap_or(1);

    crate::output::fasta::translate_sequence(&nt_seq, table, frame).ok()
}

fn detect_truncation(nt_seq: &str, aa_seq: &Option<String>, _feature: &Feature) -> Option<String> {
    if let Some(aa) = aa_seq {
        let starts_with_m = aa.starts_with('M');
        // Use modulo instead of is_multiple_of for compatibility with Rust < 1.87.0
        #[allow(clippy::manual_is_multiple_of)]
        let ends_with_stop = aa.ends_with('*') || nt_seq.len() % 3 == 0;

        match (starts_with_m, ends_with_stop) {
            (false, false) => Some("5-prime-3-prime".to_string()),
            (false, true) => Some("5-prime".to_string()),
            (true, false) => Some("3-prime".to_string()),
            (true, true) => None,
        }
    } else {
        None
    }
}

fn build_sequences(records: &[&Record]) -> Vec<SequenceInfo> {
    records
        .iter()
        .map(|record| {
            let simple_id = record
                .id
                .split('.')
                .next()
                .unwrap_or(&record.id)
                .to_string();

            let seq_type = if record.description.to_lowercase().contains("plasmid") {
                "plasmid"
            } else if record.description.to_lowercase().contains("chromosome") {
                "chromosome"
            } else if record.description.to_lowercase().contains("contig") {
                "contig"
            } else {
                "sequence"
            }
            .to_string();

            let topology = if record.topology.as_deref() == Some("Topology::Circular") {
                "circular"
            } else {
                "linear"
            }
            .to_string();

            let complete =
                topology == "circular" || record.description.to_lowercase().contains("complete");

            SequenceInfo {
                id: record.id.clone(),
                description: record.description.clone(),
                sequence: record.sequence.clone(),
                length: record.length,
                complete,
                seq_type,
                topology,
                simple_id,
            }
        })
        .collect()
}
