# Pixles Desktop

> Disclaimer: Development for this package is paused until other clients are stabilized (e.g. web, swift, kotlin)

## Development

*Prerequisite: Kotlin development for entire monorepo is setup.*
<!-- TODO: Redirect to centralized page in pixles-docs -->

- Run app for development: `./gradlew desktop:run`
- Build native desktop distribution: `./gradlew :desktop:packageDistributionForCurrentOS`
  - Outputs are written to `build/compose/binaries`
