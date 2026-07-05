use serde_json::Value;
use jsonschema::{Draft, JSONSchema};

const PROJECT_SCHEMA_JSON: &str = include_str!("../../data/schema/project.schema.json");

#[derive(Debug)]
pub enum ValidationError {
    InvalidSchema(String),
    Failed(Vec<String>),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidSchema(msg) => write!(f, "Schema invalid: {}", msg),
            ValidationError::Failed(errors) => {
                write!(f, "Validation failed:")?;
                for e in errors {
                    write!(f, " - {}", e)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validate a project JSON value against the embedded JSON Schema.
/// Returns Ok(()) if valid, or ValidationError with details.
pub fn validate_project(json: &Value) -> Result<(), ValidationError> {
    let schema_value: Value = serde_json::from_str(PROJECT_SCHEMA_JSON)
        .map_err(|e| ValidationError::InvalidSchema(e.to_string()))?;

    let schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_value)
        .map_err(|e| ValidationError::InvalidSchema(e.to_string()))?;

    let result = schema.validate(json);
    if let Err(errors) = result {
        let messages: Vec<String> = errors.map(|e| e.to_string()).collect();
        return Err(ValidationError::Failed(messages));
    }

    Ok(())
}
