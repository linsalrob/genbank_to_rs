Examples
========

This page provides comprehensive examples for using ``genbank_to``.

Basic Workflows
---------------

Single Genome Analysis
~~~~~~~~~~~~~~~~~~~~~~

Extract all common formats from a single genome:

.. code-block:: bash

   # Input: bacterial genome in GenBank format
   genbank_to -g ecoli_k12.gbk \
     -n genome.fna \
     -a proteins.faa \
     -o orfs.fna \
     -p proteins.ptt \
     -f functions.tsv

**Output Files**:

* ``genome.fna`` - Complete genome sequence
* ``proteins.faa`` - All protein sequences
* ``orfs.fna`` - ORF nucleotide sequences
* ``proteins.ptt`` - Protein table
* ``functions.tsv`` - Protein functions

Multi-Genome Processing
~~~~~~~~~~~~~~~~~~~~~~~~

Process a multi-sequence GenBank file with separate outputs:

.. code-block:: bash

   genbank_to -g multiple_genomes.gbk \
     -a proteins \
     --separate

**Output**: Creates ``proteins.SEQID.faa`` for each sequence in the file.

Phage Genome Analysis
---------------------

Complete Phage Annotation Pipeline
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Extract all information from a phage genome:

.. code-block:: bash

   # MS2 phage (small RNA phage)
   genbank_to -g ms2_phage.gbk \
     -n ms2_genome.fna \
     -a ms2_proteins.faa \
     --gff3 ms2_annotation.gff3 \
     --phage_finder ms2_phage_finder.txt \
     -f ms2_functions.tsv \
     --complex

**Analysis Steps**:

1. ``ms2_genome.fna`` - Use for sequence comparison
2. ``ms2_proteins.faa`` - Run through BLAST or HMM searches
3. ``ms2_annotation.gff3`` - Visualize in genome browsers
4. ``ms2_phage_finder.txt`` - Input to Phage Finder tool
5. ``ms2_functions.tsv`` - Functional annotation summary

Phage with Complex Headers
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Generate protein FASTA with detailed metadata:

.. code-block:: bash

   genbank_to -g phage.gbk -a phage_proteins.faa --complex

**Example Output**:

.. code-block:: text

   >NP_040622.1 Enterobacteria phage MS2 1..390 lysis protein GI:9632255
   MSKTAYDRRGKNRAYQRFVEHFRGPNETRGGWLTVRDQRNFVTAIAA...

Bacterial Genome Analysis
--------------------------

AMR Gene Analysis
~~~~~~~~~~~~~~~~~

Prepare genome for AMRFinderPlus analysis:

.. code-block:: bash

   genbank_to -g bacteria.gbk --amr bacteria_amr

**Then run AMRFinderPlus**:

.. code-block:: bash

   amrfinder \
     --protein bacteria_amr.faa \
     --gff bacteria_amr.gff \
     --nucleotide bacteria_amr.fna \
     --output amr_results.tsv

Bakta-Compatible Output
~~~~~~~~~~~~~~~~~~~~~~~~

Generate Bakta-compatible JSON:

.. code-block:: bash

   genbank_to -g bacteria.gbk \
     --bakta-json bacteria.json \
     --gram + \
     --translation-table 11

For Gram-negative bacteria:

.. code-block:: bash

   genbank_to -g gram_negative.gbk \
     --bakta-json output.json \
     --gram -

Functional Analysis
-------------------

Extract Specific Functions
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Use the functions table for downstream analysis:

.. code-block:: bash

   # Extract all functions
   genbank_to -g genome.gbk -f all_functions.tsv

   # Find specific functions (e.g., kinases)
   grep -i kinase all_functions.tsv > kinases.tsv

   # Count function types
   cut -f2 all_functions.tsv | sort | uniq -c | sort -rn > function_counts.txt

Protein Database Creation
~~~~~~~~~~~~~~~~~~~~~~~~~~

Create a searchable protein database:

.. code-block:: bash

   # Extract proteins
   genbank_to -g genome.gbk -a proteins.faa

   # Create BLAST database
   makeblastdb -in proteins.faa -dbtype prot -out genome_db

   # Search against database
   blastp -query query.faa -db genome_db -out results.txt

Sequence Extraction
-------------------

Extract Specific Sequences
~~~~~~~~~~~~~~~~~~~~~~~~~~~

From a multi-sequence GenBank file, extract only specific sequences:

.. code-block:: bash

   # Extract single sequence
   genbank_to -g multi.gbk -a proteins.faa -i NC_001417

   # Extract multiple sequences
   genbank_to -g multi.gbk \
     -a proteins.faa \
     -i NC_001417 \
     -i NC_001234 \
     -i NC_005678

**Note**: ``--separate`` is automatically enabled with ``-i``.

Compressed Output
~~~~~~~~~~~~~~~~~

Save disk space with gzip compression:

.. code-block:: bash

   genbank_to -g large_genome.gbk \
     -n genome.fna \
     -a proteins.faa \
     --zip

**Output**: ``genome.fna.gz`` and ``proteins.faa.gz``

Genome Annotation Conversion
-----------------------------

GenBank to GFF3
~~~~~~~~~~~~~~~

Convert GenBank annotations to GFF3 for use in genome browsers:

.. code-block:: bash

   genbank_to -g annotation.gbk --gff3 annotation.gff3

**Use in IGV or other genome browsers**:

.. code-block:: bash

   # Load in IGV
   igv -g genome.fasta annotation.gff3

GenBank to PTT
~~~~~~~~~~~~~~

Create NCBI-style protein tables:

