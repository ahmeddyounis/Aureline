# Docs-browser source/result truth — milestone note

This is the milestone-level note for the docs-browser source/result truth lane.
The authoritative contract document is
`docs/search/m4/docs_browser_truth_packet.md`. The canonical checked-in
artifact is `artifacts/search/m4/docs_browser_truth_packet.json`. The schema
lives at `schemas/docs/docs_browser_truth_packet.schema.json`. The fixture
corpus lives under `fixtures/search/m4/docs_browser_truth_packet/`.

The implementation lives at
`crates/aureline-docs/src/docs_browser_truth_packet/mod.rs` and is replayed by
`crates/aureline-docs/tests/docs_browser_truth_packet.rs`.
