# M5 diagnostic quality snapshots and imported-versus-live deltas

This document is the contract for the M5 diagnostic-quality snapshot and
imported-versus-live delta lane. Where the source-descriptor / collection-snapshot
lane answers *where a finding set came from and what scope was analyzed*, and the
quality-session ledger owns the single preview/apply/validate/revert contract,
this lane binds those threads into the governance state release-visible debt and
support/export truth depend on: **what quality profile, rule packs, tools,
suppressions, and baselines produced a finding set — and can an imported set and a
live set be compared without one impersonating the other?**

It ships two delivery-grade objects, both reusing — not re-minting — the shared
diagnostic, profile, and collection vocabulary:

1. A **diagnostic-quality snapshot** (`diagnostic_quality_snapshot`) captures the
   active quality-profile ref and fingerprint, the rule-pack/tool versions in
   force, the recent collection ids the findings were drawn from, the
   suppression/baseline refs and release-visible debt count, the imported scanner
   session refs, and the last save-participant outcomes. The snapshot keeps its
   own imported-versus-live origin and freshness.
2. An **imported-versus-live delta packet** (`diagnostic_delta_packet`) compares
   two finding sides — an imported SARIF/scanner/CI snapshot against a live local
   rerun, a runtime finding against a static one, or two snapshots of the same
   class — and states a compatibility verdict with explicit notes.

A snapshot whose governance state is stale, unverified, or left a fix rolled back
auto-downgrades to an effective qualification strictly below its claim with a
recorded trigger and a precise degraded label. A snapshot that cannot bind a
profile, name its tool versions, cite a recent collection, disclose an imported
origin, or join release-visible debt to suppression/baseline truth is rejected as
malformed. Imported, CI, runtime, and local-rerun findings can never impersonate
one another: each delta side keeps a distinct origin and freshness, and a
profile/rule-pack/tool/anchor mismatch blocks an exact-delta claim rather than
silently flattening the two sides. Raw source bytes, raw provider payloads, raw
scanner reports, provider cursors, credentials, and raw artifact bodies never
cross this boundary.

## Source of truth

- Record kind: `m5_diagnostic_quality_parity`
- Packet type: `DiagnosticQualityParityPacket`
  (`crates/aureline-runtime/src/m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas/`)
- Composed boundary schema:
  [`schemas/quality/diagnostic-quality-parity.schema.json`](../../schemas/quality/diagnostic-quality-parity.schema.json)
- Quality-snapshot component schema:
  [`schemas/quality/diagnostic-quality-snapshot.schema.json`](../../schemas/quality/diagnostic-quality-snapshot.schema.json)
- Delta-packet component schema:
  [`schemas/quality/diagnostic-delta-packet.schema.json`](../../schemas/quality/diagnostic-delta-packet.schema.json)
- Checked support export:
  [`artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json`](../../artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json)
- Protected fixtures:
  [`fixtures/quality/m5/imported-vs-live-deltas/`](../../fixtures/quality/m5/imported-vs-live-deltas/)
- Canonical loader:
  `aureline_runtime::m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas::current_m5_diagnostic_quality_parity_export`

## Diagnostic-quality snapshot

A `diagnostic_quality_snapshot` records, for one scope at one point in time:

- `active_profile_ref` / `profile_fingerprint` — the resolved quality profile.
- `tool_versions` — the analyzers and rule packs in force, each with a
  `tool_ref` / `tool_version` and a `rule_pack_ref` / `rule_pack_version`.
- `recent_collection_refs` — the collection-snapshot ids the findings were drawn
  from (see the source-descriptor / collection-snapshot lane).
- `suppression_refs` / `baseline_refs` / `release_visible_debt_count` — the policy
  state behind the snapshot; a non-zero debt count must join at least one
  suppression or baseline record.
- `imported_scanner_session_refs` — the imported sessions backing an imported
  origin; an `imported_snapshot` origin must cite at least one.
- `save_participant_outcomes` — the last outcome per save participant.
- `origin_class` / `freshness_class` / `imported_not_shown_as_live` — the
  imported-versus-live posture; imported and replayed evidence is held read-only.

### Auto-downgrade

Each snapshot is wrapped in an entry with a `claimed_qualification` and an
`effective_qualification`. When the governance state is stale or superseded
(`stale_governance_state`), unverified or degraded-cached
(`unverified_governance_state`), or left a fix rolled back
(`unresolved_save_participant`), the effective qualification drops strictly below
the claim, a `downgrade_trigger` is recorded, and a precise `degraded_label`
replaces any generic "unavailable / failed / partial" placeholder.

## Imported-versus-live delta packet

A `diagnostic_delta_packet` compares a `base_side` and a `compare_side`. Each side
names its `origin_class`, `freshness_class`, `source_kind`, `snapshot_ref`,
`collection_ref`, `active_profile_ref`, and `tool_version_refs`. The packet states:

- `comparison_basis_class` — `imported_vs_live_rerun`, `ci_vs_local_rerun`,
  `runtime_vs_static_analysis`, `imported_snapshot_vs_imported_snapshot`, or
  `live_snapshot_vs_live_snapshot`.
- `compatibility_class` — `compatible_exact`,
  `compatible_with_local_confirmation`, `blocked_profile_or_tool_mismatch`,
  `blocked_rule_pack_mismatch`, `blocked_anchor_mapping_uncertain`,
  `not_comparable_distinct_source`, or `not_comparable_unknown_requires_review`.
  Anything short of an exact match must carry at least one `compatibility_note`.
- `delta_counts` and `finding_deltas` — per-finding `added` / `resolved` /
  `persisting` / `suppressed` / `waived` / `unmapped` states; the counts must
  match the per-finding tally.
- `impersonation_guarded` — the two sides must be distinct, and a comparison that
  crosses the imported/live boundary (`imported_vs_live_rerun` /
  `ci_vs_local_rerun`) must carry distinct origins so neither side can pose as the
  other.

## Release-visible debt

`release_debt_projection` assembles the release-visible debt count from the
snapshots and asserts `owner_truth_preserved`, `expiry_truth_preserved`,
`baseline_join_preserved`, and `suppression_join_preserved`, with
`debt_source_refs` naming the suppression / baseline records behind the count.
Release-visible debt packets ingest this projection directly instead of a
manually assembled summary.

## Parity guarantees

`consumer_projection` asserts that Problems, review, CLI/headless, support export,
AI evidence, and release-visible debt all reference this one packet. `guardrails`
asserts that unlike sources are never flattened, anchors are never silently
repaired, imported-versus-live class and freshness stay explicit, policy state
survives clustering, every mutating fix route is a typed quality-action proposal,
and diagnostic ids and collection completeness stay exportable.

`DiagnosticQualityParityPacket::validate` enforces every guarantee above and
returns a list of `DiagnosticQualityParityViolation` tokens on failure.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_diagnostic_quality_parity > \
  artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_diagnostic_quality_parity summary > \
  artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.md
```
