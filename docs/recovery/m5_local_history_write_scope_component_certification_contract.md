# M5 Local-History / Write-Scope Component Surface Certification (M05-899)

This is the **closing capstone** of the B105 local-history / write-scope component lane. Where
the freeze matrix (`m5_local_history_write_scope_component_matrix.md`, M05-892) defines the seven
reusable components, the M05-893..896 primitive lanes narrow each one, the M05-897 consumer lane
proves they are reusable across the claimed mutation / recovery consumers, and the M05-898
accessibility / auto-narrowing capstone certifies keyboard / screen-reader / CLI / export parity
per family, this capstone **certifies** that the shared local-history / write-scope component
truth holds on every claimed M5 mutation and recovery surface — and auto-narrows any surface that
cannot sustain it.

- Boundary schema: `schemas/ui/m5-local-history-write-scope-component-certification.schema.json`
- Canonical support export: `artifacts/release/m5-local-history-write-scope-component-certification/support_export.json`
- Machine-readable matrix: `artifacts/release/m5-local-history-write-scope-component-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-local-history-write-scope-component-certification/report.md`
- Fixtures mirror: `fixtures/ui/m5-local-history-write-scope-component-certification/`
- Implementation: `crates/aureline-history/src/certify_local_history_row_checkpoint_group_restore_preview_card_retention_export_card_and_write_scope_preview_tree_truth_on_every_claimed_m5_mutation_and_recovery_surface/`

## What it certifies

The packet is keyed on the claimed **surface** a user restores from, applies broad changes
through, or exports a history from — not on component family or primitive lane. The eight
certified surfaces are:

`editor_rename_refactor`, `replace_in_files`, `import_migration`, `repair_transaction`,
`generated_artifact`, `ai_review_apply`, `recovery_console`, and `support_export`.

Each surface is scored on six truth axes:

1. `visual` — snapshot origin, actor lineage, file / object identity, branch / worktree context,
   external drift, generated / managed boundary, restore granularity, selectable apply scope, and
   retention / redaction posture are shown on the primary surface.
2. `keyboard` — the same truth and its controls are reachable without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color / glyph only.
4. `cli_export` — **always-on**: the certified surface state is reconstructable as text / JSON /
   Markdown from the same history identity.
5. `degraded_state` — a metadata-only capture, an unavailable / expired checkpoint, or a stale
   scope honestly downgrades a `restorable_checkpoint` / `reviewable_history` claim.
6. `mutation_and_recovery_provenance` — origin / actor / identity / context / drift / boundary /
   granularity / scope / retention stay explicit before any restore or multi-file apply commits,
   never inheriting a healthier lane's truth, and **restore never erases history**.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`restorable_checkpoint` / `reviewable_history` claim while a truth axis is not current — capture
was metadata-only, the restore is a partial / manual scope, the write / restore scope drifted, a
checkpoint is unavailable / expired, or a generated / managed caveat is unstated — is over-claiming
and is blocked (`red`). A surface that discloses the reduction by narrowing its support claim (with
a bound reason and a frozen downgrade trigger) is honestly `yellow`. The always-on `cli_export`
axis must always stay certified. **Restoring never erases history**: a narrowed restore adds a new
checkpoint rather than rewriting the timeline (`history_preserved` / `preserves_history_integrity`).

The support-claim ladder (strongest first) is reused from the M05-898 accessibility capstone:
`restorable_checkpoint` (5) > `reviewable_history` (4) > `narrowed_restore` (3) >
`metadata_only_history` (2) > `stale_scope_history` (1) > `unavailable_checkpoint` (0).
Certification may only narrow a claim, never strengthen it.

## Derived verdict

`derived_status` is never asserted by the author — it is recomputed from the axis outcomes,
history preservation, export parity, and claim narrowing. A row is `red` when it is malformed,
drops CLI/export parity, erases history, hides an undisclosed drift, retains a degraded axis behind
a full claim, or carries an inconsistent narrowing. It is `yellow` when a degraded axis is disclosed
and bound to a visible claim narrowing, and `green` when every axis certifies at the claimed tier.

## Coverage

The canonical packet certifies all eight surfaces exactly once (four `green`, four `yellow`, zero
`red`), every one of the seven frozen component families on at least one surface, every axis on
every row, and history preservation on every surface. Every row cites the one canonical proof
bundle (`artifacts/release/m5-local-history-write-scope-component-proof/support_export.json`) plus
the M05-897 consumer and M05-898 accessibility support exports as supporting evidence.

The `yellow` rows exercise the spec's compatibility paths: scope-drift (`import_migration` →
`stale_scope_history`), partial / manual restore (`generated_artifact` → `narrowed_restore`),
metadata-only capture (`repair_transaction` → `metadata_only_history`), and an unavailable /
expired checkpoint (`ai_review_apply` → `unavailable_checkpoint`).

## Regenerating the artifacts

The seed builder (`seeded_m5_local_history_write_scope_component_certification_packet`) is the one
source of truth for both the tests and the on-disk export. To regenerate:

```
GEN_HISTORY_CERT_ARTIFACTS=1 cargo test -p aureline-history --lib \
  certify_local_history_row_checkpoint_group_restore_preview_card_retention_export_card_and_write_scope_preview_tree_truth_on_every_claimed_m5_mutation_and_recovery_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the
seed builder. The packet is metadata-only: raw file bodies, snapshot contents, diffs, and
credentials never cross this boundary.
