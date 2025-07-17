# Pixles CLI

CLI tool for Pixles API. Primarily used for server owners, advanced users, and development.

It is a thin CLI wrapper. It uses `pixles-core-rust` for core capabilities such as uploading, downloading, among others. This CLI tool is entirely stateless for simplicity.

## Getting Started

Binaries are compiled to:

- Windows 7+ (via MinGW)
- MacOS (Arm64 + X64)
- Linux (x86_64 + aarch64)

For now, CLI has not been packaged with package managers but could be compiled from source.

<!-- TODO: Distribute via GitHub packages and package managers -->

## Development

- `cargo run -- <args>` to run the CLI with arguments
- `cargo test` to run tests
- `cargo build --release` to build the release binary
