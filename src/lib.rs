use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Headers;
use web_sys::{Request, Response, ResponseInit};

#[wasm_bindgen]
pub fn handle_request(_request: Request) -> Result<JsValue, JsValue> {
    let headers = Headers::new()?;
    headers.set("Content-Type", "text/plain; charset=utf-8")?;

    let mut init = ResponseInit::new();
    init.status_text("OK");
    init.status(200);
    init.headers(headers.unchecked_ref());
    let response = Response::new_with_opt_str_and_init(Some("Hello from Rust!"), init.as_ref())?;

    Ok(response.dyn_into()?)
}
