---
schema_version: 1
scorecard_kind: cohort_scorecard
scorecard_id: cohort_scorecard:extension_author
cohort_id: cohort:extension_author
cohort_sub_focus: extension_author
title: Extension-author cohort scorecard
owner: "@ahmeddyounis"
evidence_owner: "@ahmeddyounis"
as_of: "2026-05-15"
evidence_date: "2026-05-15"
review_window_days: 21
freshness_state: current
declared_support_class: supported
display_lifecycle_label: beta
primary_surface_refs:
  - beta_surface:extension_runtime
  - beta_surface:packaging_update_rollback
evidence_refs:
  - docs/governance/public_interface_versioning_policy.md
  - docs/release/release_evidence_packet_template.md
  - artifacts/compat/reference_workspace_rows.yaml
  - artifacts/feedback/external_alpha_known_limits.md
open_waivers: []
downgrade_policy:
  - trigger: sdk_breaking_change_without_migration
    downgrade_to: retest_pending
    propagation_refs:
      - docs/governance/public_interface_versioning_policy.md
      - artifacts/feedback/external_alpha_known_limits.md
  - trigger: publication_mirror_unavailable
    downgrade_to: limited
    propagation_refs:
      - artifacts/feedback/external_alpha_known_limits.md
  - trigger: extension_rollback_drill_failure
    downgrade_to: retest_pending
    propagation_refs:
      - docs/release/update_and_rollback_contract.md
  - trigger: extension_author_evidence_stale
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

# Extension-author cohort scorecard

This scorecard is the reviewer-facing summary of extension-author readiness
for the M3 beta admission lane. Tooling consumes the YAML front matter above
and the derived register at
`artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json`.

## Scope

`cohort:extension_author` covers the extension SDK, runtime, and publication
path. The scorecard tracks whether sample extensions, the diff report, and
the rollback drill keep the publication mirror trustworthy enough to widen
beta to external add-on authors.

## Definition of green

A current scorecard requires that:

- the SDK diff report is published against the current beta build;
- sample extensions run on three consecutive beta builds without breaking
  surface contracts;
- the publication mirror or offline path is reachable on partner hardware;
  and
- no open waiver narrows the SDK surface past the declared support class.

The validator at `ci/check_cohort_archetype_scorecards.py` recomputes the
effective support class. Expired evidence or open waivers automatically
move this row to `retest_pending`, `limited`, or `evidence_stale` in the
derived register.

## Downgrade triggers and propagation

| Trigger | Auto-state | Propagation |
|---|---|---|
| SDK breaking change without migration | `retest_pending` | `docs/governance/public_interface_versioning_policy.md`, `artifacts/feedback/external_alpha_known_limits.md` |
| Publication mirror unavailable | `limited` | `artifacts/feedback/external_alpha_known_limits.md` |
| Extension rollback drill failure | `retest_pending` | `docs/release/update_and_rollback_contract.md` |
| Evidence past freshness window | `evidence_stale` | `artifacts/feedback/external_alpha_known_limits.md` |

## Owner handoff path

- Intake and waiver entry: `@ahmeddyounis`
- SDK and runtime triage: `@ahmeddyounis`
- Release-council escalation: `docs/governance/decision_rights_and_signoff_matrix.md`

## Verification

Run the validator and refresh the capture in the same change set:

```
python3 ci/check_cohort_archetype_scorecards.py --repo-root . \
  --report artifacts/milestones/m3/captures/cohort_archetype_scorecard_validation_capture.json \
  --register artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json
```
