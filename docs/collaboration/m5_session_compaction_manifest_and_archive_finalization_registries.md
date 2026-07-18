# M5 session-compaction-manifest and archive-finalization registries

Implement lane over the frozen [M5 collaboration-state authority matrix][matrix]
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`).
It bounds collaboration *state growth* without making retained evidence or exported session history
unexplainable — building on the matrix's CRDT-backed shared-text replica-state domain (the CRDT replica of
shared session text and the local buffer it never replaces) and its sealed session-archive
compaction-manifest domain — by carrying honest projections of two registries so the claimed M5 shared
editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration
degradation banner, support / export packets, and help / docs surfaces inherit one canonical
session-compaction-manifest record and one archive-finalization descriptor per compaction rather than a
hand-authored parallel prose that has to be kept consistent. It closes the gap the source set now expects
between collaboration-control truth (shared terminal / debugger control, presenter handoff, consent /
retention envelopes, replay-free restore) and inspectable, bounded state growth: when the CRDT-backed
session text has to be compacted — to bound op-log and tombstone growth after a buffer edit, a generated
output refresh, or an imported snapshot transition — its snapshot ID, retained-object references, and
tombstone / op-log bounds are recorded first, and it lands in an explicit compacted, retained, or
tombstoned state instead of reclaiming storage as hidden churn.

## Registry-A — session-compaction-manifest record

One durable, append-only session-compaction-manifest record per compaction, carrying:

- the snapshot ID, kept mechanically distinct so a compaction never silently supersedes its recorded
  lineage;
- the retained-object references the compaction keeps;
- the tombstone / op-log bounds the compaction draws for the CRDT-backed session text;
- the actor provenance of the compaction;
- the compacted / retained / tombstoned state the session lands in;
- the export posture the compaction lineage is bound to;
- the resolution-form coverage (canonical object, accessible summary, audit record).

The compaction lineage is recorded before storage is reclaimed. A record that cannot bind its snapshot
identity, that is a hand-copied per-entry assumption instead of tracing to the shared registry, that would
let a compaction read as clean while retained state was reclaimed as hidden storage churn, or that
publishes an incomplete object degrades honestly instead of silently discarding retained session state.
The registry reuses the matrix `m5-collaboration-replica-state.schema.json` domain schema.

## Registry-B — archive-finalization-descriptor record

The typed archive-finalization-descriptor record naming the finalization decision a user can take on a
compacted session — finalize-archive, export-snapshot, export-op-log, redact-and-finalize, or leave-open —
plus its actor / time provenance, its compaction / redaction / omission facts, and its policy-labeled
export lineage. The record keeps the finalization dimensions distinct rather than collapsing a distinct
archive-finalized state into one generic stale or broken badge, never finalizes an archive without recorded
compaction lineage, never acts without actor / time provenance, and never exports an op-log, snapshot, or
archive without policy-labeled redaction and actor lineage. The registry reuses the matrix
`m5-session-compaction-manifest.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Collaboration compaction is inspectable through snapshot / manifest lineage rather than hidden storage
   churn: a record that would let a compaction — after a buffer edit, generated-output refresh, or
   imported-snapshot transition — read as clean while retained state was reclaimed without recorded
   snapshot / manifest lineage degrades instead of reading as a clean, safe object, so no compaction
   silently reclaims state over the canonical append-only lineage.
2. Session archives and optional op-log exports preserve actor provenance and compaction / redaction facts
   that support or audit flows can explain: the compacted / retained / tombstoned state, the finalization
   decision (finalize-archive, export-snapshot, export-op-log, redact-and-finalize, leave-open), and its
   actor / time provenance stay visible in the UI projection, the CSV / export, and the support packet
   instead of collapsing into a generic status pill.
3. Compaction lineage is preserved first, an archive never finalizes without recorded compaction lineage, a
   descriptor never acts without actor / time provenance, and exported op-logs / snapshots / archives never
   export without policy-labeled redaction and actor lineage; the registries keep each compaction-manifest
   and finalization dimension distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-session-compaction-manifest-and-archive-finalization-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix/mod.rs
