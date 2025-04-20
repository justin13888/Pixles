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

## Design

### gRPC + Protocol Buffers

- **Used for:** Bulk metadata sync, initial sync, delta updates  
- **Why:**  
  - 60–80% smaller payloads than JSON  
  - Highly efficient for syncing thousands of records  
  - Strongly typed to reduce data corruption  

### GraphQL

- **Used for:** UI queries, search, and filtering  
- **Why:**  
  - Ideal for flexible UI data needs  
  - Reduces API iteration cycles  
  - Great developer experience for frontend teams  

### HTTP + TUS Protocol

- **Used for:** Uploading and downloading original photo assets  
- **Why:**  
  - Resume-capable uploads for poor networks  
  - CDN-compatible  
  - Built for large file transfers  

### WebSockets + Protocol Buffers

- **Used for:** Real-time sync status, presence, notifications  
- **Why:**  
  - Efficient binary messaging  
  - Reuses existing protobuf models  
  - Low-latency delivery for system-level events  

### GraphQL Subscriptions

- **Used for:** User-facing real-time events (comments, shares, collab)  
- **Why:**  
  - Easy to use in UI clients  
  - Strong typing  
  - Filtering and selective subscriptions  

### Offline-First Architecture

- **Used for:** Local caching, editing, and sync  
- **Why:**  
  - Guarantees smooth experience regardless of connectivity  
  - Local-first UX with background resolution and merge  
