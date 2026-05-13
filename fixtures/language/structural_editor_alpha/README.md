# Structural Editor Alpha Fixtures

These fixtures exercise the editor-facing structural projection built from the
shared Tree-sitter runtime:

- Tree-sitter syntax-highlight spans for launch-wedge source files.
- Fold ranges with keyboard-accessible summary labels.
- File-local outline rows with provider and freshness state.
- Explicit degraded states for missing grammars and partial parse trees.

The fixture payloads intentionally keep source samples small and deterministic.
Runtime snapshots must exclude raw source from exported parse-session policy even
when local editor UI records include symbol labels and range references.
