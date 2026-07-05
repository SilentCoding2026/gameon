use serde_json::Value;
use jsonschema::{Draft, JSONSchema, CompilationOptions};

const PROJECT_SCHEMA_JSON: &str = r#"
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["meta", "assets", "scenes", "timeline"],
  "properties": {
    "meta": {
      "type": "object",
      "required": ["version", "name", "width", "height", "fps"],
      "properties": {
        "version": { "type": "string" },
        "name": { "type": "string" },
        "width": { "type": "integer", "minimum": 1 },
        "height": { "type": "integer", "minimum": 1 },
        "fps": { "type": "number", "exclusiveMinimum": 0 }
      }
    },
    "assets": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "type", "path", "width", "height"],
        "properties": {
          "id": { "type": "string" },
          "type": { "type": "string", "enum": ["Image"] },
          "path": { "type": "string" },
          "width": { "type": "integer", "minimum": 1 },
          "height": { "type": "integer", "minimum": 1 }
        }
      }
    },
    "scenes": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "name", "duration", "layers"],
        "properties": {
          "id": { "type": "string" },
          "name": { "type": "string" },
          "duration": { "type": "number", "exclusiveMinimum": 0 },
          "layers": {
            "type": "array",
            "items": {
              "type": "object",
              "required": ["id", "name", "keyframes"],
              "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "keyframes": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "required": ["time", "properties"],
                    "properties": {
                      "time": { "type": "number", "minimum": 0 },
                      "properties": {
                        "type": "object",
                        "required": ["x", "y", "scaleX", "scaleY", "rotation", "opacity", "visible"],
                        "properties": {
                          "x": { "type": "number" },
                          "y": { "type": "number" },
                          "scaleX": { "type": "number" },
                          "scaleY": { "type": "number" },
                          "rotation": { "type": "number" },
                          "opacity": { "type": "number", "minimum": 0, "maximum": 1 },
                          "visible": { "type": "boolean" },
                          "assetId": { "type": "string" }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    },
    "timeline": {
      "type": "object",
      "required": ["defaultScene"],
      "properties": {
        "defaultScene": { "type": "string" }
      }
    }
  }
}
"#;

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
