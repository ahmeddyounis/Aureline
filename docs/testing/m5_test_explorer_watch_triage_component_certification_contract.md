# M5 Test-Explorer / Watch / Triage Component Surface Certification (M05-915)

This is the **closing capstone** of the B107 test-explorer / watch / triage component lane. Where
the freeze matrix (`m5_test_explorer_watch_triage_component_matrix.md`, M05-908) defines the seven
reusable components, the M05-909..912 primitive lanes narrow each one, the M05-913 consumer lane
proves they are reusable across the claimed status-bar / activity / coverage / flaky / snapshot /
pipeline / imported-CI / support consumers, and the M05-914 accessibility / auto-narrowing capstone
certifies keyboard / screen-reader / CLI / export parity per family, this capstone **certifies** that
the shared test-explorer / watch / triage component truth holds on every claimed M5 test surface —
and auto-narrows any surface that cannot sustain it.

- Boundary schema: `schemas/ui/m5-test-explorer-watch-triage-component-certification.schema.json`
- Canonical support export: `artifacts/release/m5-test-explorer-watch-triage-component-certification/support_export.json`
- Machine-readable matrix: `artifacts/release/m5-test-explorer-watch-triage-component-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-test-explorer-watch-triage-component-certification/report.md`
- Fixtures mirror: `fixtures/ui/m5-test-explorer-watch-triage-component-certification/`
- Implementation: `crates/aureline-runtime/src/certify_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_truth_on_every_claimed_m5_test_surface/`

## What it certifies

The packet is keyed on the claimed **surface** a user reruns, debugs, suppresses, exports, or reviews
failing tests from — not on component family or primitive lane. The eight certified surfaces are:

`test_explorer_tree`, `editor_notebook_markers`, `status_bar_session_summary`, `watch_banner`,
`triage_panel`, `quarantine_review_sheet`, `imported_ci_view`, and `cli_export`.

Each surface is scored on six truth axes:

1. `visual` — test identity class, freshness, imported/live state, target and environment, watch
   fidelity, retry lineage, and mute / quarantine ownership are shown on the primary surface.
2. `keyboard` — the same truth and its rerun / debug / review controls are reachable without a
   pointer.
3. `screen_reader` — the same truth is announced non-visually, never color / glyph only.
4. `cli_export` — **always-on**: the certified surface state is reconstructable as text / JSON /
   Markdown from the same test identity.
5. `degraded_state` — imported or stale evidence, reduced watch fidelity, a widened rerun selection,
   or an expired / policy-blocked quarantine honestly downgrades a `trusted_live_result` /
   `reviewable_result` claim.
6. `test_intelligence_and_suppression_provenance` — test identity / freshness / imported-versus-live
   state / target / environment / watch fidelity / retry lineage / quarantine ownership / release
   impact stay explicit before any rerun, debug, suppression, export, or review, never inheriting a
   healthier lane's truth, and **rerun / debug / triage never drops result / attempt / retry
   lineage** between an imported or stale reading and a live local rerun.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`trusted_live_result` / `reviewable_result` claim while a truth axis is not current — the result
evidence is imported or stale, watch fidelity is reduced, the rerun selection widened, or the
quarantine state is expired or policy-blocked — is over-claiming and is blocked (`red`). A surface
that discloses the reduction by narrowing its test claim (with a bound reason and a frozen downgrade
trigger) is honestly `yellow`. The always-on `cli_export` axis must always stay certified. **Rerun /
debug / triage never drops lineage**: a narrowed surface preserves its result / attempt / retry
lineage continuity rather than dropping it between an imported or stale reading and a live local
rerun (`lineage_preserved` / `preserves_lineage_continuity`).

The test-claim ladder (strongest first) is reused from the M05-914 accessibility capstone:
`trusted_live_result` (5) > `reviewable_result` (4) > `widened_selection_result` (3) >
`reduced_watch_result` (2) > `imported_or_stale_result` (1) > `restricted_quarantine_result` (0).
Certification may only narrow a claim, never strengthen it.

## Derived verdict

`derived_status` is never asserted by the author — it is recomputed from the axis outcomes, lineage
preservation, export parity, and claim narrowing. A row is `red` when it is malformed, drops
CLI/export parity, drops lineage, hides an undisclosed drift, retains a degraded axis behind a full
claim, or carries an inconsistent narrowing. It is `yellow` when a degraded axis is disclosed and
bound to a visible claim narrowing, and `green` when every axis certifies at the claimed tier.

## Coverage

The canonical packet certifies all eight surfaces exactly once (four `green`, four `yellow`, zero
`red`), every one of the seven frozen component families on at least one surface, every axis on every
row, and lineage preservation on every surface. Every row cites the one canonical proof bundle
(`artifacts/release/m5-test-explorer-watch-triage-proof/support_export.json`) plus the M05-913
consumer and M05-914 accessibility support exports as supporting evidence.

The `yellow` rows exercise the spec's four auto-narrowing conditions: imported / stale evidence
(`imported_ci_view` → `imported_or_stale_result`), reduced watch fidelity (`watch_banner` →
`reduced_watch_result`), a widened rerun selection (`status_bar_session_summary` →
`widened_selection_result`), and an expired / policy-blocked quarantine (`quarantine_review_sheet` →
`restricted_quarantine_result`).

## Regenerating the artifacts

The seed builder (`seeded_m5_test_explorer_watch_triage_component_certification_packet`) is the one
source of truth for both the tests and the on-disk export. To regenerate:

```
GEN_TEST_CERT_ARTIFACTS=1 cargo test -p aureline-runtime --lib \
  certify_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_truth_on_every_claimed_m5_test_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the seed
builder. The packet is metadata-only: raw assertion diffs, log bodies, redacted evidence contents,
and credentials never cross this boundary.
