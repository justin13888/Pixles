fn main() {
    let debug_build = std::env::var("PROFILE")
        .map(|profile| profile == "debug")
        .unwrap_or(false);

    // Enable 'openapi' feature if in debug
    if debug_build {
        println!("cargo:rustc-cfg=feature=\"openapi\""); // TODO: Why this doesn't enable the feature?
        println!("cargo:warning=OpenAPI enabled for debug build");
    } else if std::env::var("CARGO_FEATURE_OPENAPI").is_ok() {
        panic!("Error: The `openapi` feature cannot be enabled in release for security reasons");
    }
}
