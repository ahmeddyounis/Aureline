# M5 retention/export-card & history-export-manifest primitive (M05-896)

Two reusable M5 compare-and-export primitives — the **retention/export card** and the
**history-export manifest** — so a local-history, refactor, import, AI-apply, recovery, or
support flow keeps its recovery baselines and outbound artifacts explicit: what survives, what
expires, what is metadata-only, which baseline a diff is measured against, and how redaction
shapes any patch or evidence export — never a bare "download" that hides the baseline, the
scope, or the redaction posture behind it.

This closes the B105 compare-and-export lane over the frozen local-history / write-scope
component matrix
(`schemas/ui/m5-local-history-write-scope-component-matrix.schema.json`), implementing the
matrix's `retention_export_card` and `history_export_manifest` families as governed resolvers.

- Module:
  `crates/aureline-history/src/ship_cross_baseline_compare_and_export_flows_so_current_versus_snapshot_snapshot_versus_disk_snapshot_versus_git_and_patch_or_evidence_export_stay_explicit_across_claimed_m5_history_refactor_import_ai_paths`
- Emitter bin: `aureline_history_retention_export_card_history_export_manifest_primitive`
- Schemas: `schemas/ui/m5-retention-export-card.schema.json` (packet),
  `schemas/ui/m5-history-export-manifest.schema.json` (manifest shape)
- Support export: `artifacts/release/m5-retention-export-card-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-retention-export-card-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-retention-export-card-primitive.md`
- Fixtures: `fixtures/ui/m5-retention-export-card-primitive/`

## Resolvers

### `resolve_retention_export_card`

Takes one card's retention posture, export-redaction posture, the cross-baseline comparisons the
underlying surface / artifact class supports, whether comparison is available, whether the card
is metadata-only, whether the export path is ready, and its opaque card label. The derived
**card posture** is computed in a fixed blocking-first order:

1. `export_blocked` — the export path is unavailable, or the redaction posture blocks export.
2. `nothing_retained` — retention has expired and purged; nothing survives to export.
3. `policy_restricted` — export is gated by policy.
4. `purge_scheduled` — a purge is pending; export before it purges.
5. `metadata_only_survives` — only metadata survives (bodies omitted or session-only).
6. `fully_shareable` — full metadata is retained and shareable.

An export can commit unless the card is `export_blocked` or `nothing_retained`. The available
cross-baseline comparisons are carried only when comparison is available, so a card never claims
a compare path it does not have. Inspect-retention and review-redaction are always offered.

### `resolve_history_export_manifest`

Takes one manifest's class, export-redaction posture, primary compare baseline, whether it
preserves actor lineage / checkpoint identity / scope, whether it would carry raw content
bodies, whether the export path is ready, and its opaque manifest label. The derived **manifest
disposition** is computed in a fixed blocking-first order:

1. `export_blocked` — the export path is unavailable, or the redaction posture blocks export.
2. `raw_body_withheld` — the manifest would carry raw content bodies and is held back.
3. `lineage_incomplete` — actor lineage, checkpoint identity, or scope is not fully preserved.
4. `policy_restricted` — export is gated by policy.
5. `redacted_share` — a properly redaction-shaped share (paths redacted, bodies omitted,
   credentials scrubbed, or an explicit redacted-share class).
6. `full_evidence` — a full-evidence bundle with lineage, identity, and scope preserved.

A manifest is shareable only when it is `full_evidence` or `redacted_share`, and it always keeps
its primary baseline explicit. No shareable manifest ever defaults to a raw sensitive content
body — a manifest that would carry one is held back, not shared. Inspect-manifest, view-lineage,
and review-redaction are always offered.

## Cross-baseline comparisons

`M5CompareBaseline` names the baselines a diff can be measured against so a comparison is never
ambiguous:

- `current_vs_snapshot` — the current working buffer versus a saved snapshot.
- `snapshot_vs_disk` — a snapshot versus the current on-disk file (external drift).
- `snapshot_vs_git_head` — a snapshot versus Git HEAD.
- `snapshot_vs_snapshot` — a snapshot versus another snapshot on the timeline.

The three acceptance-named baselines (`current_vs_snapshot`, `snapshot_vs_disk`,
`snapshot_vs_git_head`) are proven explicit across the worked card and manifest examples.

## Consumers

The matrix binds one row per claimed compare / export consumer — `local_history_timeline`,
`refactor_evidence`, `import_migration_session`, `ai_apply_evidence`, `recovery_center`, and
`support_export_desk` — each carrying the shared card / manifest anatomy, the same retention and
redaction vocabulary, worked resolver examples, and four hard invariants: it never hides the
export baseline, never hides retention or redaction posture, never defaults to raw content
bodies, and never collapses an export into a generic download.

## Acceptance coverage

- **Baselines explicit** — every acceptance-named baseline appears across the worked card
  comparisons and the worked manifest baselines.
- **Retention / export explicit** — worked cards prove retained, metadata-only, purge-scheduled,
  policy-restricted, nothing-retained, and export-blocked postures; worked manifests prove
  full-evidence, redacted-share, policy-restricted, lineage-incomplete, raw-body-withheld, and
  export-blocked dispositions.
- **No hidden baseline / scope / redaction** — every card discloses its retention and redaction
  posture, every manifest keeps its baseline explicit and omits raw bodies, and no export is
  collapsed into a generic download.

## Verify

```sh
cargo test -p aureline-history --lib ship_cross_baseline_compare_and_export_flows
cargo run -q -p aureline-history --bin aureline_history_retention_export_card_history_export_manifest_primitive -- validate
```
