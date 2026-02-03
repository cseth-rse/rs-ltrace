# rs-ltrace

A minimal, from-scratch Rust implementation of a Linux syscall tracer using `ptrace`.

This project is the foundation of a long-term systems engineering effort to build a memory-safe, production-grade reimplementation of `ltrace` in Rust.

Current Version: **v0.1 – Fork/Exec Syscall Tracing Foundation**

---

# Philosophy

This project is:

* Systems-first
* Linux-first
* x86_64-first
* No frameworks
* Minimal dependencies (only `libc`)
* Built incrementally
* Architected for 80k+ lines of code

No external ELF parsers.
No debugging crates.
No tracing frameworks.

Everything is built manually for deep understanding.

---

# What v0.1 Does

This version implements:

* `fork()`-based tracing model
* `PTRACE_TRACEME` in child
* `execvp()` execution of target program
* Parent-driven syscall stepping via `PTRACE_SYSCALL`
* Basic register extraction
* Entry/exit syscall printing
* Clean process exit detection

This matches the core tracing loop model used by tools like `strace`.

---

# Build Instructions

Linux only.

```bash
cargo build
```

---

# Usage

You must use `--` twice:

```bash
cargo run -- -- sleep 5
```

Explanation:

* First `--` tells Cargo to pass arguments to your binary
* Second `--` separates tracer from traced program

General usage:

```bash
cargo run -- -- <program> [args...]
```

Example:

```bash
cargo run -- -- ls -la
```

---

# Example Output

```
syscall(59) args: ...
 → return = 0
syscall(12) args: ...
 → return = ...
...
Process exited.
```

---

# Project Structure (v0.1)

```
src/
├── main.rs        # CLI handling
├── tracer.rs      # Core tracing loop
├── ptrace.rs      # Raw ptrace wrappers
├── registers.rs   # Register abstraction layer
└── error.rs       # Error model
```
