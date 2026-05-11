# Proof packet: M1 operator-truth demo path

Purpose: anchor proof captures for the bounded acceptance harness that
stitches the M1 operator-truth and certified-wedge prototypes into one
repeatable end-to-end walkthrough. The harness is the single
reviewable path M1 exit review walks to confirm that
preview/apply/revert lineage, typed permission prompts, install-review
fact-grid truth, representation-labeled safe preview/copy-export, AI
evidence and route/spend truth, target-graph readiness, host-boundary
cues, managed-workspace lifecycle, notebook trust badges, and the
composer/context-inspector seed line up on one shared vocabulary
rather than isolated local implementations.

The harness owns no new product vocabulary; every closed token quoted
on the acceptance checklist already belongs to an upstream bounded
wedge. This packet records what the harness proves and what it
explicitly does not.

Reviewer landing page:
[`docs/milestones/m1/operator_truth_demo_path.md`](../../../../docs/milestones/m1/operator_truth_demo_path.md).

## Canonical sources

- Reviewer doc:
  `docs/milestones/m1/operator_truth_demo_path.md`
- Acceptance checklist (canonical):
  `artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`
- Smoke runner:
  `tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py`
- Smoke README:
  `tests/ux/operator_truth_demo_smoke/README.md`
- Latest validation capture:
  `artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json`

## Upstream wedges the harness consumes (without forking)

| Acceptance row | Upstream wedge / module | Upstream reviewer doc | Upstream proof packet |
| --- | --- | --- | --- |
| `entry_preview_apply_revert` | `crates/aureline-shell/src/review_preview/` | `docs/ux/m1_preview_apply_revert_core_path.md` | `artifacts/milestones/m1/proof_packets/preview_apply_revert_core_path.md` |
| `permission_review_typed_prompt` | `crates/aureline-shell/src/permission_prompts/` | `docs/ux/m1_typed_permission_prompt.md` | `artifacts/milestones/m1/proof_packets/typed_permission_prompts_seed.md` |
| `install_review_fact_grid` | `crates/aureline-shell/src/install_review_fact_grid/` | `docs/ux/m1_install_review_fact_grid_seed.md` | `artifacts/milestones/m1/proof_packets/install_review_fact_grid_seed.md` |
| `safe_preview_representation_labels` | `crates/aureline-preview/src/safe_preview/`, `crates/aureline-shell/src/safe_preview_card/` | `docs/ux/m1_safe_preview_and_copy_export.md` | `artifacts/milestones/m1/proof_packets/safe_preview_and_copy_export.md` |
| `ai_evidence_and_spend_truth_strip` | `crates/aureline-shell/src/ai_truth_strip/` | `docs/ai/m1_evidence_and_spend_seed.md` | `artifacts/milestones/m1/proof_packets/evidence_and_spend_seed.md` |
| `target_graph_state_card` | `crates/aureline-shell/src/graph_state_card/` | `docs/ux/m1_graph_state_card_seed.md` | `artifacts/milestones/m1/proof_packets/graph_state_card_seed.md` |
| `host_boundary_cues` | `crates/aureline-shell/src/host_boundary_cues/` | `docs/ux/m1_host_boundary_cues_seed.md` | `artifacts/milestones/m1/proof_packets/host_boundary_cues_seed.md` |
| `managed_workspace_lifecycle` | `crates/aureline-shell/src/managed_workspace_labels/` | `docs/ux/m1_managed_workspace_lifecycle_seed.md` | `artifacts/milestones/m1/proof_packets/managed_workspace_lifecycle_seed.md` |
| `notebook_trust_badges` | `crates/aureline-shell/src/notebook_trust_badges/` | `docs/ux/m1_notebook_trust_badges_seed.md` | `artifacts/milestones/m1/proof_packets/notebook_trust_badges_seed.md` |
| `composer_and_context_inspector_seed` | `crates/aureline-shell/src/ai_context_inspector/` | `docs/ai/m1_composer_and_context_inspector_seed.md` | `artifacts/milestones/m1/proof_packets/composer_and_context_inspector_seed.md` |

## Protected walk

Run the smoke unattended against the checked-in checklist:

```
python3 tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py \
  --repo-root .
```

The smoke MUST:

- resolve the reviewer doc, proof packet, and protected-walk fixture
  on every acceptance row in the checked-in checklist;
- confirm every closed `expected_tokens` value resolves on the bound
  fixture and matches verbatim;
- record per-row observations in the JSON capture so reviewers can see
  what the lane actually saw, not just a pass/fail line;
- exit 0 with a `passed` line and write the durable capture to
  `artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json`.

Evidence:

- Smoke runner: `tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py`
- Checklist: `artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`
- Capture: `artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json`

## Failure drill — vocabulary drift on one wedge is caught

Each acceptance row declares one named `failure_drill` (drill id +
forced value path + forced value + expected `check_id`). The drill
replays one row through the smoke with a `forced_value` that
contradicts the upstream protected-walk fixture; the smoke MUST
surface the expected `operator_truth_demo_smoke.acceptance_row.expected_token_mismatch`
check rather than silently passing.

Replay the canonical revert-class drift drill:

```
python3 tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py \
  --repo-root . \
  --force-drill preview_apply_revert_core_path.revert_class_drift
```

The drill exits 0 only when the expected check id is reproduced. A
drill that fails to surface its expected check is itself recorded as
`operator_truth_demo_smoke.failure_drill.expected_finding_missing`.

Evidence:

- Drill table: `tests/ux/operator_truth_demo_smoke/README.md`
- Per-row drill: `artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`
  (`acceptance_rows[*].failure_drill`)

## Adjacent drills

- `notebook_trust_badges_seed.trust_rung_drift` — re-running the smoke
  with the bound notebook fixture's
  `trust_axes.notebook_trust_rung` rewritten to `untrusted_tainted`
  surfaces the typed token-mismatch check against the
  `notebook_trust_badges` row.
- `managed_workspace_lifecycle_seed.copy_class_drift` — re-running with
  the managed-workspace `expect.current_copy_class` rewritten to
  `suspended_workspace` surfaces the typed token-mismatch check
  against the `managed_workspace_lifecycle` row.
- `composer_and_context_inspector_seed.trust_posture_drift` — re-running
  with the composer fixture's
  `input.attachments.0.trust_posture` rewritten to
  `trusted_workspace_authored` surfaces the typed token-mismatch check
  against the `composer_and_context_inspector_seed` row, proving the
  smoke refuses to silently treat pasted bytes as instruction
  authority.

## Validation command

```
python3 tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py \
  --repo-root .
```

## Evidence storage

- Reviewer doc: `docs/milestones/m1/operator_truth_demo_path.md`
- Acceptance checklist:
  `artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`
- Smoke runner and README:
  `tests/ux/operator_truth_demo_smoke/`
- Latest capture:
  `artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json`

## Out of scope

- Productized acceptance harnesses for downstream milestones — this
  packet targets the M1 exit review only.
- Replacing per-wedge proof packets or per-wedge validation lanes —
  the harness is additive and points back at each upstream
  `validation_command`.
- New product-surface invention beyond stitching the existing M1
  prototype wedges into one reviewable path.
- Any market-facing claim that goes beyond the explicit bounded
  prototype labels each upstream wedge already carries.
