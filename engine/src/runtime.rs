use std::path::Path;
use crate::loader::load_project;

pub fn run(project_dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let project = load_project(project_dir)?;
    println!("[Runtime] Loaded project: {}", project.meta.name);
    println!("[Runtime] {}x{} @ {}fps", project.meta.width, project.meta.height, project.meta.fps);
    println!("[Runtime] {} scenes, {} assets", project.scenes.len(), project.assets.len());

    let output_path = project_dir.join("output").to_string_lossy().to_string();
    Ok(output_path)
}