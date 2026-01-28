License
=======

MIT License
-----------

Copyright (c) 2024 Edwards Lab

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Third-Party Licenses
--------------------

This project uses several open-source dependencies. Each dependency has its own
license:

Core Dependencies
~~~~~~~~~~~~~~~~~

* **clap** (Apache-2.0 or MIT) - Command-line argument parsing
* **gb-io** (MIT) - GenBank file parsing
* **anyhow** (Apache-2.0 or MIT) - Error handling
* **tracing** (MIT) - Logging framework
* **tracing-subscriber** (MIT) - Logging implementation

Utility Dependencies
~~~~~~~~~~~~~~~~~~~~

* **flate2** (Apache-2.0 or MIT) - Gzip compression
* **serde** (Apache-2.0 or MIT) - Serialization framework
* **serde_json** (Apache-2.0 or MIT) - JSON support
* **md5** (Apache-2.0 or MIT) - MD5 hashing
* **regex** (Apache-2.0 or MIT) - Regular expressions
* **itertools** (Apache-2.0 or MIT) - Iterator utilities

Development Dependencies
~~~~~~~~~~~~~~~~~~~~~~~~

* **insta** (Apache-2.0) - Snapshot testing
* **tempfile** (Apache-2.0 or MIT) - Temporary file handling

For complete license information for each dependency, please refer to their
respective repositories or the Cargo.lock file in this project.

Citation
--------

If you use this tool in your research, please cite:

Edwards, R. A. (2022). genbank_to: Convert NCBI GenBank files to various formats.
https://github.com/linsalrob/genbank_to

For the Rust implementation specifically:

Edwards Lab (2024). genbank_to_rs: Rust implementation of genbank_to.
https://github.com/linsalrob/genbank_to_rs
