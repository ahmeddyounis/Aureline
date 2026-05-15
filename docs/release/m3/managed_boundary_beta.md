# Managed-boundary and offboarding beta manifest

Status: seeded
As-of: 2026-05-15
Owner: @ahmeddyounis

This document is the reviewer-facing entrypoint for the M3 beta
managed-boundary and offboarding manifest. The canonical
machine-readable source lives at
`artifacts/milestones/m3/boundary_manifest_beta.yaml`; this page
narrates the row vocabulary, org-switch behavior, grace-windows,
seat/quota state, and offboarding/export semantics so docs/help,
admin, CLI/headless, support-export, and release-evidence surfaces
do not invent parallel labels.

## Why this manifest exists

Beta admission widens managed and paid surfaces. Without one source
of truth for what is local-only, what is mirrored, what is
self-hosted, what is managed, and what is paid/seat-bound, those
surfaces drift across docs, Help/About, support exports, and
release packets. The boundary manifest names every claimed beta
managed or paid row, the org-switch behavior it carries, the grace
window it consumes on entitlement loss, the seat/quota state it
observes, and the offboarding phases and export packets that apply
when managed access ends.

No managed or paid beta row widens without an entry in the manifest
and a linked beta-era evidence packet in the same change set. The
validator (`ci/check_m3_boundary_manifest_beta.py`) fails closed
when a row is missing required vocabulary coverage or beta evidence.

## Canonical artifacts

- Machine source: `artifacts/milestones/m3/boundary_manifest_beta.yaml`
- Schema: `schemas/governance/boundary_manifest_beta.schema.json`
- Validator: `ci/check_m3_boundary_manifest_beta.py`
- Validation capture: `artifacts/milestones/m3/captures/boundary_manifest_beta_validation_capture.json`
- Continuing alpha contract: `artifacts/governance/boundary_manifest_alpha.yaml`
- Anti-lock-in matrix: `artifacts/governance/open_paid_boundary_rows.yaml`
- Usage export and offboarding contract: `docs/governance/usage_export_and_offboarding_contract.md`

## Row vocabulary

The manifest uses one boundary-class vocabulary across every row:

| Class | Meaning |
|---|---|
| `local_only` | Works with no account, no managed control plane, and no seat. Org-switch and seat/quota state do not gate the row. |
| `mirrored` | Reachable via a signed mirror or offline bundle when the live service is denied. Cached metadata may persist for a declared short window. |
| `self_hosted` | Hosted-on-customer-infra alternative to a managed service. Standards-based identity (OIDC) and signed policy bundles substitute for vendor hosting. |
| `managed` | Vendor-hosted convenience layered on a self-hostable protocol or workflow. Loss of the managed surface narrows to the self-hosted or local fallback. |
| `paid_seat_bound` | Managed surface gated by a seat or paid entitlement. Seat revocation denies the managed surface but never erodes the local floor. |
| `explicitly_out_of_scope` | Reserved row preventing implicit beta drift. |

Every row also declares `deployment_profiles` from the canonical
profile vocabulary (`individual_local`, `self_hosted`,
`enterprise_online`, `air_gapped`, `managed_cloud`) and a
`local_core_continuity` statement that names what continues to work
locally when every managed/paid dependency is absent.

## Org-switch behavior

Each managed or paid row declares one org-switch behavior class:

- `preserves_local_state` — switching organizations leaves local
  buffers, Git state, and on-disk profile files untouched. Used by
  the local-only floor.
- `scopes_to_new_org` — managed projections rebind to the new org.
  Cached state from the prior org is invalidated and re-resolved
  rather than silently re-attributed.
- `denies_until_resolved` — managed action denies until the new org
  attaches the required seat, entitlement, or admin handoff. The
  local floor remains usable.
- `requires_admin_handoff` — the new org's signed identity envelope
  and policy bundle are required before the managed surface
  unblocks.
- `not_applicable` — used only by local-only and reserved rows.

Org-switch behavior MUST never widen local exposure: a switch may
narrow what is reachable but MUST NOT silently grant the new org
access to local files, unsaved buffers, or already-exported
packets.

## Grace windows

When entitlement is lost, each row declares one grace-window class:

- `short_lived` — a brief window (typically `P3D`–`P14D`) in which
  audit and export remain accessible while new managed writes deny.
- `policy_pinned` — last-known signed policy or profile remains in
  effect for a longer window (up to `P30D`) until refresh or
  expiry.
- `audit_only` — audit and entitlement snapshot remain accessible
  for the window; mutation denies immediately.
- `denied_for_beta` — no grace window is claimed for beta; the row
  denies immediately on entitlement loss.
- `not_applicable` — used by local-only and reserved rows.

Grace windows are declarative: they do not promise feature parity
during the window, only the named audit/export posture.

## Seat and quota state

Paid and managed rows declare one current `quota_state` and the
full set of `states_observed`:

- `seat_active` / `seat_unassigned` / `seat_revoked` — paid seat
  posture for the row.
- `quota_within_window` / `quota_grace` / `quota_exhausted` —
  per-seat or service-wide quota posture.
- `not_applicable` — used by local-only and unmetered rows.

Seat or quota loss narrows the managed surface only. The
`local_core_continuity` clause restates the local floor that quota
state MUST NOT erode.

## Offboarding and export semantics

Each managed or paid row declares the offboarding phases it
observes and the export packet class it produces:

- Phases: `announce` → `freeze_writes` → `export_available` →
  `managed_access_end` → `destruction_receipt_issued`.
- Export packet classes: `local_support_bundle` (no managed
  upload required), `managed_usage_export` (hosted usage records),
  `entitlement_snapshot` (identity, policy, and entitlement
  state), `destruction_receipt` (cryptographic receipt issued at
  managed-access end).

Rows that produce a destruction receipt set
`destruction_receipt_required: true`. Local-only and reserved
rows declare `phases_observed: [not_applicable]`.

## Required coverage and validation

The manifest MUST seed at least one row for each of
`local_only`, `mirrored`, `self_hosted`, `managed`, and
`paid_seat_bound`. The validator fails closed when a required
class has no row, when a managed or paid row is missing an
`absence_narrows_to` clause, or when a managed/paid row's
`linked_evidence_class` is `alpha_seed` rather than a beta-era
evidence class.

Run the validator:

```
python3 ci/check_m3_boundary_manifest_beta.py --repo-root .
```

The validator writes the capture under
`artifacts/milestones/m3/captures/boundary_manifest_beta_validation_capture.json`
and exits non-zero on any error.

## Failure drill

To confirm the guardrail is live:

1. Temporarily change one managed row's `linked_evidence_class` to
   `alpha_seed`.
2. Re-run the validator; it MUST fail with an actionable error
   naming the offending row and required class.
3. Restore the value and re-run; it MUST pass.

## Change control

Adding a new managed, mirrored, or paid_seat_bound beta surface
MUST:

- add a row in `artifacts/milestones/m3/boundary_manifest_beta.yaml`,
- link at least one beta-era evidence packet,
- update the claimed-surface register and known-limits surfaces in
  the same change set,
- pass the validator with the regenerated capture committed.

Repurposing an existing boundary, org-switch, grace-window,
quota-state, offboarding-phase, or export-packet vocabulary value
is breaking and opens a decision row. Adding a new value to any
closed vocabulary is additive-minor and bumps `schema_version`.
