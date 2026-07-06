$content = @'
name: CI Build & Test

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

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable

      - name: Verify versions
        run: |
          rustc --version
          cargo --version

      - name: Cargo check
        run: cargo check --all-targets

      - name: Cargo test
        run: cargo test --all-targets

      - name: Cargo build (release)
        run: cargo build --release

      - name: Dry-run CLI
        run: cargo run --release -- ./nonexistent || true
        continue-on-error: true
'@
[System.IO.File]::WriteAllText('c:\Users\MoMah\Documents\animation\.github\workflows\main.yml', $content, [System.Text.Encoding]::UTF8)
Write-Host 'Workflow updated'The tool is repeatedly failing with `[object Object]`. Let me try a direct PowerShell approach:

⟦cmd¦run=powershell -NoProfile -NonInteractive -Command "Set-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value 'name: CI Build & Test' -Encoding UTF8; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value 'on:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '  push:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '    branches: [main]'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '  pull_request:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '    branches: [main]'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value 'defaults:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '  run:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '    working-directory: engine'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value 'jobs:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '  build-and-check:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '    runs-on: ubuntu-latest'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '    steps:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '      - name: Checkout repository'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        uses: actions/checkout@v4'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '      - name: Install Rust stable'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        uses: dtolnay/rust-toolchain@stable'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        with:'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '          toolchain: stable'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '      - name: Verify versions'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        run: |'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '          rustc --version'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '          cargo --version'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '      - name: Cargo check'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        run: cargo check --all-targets'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '      - name: Cargo test'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        run: cargo test --all-targets'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '      - name: Cargo build (release)'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        run: cargo build --release'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value ''; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '      - name: Dry-run CLI'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        run: cargo run --release -- ./nonexistent || true'; Add-Content -Path 'c:\Users\MoMah\Documents\animation\.github\workflows\main.yml' -Value '        continue-on-error: true'"