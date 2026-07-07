# M5 local-history / write-scope component accessibility & auto-narrowing (M05-898)

The **accessibility-and-auto-narrowing capstone** over the frozen M5 local-history /
write-scope component matrix. Where the freeze matrix
(`schemas/ui/m5-local-history-write-scope-component-matrix.schema.json`) defines the seven
governed component families and the M05-893 … M05-897 lanes narrow and adopt them, this lane
certifies — per component family — that mutation and recovery claims stay
**keyboard-complete, assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather
than presenting a metadata-only capture, a partial or manual restore, a stale write scope, an
unavailable checkpoint, or an export-limited history as a still fully-restorable checkpoint.

This closes the B105 batch: after the components are frozen (M05-892), narrowed into resolvers
(M05-893 … M05-896), and adopted by consumers (M05-897), M05-898 makes the family a
release-grade contract with accessibility, export, and downgrade parity.

- Module:
  `crates/aureline-history/src/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_capture_is_metadata_only_restore_is_partial_or_manual_scope_is_stale_or_checkpoints_are_unavailable_across_claimed_m5_recovery_components`
- Schema: `schemas/ui/m5-local-history-write-scope-component-accessibility-fallback.schema.json`
- Support export: `artifacts/release/m5-local-history-write-scope-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-local-history-write-scope-component-accessibility-fallback/matrix.csv`
- Report: `artifacts/release/m5-local-history-write-scope-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-local-history-write-scope-component-accessibility-fallback/`

The packet is **metadata-only**: it carries typed class tokens, opaque summary / evidence
refs, booleans, and redacted labels — never raw file bodies, snapshot contents, diff hunks, or
credential-bearing material — so support, release, and diagnostics exports can reconstruct
exactly what an accessible fallback would have shown without leaking history material.

## What it certifies (per family)

Each `HistoryComponentAccessibilityRow` keys on one frozen
`M5LocalHistoryWriteScopeComponentFamily` and reuses the frozen `M5HistoryRequiredLabel`,
`M5HistoryDowngradeTrigger`, and `M5HistoryConsumerSurface` vocabulary rather than minting
parallel synonyms, so certified labels stay byte-identical to the matrix.

- **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and headless-reachable path into the same snapshot origin, actor,
  capture fidelity, checkpoint lineage, restore granularity, drift, selectable apply scope,
  write-scope class, managed-file caveat, retention posture, and export/redaction posture the
  rich component shows. Hierarchy-heavy families (the write-scope preview tree's nested
  workspace-root / file-node hierarchy) additionally bind their tree to a flat list / textual
  path.
- **Export parity.** The support / release / evaluation export reconstructs each component's
  meaning from typed tokens and opaque refs without a screenshot.
- **Honest auto-narrowing.** When capture is metadata-only, a restore is partial or manual, a
  write scope is stale, a checkpoint is unavailable, or history export is redaction-limited,
  the component's history-support claim auto-narrows, discloses the narrowing with a precise
  trigger and binding dimension, and preserves the canonical snapshot / actor / file /
  checkpoint identity — the underlying history is never erased opaquely.
- **Cross-surface disclosure.** The same narrowed state surfaces in the editor timeline,
  checkpoint inspector, restore review, refactor preview, AI-apply review, recovery center,
  headless CLI, and support / release exports.

## Support-claim ladder

`M5HistorySupportClaim` (strongest → weakest), with each condition state's permitted ceiling:

| Support claim | Rank | Imposed by condition state |
| --- | --- | --- |
| `restorable_checkpoint` | 5 | `captured` |
| `reviewable_history` | 4 | (family-only full claim for the export manifest) |
| `narrowed_restore` | 3 | `narrowed_restore` (partial / manual restore) |
| `metadata_only_history` | 2 | `metadata_only` (metadata-only capture / redacted export) |
| `stale_scope_history` | 1 | `stale_scope` (drifted write / restore scope) |
| `unavailable_checkpoint` | 0 | `unavailable` (expired checkpoint / policy-blocked export) |

The effective claim never exceeds the permitted ceiling; a weak dimension binds an honest
narrow block that names the ceiling-imposing dimension and its frozen downgrade trigger. A
component with every dimension intact carries no spurious narrowing.

## Claim dimensions (1:1 with families)

| Family | Primary dimension | Frozen trigger |
| --- | --- | --- |
| local-history row | `capture_fidelity` | `capture_fidelity_masked` |
| checkpoint-group card | `checkpoint_availability` | `checkpoint_lineage_unstated` |
| restore-preview card | `restore_granularity` | `restore_granularity_collapsed` |
| retention/export card | `export_disclosure` | `retention_or_redaction_undisclosed` |
| write-scope preview tree | `scope_freshness` | `write_scope_understated` |
| restore-granularity selector | `restore_scope_selection` | `restore_granularity_collapsed` |
| history-export manifest | `manifest_export_disclosure` | `retention_or_redaction_undisclosed` |

## Seeded certification (2 green / 5 yellow / 0 red)

| Row | Family | Effective claim | Status |
| --- | --- | --- | --- |
| checkpoint-group card | checkpoint unavailable / expired | `unavailable_checkpoint` | yellow |
| history-export manifest | full lineage | `reviewable_history` | green |
| local-history row | metadata-only capture | `metadata_only_history` | yellow |
| restore-preview card | partial / manual restore | `narrowed_restore` | yellow |
| retention/export card | redaction-limited export | `metadata_only_history` | yellow |
| write-scope preview tree | stale scope (hierarchy-heavy) | `stale_scope_history` | yellow |
| restore-granularity selector | full selectable scope | `restorable_checkpoint` | green |

Together the rows exercise all seven families, all seven claim dimensions, all six support
claim tiers (as effective claims), and all nine consumer surfaces.

## Regenerating artifacts

The checked-in support export, CSV, report, and fixtures are emitted from the single seeded
builder (`seeded_m5_history_component_a11y_fallback_packet`) via a gated test:

```
GEN_HISTORY_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-history --lib generate_artifacts
```

Verify with:

```
cargo test -p aureline-history --lib implement_keyboard_screen_reader
```
