use std::path::Path;
use std::{fs, env};
use std::io::{Read, Write};
use glob::glob;

pub struct CopyWebglRenderParams {
    pub output_file: &'static str,
    pub web_sys_replacement: &'static str,
    pub web_error_replacement: &'static str
}

pub fn copy_webgl_render_crate_to_file(params: &CopyWebglRenderParams) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut file = fs::File::create(&Path::new(&out_dir).join(params.output_file)).unwrap();
    let path = manifest_dir + "/../screen-sim-webgl-render/src/*.rs";

    let mut lib_rs_source = "<none>".to_string();

    let mut sources: Vec<(String, String)> = vec![];
    for node in glob(&path).unwrap() {
        let node = node.unwrap();
        let node_path = Path::new(&node);
        let filestem = node_path.file_stem().unwrap().to_str().unwrap();

        let mut source = String::new();
        fs::File::open(&node).unwrap().read_to_string(&mut source).unwrap();
        if filestem == "lib" {
            lib_rs_source = source;
        } else {
            sources.push((filestem.to_string(), source));
        }
    }

    if lib_rs_source == "<none>" {
        panic!("lib.rs has not been found.");
    }

    lib_rs_source = lib_rs_source.replace(
        "web_sys", 
        params.web_sys_replacement
    );

    lib_rs_source = lib_rs_source.replace(
        "web_error", 
        params.web_error_replacement
    );

    // @TODO Check if compiler error has been fixed. This should not be needed, but
    // if I don't put it, I got an error. So I need to put that macro manually.
    lib_rs_source = lib_rs_source.replace("#![allow(clippy::identity_op)]", "");

    for source in sources.iter() {
        lib_rs_source = lib_rs_source.replace(
            &format!("mod {};", &source.0),
            &format!("mod {} {{\n{}\n}}\n", &source.0, &source.1)
        );
    }

    file.write_fmt(format_args!("{}", lib_rs_source)).unwrap();
}