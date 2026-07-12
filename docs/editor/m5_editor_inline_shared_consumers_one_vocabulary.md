# M5 Editor-Inline Shared Consumers: One Vocabulary Across Surfaces

Closing **consumer-adoption** lane for the B133 editor-inline component batch
(M05-1121). It binds the eight reusable inline components frozen by the
[editor-inline component matrix](m5_editor_inline_components_contract.md) and built by
the editor-tab/gutter, diagnostic-decoration/code-action-chip, diff-view/review-thread,
and AI-message-card/evidence-timeline implement lanes to the concrete editor, diff,
review, notebook, AI, diagnostics, CLI/export, support, and product **consumers** that
render them.

The lane proves — by checked fixtures, not screenshots — that the eight components are
**shared product infrastructure** rather than one-surface renderer chrome.

- **Module:** `crates/aureline-editor/src/m5_editor_inline_shared_consumers_one_vocabulary_across_surfaces`
- **Schema:** [`schemas/ui/m5-editor-inline-shared-consumers.schema.json`](../../schemas/ui/m5-editor-inline-shared-consumers.schema.json)
- **Support export:** [`artifacts/release/m5-editor-inline-shared-consumers-proof/support_export.json`](../../artifacts/release/m5-editor-inline-shared-consumers-proof/support_export.json)
- **Fixtures:** [`fixtures/ui/m5-editor-inline-shared-consumers/`](../../fixtures/ui/m5-editor-inline-shared-consumers/)

## The three honesty axes

The packet validation (`M5EditorInlineSharedConsumersPacket::validate`) proves the three
batch acceptance criteria.

1. **Reuse.** Each of the eight components is adopted by **at least two distinct
   consumers** (`inline_component_reuse_unproven` otherwise), and every component and
   every consumer surface appears among the bindings (`component_coverage_missing` /
   `consumer_coverage_missing`).
2. **One vocabulary / no drift.** Every binding for the same `inline_object_id` presents
   **identical** `state_facets` — the same state word, severity/confidence word,
   anchor/freshness word, approval word, and evidence-lineage word
   (`vocabulary_drift_across_surfaces` otherwise). The `state_word` must be a token drawn
   from the frozen `M5EditorInlineDisposition` vocabulary
   (`state_word_outside_vocabulary` otherwise), so no feature rewrites *modified*,
   *outdated*, *resolved*, or *inferred_fix* in its own words.
3. **Map back to one family.** Support and CLI/export consumers must reference the
   canonical per-component schema and the frozen matrix schema by id
   (`support_export_reference_missing` otherwise), so an exported packet always maps
   inline state back to one shared contract family.

## Representations and disclosure

A surface may narrow *how much* it renders across four representations without ever
rewording the vocabulary:

| Representation | Parity state | Disclosure carried |
| --- | --- | --- |
| `desktop_full` | `facets_preserved` | none |
| `compact_narrowed` | `facets_disclosed_narrowed` | narrow note (`compaction_narrowed`, expand-in-desktop) |
| `remote_projected` | `facets_disclosed_narrowed` | narrow note (`remote_projection_narrowed`) + remote-source note |
| `exported_redacted` | `facets_disclosed_narrowed` | narrow note (`export_redaction_narrowed`) + export-safe-evidence note |

`resolve_editor_inline_render_disclosure` is the single source of truth for what a
representation must disclose. A narrowed binding that omits its note, or a full-desktop
binding that carries one, is rejected (`narrow_note_missing` / `unexpected_narrow_note`).

## Guardrails

Every binding asserts, and validation enforces, that it never:

- encodes inline state by color alone (`state_encoded_by_color_alone`);
- lets a comment anchor or evidence pointer silently drift (`anchor_or_evidence_pointer_drift`);
- blurs outdated and resolved review state (`outdated_resolved_blurred`);
- presents an inferred fix as exact (`inferred_fix_shown_as_exact`);
- hides an evidence timeline in an opaque log (`evidence_hidden_in_opaque_log`);
- rewords the inline vocabulary per surface (`vocabulary_reworded_per_surface`).

These mirror the five hard invariants of the frozen matrix plus the consumer-lane parity
guardrail.

## Coverage

The checked packet carries **21 bindings** across **8 inline objects** — one per
component — spanning all nine consumer surfaces and all four representations. The two
narrowed fixtures re-derive the same objects with additional surfaces pushed into
compact/remote and exported/redacted representations, showing that inline component-state
changes propagate consistently across desktop, compact, remote, and exported forms.

## Regenerating the artifacts

The example is the only mint-from-truth path:

```text
cargo run -p aureline-editor --example dump_m5_editor_inline_shared_consumers -- support-export > artifacts/release/m5-editor-inline-shared-consumers-proof/support_export.json
cargo run -p aureline-editor --example dump_m5_editor_inline_shared_consumers -- csv           > artifacts/release/m5-editor-inline-shared-consumers-proof/matrix.csv
cargo run -p aureline-editor --example dump_m5_editor_inline_shared_consumers -- report        > artifacts/release/m5-editor-inline-shared-consumers-proof/summary.md
cargo run -p aureline-editor --example dump_m5_editor_inline_shared_consumers -- fixture-compact-remote-narrowed    > fixtures/ui/m5-editor-inline-shared-consumers/compact_remote_narrowed.json
cargo run -p aureline-editor --example dump_m5_editor_inline_shared_consumers -- fixture-exported-redaction-narrowed > fixtures/ui/m5-editor-inline-shared-consumers/exported_redaction_narrowed.json
```

`checked_support_export_validates_and_matches_seed` and
`checked_narrowed_fixtures_validate_and_match_builders` byte-lock the checked files to the
seed builders.
