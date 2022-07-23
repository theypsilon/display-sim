/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use glob::glob;
use std::io::{Read, Write};
use std::path::Path;
use std::{env, fs};

fn main() {
    copy_render_crate_to_file(&CopyWebglRenderParams {
        output_file: "display-sim-render-copy.rs",
        glow_replacement: "crate::glow_test_stub::",
    });
}

struct CopyWebglRenderParams {
    pub output_file: &'static str,
    pub glow_replacement: &'static str,
}

fn copy_render_crate_to_file(params: &CopyWebglRenderParams) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut file = fs::File::create(&Path::new(&out_dir).join(params.output_file)).unwrap();
    let path = manifest_dir + "/../display-sim-render/src/*.rs";

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

    // @TODO Check if compiler error has been fixed. This should not be needed, but
    // if I don't put it, I got an error. So I need to put that macro manually.
    lib_rs_source = lib_rs_source.replace("#![allow(clippy::identity_op)]", "");

    for source in sources.iter() {
        lib_rs_source = lib_rs_source.replace(&format!("mod {};", &source.0), &format!("mod {} {{\n{}\n}}\n", &source.0, &source.1));
    }

    lib_rs_source = lib_rs_source.replace("glow::", params.glow_replacement);

    file.write_fmt(format_args!("{}", lib_rs_source)).unwrap();
}
