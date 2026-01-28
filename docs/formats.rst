Output Formats
==============

This page describes the various output formats supported by ``genbank_to``.

FASTA Formats
-------------

Nucleotide FASTA
~~~~~~~~~~~~~~~~

**Option**: ``-n, --nucleotide``

**Description**: Outputs the complete genome sequence(s) in FASTA format.

**Format**:

.. code-block:: text

   >sequence_id
   ATGCGATCGATCG...

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk -n genome.fna

Amino Acid FASTA
~~~~~~~~~~~~~~~~

**Option**: ``-a, --aminoacids``

**Description**: Outputs translated protein sequences from CDS features.

**Format**:

.. code-block:: text

   >protein_id
   MKLPQRSTV...

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk -a proteins.faa

ORF Nucleotide FASTA
~~~~~~~~~~~~~~~~~~~~

**Option**: ``-o, --orfs``

**Description**: Outputs nucleotide sequences of ORFs (CDS features).

**Format**:

.. code-block:: text

   >protein_id
   ATGAAGCTGCCGCAG...

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk -o orfs.fna

Complex FASTA Headers
~~~~~~~~~~~~~~~~~~~~~~

**Option**: ``--complex``

**Description**: Includes detailed metadata in FASTA headers.

**Format**:

.. code-block:: text

   >protein_id organism_name start..end product db_xref1|db_xref2
   SEQUENCE...

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk -a proteins.faa --complex

**Sample Output**:

.. code-block:: text

   >NP_040622.1 Enterobacteria phage MS2 1..390 lysis protein GI:9632255
   MSKTAYDRRGKNRAYQRFVEHFRGPNETRGGWLTVRDQRNFVTAIAA...

Table Formats
-------------

Protein Table (PTT)
~~~~~~~~~~~~~~~~~~~

**Option**: ``-p, --ptt``

**Description**: NCBI-style protein table format with 8 tab-separated columns.

**Columns**:

1. Location (start..end)
2. Strand (+/-)
3. Length (amino acids)
4. PID (protein ID)
5. Gene name
6. Synonym
7. Code (genetic code, usually 11)
8. COG
9. Product description

**Header Format**:

.. code-block:: text

   sequence_description - start..end
   number_proteins proteins
   Location    Strand    Length    PID    Gene    Synonym    Code    COG    Product

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk -p proteins.ptt

**Sample Output**:

.. code-block:: text

   NC_001417.2 Enterobacteria phage MS2, complete genome. - 1..3569
   4 proteins
   Location    Strand    Length    PID    Gene    Synonym    Code    COG    Product
   1..390    +    129    NP_040622.1    -    -    11    -    lysis protein
   390..1556    +    388    NP_040623.1    -    -    11    -    capsid protein

Functions Table
~~~~~~~~~~~~~~~

**Option**: ``-f, --functions``

**Description**: Two-column table mapping protein IDs to their functional descriptions.

**Format**:

.. code-block:: text

   protein_id<TAB>function

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk -f functions.tsv

**Sample Output**:

.. code-block:: text

   NP_040622.1    lysis protein
   NP_040623.1    capsid protein
   NP_040624.1    replicase

Annotation Formats
------------------

GFF3 Format
~~~~~~~~~~~

**Option**: ``--gff3``

**Description**: Standard GFF3 format with URL-encoded attributes.

**Columns**:

1. seqid
2. source (genbank_to)
3. type (CDS)
4. start
5. end
6. score (.)
7. strand (+/-)
8. phase (0, 1, or 2)
9. attributes (URL-encoded key=value pairs)

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk --gff3 output.gff3

**Sample Output**:

.. code-block:: text

   ##gff-version 3
   ##sequence-region NC_001417.2 1 3569
   NC_001417.2    genbank_to    CDS    1    390    .    +    0    ID=NP_040622.1;product=lysis%20protein
   NC_001417.2    genbank_to    CDS    390    1556    .    +    0    ID=NP_040623.1;product=capsid%20protein

Specialized Formats
-------------------

AMRFinderPlus Format
~~~~~~~~~~~~~~~~~~~~

