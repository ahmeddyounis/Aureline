# Operator-truth demo smoke

Unattended acceptance smoke that walks the canonical M1 operator-truth
checklist
([`/artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`](../../../artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml))
and proves the M1 prototype wedges line up on one shared vocabulary
rather than isolated local implementations. The lane is the acceptance
harness behind the reviewer-facing landing page
[`/docs/milestones/m1/operator_truth_demo_path.md`](../../../docs/milestones/m1/operator_truth_demo_path.md).

The smoke is bounded: it consumes the already-landed bounded wedges and
their protected-walk fixtures, replays them, and confirms the
checklist's closed `expected_tokens` row-by-row. It does **not**
replace the per-wedge proof packets or per-wedge validation lanes; each
checklist row points at the upstream `validation_command` reviewers
can rerun directly.

## What the lane proves

For each acceptance row, the smoke confirms:

- the reviewer-facing landing page exists in the repo;
- the owning proof packet exists in the repo;
- the named protected-walk fixture exists and parses as JSON;
- every closed `expected_tokens` entry resolves on the fixture and
  carries the exact value the checklist claims;
- a named `failure_drill` is declared on the row (drill id + forced
  value path + forced value + expected `check_id`) so vocabulary drift
  is caught loudly.

The smoke records a per-row observation in the JSON capture: the
expected and observed token values, the bound proof packet, the bound
reviewer doc, and the upstream `validation_command` so reviewers can
see *what the lane actually saw* rather than just a pass/fail line.

## Run unattended

```bash
python3 tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py \
  --repo-root .
```

The runner emits a durable JSON capture at
`artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json`
and exits non-zero on any regression.

## Failure drills (prove the lane fails loudly)

Each acceptance row declares one named failure drill in the checklist
with a forced mutation and the `check_id` the lane MUST report when
that mutation is forced.

| Drill | Forced mutation (against the checklist row) | Expected check |
| --- | --- | --- |
| `preview_apply_revert_core_path.revert_class_drift` | rewrite expected `realized_revert_class_token` to `compensating_action` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `typed_permission_prompts_seed.decision_state_drift` | flip expected `decision_state` to `denied` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `install_review_fact_grid_seed.activation_budget_drift` | rewrite expected `activation_budget_class` to `eager_within_workspace_only` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `safe_preview_and_copy_export.representation_drift` | rewrite expected `currently_visible_representation_token` to `raw` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `evidence_and_spend_seed.run_state_drift` | rewrite expected `run_state_class` to `post_run_completed_mocked` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `graph_state_card_seed.readiness_drift` | rewrite expected `readiness_label` to `stale` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `host_boundary_cues_seed.boundary_cue_drift` | rewrite expected `current_boundary_cue` to `remote_host_active` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `managed_workspace_lifecycle_seed.copy_class_drift` | rewrite expected `current_copy_class` to `suspended_workspace` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `notebook_trust_badges_seed.trust_rung_drift` | rewrite expected `notebook_trust_rung` to `untrusted_tainted` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |
| `composer_and_context_inspector_seed.trust_posture_drift` | rewrite expected attachment `trust_posture` to `trusted_workspace_authored` | `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch` |

Replay one with:

```bash
python3 tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py \
  --repo-root . \
  --force-drill preview_apply_revert_core_path.revert_class_drift
```

The drill exits 0 only if the expected `check_id` was actually
reported. A drill that *fails* to surface its expected check is
itself a failure mode — the runner records it as
`operator_truth_demo_smoke.failure_drill.expected_finding_missing`.

## Adjacent lanes

- Reviewer landing page:
  [`/docs/milestones/m1/operator_truth_demo_path.md`](../../../docs/milestones/m1/operator_truth_demo_path.md).
- Acceptance checklist (canonical):
  [`/artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`](../../../artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml).
- Proof packet:
  [`/artifacts/milestones/m1/proof_packets/operator_truth_demo_path.md`](../../../artifacts/milestones/m1/proof_packets/operator_truth_demo_path.md).
- Per-wedge proof packets and validation lanes (each row's
  `validation_command` reruns the upstream wedge's lane directly).
