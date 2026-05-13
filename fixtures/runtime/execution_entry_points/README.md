# Execution Entry Point Fixtures

These fixtures prove that terminal, task, test, debug-prep, and AI-tool
entry points render one shared execution-context summary shape and expose
exact-rerun drift before dispatch.

The shell projection under `crates/aureline-shell/src/run_context` consumes
the canonical runtime resolver object. Fixtures intentionally assert field
tokens rather than prose labels so entry points do not fork target,
toolchain, prebuild, helper-boundary, or provenance vocabulary.

Verify with:

```sh
cargo test -p aureline-shell run_context
```
