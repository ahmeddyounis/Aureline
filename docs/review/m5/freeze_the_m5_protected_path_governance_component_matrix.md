# M5 Protected-Path Governance Component Matrix

This document is the contract for the frozen M5 matrix that locks eight reusable
governed review, release, and shiproom components. The matrix is the canonical M5
component source for this lane: review workspaces, release-candidate surfaces,
shiprooms, governance/assurance dashboards, owner-coverage panels, Help/About
surfaces, and support exports consume the checked-in packet rather than cloning row
text, minting feature-local governance chrome, or minting provider-specific badges.

- Record kind: `freeze_m5_protected_path_governance_component_matrix`
- Schema: [`schemas/ui/m5-protected-path-governance-component-matrix.schema.json`](../../../schemas/ui/m5-protected-path-governance-component-matrix.schema.json)
- Canonical support export: [`artifacts/release/m5-protected-path-governance-proof/support_export.json`](../../../artifacts/release/m5-protected-path-governance-proof/support_export.json)
- Summary artifact: [`artifacts/release/m5-protected-path-governance-proof/summary.md`](../../../artifacts/release/m5-protected-path-governance-proof/summary.md)
- Design matrix: [`artifacts/design/m5-protected-path-governance-component-matrix.md`](../../../artifacts/design/m5-protected-path-governance-component-matrix.md)
- Fixtures: [`fixtures/ui/m5-protected-path-governance/`](../../../fixtures/ui/m5-protected-path-governance/)
- Producer: `aureline_review::current_stable_m5_governance_component_matrix_export`

## Components

| Component | Maturity | Source contract |
| --- | --- | --- |
| `protected_path_row` | Stable | [`schemas/ui/m5-protected-path-row.schema.json`](../../../schemas/ui/m5-protected-path-row.schema.json) |
| `ownership_card` | Stable | [`schemas/ui/m5-ownership-card.schema.json`](../../../schemas/ui/m5-ownership-card.schema.json) |
| `approver_matrix` | Stable | [`schemas/ui/m5-approver-matrix.schema.json`](../../../schemas/ui/m5-approver-matrix.schema.json) |
| `review_pack_summary` | Stable | [`schemas/ui/m5-review-pack-summary.schema.json`](../../../schemas/ui/m5-review-pack-summary.schema.json) |
| `public_surface_diff_card` | Stable | [`schemas/ui/m5-public-surface-diff-card.schema.json`](../../../schemas/ui/m5-public-surface-diff-card.schema.json) |
| `merge_control_banner` | Stable | [`schemas/ui/m5-merge-control-banner.schema.json`](../../../schemas/ui/m5-merge-control-banner.schema.json) |
| `dri_registry_row` | Beta | [`schemas/ui/m5-dri-registry-row.schema.json`](../../../schemas/ui/m5-dri-registry-row.schema.json) |
| `merge_readiness_strip` | Preview | [`schemas/ui/m5-merge-readiness-strip.schema.json`](../../../schemas/ui/m5-merge-readiness-strip.schema.json) |

Each component row binds a maturity class to the exact advisory-versus-authoritative
and provider-authoritative-versus-local-estimate enforcement distinction, the frozen
governance-state vocabulary, escalation boundary, and backup-coverage fallback it
must preserve, plus its evidence requirement, required evidence packet refs,
downgrade triggers, rollback posture, source contracts, and the consumer surfaces
that must project the component's truth.

## Frozen controlled vocabulary

The one controlled vocabulary every governance component reuses is
`governance_state_vocab`. Its tokens are frozen and reusable across all claimed M5
consumers so no surface mints a drifted label:

- `advisory` — an owner or protection hint that is not enforced.
- `authoritative` — an owner or protection rule that is authoritatively enforced.
- `covered` — owner coverage is present for the guarded path.
- `backup_missing` — owner backup coverage is missing for the guarded path.
- `waived` — a required approval is explicitly waived.
- `expired` — a required approval or review-pack window has expired.
- `stale` — provider-backed truth is stale relative to what it gates.
- `provider_authoritative` — enforcement is authoritative because the provider enforces it.
- `local_estimate` — the value is a local estimate, not provider-confirmed truth.

## Advisory / authoritative and provider / local distinctions

Every component keeps advisory hints separate from authoritative enforcement and
provider-authoritative truth separate from local estimates. The
`enforcement_distinction` field on each row names exactly which values are advisory,
which are authoritatively enforced, which are provider-authoritative, and which are
local estimates. The `trust_review` invariants
`advisory_never_masquerades_as_authoritative` and
`provider_authoritative_versus_local_estimate_distinct` require these separations to
hold for the matrix to validate. An advisory owner hint may never read as
provider-authoritative enforcement, and a local estimate may never read as the
provider's final gate.

## Owner coverage, approver state, and DRI coverage

Missing owner backup coverage (`backup_missing`), expired/waived/stale approver
state, and DRI coverage gaps stay explicit. Guarded merges never hide a missing
backup or an expired approval: the `owner_coverage_backup_missing_explicit`,
`approver_expired_waived_stale_explicit`, and `dri_coverage_gap_explicit`
invariants gate the matrix, and the `backup_coverage_fallback` field on each row
names what the component does when coverage or freshness degrades.

## Public-surface changes

Public-surface changes require a machine-generated diff and migration/evidence
context. The `public_surface_diff_machine_generated_required` and
`migration_evidence_required_for_public_surface_change` invariants require that a
public-surface change never lands without a machine-generated diff and migration
context, and the `public_surface_diff_card` row blocks its claim when the diff is
unavailable rather than presenting the change as a safe no-op.

## Merge-control blockers

Merge-control blockers are named individually, never collapsed into a generic
warning pill. The `merge_control_blocker_never_generic` invariant gates the matrix,
and the `merge_control_banner` row names each blocker (missing owner backup, expired
approval, stale review pack, unreviewed public surface) as a distinct reason.

## Proof freshness and downgrade

Each component carries downgrade triggers. Stale proof or degraded upstream evidence
narrows the claim rather than hiding the component
(`downgrade_narrows_instead_of_hides`), and stale or underqualified rows block
promotion (`stale_or_underqualified_blocks_promotion`). The `proof_freshness` block
records the SLO in hours, the last refresh timestamp, and `auto_narrow_on_stale`.

## Regenerating the artifacts

The checked support export, summary, and narrowed fixtures are emitted from the seed
packet by the gated generator test:

```
GEN_GOVERNANCE_COMPONENT_MATRIX_ARTIFACTS=1 cargo test -p aureline-review \
  freeze_the_m5_protected_path_governance_component_matrix::tests::gen_governance_component_matrix_artifacts \
  -- --exact --ignored
```

Then run `cargo test -p aureline-review --lib freeze_the_m5_protected_path_governance_component_matrix`
to confirm the checked export and fixtures validate and match the seed.
