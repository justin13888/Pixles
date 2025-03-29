# Pixles

Photo sharing for all! Or you could think of it as what Google Photo wanted to be.

## Features

- **Cross-platform**: Pixles is available on all common desktop and mobile platforms. They're fast on all.
- **Broadest format support**: Pixles supports the majority of image and video formats from ones in common smartphones to professional RAW formats. View any content on any device just like your smartphone photos and videos!
- **Privacy**: Your data is yours and end-to-end encrypted.
- **Asset-first**: Pixles implements several powerful features like real-time viewing, semantic search, AI organization, and more.
- **Open-source**: Pixles is open-source forever and you can host your own server.

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

- [Pixles API](pixles-api/README.md)
- [Pixles Web](pixles-web/README.md) (WIP)
- [Pixles Desktop](pixles-desktop/README.md) (Planned)
- [Pixles Android](pixles-android/README.md) (Planned)
- [Pixles Swift](pixles-swift/README.md) (WIP)
- [Pixles Media](pixles-media/README.md) (WIP)
- [Pixles Docs](pixles-docs/README.md)

<!-- TODO: TO be updated ^^ -->

External dependencies:

- [PostgreSQL](https://www.postgresql.org/)
- [Minio](https://min.io/)
- [RabbitMQ](https://www.rabbitmq.com/)
- [Memcached](https://memcached.org/)

<!-- TODO: To be updated ^^ -->

Considering all the technologies used, you may have to switch between IDEs to develop various parts of the project. This is what I recommend:

- `pixles-android`: Android Studio or IntelliJ IDEA with plugins
- `pixles-api`: VS Code or similar
- `pixles-core-kotlin`: Android Studio or IntelliJ IDEA with plugins
- `pixles-desktop`: VS Code or similar
- `pixles-docs`: VS Code or similar
- `pixles-media`: VS Code or similar
- `pixles-swift`: Xcode
- `pixles-web`: VS Code or similar

Reference the Development sections of each component's README for more information.

<!-- TODO: Add internationalization note -->

## FAQ

**Q: Why may Pixles be more suitable than other open-source solutions?**

A: Pixles is designed from the ground up with performance, usability, and compatibility in mind. While hosting requires some initial setup (all of which is carefully documented), we have by far the most comprehensive format support, real-time viewing capabilities. We thoroughly test the supported hardware and software combinations and conservatively push new features to stable. It should be a great option for those with large amounts of content and want a single pane of glass to manage all their assets from any device.

**Q: Why not extend off existing open-source solutions?**

A: While there are multiple great open-source solutions, they lack a lot of the involved functions that professionals and prosumers need. For prosumers interested in an open-source and self-hosted solution, we have a robust, and highly scalable solution. For professionals looking to host all their assets in a seamless and integrated service, we have a solution that may be a better fit than some proprietary options.

Side note: The original author loves open-source and has contributed to various projects. The reason for starting from the ground up is that many of the technical decisions to achieve the goals with user experience and performance require multiple critical design decisions.

## License

Pixles is licensed under the [AGPL-3.0 License](LICENSE).