**Option**: ``--amr``

**Description**: Creates three files compatible with AMRFinderPlus:

* ``.gff`` - GFF3 with special Name= attribute
* ``.faa`` - Protein sequences
* ``.fna`` - Nucleotide sequences

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk --amr output

**Creates**:

* ``output.gff``
* ``output.faa``
* ``output.fna``

**GFF Format**: Standard GFF3 with Name= attribute for AMRFinderPlus compatibility.

Phage Finder Format
~~~~~~~~~~~~~~~~~~~

**Option**: ``--phage_finder``

**Description**: Six-column tab-separated format for Phage Finder tool.

**Columns**:

1. Sequence ID
2. Start position
3. End position
4. Function/Product
5. Protein ID
6. Unknown (-)

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk --phage_finder output.txt

**Sample Output**:

.. code-block:: text

   NC_001417.2    1    390    lysis protein    NP_040622.1    -
   NC_001417.2    390    1556    capsid protein    NP_040623.1    -

Bakta JSON Format
~~~~~~~~~~~~~~~~~

**Option**: ``--bakta-json``

**Description**: Complete genome annotation in Bakta-compatible JSON format.

**Structure**:

.. code-block:: json

   {
     "version": "1.0",
     "genome": {
       "taxon": "",
       "translation_table": 11,
       "gram": "?"
     },
     "sequences": [
       {
         "id": "NC_001417.2",
         "description": "Enterobacteria phage MS2",
         "sequence": "ATCG...",
         "length": 3569,
         "type": "chromosome",
         "topology": "circular",
         "md5": "..."
       }
     ],
     "features": [
       {
         "type": "cds",
         "contig": "NC_001417.2",
         "start": 1,
         "stop": 390,
         "strand": "+",
         "gene": null,
         "product": "lysis protein",
         "locus_tag": null,
         "protein_id": "NP_040622.1",
         "aa": "MSKTAY...",
         "nt": "ATGAGC..."
       }
     ]
   }

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk --bakta-json output.json

**With Metadata**:

.. code-block:: bash

   genbank_to -g input.gbk --bakta-json output.json \
     --gram + \
     --translation-table 11

Output Options
--------------

Separate Files
~~~~~~~~~~~~~~

**Option**: ``--separate``

When processing multi-sequence GenBank files, create separate output files for each sequence.

**Example**:

.. code-block:: bash

   genbank_to -g multi.gbk -a proteins --separate

**Creates**:

* ``proteins.NC_001417.faa``
* ``proteins.NC_001234.faa``
* etc.

Gzip Compression
~~~~~~~~~~~~~~~~

**Option**: ``-z, --zip``

Compress output files with gzip.

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk -f functions.tsv --zip

**Creates**: ``functions.tsv.gz``

Pseudogene Handling
~~~~~~~~~~~~~~~~~~~

**Option**: ``--pseudo``

By default, features marked as pseudogenes are excluded from output. Use ``--pseudo`` to include them.

**Example**:

.. code-block:: bash

   genbank_to -g input.gbk -a proteins.faa --pseudo

Format Compatibility
--------------------

Output Compatibility
~~~~~~~~~~~~~~~~~~~~

All output formats are designed for 100% compatibility with the original Python ``genbank_to`` tool. Output files should be identical (or functionally equivalent) between the two implementations.

Standard Compliance
~~~~~~~~~~~~~~~~~~~

* **FASTA**: Follows standard FASTA format conventions
* **GFF3**: Complies with GFF3 specification (http://gmod.org/wiki/GFF3)
* **PTT**: Matches NCBI PTT format specification
* **JSON**: Valid JSON that can be parsed by standard JSON tools

Tool Integration
~~~~~~~~~~~~~~~~

Outputs are designed to work seamlessly with:

* **AMRFinderPlus**: Antimicrobial resistance gene finding
* **Phage Finder**: Phage identification in bacterial genomes
* **Bakta**: Rapid bacterial genome annotation
* Standard bioinformatics tools (BLAST, sequence aligners, etc.)
