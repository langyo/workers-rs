use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    pub type DurableObjectId;

    #[wasm_bindgen(method, catch, js_name=toString)]
    pub fn to_string(this: &DurableObjectId) -> Result<String, JsValue>;

    #[wasm_bindgen(method)]
    pub fn equals(this: &DurableObjectId, other: &DurableObjectId) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &DurableObjectId) -> Option<String>;
}
