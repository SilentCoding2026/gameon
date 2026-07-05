use std::path::{Path, PathBuf};
use std::{fs, io};
use serde_json;
use crate::models::EngineProject;
use crate::validator;

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Json(serde_json::Error),
    Validation(String),
    MissingProjectFile(PathBuf),
    Deserialization(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "I/O error: {}", e),
            LoadError::Json(e) => write!(f, "JSON parse error: {}", e),
            LoadError::Validation(s) => write!(f, "Validation error: {}", s),
            LoadError::MissingProjectFile(p) => write!(f, "Project file not found: {}", p.display()),
            LoadError::Deserialization(s) => write!(f, "Deserialization error: {}", s),
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::Io(e) => Some(e),
            LoadError::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        LoadError::Io(e)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(e: serde_json::Error) -> Self {
        LoadError::Json(e)
    }
}

/// Load a project from a directory containing a `project.json` file.
/// All file paths inside the project are relative to this directory.
pub fn load_project(project_dir: &Path) -> Result<EngineProject, LoadError> {
    let project_file = project_dir.join("project.json");
    if !project_file.exists() {
        return Err(LoadError::MissingProjectFile(project_file));
    }

    let raw = fs::read_to_string(&project_file)?;
    let json_value: serde_json::Value = serde_json::from_str(&raw)?;

    validator::validate_project(&json_value)
        .map_err(|e| LoadError::Validation(e.to_string()))?;

    let project: EngineProject = serde_json::from_value(json_value)
        .map_err(|e| LoadError::Deserialization(e.to_string()))?;

    // Future: resolve asset paths relative to project_dir

    Ok(project)
}
