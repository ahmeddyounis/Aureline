# Shell/editor/workspace integration checkpoint

This page is the reviewer-facing entry point for the shell/editor/workspace
integration checkpoint. The canonical truth source is
`artifacts/milestones/m1/dependency_graph.yaml`; downstream packets, dashboards,
and checklists should reference that file instead of restating prerequisites.

## Canonical artifacts

- Human entry (this file): `docs/milestones/m1/integration_checkpoint.md`
- Dependency graph (canonical): `artifacts/milestones/m1/dependency_graph.yaml`
- Proof index (first consumer): `artifacts/milestones/m1/artifact_index.yaml`
- Validator: `ci/check_m1_checkpoint.py`

## Definition of green

The checkpoint is the single integrated path:

Start Center -> open repo -> edit -> save -> restore session.

The dependency graph freezes:

- required subsystem owners for shell, editor, workspace, telemetry, and recovery;
- the blocking contracts/schemas those subsystems must share; and
- the protected fixture lane that must be able to exercise the flow end-to-end.

## How to validate

Run:

`python3 ci/check_m1_checkpoint.py --repo-root .`

Optional machine-readable report:

`python3 ci/check_m1_checkpoint.py --repo-root . --report target/m1-checkpoint/report.json`

## Update rules

1. Update `artifacts/milestones/m1/dependency_graph.yaml` first.
2. Run the validator locally.
3. If artifact locations, ownership, or consumer rules changed, update
   `artifacts/milestones/m1/artifact_index.yaml` in the same change.

## Failure drill

To confirm the guardrail is live, temporarily remove one required field (for
example: a subsystem owner, a required contract ref, or the fixture-lane ref)
from the dependency graph and rerun the validator; it must fail with an
actionable error.

## Key contracts (non-exhaustive)

- `docs/workspace/entry_restore_object_model.md`
- `docs/workspace/source_acquisition_and_bootstrap_seed.md`
- `docs/recovery/restore_chooser_contract.md`
- `docs/verification/source_fidelity_and_undo_packet.md`
- `docs/build/exact_build_identity_model.md`
- `docs/observability/observability_signal_contract.md`
- `docs/observability/replay_and_trace_bundle_contract.md`
- `artifacts/qe/test_lane_registry.yaml#fixture_repo_integration`
- `artifacts/governance/interface_freeze_matrix.yaml#integration_checkpoint.m1_shell_editor_workspace`
- `artifacts/governance/ownership_matrix.yaml`
