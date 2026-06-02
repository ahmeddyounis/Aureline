# Harden certified reference workspaces, archetype pass matrices, and downgrade automation

Workstream batch: B11 — Release engineering, certification, performance, accessibility, docs truth, and public proof.

Execution wave: W11
Readiness wave: R11
Milestone fit: Required M4 stable promotion and public-proof foundation
Delivery class: Governed artifact + release-control + certification lane
Track: Published proof, one-build identity, stable docs/help truth, certified archetypes, and operator-facing release readiness.

## Goal

Turn the certified-reference-workspaces and archetype-pass-matrix lanes into
release-grade proof so that every marketed Certified archetype is backed by a
current reference-workspace report, any stale or missing report automatically
narrows the Certified claim, and shiproom can fail promotion when the lane is
violated.

## What is in this artifact

The checked-in artifact at
`artifacts/release/harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation.json`
publishes:

- One `ReferenceWorkspaceReport` per marketed Certified archetype, carrying:
  - `certification_harness_output_ref` — the harness output that produced the report
  - `matrix_diff_from_prior_ref` — diff from the prior report
  - `owner_ref` and `owner_signoff` — named owner and sign-off
  - `known_caveat_summary` — reviewable caveats
  - `validity_window` — captured_at, expires_at, window_days
  - `report_state` — current, due_for_refresh, expired, or missing
  - `effective_state` — after narrowing automation

- One `ArchetypePassMatrixRow` per archetype, carrying:
  - `claimed_certified` and `effective_certified`
  - `matrix_state` — certified, provisional_on_waiver, narrowed_unqualified, narrowed_stale, narrowed_waiver_expired
  - `pass_criteria_refs` — criteria the archetype must pass
  - `active_downgrade_reasons` — reasons that narrowed the row

- `DowngradeRule` set covering all seven closed reasons:
  - `reference_workspace_report_stale`
  - `reference_workspace_report_missing`
  - `reference_workspace_report_manually_edited`
  - `archetype_pass_matrix_regression`
  - `archetype_decertified`
  - `waiver_expired`
  - `owner_signoff_missing`

- A `publication` block with the recomputed `hold`/`proceed` verdict.

- A `summary` block with counts that the typed model recompute from the rows.

## Downgrade automation

A Certified claim narrows automatically when any of the following is true:

| Condition | State | Action |
|---|---|---|
| Report expired | `expired` | Refresh the reference-workspace report |
| Report missing | `missing` | Capture the reference-workspace report |
| Report manually edited | `manually_edited` | Recapture through the certification harness |
| Matrix regression | `regression` | Recapture the archetype pass matrix |
| Archetype decertified | `decertified` | Inherit the upstream ceiling |
| Waiver expired | `waiver_expired` | Hold publication |
| Owner sign-off missing | `owner_signoff_missing` | Request owner sign-off |

## Current posture

At this revision two of four archetypes are certified, two are narrowed. The
publication gate holds because two blocking downgrade rules fire.

## Verification

```
cargo test -p aureline-release
```

## Risks and follow-ups

- The extension-author archetype lacks a reference-workspace report entirely;
  this is tracked as a blocking gap.
- The legacy remote-SSH report expired on 2026-05-20 and must be refreshed before
the archetype can reclaim Certified.
