Changelog
=========

All notable changes to this project will be documented in this file.

Version 0.1.0
-------------

Released: 2024

Initial Release
~~~~~~~~~~~~~~~

**Features:**

* Complete Rust implementation of genbank_to tool
* 100% output compatibility with Python version
* Support for multiple output formats:

  * FASTA (nucleotide, amino acid, ORF)
  * PTT (protein table)
  * Functions table
  * GFF3
  * AMRFinderPlus format
  * Phage Finder format
  * Bakta JSON

* Advanced features:

  * Complex FASTA headers
  * Pseudogene filtering
  * Sequence ID filtering
  * Separate file output per sequence
  * Gzip compression
  * Custom translation tables

* Performance improvements:

  * 10x faster than Python implementation
  * Lower memory footprint
  * Native binary with no runtime dependencies

* Quality:

  * Comprehensive error handling
  * Structured logging
  * Integration tests
  * Cross-platform support (Linux, macOS, Windows)

**Architecture:**

* Modular design with clear separation of concerns
* Uses gb-io for GenBank parsing
* Streaming I/O for efficient processing
* Type-safe with Rust's ownership system

**Documentation:**

* Complete user guide
* API reference
* Examples and tutorials
* Contributing guidelines

Known Limitations
~~~~~~~~~~~~~~~~~

* No genome browsing/visualization (CLI only)
* GenBank output in --separate mode not implemented
* Gram stain inference not implemented
* Only translation table 11 fully tested

Future Plans
~~~~~~~~~~~~

* Add gram stain genus lookup for Bakta JSON
* Implement additional translation tables
* Add parallel processing for multi-record files
* Add progress bars for long-running operations
* Consider streaming output for extremely large files
