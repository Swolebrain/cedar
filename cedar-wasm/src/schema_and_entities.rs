use std::str::FromStr;

use cedar_policy::{Schema, Entities};
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[serde(tag = "success")]
#[tsify(into_wasm_abi, from_wasm_abi)]
/// struct that defines the result for the syntax validation function
pub enum ParseResult {
    #[serde(rename = "true")]
    /// represents successful syntax validation
    Success,
    #[serde(rename = "false")]
    /// represents a syntax error and encloses a vector of the errors
    SyntaxError { errors: Vec<String> },
}

#[wasm_bindgen(js_name="parseSchema")]
pub fn parse_schema(input_schema: &str) -> ParseResult {
    match Schema::from_str(input_schema) {
        Ok(_schema) => ParseResult::Success,
        Err(err) => ParseResult::SyntaxError {
            errors: vec![err.to_string()],
        },
    }
}

#[wasm_bindgen(js_name="parseEntities")]
pub fn parse_entities(entities_str: &str, schema_str: &str) -> ParseResult {
    let parsed_schema = match Schema::from_str(schema_str) {
        Ok(schema) => schema,
        Err(err) => {
            return ParseResult::SyntaxError {
                errors: vec![err.to_string()],
            }
        }
    };
    match Entities::from_json_str(entities_str, Some(&parsed_schema)) {
        Ok(_entities) => ParseResult::Success,
        Err(err) => ParseResult::SyntaxError {
            errors: vec![err.to_string()],
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    // Schema validator
    #[test]
    fn validate_schema_syntax_succeeds_empty_schema() {
        let schema_str = "{}";
        assert_syntax_result_is_ok(&parse_schema(schema_str))
    }
    #[test]
    fn validate_schema_syntax_succeeds_nonempty_schema() {
        let schema_str = r#"{
          "MyNamespace": {
            "entityTypes": {},
            "actions": {}
          }
        }"#;
        assert_syntax_result_is_ok(&parse_schema(schema_str))
    }

    #[test]
    fn validate_schema_bad_syntax_fails() {
        let schema_str = r#"{
            "MyNamespace": {
              "entityTypes": {}
            }
          }"#;
        assert_syntax_result_has_errors(&parse_schema(schema_str))
    }

    // Entities

    #[test]
    fn validate_entities_succeeds() {
        let entities_str = r#"[
            {
                "uid": "TheNamespace::User::\"alice\"",
                "attrs": {
                    "department": "HardwareEngineering",
                    "jobLevel": 5
                },
                "parents": []
              }
        ]"#;
        let schema_str = r#"{
            "TheNamespace": {
                "entityTypes": {
                    "User": {
                        "memberOfTypes": [],
                        "shape": {
                            "attributes": {
                                "department": {
                                    "type": "String"
                                },
                                "jobLevel": {
                                    "type": "Long"
                                }
                            },
                            "type": "Record"
                        }
                    }
                },
                "actions": {}
            }
        }"#;
        assert_syntax_result_is_ok(&parse_entities(
            entities_str,
            schema_str,
        ));
    }

    #[test]
    fn validate_entities_fails_on_bad_entity() {
        let entities_str = r#"[
            {
                "uid": "TheNamespace::User::\"alice\"",
                "attrs": {
                    "benchPress": "doesn'tevenlift"
                },
                "parents": []
              }
        ]"#;
        let schema_str = r#"{
            "TheNamespace": {
                "entityTypes": {
                    "User": {
                        "memberOfTypes": [],
                        "shape": {
                            "attributes": {
                                "department": {
                                    "type": "String"
                                }
                            },
                            "type": "Record"
                        }
                    }
                },
                "actions": {}
            }
        }"#;
        assert_syntax_result_has_errors(&parse_entities(
            entities_str,
            schema_str,
        ));
    }

    fn assert_syntax_result_is_ok(parse_result: &ParseResult) {
        assert!(matches!(
            parse_result,
            ParseResult::Success
        ))
    }

    fn assert_syntax_result_has_errors(parse_result: &ParseResult) {
        assert!(matches!(
            parse_result,
            ParseResult::SyntaxError { errors: _ }
        ))
    }
}