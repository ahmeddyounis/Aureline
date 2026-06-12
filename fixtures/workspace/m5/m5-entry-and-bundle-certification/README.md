# Fixtures: M5 entry-and-bundle certification report

This directory contains fixture metadata for the `m5_entry_and_bundle_certification_report`
packet.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-entry-and-bundle-certification.json`

It is the certification layer above the governance matrix at:

`artifacts/workspace/m5/m5-entry-and-bundle-governance.json`

## Coverage

- `workflow_bundle`, `source_acquisition`, `project_open`, `project_import`, `session_resume`,
  `recent_work`, and `workspace_admission` are the only claimed lanes, and each carries exactly
  one certification row — no lane inherits a verified label from an adjacent one.
- Each row ingests the governance matrix's published label as its `governance_claim` (validated
  against the live `m5_entry_and_bundle_governance_matrix` packet), binds to the canonical
  governance packet via `governance_packet_ref`, and points at its source governance row via
  `governance_row_ref`. The published label can never exceed the governance claim.
- Every row covers all seven drills — `project_entry`, `recent_work`, `source_acquisition`,
  `bundle_lifecycle`, `admission`, `accessibility`, and `downgrade` — exactly once, and any
  drill that ran carries an evidence ref.
- Drill outcomes cover `passed`, `narrowed` (`source_acquisition` lanes), `failed`
  (`workspace_admission` admission drill), and `not_run` (`workspace_admission` downgrade
  drill). Evidence freshness covers `current`, `aging` (`session_resume`), `expired`
  (`project_import`), and `missing` (`workspace_admission`).
- Published label covers `verified`, `bounded`, `retest_pending`, and `withheld`, and the
  certification decision covers `admit_full`, `admit_bounded`, `admit_retest`, and `refuse`.
- The four downgrade reasons — `governance_narrowed`, `evidence_stale`, `drill_narrowed`, and
  `drill_failed` — are each exercised by at least one lane, and the five recovery paths —
  `rerun_drills`, `refresh_evidence`, `adopt_governance_narrowing`, `withhold_row`, and `none`
  — are each exercised.
- The gate is exercised in every direction: `project_open` and `recent_work` certify verified
  (clean governance, current evidence, all drills passed), proving the certifier is not a
  blanket downgrade; `workflow_bundle` adopts the governance narrowing alone; `source_acquisition`
  narrows on a drill; `project_import` narrows on expired evidence; `session_resume` narrows on
  aging evidence and a narrowed drill; and `workspace_admission` is withheld with no supported
  profile. Each lane's `published_label`, `certification_decision`, `downgrade_reasons`, and
  `downgrade_path` equal the recomputed gate, so the start-center, migration-center, help/About,
  release-center, docs/help, and support-export surfaces ingest one packet and a narrowed lane
  cannot stay green by inertia.
