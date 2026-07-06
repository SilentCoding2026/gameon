use crate::models::EngineProject;

#[derive(Debug)]
pub enum BlendMode {
    Normal,
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct LayerRenderInfo {
    pub asset_id: String,
    pub position_x: f32,
    pub position_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

pub struct Renderer {
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(project: &EngineProject) -> Self {
        Renderer {
            width: project.meta.width,
            height: project.meta.height,
        }
    }

    pub fn render_frame(&mut self, project: &EngineProject, frame: u32) -> Result<Vec<u8>, RenderError> {
        let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
        Ok(buffer)
    }
}

#[derive(Debug)]
pub enum RenderError {
    AssetNotFound(String),
    AssetLoadError(String, String),
    EvaluationError(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::AssetNotFound(id) => write!(f, "Asset not found: {}", id),
            RenderError::AssetLoadError(id, msg) => write!(f, "Failed to load asset {}: {}", id, msg),
            RenderError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}