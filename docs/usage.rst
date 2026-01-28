Usage
=====

This guide covers how to use ``genbank_to`` for various tasks.

Basic Command Structure
-----------------------

The basic command structure is:

.. code-block:: bash

   genbank_to -g <input.gbk> [OPTIONS]

The ``-g`` or ``--genbank`` option specifies the input GenBank file (required).

Common Use Cases
----------------

Extract Nucleotide Sequences
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Extract the complete genome sequence in FASTA format:

.. code-block:: bash

   genbank_to -g input.gbk -n output.fna

Extract Protein Sequences
~~~~~~~~~~~~~~~~~~~~~~~~~~

Extract all protein sequences (translated from CDS features):

.. code-block:: bash

   genbank_to -g input.gbk -a output.faa

Extract ORF Sequences
~~~~~~~~~~~~~~~~~~~~~

Extract nucleotide sequences of all ORFs (CDS features):

.. code-block:: bash

   genbank_to -g input.gbk -o orfs.fna

Generate Protein Table (PTT)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Create an NCBI-style protein table:

.. code-block:: bash

   genbank_to -g input.gbk -p proteins.ptt

Extract Function Annotations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Create a two-column table of protein IDs and their functions:

.. code-block:: bash

   genbank_to -g input.gbk -f functions.tsv

Generate GFF3 Format
~~~~~~~~~~~~~~~~~~~~

Convert to GFF3 annotation format:

.. code-block:: bash

   genbank_to -g input.gbk --gff3 output.gff3

Multiple Output Formats
~~~~~~~~~~~~~~~~~~~~~~~

Generate multiple outputs in a single run:

.. code-block:: bash

   genbank_to -g input.gbk \
     -n genome.fna \
     -a proteins.faa \
     -o orfs.fna \
     -p proteins.ptt \
     -f functions.tsv

Advanced Features
-----------------

Complex FASTA Headers
~~~~~~~~~~~~~~~~~~~~~~

Include detailed metadata in FASTA headers (organism, location, product, db_xrefs):

.. code-block:: bash

   genbank_to -g input.gbk -a output.faa --complex

Example output:

.. code-block:: text

   >protein_id organism_name location product db_xref1|db_xref2
   MKLPQRSTV...

Include Pseudogenes
~~~~~~~~~~~~~~~~~~~

By default, pseudogenes are excluded. To include them:

.. code-block:: bash

   genbank_to -g input.gbk -a output.faa --pseudo

Filter by Sequence ID
~~~~~~~~~~~~~~~~~~~~~

Process only specific sequences from a multi-record GenBank file:

.. code-block:: bash

   genbank_to -g multi.gbk -a output.faa -i NC_001417

Multiple sequence IDs can be specified:

.. code-block:: bash

   genbank_to -g multi.gbk -a output.faa -i NC_001417 -i NC_001234

Separate Output Files
~~~~~~~~~~~~~~~~~~~~~

Create separate output files for each sequence (automatically enabled with ``-i``):

.. code-block:: bash

   genbank_to -g multi.gbk -a proteins --separate

This creates files like:

* ``proteins.NC_001417.faa``
* ``proteins.NC_001234.faa``

Gzip Compression
~~~~~~~~~~~~~~~~

Compress output files with gzip:

.. code-block:: bash

   genbank_to -g input.gbk -f functions.tsv --zip

This creates ``functions.tsv.gz``.

Specialized Output Formats
---------------------------

AMRFinderPlus Format
~~~~~~~~~~~~~~~~~~~~

Generate AMRFinderPlus compatible files (creates .gff, .faa, and .fna):

.. code-block:: bash

   genbank_to -g input.gbk --amr output

This creates:

* ``output.gff`` - GFF3 format with specific Name= attribute
* ``output.faa`` - Protein sequences
* ``output.fna`` - Nucleotide sequences

Phage Finder Format
~~~~~~~~~~~~~~~~~~~

Generate output for Phage Finder tool:

.. code-block:: bash

   genbank_to -g input.gbk --phage_finder output.txt

Bakta JSON Format
~~~~~~~~~~~~~~~~~

Generate Bakta-compatible JSON output:

.. code-block:: bash

   genbank_to -g input.gbk --bakta-json output.json

With metadata:

.. code-block:: bash

   genbank_to -g input.gbk --bakta-json output.json \
     --gram + \
     --translation-table 11

Logging Options
---------------

Default Logging
~~~~~~~~~~~~~~~

By default, logs are written to ``genbank_to.log`` in the current directory.

Custom Log File
~~~~~~~~~~~~~~~

Specify a custom log file:

.. code-block:: bash

   genbank_to -g input.gbk -a output.faa --log my_analysis.log

Debug Logging
~~~~~~~~~~~~~

Enable verbose debug output:

.. code-block:: bash

   genbank_to -g input.gbk -a output.faa --debug

Command-Line Reference
----------------------

Required Options
~~~~~~~~~~~~~~~~

.. option:: -g, --genbank <GENBANK>

   Input GenBank file (required)

FASTA Output Options
~~~~~~~~~~~~~~~~~~~~

.. option:: -n, --nucleotide <NUCLEOTIDE>

   Output nucleotide sequence (FASTA format)

.. option:: -a, --aminoacids <AMINOACIDS>

   Output amino acid sequences (FASTA format)

.. option:: -o, --orfs <ORFS>

   Output ORF nucleotide sequences (FASTA format)

.. option:: -c, --complex

   Use complex identifier line in FASTA headers (includes organism, location, product, db_xrefs)

Table Format Options
~~~~~~~~~~~~~~~~~~~~

.. option:: -p, --ptt <PTT>

   Output protein table (PTT format)

.. option:: -f, --functions <FUNCTIONS>

   Output functions as two-column table (protein_id, function)

Annotation Format Options
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. option:: --gff3 <GFF3>

   Output GFF3 format

.. option:: --amr <AMR>

   Output AMRFinderPlus compatible format (creates .gff, .faa, .fna)

.. option:: --phage_finder <PHAGE_FINDER>

   Output Phage Finder format

.. option:: --bakta-json <BAKTA_JSON>

   Output Bakta JSON format

Filtering Options
~~~~~~~~~~~~~~~~~

.. option:: -i, --seqid <SEQID>

   Filter by sequence ID (can be used multiple times)

.. option:: --pseudo

   Include pseudogenes in output (default: exclude)

Output Control Options
~~~~~~~~~~~~~~~~~~~~~~

.. option:: --separate

   Split output into separate files per sequence (auto-enabled with -i)

.. option:: -z, --zip

   Gzip compress output files

Bakta-Specific Options
~~~~~~~~~~~~~~~~~~~~~~

.. option:: --gram <GRAM>

   Gram stain (+ or -) - only valid with --bakta-json

.. option:: --translation-table <TABLE>

   Translation table number (default: 11)

Logging Options
~~~~~~~~~~~~~~~

.. option:: --log <LOG>

   Log file path (default: genbank_to.log)

.. option:: -d, --debug

   Enable debug-level logging

General Options
~~~~~~~~~~~~~~~

.. option:: -h, --help

   Print help information

.. option:: -V, --version

   Print version information

Exit Codes
----------

* **0**: Success
* **1**: Error (invalid input, missing file, parsing error, etc.)

The tool will log detailed error messages to the log file and print a summary to stderr.

Examples
--------

See the :doc:`examples` page for more detailed examples and use cases.
