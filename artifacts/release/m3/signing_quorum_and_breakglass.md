# Release signing, quorum, and break-glass

> Generated from `artifacts/release/m3/maintainer_coverage_matrix.json` (`release.beta_durability_packet.m3`). Do not edit by hand; run the durability gate to refresh.

This is the split-authority projection of [`artifacts/governance/signing_quorum.yaml`](../../../artifacts/governance/signing_quorum.yaml). Release, rollback, revocation, and registry-emergency authority cite the action ids in that matrix rather than inventing per-run quorum rules in PR comments or chat logs.

- Governing principle: `no_single_human_release_path`
- Policy narrative: `docs/governance/maintainer_coverage_policy.md`
- Repository posture: single-maintainer backup waiver `artifacts/governance/ownership_matrix.yaml#waivers.single-maintainer-backup` (expires 2026-10-19).

## Signer roster

| Signer | Role | Named humans | Backup state | Scopes |
|---|---|---|---|---|
| release_operator | release_operator | @ahmeddyounis | single_maintainer_waiver | release_signing, preview_or_beta_promotion, rollback |
| security_operator | security_operator | @ahmeddyounis | single_maintainer_waiver | revocation, registry_emergency, channel_freeze |
| evidence_owner_or_auditor | backup_signer_or_auditor | @ahmeddyounis | single_maintainer_waiver | release_evidence_acceptance, signer_roster_change |

## Split-authority actions

| Authority | Signing-quorum action | Quorum profile | Min distinct humans | Author-only forbidden | Break-glass |
|---|---|---|---|---|---|
| release_signing | stable_or_lts_promotion | three_person_stable_promotion | 3 | true | none |
| rollback | channel_freeze_or_resume | two_person_cross_forum_emergency_control | 2 | true | audited_single_responder_containment |
| revocation | revocation_disable_or_kill_switch_publication | two_person_cross_forum_emergency_control | 2 | true | audited_single_responder_containment |
| registry_emergency | emergency_policy_bundle_publish | two_person_cross_forum_emergency_control | 2 | true | audited_single_responder_containment |
| signer_roster_change | signer_roster_or_trust_root_change | three_person_trust_root_change | 3 | true | none |

## Break-glass containment

- Profile: `audited_single_responder_containment`
- Maximum duration (hours): 24
- Retrospective quorum profile: `two_person_cross_forum_emergency_control`
- Forbidden for:
  - `stable_or_lts_promotion`
  - `widening_claim_scope`
  - `permanent_signer_roster_change`
  - `deleting_or_mutating_audit_history`
- Required audit fields:
  - `action_id`
  - `triggered_by`
  - `utc_invoked_at`
  - `affected_scope`
  - `evidence_refs`
  - `missing_approver_class`
  - `temporary_mitigation`
  - `expiry_or_review_deadline`
  - `retrospective_decision_refs`

