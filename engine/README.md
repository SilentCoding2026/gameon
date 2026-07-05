# Offline Animation Engine

This is the core engine of the AI-Assisted Offline Animation Engine project.
It is fully offline, deterministic, and completely controlled by a human operator.

## Purpose

- Load and manage animation projects defined entirely by data files (JSON).
- Provide a deterministic frame rendering pipeline.
- Support preview playback and final export only after explicit human approval.
- Never connect to any network or external AI service.

## Architecture

The engine is designed as a Rust library with a minimal binary entry point.
All animation logic lives inside the library, ensuring reusability and testability.

## Human Approval

Exporting any final video output is programmatically blocked unless the human
has explicitly marked the project as approved. There is no automatic approval.