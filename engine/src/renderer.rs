use engine_core::project::Project;
use engine_core::layer::{LayerRenderInfo, BlendMode};
use image::{RgbaImage, Rgba};
use std::collections::HashMap;
use std::sync::Arc;

/// Deterministic 2D renderer that produces frames for a given project.
pub struct Renderer {
    width: u32,
    height: u32,
    asset_cache: HashMap<String, Arc<RgbaImage>>,
}

impl Renderer {
    pub fn new(project: &Project) -> Self {
        Renderer {
            width: project.width,
            height: project.height,
            asset_cache: HashMap::new(),
        }
    }

    /// Render a single frame. Frame number starts at 0.
    /// All rendering is deterministic and repeatable.
    pub fn render_frame(&mut self, project: &Project, frame: u32) -> Result<RgbaImage, RenderError> {
        let layers = project
            .evaluate_frame(frame)
            .map_err(|e| RenderError::EvaluationError(e.to_string()))?;

        let mut canvas = RgbaImage::new(self.width, self.height);
        // Clear to transparent black
        for pixel in canvas.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }

        // Composite layers in order (background first)
        for layer in layers {
            self.composite_layer(&mut canvas, project, &layer)?;
        }

        Ok(canvas)
    }

    fn composite_layer(
        &mut self,
        canvas: &mut RgbaImage,
        project: &Project,
        info: &LayerRenderInfo,
    ) -> Result<(), RenderError> {
        if info.opacity <= 0.0 {
            return Ok(());
        }

        let asset_img = self.load_asset(project, &info.asset_id)?;
        let transformed = self.transform_image(&asset_img, info)?;

        let opacity = (info.opacity * 255.0).round() as u8;
        for (x, y, pixel) in transformed.enumerate_pixels() {
            let px = x as i32 + info.position_x as i32;
            let py = y as i32 + info.position_y as i32;
            if px < 0 || py < 0 || px as u32 >= self.width || py as u32 >= self.height {
                continue;
            }
            let existing = canvas.get_pixel(px as u32, py as u32);
            let src = pixel;
            let src_a = ((src[3] as f32) * (opacity as f32 / 255.0)).round() as u8;
            let blended = match info.blend_mode {
                BlendMode::Normal => {
                    blend_normal(existing, Rgba([src[0], src[1], src[2], src_a]))
                }
                BlendMode::Add => blend_add(existing, Rgba([src[0], src[1], src[2], src_a])),
                BlendMode::Multiply => {
                    blend_multiply(existing, Rgba([src[0], src[1], src[2], src_a]))
                }
            };
            canvas.put_pixel(px as u32, py as u32, blended);
        }
        Ok(())
    }

    fn load_asset(
        &mut self,
        project: &Project,
        asset_id: &str,
    ) -> Result<Arc<RgbaImage>, RenderError> {
        if let Some(img) = self.asset_cache.get(asset_id) {
            return Ok(Arc::clone(img));
        }
        let path = project
            .resolve_asset_path(asset_id)
            .ok_or_else(|| RenderError::AssetNotFound(asset_id.to_string()))?;
        let img = image::open(&path)
            .map_err(|e| RenderError::AssetLoadError(asset_id.to_string(), e.to_string()))?
            .to_rgba8();
        let img = Arc::new(img);
        self.asset_cache
            .insert(asset_id.to_string(), Arc::clone(&img));
        Ok(img)
    }

    fn transform_image(
        &self,
        src: &RgbaImage,
        info: &LayerRenderInfo,
    ) -> Result<RgbaImage, RenderError> {
        // Scale (nearest-neighbour for deterministic output)
        let scaled = if (info.scale_x - 1.0).abs() > f32::EPSILON
            || (info.scale_y - 1.0).abs() > f32::EPSILON
        {
            let new_w = (src.width() as f32 * info.scale_x).round() as u32;
            let new_h = (src.height() as f32 * info.scale_y).round() as u32;
            image::imageops::resize(src, new_w, new_h, image::imageops::FilterType::Nearest)
        } else {
            src.clone()
        };

        // Arbitrary rotation around centre using nearest-neighbour sampling
        let rotated = if info.rotation != 0.0 {
            let (w, h) = scaled.dimensions();
            let rad = info.rotation.to_radians();
            let cos = rad.cos();
            let sin = rad.sin();
            let new_w = (w as f32 * cos.abs() + h as f32 * sin.abs()).ceil() as u32;
            let new_h = (w as f32 * sin.abs() + h as f32 * cos.abs()).ceil() as u32;
            let mut out = RgbaImage::new(new_w, new_h);
            let cx = w as f32 / 2.0;
            let cy = h as f32 / 2.0;
            let ncx = new_w as f32 / 2.0;
            let ncy = new_h as f32 / 2.0;
            for y in 0..new_h {
                for x in 0..new_w {
                    let src_x = (x as f32 - ncx) * cos + (y as f32 - ncy) * sin + cx;
                    let src_y = -(x as f32 - ncx) * sin + (y as f32 - ncy) * cos + cy;
                    let sx = src_x.round() as i32;
                    let sy = src_y.round() as i32;
                    if sx >= 0 && sy >= 0 && sx < w as i32 && sy < h as i32 {
                        out.put_pixel(x, y, *scaled.get_pixel(sx as u32, sy as u32));
                    }
                }
            }
            out
        } else {
            scaled
        };

        Ok(rotated)
    }
}

// ---------- blending helpers ----------

fn blend_normal(bg: &Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
    let fg_a = fg[3] as f32 / 255.0;
    let bg_a = bg[3] as f32 / 255.0;
    let out_a = fg_a + bg_a * (1.0 - fg_a);
    if out_a == 0.0 {
        return Rgba([0, 0, 0, 0]);
    }
    let inv = 1.0 / out_a;
    let r = (fg[0] as f32 * fg_a + bg[0] as f32 * bg_a * (1.0 - fg_a)) * inv;
    let g = (fg[1] as f32 * fg_a + bg[1] as f32 * bg_a * (1.0 - fg_a)) * inv;
    let b = (fg[2] as f32 * fg_a + bg[2] as f32 * bg_a * (1.0 - fg_a)) * inv;
    Rgba([
        r.round() as u8,
        g.round() as u8,
        b.round() as u8,
        (out_a * 255.0).round() as u8,
    ])
}

fn blend_add(bg: &Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
    let fg_a = fg[3] as f32 / 255.0;
    let r = (bg[0] as f32 + fg[0] as f32 * fg_a).min(255.0);
    let g = (bg[1] as f32 + fg[1] as f32 * fg_a).min(255.0);
    let b = (bg[2] as f32 + fg[2] as f32 * fg_a).min(255.0);
    let a = (bg[3] as f32 + fg_a * 255.0).min(255.0);
    Rgba([r as u8, g as u8, b as u8, a as u8])
}

fn blend_multiply(bg: &Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
    let fg_a = fg[3] as f32 / 255.0;
    let norm = |c: u8| c as f32 / 255.0;
    let r = norm(bg[0]) * norm(fg[0]) * fg_a + norm(bg[0]) * (1.0 - fg_a);
    let g = norm(bg[1]) * norm(fg[1]) * fg_a + norm(bg[1]) * (1.0 - fg_a);
    let b = norm(bg[2]) * norm(fg[2]) * fg_a + norm(bg[2]) * (1.0 - fg_a);
    let a = fg_a + (1.0 - fg_a) * norm(bg[3]);
    Rgba([
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ])
}

// ---------- error type ----------

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