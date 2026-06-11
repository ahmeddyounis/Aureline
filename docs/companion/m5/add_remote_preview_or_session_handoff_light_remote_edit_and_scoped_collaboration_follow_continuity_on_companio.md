# Companion Remote-Preview, Session-Handoff, Light-Remote-Edit, and Collaboration-Follow Continuity

This document is the human-readable contract for the companion surfaces that let a
browser or mobile companion **remotely preview** and **hand off** an active desktop
session, perform a single **bounded light remote edit**, and **follow** a
collaborator within a host-revocable **shared scope**. The machine-readable truth
source is the checked-in support export; later browser/mobile companions, the
desktop companion panel, diagnostics, support exports, and Help/About surfaces
ingest it instead of cloning status text.

- Record kind: `add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio`
- Schema: `schemas/companion/add-remote-preview-or-session-handoff-light-remote-edit-and-scoped-collaboration-follow-continuity-on-companio.schema.json`
- Support export: `artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/support_export.json`
- Markdown summary: `artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio.md`
- Fixtures: `fixtures/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/`
- Producer crate: `aureline-companion`

## Surfaces and matrix inheritance

The packet has three surfaces. Each one inherits its qualification and staged
rollout stage from a frozen M5 companion-matrix lane (see
`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so the surface never claims more than the matrix qualifies.

| Surface | Matrix lane | Scope | Qualification | Rollout stage |
| --- | --- | --- | --- | --- |
| `remote_preview_handoff` | `companion_session_follow` | `read_only` | beta | staged_rollout |
| `light_remote_edit` | `companion_light_edit` | `bounded_write_relayed_to_host` | preview | early_access |
| `collaboration_follow` | `companion_review` | `read_only` | stable | general_availability |

## Bounded read/write scope

Read/write scope is bounded and explicit per surface:

- **Remote-preview-handoff** is strictly read-only. The companion previews a host
  session (`editor_preview`, `agent_run_preview`, `terminal_preview`,
  `diff_preview`, `build_preview`) and resumes it via an exact desktop handoff, but
  the preview is a *projection* of the authoritative local core and never mutates
  host state.
- **Collaboration-follow** is strictly read-only. The companion follows a
  collaborator (`driver`, `navigator`, `observer`, `reviewer`) within a bounded
  shared scope (`shared_file_scope`, `shared_review_scope`, `shared_session_scope`)
  and hands off into the shared review/session location, but never edits on the
  collaborator's behalf.
- **Light-remote-edit** is the *only* write-capable surface, and its writes are
  capped. Every item declares a `write_bound_summary`, carries
  `read_write_scope = bounded_write_relayed_to_host`, sets
  `requires_host_approval = true`, and moves through host-owned states
  (`drafted`, `relayed_for_preview`, `awaiting_host_approval`, `applied_by_host`,
  `rejected_by_host`). There is no "applied by companion" state and no unbounded
  authoring. The desktop host stays authoritative for every change.

The `scope_contract` block asserts these guarantees for the whole packet, and the
validator rejects any read-only surface item carrying a write scope, any
light-edit item that is not bounded or host-approved, and any collaboration item
not marked `scope_bounded`.

## Scoped collaboration-follow continuity

Collaboration-follow is never an unbounded view of a collaborator's machine. Every
item is confined to a host-granted `follow_scope` and sets `scope_bounded = true`.
When the host withdraws the scope, the follow narrows to `scope_revoked` rather than
continuing — the `collaboration_scope_revocable` continuity guarantee — and the
collaboration-follow surface narrows one qualification step.

## Local-core continuity (never strand local work)

This is the lane's distinct value. The `continuity_guarantee` block asserts that:

- the **local core stays authoritative** throughout (`local_core_authoritative`);
- a **session handoff never strands** user-owned local work
  (`handoff_never_strands_local_work`) — every item sets
  `local_work_preserved = true`, and the validator rejects any item that does not;
- a **remote preview is a read-only projection**, never an authoring surface
  (`preview_is_read_only_projection`); and
- a **revoked collaboration scope narrows the follow honestly**
  (`collaboration_scope_revocable`).

The `handoff_continuity` field on each preview item records the continuity state
(`local_authoritative`, `handoff_staged`, `handoff_resumed`,
`handoff_unavailable`). When a handoff cannot complete, it degrades to
`handoff_unavailable` and the work falls back to the local-authoritative state — it
is never dropped.

## Stale-state honesty

Every item carries a `freshness` state — `live`, `cached`, `stale`, or `unknown`.
Stale-state honesty means a degraded item is never re-shown as live:

- Any item whose freshness is `stale` or `unknown` MUST set `stale_label_shown =
  true`; the validator rejects an unlabeled stale/unknown item.
- The `stale_state_honesty` block asserts stale and unknown items are labeled,
  stale is never shown as live, and a freshness floor is enforced before an item
  is shown.

## Exact desktop handoff

Every item carries an exact `desktop_handoff` resolving to a precise host location
(file location, review panel, or agent session). The handoff carries an opaque
deep-link ref — never a payload body — and records whether an active host session
is required to resume it.

## Downgrade-aware: narrows, never hides

`apply_companion_continuity_degradation` narrows from a per-surface observation, and
records the reasons in `degraded_labels` rather than hiding the surface:

| Signal | Effect |
| --- | --- |
| Relay unavailable | Narrows every surface one step; forces every live/cached item to `stale` and labels it (`relay_unavailable`, `freshness_downgraded_to_stale`) |
| Proof stale | Labels `proof_stale`; narrows every surface one step |
| Upstream matrix lane narrowed | Labels `upstream_matrix_narrowed`; narrows every surface one step |
| Host session inactive | Downgrades every host-dependent handoff to `unresolved`, marks affected previews `handoff_unavailable`, and narrows the light-remote-edit surface, since a write can no longer relay (`host_session_inactive`, `handoff_target_unresolved`) |
| Trust narrowed | Narrows the light-remote-edit surface (`trust_narrowed`) |
| Collaboration scope revoked | Marks every collaboration item `scope_revoked` and narrows the collaboration-follow surface (`collaboration_scope_revoked`) |

Degradation narrows the claim; it never corrupts the packet, which still validates
after any single or combined observation, and local work is never stranded.

## Locality

- **Stays local:** session state, edit targets, and collaboration scopes are owned
  by the local core and stay inspectable offline; a remote preview never holds
  authoritative state.
- **Staged:** remote preview streaming, session handoff, bounded light remote
  edit, and scoped collaboration-follow roll out per cohort and capability gate.
- **Requires provider/admin continuity:** live preview, exact handoff, relaying a
  bounded edit for host approval, and a shared collaboration scope require the
  companion relay and an active host session; the local core never depends on them
  to function and never strands local work.

## Boundary safety

The packet is export-safe metadata only. It carries redacted summaries and opaque
refs — never credential bodies, raw provider payloads, or raw session/edit/
collaboration bodies. The validator runs a forbidden-material heuristic over the
serialized export.

## Regenerating

The checked-in support export, Markdown summary, and fixtures are regenerated
deterministically from the first-consumer builder:

```text
cargo run -p aureline-companion --example dump_companion_continuity_surface -- canonical > artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/support_export.json
cargo run -p aureline-companion --example dump_companion_continuity_surface -- markdown  > artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio.md
```
