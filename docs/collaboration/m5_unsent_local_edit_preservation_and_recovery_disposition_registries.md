# M5 unsent-local-edit-preservation and recovery-disposition registries

Implement lane over the frozen [M5 collaboration-state authority matrix][matrix]
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`).
It makes collaboration *downgrade* honest — building on the matrix's CRDT-backed shared-text replica
state (the local buffer a replica never replaces) and its higher-risk control-plane degradation banner —
by carrying honest projections of two registries so the claimed M5 shared editor replica view, presence /
cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, support / export
packets, and help / docs surfaces inherit one canonical preservation record and one recovery disposition
per downgrade rather than a hand-authored parallel prose that has to be kept consistent. It closes the gap
the source set now expects between collaboration-control truth (shared terminal / debugger control,
presenter handoff, consent / retention envelopes, replay-free restore) and downgrade-safe local unsent
preservation: when a shared session narrows — role loss, permission narrowing, explicit leave, host
removal, or relay failure — every unsent local shared-text edit is preserved first, as local-only state,
reconnect-ready state, or a reviewable patch packet, instead of being silently dropped or remotely
resolved.

## Registry-A — unsent-local-edit-preservation record

One durable, canonical preservation record per downgrade, carrying:

- the workspace-root and buffer / object identity of the unsent local shared text, kept mechanically
  distinct so preserved local work never reads as, or is superseded by, the shared replica;
- the downgrade trigger (role loss, permission narrowing, explicit leave, host removal, or relay failure);
- the preserved-state class the unsent work lands in — local-only, reconnect-ready, or reviewable patch
  packet;
- the export posture the preserved work is bound to;
- the resolution-form coverage (canonical object, accessible summary, audit record).

The preserved work is materialized before the session narrows. A record that cannot bind its
preserved-work identity, that is a hand-copied per-entry assumption instead of tracing to the shared
registry, that would let a downgrade read as clean while unsent local edits are dropped, or that publishes
an incomplete object degrades honestly instead of discarding local canonical truth. The registry reuses
the matrix `m5-collaboration-replica-state.schema.json` domain schema.

## Registry-B — recovery-disposition record

The typed recovery-disposition record naming the next action a user can take on the preserved work —
continue-local, reopen-share, export-patch, or discard-with-review — plus its actor / time provenance and
its policy-labeled export lineage. The record keeps the disposition dimensions distinct rather than
collapsing a distinct downgrade into one generic stale or broken badge, never applies a discard without
review, never acts without actor / time provenance, and never exports a patch, op-log, snapshot, or
archive without policy-labeled redaction and actor lineage. The registry reuses the matrix
`m5-collaboration-degradation-banner.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Unsent local collaboration edits are never silently dropped on downgrade or disconnect: a record that
   would let a downgrade — role loss, permission narrowing, explicit leave, host removal, or relay
   failure — read as clean while unsent local edits are discarded degrades instead of reading as a clean,
   safe object, so no downgrade silently supersedes the canonical local buffer, VFS, or Git truth.
2. Users can inspect and act on preserved local-only state or reviewable patch packets before rejoining,
   exporting, or discarding: the preserved-state class, the recovery action (continue-local, reopen-share,
   export-patch, discard-with-review), and its actor / time provenance stay visible in the UI projection,
   the CSV / export, and the support packet instead of collapsing into a generic status pill.
3. Downgrade preserves local unsent work first, a discard never applies without review, a disposition
   never acts without actor / time provenance, and exported patches / op-logs / snapshots / archives never
   export without policy-labeled redaction and actor lineage; the registries keep each preservation and
   recovery dimension distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-unsent-local-edit-preservation-and-recovery-disposition-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix/mod.rs
