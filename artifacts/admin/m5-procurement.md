# Procurement — evidence companion

Human-readable companion to
[`/fixtures/admin/m5-procurement/canonical_procurement.json`](../../fixtures/admin/m5-procurement/canonical_procurement.json)
and its boundary schema
[`/schemas/admin/m5-procurement.schema.json`](../../schemas/admin/m5-procurement.schema.json).
It gives reviewers the rendered per-profile procurement / verification packets,
renewal / trial / seat-change cards, and admin-handoff packets without reading the
JSON. The contract narrative lives in
[`/docs/admin/m5-procurement.md`](../../docs/admin/m5-procurement.md), and the
frozen object model it binds back to lives in
[`/artifacts/admin/m5-admin-plane.md`](./m5-admin-plane.md).

- Bundle id: `m5-procurement:bundle:0001`
- Record kind: `m5_procurement_bundle`
- Binds matrix: `m5-admin-plane:matrix:0001`
- Profiles: 4 · Event cards: 7 · Invariants: 18

## Verification packets and coverage

| Profile | Deployment | Verification posture | Packet state | Within validity | Completeness | Coverage state |
| --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | managed_cloud | signed_verified | active_enforced | yes | complete | active_enforced |
| `self_hosted` | self_hosted | signed_verified | active_enforced | yes | complete | active_enforced |
| `sovereign_air_gapped` | sovereign_air_gapped | signature_expired | signature_unverified | no | partial_offline | signature_unverified |
| `mirrored_offline` | managed_cloud | unverifiable_offline | unconfirmed_stale | yes | partial_offline | unconfirmed_stale |

The managed-cloud and self-hosted packets are signed and currently verified within
their validity windows. The sovereign packet is a signed offline bundle past its
validity window, so it is labeled `signature_unverified` rather than shown verified;
the mirrored packet is last-synced and labeled `unconfirmed_stale`. No stale,
unverified, or past-validity packet is shown under a confirmed `active_enforced`
state. Every profile stays locally inspectable with no vendor console and keeps
export reachable without a paid seat.

## Renewal / trial / seat-change cards

| Profile | Events |
| --- | --- |
| `managed_cloud` | renewal · seat_increase |
| `self_hosted` | trial_start · plan_downgrade |
| `sovereign_air_gapped` | trial_expiry · seat_decrease |
| `mirrored_offline` | cancellation |

Across the bundle every commercial event class appears at least once. Each card
discloses its event type, effective date, impacted managed features, as-of date,
local-only path, and export/support next step. In every card the ordered next
actions place the recovery actions — export, delete, support, continue local-only —
ahead of the single commercial call-to-action:

```
1. export_user_data → 2. delete_user_data → 3. open_support → 4. continue_local_only → 5. <commercial CTA>
```

Every card is flagged `outranks_recovery_actions = false` and
`requires_paid_seat_for_recovery = false`, so a renewal, trial-expiry, seat-decrease,
plan-downgrade, or cancellation prompt never crowds out user-owned data recovery.

## Admin-handoff packets

| Profile | Build | Channel | Install | Workspace archetype | Bundle ids |
| --- | --- | --- | --- | --- | --- |
| `managed_cloud` | build.managed_cloud.2026.06 | pinned_managed | managed_image | managed_org_workspace | 2 |
| `self_hosted` | build.self_hosted.2026.06 | extended | per_machine | self_hosted_workspace | 2 |
| `sovereign_air_gapped` | build.sovereign_air_gapped.2026.06 | pinned_offline | sovereign_image | sovereign_workspace | 2 |
| `mirrored_offline` | build.mirrored_offline.2026.06 | pinned_offline | managed_image | mirrored_workspace | 2 |

Each handoff is auto-derived (`auto_derived = true`), carries the affected features
and an export-safe summary, and reuses the effective-policy and decision-history
objects by ref.

## Canonical objects reused (not restated)

Every packet, card, and handoff reuses the canonical managed-state objects by ref.
Across the bundle all six families appear: `effective_policy`, `entitlement_seat`,
`retention_deletion`, `endpoint_posture` (verification packets);
`entitlement_seat`, `offboarding_continuity` (event cards); and `effective_policy`,
`decision_history` (admin handoffs).

## Invariants (all hold)

| Invariant | Holds |
| --- | --- |
| `procurement.surface_states_within_matrix` | yes |
| `procurement.profiles_covered` | yes |
| `procurement.consumer_parity` | yes |
| `procurement.verification_no_silent_green` | yes |
| `procurement.validity_labeled` | yes |
| `procurement.export_paths_present` | yes |
| `procurement.owner_scope_and_asof` | yes |
| `procurement.evidence_refs_present` | yes |
| `procurement.events_disclose_impact` | yes |
| `procurement.events_never_outrank_recovery` | yes |
| `procurement.handoff_complete` | yes |
| `procurement.reuses_canonical_objects` | yes |
| `procurement.no_paid_seat_for_recovery` | yes |
| `procurement.locally_inspectable_offline` | yes |
| `procurement.export_parity` | yes |
| `procurement.coverage_labeled` | yes |
| `procurement.residual_dependencies_honest` | yes |
| `procurement.export_safe` | yes |

The freeze gate (`crates/aureline-policy/tests/m5_procurement.rs`) rebuilds the
bundle in code and asserts it equals this fixture byte-for-byte, so the rendered
packets cannot drift from the published artifact without failing CI.
