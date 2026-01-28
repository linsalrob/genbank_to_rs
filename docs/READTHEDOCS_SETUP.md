# Setting Up ReadTheDocs for genbank_to_rs

This guide explains how to set up ReadTheDocs for this project.

## Prerequisites

1. A GitHub account with access to this repository
2. A ReadTheDocs account (https://readthedocs.org/)

## Setup Steps

### 1. Connect GitHub to ReadTheDocs

1. Go to https://readthedocs.org/
2. Sign in with your GitHub account
3. Authorize ReadTheDocs to access your GitHub repositories

### 2. Import the Project

1. Click on "Import a Project" or go to https://readthedocs.org/dashboard/import/
2. Click "Import Manually"
3. Fill in the project details:
   - **Name**: `genbank-to-rs`
   - **Repository URL**: `https://github.com/linsalrob/genbank_to_rs`
   - **Repository type**: Git
   - **Default branch**: `main`
4. Click "Next"

### 3. Configure the Project (Optional)

The project is already configured via `.readthedocs.yaml` in the repository root, so no additional configuration should be needed. However, you can verify:

1. Go to the project settings
2. Check that "Build documentation" is enabled
3. Verify Python version is set to 3.12 (configured in `.readthedocs.yaml`)

### 4. Build Documentation

1. ReadTheDocs will automatically build documentation on each push to the repository
2. You can manually trigger a build from the project dashboard
3. The documentation will be available at: `https://genbank-to-rs.readthedocs.io/`

### 5. Configure Webhooks (Automatic)

ReadTheDocs automatically sets up webhooks for your repository, so documentation will rebuild on each push.

### 6. Enable Versions (Optional)

1. Go to "Versions" in the project settings
2. Enable builds for specific tags or branches as needed
3. Set the default version (typically "latest" or "stable")

## Troubleshooting

### Build Fails

Check the build log on ReadTheDocs:
1. Go to the project dashboard
2. Click on "Builds"
3. Click on the failed build to see the log

Common issues:
- Missing dependencies: Check `docs/requirements.txt`
- Sphinx errors: Test locally with `cd docs && make html`
- Configuration errors: Verify `.readthedocs.yaml`

### Local Testing

Before pushing changes, test the documentation locally:

```bash
cd docs
pip install -r requirements.txt
make html
# Open _build/html/index.html in a browser
```

### Clean Build

If you see caching issues:

```bash
cd docs
make clean
make html
```

## Files Involved

- `.readthedocs.yaml` - Main ReadTheDocs configuration
- `docs/conf.py` - Sphinx configuration
- `docs/requirements.txt` - Python dependencies for building docs
- `docs/*.rst` - Documentation content
- `docs/Makefile` - Build automation for Unix/Linux/macOS
- `docs/make.bat` - Build automation for Windows

## Badge (Optional)

Add a ReadTheDocs badge to your README:

```markdown
[![Documentation Status](https://readthedocs.org/projects/genbank-to-rs/badge/?version=latest)](https://genbank-to-rs.readthedocs.io/en/latest/?badge=latest)
```

## Custom Domain (Optional)

If you want to use a custom domain:

1. Go to Admin → Domains
2. Add your custom domain
3. Follow the DNS configuration instructions

## Support

- ReadTheDocs Documentation: https://docs.readthedocs.io/
- Support: https://docs.readthedocs.io/en/stable/support.html
- Community: https://github.com/readthedocs/readthedocs.org/discussions
