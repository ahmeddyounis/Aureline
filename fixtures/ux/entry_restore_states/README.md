# Entry / restore placeholder truth fixtures

Seed corpus for the audit frozen in
[`/docs/ux/entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md)
and the copy-review rows in
[`/artifacts/ux/startup_state_copy_review.yaml`](../../../artifacts/ux/startup_state_copy_review.yaml).

Each file is a single YAML document describing one placeholder
startup state the dogfood shell is allowed to render before
the full Start Center, recovery ladder, and migration-center
surfaces ship. A fixture is a **seed**: it pins the truthful
cause, user impact, next-safe action hooks, blocked-capability
axis, recovery-ladder and support-packet linkage, measurement
hooks, and the build-identity / fixture-path refs a later
dogfood telemetry or journey-trace lane will fire against.

Every fixture:

- Pins `startup_state` to the row token defined in the audit
  (`startup_state:first_run`,
  `startup_state:reopen_with_pending_restore`,
  `startup_state:restore_failed`,
  `startup_state:restore_skipped`,
  `startup_state:open_without_restore`,
  `startup_state:warming_startup`,
  `startup_state:partial_startup`,
  `startup_state:offline_startup`,
  `startup_state:unsupported_startup`,
  `startup_state:empty_state_or_placeholder_transition`).
- Resolves every axis to vocabulary re-exported from the
  entry-restore object model §1–§4 and the onboarding
  measurement plan §3–§4.
- Cites at least one `next_step_decision_hook`, one
  recovery-ladder rung id (or `rung.none_required`), one
  support-packet family, one journey-trace `journey_class`,
  and one protected-metric row id the future telemetry /
  journey-trace will fire against.
- Carries no raw absolute paths, raw URLs, raw credential
  material, raw prompt text, or raw logs. Every identity is
  an opaque ref; every timestamp is a monotonic placeholder.
- Asserts `overclaims_readiness = false`. A fixture that
  pins this field to `true` is non-conforming; the audit
  contract forbids placeholder states that imply readiness
  the underlying state does not hold.

## Cases

| Fixture | Startup state | Key truthful-cause tokens | Blocked capability |
| --- | --- | --- | --- |
| [`first_run.yaml`](./first_run.yaml) | `startup_state:first_run` | no prior session; `admitted` first-run landing on Start Center | `no_durable_edits_yet` |
| [`reopen_with_pending_restore.yaml`](./reopen_with_pending_restore.yaml) | `startup_state:reopen_with_pending_restore` | `restore_prompt_record` pending, `compatible_restore` advertised | `index_not_authoritative` |
| [`restore_failed.yaml`](./restore_failed.yaml) | `startup_state:restore_failed` | `corrupt_restorable_state`; advertised level degraded to `no_restore` | `prior_work_requires_manual_recovery` |
| [`restore_skipped.yaml`](./restore_skipped.yaml) | `startup_state:restore_skipped` | user declined restore prompt; recovery journal retained | `prior_work_preserved_as_evidence_only` |
| [`open_without_restore.yaml`](./open_without_restore.yaml) | `startup_state:open_without_restore` | explicit `open_without_restore` decision hook fired on a restore-adjacent path | `index_not_authoritative` |
| [`warming_startup.yaml`](./warming_startup.yaml) | `startup_state:warming_startup` | `first_useful_navigation_reached` fired before `semantic_warmup_completed` | `semantic_lookups_pending` |
| [`partial_startup.yaml`](./partial_startup.yaml) | `startup_state:partial_startup` | `missing_extension_host` and `managed_workspace_not_ready` on a managed-cloud reopen | `extension_host_offline` |
| [`offline_startup.yaml`](./offline_startup.yaml) | `startup_state:offline_startup` | `remote_unreachable` on a remote-repo recent-work row; local-only rows remain reachable | `remote_target_unreachable` |
| [`unsupported_startup.yaml`](./unsupported_startup.yaml) | `startup_state:unsupported_startup` | `policy_blocked_restore` plus `binary_or_extension_version_changed` on a policy-managed workspace | `policy_restricted_mode` |
| [`empty_state_or_placeholder_transition.yaml`](./empty_state_or_placeholder_transition.yaml) | `startup_state:empty_state_or_placeholder_transition` | `missing_extension_placeholder` zone plus unresolved protocol-handler-reentry card | `placeholder_surface_no_capability` |

## Schema references

- Entry / restore vocabulary:
  [`/schemas/workspace/entry_and_restore_result.schema.json`](../../../schemas/workspace/entry_and_restore_result.schema.json).
- Journey-trace measurement hooks:
  [`/schemas/traces/journey_trace.schema.json`](../../../schemas/traces/journey_trace.schema.json).
- Protected-metric registry:
  [`/artifacts/bench/protected_metrics.yaml`](../../../artifacts/bench/protected_metrics.yaml).

## Build identity

Every fixture carries a `running_build_identity_ref` reserved
for later exact-build-identity wiring (see
[`docs/build/exact_build_identity_model.md`](../../../docs/build/exact_build_identity_model.md)
if / when landed). Today the ref is an opaque seed id; a later
lane resolves it against the fixture / build-identity record
without renaming the field.
