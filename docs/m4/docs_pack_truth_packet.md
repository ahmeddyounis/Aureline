# Docs-pack truth — milestone note

This is the milestone-level note for the docs-pack truth lane that hardens
docs-pack manifests, mirror/offline truth, stale-example detection, and
citation-set export across help surfaces. The authoritative contract document
is `docs/search/m4/docs_pack_truth_packet.md`. The canonical checked-in
artifact is `artifacts/search/m4/docs_pack_truth_packet.json`. The schema lives
at `schemas/docs/docs_pack_truth_packet.schema.json`. The fixture corpus lives
under `fixtures/search/m4/docs_pack_truth_packet/`.

The implementation lives at
`crates/aureline-docs/src/docs_pack_truth_packet/mod.rs` and is replayed by
`crates/aureline-docs/tests/docs_pack_truth_packet.rs`.
