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

    // Feature flags
    let features = vec![
        "graphql",
        "upload",
        "metadata",
    ];
    let mut has_server_feature = false;

    for feature in features.iter() {
        // println!("cargo:warning=Checking feature: {}", &feature);
        if std::env::var(format!("CARGO_FEATURE_{}", &feature.to_uppercase())).is_ok() {
            has_server_feature = true;
        }
    }

    if !has_server_feature {
        panic!("No server feature enabled. At least one of {features:?} should be enabled.");
    }
}
