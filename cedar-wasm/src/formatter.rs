use cedar_policy_formatter::{policies_str_to_pretty, Config};
use serde::{Deserialize, Serialize};

use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct FormattingResult {
    success: bool,
    #[serde(rename="formattedPolicy", skip_serializing_if="Option::is_none")]
    formatted_policy: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    error: Option<String>,
}

#[wasm_bindgen(js_name="formatPolicies")]
pub fn format_policies(policies_str: &str, line_width: i32, indent_width: i32) -> FormattingResult {
    let line_width: Result<usize, _> = line_width.try_into();
    let indent_width: Result<isize, _> = indent_width.try_into();
    if line_width.is_err() || indent_width.is_err() {
        return FormattingResult {
            success: false,
            formatted_policy: None,
            error: Some("Input size error (line or indent width)".to_string()),
        }
    }
    let config = Config {
        line_width: line_width.unwrap(),
        indent_width: indent_width.unwrap(),
    };
    match policies_str_to_pretty(policies_str, &config) {
        Ok(prettified_policy) => FormattingResult {
            success: true,
            formatted_policy: Some(prettified_policy),
            error: None,
        },
        Err(err) => FormattingResult {
            success: false,
            formatted_policy: None,
            error: Some(err.to_string()),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_policies() {
        let policy = r#"permit(principal, action == Action::"view", resource in Albums::"gangsta rap") when {principal.is_gangsta == true};"#;
        let expected = "{\"success\":true,\"formatted_policy\":\"permit (\\n    principal,\\n    action == Action::\\\"view\\\",\\n    resource in Albums::\\\"gangsta rap\\\"\\n)\\nwhen { principal.is_gangsta == true };\",\"error\":null}";
        assert_eq!(format_policies(policy, 80, 4).formatted_policy.unwrap(), expected);
    }
}
