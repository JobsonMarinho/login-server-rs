fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/login.proto"], &["proto"])
        .unwrap();
}
