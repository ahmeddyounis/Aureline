# M1 operator-truth demo path

This page is the reviewer-facing landing page for the bounded acceptance
harness that stitches the M1 operator-truth and certified-wedge
prototypes into **one repeatable end-to-end walkthrough**. The harness
itself owns no new product vocabulary; it consumes the already-landed
upstream wedges and proves they line up on one shared vocabulary
instead of isolated local implementations.

The harness is bounded:

- It traverses **one** demo path that exercises entry, edit/save,
  preview/apply/revert, permission review, evidence/route truth,
  boundary cues, and acceptance evidence in a single coherent
  walkthrough. It is an acceptance harness, not a new product surface.
- It does **not** re-mint vocabulary. Every wedge listed below already
  owns its closed token sets, and the harness only quotes them.
- It is **not** the only proof for the underlying tasks; each upstream
  wedge keeps its own proof packet and validation lane.
- Out of scope: any new product surface invention; any broader
  market-facing claim beyond stitching the existing prototype wedges
  into one reviewable path.

## Bounded acceptance path (end-to-end)

The path traverses, in order, the M1 prototype wedges that operator
truth depends on. Each step names the **upstream wedge** the reviewer
inspects and the **cue** that must be visibly anchored before the
walkthrough advances.

1. **Entry — open the destructive core wedge.** The shell opens the
   bounded preview/apply/revert wedge against three workspace-local
   targets seeded with text containing `legacy_fn`. Verify the
   prototype label and lineage block are surfaced verbatim.
   - Wedge: [`crates/aureline-shell/src/review_preview/`](../../../crates/aureline-shell/src/review_preview/mod.rs).
   - Reviewer doc: [`docs/ux/m1_preview_apply_revert_core_path.md`](../../ux/m1_preview_apply_revert_core_path.md).
   - Fixture: [`fixtures/ux/preview_apply_cases/destructive_core_path/protected_walk.json`](../../../fixtures/ux/preview_apply_cases/destructive_core_path/protected_walk.json).
2. **Edit / save — propose a bulk replace.** `legacy_fn` →
   `modern_fn` across the three targets. The proposal MUST advertise
   `consequence_class = destructive_reversible_with_checkpoint` and
   `revert_class = restore_from_checkpoint`. `packet_id` and
   `proposal_id` are minted and stay bound through every later phase.
3. **Preview.** The wedge computes the per-target diff and reports
   `apply_admissibility = admitted`,
   `overall_basis_drift_state = no_drift`,
   `total_match_count = 4`. `preview_id` is minted.
4. **Permission review — typed permission prompt on the
   ecosystem-bearing install-review path.** Confirm the typed prompt
   answers, verbatim, the six required questions
   (`who_is_asking`, `what_boundary`, `why_needed`,
   `what_changes_if_allowed`, `what_works_if_denied`,
   `grant_persistence_statement`) and that the install-review fact
   grid the prompt is bound to renders cleanly.
   - Wedge: [`crates/aureline-shell/src/permission_prompts/`](../../../crates/aureline-shell/src/permission_prompts/mod.rs).
   - Reviewer doc: [`docs/ux/m1_typed_permission_prompt.md`](../../ux/m1_typed_permission_prompt.md).
   - Fixture: [`fixtures/permissions/publish_or_ecosystem_cases/protected_walk_verified_publisher_approve.json`](../../../fixtures/permissions/publish_or_ecosystem_cases/protected_walk_verified_publisher_approve.json).
5. **Install-review fact grid — bound to the same install path.**
   Verify the install-review fact grid renders publisher facts,
   declared permissions with non-empty rationales, the
   declared-vs-effective permission diff, the activation budget, the
   rollback posture, and the typed `install_decision_class = admit`
   without firing any invariant.
   - Wedge: [`crates/aureline-shell/src/install_review_fact_grid/`](../../../crates/aureline-shell/src/install_review_fact_grid/mod.rs).
   - Reviewer doc: [`docs/ux/m1_install_review_fact_grid_seed.md`](../../ux/m1_install_review_fact_grid_seed.md).
   - Fixture: [`fixtures/install/m1_fact_grid_cases/protected_walk_verified_publisher_admit.json`](../../../fixtures/install/m1_fact_grid_cases/protected_walk_verified_publisher_admit.json).
6. **Safe preview / copy-export — representation-labeled.** Confirm
   the safe-preview card surfaces representation class
   (`raw` / `sanitized` / `escaped` / `blocked_metadata_only`) and the
   copy-export representation parity rule applies on the rendered row.
   - Reviewer doc: [`docs/ux/m1_safe_preview_and_copy_export.md`](../../ux/m1_safe_preview_and_copy_export.md).
   - Crate: [`crates/aureline-preview/src/safe_preview/`](../../../crates/aureline-preview/src/safe_preview/mod.rs)
     plus the shell card consumer
     [`crates/aureline-shell/src/safe_preview_card/`](../../../crates/aureline-shell/src/safe_preview_card/mod.rs).
