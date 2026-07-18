# M5 comment-annotation-review-pin anchor-history and rebind-review registries

Implement lane over the frozen [M5 collaboration-state authority matrix][matrix]
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`).
It makes collaboration *anchors* as honest and reviewable as other Aureline evidence objects — building on
the matrix's server-ordered comments / annotations / review-pins anchor-history domain (the append-only
drift ledger a pin never silently overwrites) and its presenter / follow convergence-state domain — by
carrying honest projections of two registries so the claimed M5 shared editor replica view, presence /
cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, support / export
packets, and help / docs surfaces inherit one canonical anchor-history record and one rebind-review
disposition per drift rather than a hand-authored parallel prose that has to be kept consistent. It closes
the gap the source set now expects between collaboration-control truth (shared terminal / debugger control,
presenter handoff, consent / retention envelopes, replay-free restore) and reviewable anchor drift: when a
comment, annotation, or review pin has to move — because the buffer was edited, a generated output
refreshed, or an imported snapshot transitioned — its textual anchor, semantic anchor, and revision-pair
lineage are recorded first, and it lands in an explicit drift, unresolved, or rebound state instead of
silently jumping to a new location.

## Registry-A — anchor-history record

One durable, append-only anchor-history record per drift, carrying:

- the comment / annotation / review-pin identity, kept mechanically distinct so a drifted anchor never
  silently supersedes its recorded history;
- the textual anchor and the semantic anchor the pin is bound to;
- the revision-pair lineage across which the anchor moved (a buffer edit, a generated-output refresh, or an
  imported-snapshot transition);
- the drift / unresolved / rebound state the anchor lands in;
- the export posture the anchor history is bound to;
- the resolution-form coverage (canonical object, accessible summary, audit record).

The append-only drift history is recorded before the pin moves. A record that cannot bind its anchor
identity, that is a hand-copied per-entry assumption instead of tracing to the shared registry, that would
let a drift read as clean while a comment or pin jumped without recorded drift history, or that publishes an
incomplete object degrades honestly instead of silently rebinding the anchor. The registry reuses the
matrix `m5-collaboration-anchor-history.schema.json` domain schema.

## Registry-B — rebind-review-disposition record

The typed rebind-review-disposition record naming the manual or assisted rebind decision a user can take on
a drifted anchor — keep-textual, keep-semantic, manual-rebind, assisted-rebind, or leave-unresolved — plus
its actor / time provenance and its policy-labeled export lineage. The record keeps the rebind dimensions
distinct rather than collapsing a distinct anchor-unresolved state into one generic stale or broken badge,
never applies a rebind without review, never acts without actor / time provenance, and never exports an
op-log, snapshot, or archive without policy-labeled redaction and actor lineage. The registry reuses the
matrix `m5-collaboration-convergence-state.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Collaboration comments or review pins never silently jump to a new location without drift history and
   rebind state: a record that would let a drift — from a buffer edit, generated-output refresh, or
   imported-snapshot transition — read as clean while a pin jumped without recorded drift history degrades
   instead of reading as a clean, safe object, so no drift silently rebinds an anchor over the canonical
   append-only history.
2. Export / support flows can show current anchor state plus history of drift and rebound decisions: the
   drift / unresolved / rebound state, the rebind decision (keep-textual, keep-semantic, manual-rebind,
   assisted-rebind, leave-unresolved), and its actor / time provenance stay visible in the UI projection,
   the CSV / export, and the support packet instead of collapsing into a generic status pill.
3. Append-only drift history is preserved first, a rebind never applies without review, a disposition never
   acts without actor / time provenance, and exported op-logs / snapshots / archives never export without
   policy-labeled redaction and actor lineage; the registries keep each anchor-history and rebind dimension
   distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-comment-annotation-review-pin-anchor-history-and-rebind-review-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix/mod.rs
