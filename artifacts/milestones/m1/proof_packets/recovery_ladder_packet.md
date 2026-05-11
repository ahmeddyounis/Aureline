# Proof packet: M1 recovery-ladder lane

Purpose: anchor proof captures for the unattended M1 recovery-ladder
lane that proves safe mode, suspect-extension quarantine, open without
restore, cache / index repair, and restricted-mode fallback are bound
into one typed packet model with reversal-class honesty,
preserved-state guarantees, named escalation triggers, and a stable
linkage to the support bundle, the object-issue handoff packet, and
Project Doctor findings.

Reviewer entry point:
[`/docs/support/recovery_ladder_m1.md`](../../../docs/support/recovery_ladder_m1.md).

Canonical sources (non-exhaustive):

- `artifacts/support/recovery_ladder_cases.yaml` — joined lane matrix
  the runner consumes; each row binds one rung's seed case + reviewer
  example + linkage refs + named failure drill.
- `schemas/support/recovery_ladder_packet.schema.json` — boundary
  schema for the lane matrix (vocabularies, row shape, failure-drill
  envelope).
- `schemas/support/recovery_action.schema.json` — per-rung
  `recovery_action_record` and `recovery_ladder_seed_case_record`
  contract whose closed vocabularies the matrix re-exports.
- `docs/support/recovery_ladder_packet.md` — canonical packet contract;
  reversal-class honesty rules; linkage rules to the support bundle,
  the object-issue handoff packet, and Project Doctor findings.
- `fixtures/support/recovery_ladder_cases/*.yaml` — per-rung seed
  cases the matrix joins on `seed_case_ref`.
- `artifacts/support/recovery_examples/*.json` — per-rung reviewer
  examples the matrix joins on `reviewer_example_ref`.
- `artifacts/recovery/recovery_rungs.yaml` — recovery rung matrix and
  transition policy that names the eight rungs the matrix's
  `rung_class_vocabulary` re-exports.
- `fixtures/support/scenario_matrix.yaml` — Project Doctor scenario
  registry the matrix's `project_doctor_finding_ref` resolves against.
- `tests/recovery/m1_recovery_ladder_lane/run_recovery_ladder_lane.py`
  — unattended runner that replays the matrix and emits the durable
  JSON capture.

Live runtime consumers (read-only):

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

Validation captures:

- `artifacts/milestones/m1/captures/recovery_ladder_validation_capture.json`

Refresh: re-run the validation lane after a change to the lane matrix,
the per-rung seed cases or reviewer examples, the recovery rung
matrix, the Project Doctor scenario matrix, the recovery-action
schema, the lane-matrix schema, or the canonical packet doc.

Closure rule: the lane stays open until the latest capture lands
under the governed proof root and every row reports PASS for seed-case
agreement, reviewer-example presence, closed-vocabulary tokens, the
`user_authored_files` preservation rule, the `no_undo_export_only` and
`checkpoint_restore` honesty rules, the Project Doctor finding
binding, and its named failure drill — and the five required rungs
(`safe_mode`, `extension_quarantine`, `open_without_restore`,
`cache_reset_candidate`, `restricted_reopen`) are all observed.
