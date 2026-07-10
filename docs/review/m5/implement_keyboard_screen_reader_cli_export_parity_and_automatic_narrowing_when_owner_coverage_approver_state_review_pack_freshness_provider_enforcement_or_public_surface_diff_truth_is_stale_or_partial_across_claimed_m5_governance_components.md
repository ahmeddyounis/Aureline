# M5 Protected-Path Governance Component Accessibility, Headless, and Export Parity

Task: **M05-1050** · Batch: **B124** · Crate: `aureline-review`

This lane is the accessibility / headless / export capstone over the eight shared M5
protected-path governance components frozen in the
[protected-path governance component matrix](../../../schemas/ui/m5-protected-path-governance-component-matrix.schema.json)
and implemented by the protected-path / ownership, approver-matrix / review-pack,
public-surface-diff / merge-control, and DRI-registry / merge-readiness lanes. Where the
[shared-consumer lane](../../../schemas/ui/m5-protected-path-governance-component-consumer.schema.json)
proves owner / approver / public-surface language stays aligned across desktop surfaces,
this lane proves the harder claim: that protection reason, owner source,
advisory-versus-authoritative enforcement, approver state, review-pack freshness, and
public-surface change class are exposed just as honestly in assistive, headless, and
exported forms as they are on the desktop.

## The eight components

`protected_path_row`, `ownership_card`, `approver_matrix`, `review_pack_summary`,
`public_surface_diff_card`, `merge_control_banner`, `dri_registry_row`, and
`merge_readiness_strip`. Each row points at the frozen matrix contract and at the
canonical implement-lane control schema for its component family.

## Honesty axis 1 — parity across forms

Every claimed component exposes a keyboard label, a screen-reader label, a CLI enum token,
an export enum token, and a human-readable explanation field, and renders on the desktop,
the headless CLI, and the support export alike. No component may be pointer-only,
export-opaque, semantically stronger on the desktop than in CLI or export, or collapsed to
a vague `governed` label that drops the owner / approver / public-surface semantics the GUI
shows. This is the first acceptance criterion: accessibility and export surfaces preserve
the same owner / approver / public-surface semantics as the GUI.

## Honesty axis 2 — automatic narrowing under partial governance evidence

Each component carries a governed-authority claim drawn from `GovernanceComponentClaimTier`.
When a weakening condition holds, the claim narrows to the ceiling that condition permits,
discloses the narrowing through an explicit trigger and next action, and keeps the explicit
owner / approver / public-surface semantics rather than a clean governed pass. This is the
second acceptance criterion.

| Condition | Permitted ceiling | Required note (beyond the always-kept governed-semantics note) |
| --- | --- | --- |
| `governance_truth_trusted` | `full_governed_authority` | — (no narrowing) |
| `provider_enforcement_stale_or_partial` | `advisory_enforcement_only` | advisory-not-authoritative enforcement note |
| `owner_coverage_partial` | `owner_backup_coverage_missing` | missing-backup-coverage note |
| `approver_state_stale_or_partial` | `approver_state_narrowed` | waived/expired approver-state note |
| `review_pack_freshness_stale` | `review_pack_stale_disclosed` | — (governed-semantics note only) |
| `public_surface_diff_truth_partial` | `public_surface_evidence_withheld` | missing public-surface diff / migration note |

The required notes encode the spec guardrails directly: an advisory owner hint is never
promoted to provider-authoritative enforcement, a guarded merge never hides missing backup
coverage or expired approver state, and a public-surface change never reads clean without
its machine-generated diff and migration evidence.

## Boundary and evidence

- Boundary schema: [`m5-protected-path-governance-component-accessibility-parity.schema.json`](../../../schemas/ui/m5-protected-path-governance-component-accessibility-parity.schema.json)
- Support export: [`artifacts/release/m5-protected-path-governance-accessibility-proof/support_export.json`](../../../artifacts/release/m5-protected-path-governance-accessibility-proof/support_export.json)
- Summary: [`artifacts/release/m5-protected-path-governance-accessibility-proof/summary.md`](../../../artifacts/release/m5-protected-path-governance-accessibility-proof/summary.md)
- Narrowed fixtures: [`fixtures/ui/m5-protected-path-governance-component-accessibility-parity/`](../../../fixtures/ui/m5-protected-path-governance-component-accessibility-parity/)

The packet references upstream component and consumer contracts by id rather than embedding
their content; raw provider responses, credentials, and CODEOWNERS payloads stay outside the
support boundary. Regenerate the checked-in artifacts with
`GEN_GOVERNANCE_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-review --lib regenerate_governance_component_accessibility_artifacts`
and review the diff.
