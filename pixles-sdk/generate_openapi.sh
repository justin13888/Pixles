#!/bin/bash
set -e

# Navigate to pixles-api directory
cd "$(dirname "$0")/../pixles-api"

# Run the gen_openapi binary
# output path is relative to pixles-api, so ../pixles-sdk/openapi.json
cargo run --bin gen_openapi --features full -- ../pixles-sdk/openapi.json
