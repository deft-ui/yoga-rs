extern crate bindgen;
extern crate cc;

use bindgen::{NonCopyUnionStyle, RustTarget};
use cc::Build;
use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src/yoga/yoga");

    Command::new("git")
        .args(["submodule", "init"])
        .status()
        .expect("Unable to initialize git submodules");
    Command::new("git")
        .args(["submodule", "update"])
        .status()
        .expect("Unable to update the submodule repositories");

    Build::new()
		.cpp(true)
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/BUCK#L13
		.flag("-std=c++20")
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/yoga_defs.bzl#L49-L56
		.flag("-fno-omit-frame-pointer")
		.flag("-fexceptions")
		.flag("-Wall")
		.flag("-O3")
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/yoga_defs.bzl#L58-L60
		.flag("-fPIC")
		// Include path
		.include("src/yoga")
		// C++ Files
		.file("src/yoga/yoga/YGValue.cpp")
		.file("src/yoga/yoga/config/Config.cpp")
		.file("src/yoga/yoga/YGEnums.cpp")
		.file("src/yoga/yoga/YGNodeLayout.cpp")
		.file("src/yoga/yoga/algorithm/AbsoluteLayout.cpp")
		.file("src/yoga/yoga/algorithm/PixelGrid.cpp")
		.file("src/yoga/yoga/algorithm/Baseline.cpp")
		.file("src/yoga/yoga/algorithm/CalculateLayout.cpp")
		.file("src/yoga/yoga/algorithm/FlexLine.cpp")
		.file("src/yoga/yoga/algorithm/Cache.cpp")
		.file("src/yoga/yoga/event/event.cpp")
		.file("src/yoga/yoga/YGPixelGrid.cpp")
		.file("src/yoga/yoga/YGNode.cpp")
		.file("src/yoga/yoga/debug/Log.cpp")
		.file("src/yoga/yoga/debug/AssertFatal.cpp")
		.file("src/yoga/yoga/YGConfig.cpp")
		.file("src/yoga/yoga/YGNodeStyle.cpp")
		.file("src/yoga/yoga/node/LayoutResults.cpp")
		.file("src/yoga/yoga/node/Node.cpp")
		.compile("libyoga.a");

    let bindings = bindgen::Builder::default()
        .rust_target(RustTarget::Stable_1_64)
        .clang_arg("--language=c++")
        .clang_arg("-std=c++11")
        .clang_arg("-stdlib=libc++")
		.clang_arg("-Isrc/yoga")
        .no_convert_floats()
        .enable_cxx_namespaces()
        .allowlist_type("YG.*")
        .allowlist_function("YG.*")
        // .allowlist_var("YG.*")
        .layout_tests(false)
        .rustfmt_bindings(true)
        .rustified_enum("YG.*")
        .manually_drop_union(".*")
        .default_non_copy_union_style(NonCopyUnionStyle::ManuallyDrop)
        .header("src/wrapper.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs")).expect("Unable to write bindings!");
}
