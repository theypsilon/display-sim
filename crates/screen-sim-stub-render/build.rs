use build_tools::{copy_webgl_render_crate_to_file, CopyWebglRenderParams};
fn main() {
    copy_webgl_render_crate_to_file(&CopyWebglRenderParams{
        output_file: "screen-sim-webgl-render-copy.rs",
        web_sys_replacement: "crate::stubs",
        web_error_replacement: "crate::stubs",
    });
}