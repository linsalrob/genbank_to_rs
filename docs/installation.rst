Installation
============

This guide covers different methods for installing ``genbank_to_rs``.

Prerequisites
-------------

* Rust 1.70 or later
* Cargo (Rust's package manager, included with Rust)

Installing Rust
~~~~~~~~~~~~~~~

If you don't have Rust installed, install it using rustup:

.. code-block:: bash

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

For Windows, download and run `rustup-init.exe <https://rustup.rs/>`_.

After installation, verify Rust is installed:

.. code-block:: bash

   rustc --version
   cargo --version

Installation Methods
--------------------

From Source (Recommended)
~~~~~~~~~~~~~~~~~~~~~~~~~

Clone the repository and build from source:

.. code-block:: bash

   git clone https://github.com/linsalrob/genbank_to_rs.git
   cd genbank_to_rs
   cargo build --release

The binary will be located at ``./target/release/genbank_to``.

You can then copy it to a location in your PATH:

.. code-block:: bash

   # Linux/macOS
   sudo cp target/release/genbank_to /usr/local/bin/

   # Or add to your user bin
   cp target/release/genbank_to ~/.local/bin/

Using Cargo Install
~~~~~~~~~~~~~~~~~~~

Install directly using cargo:

.. code-block:: bash

   cargo install --path .

This will install the ``genbank_to`` binary to ``~/.cargo/bin/``, which should be in your PATH.

Development Installation
~~~~~~~~~~~~~~~~~~~~~~~~

For development work, clone the repository and use cargo for development builds:

.. code-block:: bash

   git clone https://github.com/linsalrob/genbank_to_rs.git
   cd genbank_to_rs
   cargo build

The development binary will be at ``./target/debug/genbank_to``.

Verification
------------

Verify the installation by checking the version:

.. code-block:: bash

   genbank_to --version

You should see output like:

.. code-block:: text

   genbank_to 0.1.0

Try the help command to see all available options:

.. code-block:: bash

   genbank_to --help

System Requirements
-------------------

* **Disk Space**: ~50 MB for the binary and dependencies
* **Memory**: Minimal, typically < 100 MB for most GenBank files
* **CPU**: Any modern x86_64 or ARM64 processor

Supported Platforms
-------------------

* **Linux**: x86_64, ARM64
* **macOS**: x86_64 (Intel), ARM64 (Apple Silicon)
* **Windows**: x86_64

Troubleshooting
---------------

Rust Compilation Errors
~~~~~~~~~~~~~~~~~~~~~~~~

If you encounter compilation errors, ensure you have the latest stable Rust version:

.. code-block:: bash

   rustup update stable

Missing Dependencies
~~~~~~~~~~~~~~~~~~~~

On some Linux systems, you may need to install additional dependencies:

.. code-block:: bash

   # Debian/Ubuntu
   sudo apt-get install build-essential pkg-config

   # RHEL/CentOS/Fedora
   sudo yum install gcc pkg-config

Permission Denied
~~~~~~~~~~~~~~~~~

If you get permission errors when installing to system directories, either:

1. Use sudo (Linux/macOS):

   .. code-block:: bash

      sudo cargo install --path .

2. Install to user directory without sudo:

   .. code-block:: bash

      cargo install --path . --root ~/.local

Updating
--------

To update to the latest version:

.. code-block:: bash

   cd genbank_to_rs
   git pull
   cargo install --path . --force

The ``--force`` flag will overwrite the existing installation.
