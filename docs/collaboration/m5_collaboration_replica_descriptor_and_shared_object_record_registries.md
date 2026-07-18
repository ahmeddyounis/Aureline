# M5 collaboration-replica-descriptor and shared-object-record registries

First implement lane over the frozen [M5 collaboration-state authority matrix][matrix]
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`).
It makes the matrix's collaboration replica descriptor and its per-class shared-object record operable —
as durable, resolved records — by carrying honest projections of two registries so the claimed M5 shared
editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration
degradation banner, support / export packets, and help / docs surfaces inherit one canonical replica
descriptor and one declared authority model per shared object rather than a hand-authored parallel prose
that has to be kept consistent. It closes the gap between the already-landed collaboration-control truth
(shared terminal / debugger control, presenter handoff, consent / retention envelopes, replay-free
restore) and the broader shared-object authority and convergence contract the source set now expects:
entering share mode materializes typed replica and shared-object records instead of ambient per-surface
state, collaboration replicas never replace canonical local buffer, VFS, or Git truth, and every shared
object declares which authority model it holds.

## Registry-A — collaboration replica descriptor

One durable, canonical collaboration replica descriptor per shared object entering share mode, carrying:

- the workspace-root identity the shared object belongs to;
- the buffer / object identity, kept mechanically distinct so a shared replica never reads as the
  canonical local buffer itself;
- the revision and session epoch, and the trust and policy epoch, the replica is pinned to;
- the export posture the replica is bound to;
- the resolution-form coverage (canonical object, accessible summary, audit record).

Entering share mode creates the replica record rather than replacing local buffer authority outright. A
descriptor that cannot bind its identity to its buffer / object identity, that is a hand-copied per-entry
assumption instead of tracing to the shared registry, that would discard unsent local edits on a
permission or relay downgrade, or that publishes an incomplete object degrades honestly instead of
letting a replica overwrite local canonical truth. The registry reuses the matrix
`m5-collaboration-replica-state.schema.json` domain schema.

## Registry-B — per-class shared-object record

The typed shared-object record naming which authority model each object holds — CRDT-convergent shared
editable text, sampled presence / cursor / selection samples, server-ordered comments / annotations /
review pins, presenter / follow state, linked higher-risk control objects, or immutable sealed-archive
evidence — plus its convergence and merge-drift posture, its append-only and reviewable anchor-drift
history, and its policy-labeled export lineage. The record keeps the authority-model dimensions distinct
rather than flattening a degraded state into one generic stale or broken badge, never silently rebinds a
comment or pin without drift history, and never exports an op-log, snapshot, or archive without
policy-labeled redaction and actor lineage. The registry reuses the matrix
`m5-shared-object-descriptor.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Entering share mode creates explicit collaboration replica records rather than replacing local buffer
   authority outright: a descriptor that cannot bind its buffer / object identity, or that would let a
   replica overwrite local canonical truth or discard unsent local edits on downgrade, degrades instead
   of reading as a clean, share-ready object, so no shared object silently supersedes the canonical local
   buffer, VFS, or Git truth.
2. Inspect surfaces can tell which object class is CRDT-convergent, server-ordered, host-authoritative,
   or immutable evidence: the declared authority model, the convergence state, the anchor-drift history,
   and the export posture stay visible in the UI projection, the CSV / export, and the support packet
   instead of collapsing into a generic status pill.
3. Collaboration replicas never replace canonical local buffer / VFS / Git truth, permission or relay
   downgrade preserves local unsent work first, anchor drift stays append-only and reviewable,
   convergence- or awareness-degraded state is never collapsed into a generic stale badge, and op-logs /
   snapshots / archives never export without policy-labeled redaction and actor lineage; the registries
   keep each authority and convergence dimension distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-collaboration-replica-descriptor-and-shared-object-record-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix/mod.rs
