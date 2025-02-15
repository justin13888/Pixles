# Pixles Media

High-level, cross-platform image and video processing library.

<!-- TODO: WIP -->

## Features

<!-- TODO -->

## Supported Formats

<!-- TODO -->

## Development

Prerequisites:

- Conan: `pip install conan`
- `sudo apt install libva-dev libvdpau-dev libx11-dev libx11-xcb-dev libfontenc-dev libice-dev libsm-dev libxau-dev libxaw7-dev libxkbfile-dev libxmuu-dev libxres-dev libxv-dev libxxf86vm-dev libxcb-glx0-dev libxcb-render-util0-dev libxcb-xkb-dev libxcb-icccm4-dev libxcb-image0-dev libxcb-keysyms1-dev libxcb-randr0-dev libxcb-shape0-dev libxcb-sync-dev libxcb-xfixes0-dev libxcb-xinerama0-dev libxcb-dri3-dev libxcb-cursor-dev libxcb-dri2-0-dev libxcb-dri3-dev libxcb-present-dev libxcb-composite0-dev libxcb-ewmh-dev libxcb-res0-dev libxcb-util-dev libxcb-util0-dev`

Install dependencies:

```bash
conan install . --output-folder=build --build=missing
```

Build:

```bash
conan build .
```

Format code:

```bash
ninja -C <build_folder> clang-format
```

Run everything (e.g. install,build, package, test)

```bash
conan create . --build=missing
```

Run tests:

<!-- ```bash
ctest --output-on-failure --config Release
``` -->
<!-- TODO： Check ^^ -->
