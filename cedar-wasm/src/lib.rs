#![forbid(unsafe_code)]

use wasm_bindgen::prelude::*;

mod evaluator;
mod formatter;
mod policies_and_templates;
mod schema_and_entities;
mod validator;

#[wasm_bindgen(js_name="getCedarVersion")]
pub fn get_cedar_version() -> String {
    std::env!("CEDAR_VERSION").to_string()
}