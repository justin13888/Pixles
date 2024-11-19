# Pixles API

This is GraphQL API for all Pixles clients, written in Rust, async-graphql, and SeaORM.

## Development

- Populate `.env` file
- `cargo install systemfd cargo-watch`
- `cargo install sea-orm-cli`
- `systemfd --no-pid -s 3000 -- cargo watch -x run`
- Open `http://localhost:3000/graphql` in your browser