.. code-block:: bash

   genbank_to -g genome.gbk -p proteins.ptt

**PTT format** is useful for:

* Rapid protein inventory
* Genomic coordinate lookups
* Legacy NCBI pipeline compatibility

Batch Processing
----------------

Process Multiple Files
~~~~~~~~~~~~~~~~~~~~~~

Process multiple GenBank files in a loop:

.. code-block:: bash

   #!/bin/bash
   for gbk in *.gbk; do
       base=$(basename "$gbk" .gbk)
       echo "Processing $gbk..."
       genbank_to -g "$gbk" \
         -n "${base}_genome.fna" \
         -a "${base}_proteins.faa" \
         -f "${base}_functions.tsv" \
         --log "${base}.log"
   done

Parallel Processing
~~~~~~~~~~~~~~~~~~~

Use GNU parallel for faster batch processing:

.. code-block:: bash

   #!/bin/bash
   parallel genbank_to -g {} \
     -n {.}_genome.fna \
     -a {.}_proteins.faa \
     ::: *.gbk

Pseudogene Analysis
-------------------

Include Pseudogenes
~~~~~~~~~~~~~~~~~~~

By default, pseudogenes are excluded. To include them:

.. code-block:: bash

   genbank_to -g genome.gbk -a all_proteins.faa --pseudo

Compare with and without pseudogenes:

.. code-block:: bash

   # Without pseudogenes (default)
   genbank_to -g genome.gbk -a active_proteins.faa

   # With pseudogenes
   genbank_to -g genome.gbk -a all_proteins.faa --pseudo

   # Count difference
   echo "Active proteins: $(grep -c '^>' active_proteins.faa)"
   echo "All proteins: $(grep -c '^>' all_proteins.faa)"

Quality Control
---------------

Verify Output
~~~~~~~~~~~~~

Basic verification of outputs:

.. code-block:: bash

   # Run conversion
   genbank_to -g genome.gbk \
     -n genome.fna \
     -a proteins.faa \
     -p proteins.ptt

   # Verify outputs
   echo "Nucleotide sequences: $(grep -c '^>' genome.fna)"
   echo "Protein sequences: $(grep -c '^>' proteins.faa)"
   echo "PTT entries: $(($(wc -l < proteins.ptt) - 3))"  # Subtract header lines

   # Check for errors in log
   grep -i error genbank_to.log

Debugging
~~~~~~~~~

Enable debug logging for troubleshooting:

.. code-block:: bash

   genbank_to -g problematic.gbk \
     -a output.faa \
     --debug \
     --log debug.log

   # Review debug log
   less debug.log

Integration Examples
--------------------

BLAST Workflow
~~~~~~~~~~~~~~

.. code-block:: bash

   # 1. Extract proteins from genome
   genbank_to -g genome.gbk -a proteins.faa

   # 2. Create BLAST database
   makeblastdb -in proteins.faa -dbtype prot -out genome_db

   # 3. Search query against genome
   blastp -query query.faa -db genome_db \
     -outfmt 6 -out blast_results.tsv

Prokka Comparison
~~~~~~~~~~~~~~~~~

.. code-block:: bash

   # Extract from existing GenBank annotation
   genbank_to -g existing.gbk -a existing_proteins.faa

   # Run Prokka on same genome
   prokka --outdir prokka_out genome.fasta

   # Compare protein counts
   echo "Existing: $(grep -c '^>' existing_proteins.faa)"
   echo "Prokka: $(grep -c '^>' prokka_out/*.faa)"

Multiple Format Pipeline
~~~~~~~~~~~~~~~~~~~~~~~~~

Complete analysis pipeline:

.. code-block:: bash

   #!/bin/bash
   GENOME="ecoli.gbk"
   PREFIX="ecoli"

   # 1. Extract all formats
   genbank_to -g "$GENOME" \
     -n "${PREFIX}_genome.fna" \
     -a "${PREFIX}_proteins.faa" \
     -o "${PREFIX}_orfs.fna" \
     -f "${PREFIX}_functions.tsv" \
     --gff3 "${PREFIX}.gff3" \
     --amr "${PREFIX}_amr"

   # 2. Create BLAST database
   makeblastdb -in "${PREFIX}_proteins.faa" -dbtype prot

   # 3. Run AMRFinderPlus
   amrfinder \
     --protein "${PREFIX}_amr.faa" \
     --gff "${PREFIX}_amr.gff" \
     --output "${PREFIX}_amr_results.tsv"

   # 4. Summary statistics
   echo "Genome size: $(grep -v '^>' ${PREFIX}_genome.fna | wc -c)"
   echo "Proteins: $(grep -c '^>' ${PREFIX}_proteins.faa)"
   echo "ORFs: $(grep -c '^>' ${PREFIX}_orfs.fna)"
   echo "Functions: $(wc -l < ${PREFIX}_functions.tsv)"

Performance Tips
----------------

Large Files
~~~~~~~~~~~

For very large GenBank files:

.. code-block:: bash

   # Use compression to save disk space
   genbank_to -g large_genome.gbk \
     -a proteins.faa \
     --zip

   # Extract only what you need
   genbank_to -g large_genome.gbk \
     -f functions.tsv  # Only functions, not sequences

Memory-Efficient Processing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Process one sequence at a time from multi-sequence files:

.. code-block:: bash

   # Get list of sequence IDs
   grep '^LOCUS' multi.gbk | awk '{print $2}' > seq_ids.txt

   # Process each separately
   while read seqid; do
       genbank_to -g multi.gbk \
         -a "${seqid}_proteins.faa" \
         -i "$seqid"
   done < seq_ids.txt
