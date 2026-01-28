Contributing
============

Thank you for your interest in contributing to ``genbank_to_rs``!

This document provides guidelines for contributing to the project.

Getting Started
---------------

Prerequisites
~~~~~~~~~~~~~

* Rust 1.70 or later
* Git
* Familiarity with GenBank format

Setting Up Development Environment
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. Fork the repository on GitHub
2. Clone your fork:

   .. code-block:: bash

      git clone https://github.com/YOUR_USERNAME/genbank_to_rs.git
      cd genbank_to_rs

3. Add upstream remote:

   .. code-block:: bash

      git remote add upstream https://github.com/linsalrob/genbank_to_rs.git

4. Build the project:

   .. code-block:: bash

      cargo build

5. Run tests:

   .. code-block:: bash

      cargo test

Development Workflow
--------------------

Branch Strategy
~~~~~~~~~~~~~~~

* ``main`` - Stable branch
* Feature branches - For new features or bug fixes

Creating a Feature Branch
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

   git checkout -b feature/your-feature-name

Making Changes
~~~~~~~~~~~~~~

1. Make your changes
2. Add tests for new functionality
3. Ensure all tests pass:

   .. code-block:: bash

      cargo test

4. Format your code:

   .. code-block:: bash

      cargo fmt

5. Run linter:

   .. code-block:: bash

      cargo clippy

6. Commit your changes:

   .. code-block:: bash

      git add .
      git commit -m "Add feature: your feature description"

Submitting Changes
~~~~~~~~~~~~~~~~~~

1. Push to your fork:

   .. code-block:: bash

      git push origin feature/your-feature-name

2. Create a Pull Request on GitHub
3. Wait for review and address feedback

Code Style
----------

Formatting
~~~~~~~~~~

Use ``rustfmt`` for consistent code formatting:

.. code-block:: bash

   cargo fmt

Configuration is in ``rustfmt.toml`` (if present) or uses defaults.

Linting
~~~~~~~

Use ``clippy`` for catching common mistakes:

.. code-block:: bash

   cargo clippy

Fix all warnings before submitting.

Naming Conventions
~~~~~~~~~~~~~~~~~~

* **Functions**: ``snake_case``
* **Types**: ``PascalCase``
* **Constants**: ``SCREAMING_SNAKE_CASE``
* **Modules**: ``snake_case``

Documentation
~~~~~~~~~~~~~

* Document all public APIs
* Use doc comments (``///``)
* Include examples where appropriate

.. code-block:: rust

   /// Parse a GenBank location string.
   ///
   /// # Arguments
   ///
   /// * `loc_str` - Location string (e.g., "123..456")
   ///
   /// # Examples
   ///
   /// ```
   /// let loc = parse_location("123..456")?;
   /// ```
   pub fn parse_location(loc_str: &str) -> Result<Location> {
       // ...
   }

Testing Guidelines
------------------

Types of Tests
~~~~~~~~~~~~~~

* **Unit Tests**: In the same file as the code
* **Integration Tests**: In ``tests/`` directory

Writing Tests
~~~~~~~~~~~~~

.. code-block:: rust

   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_parse_simple_location() {
           let loc = parse_location("123..456").unwrap();
           assert_eq!(loc.start(), 123);
           assert_eq!(loc.end(), 456);
       }
   }

Running Tests
~~~~~~~~~~~~~

Run all tests:

.. code-block:: bash

   cargo test

Run specific test:

.. code-block:: bash

   cargo test test_name

Run with output:

.. code-block:: bash

   cargo test -- --nocapture

Integration Tests
~~~~~~~~~~~~~~~~~

Integration tests use real GenBank files from ``tests/`` directory.

Add new test data conservatively - keep test files small.

Test Coverage
~~~~~~~~~~~~~

Aim for good test coverage of:

* Core parsing logic
* Output format generation
* Edge cases and error handling

Commit Guidelines
-----------------

Commit Messages
~~~~~~~~~~~~~~~

Use clear, descriptive commit messages:

* **Good**: "Add support for circular genomes in GFF3 output"
* **Bad**: "Fix bug"

Format:

.. code-block:: text

   Short summary (50 chars or less)

   More detailed explanation if needed. Wrap at 72 characters.
   Include motivation for the change and contrast with previous
   behavior.

   - Bullet points are okay
   - Use present tense: "Add feature" not "Added feature"

