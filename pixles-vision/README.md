# pixles-vision

This package implements experiemental computer vision features for Pixles (not all will be productionized immediately). There is both training and export scripts, as well as notebooks for exploring new model capabilities relevant to the project.

## Setup

Prerequisite: This project uses [rye](https://github.com/astral-sh/rye#installation) for ease of Python dependency management.

```bash
# Sync dependencies
rye sync
# Activate venv
. .venv/bin/activate
```

Other common commands include:

- Format: `rye fmt`
- Lint: `rye lint`
- Check: `rye check`
- Run tests: `rye run pytest`
- Run notebook: `rye run jupyter notebook`
