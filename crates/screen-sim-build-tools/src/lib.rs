use std::path::Path;
use std::{fs, env};
use std::io::{Read, Write};
use glob::glob;

pub fn copy_screen_sim_webgl_render_modules_to_out_folder(output_filename: &str) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut file = fs::File::create(&Path::new(&out_dir).join(output_filename)).unwrap();
    let path = manifest_dir + "/../screen-sim-webgl-render/src/*.rs";

    let mut sources: Vec<String> = vec![];
    for node in glob(&path).unwrap() {
        let node = node.unwrap();
        let node_path = Path::new(&node);
        let filestem = node_path.file_stem().unwrap().to_str().unwrap();

        if filestem == "lib" {
            continue;
        }

        let mut source = String::new();
        source += &format!("\npub mod {} {{\n", filestem);
        fs::File::open(&node).unwrap().read_to_string(&mut source).unwrap();
        source += "}\n";
        sources.push(source);
    }

    for source in sources.iter() {
        file.write_fmt(format_args!("{}", source)).unwrap();
    }
}