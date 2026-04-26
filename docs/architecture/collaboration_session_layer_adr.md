# Collaboration Session Layer ADR Seed

- **Decision id:** pending formal register row
- **Status:** Accepted
- **Decision date:** 2026-04-26
- **Owner:** `@ahmeddyounis`
- **Forum:** architecture_council
- **Related requirement ids:** `none`

## Context

Aureline's product sources require CRDTs to be scoped to live
collaboration sessions instead of replacing the local text model. The
PRD calls the piece-tree local buffer the ordinary editing authority
and says CRDTs are the collaboration/session protocol, not the sole
in-memory representation. It also says live collaboration must use
CRDTs only with aggressive checkpointing and compaction, while canonical
files remain in the VCS workspace instead of a permanent CRDT log.

The same sources require presence to be transport-separated from text
convergence and from shell/debug authority. Presence is low-latency,
disposable, and session-scoped; shared terminal/debug control is
opt-in and visibly active; disconnects must never corrupt local buffers
or autosave journals; and retained comments, recordings, transcripts,
or replay artifacts require explicit policy and retention posture.

## Decision

Aureline will use a CRDT-style **collaboration session layer** as a
projection above local workspace authority, with a separate
**presence plane** for online state, cursors, selections, follow, and
presenter awareness. The session layer is allowed to carry bounded
operation logs, snapshots, participant proposals, and shared-object
metadata for an active session. It is not the canonical project
database. The local workspace, VFS save truth, mutation journal, local
history, and canonical notebook/file formats remain authoritative for
project state. Captured collaboration exports are immutable evidence
objects with attribution, retention, redaction, and delete/export
policy; they never become writable project truth.

## Boundary Model

| Boundary | Examples | Authority | Persistence | Export posture |
|---|---|---|---|---|
| `canonical_project_state` | worktree files, notebooks, local history, mutation journal, save checkpoints | local workspace / VFS / notebook authority | durable local or declared remote workspace | exported through workspace, VCS, notebook, or local-history paths |
| `shared_session_artifact` | CRDT operation window, session snapshot, participant proposal queue, anchor metadata, session topology | session owner plus admitted participant roles | session-scoped, bounded, compactable | export only when a retention envelope admits it |
| `presence_plane_ephemeral` | cursor, selection, online status, follow target, presenter focus, viewer awareness | presenter/session scope for view state only | ephemeral, TTL bounded, not durable | not exportable except aggregate metadata when policy admits it |
| `captured_export_evidence` | sealed session archive, replay bundle, support packet, redacted audit export | evidence owner under session policy | immutable retained object with manifest hash | export/delete governed by explicit retention and redaction policy |

Rules:

- A `shared_session_artifact` may reference canonical state by opaque
  ref, snapshot id, revision id, or mutation id. It MUST NOT inline raw
  canonical file contents unless the session policy explicitly admits a
  retained text/comment/replay payload.
- A `presence_plane_ephemeral` row MUST NOT be replayed as an edit, save
  order, control grant, or presenter handoff after disconnect.
- A `captured_export_evidence` row is attributable evidence. It may
  explain what happened and may seed a review/replay view, but applying
  it back to a workspace requires a fresh import/review path that writes
  new local mutations.

## Session Layer

The collaboration session layer uses CRDT-style convergence for live
editable projections and review annotations. It bridges to local buffers
through explicit checkpoints and proposal/admission records:

- `editor_buffer` session-projection rows carry bounded operation
  windows and compaction checkpoints, not unbounded canonical history.
- `participant_proposal` rows preserve local unsent edits when a
  participant is offline, downgraded, or blocked from submitting shared
  writes.
- `anchor_record` rows bind comments, review notes, and shared debug
  locations to opaque buffer/node refs plus drift classes.
- `session_snapshot` rows summarize a compacted session window and name
  the canonical source refs they were derived from.

The existing collaboration contracts remain the vocabulary source for
session lifecycle, follow/presenter state, control grants, retention,
consent, and downgrade labels:

- [`docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md)
- [`docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md)
- [`docs/collaboration/consent_retention_contract.md`](../collaboration/consent_retention_contract.md)

## Presence Plane

Presence is a separate plane, not a shortcut into shared edit authority.
It carries online/offline state, cursor/selection hints, focus/follow
signals, presenter stance, viewer awareness, and latency/degradation
health. Presence MAY be sampled, coalesced, dropped, or expired before
shared text convergence is affected.

Presence loss follows this ordering:

1. Degrade presence/awareness first.
2. Degrade live follow/presenter projection next.
3. Pause shared convergence or control lanes only when their own health
   or policy check fails.
4. Preserve local editing and unsent work in every case.

No degraded presence state queues hidden control grants, queues
presenter handoff, implies save ordering, or injects input into another
participant's workspace.

## Compaction And Snapshots

CRDT metadata is bounded by snapshot and compaction rules:

- Every live session declares `operation_window_policy_class`,
  `snapshot_cadence_class`, `tombstone_gc_class`, and
  `snapshot_authority_class`.
