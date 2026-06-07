# Organization Admin, Provisioning, Seat Lifecycle, and Rollout Truth

This contract defines the canonical packet for enterprise-managed organization
administration truth. Admin, diagnostics, About, Help, support export, and
evaluation export surfaces consume
`aureline_policy::stabilize_organization_admin_provisioning_and_seat_lifecycle_truth`
instead of cloning managed-boundary prose.

## Stable Contract

The packet qualifies stable only when all of the following are true:

1. The organization overview names the org or tenant, deployment mode, policy
   source, seat summary, rollout-ring summary, last successful sync, and export
   and support actions.
2. Directory/provider cards cover `oidc`, `scim`, `signed_file_bundle`, and
   `manual` provisioning classes, each with provider state, freshness,
   scope/count summary, and fallback/manual path.
3. User or seat lifecycle rows expose principal ref, source of truth, role, seat
   class, lifecycle state, last seen, local-artifact safety note, and admin
   action/result lineage.
4. Lifecycle impact previews cover `seat_transfer`, `suspension`, `downgrade`,
   `org_switch`, and `deprovision`, including managed AI, sync, collaboration,
   review, and marketplace entitlement impact.
5. Seat loss, grace-window, downgrade, org switch, and deprovision previews keep
   local editing, local history, unsaved work recovery, local-only
   continuation, and export/offboarding rights explicit.
6. Rollout-ring audit rows expose ring class, ring state, policy source,
   build/channel summary, enrolled and drifted counts, last audit time, local
   safety, and rollback action.
7. Tenant/org identity, provisioning source, rollout ring, and local-safety
   truth remain visible in admin, diagnostics, About, Help, support packet, and
   export packet projections.
8. Provider outage, auth drift, scope mismatch, seat loss, region/policy
   blocker, and deprovisioning impact use typed failure tokens. Generic admin
   error or generic sign-in failure copy does not qualify.
9. Raw secrets, bearer tokens, private keys, raw provisioning payloads, and
   private tenant payloads stay outside the packet.

## Qualification

`validate_organization_admin_truth_page` returns `Ok(())` only when the audit
has no defects.

Preview narrowing is used when required coverage is absent:

- `missing_provisioning_class_coverage`
- `missing_impact_preview_coverage`
- `rollout_ring_truth_missing`

Beta narrowing is used for incomplete but non-destructive truth:

- `overview_truth_missing`
- `provider_card_incomplete`
- `failure_kind_not_specific`
- `seat_lifecycle_truth_missing`
- `impact_preview_truth_missing`
- `boundary_visibility_incomplete`

Withdrawal is immediate when local-safety or raw-material guardrails are broken:

- `raw_private_material_exposed`
- `local_safety_guarantee_missing`

## Support And Export Projection

`OrganizationAdminTruthSupportExport` carries:

- org/tenant ref
- rollout ring refs
- provisioning source tokens
- seat lifecycle row refs
- local-safety guarantee notes
- the embedded canonical page
- typed narrow reasons and defect counts

This makes support and export packets explain the same tenant boundary, rollout
ring, provisioning source, and deprovision/local-safety truth as the admin UI.

## Canonical Paths

- Runtime owner:
  `aureline_policy::stabilize_organization_admin_provisioning_and_seat_lifecycle_truth`
- Artifact:
  `artifacts/enterprise/m4/stabilize-organization-admin-provisioning-and-seat-lifecycle-truth.md`
- Fixtures:
  `fixtures/enterprise/m4/stabilize-organization-admin-provisioning-and-seat-lifecycle-truth/`
- Schema:
  `schemas/enterprise/seat-lifecycle-and-rollout-ring.schema.json`
