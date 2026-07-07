# M5 Failure-Triage-Panel / Quarantine-Review-Sheet / Environment-Matrix-Card Primitive

- Packet: `m5-failure-triage-quarantine-environment-primitive:stable:0001`
- Label: `M5 failure-triage-panel / quarantine-review-sheet / environment-matrix-card primitive: assertion/diff summaries, recent attempt sequences, environment/build/runtime deltas, classifier confidence, evidence-gated rerun/debug/open-review actions, preserved suppression scope/kind/reason/owner/expiry/linked-artifacts/release-impact with always-visible quarantines and a restore action, and target/runtime/toolchain/build compatibility comparison that never implies safe equivalence across incompatible environments`
- Quality surfaces: 5 (5 stable)
- Triage postures: assertion_evidence_panel, runtime_evidence_panel, timeout_evidence_panel, environment_evidence_panel, flaky_review_panel, unclassified_evidence_panel
- Quarantine postures: expired_suppression, unowned_suppression, hidden_release_suppression, review_due_suppression, blocking_suppression, governed_suppression
- Environment postures: incompatible_matrix, unverified_matrix, mixed_matrix, compatible_matrix
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Quality surfaces

- **Test Explorer Triage View**: `stable`
  - Owner: Test explorer triage owner
  - Scope: The test-explorer triage view renders the shared failure-triage panel so a live-local assertion failure reads with its assertion/diff summary, recent attempt sequence, environment/build/runtime deltas, and classifier confidence before it can escalate to the quarantine review, and it renders the quarantine review sheet so a blocking or hidden-from-release suppression stays visible with its owner, expiry, and release impact and always offers restore, and it renders the environment-matrix card so a fully compatible unit-test matrix compares every leg without implying safe equivalence
  - Worked triage: 2 / quarantine: 2 / environment: 1
    - triage `triage:explorer::auth-assert` (`assertion_failure`) → `assertion_evidence_panel` (evidence `true`, open-review `true`)
    - triage `triage:explorer::ci-timeout` (`timeout`) → `timeout_evidence_panel` (evidence `true`, open-review `true`)
    - quarantine `quarantine:explorer::auth-blocking` (`team_owned`) → `blocking_suppression` (visible `true`, restore `true`)
    - quarantine `quarantine:explorer::pricing-hidden` (`self_owned`) → `hidden_release_suppression` (visible `true`, restore `true`)
    - environment `environment:explorer::unit-compatible` (`fully_compatible`) → `compatible_matrix` (incompatible-leg `false`, safe-equivalence `false`)
- **Editor Inline Triage**: `stable`
  - Owner: Editor inline triage owner
  - Scope: The editor inline triage renders the shared failure-triage panel so a low-confidence runtime error reads as provisional with its evidence before rerun/debug/review, renders the quarantine review sheet so an unowned muted suppression stays visible with a reassign-owner action rather than disappearing into a filter, and renders the environment-matrix card so a mixed integration matrix warns instead of implying safe equivalence
  - Worked triage: 1 / quarantine: 1 / environment: 1
    - triage `triage:editor::reducer-panic` (`runtime_error`) → `runtime_evidence_panel` (evidence `true`, open-review `true`)
    - quarantine `quarantine:editor::legacy-unowned` (`unowned`) → `unowned_suppression` (visible `true`, restore `true`)
    - environment `environment:editor::integration-mixed` (`partially_compatible`) → `mixed_matrix` (incompatible-leg `false`, safe-equivalence `false`)
- **Notebook Triage View**: `stable`
  - Owner: Notebook triage owner
  - Scope: The notebook triage view renders the shared failure-triage panel so a replayed environment error reads with its deltas and recent attempts, renders the quarantine review sheet so an expired, owner-expired skip suppression reads with renew and reassign actions and its linked artifacts while staying visible, and renders the environment-matrix card so an incompatible browser end-to-end matrix warns and never implies safe equivalence
  - Worked triage: 1 / quarantine: 1 / environment: 1
    - triage `triage:notebook::gpu-missing` (`environment_error`) → `environment_evidence_panel` (evidence `true`, open-review `true`)
    - quarantine `quarantine:notebook::locale-expired` (`owner_expired`) → `expired_suppression` (visible `true`, restore `true`)
    - environment `environment:notebook::e2e-incompatible` (`incompatible`) → `incompatible_matrix` (incompatible-leg `true`, safe-equivalence `false`)
- **Run Panel Triage**: `stable`
  - Owner: Run panel triage owner
  - Scope: The run-panel triage renders the shared failure-triage panel so a flaky failure under review reads with its recent pass/fail attempt sequence and confidence, renders the quarantine review sheet so a CI-enforced review-due quarantine reads with a renew action while staying visible with its informational release impact, and renders the environment-matrix card so an unverified benchmark matrix reads as unverified rather than compatible
  - Worked triage: 1 / quarantine: 1 / environment: 1
    - triage `triage:run-panel::scheduler-flaky` (`flaky_under_review`) → `flaky_review_panel` (evidence `true`, open-review `true`)
    - quarantine `quarantine:run-panel::tagged-review-due` (`ci_enforced`) → `review_due_suppression` (visible `true`, restore `true`)
    - environment `environment:run-panel::benchmark-unverified` (`unverified`) → `unverified_matrix` (incompatible-leg `false`, safe-equivalence `false`)
- **Quality Report Export**: `stable`
  - Owner: Quality report export owner
  - Scope: The quality report export renders the shared failure-triage panel so an unclassified failure with unknown confidence still reads as provisional with its recent attempts, renders the quarantine review sheet so a governed self-owned permanent mute reads as governed with no outstanding review while still preserving its reason and restore, and renders the environment-matrix card so a compatible contract matrix compares every leg — the same triage a reviewer reads in the tree and run-panel consumers
  - Worked triage: 1 / quarantine: 1 / environment: 1
    - triage `triage:report::unclassified-exit` (`unknown_failure`) → `unclassified_evidence_panel` (evidence `true`, open-review `true`)
    - quarantine `quarantine:report::vendor-governed` (`self_owned`) → `governed_suppression` (visible `true`, restore `true`)
    - environment `environment:report::contract-compatible` (`fully_compatible`) → `compatible_matrix` (incompatible-leg `false`, safe-equivalence `false`)
