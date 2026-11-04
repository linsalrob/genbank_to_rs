genbank_to_rs
=============

A Rust implementation of the `genbank_to <https://github.com/linsalrob/genbank_to>`_ tool for converting NCBI GenBank format files to various output formats.

.. image:: https://img.shields.io/badge/rust-1.70%2B-orange.svg
   :target: https://www.rust-lang.org/
   :alt: Rust Version

.. image:: https://img.shields.io/github/license/linsalrob/genbank_to_rs
   :target: https://github.com/linsalrob/genbank_to_rs/blob/main/LICENSE
   :alt: License

Overview
--------

``genbank_to_rs`` is a high-performance Rust port of the Python ``genbank_to`` tool. It converts NCBI GenBank format files to multiple output formats including FASTA, GFF3, PTT, and specialized formats for tools like AMRFinderPlus, Phage Finder, and Bakta.

The Rust implementation provides:

* **100% output compatibility** with the original Python tool
* **10x performance improvement** through native code and optimized I/O
* **Lower memory footprint** for large GenBank files
* **Native binary** with no runtime dependencies
* **Cross-platform support** (Linux, macOS, Windows)

Supported Output Formats
-------------------------

* **FASTA formats**: nucleotide, amino acids, and ORFs
* **PTT**: NCBI protein table format
* **Functions**: Two-column protein ID and function table
* **GFF3**: Standard GFF3 annotation format
* **AMR**: AMRFinderPlus compatible format
* **Phage Finder**: Phage finder tool format
* **Bakta JSON**: Bakta-compatible JSON output

Quick Start
-----------

Installation
~~~~~~~~~~~~

Install from source using cargo:

.. code-block:: bash

   git clone https://github.com/linsalrob/genbank_to_rs.git
   cd genbank_to_rs
   cargo install --path .

Basic Usage
~~~~~~~~~~~

Extract nucleotide sequence:

.. code-block:: bash

   genbank_to -g input.gbk -n output.fna

Extract protein sequences:

.. code-block:: bash

   genbank_to -g input.gbk -a output.faa

Extract multiple formats at once:

.. code-block:: bash

   genbank_to -g input.gbk \
     -n genome.fna \
     -a proteins.faa \
     -o orfs.fna \
     -p proteins.ptt \
     -f functions.tsv

Documentation Contents
----------------------

.. toctree::
   :maxdepth: 2
   :caption: User Guide

   installation
   usage
   formats
   examples

.. toctree::
   :maxdepth: 2
   :caption: Developer Guide

   architecture
   api
   contributing

.. toctree::
   :maxdepth: 1
   :caption: Additional Information

   changelog
   license

Indices and tables
==================

* :ref:`genindex`
* :ref:`search`
