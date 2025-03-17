# Pixles API

This is API service for all Pixles clients, written in Rust.

There are currently two servable components:

- `graphql`: GraphQL API written with `async-graphql`
- `upload`: Dedicated, optimized upload server

<!-- TODO: Elaborate more on the responsibilities, goals, technical requirements of each component -->

They can be packaged together or separately (recommended for production).

## Development

### Prerequisites

*We assume Linux-based system for this service.*

- Rust 1.86+ 
- Populate `.env` file based on `.env.example`
- `cargo install systemfd cargo-watch`
- `cargo install sea-orm-cli`
- Docker and Docker Compose

### Running

- Spin up some dependencies: `docker compose up` (could spin up individual services manually if needed)
    - Remove existing data: `docker compose down -v`
- Start development server: `RUST_BACKTRACE=1 COLORBT_SHOW_HIDDEN=1 systemfd --no-pid -s 3000 -- cargo watch -x run`
    - *Append feature flags to enable specific parts of server*
- The following endpoints should be up:
    - GraphQL: <http://localhost:3000/graphql>
        - GraphiQL: <http://localhost:3000/playground>
    - Upload: <http://localhost:3000/upload>

### Building in Docker

- Build local image: `docker build -t pixles-api:latest -f Containerfile .`
- Run local build: `docker run --env-file ./.env -p 3000:3000 pixles-api:latest`
