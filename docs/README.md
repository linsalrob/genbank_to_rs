# Documentation for genbank_to_rs

This directory contains the documentation for genbank_to_rs, built with Sphinx and hosted on Read the Docs.

## Building Documentation Locally

### Prerequisites

Install Python and the required packages:

```bash
pip install -r requirements.txt
```

### Build HTML Documentation

```bash
make html
```

The documentation will be generated in `_build/html/`. Open `_build/html/index.html` in your browser.

### Build PDF Documentation

```bash
make latexpdf
```

Requires LaTeX installation.

### Clean Build Files

```bash
make clean
```

## Documentation Structure

- `index.rst` - Main documentation entry point
- `installation.rst` - Installation guide
- `usage.rst` - Usage guide with command-line reference
- `formats.rst` - Output format specifications
- `examples.rst` - Comprehensive examples
- `architecture.rst` - Internal architecture documentation
- `api.rst` - API reference for developers
- `contributing.rst` - Contributing guidelines
- `changelog.rst` - Version history
- `license.rst` - License information

## Configuration

- `conf.py` - Sphinx configuration
- `requirements.txt` - Python dependencies for building docs
- `.readthedocs.yaml` - Read the Docs configuration (in project root)

## Read the Docs

The documentation is automatically built and hosted on Read the Docs when changes are pushed to the repository.

Visit: https://genbank-to-rs.readthedocs.io/ (once configured)

## Contributing to Documentation

To contribute to the documentation:

1. Edit the relevant `.rst` files
2. Build locally to verify changes
3. Submit a pull request

For more information, see `contributing.rst`.
