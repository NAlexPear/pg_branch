use std::env;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct BindgenAllowlist {
    types: Option<Vec<String>>,
    functions: Option<Vec<String>>,
    vars: Option<Vec<String>>,
}

fn build() {
    println!("cargo:rerun-if-changed=libbtrfsutil/btrfsutil.h");
    println!("cargo:rerun-if-changed=bindgen_allowlist.toml");

    let bindgen_allowlist_string: String = String::from_utf8(
        std::fs::read("bindgen_allowlist.toml").expect("Failed to open bindgen_allowlist.toml"),
    )
    .expect("Failed to convert the allowlist file to UTF-8 string");
    let bindgen_allowlist: BindgenAllowlist = toml::from_str(bindgen_allowlist_string.as_str())
        .expect("Failed to deserialize bindgen allowlist");

    let mut bindings_builder: bindgen::Builder =
        bindgen::Builder::default().header("libbtrfsutil/btrfsutil.h");

    if let Some(val) = bindgen_allowlist.types {
        for type_name in val {
            bindings_builder = bindings_builder.allowlist_type(type_name);
        }
    }
    if let Some(val) = bindgen_allowlist.functions {
        for function_name in val {
            bindings_builder = bindings_builder.allowlist_function(function_name);
        }
    }
    if let Some(val) = bindgen_allowlist.vars {
        for var_name in val {
            bindings_builder = bindings_builder.allowlist_var(var_name);
        }
    }
    bindings_builder = bindings_builder.parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings: bindgen::Bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    if cfg!(not(docs_rs)) {
        println!("cargo:rustc-link-lib=btrfsutil");
    }
    build();
}
