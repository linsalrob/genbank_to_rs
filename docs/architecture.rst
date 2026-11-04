Architecture
============

This page describes the internal architecture of ``genbank_to_rs``.

Overview
--------

``genbank_to_rs`` is structured as a modular Rust application with clear separation of concerns:

* **CLI Layer**: Command-line argument parsing and validation
* **Parser Layer**: GenBank file parsing and data extraction
* **Model Layer**: Internal data representation
* **Output Layer**: Format-specific output generation
* **Utility Layer**: Shared functionality (translation, compression, etc.)

Project Structure
-----------------

.. code-block:: text

   genbank_to_rs/
   ├── src/
   │   ├── main.rs           # Application entry point
   │   ├── cli.rs            # CLI argument parsing
   │   ├── parse.rs          # GenBank parsing logic
   │   ├── model.rs          # Data structures
   │   ├── logging.rs        # Logging setup
   │   ├── util.rs           # Utility functions
   │   └── output/           # Output format modules
   │       ├── mod.rs        # Output module exports
   │       ├── fasta.rs      # FASTA format writers
   │       ├── ptt.rs        # PTT format writer
   │       ├── functions.rs  # Functions table writer
   │       ├── gff3.rs       # GFF3 format writer
   │       ├── amr.rs        # AMRFinderPlus format
   │       ├── phage_finder.rs   # Phage Finder format
   │       ├── bakta_json.rs     # Bakta JSON format
   │       └── genbank.rs    # GenBank format writer
   ├── tests/                # Integration tests
   └── docs/                 # Documentation

Module Descriptions
-------------------

main.rs
~~~~~~~

Application entry point that orchestrates the workflow:

1. Parse command-line arguments
2. Validate arguments
3. Initialize logging
4. Parse GenBank file(s)
5. Filter records if needed
6. Generate requested outputs
7. Handle errors gracefully

**Key Functions**:

* ``main()`` - Entry point with error handling

cli.rs
~~~~~~

Command-line interface using ``clap`` derive macros.

**Key Types**:

* ``Args`` - Main CLI argument structure

**Features**:

* Derives from ``clap::Parser``
* Comprehensive help text
* Validation logic (``validate()`` method)
* Auto-enables ``--separate`` with ``-i``

parse.rs
~~~~~~~~

GenBank file parsing using the ``gb-io`` library.

**Key Functions**:

* ``parse_genbank_file()`` - Main parsing entry point
* ``parse_location()`` - Parse location strings (complement, join, ranges)
* ``extract_qualifiers()`` - Extract feature qualifiers

**Features**:

* Multi-record file support
* Custom location parser for complex features
* Robust error handling

model.rs
~~~~~~~~

Internal data structures representing GenBank records.

**Key Types**:

* ``Record`` - Complete GenBank record

  * ``id`` - Sequence identifier
  * ``description`` - Sequence description
  * ``sequence`` - Nucleotide sequence
  * ``features`` - List of features

* ``Feature`` - Annotation feature

  * ``feature_type`` - Type (CDS, gene, etc.)
  * ``location`` - Genomic location
  * ``qualifiers`` - Feature qualifiers

* ``Location`` - Genomic location

  * Simple: ``start..end``
  * Complement: ``complement(start..end)``
  * Join: ``join(loc1, loc2, ...)``

* ``Qualifiers`` - Feature annotations

  * ``locus_tag`` - Locus tag
  * ``protein_id`` - Protein ID
  * ``product`` - Product description
  * ``translation`` - Amino acid sequence
  * ``gene`` - Gene name
  * ``db_xref`` - Database cross-references
  * ``pseudo`` - Pseudogene flag

logging.rs
~~~~~~~~~~

Logging configuration using ``tracing`` and ``tracing-subscriber``.

**Key Functions**:

* ``init_logging()`` - Initialize file-based logging

**Features**:

* File-based logs (default: ``genbank_to.log``)
* Configurable log level (info/debug)
* Structured logging with spans

util.rs
~~~~~~~

Utility functions shared across modules.

**Key Functions**:

* ``translate()`` - Translate nucleotide to amino acid
* ``get_translation_table()`` - Get codon table
* ``create_writer()`` - Create file writer (plain or gzipped)

**Features**:

* Standard genetic code support
* Custom translation table support
* Gzip compression handling

Output Modules
--------------

output/fasta.rs
~~~~~~~~~~~~~~~

FASTA format output for nucleotide, amino acid, and ORF sequences.

**Key Functions**:

* ``write_nucleotide_fasta()`` - Complete genome sequences
* ``write_aminoacid_fasta()`` - Protein sequences
* ``write_orf_fasta()`` - ORF nucleotide sequences

**Features**:

* Simple and complex headers
* Separate file output per sequence
* Pseudogene filtering

output/ptt.rs
~~~~~~~~~~~~~

NCBI protein table format (8-column tab-separated).

**Key Functions**:

* ``write_ptt()`` - Generate PTT output

**Format**: Location, Strand, Length, PID, Gene, Synonym, Code, COG, Product

output/functions.rs
~~~~~~~~~~~~~~~~~~~

Two-column table of protein IDs and functions.

**Key Functions**:

* ``write_functions()`` - Generate functions table

**Format**: ``protein_id<TAB>function``

output/gff3.rs
~~~~~~~~~~~~~~

GFF3 annotation format.

**Key Functions**:

