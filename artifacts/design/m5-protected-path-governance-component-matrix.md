# M5 Protected-Path Governance Component Matrix (Design)

This is the design-side component inventory for the eight reusable M5 governance
components frozen by
`aureline_review::current_stable_m5_governance_component_matrix_export`. Every
claimed M5 governed review, release, and shiproom surface consumes this one shared
component family instead of feature-local governance chrome, private row text, or
provider-specific badges.

| Component | Maturity | Enforcement distinction | Governance-state vocabulary | Escalation boundary | Backup-coverage fallback | Source contract |
| --- | --- | --- | --- | --- | --- | --- |
| `protected_path_row` | Stable | Provider branch protection labeled `provider_authoritative`; local match `authoritative` when enforced, `advisory` when a hint; match count `local_estimate` | `advisory`, `authoritative`, `provider_authoritative`, `local_estimate`, `stale` | Requesting a protected-path exception is an explicit handoff with labeled return path | Last-known protection reason labeled stale; local matching continues without asserting the provider gate | `schemas/ui/m5-protected-path-row.schema.json` |
| `ownership_card` | Stable | Provider-resolved owners `provider_authoritative`; manifest owners `authoritative`/`advisory`; advisory hint never masquerades as provider | `advisory`, `authoritative`, `covered`, `backup_missing`, `provider_authoritative` | Requesting an owner or backup assignment is an explicit handoff | Missing backup labeled `backup_missing`; path never shown as covered | `schemas/ui/m5-ownership-card.schema.json` |
| `approver_matrix` | Stable | Provider-recomputed approval `provider_authoritative`; local remaining-approval prediction `local_estimate` | `authoritative`, `waived`, `expired`, `stale`, `provider_authoritative`, `local_estimate` | Re-requesting or waiving approval is an explicit handoff | Stale rows labeled stale; expired approvals marked expired; local review continues | `schemas/ui/m5-approver-matrix.schema.json` |
| `review_pack_summary` | Stable | Provider-confirmed results `provider_authoritative`; local parity `local_estimate` | `authoritative`, `stale`, `expired`, `provider_authoritative`, `local_estimate` | Re-running the pack on the provider is an explicit handoff | Stale pack labeled stale; last-known parity shown; local re-run offered | `schemas/ui/m5-review-pack-summary.schema.json` |
| `public_surface_diff_card` | Stable | Provider-published surface `provider_authoritative`; local diff `local_estimate` until confirmed | `authoritative`, `stale`, `provider_authoritative`, `local_estimate` | Publishing or migrating a surface is an explicit handoff | Missing diff blocks the claim; never shown as a safe no-op | `schemas/ui/m5-public-surface-diff-card.schema.json` |
| `merge_control_banner` | Stable | Provider-enforced blockers `provider_authoritative`; predicted blockers `local_estimate` | `advisory`, `authoritative`, `backup_missing`, `expired`, `stale`, `provider_authoritative`, `local_estimate` | Overriding or waiving a blocker is an explicit handoff | Stale blockers labeled stale; each blocker named; never flattened | `schemas/ui/m5-merge-control-banner.schema.json` |
| `dri_registry_row` | Beta | Registry DRI `authoritative`; inferred DRI `advisory`/`local_estimate` | `advisory`, `authoritative`, `covered`, `backup_missing`, `local_estimate` | Assigning or reassigning a DRI is an explicit handoff | Coverage gap labeled `backup_missing`; inferred DRI shown advisory | `schemas/ui/m5-dri-registry-row.schema.json` |
| `merge_readiness_strip` | Preview | Provider readiness gates `provider_authoritative`; local readiness `local_estimate` | `authoritative`, `backup_missing`, `stale`, `provider_authoritative`, `local_estimate` | Resolving a readiness blocker is an explicit handoff | Stale gates labeled stale; local readiness continues without asserting approval | `schemas/ui/m5-merge-readiness-strip.schema.json` |

## Hard invariants

The `trust_review` block encodes the guardrails as invariants that must all hold:
advisory owner hints never masquerade as authoritative enforcement,
provider-authoritative enforcement is never flattened into a local estimate,
missing owner backup coverage is named, expired/waived/stale approver state stays
explicit, review-pack freshness and parity stay explicit, public-surface changes
require a machine-generated diff and migration/evidence context, the protection
reason is always explicit, DRI coverage gaps stay explicit, and merge-control
blockers are named rather than shown as a generic warning. Escalation handoff stays
explicit with a safe return path. Downgrade narrows the claim rather than hiding the
component, and stale or underqualified rows block promotion.

## Consumer projection

The `consumer_projection` block records that each component projects its truth on the
surfaces that consume it — protected-path row (reason + enforcement authority),
ownership card (owner source + coverage), approver matrix (required approvers +
state), review-pack summary (freshness + parity), public-surface diff card (change
class + machine-generated diff), merge-control banner (blockers, not generic), DRI
registry row (DRI + coverage), merge-readiness strip (blocking + ownership) — plus
CLI/headless and support export.

## Narrowed fixtures

Two checked fixtures exercise auto-narrowing while keeping every component present
and every invariant satisfied:

- `ownership_card_backup_missing_narrowed.json` — the ownership card narrows to Beta
  because owner backup coverage is missing; the path is labeled `backup_missing`.
- `merge_control_banner_held.json` — the merge-control banner is held pending
  upstream graduation while the other components stay at their baseline maturities.
