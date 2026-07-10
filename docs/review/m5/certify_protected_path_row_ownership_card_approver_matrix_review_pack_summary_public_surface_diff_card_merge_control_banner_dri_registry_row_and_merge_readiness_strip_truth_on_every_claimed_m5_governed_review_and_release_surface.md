# M5 Protected-Path Governance-Component Surface Certification

Closing certification capstone (M05-1051, batch B124) over the eight shared
protected-path governance components frozen in
`freeze_the_m5_protected_path_governance_component_matrix` — protected-path-row,
ownership-card, approver-matrix, review-pack-summary, public-surface-diff-card,
merge-control-banner, dri-registry-row, and merge-readiness-strip.

Where the implement lanes ship the components, the consumer lane binds them across
desktop surfaces, and the accessibility lane proves keyboard / screen-reader / CLI /
export parity, this lane certifies the release claim: **on every claimed M5 governed
review and release surface, the same reusable governance component truth is presented
with no hidden enforcement-authority, owner-coverage, approver-state,
review-pack-freshness, or public-surface-diff drift — and where a surface cannot
present the full claim it is explicitly narrowed with current evidence, never silently
dropped.**

- Boundary schema:
  `schemas/ui/m5-protected-path-governance-component-certification.schema.json`
- Record kind: `m5_protected_path_governance_component_surface_certification_truth`
- Checked support export:
  `artifacts/review/m5/certify_protected_path_row_ownership_card_approver_matrix_review_pack_summary_public_surface_diff_card_merge_control_banner_dri_registry_row_and_merge_readiness_strip_truth_on_every_claimed_m5_governed_review_and_release_surface/support_export.json`
- Release proof packet:
  `artifacts/release/m5-protected-path-governance-certification-proof/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- Protected fixtures:
  `fixtures/ui/m5-protected-path-governance-component-certification/`

## Certified surfaces

Eight claimed governed review / release surfaces are certified: `review_workspace_surface`,
`merge_queue_surface`, `release_center_surface`, `help_governance_surface`,
`support_export`, `shiproom_surface`, `exported_governance_packet`, and `cli_headless`.
The canonical seed certifies four green and narrows four with current evidence, and every
one of the eight shared components appears on at least one surface.

## Certification axes

Each surface row scores six axes (`GovernanceComponentCertificationAxis`):

1. `visual` — always-on: visual rendering carries the controlled component truth.
2. `keyboard` — always-on: keyboard reach and operation carry it.
3. `screen_reader` — always-on: screen-reader labelling carries it.
4. `cli_export` — always-on: CLI and export forms carry it.
5. `degraded_state` — narrows a claim when provider enforcement, owner coverage,
   approver state, review-pack freshness, or public-surface diff truth weakens.
6. `enforcement_ownership_provenance` — the certification-specific separation axis:
   keeps the advisory-versus-authoritative enforcement, owner-source, and
   public-surface change-class distinctions explicit so a certified surface never
   implies its enforcement is provider-authoritative, its owner coverage complete, or
   its public-surface change clean without evidence.

## Claim tiers and narrowing

A surface claims a governed-authority tier drawn from `GovernanceComponentClaimTier`
(`full_governed_authority` down through `advisory_enforcement_only`,
`owner_backup_coverage_missing`, `approver_state_narrowed`, `review_pack_stale_disclosed`,
`public_surface_evidence_withheld`). The certification may only ever narrow that claim:
a certified claim that exceeds the claimed one is a validation failure.

A surface earns `certified_parity` (green) only when its certified claim equals its
claimed claim, no axis narrows, and component truth is preserved. It narrows to
`narrowed_parity` (yellow) the moment an axis narrows or the certified claim drops below
the claimed one, and it fails to `parity_blocked` (red) whenever the protection reason,
owner source, advisory-versus-authoritative enforcement, approver state, review-pack
freshness, public-surface change class, merge-control blockers, DRI coverage, or
exportable escalation continuity is flattened out of the export. **That last rule is the
delta of this capstone: certification may narrow a claim but may never drop the
component's meaning.**

## Automatic narrowing

`GovernanceComponentCertificationPacket::apply_downgrade_automation` consumes per-surface
observations. A surface reported with a flattened component truth blocks (red); a
still-green surface whose governance truth (provider enforcement, owner coverage,
approver state, review-pack freshness, and public-surface diff) went stale narrows its
full-governed-authority claim to a disclosed `advisory_enforcement_only` ceiling, marks
the `enforcement_ownership_provenance` axis narrowed, and discloses the
`provider_enforcement_advisory_or_stale` trigger. The summary is recomputed so shiproom,
support, and docs can trust the aggregate counts.

## Guardrails

- An advisory owner hint never reads as provider-authoritative enforcement.
- A guarded merge never hides missing backup coverage or expired approver state.
- A public-surface change never lands without its machine-generated diff and
  migration / evidence context.
- Raw provider responses, credentials, and CODEOWNERS payloads stay outside the support
  boundary; the export is metadata-safe.

## Regeneration

The checked-in export, summary, release proof packet, and fixtures are regenerated by the
gated test:

```
GEN_GOVERNANCE_COMPONENT_CERTIFICATION_ARTIFACTS=1 \
  cargo test -p aureline-review --lib regenerate_governance_component_certification_artifacts
```
