# Pixles API

This is GraphQL API for all Pixles clients, written in Rust, async-graphql, and SeaORM.

## Development

- Populate `.env` file
- `cargo install systemfd cargo-watch`
- `cargo install sea-orm-cli`
- `RUST_BACKTRACE=full COLORBT_SHOW_HIDDEN=1 systemfd --no-pid -s 3000 -- cargo watch -x run`
- Open <http://localhost:3000/graphql> in your browser

### Building in Docker

- Build local image: `docker build -t pixles-api:latest -f Containerfile .`
- Run local build: `docker run --env-file ./.env -p 3000:3000 pixles-api:latest`
