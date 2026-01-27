//! CLI argument parsing and validation
//!
//! Maps to Python's argparse configuration in GenBankToLib/main.py

use clap::Parser;
use std::path::PathBuf;

/// Convert NCBI GenBank format files to various output formats
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input GenBank file
    #[arg(short = 'g', long = "genbank", required = true)]
    pub genbank: PathBuf,

    /// Complex identifier line (for FASTA headers)
    #[arg(short = 'c', long = "complex")]
    pub complex: bool,

    /// Output amino acid sequences (FASTA)
    #[arg(short = 'a', long = "aminoacids")]
    pub aminoacids: Option<PathBuf>,

    /// Output nucleotide sequence (FASTA)
    #[arg(short = 'n', long = "nucleotide")]
    pub nucleotide: Option<PathBuf>,

    /// Output protein table (PTT format)
    #[arg(short = 'p', long = "ptt")]
    pub ptt: Option<PathBuf>,

    /// Output ORF nucleotide sequences
    #[arg(short = 'o', long = "orfs")]
    pub orfs: Option<PathBuf>,

    /// Output functions (two-column: protein_id, function)
    #[arg(short = 'f', long = "functions")]
    pub functions: Option<PathBuf>,

    /// Filter by sequence ID (can be used multiple times)
    #[arg(short = 'i', long = "seqid")]
    pub seqid: Vec<String>,

    /// Output GFF3 format
    #[arg(long = "gff3")]
    pub gff3: Option<PathBuf>,

    /// Output AMRFinderPlus format (creates .gff, .faa, .fna)
    #[arg(long = "amr")]
    pub amr: Option<PathBuf>,

    /// Output phage_finder format
    #[arg(long = "phage_finder")]
    pub phage_finder: Option<PathBuf>,

    /// Output Bakta JSON format
    #[arg(long = "bakta-json")]
    pub bakta_json: Option<PathBuf>,

    /// Include pseudo genes
    #[arg(long = "pseudo")]
    pub pseudo: bool,

    /// Split output into separate files per sequence
    #[arg(long = "separate")]
    pub separate: bool,

    /// Gzip compress output
    #[arg(short = 'z', long = "zip")]
    pub zip: bool,

    /// Log file path
    #[arg(long = "log", default_value = "genbank_to.log")]
    pub log: PathBuf,

    /// Enable debug logging
    #[arg(short = 'd', long = "debug")]
    pub debug: bool,

    /// Gram stain (+ or -) - only valid with --bakta-json
    #[arg(long = "gram")]
    pub gram: Option<String>,

    /// Translation table (default: 11) - only valid with --bakta-json
    #[arg(long = "translation-table", default_value = "11")]
    pub translation_table: u8,
}

impl Args {
    /// Validate arguments and apply auto-enabled flags
    pub fn validate(&mut self) -> anyhow::Result<()> {
        // Auto-enable --separate if --seqid is provided
        if !self.seqid.is_empty() && !self.separate {
            self.separate = true;
        }

        // Validate --gram and --translation-table only with --bakta-json
        if self.gram.is_some() && self.bakta_json.is_none() {
            anyhow::bail!("--gram can only be used with --bakta-json");
        }

        // Validate gram value if provided
        if let Some(ref gram) = self.gram {
            if gram != "+" && gram != "-" {
                anyhow::bail!("--gram must be '+' or '-'");
            }
        }

        Ok(())
    }

    /// Check if any output format is requested
    pub fn has_output(&self) -> bool {
        self.aminoacids.is_some()
            || self.nucleotide.is_some()
            || self.ptt.is_some()
            || self.orfs.is_some()
            || self.functions.is_some()
            || self.gff3.is_some()
            || self.amr.is_some()
            || self.phage_finder.is_some()
            || self.bakta_json.is_some()
    }
}
