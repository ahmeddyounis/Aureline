# Operator handoff bundles & shift digests — evidence companion

Human-readable companion to
[`/fixtures/ops/m5-handoff-digests/canonical_handoff_digests.json`](../../fixtures/ops/m5-handoff-digests/canonical_handoff_digests.json)
and its boundary schema
[`/schemas/ops/m5-handoff-digests.schema.json`](../../schemas/ops/m5-handoff-digests.schema.json).
It gives reviewers the frozen packet, group, evidence, and invariant tables without
reading the JSON. The contract narrative lives in
[`/docs/ops/m5-handoff-digests.md`](../../docs/ops/m5-handoff-digests.md).

- Set id: `m5-handoff-digests:set:0001`
- Record kind: `m5_handoff_digest_set`
- Bound matrix: `fixtures/ops/m5-operator-surfaces/canonical_matrix.json`
  (`m5_operator_surface_matrix`)
- Packets: 4 · Object groups: 9 · Evidence items: 11 · Unresolved questions: 6 ·
  Packet kinds: 2 · Storage classes: 4 · Reopen-anchor classes: 4 · Severities: 4 ·
  Question statuses: 4 · Target audiences: 3 · Share postures: 3 · Object kinds: 6 ·
  Invariants: 21

## Packets

| Packet | Kind | Target / audience | Scope / share posture | Default redaction | Groups | Questions |
| --- | --- | --- | --- | --- | --- | --- |
| `outgoing_shift_handoff` | handoff_bundle | incoming_on_call / next_operator_shift | shared_team / workspace_shared | operator_only_restricted | 2 | 2 |
| `client_status_handoff` | handoff_bundle | customer_success_lead / client_facing | shared_team / workspace_shared | metadata_safe_default | 2 | 1 |
| `daily_operations_digest` | shift_digest | operations_lead / team_wide | managed_org / org_shared | internal_support_restricted | 3 | 2 |
| `night_shift_digest` | shift_digest | night_on_call / next_operator_shift | local_private / private | private_triage_only | 2 | 1 |

## Object groups (severity before chronology) and reopen anchors

Groups are ordered by `severity` (most severe first); each preserves its latest
update time and major blocker, and reopens onto the canonical object or a truthful
placeholder — never a generic dashboard.

| Packet | Severity | Object | Kind | Blocker | Latest update | Reopen anchor |
| --- | --- | --- | --- | --- | --- | --- |
| outgoing | sev1 | `aureline://incident/inc-3001` | incident_record | blocked | 2026-06-21T22:15Z | `live_object` |
| outgoing | sev2 | `aureline://support-case/case-8801` | support_case | blocked | 2026-06-21T21:00Z | `mirrored_offline_view` |
| client | sev2 | `aureline://incident/inc-3001` | incident_record | none | 2026-06-21T22:15Z | `cached_object_snapshot` |
| client | sev3 | `aureline://release-gate/rel-204` | release_gate | none | 2026-06-21T20:10Z | **`truthful_placeholder`** |
| daily | sev1 | `aureline://incident/inc-3001` | incident_record | blocked | 2026-06-22T02:00Z | `live_object` |
| daily | sev2 | `aureline://admin-approval/req-501` | admin_approval_request | blocked | 2026-06-22T06:00Z | `cached_object_snapshot` |
| daily | sev3 | `aureline://service-health/svc-auth` | service_health_record | none | 2026-06-22T01:00Z | `mirrored_offline_view` |
| night | sev2 | `aureline://support-case/case-8802` | support_case | none | 2026-06-22T03:30Z | `live_object` |
| night | sev4 | `aureline://review-item/rev-77` | review_item | none | 2026-06-22T05:00Z | `cached_object_snapshot` |

The archived `release-gate/rel-204` is the lived proof of reopen-safe continuity:
its object no longer resolves, so its anchor is a `truthful_placeholder` that names
the archived gate rather than dropping the next operator on an unscoped home screen.

## Evidence — the storage / freshness distinction (never flattened)

`is_live` is true only for a `live_link`; `can_refresh` is false only for a
`snapshot`. The roll-up counts each storage class separately.

| Packet | Object | Storage class | Freshness | is_live | can_refresh |
| --- | --- | --- | --- | --- | --- |
| outgoing | inc-3001 | `live_link` | fresh | yes | yes |
| outgoing | inc-3001 | `cached` | recent | no | yes |
| outgoing | case-8801 | `mirrored` | stale | no | yes |
| outgoing | case-8801 | `snapshot` | recent | no | no |
| client | inc-3001 | `cached` | recent | no | yes |
| client | rel-204 | `snapshot` | very_stale | no | no |
| daily | inc-3001 | `live_link` | fresh | yes | yes |
| daily | req-501 | `cached` | recent | no | yes |
| daily | svc-auth | `mirrored` | recent | no | yes |
| night | case-8802 | `snapshot` | fresh | no | no |
| night | rev-77 | `cached` | stale | no | yes |

The `outgoing_shift_handoff` alone proves all four storage classes side by side
(live link, cached, mirrored, snapshot) — the distinction is lived, not theoretical.

## Per-packet storage roll-up (counted separately, never merged)

| Packet | live_link | cached | mirrored | snapshot |
| --- | --- | --- | --- | --- |
| outgoing | 1 | 1 | 1 | 1 |
| client | 0 | 1 | 0 | 1 |
| daily | 1 | 1 | 1 | 0 |
| night | 0 | 1 | 0 | 1 |

