# Effective Profile and Save Participant Governance Artifact

This artifact records the stable quality-lane governance proof now represented
by `QualityReleaseDebtPacket`.

Evidence:

- Schema: `/schemas/quality/effective-profile-and-save-participant-governance.schema.json`
- Fixture: `/fixtures/quality/profile_and_suppression_governance/release_debt_packet.yaml`
- Runtime tests: `crates/aureline-runtime/tests/quality_governance.rs`

Covered behavior:

- Policy-locked effective profile emits winning source and source chain truth.
- Save participant ordering is explicit and reconstructable from session
  proposals.
- Safe local formatting can auto-apply on save.
- Workspace-wide fix-all requires preview before apply.
- Policy-scoped suppression proposal is blocked pending policy review.
- Suppression and baseline debt are counted separately in release-visible debt
  rows.
