---
schema_version: 1
scorecard_kind: archetype_scorecard
scorecard_id: archetype_scorecard:c_or_cpp_native_project
archetype_row_ref: archetype_row:c_or_cpp_native_project
public_label: C / C++ native project
inherited_from_milestone: m3
title: C / C++ native project archetype scorecard
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
evidence_refs:
  - artifacts/compat/reference_workspace_rows.yaml#archetype_row:c_or_cpp_native_project
  - artifacts/compat/m3/reference_workspace_register.yaml#m3_reference_workspace:cpp_native
  - artifacts/compat/m3/reference_workspace_report.json#reference_workspace_report_row:cpp_native
  - artifacts/compat/m3/reference_workspace_badges.json#reference_workspace_badge:c_or_cpp_native_project
  - fixtures/reference_workspaces/m3/cpp_native/workspace.yaml
  - fixtures/reference_workspaces/m3/cpp_native/harness.yaml
  - docs/compat/m3/reference_workspaces_beta.md
  - docs/release/certified_archetype_report_template.md
open_waivers:
  - waiver_id: waiver:archetype.c_or_cpp_native_project.first_beta_seed
    state: active
    expires_on: "2026-06-30"
    impact: "Reference workspace fixture and pass/fail harness are seeded; the row holds a Limited claim until current workflow captures land."
    propagation_refs:
      - artifacts/feedback/external_alpha_known_limits.md
      - docs/release/certified_archetype_report_template.md
downgrade_policy:
  - trigger: reference_workspace_report_stale
    downgrade_to: retest_pending
    propagation_refs:
      - docs/release/certified_archetype_report_template.md
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
  - docs/compat/m3/reference_workspaces_beta.md
  - docs/release/certified_archetype_report_template.md
  - docs/release/compatibility_report_template.md
  - docs/migration/compatibility_scorecard_contract.md
---

# Archetype scorecard: C / C++ native project

This scorecard is the reviewer-facing readiness summary for
`archetype_row:c_or_cpp_native_project` in the M3 beta admission lane.
Its effective support class is capped by the current reference-workspace
report at `artifacts/compat/m3/reference_workspace_report.json`.

## Definition of green

The archetype row holds its declared support class while:

- a reference workspace, CMake or build-system shape, and benchmark corpus
  row are current for the named C/C++ toolchain;
- the seeded workflow set boots, opens, searches, renames, runs tests, and
  supports debug on the current beta build; and
- no open waiver narrows the public claim past the declared class.

The first-beta waiver is active; the validator derives a `limited` effective
state until current workflow captures land.

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
