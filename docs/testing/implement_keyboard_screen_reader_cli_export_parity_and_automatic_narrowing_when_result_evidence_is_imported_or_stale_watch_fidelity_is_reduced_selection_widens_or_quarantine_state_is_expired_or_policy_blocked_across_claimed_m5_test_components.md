# M5 Test-Explorer / Watch / Triage Component Accessibility & Auto-Narrowing (M05-914)

This contract certifies keyboard / screen-reader / CLI / export **parity** and honest
**automatic narrowing** for the frozen M5 test-explorer / watch / triage component families —
test-tree row, inline result marker, session-summary bar, watch-mode banner, failure-triage
panel, quarantine-review sheet, and environment-matrix card.

It is the B107 accessibility-and-auto-narrowing capstone over the frozen component matrix
(`schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json`). Where the freeze
matrix defines the reusable primitives and the 909–913 implementation / consumer lanes resolve
their per-surface truth, this lane proves that, per family, test-result claims stay
keyboard-complete, assistive-tech-reachable, CLI/export-safe, and self-narrowing.

## Boundary

- Schema: `schemas/ui/m5-test-explorer-watch-triage-component-accessibility-fallback.schema.json`
- Support export: `artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback/matrix.csv`
- Markdown report: `artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-test-explorer-watch-triage-component-accessibility-fallback/`

The packet is **metadata-only**: it carries typed class tokens, opaque summary / evidence refs,
booleans, and redacted labels. Raw logs, assertion bodies, transcripts, attachment bytes, and
credential-bearing material never cross this boundary, so support, release, and diagnostics
exports can reconstruct exactly what an accessible fallback would have shown without leaking test
material.

## Reused frozen vocabulary

The capstone certifies the freeze matrix's families rather than minting parallel synonyms. It
reuses, byte-identical:

- `M5TestExplorerWatchTriageComponentFamily` — the seven governed families.
- `M5TestConsumerSurface` — the nine consumer surfaces the narrowed state must reach.
- `M5TestRequiredLabel` — required labels (`identity`, `state`, `keyboard_route` mandatory).
- `M5TestDowngradeTrigger` — the frozen downgrade-trigger vocabulary each narrowing names.

## Keyboard / screen-reader / CLI reach

Every family exposes a keyboard-complete, screen-reader-reachable, and CLI/headless-reachable
path into the same test identity class, imported/live result origin, freshness, target class,
environment lane, watch fidelity, retry / attempt lineage, mute / quarantine ownership, and
release impact the rich component shows. Keyboard / screen-reader flows cover run, rerun-failed,
debug-failed, open-triage, expand-parameterized-cases, inspect-watch-state, and review-quarantine.
Hierarchy-heavy families (the environment-matrix card's nested target × environment legs and the
failure-triage panel's nested recent attempts) additionally bind their tree to a flat list /
textual path.

## Export parity

The support / release / CLI export reconstructs each component's meaning from typed tokens and
opaque refs without a screenshot, preserving the same stable test IDs, target class, freshness,
watch-state vocabulary, quarantine ownership, and widening-selection notes shown in-product. Each
component's fallback is copyable as text / JSON / Markdown; a screenshot is never the only export.

## Honest auto-narrowing

When result evidence is imported or stale, watch fidelity is reduced, the rerun selection widens,
or a quarantine is expired / policy-blocked, the component's test claim auto-narrows from
`TrustedLiveResult` / `ReviewableResult` to an imported-or-stale / reduced-watch /
widened-selection / restricted-quarantine result. The narrowing discloses a precise trigger and
binding dimension and preserves the canonical identity / origin / attempt / retry lineage — the
underlying result lineage is never dropped opaquely. A component with every dimension intact must
NOT carry a spurious narrowing.

### Dimension → condition → ceiling → frozen trigger

| Claim dimension        | Weak condition state             | Permitted ceiling               | Frozen trigger                     |
| ---------------------- | -------------------------------- | ------------------------------- | ---------------------------------- |
| `result_evidence`      | `evidence_imported_or_stale`     | `imported_or_stale_result`      | `result_origin_unstated`           |
| `watch_fidelity`       | `watch_fidelity_reduced`         | `reduced_watch_result`          | `watch_fidelity_unstated`          |
| `selection_scope`      | `selection_widened`              | `widened_selection_result`      | `rerun_scope_widened`              |
| `quarantine_visibility`| `quarantine_expired_or_blocked`  | `restricted_quarantine_result`  | `quarantine_release_impact_hidden` |

The `results_live_exact` baseline imposes no ceiling and permits the strongest
`trusted_live_result` claim.

## Cross-surface disclosure

The same narrowed state surfaces in the test-tree UI, editor-gutter, session-summary,
watch-banner, triage-panel, quarantine-sheet, headless CLI, and support / release exports so
product, docs, and release publication stay aligned on test-component downgrade behavior. A
live-looking red or green mark can never outrun the origin / freshness / watch / selection /
quarantine proof it is being viewed away from.

## Verification

```
cargo test -p aureline-runtime --lib -- implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_result_evidence
```

Regenerate the checked-in artifacts + fixtures (gated):

```
GEN_TEST_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-runtime --lib -- generate_artifacts
```
