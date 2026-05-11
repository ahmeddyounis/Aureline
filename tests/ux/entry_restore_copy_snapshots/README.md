# Entry / restore placeholder audit and truth-state copy review

Unattended runner that walks the protected first-run, open, import, and
restore dogfood paths and asserts every placeholder, restore prompt,
recent-work row, and degraded-state copy quotes the agreed truth
vocabulary (verbs, blocked capabilities, recovery-ladder rungs,
support-packet families, journey classes, protected metrics) instead of
hiding missing roots, stale restores, or crash conditions behind ready-
sounding labels.

## Run the protected walk

```bash
python3 tests/ux/entry_restore_copy_snapshots/run_entry_restore_copy_audit.py \
  --repo-root .
```

The command emits a durable JSON capture at
`artifacts/milestones/m1/captures/entry_restore_truth_audit_validation_capture.json`
and exits non-zero on any regression. The capture records, per case:

- the bound `startup_state_token` and `dogfood_path_class`;
- the resolved source-fixture summary
  (`next_step_decision_hooks`, `blocked_capability_tokens`,
  `recovery_ladder_rung_refs`, `support_packet_family_refs`,
  `journey_classes`, `protected_metric_refs`);
- the matching copy-review row's
  `forbidden_labels` and hook set;
- every dogfood copy snapshot inspected (path, fields, matches).

## Run a named failure drill

Each case under `fixtures/ux/entry_restore_placeholder_cases/` declares
one failure drill. The drill mutates the case state (drops a required
hook, drops a blocked-capability token, forces `overclaims_readiness`,
or injects a forbidden label into a snapshot field) and the runner
verifies the audit reports the expected `check_id`:

```bash
python3 tests/ux/entry_restore_copy_snapshots/run_entry_restore_copy_audit.py \
  --repo-root . \
  --force-drill first_run_inject_workspace_ready_label
```

The drill exits 0 only if the expected `check_id` was actually reported.
A drill that fails to surface its expected check is itself a failure
mode — the runner records it as
`entry_restore_copy_audit.failure_drill.expected_finding_missing`.

## Required coverage

The audit refuses to pass unless cases collectively cover:

- dogfood path classes `first_run`, `open`, `restore`,
  `placeholder_transition`;
- entry verbs `open`, `clone`, `import`, `restore`.

## Wiring

| Input  | Path                                                                                  |
| ------ | ------------------------------------------------------------------------------------- |
| Cases  | `fixtures/ux/entry_restore_placeholder_cases/*.yaml`                                  |
| Seeds  | `fixtures/ux/entry_restore_states/<state>.yaml`                                       |
| Review | `artifacts/ux/startup_state_copy_review.yaml`                                         |
| Sink   | `artifacts/milestones/m1/captures/entry_restore_truth_audit_validation_capture.json`  |
| Packet | `artifacts/milestones/m1/proof_packets/entry_restore_truth_audit.md`                  |
| Index  | `artifacts/milestones/m1/artifact_index.yaml#entry_restore_truth_audit`               |
| Page   | `artifacts/ux/m1/entry_restore_truth_audit.md`                                        |
