# pixles-vision

This package implements experiemental computer vision features for Pixles (not all will be productionized immediately). There is both training and export scripts, as well as notebooks for exploring new model capabilities relevant to the project.

## Setup

Prerequisite:
- This project uses [rye](https://github.com/astral-sh/rye#installation) for ease of Python dependency management.
- Since many ML packages are platform-specific, we assume development is done on Linux.

```bash
# Activate venv
. .venv/bin/activate

# Sync dependencies
rye sync --no-lock # CUDA, CPU
# or
rye sync --features rocm # If using ROCm

# ROCm in WSL only
location="./.venv/lib/python3.12/site-packages" # Verify this path
cd ${location}/torch/lib/
rm libhsa-runtime64.so*

# If using ROCm, need to re-install ROCm-compatible PyTorch manually
# Note: AMD ROCm documentation may need to be modified to work with rye instead of pip
# Follow: <https://rocm.docs.amd.com/projects/radeon/en/latest/docs/install/native_linux/install-pytorch.html>
```

Other common commands include:

- Format: `rye fmt`
- Lint: `rye lint`
- Check: `rye check`
- Run tests: `rye run pytest`
- Run notebook: `rye run jupyter notebook`
