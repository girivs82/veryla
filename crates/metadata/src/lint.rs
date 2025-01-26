use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lint {
    #[serde(default)]
    pub naming: LintNaming,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintNaming {
    #[serde(default)]
    pub case_power_domain: Option<Case>,
    #[serde(default)]
    pub case_enum: Option<Case>,
    #[serde(default)]
    pub case_function: Option<Case>,
    #[serde(default)]
    pub case_function_inout: Option<Case>,
    #[serde(default)]
    pub case_function_input: Option<Case>,
    #[serde(default)]
    pub case_function_output: Option<Case>,
    #[serde(default)]
    pub case_function_ref: Option<Case>,
    #[serde(default)]
    pub case_instance: Option<Case>,
    #[serde(default)]
    pub case_interface: Option<Case>,
    #[serde(default)]
    pub case_modport: Option<Case>,
    #[serde(default)]
    pub case_entity: Option<Case>,
    #[serde(default)]
    pub case_package: Option<Case>,
    #[serde(default)]
    pub case_parameter: Option<Case>,
    #[serde(default)]
    pub case_port_inout: Option<Case>,
    #[serde(default)]
    pub case_port_input: Option<Case>,
    #[serde(default)]
    pub case_port_modport: Option<Case>,
    #[serde(default)]
    pub case_port_output: Option<Case>,
    #[serde(default)]
    pub case_reg: Option<Case>,
    #[serde(default)]
    pub case_struct: Option<Case>,
    #[serde(default)]
    pub case_union: Option<Case>,
    #[serde(default)]
    pub case_var: Option<Case>,
    #[serde(default)]
    pub case_wire: Option<Case>,
    #[serde(default)]
    pub prefix_power_domain: Option<String>,
    #[serde(default)]
    pub prefix_enum: Option<String>,
    #[serde(default)]
    pub prefix_function: Option<String>,
    #[serde(default)]
    pub prefix_function_inout: Option<String>,
    #[serde(default)]
    pub prefix_function_input: Option<String>,
    #[serde(default)]
    pub prefix_function_output: Option<String>,
    #[serde(default)]
    pub prefix_function_ref: Option<String>,
    #[serde(default)]
    pub prefix_instance: Option<String>,
    #[serde(default)]
    pub prefix_interface: Option<String>,
    #[serde(default)]
    pub prefix_modport: Option<String>,
    #[serde(default)]
    pub prefix_entity: Option<String>,
    #[serde(default)]
    pub prefix_package: Option<String>,
    #[serde(default)]
    pub prefix_parameter: Option<String>,
    #[serde(default)]
    pub prefix_port_inout: Option<String>,
    #[serde(default)]
    pub prefix_port_input: Option<String>,
    #[serde(default)]
    pub prefix_port_modport: Option<String>,
    #[serde(default)]
    pub prefix_port_output: Option<String>,
    #[serde(default)]
    pub prefix_reg: Option<String>,
    #[serde(default)]
    pub prefix_struct: Option<String>,
    #[serde(default)]
    pub prefix_union: Option<String>,
    #[serde(default)]
    pub prefix_var: Option<String>,
    #[serde(default)]
    pub prefix_wire: Option<String>,
    #[serde(default)]
    pub suffix_power_domain: Option<String>,
    #[serde(default)]
    pub suffix_enum: Option<String>,
    #[serde(default)]
    pub suffix_function: Option<String>,
    #[serde(default)]
    pub suffix_function_inout: Option<String>,
    #[serde(default)]
    pub suffix_function_input: Option<String>,
    #[serde(default)]
    pub suffix_function_output: Option<String>,
    #[serde(default)]
    pub suffix_function_ref: Option<String>,
    #[serde(default)]
    pub suffix_instance: Option<String>,
    #[serde(default)]
    pub suffix_interface: Option<String>,
    #[serde(default)]
    pub suffix_modport: Option<String>,
    #[serde(default)]
    pub suffix_entity: Option<String>,
    #[serde(default)]
    pub suffix_package: Option<String>,
    #[serde(default)]
    pub suffix_parameter: Option<String>,
    #[serde(default)]
    pub suffix_port_inout: Option<String>,
    #[serde(default)]
    pub suffix_port_input: Option<String>,
    #[serde(default)]
    pub suffix_port_modport: Option<String>,
    #[serde(default)]
    pub suffix_port_output: Option<String>,
    #[serde(default)]
    pub suffix_reg: Option<String>,
    #[serde(default)]
    pub suffix_struct: Option<String>,
    #[serde(default)]
    pub suffix_union: Option<String>,
    #[serde(default)]
    pub suffix_var: Option<String>,
    #[serde(default)]
    pub suffix_wire: Option<String>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_power_domain: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_enum: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_function: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_function_inout: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_function_input: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_function_output: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_function_ref: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_instance: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_interface: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_modport: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_entity: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_package: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_parameter: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_port_inout: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_port_input: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_port_modport: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_port_output: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_reg: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_struct: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_union: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_var: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_forbidden_wire: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_power_domain: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_enum: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_function: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_function_inout: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_function_input: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_function_output: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_function_ref: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_instance: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_interface: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_modport: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_entity: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_package: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_parameter: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_port_inout: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_port_input: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_port_modport: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_port_output: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_reg: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_struct: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_union: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_var: Option<Regex>,
    #[serde(default, with = "serde_regex")]
    pub re_required_wire: Option<Regex>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Case {
    #[default]
    #[serde(rename = "snake")]
    Snake,
    #[serde(rename = "screaming_snake")]
    ScreamingSnake,
    #[serde(rename = "upper_camel")]
    UpperCamel,
    #[serde(rename = "lower_camel")]
    LowerCamel,
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Case::Snake => "snake_case".to_string(),
            Case::ScreamingSnake => "SCREAMING_SNAKE_CASE".to_string(),
            Case::UpperCamel => "UpperCamelCase".to_string(),
            Case::LowerCamel => "lowerCamelCase".to_string(),
        };
        text.fmt(f)
    }
}
