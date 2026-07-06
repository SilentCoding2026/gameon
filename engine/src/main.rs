use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <project_path>");
        eprintln!("  <project_path> must be a directory containing JSON project files");
        std::process::exit(1);
    }

    let project_path = &args[1];
    let path = Path::new(project_path);

    if !path.exists() {
        eprintln!("Error: Project path does not exist: {}", project_path);
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("Error: Project path is not a directory: {}", project_path);
        std::process::exit(1);
    }

    println!("Engine start");
    println!("Project path: {}", project_path);
    println!("Runtime start");

    match animation_engine_lib::run_cli(path) {
        Ok(()) => {
            println!("Runtime end");
            println!("Engine finished successfully");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}