---
schema_version: 1
scorecard_kind: cohort_scorecard
scorecard_id: cohort_scorecard:design_partner
cohort_id: cohort:design_partner_managed_pilot
cohort_sub_focus: design_partner
title: Design-partner cohort scorecard
owner: "@ahmeddyounis"
evidence_owner: "@ahmeddyounis"
as_of: "2026-05-15"
evidence_date: "2026-05-15"
review_window_days: 21
freshness_state: current
declared_support_class: supported
display_lifecycle_label: beta
primary_surface_refs:
  - beta_surface:packaging_update_rollback
  - beta_surface:support_export_diagnostics
  - beta_surface:importer_and_migration
evidence_refs:
  - artifacts/milestones/m2/design_partner_intake_packet.md
  - artifacts/milestones/m2/design_partner_task_pack.md
  - artifacts/feedback/external_alpha_known_limits.md
  - artifacts/compat/reference_workspace_rows.yaml
  - docs/release/release_evidence_packet_template.md
open_waivers: []
downgrade_policy:
  - trigger: support_bundle_redaction_break
    downgrade_to: retest_pending
    propagation_refs:
      - docs/governance/usage_export_and_offboarding_contract.md
      - artifacts/feedback/external_alpha_known_limits.md
  - trigger: rollback_drill_failure
    downgrade_to: retest_pending
    propagation_refs:
      - docs/release/update_and_rollback_contract.md
  - trigger: design_partner_evidence_stale
    downgrade_to: evidence_stale
    propagation_refs:
      - artifacts/feedback/external_alpha_known_limits.md
owner_handoff_path:
  intake_owner: "@ahmeddyounis"
  triage_owner: "@ahmeddyounis"
  release_owner: "@ahmeddyounis"
  escalation_ref: docs/governance/decision_rights_and_signoff_matrix.md
consuming_surfaces:
  - docs/milestones/m3/beta_admission_matrix.md
  - docs/release/release_evidence_packet_template.md
  - artifacts/feedback/external_alpha_known_limits.md
---

# Design-partner cohort scorecard

This scorecard is the reviewer-facing summary of design-partner readiness in
the M3 beta admission lane. Tooling consumes the YAML front matter above and
the derived register at
`artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json`.

## Scope

The design-partner facet of `cohort:design_partner_managed_pilot` covers
named external partners working on packaging, support export, rollback, and
migration evidence on real partner hardware. Managed-pilot coverage lives in
the sibling scorecard at
`artifacts/milestones/m3/cohorts/managed_pilot_scorecard.md`.

## Definition of green

A current scorecard requires that:

- the design-partner intake packet and feedback taxonomy remain authoritative
  for redaction state and routing;
- a reproducible support export passes redaction review on the current beta
  build;
- a rollback drill completes on partner hardware without data loss; and
- no open waiver narrows scope past the declared support class.

The validator at `ci/check_cohort_archetype_scorecards.py` recomputes the
effective support class. Expired evidence or open waivers automatically
move this row to `retest_pending`, `limited`, or `evidence_stale` in the
derived register; the row never silently stays green.

## Downgrade triggers and propagation

| Trigger | Auto-state | Propagation |
|---|---|---|
| Support-export redaction break | `retest_pending` | `docs/governance/usage_export_and_offboarding_contract.md`, `artifacts/feedback/external_alpha_known_limits.md` |
| Rollback drill failure | `retest_pending` | `docs/release/update_and_rollback_contract.md` |
| Evidence past freshness window | `evidence_stale` | `artifacts/feedback/external_alpha_known_limits.md` |

## Owner handoff path

- Intake and waiver entry: `@ahmeddyounis`
- Partner triage and routing: `@ahmeddyounis`
- Release-council escalation: `docs/governance/decision_rights_and_signoff_matrix.md`

## Verification

Run the validator and refresh the capture in the same change set:

```
python3 ci/check_cohort_archetype_scorecards.py --repo-root . \
  --report artifacts/milestones/m3/captures/cohort_archetype_scorecard_validation_capture.json \
  --register artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json
```
