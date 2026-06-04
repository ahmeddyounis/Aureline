# Effective Profile and Save Participant Governance

Stable quality lanes must emit one reconstructable packet for the effective
profile, save participant order, fix-safety posture, and release-visible debt
state. The runtime source of truth is `aureline-runtime::quality`.

Machine-readable companions:

- [`/schemas/quality/effective-profile-and-save-participant-governance.schema.json`](../../schemas/quality/effective-profile-and-save-participant-governance.schema.json)
- [`/fixtures/quality/profile_and_suppression_governance/release_debt_packet.yaml`](../../fixtures/quality/profile_and_suppression_governance/release_debt_packet.yaml)

## Stable Requirements

- Effective profile resolution preserves every candidate source and marks the
  winning source, imported read-only mappings, unmapped keys, and policy
  overrides.
- Save participants are ordered by declared phase and proposal order, then
  exported with action class, mutation scope, fix-safety class, preview posture,
  apply posture, checkpoint, preview, and revert refs.
- Hot-save auto-apply is allowed only for `safe_local_text_edit` participants
  that do not require preview and are not blocked.
- Whole-file, generated, multi-file, workspace-wide, unknown, and policy-scoped
  mutations require preview, typed review, or policy blocking before mutation.
- Suppression, baseline, and waiver states remain distinct in release-visible
  debt rows. A suppression is not a baseline, and a waiver remains visible as
  debt.
- Support exports and release packets cite opaque lineage refs instead of raw
  source, raw paths, raw logs, raw tool arguments, provider payloads, or secrets.

## Runtime Records

`QualityReleaseDebtPacket` links:

- `effective_profile_ref` and `winning_source_ref`
- `quality_session_ref`
- ordered `QualitySaveParticipantRow` entries
- `QualityReleaseDebtRow` entries for suppressed, baselined, waived, new,
  resolved, and unmapped debt states
- `QualityReleaseDebtCounts`
- support and release evidence refs

The packet sets `reconstructable_without_local_editor_state` to `true` only
because every profile, participant, and debt row uses exported refs and stable
tokens.
