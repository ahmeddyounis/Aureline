---
schema_version: 1
scorecard_kind: cohort_scorecard
scorecard_id: cohort_scorecard:managed_pilot
cohort_id: cohort:design_partner_managed_pilot
cohort_sub_focus: managed_pilot
title: Managed-pilot cohort scorecard
owner: "@ahmeddyounis"
evidence_owner: "@ahmeddyounis"
as_of: "2026-05-15"
evidence_date: "2026-05-15"
review_window_days: 21
freshness_state: current
declared_support_class: supported
display_lifecycle_label: beta
primary_surface_refs:
  - beta_surface:policy_proxy_transport
  - beta_surface:packaging_update_rollback
  - beta_surface:support_export_diagnostics
evidence_refs:
  - artifacts/deployment/locality_matrix.yaml
  - docs/release/release_evidence_packet_template.md
  - artifacts/feedback/external_alpha_known_limits.md
  - artifacts/compat/reference_workspace_rows.yaml
open_waivers: []
downgrade_policy:
  - trigger: policy_bundle_schema_break
    downgrade_to: retest_pending
    propagation_refs:
      - docs/governance/policy_flag_schema_stack.md
      - artifacts/feedback/external_alpha_known_limits.md
  - trigger: proxy_lab_unavailable
    downgrade_to: limited
    propagation_refs:
      - artifacts/feedback/external_alpha_known_limits.md
  - trigger: managed_pilot_rollback_drill_failure
    downgrade_to: retest_pending
    propagation_refs:
      - docs/release/update_and_rollback_contract.md
  - trigger: managed_pilot_evidence_stale
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

# Managed-pilot cohort scorecard

This scorecard is the reviewer-facing summary of managed-pilot readiness for
the M3 beta admission lane. Tooling consumes the YAML front matter above and
the derived register at
`artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json`.

## Scope

The managed-pilot facet of `cohort:design_partner_managed_pilot` covers
named organizations running policy bundles, enterprise proxies, and support
exports on managed deployment profiles. The design-partner facet lives in
the sibling scorecard at
`artifacts/milestones/m3/cohorts/design_partner_scorecard.md`.

## Definition of green

A current scorecard requires that:

- a declared deployment profile resolves through
  `artifacts/deployment/locality_matrix.yaml`;
- a policy bundle and identity envelope are captured for the current beta
  build;
- the proxy lab is reachable on managed hardware; and
- no open waiver narrows scope past the declared support class.

The validator at `ci/check_cohort_archetype_scorecards.py` recomputes the
effective support class. Expired evidence or open waivers automatically
move this row to `retest_pending`, `limited`, or `evidence_stale` in the
derived register.

## Downgrade triggers and propagation

| Trigger | Auto-state | Propagation |
|---|---|---|
| Policy bundle schema break | `retest_pending` | `docs/governance/policy_flag_schema_stack.md`, `artifacts/feedback/external_alpha_known_limits.md` |
| Proxy lab unavailable | `limited` | `artifacts/feedback/external_alpha_known_limits.md` |
| Managed rollback drill failure | `retest_pending` | `docs/release/update_and_rollback_contract.md` |
| Evidence past freshness window | `evidence_stale` | `artifacts/feedback/external_alpha_known_limits.md` |

## Owner handoff path

- Intake and waiver entry: `@ahmeddyounis`
- Managed-pilot triage and routing: `@ahmeddyounis`
- Release-council escalation: `docs/governance/decision_rights_and_signoff_matrix.md`

## Verification

Run the validator and refresh the capture in the same change set:

```
python3 ci/check_cohort_archetype_scorecards.py --repo-root . \
  --report artifacts/milestones/m3/captures/cohort_archetype_scorecard_validation_capture.json \
  --register artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json
```
