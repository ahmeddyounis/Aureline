# Fixtures: M5 workflow-bundle and project-entry governance matrix

This directory contains fixture metadata for the `m5_entry_and_bundle_governance_matrix`
packet.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-entry-and-bundle-governance.json`

## Coverage

- `workflow_bundle`, `source_acquisition`, `project_open`, `project_import`, `session_resume`,
  `recent_work`, and `workspace_admission` are the only claimed lanes, and each carries exactly
  one row — no lane inherits a verified label from an adjacent one.
- Each lane is pinned to one distinct entry verb (`clone`, `open`, `import`, `resume`,
  `install`), validated against `EntryBundleLane::entry_verb`, so clone, open, import, and
  resume never blur together; every verb is exercised.
- Each row binds to the canonical entry-truth packet it governs via `packet_ref` (validated
  against `EntryBundleLane::source_packet`), so the governance matrix aggregates the landed
  stable-line entry and bundle packets rather than a parallel spreadsheet, and each row carries
  its own conformance, evidence, governance-receipt, release-evidence, help-surface,
  docs-badge, and support-export refs.
- Source trust covers `first_party`, `trusted_remote`, `unverified_remote`, and `untrusted`;
  archetype confidence covers `confirmed`, `probable`, `mixed`, and `undetected`; root
  resolution covers `resolved`, `single_root_assumed`, `probable_multi_root`, and `missing`;
  restore fidelity covers `exact`, `partial`, `degraded`, and `unavailable`; bundle scorecard
  covers `current`, `aging`, `stale`, and `missing`; entry topology support covers
  `supported`, `degraded_support`, `experimental`, and `unsupported`.
- The setup-queue class covers `ready`, `setup_later`, `blocked_on_setup`, and `missing_root`,
  so a ready entry stays distinct from a setup-later, blocked, or root-missing one. The
  `source_acquisition` and `project_import` lanes carry a non-zero `deferred_setup_count`, the
  `workspace_admission` lane carries a non-zero `missing_root_count`, and the verified
  `project_open` and `recent_work` lanes defer nothing.
- Published label covers `verified`, `bounded`, `retest_pending`, and `withheld`, and the
  admission outcome covers `admit_full`, `admit_bounded`, `admit_retest`, and `refuse`.
- The six downgrade reasons — `unverified_source`, `probable_or_mixed_detection`,
  `missing_roots`, `partial_restore`, `stale_bundle_scorecard`, and
  `unsupported_entry_topology` — are each exercised by at least one lane, and `project_import`
  exercises all six at once.
- The governance gate is exercised in every direction: the clean `project_open` and
  `recent_work` lanes admit at full trust (verified); the probable, trusted-remote
  `source_acquisition` clone narrows to a bounded label; the stale `workflow_bundle`, the
  mixed `project_import`, and the degraded-restore `session_resume` lanes narrow to
  retest-pending; and the untrusted, undetected, root-missing `workspace_admission` lane is
  refused entirely. `source_acquisition` and `project_import` are the automatic-downgrade
  cases — a probable clone and a mixed import are dropped from their declared verified label
  rather than left widening trust — while `project_open` and `recent_work` prove the gate is
  not a blanket downgrade. The trust-sensitive `workflow_bundle`, `source_acquisition`,
  `project_import`, `session_resume`, and `workspace_admission` lanes narrow safely instead of
  inheriting a broader stable claim. Each lane's `published_assurance`, `admission_outcome`,
  and `downgrade_reasons` equal the recomputed gate decision, so release, help/start-center,
  docs, and support-export surfaces ingest one packet and a narrowed lane cannot stay stable
  by inertia.
