fn main() {
	prost_build::Config::new()
		.out_dir("src")
		.compile_protos(&["proto/auth_service.proto"], &["proto/"])
		.unwrap();
}
