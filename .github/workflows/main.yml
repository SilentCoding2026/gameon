name: CI Build & Dry Execution

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

defaults:
  run:
    working-directory: engine

jobs:
  build-and-check:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust (rustup)
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          profile: minimal

      - name: Verify rustc and cargo versions
        run: |
          rustc --version
          cargo --version

      - name: Run cargo check
        run: cargo check

      - name: Run cargo build (release)
        run: cargo build --release