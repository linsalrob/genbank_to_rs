API Reference
=============

This page documents the public API of ``genbank_to_rs``.

.. note::
   This is a command-line tool. This documentation describes the internal
   Rust API for developers who want to use or extend the library.

Core Modules
------------

cli Module
~~~~~~~~~~

Command-line interface parsing and validation.

**Type: Args**

.. code-block:: rust

   pub struct Args {
       pub genbank: PathBuf,
       pub complex: bool,
       pub aminoacids: Option<PathBuf>,
       pub nucleotide: Option<PathBuf>,
       pub ptt: Option<PathBuf>,
       pub orfs: Option<PathBuf>,
       pub functions: Option<PathBuf>,
       pub seqid: Vec<String>,
       pub gff3: Option<PathBuf>,
       pub amr: Option<PathBuf>,
       pub phage_finder: Option<PathBuf>,
       pub bakta_json: Option<PathBuf>,
       pub pseudo: bool,
       pub separate: bool,
       pub zip: bool,
       pub log: PathBuf,
       pub debug: bool,
       pub gram: Option<String>,
       pub translation_table: i32,
   }

**Methods:**

* ``validate(&mut self) -> Result<()>`` - Validate arguments and apply auto-enabled flags

model Module
~~~~~~~~~~~~

Internal data structures representing GenBank records.

**Type: Record**

.. code-block:: rust

   pub struct Record {
       pub id: String,
       pub description: String,
       pub sequence: String,
       pub features: Vec<Feature>,
   }

**Type: Feature**

.. code-block:: rust

   pub struct Feature {
       pub feature_type: String,
       pub location: Location,
       pub qualifiers: Qualifiers,
   }

**Type: Location**

.. code-block:: rust

   pub enum Location {
       Simple {
           start: usize,
           end: usize,
           complement: bool,
       },
       Join {
           locations: Vec<Location>,
       },
   }

**Methods:**

* ``start(&self) -> usize`` - Get start position
* ``end(&self) -> usize`` - Get end position
* ``is_complement(&self) -> bool`` - Check if on complement strand
* ``is_simple(&self) -> bool`` - Check if simple location (not join)

**Type: Qualifiers**

.. code-block:: rust

   pub struct Qualifiers {
       pub locus_tag: Option<String>,
       pub protein_id: Option<String>,
       pub product: Option<String>,
       pub translation: Option<String>,
       pub gene: Option<String>,
       pub db_xref: Vec<String>,
       pub codon_start: Option<usize>,
       pub transl_table: Option<i32>,
       pub note: Option<String>,
       pub pseudo: bool,
   }

parse Module
~~~~~~~~~~~~

GenBank file parsing functions.

**Function: parse_genbank_file**

.. code-block:: rust

   pub fn parse_genbank_file(path: &Path) -> Result<Vec<Record>>

Parse a GenBank file and return a vector of records.

**Parameters:**

* ``path`` - Path to GenBank file

**Returns:**

* ``Result<Vec<Record>>`` - Parsed records or error

**Errors:**

* File not found
* Invalid GenBank format
* Missing required fields

**Function: parse_location**

.. code-block:: rust

   pub fn parse_location(loc_str: &str) -> Result<Location>

Parse a GenBank location string.

**Parameters:**

* ``loc_str`` - Location string (e.g., "123..456", "complement(123..456)", "join(1..100,200..300)")

**Returns:**

* ``Result<Location>`` - Parsed location or error

**Examples:**

* ``"123..456"`` → Simple location
* ``"complement(123..456)"`` → Complement strand
* ``"join(1..100,200..300)"`` → Joined locations

util Module
~~~~~~~~~~~

Utility functions for sequence processing.

**Function: translate**

.. code-block:: rust

   pub fn translate(
       sequence: &str,
       table: i32,
       partial: bool,
   ) -> Result<String>

Translate nucleotide sequence to amino acids.

**Parameters:**

* ``sequence`` - Nucleotide sequence
* ``table`` - Translation table number (typically 11)
* ``partial`` - Whether to allow partial codons

**Returns:**

* ``Result<String>`` - Amino acid sequence or error

**Function: create_writer**

.. code-block:: rust

   pub fn create_writer(
       path: &Path,
       zip: bool,
   ) -> Result<Box<dyn Write>>

Create a file writer, optionally with gzip compression.

**Parameters:**

* ``path`` - Output file path
* ``zip`` - Whether to use gzip compression

**Returns:**

* ``Result<Box<dyn Write>>`` - Writer instance or error

Output Module
~~~~~~~~~~~~~

Output format generation functions.

FASTA Output
^^^^^^^^^^^^

**Function: write_nucleotide_fasta**

.. code-block:: rust

   pub fn write_nucleotide_fasta(
       records: &[&Record],
       output: &Path,
       zip: bool,
   ) -> Result<()>

Write nucleotide sequences in FASTA format.

**Function: write_aminoacid_fasta**

.. code-block:: rust

   pub fn write_aminoacid_fasta(
       records: &[&Record],
       output: &Path,
       complex: bool,
       pseudo: bool,
       table: i32,
       zip: bool,
   ) -> Result<()>

Write amino acid sequences in FASTA format.

**Function: write_orf_fasta**

.. code-block:: rust

   pub fn write_orf_fasta(
       records: &[&Record],
       output: &Path,
       pseudo: bool,
       zip: bool,
   ) -> Result<()>

