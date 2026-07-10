# Shared Protected-Path Governance Component Consumers: Owner, Approver, and Public-Surface Parity

This is the consumer-adoption lane for the eight reusable M5 protected-path
governance components frozen in
`freeze_the_m5_protected_path_governance_component_matrix` and implemented by the
protected-path-row / ownership-card, approver-matrix / review-pack-summary,
public-surface-diff-card / merge-control-banner, and DRI-registry-row /
merge-readiness-strip lanes. It binds each shared component to the seven consumer
surfaces that render it and proves — by fixtures, not screenshots — that the same
governed change presents the same owner-coverage, approver-state,
public-surface-impact, and merge-blocker language wherever it appears.

- Boundary schema: [`schemas/ui/m5-protected-path-governance-component-consumer.schema.json`](../../../schemas/ui/m5-protected-path-governance-component-consumer.schema.json)
- Support export: [`artifacts/release/m5-protected-path-governance-consumers-proof/support_export.json`](../../../artifacts/release/m5-protected-path-governance-consumers-proof/support_export.json)
- Protected fixtures: [`fixtures/ui/m5-protected-path-governance-component-consumers/`](../../../fixtures/ui/m5-protected-path-governance-component-consumers/)

## Consumers

| Consumer | Surface |
| --- | --- |
| `review_workspace` | Review workspace list / detail |
| `merge_queue` | Merge queue |
| `release_center` | Release-center packet |
| `help_surface` | Help / About surface |
| `support_packet` | Support packet |
| `shiproom` | Shiproom / escalation summary |
| `cli_export` | CLI / headless export payload |

## Parity facets

For a given governed change, every consumer surface must present identical values
for all four parity facets:

- `owner_coverage_label` — who owns the change (owner source and coverage).
- `approver_state_label` — the required-approver state language.
- `public_surface_impact_label` — whether public-surface impact exists and its class.
- `merge_blocker_label` — what is blocked (the merge-control blocker language).

A surface may narrow *how much* it renders, but it may never reword any of these
values per surface. Narrowing never touches the parity facets; it is disclosed
additively through an explicit narrow banner and, where required, an
enforcement-authority or evidence-continuity note.

## Evidence-driven narrowing

Every binding carries a governance evidence / enforcement state. Consumers must
degrade the same way when evidence or enforcement state is stale:

| Evidence state | Projection mode | Required disclosure |
| --- | --- | --- |
| `provider_authoritative_fresh` | `full_parity` | none |
| `enforcement_advisory_or_local_estimate` | `enforcement_narrowed` | narrow banner + enforcement-authority note |
| `owner_backup_coverage_missing` | `coverage_narrowed` | narrow banner + evidence-continuity note |
| `approver_state_expired_or_waived` | `approval_narrowed` | narrow banner + evidence-continuity note |
| `public_surface_evidence_missing` | `public_surface_narrowed` | narrow banner + evidence-continuity note |
| `proof_stale_relative_to_change` | `stale_narrowed` | narrow banner + enforcement-authority note |

The frozen governance-state vocabulary (`advisory`, `authoritative`, `covered`,
`backup_missing`, `waived`, `expired`, `stale`, `provider_authoritative`,
`local_estimate`) is reused directly from the matrix, so a downgrade trigger, an
owner-coverage state, and an enforcement-authority distinction read the same on
every surface.

## Guardrails (each must be false per binding)

- `advisory_owner_reads_as_provider_authoritative` — an advisory owner hint never
  reads as provider-authoritative enforcement.
- `guarded_merge_hides_missing_backup_coverage` — a guarded merge never hides
  missing backup coverage.
- `guarded_merge_hides_expired_approver_state` — a guarded merge never hides an
  expired approver state.
- `public_surface_change_hides_diff_or_migration_evidence` — a public-surface change
  never lands without a machine-generated diff and its migration / evidence context.
- `rewords_governance_labels_per_surface` — labels are never reworded per surface.

## Reuse and canonical contracts

Component reuse is proven, not inferred: every one of the eight shared components is
adopted by at least two distinct consumers, and Help, support, and CLI/export
bindings point at the canonical component contracts (the component-controls schema
of its implement lane plus the frozen component matrix) by id. The eight components
pair onto the four implement-lane controls contracts:

| Components | Controls contract |
| --- | --- |
| `protected_path_row`, `ownership_card` | `schemas/ui/m5-protected-path-ownership-controls.schema.json` |
| `approver_matrix`, `review_pack_summary` | `schemas/ui/m5-approver-review-pack-controls.schema.json` |
| `public_surface_diff_card`, `merge_control_banner` | `schemas/ui/m5-public-surface-diff-merge-control-controls.schema.json` |
| `dri_registry_row`, `merge_readiness_strip` | `schemas/ui/m5-dri-registry-merge-readiness-controls.schema.json` |

## Regenerating artifacts

Regenerate the checked-in support export, summary, and fixtures after a contract
change:

```sh
GEN_GOVERNANCE_COMPONENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-review --lib \
  regenerate_governance_component_consumer_artifacts
```
