use web_error::WebResult;
use web_sys::Window;

pub fn window() -> WebResult<Window> {
    Ok(web_sys::window().ok_or("cannot access window")?)
}

pub fn now() -> WebResult<f64> {
    Ok(window()?.performance().ok_or("cannot access performance")?.now())
}
