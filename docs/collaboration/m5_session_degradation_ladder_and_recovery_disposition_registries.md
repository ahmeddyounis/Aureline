# M5 session-degradation-ladder and recovery-disposition registries

Implement lane over the frozen [M5 collaboration-state authority matrix][matrix]
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`).
It keeps a *failing* shared session truthful about what degraded and what still remains safely local —
building on the matrix's higher-risk control-plane degradation-banner domain (the separate higher-risk
control plane and the convergence / awareness degradation banner it drives) and its presenter / follow
convergence-state domain — by carrying honest projections of two registries so the claimed M5 shared editor
replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation
banner, support / export packets, and help / docs surfaces inherit one canonical degradation-ladder record
and one recovery-disposition descriptor per degraded session rather than a hand-authored parallel prose that
has to be kept consistent. It closes the gap the source set now expects between collaboration-control truth
(shared terminal / debugger control, presenter handoff, consent / retention envelopes, replay-free restore)
and a truthful, ordered degradation model: when a shared session degrades — participant lag, shared
degraded, relay partition, awareness degraded, or convergence degraded — the degraded state, its sticky
banner, its recent-activity rows, and the recovery paths still open are recorded first, and the session
degrades in the order the source set requires (local editing last) instead of freezing local work or letting
remote authority silently resolve divergence.

## Registry-A — degradation-ladder record

One durable, append-only degradation-ladder record per degraded session, carrying:

- the degraded session state (participant-lag, shared-degraded, relay-partition, awareness-degraded, or
  convergence-degraded), kept mechanically distinct so a distinct degradation never collapses into a
  generic stale or broken badge;
- the sticky banner the degradation raises and holds until the session recovers;
- the recent-activity rows the degradation keeps visible;
- the degradation-order rung the session sits at, so local editing degrades last;
- the actor provenance of the degradation transition;
- the recovery paths still available (continue-local, retry-shared, retry-convergence, review-and-reconcile,
  leave-open);
- the resolution-form coverage (canonical object, accessible summary, audit record).

The degraded state and its still-open recovery paths are recorded before any shared or convergence rung is
touched. A record that would freeze local editing on relay or participant failure, that is a hand-copied
per-entry assumption instead of tracing to the shared registry, that would let remote authority silently
resolve divergence, or that collapses a distinct awareness-degraded / convergence-degraded state into a
generic badge degrades honestly instead of hiding what still remains safely local. The registry reuses the
matrix `m5-collaboration-degradation-banner.schema.json` domain schema.

## Registry-B — recovery-disposition descriptor

The typed recovery-disposition descriptor naming the recovery path a user can take on a degraded session —
continue-local, retry-shared, retry-convergence, review-and-reconcile, or leave-open — plus its actor / time
provenance, its awareness / convergence facts, and its local-edit-first continuity lineage. The descriptor
keeps the recovery dimensions distinct rather than collapsing a lost-awareness, lost-convergence, or
lost-both state into one lane, never resolves divergence by remote authority on the user's behalf, never
acts without actor / time provenance, and never discards unsent local edits to take a recovery path. The
registry reuses the matrix `m5-collaboration-convergence-state.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Relay or participant failure never freezes local editing or silently "fixes" divergence by remote
   authority: a record that would let a relay partition or participant failure freeze local editing, or let
   remote authority resolve divergence without a user-taken recovery path, degrades instead of reading as a
   clean, safe object, so local editing stays first and divergence is never silently resolved.
2. Users can tell whether a session lost awareness, convergence, or both, and what recovery path remains
   available: the degraded state (participant-lag, shared-degraded, relay-partition, awareness-degraded,
   convergence-degraded), the sticky banner, the recent-activity rows, the recovery path
   (continue-local, retry-shared, retry-convergence, review-and-reconcile, leave-open), and its actor / time
   provenance stay visible in the UI projection, the CSV / export, and the support packet instead of
   collapsing into a generic status pill.
3. The degradation order is preserved (local editing first, then shared awareness, then convergence), a
   recovery path never acts without actor / time provenance, and no recovery discards unsent local edits;
   the registries keep each degradation-ladder rung and recovery dimension distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-session-degradation-ladder-and-recovery-disposition-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix/mod.rs
