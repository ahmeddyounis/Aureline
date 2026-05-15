# M3 → stable planning handoff checklist

This checklist is the canonical input for M4 stable planning. Stable
planning should consume this packet rather than walk milestone notes or
recompose lists locally. The checklist is paired with the M3 cutline
packet (`docs/milestones/m3/cutline_packet.md`) and the unlock map
(`artifacts/milestones/m3/unlock_map.yaml`); the three must agree, and
any widening of beta scope must update them in the same change set.

The handoff is allowed to start once all checklist sections are signed
off in the cited captures and the cutline validator passes.

- Cutline packet: `docs/milestones/m3/cutline_packet.md`
- Unlock map: `artifacts/milestones/m3/unlock_map.yaml`
- Claimed-surface register: `artifacts/milestones/m3/claimed_surface_register.json`
- Cohort guardrails: `artifacts/milestones/m3/cohort_guardrails.yaml`
- Dependency graph: `artifacts/milestones/m3/dependency_graph.mmd`
- Cutline validator: `ci/check_m3_cutline_packet.py`
- Latest validation capture: `artifacts/milestones/m3/captures/cutline_packet_validation_capture.json`

## Section 1 — Scope is frozen and inspectable

- [ ] Beta admission matrix names every claimed surface, cohort, and
      archetype row with one canonical id
      (`docs/milestones/m3/beta_admission_matrix.md`).
- [ ] Claimed-surface register lists held / out-of-scope rows with
      widening rules
      (`artifacts/milestones/m3/claimed_surface_register.json`).
- [ ] Unlock map mirrors the held / out-of-scope rows as beta-only
      surfaces (`artifacts/milestones/m3/unlock_map.yaml`).
- [ ] Beta admission validator passes
      (`python3 ci/check_beta_admission.py --repo-root .`).
- [ ] Cutline validator passes
      (`python3 ci/check_m3_cutline_packet.py --repo-root .`).

## Section 2 — Non-descopable M3 truths are green

For each row, attach the evidence packet ref and freshness date in the
cutline validator capture.

- [ ] `m3_cutline:rollback_proven_in_partner_orgs` — propagation refs:
      `docs/release/update_and_rollback_contract.md`,
      `docs/release/update_verification_and_rollback_sequence.md`.
- [ ] `m3_cutline:policy_explainability_baseline` — propagation refs:
      `docs/governance/policy_flag_schema_stack.md`.
- [ ] `m3_cutline:extension_isolation_envelope` — propagation refs:
      `docs/extensions/sdk_publication_contract.md`,
      `docs/extensions/extension_lifecycle_and_quarantine_sequence.md`.
- [ ] `m3_cutline:migration_honesty_register` — propagation refs:
      `docs/migration/migration_equivalence_and_parity_scorecard.md`.
- [ ] `m3_cutline:supportability_baseline` — propagation refs:
      `docs/governance/usage_export_and_offboarding_contract.md`.
- [ ] `m3_cutline:debug_test_task_truth` — propagation refs:
      `docs/release/certified_archetype_report_template.md`.
- [ ] `m3_cutline:certified_archetype_publication` — propagation refs:
      `docs/release/certified_archetype_report_template.md`,
      `docs/release/compatibility_report_template.md`.

## Section 3 — Downstream M4 unlocks have a green path

- [ ] `m4_unlock:stable_api_freeze` cites
      `docs/governance/public_interface_versioning_policy.md`,
      `docs/governance/interface_freeze_guide.md`,
      `docs/governance/interface_freeze_matrix.md`,
      `docs/governance/frozen_surface_ci_policy.md`.
- [ ] `m4_unlock:release_center_rehearsal` cites
      `docs/release/release_center_object_model_contract.md`,
      `docs/release/release_center_provenance_linkage.md`,
      `docs/release/shiproom_runbook.md`,
      `docs/release/shiproom_dashboard_contract.md`,
      `docs/release/update_and_rollback_contract.md`,
      `docs/release/update_verification_and_rollback_sequence.md`.
- [ ] `m4_unlock:certified_archetype_publication` cites
      `docs/release/certified_archetype_report_template.md`,
      `docs/release/compatibility_report_template.md`,
      `docs/migration/compatibility_scorecard_contract.md`,
      `artifacts/compat/reference_workspace_rows.yaml`,
      `artifacts/compat/archetype_rubric.yaml`.

## Section 4 — Beta-only surfaces remain beta-only at handoff

- [ ] Managed-cloud daily-driver parity is documented as a scoped
      handoff surface; widening requires release-council review and a
      known-limit note.
- [ ] `.NET service or app` remains held for M4 or later; widening
      requires the change-control workflow.
- [ ] Notebook-first data workflow remains held for M4 or later;
      widening requires the change-control workflow.
- [ ] Long-tail framework breadth beyond certified archetypes remains
      out of scope.

## Section 5 — Same-change controls

- [ ] Any new beta row, archetype, ecosystem lane, or managed claim
      added since the prior handoff updated the canonical matrix,
      cohort guardrails, claim-surface register, unlock map, and
      cutline packet in the same change set.
- [ ] Captures for both validators are refreshed and checked in.
- [ ] Stable planning entry brief cites this checklist by path instead
      of restating the scope.

## How to refresh

1. Run both validators:
   - `python3 ci/check_beta_admission.py --repo-root .`
   - `python3 ci/check_m3_cutline_packet.py --repo-root .`
2. Refresh captures in
   `artifacts/milestones/m3/captures/` in the same change set.
3. Update the cutline packet and unlock map first, then refresh this
   checklist so M4 planning consumes one canonical source.
