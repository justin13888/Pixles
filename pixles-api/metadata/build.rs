fn main() {
    tonic_build::configure()
        .compile_protos(
            &["proto/photolibrary/metadata/v1/metadata.proto"],
            &["proto"]
        )
        .unwrap();

    tonic_build::configure()
        .compile_protos(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();
}
