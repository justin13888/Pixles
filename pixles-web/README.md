# Pixles App

This is a fully-featured web client for Pixles. It is built using React, Rsbuild, Tailwind CSS, Tanstack, and more.

## Development

### Prerequisites

- Install Bun
- Get `pixles-api` setup and development server running (see [pixles-api/README.md](../pixles-api/README.md))

### Starting

1. Run

    ```bash
    # Install dependencies
    bun install
    # Generate GraphQL types
    bun run codegen:watch
    # Run development server
    bun dev
    # Build production build
    bun run build
    # Preview production build locally
    bun run preview

    # After graphql queries are changed (if not using watch mode)
    bun run codegen
    ```

2. Open <http://localhost:5173/> with your browser to see the result.
