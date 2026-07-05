# AI-Assisted Offline Animation Engine (Human-Controlled)

A professional 2D animation engine and editor. The system is fully offline, deterministic, and keeps the human in the loop at all times. External AI assistants may only produce draft data files (JSON) that a human can choose to import, review, and approve.

## Core Principles

- **Offline only** – no network access, no cloud dependency.
- **Deterministic** – same input always yields the same output.
- **Human-in-the-loop** – every action that modifies the project or triggers an export must be approved by the user.
- **AI is external** – the engine never talks to an AI. Any AI assistance happens outside the application via manual file exchange.

## Project Structure

```
/
├── engine/         # Rust engine (core library + binary)
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   ├── Cargo.toml
│   └── README.md
├── data/           # Animation project data (JSON)
│   ├── schema/     # JSON schemas for data validation
│   ├── example/    # Example projects
│   └── README.md
└── README.md       # This file
```

## Building

The engine is written in Rust and uses Cargo.

```bash
cd engine
cargo build
```

## Usage (Future)

The engine binary will accept commands for loading projects, previewing frames, and exporting video. All export operations require an explicit approval step.

## License

Proprietary. All rights reserved.