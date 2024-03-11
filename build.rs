use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
	// for entry in WalkDir::new("proto/").into_iter().filter_map(|e| e.ok()) {
	// 	let path: PathBuf = entry.path().into();
	// 	if path.is_file() && path.to_str().unwrap().ends_with(".proto") {
	// 		println!("=>> {:?}", path);
	// 		let mut output_path = PathBuf::from("src/");
	// 		output_path.extend(path.components().skip(1));

	// 		tonic_build::configure()
	// 			.build_server(true)
	// 			.build_client(false)
	// 			.out_dir(output_path.parent().unwrap())
	// 			.compile(&[&path], &[path.parent().unwrap()])
	// 			.unwrap();
	// 	}
	// }

	tonic_build::configure()
		.build_server(true)
		.build_client(false)
		.compile(&["proto/auth_service.proto"], &["proto"])
		.unwrap();
}
