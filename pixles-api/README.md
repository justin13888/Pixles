# Pixles API

This is API service for all Pixles clients, written in Rust.

There are multiple servable components (built together for development but separately for production):

- `graphql`: GraphQL API written for Pixles GUI clients
- `upload`: High-performance, asynchronous upload server, with [TUS](https://tus.io/protocols/resumable-upload), active session management, and metadata+preview processing
- `metadata`: gRPC-based API for high-volume metadata fetching

<!-- TODO: Elaborate more on the responsibilities, goals, technical requirements of each component -->

They can be packaged together or separately (recommended for production).

## Development

### Prerequisites

_We assume Linux-based system for this service._

- Rust 1.86+
- Populate `.env` file based on `.env.example`
- `cargo install systemfd cargo-watch`
- `cargo install sea-orm-cli`
- Podman
  - Note: Most OCI runtimes should work identically in theory but our recommended deployment methods are Kubernetes and Podman.
- Protobuf compiler

  ```bash
  # Ubuntu/Debian
  sudo apt update && sudo apt upgrade -y
  sudo apt install -y protobuf-compiler libprotobuf-dev
  ```

  ```bash
  # Arch Linux
  sudo pacman -S protobuf
  ```

  ```bash
  # macOS
  brew install protobuf
  ```

### Running

- Spin up some dependencies: `podman compose up` (could spin up individual services manually if needed)
  - Remove existing data: `podman compose down -v`
- Start development server: `RUST_BACKTRACE=1 COLORBT_SHOW_HIDDEN=1 systemfd --no-pid -s 3000 -- cargo watch -x run`
  - _Append feature flags to enable specific parts of server_
- The following endpoints should be up:
  - Auth: <http://localhost:3000/auth>
  - GraphQL: <http://localhost:3000/graphql>
    - GraphiQL: <http://localhost:3000/playground>
  - Upload: <http://localhost:3000/upload>
  - Metadata: <http://localhost:3000/metadata>
  - OpenAPI: <http://localhost:3000/openapi>

### Building with Podman

_Note: These commands usually work similarly across other OCI tools like Podman/Docker. But prefer building with containerd._

- Build local image: `podman build -t pixles-api:latest -f Containerfile .`
- Run local build: `podman run --network host --env-file ./.env pixles-api:latest`
