# Organization Admin, Provisioning, Seat Lifecycle, and Rollout Truth Proof Packet

- Packet: `policy:organization-admin-truth:seeded:0001`
- Schema version: `1`
- Contract ref: `policy:organization_admin_provisioning_seat_lifecycle_truth:v1`
- Qualification: `stable`
- Defects: `0`

## Coverage

| Area | Stable evidence |
| --- | --- |
| Organization overview | Org/tenant ref, deployment mode, policy source, seat summary, rollout summary, last sync, export action, and support action are present. |
| Provisioning sources | Provider cards cover `oidc`, `scim`, `signed_file_bundle`, and `manual`. |
| Lifecycle rows | Active, suspended, grace-window, and deprovisioned rows include principal, source of truth, role, seat class, lifecycle state, last seen, local safety, and action lineage. |
| Impact previews | `seat_transfer`, `suspension`, `downgrade`, `org_switch`, and `deprovision` preview managed AI, sync, collaboration, review, and marketplace impact. |
| Rollout rings | Stable and pilot ring audit rows include state, policy source, build/channel summary, counts, audit freshness, local safety, and rollback action. |
| Support/export | Export projection carries org/tenant ref, rollout ring refs, provisioning source tokens, seat lifecycle refs, and local-safety guarantee notes. |

## Guardrails

- Seat loss and deprovisioning are typed as `seat_loss` or
  `deprovisioning_impact`, never generic sign-in failure.
- Deprovision removes managed access only in the seeded packet; local clones,
  local history, unsaved recovery, and export/offboarding access remain explicit.
- Missing local editing, local history, unsaved-work, or export/offboarding
  guarantees withdraws the packet.
- Raw secret, token, key, or private payload exposure withdraws the packet.

## Fixture Set

- `page.json`
- `overview.json`
- `providers.json`
- `seats.json`
- `impacts.json`
- `rollout_rings.json`
- `summary.json`
- `defects.json`
- `support_export.json`
- `drill_local_safety_withdrawn.json`
- `drill_missing_provisioning_preview.json`
- `drill_generic_failure_beta.json`