## Unresolved questions (what remains unresolved / next safe action)

| Packet | Status | Question | Owner | Object | Next safe action |
| --- | --- | --- | --- | --- | --- |
| outgoing | open | Will the connection-pool ceiling hold past the morning peak? | incoming_on_call | inc-3001 | Watch the auth-latency tile; if it re-reddens, execute the prepared rollback. |
| outgoing | blocked | Can the canary hotfix redeploy once the read-only window lifts? | release_operator | case-8801 | Hold; re-attempt only after the window lifts and a fresh approval is captured. |
| client | needs_decision | Does the customer need a written incident summary? | customer_success_lead | inc-3001 | Decide with the IC; share only the metadata-safe summary, never raw evidence. |
| daily | open | Is the auth incident safe to close? | operations_lead | inc-3001 | Keep open until 4h stable; the staged rollback stays ready. |
| daily | investigating | Who approves the held access grant? | security_owner | req-501 | Route to the security owner; the reviewer cannot self-approve. |
| night | open | Should case-8802 page the daytime owner? | night_on_call | case-8802 | No page overnight unless it escalates to Sev1; carry it into the morning digest. |

All four question statuses (`open`, `investigating`, `blocked`, `needs_decision`)
appear, each naming an owner, a canonical object, and a next safe action.

## Scope, share posture, and the export gate

| Packet | Scope | Share posture | Boundary ack | What crosses on share |
| --- | --- | --- | --- | --- |
| outgoing | shared_team | workspace_shared | required | object identity, grouping, severities, updates, blockers, evidence with storage class & freshness, questions, ownership — never raw payloads/credentials/URLs |
| client | shared_team | workspace_shared | required | metadata-safe labels, severities, updates, and the open decision; internal evidence bodies, raw payloads, credentials, URLs never cross |
| daily | managed_org | org_shared | required | the same fields, visible org-wide under managed governance |
| night | local_private | private | not required | nothing crosses until scope changes; export is a local snapshot only |

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `continuity.surface_binding` | Every packet binds its operator-surface matrix family (handoff bundle or shift digest) by the matrix's own surface id. |
| `continuity.both_surfaces_present` | The set proves both the handoff-bundle and the shift-digest surfaces. |
| `continuity.canonical_object_linkage` | Every group object, evidence ref, question link, and resolvable reopen target is a canonical aureline:// handle. |
| `continuity.storage_class_not_flattened` | The set proves all four storage classes — live link, cached, mirrored, and snapshot — and every roll-up counts them separately, never flattening them into one blob. |
| `continuity.evidence_freshness_preserved` | Every evidence item carries an origin, a captured-at, and live/refresh flags computed from its storage class. |
| `continuity.digests_group_by_severity_before_chronology` | Every digest orders its groups by severity (most severe first) and orders events chronologically only within a group. |
| `continuity.all_packets_grouped_and_chronological` | Every packet — handoff bundle and digest alike — keeps its groups severity-ordered and its within-group events chronological. |
| `continuity.latest_update_and_blockers_preserved` | Every group preserves its latest update time and its blocker reason, its severity is the most severe of its events, and the roll-up's latest update is the newest group's. |
| `continuity.reopen_lands_on_object_or_placeholder` | Every reopen anchor resolves to a canonical object or a truthful placeholder that names what the object was — never a generic dashboard. |
| `continuity.reopen_anchor_classes_distinct` | The set proves all four reopen-anchor classes — live object, cached snapshot, mirrored offline view, and truthful placeholder. |
| `continuity.unresolved_questions_answerable` | Every packet carries unresolved questions, each naming an owner, a canonical object, and a next safe action, with a reason when blocked. |
| `continuity.scope_boundary_truth` | Every packet declares a scope and a matching export gate that names what crosses the boundary on share/export and requires acknowledgement above private scope. |
| `continuity.share_postures_distinct` | The set proves a private, a workspace-shared, and an org-shared packet. |
| `continuity.ownership_present` | Every packet names an owning role, a decision right, and the target role it is handed to. |
| `continuity.export_parity` | Each packet's frozen export equals re-exporting it and is labeled snapshot_only. |
| `continuity.export_preserves_storage_distinction` | Each export preserves the exact object groups (with every evidence item's storage class and freshness), unresolved questions, reopen anchor, and roll-up. |
| `continuity.roll_up_answers_three_questions` | Each roll-up answers what changed, what remains unresolved, and the next safe action, and its headline keeps the storage classes distinct. |
| `continuity.first_real_packets_present` | The outgoing-shift handoff, client handoff, daily operations digest, and night-shift digest are all present. |
| `continuity.object_kinds_distinct` | The set proves all six canonical object kinds across its groups. |
| `continuity.severities_distinct` | The set proves all four severities across its groups. |
| `continuity.stable_ids_unique` | Packet, group, evidence, and question ids are unique. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-support --example dump_m5_handoff_digests > \
  fixtures/ops/m5-handoff-digests/canonical_handoff_digests.json

# Freeze gate: in-code set must equal the checked-in fixture
cargo test -p aureline-support --test m5_handoff_digests

# Human-readable projection
cargo run -p aureline-support --example dump_m5_handoff_digests -- --lines
```
