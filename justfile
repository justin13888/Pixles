# Pixles monorepo task runner
# Plain name = auto-fix, -check suffix = verify-only

# ── Aggregate: format ────────────────────────────────────────────────────────

[group('all')]
format: format-rust format-web format-docs format-kotlin format-vision format-swift

[group('all')]
format-check: format-check-rust format-check-web format-check-docs format-check-kotlin format-check-vision format-check-swift

# ── Aggregate: lint ──────────────────────────────────────────────────────────

[group('all')]
lint: lint-rust lint-web lint-docs lint-kotlin lint-vision lint-swift

[group('all')]
lint-check: lint-check-rust lint-check-web lint-check-docs lint-check-kotlin lint-check-vision lint-check-swift

# ── Aggregate: test ──────────────────────────────────────────────────────────

[group('all')]
test: test-rust test-web test-kotlin

[group('all')]
test-coverage: test-coverage-rust

# ── Aggregate: build ─────────────────────────────────────────────────────────

[group('all')]
build: build-rust build-web build-docs build-kotlin

# ── Aggregate: check (CI gate) ───────────────────────────────────────────────

[group('all')]
check: format-check lint-check test

# ── Rust ─────────────────────────────────────────────────────────────────────

[group('rust')]
format-rust:
    cargo fmt

[group('rust')]
format-check-rust:
    cargo fmt --check

[group('rust')]
lint-rust:
    cargo clippy --workspace --fix --allow-dirty

[group('rust')]
lint-check-rust:
    cargo clippy --workspace -- -D warnings

[group('rust')]
test-rust:
    cargo test --workspace

[group('rust')]
test-coverage-rust:
    cargo llvm-cov --workspace --fail-under-lines 0

[group('rust')]
build-rust:
    cargo build --workspace

# ── Web ──────────────────────────────────────────────────────────────────────

[group('web')]
format-web:
    cd pixles-web && bunx biome format --write .

[group('web')]
format-check-web:
    cd pixles-web && bunx biome format .

[group('web')]
lint-web:
    cd pixles-web && bunx biome check --write .

[group('web')]
lint-check-web:
    cd pixles-web && bunx biome check .

[group('web')]
test-web:
    cd pixles-web && bun test

[group('web')]
build-web:
    cd pixles-web && bun run build

# ── Docs ─────────────────────────────────────────────────────────────────────

[group('docs')]
format-docs:
    cd pixles-docs && bunx biome format --write .

[group('docs')]
format-check-docs:
    cd pixles-docs && bunx biome format .

[group('docs')]
lint-docs:
    cd pixles-docs && bunx biome check --write .

[group('docs')]
lint-check-docs:
    cd pixles-docs && bunx biome check .

[group('docs')]
build-docs:
    cd pixles-docs && bun run build

# ── Kotlin ───────────────────────────────────────────────────────────────────

[group('kotlin')]
format-kotlin:
    ./gradlew ktlintFormat

[group('kotlin')]
format-check-kotlin:
    ./gradlew ktlintCheck

[group('kotlin')]
lint-kotlin:
    ./gradlew detekt

[group('kotlin')]
lint-check-kotlin:
    ./gradlew detekt

[group('kotlin')]
test-kotlin:
    ./gradlew test

[group('kotlin')]
build-kotlin:
    ./gradlew build

# ── Swift ────────────────────────────────────────────────────────────────────

[group('swift')]
format-swift:
    #!/usr/bin/env bash
    if [ "$(uname)" != "Darwin" ]; then
        echo "Skipping swift format (not macOS)"
        exit 0
    fi
    cd pixles-swift && swift run -c release --package-path BuildTools swiftformat .

[group('swift')]
format-check-swift:
    #!/usr/bin/env bash
    if [ "$(uname)" != "Darwin" ]; then
        echo "Skipping swift format check (not macOS)"
        exit 0
    fi
    cd pixles-swift && swift run -c release --package-path BuildTools swiftformat --lint .

[group('swift')]
lint-swift:
    #!/usr/bin/env bash
    if [ "$(uname)" != "Darwin" ]; then
        echo "Skipping swiftlint (not macOS)"
        exit 0
    fi
    cd pixles-swift && swiftlint

[group('swift')]
lint-check-swift:
    #!/usr/bin/env bash
    if [ "$(uname)" != "Darwin" ]; then
        echo "Skipping swiftlint check (not macOS)"
        exit 0
    fi
    cd pixles-swift && swiftlint

# ── Vision ───────────────────────────────────────────────────────────────────

[group('vision')]
format-vision:
    cd pixles-vision && uv run ruff format

[group('vision')]
format-check-vision:
    cd pixles-vision && uv run ruff format --check

[group('vision')]
lint-vision:
    cd pixles-vision && uv run ruff check --fix

[group('vision')]
lint-check-vision:
    cd pixles-vision && uv run ruff check && uv run ty check

# ── Setup ────────────────────────────────────────────────────────────────────

[group('setup')]
hooks-install:
    lefthook install

[group('setup')]
hooks-uninstall:
    lefthook uninstall

[group('setup')]
install:
    cd pixles-web && bun install
    cd pixles-docs && bun install
    cd pixles-vision && uv sync
