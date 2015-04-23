use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
	let zlib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let zlib_native_dir = Path::new(&zlib_dir).join("zlib-1.2.8");
	let build_dir = zlib_native_dir.join("build");
	fs::create_dir(&build_dir).is_ok();
	Command::new("cmake")
		.arg("..")
		.current_dir(&build_dir)
		.status().unwrap();
	Command::new("make")
		.current_dir(&build_dir)
		.status().unwrap();
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("libz.a");
	fs::copy(build_dir.join("libz.a"),dest_path).unwrap();
	println!("cargo:rustc-flags= -L native={}",out_dir);
}