Commit Size
~~~~~~~~~~~

* Keep commits focused and atomic
* One logical change per commit
* Related changes can be in one commit

Pull Request Guidelines
-----------------------

PR Description
~~~~~~~~~~~~~~

Provide a clear description:

* What does this PR do?
* Why is this change needed?
* Any breaking changes?
* Related issues?

PR Template
~~~~~~~~~~~

.. code-block:: markdown

   ## Description
   Brief description of changes

   ## Motivation
   Why is this change needed?

   ## Changes
   - List of changes
   - Another change

   ## Testing
   How was this tested?

   ## Checklist
   - [ ] Tests pass
   - [ ] Code formatted (cargo fmt)
   - [ ] No clippy warnings
   - [ ] Documentation updated

Code Review
~~~~~~~~~~~

* Respond to feedback promptly
* Be open to suggestions
* Discuss significant design decisions

Areas for Contribution
----------------------

Good First Issues
~~~~~~~~~~~~~~~~~

Look for issues labeled "good first issue" on GitHub:

* Documentation improvements
* Test coverage
* Small bug fixes

Feature Requests
~~~~~~~~~~~~~~~~

* New output formats
* Performance improvements
* Additional GenBank feature support

Bug Fixes
~~~~~~~~~

* Check existing issues
* Provide test case if possible
* Fix root cause, not symptoms

Documentation
~~~~~~~~~~~~~

* Improve existing documentation
* Add examples
* Fix typos and clarity

Performance
~~~~~~~~~~~

* Profile and optimize hot paths
* Add benchmarks
* Reduce memory usage

Specific Contribution Areas
---------------------------

Adding Output Formats
~~~~~~~~~~~~~~~~~~~~~

To add a new output format:

1. Create new module in ``src/output/``
2. Implement writer function
3. Add CLI option in ``src/cli.rs``
4. Add call in ``src/main.rs``
5. Add tests
6. Update documentation

Example structure:

.. code-block:: rust

   // src/output/new_format.rs
   use anyhow::Result;
   use crate::model::Record;

   pub fn write_new_format(
       records: &[&Record],
       output: &Path,
       zip: bool,
   ) -> Result<()> {
       // Implementation
   }

Improving Parser
~~~~~~~~~~~~~~~~

To improve GenBank parsing:

* Add support for more feature types
* Handle edge cases
* Improve error messages
* Add validation

Enhancing Performance
~~~~~~~~~~~~~~~~~~~~~

* Profile with ``cargo flamegraph``
* Optimize hot paths
* Consider parallelization
* Reduce allocations

Building and Testing
--------------------

Debug Build
~~~~~~~~~~~

.. code-block:: bash

   cargo build

Release Build
~~~~~~~~~~~~~

.. code-block:: bash

   cargo build --release

Running the Tool
~~~~~~~~~~~~~~~~

.. code-block:: bash

   # Debug build
   ./target/debug/genbank_to --help

   # Release build
   ./target/release/genbank_to --help

Benchmarking
~~~~~~~~~~~~

.. code-block:: bash

   # Time execution
   time ./target/release/genbank_to -g test.gbk -a out.faa

Documentation
-------------

Building Documentation
~~~~~~~~~~~~~~~~~~~~~~

Rust API documentation:

.. code-block:: bash

   cargo doc --open

User documentation (ReadTheDocs):

.. code-block:: bash

   cd docs
   pip install sphinx sphinx-rtd-theme myst-parser
   make html

Updating Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Update docstrings for code changes
* Update user docs for new features
* Keep examples up to date

Community
---------

Communication
~~~~~~~~~~~~~

* GitHub Issues - Bug reports, feature requests
* GitHub Discussions - General questions
* Pull Requests - Code contributions

Code of Conduct
~~~~~~~~~~~~~~~

Be respectful and constructive:

* Welcome newcomers
* Be patient with questions
* Give constructive feedback
* Focus on what is best for the project

Getting Help
~~~~~~~~~~~~

* Check existing documentation
* Search existing issues
* Ask in GitHub Discussions
* Open a new issue if needed

License
-------

By contributing, you agree that your contributions will be licensed under
the MIT License.

Recognition
-----------

Contributors will be acknowledged in:

* Release notes
* CONTRIBUTORS.md (if maintained)
* Git history

Thank You!
----------

Your contributions make this project better for everyone. Thank you for
taking the time to contribute!
