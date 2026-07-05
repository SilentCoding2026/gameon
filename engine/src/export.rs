
use std::io::Write;
use std::process::{Command, Stdio};

use crate::renderer::Renderer;
use engine_core::project::Project;
use image::RgbaImage;

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

/// Export the approved animation project to an MP4 video file.
///
/// # Panics
/// Panics immediately if the project has not been explicitly approved by a human.
pub fn export_video(
    project: &Project,
    output_path: &str,
    settings: ExportSettings,
) -> Result<(), ExportError> {
    // ---------- HUMAN APPROVAL GUARD ----------
    if !project.approved {
        panic!(
            "Export blocked: project has not been approved by a human operator. \
             Open the editor and set 'approved = true' after review."
        );
    }

    let width = project.width;
    let height = project.height;
    let total_frames = (project.duration_seconds * settings.fps as f32).ceil() as u32;

    // Launch ffmpeg with raw RGBA input via stdin
    let mut child = Command::new("ffmpeg")
        .args(&[
            "-y", // overwrite output
            "-f", "rawvideo",
            "-pix_fmt", "rgba",
            "-s", &format!("{}x{}", width, height),
            "-r", &settings.fps.to_string(),
            "-i", "-",
            "-c:v", &settings.codec,
            "-pix_fmt", &settings.pixel_format,
            "-an", // no audio
            output_path,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ExportError::FfmpegLaunch(e.to_string()))?;

    let mut stdin = child.stdin.take().expect("Failed to capture ffmpeg stdin");

    // Reuse a single renderer to keep asset cache warm
    let mut renderer = Renderer::new(project);

    for frame_idx in 0..total_frames {
        let image: RgbaImage = renderer
            .render_frame(project, frame_idx)
            .map_err(|e| ExportError::RenderError(frame_idx, e.to_string()))?;

        stdin
            .write_all(&image.into_raw())
            .map_err(|e| ExportError::WriteError(frame_idx, e.to_string()))?;
    }

    // Close stdin to signal end of stream
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

// ---------- error type ----------

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