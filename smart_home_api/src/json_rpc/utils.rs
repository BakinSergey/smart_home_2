use jsonschema::Validator;
use serde_json as json;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// json validator from given jsonschema file path
pub fn get_validator(path: PathBuf) -> anyhow::Result<Validator> {
    let jdata = fs::read_to_string(path)?;
    let schema = json::from_str(&jdata).expect("parse json failed");
    jsonschema::validator_for(&schema).map_err(|e| e.into())
}

pub fn unquoted(value: &Value) -> String {
    value.to_string().trim_matches('"').to_string()
}