- Compaction MAY replace older operation windows with snapshot refs for
  live convergence and replay. It MUST preserve enough attribution,
  participant role, policy epoch, and anchor-drift metadata to audit or
  export retained evidence.
- Compaction MUST NOT delete unsent local work, pending outbound
  proposals, local journals, or user-owned recovery state.
- A snapshot that is retained as evidence is immutable. A later
  correction or redaction mints a replacement evidence object and links
  the prior object; it never rewrites the sealed artifact in place.

## Downgrade And Offline Continuation

Offline continuation is a typed narrowing, not false parity with a live
shared session:

- `relay_outage_local_continuation` keeps local editing active, moves
  session propagation to a degraded state, and records that presence may
  be stale or absent.
- `viewer_fallback_preserved_local_work` stops future shared writes for
  the participant while preserving unsent local work in a local journal
  or pending outbound proposal queue.
- `provider_outage_archive_deferred` lets local work and local evidence
  continue while managed archive publication waits for recovery.
- `reconciliation_pending_review` requires target, policy, auth, and
  snapshot compatibility checks before any queued proposal is replayed.

Presenter handoff, control grants, shared terminal/debug input, remote
execution, Git pushes, destructive provider writes, and paid model
dispatches are not silently queued while offline. The user must rejoin
or reauthorize them under fresh context.

## Recovery, Replay, Export, Retention, And Redaction

Every retained collaboration object declares:

- who authored it and which role admitted it;
- which session and policy epoch admitted retention;
- whether it is canonical project state, shared-session state, presence
  state, or captured export evidence;
- which recovery paths are available;
- which redaction profile applies before support, admin, or offboarding
  export; and
- which delete/hold posture governs retained copies.

Replay surfaces are view/review surfaces unless an explicit apply/import
action writes new canonical workspace mutations. Support bundles and
admin exports carry manifests, refs, hashes, omission reasons, and
redaction summaries by default. Raw code, terminal bytes, debug payloads,
URLs, absolute paths, user identifiers, tokens, secrets, and provider
payloads remain excluded unless a higher-trust retention path explicitly
admits them.

## Linked Artifacts

- [`schemas/collab/session_topology.schema.json`](../../schemas/collab/session_topology.schema.json)
  freezes the session topology, participant role, share-scope,
  presence-plane, compaction/snapshot, downgrade, and offline
  continuation rows.
- [`schemas/collab/shared_object_authority.schema.json`](../../schemas/collab/shared_object_authority.schema.json)
  freezes authority, storage-boundary, anchor-drift, export evidence,
  retention, replay, and redaction rows for shared objects.
- [`fixtures/collab/session_cases/`](../../fixtures/collab/session_cases/)
  contains worked examples for relay outage, viewer fallback,
  presenter/follow degradation, anchor drift, and archive sealing.

## Consequences

- Collaboration code cannot treat the CRDT operation log as the project
  database or use a session snapshot as a writable file source.
- Presence, follow, presenter, shared edit, shared terminal/debug, and
  archive/evidence lanes have separate authority and retention rows.
- Provider outages and offline continuation preserve local work with
  typed downgrade states instead of claiming that live collaboration is
  still equivalent.
- Notebook sharing, review, support, and retained audit surfaces can cite
  the same authority, storage-boundary, export, and redaction vocabulary.

## Alternatives Considered

- **Permanent CRDT as canonical storage.** Rejected because it collapses
  local save truth, VCS workspace semantics, and session evidence into
  one log with high metadata and compaction risk.
- **Single mixed presence/text/control transport.** Rejected because
  presence loss must be cheap and disposable, while shared text,
  terminal/debug control, and retained evidence have distinct authority
  and redaction rules.
- **Archive as session database.** Rejected because archives are evidence
  outputs with retention/delete posture, not mutable project truth.

## Source Anchors

- `.t2/docs/Aureline_PRD.md:150` — CRDTs are the collaboration/session
  protocol, not the only text representation.
- `.t2/docs/Aureline_PRD.md:241` — live session state uses CRDTs but
  canonical files persist to the VCS workspace.
- `.t2/docs/Aureline_PRD.md:243` — presence/awareness is separate,
  disposable, and session-scoped.
- `.t2/docs/Aureline_PRD.md:1305` — collaboration is a separate
  capability plane with explicit permissions, persistence, and evidence.
- `.t2/docs/Aureline_PRD.md:1320` — presence channels are separated from
  sensitive shell/debug channels.
- `.t2/docs/Aureline_PRD.md:1325` — sessions are ephemeral by default and
  retention requires explicit policy.
- `.t2/docs/Aureline_PRD.md:1326` — disconnects never corrupt local
  buffers or autosave journals.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9833` — editable
  text uses a CRDT-backed session log bridged to local buffers.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9834` — presence
  disappearance never implies edit rollback or save order.
- `.t2/docs/Aureline_Technical_Design_Document.md:1602` — presence and
  collaboration control are not silently queued offline.

## Supersession History

None.
