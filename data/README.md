# Animation Data Files

This directory contains all data that defines an animation project.
The engine reads these files to reconstruct scenes, assets, timelines, and settings.

## Data Format

- All data is stored as **JSON** files.
- Schema definitions (future) will be provided under `schema/`.
- Example projects go into `example/`.

## Role of Data

Data files serve as the single source of truth. They can be:

- Written by a human using any text editor.
- Generated externally by an AI assistant (outside the application).
- Imported, edited, and previewed inside the engine after human review.

The engine never alters these files automatically; all changes must be human-initiated.