use std::str::FromStr;

use cedar_policy::{Policy, PolicySet};
use serde::{Deserialize, Serialize};

use tsify::Tsify;
use wasm_bindgen::prelude::*;


#[derive(Tsify, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum JsonToPolicyResult {
    Success {
        policy_text: String
    },
    Error { errors: Vec<String> }
}

#[wasm_bindgen(js_name="policyTextFromJson")]
pub fn policy_text_from_json(json_str: &str) -> JsonToPolicyResult {
    let parsed_json = serde_json::from_str(json_str).expect("Deserialization failed");
    let policy = Policy::from_json(None, parsed_json);
    match policy {
        Ok(p) => JsonToPolicyResult::Success {
            policy_text: p.to_string(),
        },
        Err(e) => JsonToPolicyResult::Error {
            errors: vec![e.to_string()]
        },
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PolicyToJsonResult {
    Success {
        policy: cedar_policy_core::est::Policy,
    },
    Error {
        errors: Vec<String>,
    }
}

#[wasm_bindgen(js_name="policyTextToJson")]
pub fn policy_text_to_json(cedar_str: &str) -> PolicyToJsonResult {
    let parsed_policy = match Policy::from_str(cedar_str) {
        Ok(policy) => policy,
        Err(errs) => {
            return PolicyToJsonResult::Error { errors: errs.errors_as_strings() };
        }
    };
    match parsed_policy.to_json_policy() {
        Ok(json_policy) => PolicyToJsonResult::Success {
            policy: json_policy,
        },
        Err(err) => PolicyToJsonResult::Error {
            errors: vec![err.to_string()],
        },
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
/// struct that defines the result for the syntax validation function
pub enum PolicySetParseResult {
    /// represents successful syntax validation
    Success { policies: i32, templates: i32 },
    /// represents a syntax error and encloses a vector of the errors
    SyntaxError { errors: Vec<String> },
}

#[wasm_bindgen(js_name="parsePolicySet")]
pub fn parse_policy_set(input_policies_str: &str) -> PolicySetParseResult {
    match PolicySet::from_str(input_policies_str) {
        Err(validation_errors) => PolicySetParseResult::SyntaxError {
            errors: validation_errors.errors_as_strings(),
        },
        Ok(policy_set) => {
            let policies_count: Result<i32, <i32 as TryFrom<usize>>::Error> =
                policy_set.policies().count().try_into();
            let templates_count: Result<i32, <i32 as TryFrom<usize>>::Error> =
                policy_set.templates().count().try_into();
            match (policies_count, templates_count) {
                (Ok(p), Ok(t)) => PolicySetParseResult::Success {
                    policies: p,
                    templates: t,
                },
                _ => PolicySetParseResult::SyntaxError {
                    errors: vec!["Error counting policies or templates".to_string()],
                },
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Template{
    text: String,
    slots: Vec<String>,
    parse_errors: Option<Vec<String>>,
}

#[wasm_bindgen]
impl Template {
    #[wasm_bindgen(constructor)]
    pub fn new(template_str: &str) -> Template {
        match cedar_policy::Template::from_str(template_str) {
            Err(parse_errs) => Self {
                text: String::from(""),
                slots: vec![],
                parse_errors: Some(parse_errs.errors_as_strings()),
            },
            Ok(template) => match template.slots().count() {
                1 | 2 => Self {
                    text: template_str.to_string(),
                    slots: template.slots().map(|slot| slot.to_string()).collect(),
                    parse_errors: None,
                },
                _ => Self {
                    text: String::from(""),
                    slots: vec![],
                    parse_errors: Some(vec!["Expected template to have 1 or 2 slots".to_string()]),
                },
            },
        }
    }
    #[wasm_bindgen(js_name="isValid")]
    pub fn is_valid(&self) -> bool {
        self.parse_errors.is_none()
    }
    #[wasm_bindgen(getter, js_name="parseErrors")]
    pub fn parse_errors(&self) -> Option<Vec<String>> {
        self.parse_errors.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn slots(&self) -> Vec<String> {
        self.slots.clone()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use serde_json::json;

    use super::*;

    #[test]
    fn test_conversion_from_cedar() {
        let cedar_repr = r#"permit(principal, action, resource) when { principal has "Email" && principal.Email == "a@a.com" };"#;
        let json_conversion_result = policy_text_to_json(cedar_repr);
        assert!(
            matches!(
                json_conversion_result,
                PolicyToJsonResult::Success { policy: _ }
            )
        )
    }

    #[test]
    fn test_convertion_from_json() {
        let est_repr = r#"{
            "effect": "permit",
            "action": {
                "entity": {
                    "id": "pop",
                    "type": "Action"
                },
                "op": "=="
            },
            "principal": {
                "entity": {
                    "id": "DeathRowRecords",
                    "type": "UserGroup"
                },
                "op": "in"
            },
            "resource": {
                "op": "All"
            },
            "conditions": []
        }"#;

        let cedar_convertion_result: JsonToPolicyResult = policy_text_from_json(&est_repr);
        match cedar_convertion_result {
            JsonToPolicyResult::Success { policy_text } => assert_eq!(
                &policy_text,
                "permit(\n  principal in UserGroup::\"DeathRowRecords\",\n  action == Action::\"pop\",\n  resource\n) when {\n  true\n};"
            ),
            JsonToPolicyResult::Error { errors } => {
                dbg!(errors);
                panic!("Test failed")
            }
        }
    }

    #[test]
    fn can_parse_1_policy() {
        let stringified_result = parse_policy_set("permit(principal, action, resource);");
        assert_result_is_ok(&stringified_result);
    }

    #[test]
    fn can_parse_multi_policy() {
        assert_result_is_ok(&parse_policy_set(
            "forbid(principal, action, resource); permit(principal == User::\"alice\", action == Action::\"view\", resource in Albums::\"alice_albums\");"
        ));
    }

    #[test]
    fn parse_returns_validation_errors_when_expected_1_policy() {
        assert_result_had_syntax_errors(&parse_policy_set("permit(2pac, action, resource)"));
    }

    #[test]
    fn parse_returns_validation_errors_when_expected_multi_policy() {
        assert_result_had_syntax_errors(&parse_policy_set(
            "forbid(principal, action, resource);permit(2pac, action, resource)",
        ));
    }

    fn assert_result_is_ok(result: &PolicySetParseResult) {
        assert!(matches!(
            result,
            PolicySetParseResult::Success {
                policies: _,
                templates: _,
            }
        ));
    }

    fn assert_result_had_syntax_errors(result: &PolicySetParseResult) {
        assert!(matches!(
            result,
            PolicySetParseResult::SyntaxError { errors: _ }
        ));
    }

}
