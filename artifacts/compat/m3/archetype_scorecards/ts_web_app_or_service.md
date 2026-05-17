---
schema_version: 1
scorecard_kind: archetype_scorecard
scorecard_id: archetype_scorecard:ts_web_app_or_service
archetype_row_ref: archetype_row:ts_web_app_or_service
public_label: TypeScript / JavaScript web app or service
inherited_from_milestone: m2
title: TS/JS web app or service archetype scorecard
owner: "@ahmeddyounis"
evidence_owner: "@ahmeddyounis"
as_of: "2026-05-15"
evidence_date: "2026-05-15"
review_window_days: 21
freshness_state: current
declared_support_class: experimental
target_support_class_at_beta_exit: supported
target_support_class_at_stable: certified
display_lifecycle_label: beta
minimum_platform_matrix:
  - macos_arm64
  - macos_x86_64
  - linux_x86_64
  - windows_x86_64
minimum_mode_matrix:
  - local_only
  - local_plus_one_remote_mode
evidence_refs:
  - artifacts/compat/reference_workspace_rows.yaml#archetype_row:ts_web_app_or_service
  - artifacts/compat/m3/reference_workspace_report.json#reference_workspace_report_row:ts_web_app_or_service
  - artifacts/compat/m3/reference_workspace_badges.json#reference_workspace_badge:ts_web_app_or_service
  - artifacts/certification/m2_archetype_seed_rows.yaml#archetype_certification_seed:ts_web_app_or_service
  - artifacts/milestones/m2/alpha_wedge_matrix.yaml#alpha_wedge:typescript_javascript
  - docs/release/certified_archetype_report_template.md
open_waivers: []
downgrade_policy:
  - trigger: reference_workspace_report_stale
    downgrade_to: retest_pending
    propagation_refs:
      - docs/release/certified_archetype_report_template.md
      - artifacts/feedback/external_alpha_known_limits.md
  - trigger: archetype_seed_evidence_stale
    downgrade_to: evidence_stale
    propagation_refs:
      - docs/release/certified_archetype_report_template.md
  - trigger: regression_on_certified_archetype
    downgrade_to: limited
    propagation_refs:
      - artifacts/feedback/external_alpha_known_limits.md
owner_handoff_path:
  intake_owner: "@ahmeddyounis"
  triage_owner: "@ahmeddyounis"
  release_owner: "@ahmeddyounis"
  escalation_ref: docs/governance/decision_rights_and_signoff_matrix.md
consuming_surfaces:
  - docs/milestones/m3/beta_admission_matrix.md
  - docs/release/certified_archetype_report_template.md
  - docs/release/compatibility_report_template.md
  - docs/migration/compatibility_scorecard_contract.md
---

# Archetype scorecard: TypeScript / JavaScript web app or service

This scorecard is the reviewer-facing readiness summary for
`archetype_row:ts_web_app_or_service` in the M3 beta admission lane.
Tooling consumes the YAML front matter above and the derived register at
`artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json`.
Its effective support class is capped by the current reference-workspace
report at `artifacts/compat/m3/reference_workspace_report.json`.

## Definition of green

The archetype row holds its declared support class while:

- the alpha TS/JS wedge evidence remains current and unblocked;
- the seeded reference workspace boots, opens, searches, renames, runs
  tests, and supports debug on the current beta build; and
- no open waiver narrows the public claim past the declared class.

Expired evidence and open waivers automatically move this row to
`retest_pending`, `limited`, or `evidence_stale` in the derived register.

## Downgrade triggers

| Trigger | Auto-state | Propagation |
|---|---|---|
| Reference-workspace report stale or not-run | `retest_pending` | `artifacts/compat/m3/reference_workspace_report.json`, `docs/release/certified_archetype_report_template.md` |
| Archetype seed evidence stale | `evidence_stale` | `docs/release/certified_archetype_report_template.md` |
| Regression on certified archetype | `limited` | `artifacts/feedback/external_alpha_known_limits.md` |

## Owner handoff path

- Intake and waiver entry: `@ahmeddyounis`
- Archetype triage and routing: `@ahmeddyounis`
- Release-council escalation: `docs/governance/decision_rights_and_signoff_matrix.md`

## Verification

`python3 ci/check_cohort_archetype_scorecards.py --repo-root .`
