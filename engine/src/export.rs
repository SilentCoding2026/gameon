use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::models::EngineProject;

pub struct ExportSettings {
    pub fps: u32,
    pub codec: String,
    pub pixel_format: String,
}

impl Default for ExportSettings {
    fn default() -> Self {
        ExportSettings {
            fps: 24,
            codec: "libx264".into(),
            pixel_format: "yuv420p".into(),
        }
    }
}

pub fn export_video(
    project: &EngineProject,
    output_path: &Path,
    settings: ExportSettings,
) -> Result<(), ExportError> {
    let width = project.meta.width;
    let height = project.meta.height;

    let total_frames: u32 = project
        .scenes
        .iter()
        .map(|s| (s.duration * settings.fps as f32).ceil() as u32)
        .sum();

    let mut child = Command::new("ffmpeg")
        .args([
            "-y",
            "-f", "rawvideo",
            "-pix_fmt", "rgba",
            "-s", &format!("{}x{}", width, height),
            "-r", &settings.fps.to_string(),
            "-i", "-",
            "-c:v", &settings.codec,
            "-pix_fmt", &settings.pixel_format,
            "-an",
        ])
        .arg(output_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ExportError::FfmpegLaunch(e.to_string()))?;

    let stdin = child.stdin.take().ok_or_else(|| ExportError::FfmpegLaunch("stdin".into()))?;

    drop(stdin);

    let output = child
        .wait_with_output()
        .map_err(|e| ExportError::FfmpegProcess(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ExportError::FfmpegError(stderr.to_string()));
    }

    Ok(())
}

#[derive(Debug)]
pub enum ExportError {
    FfmpegLaunch(String),
    RenderError(u32, String),
    WriteError(u32, String),
    FfmpegProcess(String),
    FfmpegError(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::FfmpegLaunch(msg) => write!(f, "Failed to launch ffmpeg: {}", msg),
            ExportError::RenderError(frame, msg) => {
                write!(f, "Render error at frame {}: {}", frame, msg)
            }
            ExportError::WriteError(frame, msg) => {
                write!(f, "Write error at frame {}: {}", frame, msg)
            }
            ExportError::FfmpegProcess(msg) => write!(f, "ffmpeg process error: {}", msg),
            ExportError::FfmpegError(msg) => write!(f, "ffmpeg encoding error: {}", msg),
        }
    }
}

impl std::error::Error for ExportError {}