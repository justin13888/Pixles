from conan import ConanFile
from conan.tools.meson import MesonToolchain, Meson
from conan.tools.layout import basic_layout

class PixlesMediaConan(ConanFile):
    name = "pixles_media"
    version = "0.1.0"
    description = "High-level, cross-platform image and video processing library."
    license = "AGPL-3.0-only"
    author = "Justin Chung"
    homepage = "https://github.com/justin13888/Pixles"
    url = "https://github.com/justin13888/Pixles"
    topics = ("image", "video", "processing")
    package_type = "library"
    requires = "ffmpeg/7.0.1"
    tool_requires = "meson/1.6.0", "ninja/1.12.1"
    generators = "PkgConfigDeps"

    # Binary configuration
    settings = "os", "compiler", "build_type", "arch"
    options = {"shared": [True, False], "fPIC": [True, False]}
    default_options = {"shared": False, "fPIC": True}

    # Sources are located in the same place as this recipe, copy them to the recipe
    exports_sources = "meson.build", "src/*"

    def config_options(self):
        if self.settings.os == "Windows":
            self.options.rm_safe("fPIC")

    def configure(self):
        if self.options.shared:
            self.options.rm_safe("fPIC")

    def layout(self):
        basic_layout(self)

    def generate(self):
        tc = MesonToolchain(self)
        tc.generate()

    def build(self):
        meson = Meson(self)
        meson.configure()
        meson.build()

    def package(self):
        meson = Meson(self)
        meson.install()

    def package_info(self):
        self.cpp_info.libs = ["pixles_media"]
