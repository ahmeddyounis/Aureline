# Grapheme navigation cases

These fixtures cover cursor movement and deletion semantics that must remain
grapheme-aware (extended grapheme clusters) rather than operating on raw bytes
or Unicode scalar values.

The harness lives in `crates/aureline-editor/tests/grapheme_nav_cases.rs`.

