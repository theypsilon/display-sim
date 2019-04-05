use web_common::wasm_error::WasmResult;
use web_sys::Window;

pub fn window() -> WasmResult<Window> {
    Ok(web_sys::window().ok_or("cannot access window")?)
}

pub fn now() -> WasmResult<f64> {
    Ok(window()?.performance().ok_or("cannot access performance")?.now())
}