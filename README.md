# genbank_to_rs

A Rust implementation of the [genbank_to](https://github.com/linsalrob/genbank_to) tool for converting NCBI GenBank format files to various output formats.

## Features

Convert GenBank files to:
- **FASTA formats**: nucleotide (-n), amino acids (-a), and ORFs (-o)
- **PTT**: NCBI protein table format (-p)
- **Functions**: Two-column protein ID and function table (-f)
- **GFF3**: Standard GFF3 annotation format (--gff3)
- **AMR**: AMRFinderPlus compatible format (--amr)
- **Phage Finder**: Phage finder tool format (--phage_finder)
- **Bakta JSON**: Bakta-compatible JSON output (--bakta-json)

**Additional Features**:
- **Automatic gzip detection**: Transparently reads gzip-compressed GenBank files (`.gbk.gz`) by detecting the gzip magic number
- **Gzip output compression**: Compress output files with the `--zip` flag

## Installation

### From source

```bash
git clone https://github.com/linsalrob/genbank_to_rs.git
cd genbank_to_rs
cargo build --release
# Binary will be at ./target/release/genbank_to
```

### Using cargo

```bash
cargo install --path .
```

## Usage

### Basic Examples

Extract nucleotide sequence:
```bash
genbank_to -g input.gbk -n output.fna
```

Extract protein sequences:
```bash
genbank_to -g input.gbk -a output.faa
```

Extract multiple formats at once:
```bash
genbank_to -g input.gbk \
  -n genome.fna \
  -a proteins.faa \
  -o orfs.fna \
  -p proteins.ptt \
  -f functions.tsv
```

### Advanced Features

**Complex FASTA headers** (includes organism, location, product, db_xrefs):
```bash
genbank_to -g input.gbk -a output.faa --complex
```

**Include pseudogenes**:
```bash
genbank_to -g input.gbk -a output.faa --pseudo
```

**Filter by sequence ID**:
```bash
genbank_to -g multi.gbk -a output.faa -i NC_001417
```

**Separate output per sequence** (auto-enabled with -i):
```bash
genbank_to -g multi.gbk -a proteins --separate
# Creates: proteins.NC_001417.faa, proteins.NC_001234.faa, etc.
```

**Read gzip-compressed GenBank files** (automatic detection):
```bash
# Works transparently with gzip-compressed files
genbank_to -g input.gbk.gz -n output.fna
genbank_to -g input.gbk.gz -a output.faa
```

**Gzip output compression**:
```bash
genbank_to -g input.gbk -f functions.tsv --zip
```

**AMRFinderPlus format** (creates .gff, .faa, and .fna):
```bash
genbank_to -g input.gbk --amr output
# Creates: output.gff, output.faa, output.fna
```

**Bakta JSON format**:
```bash
genbank_to -g input.gbk --bakta-json output.json
```

With metadata:
```bash
genbank_to -g input.gbk --bakta-json output.json --gram + --translation-table 11
```

### Logging

By default, logs are written to `genbank_to.log`. You can customize this:

```bash
genbank_to -g input.gbk -a output.faa --log my.log --debug
```

## Command-Line Options

```
Options:
  -g, --genbank <GENBANK>              Input GenBank file (required)
  -c, --complex                        Complex identifier line (for FASTA headers)
  -a, --aminoacids <AMINOACIDS>        Output amino acid sequences (FASTA)
  -n, --nucleotide <NUCLEOTIDE>        Output nucleotide sequence (FASTA)
  -p, --ptt <PTT>                      Output protein table (PTT format)
  -o, --orfs <ORFS>                    Output ORF nucleotide sequences
  -f, --functions <FUNCTIONS>          Output functions (two-column: protein_id, function)
  -i, --seqid <SEQID>                  Filter by sequence ID (can be used multiple times)
      --gff3 <GFF3>                    Output GFF3 format
      --amr <AMR>                      Output AMRFinderPlus format
      --phage_finder <PHAGE_FINDER>    Output phage_finder format
      --bakta-json <BAKTA_JSON>        Output Bakta JSON format
      --pseudo                         Include pseudo genes
      --separate                       Split output into separate files per sequence
  -z, --zip                            Gzip compress output
      --log <LOG>                      Log file path [default: genbank_to.log]
  -d, --debug                          Enable debug logging
      --gram <GRAM>                    Gram stain (+ or -) - only valid with --bakta-json
      --translation-table <TABLE>      Translation table [default: 11]
  -h, --help                           Print help
  -V, --version                        Print version
```

## Performance

The Rust implementation offers significant performance improvements over the Python version:
- Faster parsing and I/O through streaming
- Parallel processing capabilities
- Lower memory footprint
- Native binary with no runtime dependencies

## Compatibility

This Rust implementation aims for 100% output compatibility with the Python [genbank_to](https://github.com/linsalrob/genbank_to) tool. All output formats match the original Python implementation.

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
cargo fmt --check
```

## License

MIT License - See LICENSE file for details

## Citation

If you use this tool in your research, please cite:

Edwards, R. A. (2022). genbank_to: Convert NCBI GenBank files to various formats. https://github.com/linsalrob/genbank_to

## Authors

- Original Python implementation: Rob Edwards
- Rust port: Edwards Lab

## Acknowledgments

This tool uses the `gb-io` crate for GenBank parsing.