7. **AI composer + context-inspector evidence packet and
   route/spend truth strip.** Confirm the truth strip carries the
   eight canonical rows (`provider`, `route`, `dispatch_target`,
   `local_or_remote_path`, `spend_posture`, `run_state`,
   `context_summary`, `build_identity`) and the AI run-state class is
   `dispatch_disabled_in_m1_seed` for the protected walk.
   - Wedge: [`crates/aureline-shell/src/ai_truth_strip/`](../../../crates/aureline-shell/src/ai_truth_strip/mod.rs).
   - Composer wedge: [`crates/aureline-shell/src/ai_context_inspector/`](../../../crates/aureline-shell/src/ai_context_inspector/mod.rs).
   - Reviewer docs:
     [`docs/ai/m1_evidence_and_spend_seed.md`](../../ai/m1_evidence_and_spend_seed.md)
     and
     [`docs/ai/m1_composer_and_context_inspector_seed.md`](../../ai/m1_composer_and_context_inspector_seed.md).
   - Fixture: [`fixtures/ai/m1_evidence_and_spend_seed_cases/protected_walk_local_no_dispatch.json`](../../../fixtures/ai/m1_evidence_and_spend_seed_cases/protected_walk_local_no_dispatch.json).
8. **Boundary cues — host-boundary, target-graph readiness, managed
   lifecycle, notebook trust badges.** Confirm each certified wedge
   surfaces its own boundary truth: target-identity handoff on the
   terminal pane, target-graph readiness on the graph card, four
   visibly distinct managed workspace copy classes, and the four
   notebook trust axes.
   - Host-boundary wedge:
     [`crates/aureline-shell/src/host_boundary_cues/`](../../../crates/aureline-shell/src/host_boundary_cues/mod.rs)
     /
     [`docs/ux/m1_host_boundary_cues_seed.md`](../../ux/m1_host_boundary_cues_seed.md).
   - Graph state card wedge:
     [`crates/aureline-shell/src/graph_state_card/`](../../../crates/aureline-shell/src/graph_state_card/mod.rs)
     /
     [`docs/ux/m1_graph_state_card_seed.md`](../../ux/m1_graph_state_card_seed.md).
   - Managed-workspace lifecycle wedge:
     [`crates/aureline-shell/src/managed_workspace_labels/`](../../../crates/aureline-shell/src/managed_workspace_labels/mod.rs)
     /
     [`docs/ux/m1_managed_workspace_lifecycle_seed.md`](../../ux/m1_managed_workspace_lifecycle_seed.md).
   - Notebook trust-badges wedge:
     [`crates/aureline-shell/src/notebook_trust_badges/`](../../../crates/aureline-shell/src/notebook_trust_badges/mod.rs)
     /
     [`docs/ux/m1_notebook_trust_badges_seed.md`](../../ux/m1_notebook_trust_badges_seed.md).
9. **Apply, validate, revert.** Drive the destructive core wedge
   through apply (mints `mutation_group_id` and
   `local_history_group_id`), validate
   (`all_targets_matched = true`), and revert (`revert_id`,
   `realized_revert_class = restore_from_checkpoint`,
   `group_resolution = reverted`). Lineage stays bound across every
   transition.
10. **Acceptance evidence.** Run the acceptance smoke
    (`tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py`)
    against
    [`artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`](../../../artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml).
    Confirm every checklist row is bound to its upstream wedge's proof
    packet and at least one fixture in the repo.

## Failure drill — vocabulary drift on one wedge is caught

Re-run the integrated demo path after mutating one prototype wedge's
expected vocabulary in the acceptance checklist (for example, swap the
preview/apply/revert expected `realized_revert_class_token` from
`restore_from_checkpoint` to `compensating_action`). The smoke runner
MUST exit non-zero with
`operator_truth_demo_smoke.acceptance_row.expected_token_mismatch`
against the mutated row. A drift that fails to surface its expected
check is itself recorded as
`operator_truth_demo_smoke.failure_drill.expected_finding_missing`.

The named failure drill is replayed with:

```bash
python3 tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py \
  --repo-root . \
  --force-drill preview_apply_revert_core_path.revert_class_drift
```

## Acceptance checklist

The checklist is canonicalized at
[`artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`](../../../artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml).
Each row carries:

- a stable `row_id`;
- the bound `wedge_id` (e.g. `preview_apply_revert_core_path`,
  `typed_permission_prompts_seed`);
- the `reviewer_doc_ref` reviewers open during M1 exit review;
- the upstream `proof_packet_ref` and `validation_command` reviewers
  re-run to refresh evidence;
- the named `protected_walk_fixture_ref` the smoke replays;
- the `expected_tokens` block carrying the closed vocabulary the
  acceptance smoke checks for in that fixture;
- the named `failure_drill` (drill id + expected `check_id`) the
  smoke replays so the lane fails loudly when vocabulary drifts.

The checklist is the single source of truth for "what must be visible
on the operator-truth demo path", and it is structured so the smoke
runs unattended without a presenter-only script.

