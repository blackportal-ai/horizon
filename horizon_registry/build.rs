fn main() {
    tonic_build::configure()
        .compile_protos(
            &["proto/horizon_registry.proto"],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}