//! Utility functions for file I/O and formatting
//!
//! Helpers for gzip compression, FASTA wrapping, etc.

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Writer that can optionally compress with gzip
pub enum MaybeGzWriter {
    Plain(BufWriter<File>),
    Gzip(BufWriter<GzEncoder<File>>),
}

impl MaybeGzWriter {
    /// Create a new writer, optionally with gzip compression
    pub fn new(path: &Path, compress: bool) -> anyhow::Result<Self> {
        let file = File::create(path)?;
        
        if compress {
            let encoder = GzEncoder::new(file, Compression::default());
            Ok(MaybeGzWriter::Gzip(BufWriter::new(encoder)))
        } else {
            Ok(MaybeGzWriter::Plain(BufWriter::new(file)))
        }
    }
}

impl Write for MaybeGzWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            MaybeGzWriter::Plain(w) => w.write(buf),
            MaybeGzWriter::Gzip(w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            MaybeGzWriter::Plain(w) => w.flush(),
            MaybeGzWriter::Gzip(w) => w.flush(),
        }
    }
}

/// Write FASTA record (no wrapping - single line per sequence to match Python)
pub fn write_fasta<W: Write>(writer: &mut W, header: &str, sequence: &str) -> anyhow::Result<()> {
    writeln!(writer, ">{}", header)?;
    writeln!(writer, "{}", sequence)?;
    Ok(())
}

/// URL encode a string for GFF3 attributes (spaces, equals, semicolons, etc.)
pub fn url_encode_gff(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '=' => "%3D".to_string(),
            ';' => "%3B".to_string(),
            ',' => "%2C".to_string(),
            '\n' => "%0A".to_string(),
            '\r' => "%0D".to_string(),
            '\t' => "%09".to_string(),
            '%' => "%25".to_string(),
            _ if c.is_ascii_graphic() || c.is_ascii_whitespace() => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

/// Check if a file is gzip compressed by reading magic bytes
pub fn is_gzip(path: &Path) -> anyhow::Result<bool> {
    use std::io::Read;
    let mut file = File::open(path)?;
    let mut magic = [0u8; 2];
    file.read_exact(&mut magic).ok();
    Ok(magic == [0x1f, 0x8b])
}
