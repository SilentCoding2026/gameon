pub mod models;
pub mod loader;
pub mod validator;
pub mod scene_graph;
pub mod animation;
pub mod timeline;
pub mod approval;
pub mod renderer;
pub mod export;
pub mod runtime;

pub use loader::load_project;
pub use models::EngineProject;

pub fn run_cli(project_path: &std::path::Path) -> Result<(), String> {
        crate::runtime::run(project_path).map(|_| ()).map_err(|e| e.to_string())
}
