# Companion Session-Follow and Incident-Awareness Surfaces

This document is the human-readable contract for the companion surfaces that let a
browser or mobile companion **follow** an active desktop session and stay
**aware** of incidents, plus the single **bounded light-edit** surface that may
write at all. The machine-readable truth source is the checked-in support export;
later browser/mobile companions, the incident workspace, the desktop companion
panel, diagnostics, support exports, and Help/About surfaces ingest it instead of
cloning status text.

- Record kind: `ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty`
- Schema: `schemas/companion/ship-session-follow-and-incident-awareness-surfaces-with-bounded-read-write-scope-and-stale-state-honesty.schema.json`
- Support export: `artifacts/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/support_export.json`
- Markdown summary: `artifacts/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty.md`
- Fixtures: `fixtures/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/`
- Producer crate: `aureline-companion`

## Surfaces and matrix inheritance

The packet has three surfaces. Each one inherits its qualification and staged
rollout stage from a frozen M5 companion-matrix lane (see
`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so the surface never claims more than the matrix qualifies.

| Surface | Matrix lane | Scope | Qualification | Rollout stage |
| --- | --- | --- | --- | --- |
| `session_follow` | `companion_session_follow` | `read_only` | beta | staged_rollout |
| `incident_awareness` | `incident_workspace` | `read_only` | stable | general_availability |
| `bounded_light_edit` | `companion_light_edit` | `bounded_write_relayed_to_host` | preview | early_access |

## Bounded read/write scope

Read/write scope is bounded and explicit per surface:

- **Session-follow** is strictly read-only. The companion observes an active
  desktop session (`active_editor`, `agent_run`, `terminal_stream`,
  `debug_session`, `review_session`) and resumes it via an exact desktop handoff,
  but never mutates host state.
- **Incident-awareness** is strictly read-only. The companion sees incident
  severity, awareness state, and attribution, and hands off into the incident
  workspace, but never edits the incident.
- **Bounded light-edit** is the *only* write-capable surface, and its writes are
  capped. Every item declares a `write_bound_summary`, carries
  `read_write_scope = bounded_write_relayed_to_host`, sets
  `requires_host_approval = true`, and moves through host-owned states
  (`drafted`, `relayed_for_preview`, `awaiting_host_approval`, `applied_by_host`,
  `rejected_by_host`). There is no "applied by companion" state and no unbounded
  authoring. The desktop host stays authoritative for every change.

The `scope_contract` block asserts these guarantees for the whole packet, and the
validator rejects any read-only surface item carrying a write scope and any
light-edit item that is not bounded or host-approved.

## Stale-state honesty

Every item carries a `freshness` state — `live`, `cached`, `stale`, or `unknown`.
Stale-state honesty means a degraded item is never re-shown as live:

- Any item whose freshness is `stale` or `unknown` MUST set `stale_label_shown =
  true`; the validator rejects an unlabeled stale/unknown item.
- The `stale_state_honesty` block asserts stale and unknown items are labeled,
  stale is never shown as live, and a freshness floor is enforced before an item
  is shown.

## Exact desktop handoff

Every item carries an exact [`desktop_handoff`] resolving to a precise host
location (file location, review panel, incident workspace, or agent session). The
handoff carries an opaque deep-link ref — never a payload body — and records
whether an active host session is required to resume it.

## Downgrade-aware: narrows, never hides

`apply_companion_scope_degradation` narrows from a per-surface observation, and
records the reasons in `degraded_labels` rather than hiding the surface:

| Signal | Effect |
| --- | --- |
| Relay unavailable | Narrows every surface one step; forces every live/cached item to `stale` and labels it (`relay_unavailable`, `freshness_downgraded_to_stale`) |
| Proof stale | Labels `proof_stale`; narrows every surface one step |
| Upstream matrix lane narrowed | Labels `upstream_matrix_narrowed`; narrows every surface one step |
| Host session inactive | Downgrades every host-dependent handoff to `unresolved`; narrows the bounded light-edit surface, since a write can no longer relay (`host_session_inactive`, `handoff_target_unresolved`) |
| Trust narrowed | Narrows the bounded light-edit surface (`trust_narrowed`) |
| Incident attribution lost | Marks every incident item `unattributed` and narrows the incident-awareness surface (`incident_attribution_lost`) |

Degradation narrows the claim; it never corrupts the packet, which still validates
after any single or combined observation.

## Locality

- **Stays local:** session state, incident evidence, and edit targets are owned by
  the local core and stay inspectable offline.
- **Staged:** companion session-follow streaming and bounded light-edit roll out
  per cohort and capability gate.
- **Requires provider/admin continuity:** live follow, exact handoff, and relaying
  a bounded edit for host approval require the companion relay and an active host
  session; the local core never depends on them to function.

## Boundary safety

The packet is export-safe metadata only. It carries redacted summaries and opaque
refs — never credential bodies, raw provider payloads, or raw session/incident
bodies. The validator runs a forbidden-material heuristic over the serialized
export.

## Regenerating

The checked-in support export, Markdown summary, and fixtures are regenerated
deterministically from the first-consumer builder:

```text
cargo run -p aureline-companion --example dump_companion_scope_surface -- canonical > artifacts/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/support_export.json
cargo run -p aureline-companion --example dump_companion_scope_surface -- markdown  > artifacts/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty.md
```
