use std::env;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct BindgenWhitelist {
    types: Option<Vec<String>>,
    functions: Option<Vec<String>>,
    vars: Option<Vec<String>>,
}

fn build() {
    println!("cargo:rerun-if-changed=libbtrfsutil/btrfsutil.h");
    println!("cargo:rerun-if-changed=bindgen_whitelist.toml");

    let bindgen_whitelist_string: String = String::from_utf8(
        std::fs::read("bindgen_whitelist.toml").expect("Failed to open bindgen_whitelist.toml"),
    )
    .expect("Failed to convert the whitelist file to UTF-8 string");
    let bindgen_whitelist: BindgenWhitelist = toml::from_str(bindgen_whitelist_string.as_str())
        .expect("Failed to deserialize bindgen whitelist");

    let mut bindings_builder: bindgen::Builder =
        bindgen::Builder::default().header("libbtrfsutil/btrfsutil.h");

    if let Some(val) = bindgen_whitelist.types {
        for type_name in val {
            bindings_builder = bindings_builder.allowlist_type(type_name);
        }
    }
    if let Some(val) = bindgen_whitelist.functions {
        for function_name in val {
            bindings_builder = bindings_builder.allowlist_function(function_name);
        }
    }
    if let Some(val) = bindgen_whitelist.vars {
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
