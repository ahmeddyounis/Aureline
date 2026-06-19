# M5 diagnostic clusters and cross-source dedupe

This document is the contract for the M5 diagnostic display-clustering lane. Where
the diagnostic-truth freeze binds each surface to a canonical
record/source/remap/session vocabulary, the normalized-record set proves
per-finding identity, and the source/collection lane answers where a finding set
came from, this lane answers a different ergonomics question: **how does Aureline
group several findings into one compact row without losing the distinct provenance,
scope, and environment facts a user needs to debug and trust them?**

The answer is a *display cluster* that is always a view over real records, never a
new synthetic finding minted by flattening members. Different sources reporting a
similar finding can be shown as one ergonomic summary, but every constituent keeps
its own canonical id, source descriptor, target/environment ref, policy state, and
imported-versus-live class — and stays recoverable from a detail sheet.

## Source of truth

- Set record kind: `m5_diagnostic_cluster_set`
- Packet type: `DiagnosticClusterSetPacket`
  (`crates/aureline-runtime/src/cluster_m5_diagnostics_with_cross_source_dedupe_and_source_preserving_detail_sheets/`)
- Boundary schema:
  [`schemas/quality/diagnostic-cluster.schema.json`](../../schemas/quality/diagnostic-cluster.schema.json)
- Checked support export:
  [`artifacts/m5/diagnostics/cluster-proof/support_export.json`](../../artifacts/m5/diagnostics/cluster-proof/support_export.json)
- Summary artifact:
  [`artifacts/m5/diagnostics/cluster-proof/support_export.md`](../../artifacts/m5/diagnostics/cluster-proof/support_export.md)
- Fixtures:
  [`fixtures/quality/m5/cluster-and-dedupe/`](../../fixtures/quality/m5/cluster-and-dedupe/)
- Loader:
  `aureline_runtime::cluster_m5_diagnostics_with_cross_source_dedupe_and_source_preserving_detail_sheets::current_m5_diagnostic_cluster_set_export`
- Conformance dump:
  `cargo run -p aureline-runtime --example dump_m5_diagnostic_clusters`

This lane reuses — it does not re-mint — the shared diagnostic vocabulary. Each
constituent is one canonical `DiagnosticRecord`; the dedupe reason is the closed
`DiagnosticClusterMeaningClass` owned by the diagnostic-truth freeze.

## A display cluster carries the compact row's facts

A `DiagnosticDisplayCluster` carries everything a compact row needs:

- a stable `cluster_id`;
- a `primary_diagnostic_id` and `primary_anchor_ref` for the headline row;
- the `contributing_diagnostic_ids`;
- a typed `dedupe_reason_class` and an export-safe `dedupe_reason_detail`
  explaining why the members were shown as one row;
- `aggregate_counts` (member count, distinct source kinds, distinct origins,
  per-severity counts, imported/live counts, suppressed/baselined counts); and
- a `dominant_display_state` (the most severe member's severity, the most
  cautionary freshness and remap state, and whether any member is imported,
  live, or requires disclosure).

The dominant freshness and remap state are the **most cautionary** members, so a
compact row never reads as fresher or better-anchored than its least-trustworthy
constituent.

### Dedupe reasons

The dedupe reason is a closed vocabulary; convenience clustering can never imply a
stronger relationship than the evidence proves:

| Reason | Meaning |
| --- | --- |
| `no_clustering` | One record, no clustering applied. |
| `exact_duplicate` | The same finding observed by the same source more than once. |
| `cross_source_corroboration` | The same underlying issue corroborated by multiple distinct sources. |
| `related_by_location` | Findings grouped because they share a location or range. |
| `related_by_cause` | Findings grouped because they share one causal origin. |
| `display_rollup_only` | A display-only roll-up that must preserve each member's provenance. |

## Detail sheets preserve every member's truth

Even when the default display row is clustered, each constituent has a
`DiagnosticClusterMemberDetailSheet` that preserves:

- the constituent's canonical `member_diagnostic_id` and a `reopen_surface_ref`,
  so the member is recoverable;
- its `source_kind`, `evidence_plane_class`, `origin_class`, derived
  `imported_live_class`, `confidence_class`, and `support_class`;
- its `freshness_class`, `remap_state_class`, and anchor family;
- its `source_descriptor_ref`, `producer_ref`, `tool_ref`, `tool_version_ref`,
  `adapter_ref`, and `target_or_environment_ref`; and
- its policy state — `support_class`, `suppression_refs`, `baseline_refs`, and
  `redaction_class`.

The imported-versus-live class is always explicit and is derived from the origin
class, so an imported scanner snapshot in a cross-source cluster can never read as
live local truth.

## Exposing clusters across surfaces

Problems, review, support export, and AI evidence each receive a
`DiagnosticClusterSurfaceProjection` that exposes the dedupe reason and the full
member list, so a user can always audit why several findings were shown as one
summary. Every projection keeps constituents recoverable and carries no raw
content.

The `DiagnosticClusterSupportExport` preserves both the cluster meaning (per-cluster
dedupe reason) and the constituent findings (per-cluster membership plus the
flattened distinct id set) rather than serializing a lossy display-only row. Raw
source content and raw payloads are omitted by default.

## What the validator refuses

`DiagnosticClusterSetPacket::validate` returns a typed violation list. It refuses a
packet that:

- flattens unlike sources into a synthetic finding
  (`synthetic_finding_flattening`) or marks a cluster synthetic;
- drops a member's source, origin, freshness, or remap label
  (`cluster_dropped_provenance`);
- cannot recover a constituent from its detail sheet (`member_not_recoverable`);
- names a primary diagnostic that is not a member (`primary_not_a_member`);
- lets a cluster's aggregate counts (`aggregate_counts_inconsistent`) or dominant
  display state (`dominant_state_inconsistent`) disagree with its members;
- proves no cross-source clustering at all (`cross_source_cluster_missing`);
- hides the dedupe reason or membership from a required surface
  (`surface_projection_missing`, `surface_projection_drops_dedupe_or_membership`);
- serializes a lossy or raw support export (`support_export_lossy`,
  `support_export_includes_raw_content`); or
- carries forbidden boundary material (`raw_boundary_material_in_export`).

## Guardrails

- Clustering is display-only: it never flattens unlike sources into one synthetic
  finding, silently repairs anchors, or erases imported/live class, target or
  environment refs, or policy state.
- Diagnostic ids and collection completeness stay exportable and support-safe.
- Raw source bytes, raw provider payloads, raw scanner reports, credentials, and
  raw artifact bodies never cross this boundary.