* ``write_gff3()`` - Generate GFF3 output

**Features**:

* URL-encoded attributes
* Standard GFF3 compliance
* Sequence region headers

output/amr.rs
~~~~~~~~~~~~~

AMRFinderPlus compatible format (three files: .gff, .faa, .fna).

**Key Functions**:

* ``write_amr()`` - Generate AMRFinderPlus files

**Features**:

* Special ``Name=`` attribute in GFF for AMRFinderPlus
* Protein sequences (.faa)
* Nucleotide sequences (.fna)

output/phage_finder.rs
~~~~~~~~~~~~~~~~~~~~~~

Phage Finder tool format (6-column tab-separated).

**Key Functions**:

* ``write_phage_finder()`` - Generate Phage Finder output

**Format**: SeqID, Start, End, Function, ProteinID, Unknown

output/bakta_json.rs
~~~~~~~~~~~~~~~~~~~~

Bakta-compatible JSON output.

**Key Functions**:

* ``write_bakta_json()`` - Generate Bakta JSON

**Features**:

* Complete genome annotation JSON
* MD5 checksums
* Gram stain and translation table metadata

Data Flow
---------

.. code-block:: text

   GenBank File
        ↓
   [parse_genbank_file]
        ↓
   Vec<Record>
        ↓
   [Filter by seqid if needed]
        ↓
   Filtered Records
        ↓
   ┌─────────────────────────────────────┐
   │  Output Format Selection            │
   ├─────────────────────────────────────┤
   │  - FASTA (nucleotide/aa/orf)        │
   │  - PTT                              │
   │  - Functions                        │
   │  - GFF3                             │
   │  - AMR                              │
   │  - Phage Finder                     │
   │  - Bakta JSON                       │
   └─────────────────────────────────────┘
        ↓
   Output Files

Error Handling
--------------

The application uses ``anyhow::Result`` for error handling throughout:

* **Parsing Errors**: Invalid GenBank format, missing required fields
* **I/O Errors**: File not found, permission denied, disk full
* **Validation Errors**: Invalid arguments, conflicting options
* **Translation Errors**: Invalid codon, frame issues

All errors are:

1. Logged to the log file
2. Printed to stderr with context
3. Result in non-zero exit code

Performance Considerations
--------------------------

Memory Usage
~~~~~~~~~~~~

* **Streaming I/O**: Input files are read in streaming fashion
* **Record Storage**: All records loaded into memory for processing
* **Output Buffering**: Uses buffered writers for efficient I/O

**Memory Profile**: ~O(n) where n is total size of GenBank records

CPU Usage
~~~~~~~~~

* **Single-threaded**: Currently single-threaded processing
* **Translation**: CPU-intensive for large genomes with many CDSs
* **Compression**: Gzip compression adds CPU overhead

**Future**: Could parallelize per-record processing for multi-record files

I/O Patterns
~~~~~~~~~~~~

* **Sequential Reads**: GenBank file read sequentially
* **Buffered Writes**: All output uses buffered writers
* **Gzip Compression**: Optional compression for space savings

Dependencies
------------

Core Dependencies
~~~~~~~~~~~~~~~~~

* ``clap`` 4.5 - CLI parsing with derive macros
* ``gb-io`` 0.7 - GenBank format parsing
* ``anyhow`` 1.0 - Flexible error handling
* ``tracing`` 0.1 - Structured logging framework
* ``tracing-subscriber`` 0.3 - Logging implementation

Utility Dependencies
~~~~~~~~~~~~~~~~~~~~

* ``flate2`` 1.0 - Gzip compression
* ``serde`` 1.0 - Serialization framework
* ``serde_json`` 1.0 - JSON output
* ``md5`` 0.7 - Checksum generation
* ``regex`` 1.10 - Pattern matching
* ``itertools`` 0.12 - Iterator utilities

Testing
-------

Integration Tests
~~~~~~~~~~~~~~~~~

Located in ``tests/integration_test.rs``:

* Command-line option tests
* Output format validation
* Feature flag tests
* Edge case handling

**Test Data**: Uses real GenBank files (e.g., MS2 phage)

Test Approach
~~~~~~~~~~~~~

* **Black-box testing**: Tests via CLI interface
* **Snapshot testing**: Uses ``insta`` for output validation
* **Temporary files**: Uses ``tempfile`` for isolated test environments

Code Quality
------------

Linting
~~~~~~~

.. code-block:: bash

   cargo clippy

Format Checking
~~~~~~~~~~~~~~~

.. code-block:: bash

   cargo fmt --check

Testing
~~~~~~~

.. code-block:: bash

   cargo test

Future Enhancements
-------------------

Potential Improvements
~~~~~~~~~~~~~~~~~~~~~~

1. **Parallelization**: Process multiple records in parallel
2. **Streaming Output**: For extremely large files
3. **Custom Translation Tables**: Support all NCBI translation tables
4. **Gram Stain Lookup**: Automatic genus-based inference
5. **Progress Bars**: For long-running operations
6. **GenBank Output**: Full GenBank format writing in --separate mode

Performance Optimizations
~~~~~~~~~~~~~~~~~~~~~~~~~~

1. **Zero-copy Parsing**: Reduce string allocations
2. **Lazy Translation**: Only translate when needed
3. **Memory Mapping**: For extremely large files
4. **SIMD**: For sequence processing operations
