---
title: Architecture
description: How Pixles combines gRPC, GraphQL, TUS, and more to deliver a high-performance and robust API.
---

Pixles is a cross-platform photo service designed for professional and enthusiast photographers who expect fast syncing, seamless uploads, and powerful search—regardless of device or network conditions. We need a high-performance backend with a modern, developer-friendly interface for building rich client apps on all platforms.

To achieve this, Pixles employs a **hybrid API strategy** that balances performance, flexibility, and reliability. Here's a breakdown of the core technology decisions and why they were made.

## Requirements

- **Performance Optimization:** Each data channel should use the best-fit protocol:
- **Developer Experience:** UI and backend teams can move independently with tools tailored to their workflows.
- **Cross-Platform Consistency:** Various data models should be serializable and deserializable across platforms.
- **Network Efficiency:** Use binary formats where necessary to reduce payload size and energy use, especially on mobile.
- **Scalability:** Decoupled subsystems (sync, search, uploads, UI) with differing performance requirements and domains must be able to scale independently.

## Technology Stack

| Technology | Use Case | Benefits |
|------------|----------|----------|
| **gRPC + Protocol Buffers** | Bulk metadata sync, initial sync, delta updates | - 60–80% smaller payloads than JSON<br>- Highly efficient for syncing thousands of records<br>- Strongly typed to reduce data corruption |
| **GraphQL** | UI queries, search, and filtering | - Ideal for flexible UI data needs<br>- Reduces API iteration cycles<br>- Great developer experience for frontend teams |
| **HTTP + TUS Protocol** | Uploading and downloading original photo assets | - Resume-capable uploads for poor networks<br>- CDN-compatible<br>- Built for large file transfers |
| **WebSockets + Protocol Buffers** | Real-time sync status, presence, notifications | - Efficient binary messaging<br>- Reuses existing protobuf models<br>- Low-latency delivery for system-level events |
| **GraphQL Subscriptions** | User-facing real-time events (comments, shares, collab) | - Easy to use in UI clients<br>- Strong typing<br>- Filtering and selective subscriptions |
| **Offline-First Architecture** | Local caching, editing, and sync | - Guarantees smooth experience regardless of connectivity<br>- Local-first UX with background resolution and merge |
