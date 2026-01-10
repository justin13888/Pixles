# Pixles SDK

SDK for Pixles API. Assess all Pixles APIs statelessly via one library only. Note this SDK currently is for Rust and rather than supporting other languages via bindings, we recommend generating the respective OpenAPI, GraphQL, gRPC, etc. clients via the coresponding API specifications you need with tools from the native language.

## APIs Supported

- [Auth](../pixles-api/auth/README.md)
<!-- - [GraphQL](../pixles-api/graphql/README.md)
- [Upload](../pixles-api/upload/README.md)
- [Metadata](../pixles-api/metadata/README.md) -->

## Development

- Generate OpenAPI spec: `./generate_openapi.sh`
  - [Progenitor](https://github.com/oxidecomputer/progenitor) is used to generate the SDK as long as the OpenAPI spec is provided at the assumed path.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pixles-sdk = "0.1"
```
