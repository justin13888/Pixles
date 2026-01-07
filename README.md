# Pixles

Open Asset Management Scaled to Millions.

> Disclaimer: This project continues to be in active development. Star this repo to get the latest updates!

## Features

- **Cross-platform**: Pixles is available on all common desktop and mobile platforms. They're fast on all.
- **Broadest format support**: Pixles supports the majority of image and video formats from ones in common smartphones to professional RAW formats. View any content on any device just like your smartphone photos and videos!
- **Privacy**: Your data is yours and end-to-end encrypted.
- **Asset-first**: Pixles implements several powerful features like real-time viewing, semantic search, AI organization, and more.
- **Open-source**: Pixles is open-source forever and you can host your own server.

<!-- TODO: Update this -->

## Screenshots

<!-- TODO: Add screenshots -->

## Who is Pixles for?

- **Photographers**: Pixles is designed for photographers who want to store and share their photos with clients and peers.
- **Families and friends**: Pixles is designed for prosumers who want to store and share their photos and videos with each other at full quality, complete with metadata.
- **Organizations**: Organizations can use Pixles to share full-quality photos and videos with their members and clients.
- **People who care about privacy**: Pixles implements the best of privacy and security practices and leaves the data in your hands.

### Who is Pixles not for?

This is a personal choice but if you're happy with existing services like Google Photos or iCloud, or sending highly compressed content over messaging apps, Pixles might not be for you.

## Some similar alternatives

- **Google Photos or similar**: Google Photos is a great service for storing and sharing photos and videos. However, it compresses and strips metadata from your photos and videos by default, and does not support many more professional or non-smartphone formats.
- **AirDrop, Quick Share or some messaging app**: These options are great for sharing photos and videos quickly, but they compress the content, have size limits, and/or do not store it for long-term access. If you have more than a few gigabytes of content, Pixles should offer a much more comfortable experience.

## Getting Started

Pixles is available on all common platforms. We expect the smoothest possible experience whether you are uploading straight from your phone or uploading from your dedicated cameras (e.g. mirrorless, GoPro, drone, cinema cameras).

<!-- Install any of the following clients for your use case:

- **Desktop**: [Download for Windows](#) | [Download for macOS](#) | [Download for Linux](#)
- **Mobile**: [Download for Android](#) | [Download for iOS](#)
- **Web**: [Open in browser](#)

### Self-hosting

Pixles is open-source and designed to be friendly to self-host. See this [guide](#) for more information. -->

<!-- TODO -->

## Development

<!-- TODO: Add complete architecture diagram -->

Components:

- [Pixles API](pixles-api/README.md): Various API services (HTTP, gRPC, GraphQL, WebSockets, etc.)
- [Pixles Web](pixles-web/README.md) (WIP): Web client in React
- [Pixles Core Kotlin](pixles-core-kotlin/README.md): Shared core Kotlin multiplatform library for client-specific logic
- [Pixles Desktop](pixles-desktop/README.md) (Planned): Windows/Linux desktop client
- [Pixles Android](pixles-android/README.md) (WIP): Jetpack Compose App
- [Pixles Swift](pixles-swift/README.md): SwiftUI client for iOS/macOS
- [Pixles Media](pixles-media/README.md) (Beta): C++ library for certain offloading
- [Pixles Docs](pixles-docs/README.md): Documentation website in Starlight (Astro)

<!-- TODO: ensure readme links work ^^ -->
<!-- TODO: TO be updated ^^ -->

External dependencies:

- [PostgreSQL](https://www.postgresql.org/)
- [MinIO](https://min.io/)
- [RabbitMQ](https://www.rabbitmq.com/)
- [Memcached](https://memcached.org/)

- [Envoy](https://github.com/envoyproxy/envoy)
- [Istio](https://github.com/istio/istio)

<!-- TODO: To be updated ^^ -->

Considering all the technologies used, you may have to switch between IDEs to develop various parts of the project. This is what we recommend:

- `pixles-android`: Android Studio or IntelliJ IDEA with plugins
- `pixles-api`: VS Code or similar
- `pixles-core-kotlin`: Android Studio or IntelliJ IDEA with plugins
- `pixles-desktop`: VS Code or similar
- `pixles-docs`: VS Code or similar
- `pixles-media`: VS Code or similar
- `pixles-swift`: Xcode
- `pixles-web`: VS Code or similar

<!-- TODO: Update list of components ^^ -->

### Setup

We use [Bazel](https://bazel.build) as our build system due to its strong multi-language support. As of writing, there are 5+ programming languages used (Rust, TypeScript, Kotlin, Swift/Objective-C, C/C++, Python). Various tools will need to be setup based on services you need to work on.

Setup in the following order:

- Install [Bazelisk](https://bazel.build/install/bazelisk) for development.
- Setup all necessary tools related to Kotlin Multiplatform: <https://www.jetbrains.com/help/kotlin-multiplatform-dev/multiplatform-setup.html>
- Setup each of the following tools in the Development sections of each component's README.

### Style and Guidelines

- Due to the numerous languages in this monorepo, we use multiple linters/formatters, each native to each language/technology. CI/CD will enforce these and it is recommended to use the same tools in the IDE of your choice to reduce merge conflicts. (Also, all code is standardized to 4 spaces as some languages have specific guidelines while others (e.g. TypeScript) have mixed guides.)

<!-- TODO: Add internationalization note -->

## FAQ

**Q: Why may Pixles be more suitable than other open-source solutions?**

A: Pixles is designed from the ground up with performance, usability, and compatibility in mind. While hosting requires some initial setup (all of which is carefully documented), we have by far the most comprehensive format support, real-time viewing capabilities. We thoroughly test the supported hardware and software combinations and conservatively push new features to stable. It should be a great option for those with large amounts of content and want a single pane of glass to manage all their assets from any device.

**Q: Why not extend off existing open-source solutions?**

A: While there are multiple great open-source solutions, they lack a lot of the involved functions that professionals and prosumers need. For prosumers interested in an open-source and self-hosted solution, we have a robust, and highly scalable solution. For professionals looking to host all their assets in a seamless and integrated service, we have a solution that may be a better fit than some proprietary options.

Side note: The original author loves open-source and has contributed to various projects. The reason for starting from the ground up is that many of the technical decisions to achieve the goals with user experience and performance require multiple critical design decisions.
**Q: For the API, were languages other than Rust considered?**

A: Yes, we considered many languages. Some other languages considered included Go, TypeScript, Kotlin/Java. In fact, the first PoC was as a single REST API written in TypeScript. However, the current development has developed into multiple APIs (GraphQL, REST, gRPC) and processing logic offloaded to clients of various platforms. Rust offers both the memory-safety and performance requirements, as well as the cross-platform flexiblity that some other languages may equally excel at. On the APIs, Rust libraries also tend to be newer and allowed for Linux-specific optimizations such as using `io_uring` for high-performance async I/O. Additonally, note that several other languages with other strengths are embraced.

**Q: How do bugfixes happen?**

A: Even if the best development practices, rigorous testing, and conservative designs, no software is without bugs. Bug reporting and thoughtful feature requests submitted to the issues page would be much appreciated. Certain types of bugs, such as those affecting data integrity, should be marked with the appropriate tags and will be ironed out ASAP. Patches typically roll out to the latest major version and marked in a notice board separate from CHANGELOGs.

## How to contribute

Pixles primarily benefits from active contributions and feedback. See [CONTRIBUTING.md](./CONTRIBUTING.md) for details.

## License

Pixles is licensed under the [AGPL-3.0 License](LICENSE).
