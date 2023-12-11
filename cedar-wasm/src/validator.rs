use cedar_policy::frontend::{validate::json_validate, utils::InterfaceResult};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name="validateSemantics")]
pub fn validate_semantics(input: &str) -> InterfaceResult {
    json_validate(input)
}