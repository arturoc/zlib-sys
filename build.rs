use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn build_unix() {
	let zlib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let zlib_native_dir = Path::new(&zlib_dir).join("zlib-1.2.8");
	let build_dir = zlib_native_dir.join("build");
	fs::remove_dir_all(&build_dir).is_ok();
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
	println!("cargo:rustc-link-lib=static=z");
	println!("cargo:rustc-link-search=native={}",out_dir);
}


fn build_emscripten() {
	let zlib_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let zlib_native_dir = Path::new(&zlib_dir).join("zlib-1.2.8");
	let build_dir = zlib_native_dir.join("build");
	fs::remove_dir_all(&build_dir).is_ok();
	fs::create_dir(&build_dir).is_ok();
	Command::new("emcmake")
		.arg("cmake")
		.arg("..")
		.current_dir(&build_dir)
		.status().unwrap();
	Command::new("emmake")
		.arg("make")
		.current_dir(&build_dir)
		.status().unwrap();
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("libz.a");
	fs::copy(build_dir.join("libz.a"),dest_path).unwrap();
	println!("cargo:rustc-link-lib=static=z");
	println!("cargo:rustc-link-search=native={}",out_dir);
}


fn main(){
	let target_triple = env::var("TARGET").unwrap();
	if target_triple.contains("linux") {
		build_unix()
	}else if target_triple.contains("macos") {
		build_unix()
	}else if target_triple.contains("emscripten") {
		build_emscripten()
	}else{
		panic!("target OS {} not suported yet", target_triple);
	}
}
