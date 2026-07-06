## 📦 PROJECT HANDOFF – FULL TECHNICAL CONTEXT (FOR CODING AGENT)

### 1️⃣ Project Overview

This is a **Rust-based engine project** structured as a Cargo workspace / crate.

The project goal is to provide:

* A **library crate** (`engine`) exposing a stable API
* A **CLI binary** that runs the engine on a given project path
* CI compatibility (GitHub Actions)
* Clean separation between:

  * CLI entry point
  * Engine runtime logic
  * Internal modules

The project currently **has errors and architectural inconsistencies**, and needs debugging + correction.

---

### 2️⃣ Environment Constraints

* Rust toolchain is **NOT installed locally**
* Development & testing is expected to happen via:

  * **GitHub Actions**
  * Or remote CI runners
* Assume:

  * `cargo build`
  * `cargo test`
  * `cargo run`
    will be executed in CI

So:

* Code must compile on **stable Rust**
* No nightly features
* No system-specific assumptions

---

### 3️⃣ Current Project State

* All project files exist **EXCEPT** `engine/src/main.rs` (recently added manually)
* The current `main.rs` was AI-generated but is **likely incorrect**
* There are:

  * Compilation errors
  * Possibly wrong imports
  * Possibly wrong public API assumptions

The agent must **not trust previous AI output blindly**.

---

### 4️⃣ Architectural Decision (MANDATORY)

This decision is **fixed and must not be changed**.

#### 🔒 Stable Public API

The engine library **MUST expose exactly ONE CLI-facing function**:

```rust
pub fn run_cli(project_path: &std::path::Path) -> Result<(), String>
```

Rules:

* This function lives in the **library crate**
* It is the **only** function `main.rs` is allowed to call
* It must:

  * Validate input
  * Run the engine runtime
  * Return `Ok(())` on success
  * Return `Err(String)` on failure

---

### 5️⃣ CLI Responsibilities (`main.rs`)

The binary entry point:

File:

```
engine/src/main.rs
```

Responsibilities ONLY:

1. Parse CLI arguments
2. Validate argument count
3. Validate that the project path exists
4. Log lifecycle events
5. Call `run_cli(...)`
6. Exit with non-zero code on error

❌ Must NOT:

* Contain engine logic
* Import internal modules
* Guess function names
* Use unstable APIs

---

### 6️⃣ CLI Behavior Specification

Command usage:

```
<binary> <project_path>
```

Rules:

* Exactly **1 argument required**
* On error:

  * Print clear message to stderr
  * Exit with code `1`
* On success:

  * Print lifecycle logs

Required logs (order matters):

1. `Engine start`
2. `Project path: <path>`
3. `Runtime start`
4. `Runtime end`
5. `Engine finished successfully`

---

### 7️⃣ Engine Library Responsibilities

The library crate (`engine`) must:

* Expose `run_cli(...)` publicly
* Internally:

  * Initialize runtime
  * Load project from path
  * Execute logic
* Internals can change freely
* Public API must remain stable

If internal APIs are broken or mismatched:

* Fix them
* Refactor safely
* Prefer clarity over cleverness

---

### 8️⃣ Error Handling Policy

* Internals may use:

  * `Result<T, E>`
  * Custom error enums
* BUT:

  * `run_cli` MUST convert errors to `String`
  * No panics
  * No unwraps in public path

---

### 9️⃣ CI Expectations

The following commands must succeed in CI:

```bash
cargo build --all
cargo test --all
cargo run -- <some_existing_path>
```

If tests are missing:

* That is acceptable
* Build must still pass

---

### 🔟 What the Agent Must Do

The agent is expected to:

1. Inspect the entire repository
2. Identify:

   * Broken imports
   * Wrong module visibility
   * API mismatches
3. Fix:

   * `main.rs`
   * `lib.rs`
   * Any broken internal modules
4. Ensure:

   * Clean compilation
   * Correct architecture
   * CI safety

---

### 1️⃣1️⃣ What NOT to Do

❌ Do NOT:

* Rewrite the project in another language
* Remove the CLI
* Add unnecessary dependencies
* Over-engineer
* Introduce async unless already used

---

### 1️⃣2️⃣ Success Criteria (IMPORTANT)

The task is successful ONLY if:

* `cargo build` passes
* `cargo run -- <valid_path>` works
* `main.rs` only calls `run_cli`
* Architecture matches this document
* Code is readable and maintainable

---

### 1️⃣3️⃣ Authority Note

This document overrides:

* Previous AI output
* Incomplete comments
* Incorrect assumptions

If anything conflicts:
➡️ **Follow this document**

---

## ✅ End of Handoff

---

اگر خواستی، در پیام بعدی می‌توانم:

* همین متن را **به انگلیسی رسمی‌تر مخصوص Devin / OpenHands** بازنویسی کنم
* یا **نسخه مخصوص Cursor Agent / Claude Code** بدهم
* یا یک **checklist CI debug** اضافه کنم

فقط بگو کدام agent را می‌خواهی.
