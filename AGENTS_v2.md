#PROJECT ai_offline_animation_engine
 #DIRECTORY c:\Users\MoMah\Documents\animation
  data/ # project JSON files and schemas
  engine/ # Rust crate (lib + bin)
  .github/workflows/ # CI: cargo check + build on push/PR to main

#ENTRYPOINTS
 engine/src/main.rs → animation_engine_lib::run_cli(path) # CLI binary
 engine/src/lib.rs → pub fn run_cli(project_path: &Path) -> Result<(), String> # single public API
 data/schema/project.schema.json → JSON Schema Draft-7 # validation schema

#MODULES
 engine/src/lib.rs → pub mod models, loader, validator, scene_graph, animation, timeline, approval, renderer, export, runtime
  models → EngineProject, ProjectMeta, Asset, Scene, Layer, Keyframe, KeyframeProperties, Timeline # serde Deserialize
  loader → load_project(project_dir) → Result<EngineProject, LoadError> # reads project.json, validates schema
  validator → validate_project(json) → Result<(), ValidationError> # jsonschema Draft-7 validation
  scene_graph → Mat3, Transform, SceneNode, SceneGraph # 2D affine transforms, world matrix traversal
  animation → Vec2, Easing, Keyframe<T>, Curve<T>, Transform, AnimationClip # deterministic keyframe sampling
  timeline → Timeline # frame-accurate playback control (play/pause/stop/seek/loop)
  approval → Approval # human-operator gate for export (approved: bool)
  renderer → Renderer::render_frame(_project, _frame) → Vec<u8> # stub - RGBA frame buffer
  export → export_video(project, output_path, settings) → Result<(), ExportError> # ffmpeg piping
  runtime → run(project_dir) → Result<String, Box<dyn Error>> # orchestrates load to export

#RUNTIME-GRAPH
 main.rs → run_cli(path) → runtime::run(path) → loader::load_project → validator::validate_project
 runtime::run → export::export_video → Renderer::render_frame → ffmpeg (external process)

#SCHEMA
 EngineProject { meta: ProjectMeta, assets: [Asset], scenes: [Scene], timeline: Timeline }
 ProjectMeta { version, name, width: u32, height: u32, fps: f32 }
 Asset { id, asset_type: AssetType::Image, path, width, height }
 Scene { id, name, duration: f32, layers: [Layer] }
 Layer { id, name, keyframes: [Keyframe] }
 Keyframe { time: f32, properties: KeyframeProperties }
 KeyframeProperties { x, y, scaleX, scaleY, rotation, opacity, visible, assetId? }
 AssetType = Image

#ENV
 CI: ubuntu-latest, Rust stable, no local Rust required
 External: ffmpeg (for video export)
