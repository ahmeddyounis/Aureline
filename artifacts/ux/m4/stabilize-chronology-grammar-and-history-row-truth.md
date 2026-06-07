# Stabilize Chronology Grammar And History Row Truth

Release evidence:

- Canonical Rust model: `crates/aureline-chronology/src/stabilize_chronology_grammar_and_history_row_truth/mod.rs`
- Boundary schema: `schemas/ux/chronology-history-row.schema.json`
- UX contract note: `docs/ux/m4/stabilize-chronology-grammar-and-history-row-truth.md`
- Fixture packet: `fixtures/ux/m4/stabilize-chronology-grammar-and-history-row-truth/chronology_packet.json`
- Export packet: `fixtures/ux/m4/stabilize-chronology-grammar-and-history-row-truth/support_export_packet.json`
- Accessibility fixture: `fixtures/ux/m4/stabilize-chronology-grammar-and-history-row-truth/accessibility_fixture.json`

Validation coverage:

- actor/action/object/outcome grammar is deterministic and rejects drift
- required stable surfaces are present: activity, task/test, debug, provider, AI, policy, and recovery
- provenance badges are preserved into export markers
- absolute timestamps, relative-age hints, freshness, stale reasons, and visible reasons are preserved into exports
- local follow-up controls remain local for provider-owned rows
- exact reopen targets are required and generic home/search fallbacks are rejected
- accessibility projections expose identity, provenance, time posture, follow-up state, keyboard paths, and reduced-motion behavior without relying on color or hover
