---
schema_version: 1
scorecard_kind: archetype_scorecard
scorecard_id: archetype_scorecard:java_or_kotlin_service
archetype_row_ref: archetype_row:java_or_kotlin_service
public_label: Java / Kotlin service
inherited_from_milestone: m3
title: Java / Kotlin service archetype scorecard
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
  - artifacts/compat/reference_workspace_rows.yaml#archetype_row:java_or_kotlin_service
  - docs/release/certified_archetype_report_template.md
open_waivers:
  - waiver_id: waiver:archetype.java_or_kotlin_service.first_beta_seed
    state: active
    expires_on: "2026-06-30"
    impact: "Reference workspace fixture is reservation-only at M3 entry; the row holds a Limited claim until the workspace and capture land."
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
  - docs/release/certified_archetype_report_template.md
  - docs/release/compatibility_report_template.md
  - docs/migration/compatibility_scorecard_contract.md
---

# Archetype scorecard: Java / Kotlin service

This scorecard is the reviewer-facing readiness summary for
`archetype_row:java_or_kotlin_service` in the M3 beta admission lane.

## Definition of green

The archetype row holds its declared support class while:

- a reference workspace, toolchain matrix, and benchmark corpus row are
  current for Gradle, Maven, and the named JVM toolchain;
- the seeded workflow set boots, opens, searches, renames, runs tests, and
  supports debug on the current beta build; and
- no open waiver narrows the public claim past the declared class.

The first-beta waiver is active; the validator derives a `limited` effective
state until the reference workspace and capture land.

## Downgrade triggers

| Trigger | Auto-state | Propagation |
|---|---|---|
| Reference-workspace report stale | `retest_pending` | `docs/release/certified_archetype_report_template.md` |
| Archetype seed evidence stale | `evidence_stale` | `docs/release/certified_archetype_report_template.md` |
| Regression on certified archetype | `limited` | `artifacts/feedback/external_alpha_known_limits.md` |

## Owner handoff path

- Intake and waiver entry: `@ahmeddyounis`
- Archetype triage and routing: `@ahmeddyounis`
- Release-council escalation: `docs/governance/decision_rights_and_signoff_matrix.md`

## Verification

`python3 ci/check_cohort_archetype_scorecards.py --repo-root .`
