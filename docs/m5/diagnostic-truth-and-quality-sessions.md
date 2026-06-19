# M5 diagnostic-record, source-descriptor, collection-snapshot, anchor-remap, and quality-session matrix

This document is the contract for the M5 diagnostic-truth freeze. It binds
**every claimed M5 diagnostic-producing surface** to a single bounded
diagnostic-truth lane, so Milestone 5 can ship its widening set of finding
surfaces with one canonical record, source, collection, remap, and
quality-session model instead of provider-local finding identities, hidden
freshness assumptions, or feature copy that outruns evidence.

M5 widens the surfaces that produce or preserve findings: notebook cells,
framework packs, request / API tooling, data tooling, preview runtimes, package
lanes, the language-provider plane, the editor-structural guard, and imported
scanner / SARIF snapshots. Those lanes only stay trustworthy if every finding
resolves to one canonical diagnostic identity with an explicit source kind,
imported-versus-live class, freshness, remap state, collection completeness,
cluster meaning, and the quality-session outcome that produced or can act on it —
rather than letting the editor, Problems, review, the CLI, AI evidence, and
support export each infer provider-local meanings.

The matrix is canonical: no editor, Problems, review, CLI/headless, AI-evidence,
support-export, or release-visible debt surface may present a greener claim than
this matrix, and any row that cannot identify a source kind, an
imported-versus-live origin, proven freshness, a remap state, a collection
completeness, or a governing quality session auto-downgrades before it publishes.

Unlike sources are never flattened into synthetic findings. Convenience
clustering never erases source, freshness, or remap provenance. Anchors are never
silently repaired; remap is append-only evidence. Imported or replayed evidence
never masquerades as live local truth. Partial, incremental, imported, or
filtered collection completeness stays disclosed and exportable. Target,
environment, and policy refs survive. Every mutating fix route is a typed
quality-action proposal with a safety class, a preview requirement, and a
rollback boundary.

## Source of truth

- Packet type: `DiagnosticTruthLaneMatrixPacket`
  (`crates/aureline-runtime/src/freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix/`).
- Boundary schema:
  `schemas/quality/m5-diagnostic-truth-lane.schema.json`.
- Checked support export:
  `artifacts/m5/diagnostics/freeze-packet/support_export.json`.
- Markdown summary:
  `artifacts/m5/diagnostics/freeze-packet/support_export.md`.
- Protected fixtures:
  `fixtures/quality/m5/diagnostic-contract-regression/`.

The canonical record/source/anchor-remap/cluster/plane-snapshot objects are owned
by `crates/aureline-runtime/src/diagnostics/`, and the quality-action / session
governance objects by `crates/aureline-runtime/src/quality/`. This freeze reuses
their vocabularies rather than minting synonyms; downstream support exports, AI
evidence, review surfaces, and release-visible debt packets ingest the lane
directly through `current_m5_diagnostic_truth_lane_export()` instead of cloning
provider-local state.

## Claimed surfaces

Each row maps one diagnostic-producing surface onto the canonical lane:

- `notebook_cell_diagnostics`
- `framework_pack_diagnostics`
- `request_tooling_diagnostics`
- `data_tooling_diagnostics`
- `preview_runtime_diagnostics`
- `package_lane_diagnostics`
- `language_provider_diagnostics`
- `editor_structural_diagnostics`
- `imported_scanner_diagnostics`

## Per-row dimensions

Each `DiagnosticLaneRow` carries:

- `source_kind` — reused `DiagnosticSourceKind` (editor-structural, language
  service, build/task, runtime/test, scanner import, policy, heuristic).
- `origin_class` — reused `DiagnosticOriginClass` naming imported-versus-live
  truth (live local / remote, managed provider live, imported snapshot, replayed
  support bundle, local cache).
- `freshness_class` — reused `DiagnosticFreshnessClass` (current, recent,
  warm/degraded cached, stale, superseded, imported snapshot, unverified).
- `remap_state_class` — reused `DiagnosticAnchorRemapStateClass` (exact,
  contextual, stale, unmapped, imported static).
- `collection_completeness_class` — owned by this freeze (complete enumeration,
  partial-visible scan, incremental since last, imported snapshot set, filtered
  view, unknown-requires-review).
- `cluster_meaning_class` — owned by this freeze (no clustering, exact duplicate,
  cross-source corroboration, related by location, related by cause, display
  roll-up only).
- `quality_session_outcome_class` — reused `QualitySessionOutcomeClass`
  (applied, preview-required, skipped, timed-out, rebase-required,
  blocked-by-policy, failed, reverted) naming the session that produced or can act
  on the lane.

Plus the per-row invariant booleans: provenance preserved in clustering, imported
never shown as live, freshness/remap disclosed, anchor remap append-only,
collection completeness disclosed, target/environment refs preserved, and every
mutating fix a typed proposal.

## Auto-downgrade

A row is **identity-complete** only when it identifies a source kind, an origin
class, a proven freshness (anything but `unverified`), a remap state, a backing
collection completeness (anything but `unknown_requires_review`), and a governing
quality-session outcome. A row that is not identity-complete must:

- set `effective_qualification` strictly below `claimed_qualification` by rank
  (`held` < `preview` < `beta` < `stable`),
- record a `downgrade_trigger` (unidentified source kind / origin class, unproven
  freshness, unresolved remap state, unknown collection completeness, unlinked
  quality session, or stale evidence window), and
- carry a precise `degraded_label` rather than a generic provider error.

The checked export demonstrates this with the data-tooling row: it claims `beta`
but no governing quality session yet binds its mutating fix routes, so it
auto-downgrades to `held` with an `unlinked_quality_session` trigger and a precise
label.

## Guardrails

`DiagnosticTruthLaneMatrixPacket::validate` rejects a matrix that flattens unlike
sources, lets clustering erase provenance, silently repairs an anchor, renders
imported or replayed evidence as live local truth, hides partial / filtered /
imported collection completeness, drops target / environment / policy refs, or
lets a mutating fix bypass the typed quality-action proposal contract. It also
requires every claimed surface to be represented, at least one consistent
downgraded row to exercise the rule, the canonical schema/doc/artifact source
contracts to be present, and the export to be free of credential or raw-payload
material.

## Consumer projection

The packet records that the editor, Problems, review, CLI/headless, AI evidence,
and support export all ingest this one lane, and that downgraded rows are labeled
below their current claim everywhere. Later support exports, AI evidence, review
surfaces, and release-visible debt packets should consume this lane directly
rather than re-deriving provider-local finding identities.

## Boundary safety

The packet carries only typed class tokens, booleans, opaque ids, and
redaction-aware reviewable labels. Raw source bytes, raw provider payloads, raw
scanner reports, provider cursors, credentials, and raw artifact bodies never
cross this boundary.
