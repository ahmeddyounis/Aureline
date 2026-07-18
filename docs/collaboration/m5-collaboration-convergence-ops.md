# M5 collaboration-replica, shared-object-authority, anchor-drift, convergence-state, and session-archive ops

This is the operator- and consumer-facing contract for the frozen M5 collaboration-state matrix. It sits
beneath the already-frozen collaboration-control surfaces and freezes the underlying shared-object authority,
convergence, drift, downgrade, and export model. The Rust validator in
`crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`
is the authoritative gate; this document explains the vocabulary and how consumers read it.

- Combined matrix schema: `schemas/collaboration/m5-collaboration-state-authority-matrix.schema.json`
- Support export (canonical mint): `artifacts/release/m5-collaboration-convergence-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-collaboration-convergence-proof/matrix.csv`
- Design report: `artifacts/design/m5-collaboration-state-authority-matrix.md`
- Health dashboard: `dashboards/m5-collaboration-convergence-health.json`
- Narrowed fixtures: `fixtures/collaboration/m5-convergence/`

## Governed object classes

Every claimed M5 collaboration surface points to exactly one governed object-class row rather than inferring
behavior from a generic session-state pill. No surface may claim collaboration parity for an object class not
present in the matrix.

| Object class | Authority model | Canonical domain schema |
| --- | --- | --- |
| `crdt_backed_shared_text` | CRDT-convergent replica; local buffer / VFS / Git truth stays canonical | `schemas/collaboration/m5-collaboration-replica-state.schema.json` |
| `sampled_presence_cursors_selections` | Sampled, non-authoritative presence | `schemas/collaboration/m5-shared-object-descriptor.schema.json` |
| `server_ordered_comments_annotations_review_pins` | Server-ordered sequence with append-only anchor drift | `schemas/collaboration/m5-collaboration-anchor-history.schema.json` |
| `presenter_follow_state` | Host-authoritative; follow is view-only | `schemas/collaboration/m5-collaboration-convergence-state.schema.json` |
| `higher_risk_control_plane` | Separate control plane driving the degradation banner | `schemas/ui/m5-collaboration-degradation-banner.schema.json` |
| `sealed_session_archive` | Sealed archive with bounded compaction lineage | `schemas/collaboration/m5-session-compaction-manifest.schema.json` |

## Shared role taxonomy

Each object class binds to the same seven-role vocabulary. The first four are hard posture requirements that
must be present before a class may surface as a collaboration-state result; the last three are contextual.

- `authority_model_disclosure` (gate) — declares whether the object converges, is server-ordered,
  host-authoritative, or defers to local canonical truth.
- `local_truth_preservation_disclosure` (gate) — a replica never replaces the canonical local buffer, VFS, or
  Git truth.
- `merge_and_drift_semantics_disclosure` (gate) — how concurrent edits merge and how anchors drift.
- `downgrade_behavior_disclosure` (gate) — a permission or relay downgrade preserves local unsent work first.
- `anchor_drift_history_disclosure` — anchor drift stays append-only and reviewable.
- `export_posture_disclosure` — op-logs, snapshots, and archives carry policy-labeled redaction and actor
  lineage.
- `provenance_and_freshness_disclosure` — search, AI, review, docs, and support never consume stale
  collaboration state as current.

## Convergence-state vocabulary

The `convergence_state` field makes a converged object mechanically distinct from `converging_pending_ops`,
`server_ordered`, `host_authoritative`, `locally_pending_unsent`, `convergence_degraded`, `awareness_degraded`,
`anchor_unresolved`, `anchor_rebound_append_only`, `relay_partitioned`, `reconciliation_required`,
`compaction_pending`, `sealed_archived`, `local_canonical_preserved`, `sampled_presence_only`, and
`provenance_stale`. Consumers key off this state rather than a generic stale-or-broken pill.

## Hard invariants

Every row asserts, and the validator enforces, that the class never:

1. lets a replica overwrite the canonical local buffer, VFS, or Git truth implicitly;
2. discards unsent local edits on a permission downgrade, relay failure, or leave-session flow;
3. rebinds comments, annotations, or review pins without append-only drift history;
4. collapses a convergence-degraded or awareness-degraded state into a generic stale or broken badge;
5. exports op-logs, snapshots, or archives without policy-labeled redaction and actor lineage.

## Downgrade and degradation

Permission and relay downgrades resolve through the downgrade gate
(`converged_local_work_preserved`, `blocked_by_unsent_local_work_at_risk`, `blocked_by_permission_downgrade`,
`blocked_by_relay_partition`, `blocked_by_unreviewed_anchor_drift`). The higher-risk control plane drives the
collaboration degradation banner, which always names the exact degraded state instead of collapsing it.

## Consumers

The shared editor replica view, presence / cursor layer, comment / annotation / review-pin layer, presenter /
follow banner, collaboration degradation banner, session archive and compaction view, search / AI provenance
consumer, support / export packet, and help / docs surfaces all read this one matrix. Later rows may not invent a
parallel collaboration-state vocabulary; they narrow automatically when the matrix row is missing or stale.

## Binds back to

The matrix binds back to the already-frozen collaboration-control component matrix
(`schemas/collaboration/m5-collaboration-control-component-matrix.schema.json`), the stable-proof-index, and the
migration-task-row so collaboration-state and collaboration-control truth share one contract.