Write ORF nucleotide sequences in FASTA format.

PTT Output
^^^^^^^^^^

**Function: write_ptt**

.. code-block:: rust

   pub fn write_ptt(
       records: &[&Record],
       output: &Path,
       pseudo: bool,
       table: i32,
       zip: bool,
   ) -> Result<()>

Write protein table in PTT format.

Functions Output
^^^^^^^^^^^^^^^^

**Function: write_functions**

.. code-block:: rust

   pub fn write_functions(
       records: &[&Record],
       output: &Path,
       pseudo: bool,
       zip: bool,
   ) -> Result<()>

Write functions table (protein_id, function).

GFF3 Output
^^^^^^^^^^^

**Function: write_gff3**

.. code-block:: rust

   pub fn write_gff3(
       records: &[&Record],
       output: &Path,
       pseudo: bool,
       zip: bool,
   ) -> Result<()>

Write GFF3 annotation format.

AMR Output
^^^^^^^^^^

**Function: write_amr**

.. code-block:: rust

   pub fn write_amr(
       records: &[&Record],
       output: &Path,
       pseudo: bool,
       table: i32,
   ) -> Result<()>

Write AMRFinderPlus compatible files (.gff, .faa, .fna).

Phage Finder Output
^^^^^^^^^^^^^^^^^^^

**Function: write_phage_finder**

.. code-block:: rust

   pub fn write_phage_finder(
       records: &[&Record],
       output: &Path,
       pseudo: bool,
       zip: bool,
   ) -> Result<()>

Write Phage Finder format.

Bakta JSON Output
^^^^^^^^^^^^^^^^^

**Function: write_bakta_json**

.. code-block:: rust

   pub fn write_bakta_json(
       records: &[&Record],
       output: &Path,
       gram: &Option<String>,
       translation_table: i32,
   ) -> Result<()>

Write Bakta-compatible JSON format.

Error Handling
--------------

All functions return ``anyhow::Result<T>`` for flexible error handling.

Common Error Types
~~~~~~~~~~~~~~~~~~

* **IoError**: File I/O errors (not found, permission denied, etc.)
* **ParseError**: GenBank parsing errors
* **ValidationError**: Invalid arguments or data
* **TranslationError**: Sequence translation errors

Example Error Handling
~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use anyhow::{Context, Result};

   fn process_genbank(path: &Path) -> Result<()> {
       let records = parse_genbank_file(path)
           .context("Failed to parse GenBank file")?;
       
       write_aminoacid_fasta(&records, &output, false, false, 11, false)
           .context("Failed to write amino acid FASTA")?;
       
       Ok(())
   }

Logging
-------

The application uses the ``tracing`` framework for structured logging.

Log Levels
~~~~~~~~~~

* **ERROR**: Critical errors that prevent operation
* **WARN**: Warnings about potential issues
* **INFO**: Normal operational messages
* **DEBUG**: Detailed debugging information

Example Logging Usage
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use tracing::{info, warn, error, debug};

   fn process_record(record: &Record) {
       info!("Processing record: {}", record.id);
       
       if record.features.is_empty() {
           warn!("Record {} has no features", record.id);
       }
       
       debug!("Record has {} features", record.features.len());
   }

Type Reference
--------------

PathBuf
~~~~~~~

Standard Rust ``std::path::PathBuf`` for file paths.

String vs &str
~~~~~~~~~~~~~~

* ``String`` - Owned string data
* ``&str`` - Borrowed string slice

Result<T>
~~~~~~~~~

Rust ``Result`` type, aliased to ``anyhow::Result<T>`` for flexible error handling.

Vec<T>
~~~~~~

Standard Rust vector for collections.

Option<T>
~~~~~~~~~

Standard Rust ``Option`` for optional values:

* ``Some(value)`` - Contains a value
* ``None`` - No value

Examples
--------

Parsing and Processing
~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use genbank_to_rs::{parse, model, output};

   fn main() -> anyhow::Result<()> {
       // Parse GenBank file
       let records = parse::parse_genbank_file("input.gbk")?;
       
       // Filter CDS features
       for record in &records {
           let cds_features: Vec<_> = record.features.iter()
               .filter(|f| f.feature_type == "CDS")
               .collect();
           
           println!("{} has {} CDS features", 
                    record.id, cds_features.len());
       }
       
       // Generate output
       let refs: Vec<_> = records.iter().collect();
       output::fasta::write_aminoacid_fasta(
           &refs, 
           Path::new("output.faa"),
           false,  // complex headers
           false,  // include pseudo
           11,     // translation table
           false   // zip
       )?;
       
       Ok(())
   }

Custom Processing
~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use genbank_to_rs::model::*;

   fn count_genes_by_strand(record: &Record) -> (usize, usize) {
       let mut forward = 0;
       let mut reverse = 0;
       
       for feature in &record.features {
           if feature.feature_type == "CDS" {
               if feature.location.is_complement() {
                   reverse += 1;
               } else {
                   forward += 1;
               }
           }
       }
       
       (forward, reverse)
   }

Testing
-------

The crate includes integration tests in ``tests/integration_test.rs``.

Running Tests
~~~~~~~~~~~~~

.. code-block:: bash

   cargo test

Running Specific Tests
~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   cargo test test_help
   cargo test test_fasta

With Output
~~~~~~~~~~~

.. code-block:: bash

   cargo test -- --nocapture
