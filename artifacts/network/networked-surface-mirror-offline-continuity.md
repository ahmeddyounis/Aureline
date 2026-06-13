# Networked-Surface Mirror/Offline Continuity — Stable Packet

- Packet: `remote:networked_surface_mirror_offline_continuity:default`
- Schema version: `1`
- Contract ref: `remote:networked_surface_mirror_offline_continuity:v1`
- Schema: `schemas/network/networked_surface_mirror_offline_continuity.schema.json`
- Runtime owner: `aureline_remote::networked_surface_mirror_offline_continuity`
- Evidence index: `artifacts/release/m5/xt12-evidence-index.md`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all (5)

This continuity packet is the capstone of the networked-surface
transport-governance lane. It makes mirror-only, local-file-bundle,
public-direct, blocked, and deferred route handling explicit across the claimed
M5 artifact families (docs packs, registries, model packs, request workspaces,
and companion handoffs), surfaces typed stale-mirror warnings, declares each
family's public-fallback rule, and proves local-core work continues when a
network action is denied or deferred.

## Continuity coverage

| Artifact family | Route handling | Mirror/offline behavior | Stale-mirror warning | Public-fallback rule | Local-core workflow |
|---|---|---|---|---|---|
| `docs_pack` | `local_file_bundle` | `cached_offline` | `none` | `no_public_fallback` | local documentation reading and offline search |
| `registry` | `mirror_route` | `mirror_first_then_deny` | `stale_within_grace` | `mirror_only_no_fallback` | already-installed extensions and tools continue |
| `model_pack` | `blocked` | `deny_all` | `none` | `deny_all_no_fallback` | already-installed local models continue |
| `request_workspace` | `public_direct` | `local_core_only` | `none` | `explicit_public_direct_allowed` | local request-collection editing and replay |
| `companion_handoff` | `deferred` | `offline_grace` | `none` | `no_public_fallback` | the local workspace continues without the companion |

All five route-handling behaviors are distinguished across the covered families.

## Stable field catalog (product / CLI / support parity)

Every continuity record renders these fields, in this order, on every family, so
the route-handling tokens and field names match across product UI, CLI/headless
output, and support exports:

`artifact_family`, `continuity_route`, `origin_scope`, `egress_class`,
`mirror_offline_behavior`, `stale_mirror_warning`, `public_fallback_rule`,
`local_core_workflow`, `denial_reason`.

## Key invariants verified

1. All five required artifact families have a continuity record, and all five route-handling behaviors are distinguished.
2. No raw private material is present on any record (`raw_private_material_excluded: true`); mirrors and trust are named by opaque ref only.
3. Every record resolved through the shared transport-governance layer (`no_bypass: true`).
4. No mirror-only or deny-all profile permits a silent fall-through to the public internet (`no_silent_public_fallback: true`).
5. The only `deferred` family (`companion_handoff`) queues an idempotent action; no non-idempotent action enters a replay queue.
6. Every record preserves local-core continuity (`local_core_continuity_preserved: true`).
7. The `blocked` family (`model_pack`) carries a typed `policy_blocked` reason.
8. Every record carries a non-empty trust-proof ref.
9. Every record's trust proof is fresh (or stale only within an accepted grace window).
10. Every record's declared public-fallback rule is consistent with its route handling.
11. No record serves a mirror whose stale-mirror warning is blocking instead of blocking the route.

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false` (narrow reason `raw_private_material_exposed`),
- a record with `no_bypass: false` (narrow reason `bypassed_shared_governance`),
- a mirror-only or deny-all profile that would silently reach the public internet (narrow reason `silent_public_fallback_resolved`),
- a deferred record that queues a non-idempotent action (narrow reason `non_idempotent_replay_queued`).

A missing required family narrows the packet to `preview`. A block without a reason, a stale-beyond-window proof, an inconsistent fallback rule, a stale-mirror served beyond grace, or a record that does not preserve local-core continuity narrows the affected row to `beta`, which lets release and support tooling detect and automatically narrow stale or under-qualified rows before publication.

## Drill coverage

| Drill | Record mutated | Outcome |
|---|---|---|
| `drill_missing_family_preview` | drops `docs_pack` | `preview` |
| `drill_raw_material_withdrawn` | `registry` exposes raw material | `withdrawn` |
| `drill_silent_fallback_withdrawn` | mirror-only `registry` flips to `public_direct` | `withdrawn` |
| `drill_non_idempotent_withdrawn` | `companion_handoff` defers a non-idempotent action | `withdrawn` |
| `drill_blocked_no_reason_beta` | `model_pack` blocked without a reason | row `beta` |
| `drill_stale_mirror_beta` | `registry` serves a stale-beyond-grace mirror | row `beta` |

Fixtures: `fixtures/network/networked_surface_mirror_offline_continuity/`.
