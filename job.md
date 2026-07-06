You are a senior Rust systems engineer.

You have NO prior context. Everything you need is defined below.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PROJECT CONTEXT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Project name:
AI-Assisted Offline Animation Engine (Human-Controlled)

This is a PROFESSIONAL, OFFLINE 2D animation engine.

IMPORTANT CONSTRAINTS:
- No UI
- No internet access
- No AI usage inside the engine
- Deterministic behavior
- Human approval required before export

The engine operates via command-line execution.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
EXISTING CODE ASSUMPTIONS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Assume ALL of the following modules already exist and are correct:

- engine::loader
- engine::validator
- engine::models
- engine::scene_graph
- engine::animation
- engine::timeline
- engine::approval
- engine::renderer
- engine::export
- engine::runtime (or equivalent integration module)
- engine::lib exposes a public runtime API

DO NOT reimplement any of these modules.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PURPOSE OF main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The purpose of main.rs is ONLY to act as a CLI entry point.

Responsibilities of main.rs:
1. Parse command-line arguments
2. Validate required arguments
3. Call the engine runtime
4. Print clear logs
5. Exit with non-zero code on failure

main.rs MUST NOT:
- Contain engine logic
- Contain rendering logic
- Contain animation logic
- Contain validation logic

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
COMMAND-LINE INTERFACE SPEC
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The executable is called with:

cargo run -- <project_path>

Where:
- <project_path> is a directory containing JSON project files

Behavior:
- If no argument is provided → print usage and exit with error
- If path does not exist → print error and exit
- If engine runtime fails → print error and exit non-zero
- On success → print clear progress logs

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
LOGGING REQUIREMENTS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

At minimum, print:
- Engine start
- Project path
- Runtime start
- Runtime completion
- Error messages if any

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
FILES TO CREATE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

engine/src/main.rs

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
OUTPUT RULES (ABSOLUTE)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

- DO NOT explain
- DO NOT add markdown
- DO NOT add extra files
- ONLY generate the file content
- File must be complete and compilable

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
OUTPUT FORMAT (STRICT)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

--- FILE: engine/src/main.rs ---
<full Rust source code>