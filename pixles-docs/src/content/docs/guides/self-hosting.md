---
title: Self-Hosting Pixles Server
description: Get control of your data
---

Pixles was meant to be self-hosted from the start. However, with its emphasis on performance and robustness, it has several components that require careful setup. This guide will explain everything inside the `pixles-api` package which consist of sub-components for various functionality.

Skip to [Deploying](#deploying) to see quick start instructions.

## Prerequisites

Unlike other open-source projects, Pixles makes some assumptions about your environment and provides very specific instructions for setup. Assume some technical knowledge and while some technical details require some quick googling or "GPTing", the one-click installer should be sufficient for most users.

## Hardware

- Operating System: Some modern GNU/Linux distribution capable of running containers (Docker or some Kubernetes distribution)
- CPU: Both x86 and arm64 (AES-NI should be enabled, Intel QAT would help with TLS) (minimum requirements TBD)
- Memory: (minimum requirements TBD)
- Storage: (minimum requirements TBD)

## Software

Since Pixles extensively uses container technologies for both development and production, the specific software requirements are less important other than to ensure potential compatibility issues are isolated for the most common, popular target (i.e., some sort of Linux distribution with glibc). For beginners, we recommend installing the newest Ubuntu LTS server (although other distributions like Rocky Linux are officially tested on). Docker/Podman installation method would be easiest too unless you have multiple-nodes or look for service resiliency.

### Components

The Pixles API is written almost entirely in Rust with several binary components serving distinct purposes:

- [GraphQL](/pixles-api/graphql/): GraphQL API for majority of user-facing functionality. Flexible and cross-platform.
- [Upload](/pixles-api/upload/): A performant TUS-based upload service. Enables high-throughput, resumable uploads.
- [Metadata](/pixles-api/metadata/): Used for efficient metadata fetching and updating. Consists of two parts:
  - A gRPC (web) service for efficient fetching and updating metadata. We strictly prefer binary-based protocols (i.e. no JSON) for lower-serialization costs with mobile clients.
  - WebSocket + ProtoBuf service for efficient real-time updates

*Note: These components may be combined into a single web server for low-resource environments. It is used in the one-click Docker installer as well.*

External dependencies:

- [PostgresSQL](https://www.postgresql.org/): Main datastore for application data
- [Valkey](https://valkey.io/): Key-value store for caching and ephemeral data
- [MinIO](https://min.io/): Object storage
- *Various filesystems*: Various API components (e.g. `pixles-upload` and `pixles-metadata`) need direct filesystem access with various requirements (explained below).

## Deploying

### One-Click Docker Installer

Prior to installation, double check you have planned for the following requirements:

- Sufficient flash storage (e.g. 16 GB+) for temporary chunked writes for `pixles-upload`

<!-- TODO -->

### Kubernetes

*Disclaimer: This is the most robust and thoroughly-tested method. However, despite all the explicit documentation to avoid potential issues, it requires technical knowledge with Kubernetes.*

Pixles tests on vanilla Kubernetes installations. However, it is known to work on most standards-compliant Kubernetes distributions. If uncertain, we recommend using RKE2 with Cilium/Calico just because the developer uses it personally.

<!-- TODO -->
