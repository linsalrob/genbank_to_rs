//! Internal data model for GenBank records and features
//!
//! Represents parsed GenBank data in a format suitable for multiple output formats

use std::ops::Range;

/// A GenBank record (sequence with annotations)
#[derive(Debug, Clone)]
pub struct Record {
    pub id: String,
    pub description: String,
    pub sequence: String,
    pub length: usize,
    pub topology: Option<String>,
    #[allow(dead_code)]
    pub molecule_type: Option<String>,
    pub organism: Option<String>,
    pub features: Vec<Feature>,
}

/// A feature annotation (CDS, gene, tRNA, etc.)
#[derive(Debug, Clone)]
pub struct Feature {
    pub feature_type: String,
    pub location: Location,
    pub qualifiers: Qualifiers,
}

/// Location information for a feature
#[derive(Debug, Clone)]
pub struct Location {
    pub strand: Strand,
    pub ranges: Vec<Range<usize>>,
    #[allow(dead_code)]
    pub is_join: bool,
}

/// Strand orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Strand {
    Forward,
    Reverse,
    Unknown,
}

impl Strand {
    pub fn to_int(self) -> i8 {
        match self {
            Strand::Forward => 1,
            Strand::Reverse => -1,
            Strand::Unknown => 0,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Strand::Forward => '+',
            Strand::Reverse => '-',
            Strand::Unknown => '.',
        }
    }
}

/// Feature qualifiers (key-value pairs from GenBank)
#[derive(Debug, Clone, Default)]
pub struct Qualifiers {
    pub protein_id: Option<String>,
    pub locus_tag: Option<String>,
    pub gene: Option<String>,
    pub product: Option<String>,
    pub translation: Option<String>,
    pub codon_start: Option<u8>,
    pub transl_table: Option<u8>,
    pub pseudo: bool,
    pub note: Option<String>,
    pub db_xref: Vec<String>,
    pub inference: Vec<String>,
    pub anticodon: Option<String>,
    pub other: std::collections::HashMap<String, Vec<String>>,
}

impl Location {
    /// Get the start position (1-based)
    pub fn start(&self) -> usize {
        self.ranges.first().map(|r| r.start).unwrap_or(0)
    }

    /// Get the end position (1-based, inclusive)
    pub fn end(&self) -> usize {
        self.ranges.last().map(|r| r.end).unwrap_or(0)
    }

    /// Extract nucleotide sequence from record
    pub fn extract_sequence(&self, record_seq: &str) -> String {
        let mut seq = String::new();
        for range in &self.ranges {
            // GenBank uses 1-based inclusive coordinates
            // We store 1-based, so adjust to 0-based for slicing
            let start = range.start.saturating_sub(1);
            let end = range.end;
            if end <= record_seq.len() {
                seq.push_str(&record_seq[start..end]);
            }
        }

        if self.strand == Strand::Reverse {
            reverse_complement(&seq)
        } else {
            seq
        }
    }
}

/// Reverse complement a DNA sequence
pub fn reverse_complement(seq: &str) -> String {
    seq.chars()
        .rev()
        .map(|c| match c.to_ascii_uppercase() {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            'N' => 'N',
            other => other,
        })
        .collect()
}

impl Feature {
    /// Get feature ID (protein_id > locus_tag > fallback)
    pub fn feature_id(&self, record_id: &str) -> String {
        if let Some(ref pid) = self.qualifiers.protein_id {
            return pid.clone();
        }
        if let Some(ref tag) = self.qualifiers.locus_tag {
            return tag.clone();
        }
        // Fallback: use db_xref or location
        if let Some(xref) = self.qualifiers.db_xref.first() {
            return xref.clone();
        }
        format!(
            "{}_{}_{}",
            record_id,
            self.location.start(),
            self.location.end()
        )
    }

    /// Check if this is a pseudogene
    pub fn is_pseudo(&self) -> bool {
        self.qualifiers.pseudo
    }

    /// Get product or default
    pub fn product(&self) -> String {
        self.qualifiers
            .product
            .clone()
            .unwrap_or_else(|| "hypothetical protein".to_string())
    }
}