## Run the smoke unattended

```bash
python3 tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py \
  --repo-root .
```

The runner emits a durable JSON capture at
[`artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json`](../../../artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json)
and exits non-zero on any regression. The capture records, per
checklist row: the bound proof packet, the bound reviewer doc, the
fixture replayed, the closed expected tokens observed, and the
upstream `validation_command` reviewers can rerun to refresh that
row's evidence.

## Reuse-before-build map

The harness does **not** fork shared contracts. It consumes:

| Question on the demo path | Owning wedge | Upstream contract not forked |
| --- | --- | --- |
| Preview / apply / revert lineage on a destructive path | [`review_preview`](../../../crates/aureline-shell/src/review_preview/mod.rs) | [`docs/ux/preview_apply_revert_contract.md`](../../ux/preview_apply_revert_contract.md), [`crates/aureline-history/`](../../../crates/aureline-history/) |
| Typed permission prompt on the ecosystem-bearing install-review path | [`permission_prompts`](../../../crates/aureline-shell/src/permission_prompts/mod.rs) | [`crates/aureline-extensions/src/manifest_baseline/mod.rs`](../../../crates/aureline-extensions/src/manifest_baseline/mod.rs), [`docs/ux/trust_prompt_contract.md`](../../ux/trust_prompt_contract.md) |
| Install-review fact grid for the same install path | [`install_review_fact_grid`](../../../crates/aureline-shell/src/install_review_fact_grid/mod.rs) | [`crates/aureline-extensions/src/manifest_baseline/mod.rs`](../../../crates/aureline-extensions/src/manifest_baseline/mod.rs) |
| Representation-labeled safe preview / copy-export | [`safe_preview`](../../../crates/aureline-preview/src/safe_preview/mod.rs) and shell card [`safe_preview_card`](../../../crates/aureline-shell/src/safe_preview_card/mod.rs) | [`docs/ux/copy_export_representation_parity.md`](../../ux/copy_export_representation_parity.md), [`crates/aureline-content-safety/`](../../../crates/aureline-content-safety/) |
| AI evidence packet and route/spend truth strip | [`ai_truth_strip`](../../../crates/aureline-shell/src/ai_truth_strip/mod.rs) | [`crates/aureline-ai/src/composer/`](../../../crates/aureline-ai/), [`docs/ai/spend_and_route_receipt_contract.md`](../../ai/spend_and_route_receipt_contract.md) |
| Composer + context inspector seed for the AI wedge | [`ai_context_inspector`](../../../crates/aureline-shell/src/ai_context_inspector/mod.rs) | [`crates/aureline-ai/src/composer/`](../../../crates/aureline-ai/) |
| Target graph state card | [`graph_state_card`](../../../crates/aureline-shell/src/graph_state_card/mod.rs) | [`crates/aureline-graph-proto/`](../../../crates/aureline-graph-proto/), [`crates/aureline-reactive-state/`](../../../crates/aureline-reactive-state/) |
| Host-boundary cues + target-identity handoff | [`host_boundary_cues`](../../../crates/aureline-shell/src/host_boundary_cues/mod.rs) | [`crates/aureline-runtime/`](../../../crates/aureline-runtime/), [`crates/aureline-terminal/`](../../../crates/aureline-terminal/) |
| Managed workspace lifecycle labels | [`managed_workspace_labels`](../../../crates/aureline-shell/src/managed_workspace_labels/mod.rs) | [`docs/auth/managed_auth_and_session_continuity_contract.md`](../../auth/managed_auth_and_session_continuity_contract.md), [`schemas/governance/locality_tenancy_keymode.schema.json`](../../../schemas/governance/locality_tenancy_keymode.schema.json) |
| Notebook trust badges and representation-state | [`notebook_trust_badges`](../../../crates/aureline-shell/src/notebook_trust_badges/mod.rs) | [`docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`](../../notebooks/notebook_trust_and_roundtrip_preview_contract.md) |

## Out of scope

- New product-surface invention beyond stitching the existing M1
  prototype wedges into one reviewable path.
- Productized acceptance harnesses for downstream milestones — this
  page targets the M1 exit review only.
- Replacing per-wedge proof packets or validation lanes — this harness
  is additive and must not become the only proof for the underlying
  tasks.
- Any market-facing claim that goes beyond the explicit bounded
  prototype labels each upstream wedge already carries.

## Evidence sink

- Proof packet:
  [`artifacts/milestones/m1/proof_packets/operator_truth_demo_path.md`](../../../artifacts/milestones/m1/proof_packets/operator_truth_demo_path.md).
- Acceptance checklist:
  [`artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml`](../../../artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml).
- Smoke runner:
  [`tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py`](../../../tests/ux/operator_truth_demo_smoke/run_operator_truth_demo_smoke.py).
- Latest validation capture:
  [`artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json`](../../../artifacts/milestones/m1/captures/operator_truth_demo_smoke_validation_capture.json).
- Artifact-index registration:
  [`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
  (lane id `operator_truth_demo_path`).
