# Entry / restore placeholder audit cases

Seed corpus for the unattended audit at
[`tests/ux/entry_restore_copy_snapshots/run_entry_restore_copy_audit.py`](../../../tests/ux/entry_restore_copy_snapshots/run_entry_restore_copy_audit.py)
that walks the protected first-run, open, import, and restore dogfood
paths and asserts every placeholder, restore prompt, recent-work row,
and degraded-state surface quotes the agreed truth vocabulary instead
of hiding missing roots, stale restores, or crash conditions behind
ready-sounding labels.

Each case file is a single YAML document binding one audited
`startup_state` row (per
[`docs/ux/entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md)
and the copy-review registry at
[`artifacts/ux/startup_state_copy_review.yaml`](../../../artifacts/ux/startup_state_copy_review.yaml))
to:

- the upstream seed fixture under
  [`fixtures/ux/entry_restore_states/`](../entry_restore_states/);
- the copy-review row id (`ux:startup-state:*`);
- the closed set of required next-safe-action hooks, blocked-capability
  tokens, recovery-ladder rungs, support-packet families, journey
  classes, and protected-metric refs the placeholder owes its user;
- the list of forbidden label fragments that would imply readiness the
  underlying state does not hold;
- the dogfood copy snapshots — rendered surface fixtures whose copy
  fields MUST NOT contain any forbidden label fragment;
- a named **failure drill** that mutates the case state so the audit
  reports a precise `check_id` rather than silently passing.

Each row pins `record_kind: entry_restore_copy_audit_case_record` and
`schema_version: 1`. No row mints private state tokens; every value
resolves through the frozen vocabularies re-exported by
`artifacts/ux/startup_state_copy_review.yaml`.

## Cases

| File | Startup state | Dogfood path |
| --- | --- | --- |
| [`first_run.yaml`](./first_run.yaml) | `startup_state:first_run` | first_run |
| [`reopen_with_pending_restore.yaml`](./reopen_with_pending_restore.yaml) | `startup_state:reopen_with_pending_restore` | restore |
| [`restore_failed.yaml`](./restore_failed.yaml) | `startup_state:restore_failed` | restore |
| [`restore_skipped.yaml`](./restore_skipped.yaml) | `startup_state:restore_skipped` | restore |
| [`open_without_restore.yaml`](./open_without_restore.yaml) | `startup_state:open_without_restore` | open |
| [`warming_startup.yaml`](./warming_startup.yaml) | `startup_state:warming_startup` | open |
| [`partial_startup.yaml`](./partial_startup.yaml) | `startup_state:partial_startup` | open |
| [`offline_startup.yaml`](./offline_startup.yaml) | `startup_state:offline_startup` | open |
| [`unsupported_startup.yaml`](./unsupported_startup.yaml) | `startup_state:unsupported_startup` | open |
| [`empty_state_or_placeholder_transition.yaml`](./empty_state_or_placeholder_transition.yaml) | `startup_state:empty_state_or_placeholder_transition` | placeholder_transition |

The `entry_verbs_covered` axis across the corpus exercises `open`,
`clone`, `import`, and `restore` so the protected walk reaches the
four dogfood paths the M1 exit gate names.
