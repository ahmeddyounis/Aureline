# Safe-mode runtime profile with bounded feature cuts and preserved local editing

The safe-mode runtime profile is the bounded recovery posture a blocked
user can enter after a startup crash loop, an exhausted restart budget,
a managed-policy forcing, or an explicit diagnostics request. The
profile *declares* — by name and reason — which hosts, services, and
surfaces are disabled or narrowed, and it *preserves* local editing,
basic navigation, and the diagnostics/export paths the recovery ladder
relies on. Entering or leaving safe mode is recorded as a typed
transition that pins user-owned state and durable state as undeleted,
so a blocked user can recover without manual config deletion or hidden
reruns.

The implementation lives in
[`crates/aureline-support/src/safe_mode/mod.rs`](../../../crates/aureline-support/src/safe_mode/mod.rs)
and the boundary schema lives at
[`/schemas/support/safe_mode_profile.schema.json`](../../../schemas/support/safe_mode_profile.schema.json).
The protected fixture corpus lives under
[`fixtures/recovery/m3/safe_mode/`](../../../fixtures/recovery/m3/safe_mode/).

## What this beta row owns

- A typed [`SafeModeProfile`] record that names every narrowing as a
  typed `(host|service|surface, disposition_class, reason_class,
  summary)` row. Disposition is one of `disabled`,
  `narrowed_to_read_only`, `narrowed_to_local_only`, or
  `narrowed_to_reviewer_view`; reason is one of seven closed classes
  including `startup_crash_loop_detected`, `unsafe_replay_detected`,
  `policy_forced_safe_mode`, and `user_opt_in_request`.
- A typed [`SafeModeTransition`] record for every `enter` and `exit`
  event. The record observes the preserved state classes at the
  transition boundary and pins both `user_owned_state_deleted` and
  `durable_state_deleted` to `false`.
- A [`SafeModeEvaluator`] that validates profile and transition shapes,
  cross-checks that a transition's `profile_ref` matches the bound
  profile, and projects one [`SafeModeSupportPacket`] per profile
  lifecycle. The packet excludes raw private material and ambient
  authority, quotes the doc and schema refs verbatim, and carries the
  declared narrowing rows and the matching transition rows.
- The boundary JSON schema and three canonical fixtures covering the
  `post_crash_loop_profile`, `user_invoked_profile`, and
  `policy_forced_profile` classes.

## Acceptance and how this row meets it

- **Safe mode declares exactly which hosts, services, and surfaces are
  disabled or narrowed and why.** Every `declared_hosts`,
  `declared_services`, and `declared_surfaces` row carries an opaque id,
  a closed `*_class`, a closed `disposition_class`, a closed
  `reason_class`, and a reviewer-safe `narrowing_summary`. The
  evaluator refuses a profile with empty hosts, empty services, or
  empty/duplicate ids; the schema mirrors the same shape so the chrome
  and the headless export cannot disagree on what was disabled.
- **Local editing, basic navigation, and diagnostics/export remain
  available where the architecture allows them.** Every profile
  declares a `preserved_capabilities` list. The evaluator requires the
  list to contain `local_editing`, and the canonical fixtures also
  preserve `basic_navigation`, `local_search`, `local_git_operations`,
  `local_diagnostics_export`, `support_bundle_preview`,
  `project_doctor_surfaces`, and `safe_mode_exit_action` so the
  blocked user can keep working, run Project Doctor, build a
  support-bundle preview, and then explicitly exit safe mode.
- **Safe-mode entry and exit preserve user-owned state and avoid
  destructive resets.** The profile's `destructive_resets_present`
  field is pinned to `false`. Every transition record pins
  `user_owned_state_deleted = false` and `durable_state_deleted =
  false`, and the evaluator rejects any transition that fails to
  observe `user_authored_files` in `preserved_state_classes_observed`.
  The transition's `entry_reason_class` / `exit_reason_class`
  vocabulary is closed and gated by `transition_class` so an entry
  cannot silently double as an exit.

## Failure-drill posture

The evaluator fails closed before widening any narrowing:

- A profile that declares a destructive reset is refused.
- A profile that drops `local_editing` or `user_authored_files`
  preservation is refused.
- A profile without a `doctor.finding.*` ref or without any narrowed
  hosts/services is refused.
- Duplicate `host_id`, `service_id`, or `surface_id` values are refused.
- A transition that deletes user-owned or durable state is refused.
- An enter transition that names an `exit_reason_class` (or vice
  versa) is refused.
- A transition whose `profile_ref` does not match the bound profile is
  refused at `support_packet` time.

## First consumers

- The `aureline-support` safe-mode module is the canonical projection
  for support-export and recovery-ladder review.
  `SafeModeEvaluator::support_packet` folds one profile and its
  bound transitions into a metadata-safe
  [`SafeModeSupportPacket`] that the support-export pipeline can
  serialize verbatim.
- The boundary schema is the contract the headless export writer and
  the support-export chrome share — both reconstruct the same packet
  shape from the on-disk record verbatim, never re-derive it from a
  side channel.

## Related contracts

- [Recovery-ladder alpha](../recovery_ladder_alpha.md) — the parent
  rung contract for safe mode. This beta row owns the standalone
  declarative profile and the entry/exit transitions that bind to the
  same Project Doctor finding.
- [Recovery-action schema](../../../schemas/support/recovery_action.schema.json)
  — the closed `safe_mode` rung class, `lost_capability_class`, and
  `preserved_state_class` vocabularies this beta row mirrors.
- [Support-bundle contract](../support_bundle_contract.md) — the
  parent contract for every metadata-safe support projection.

## Out of scope for this beta row

- Live runtime enforcement (host launcher, service supervisor, panel
  gating) — those bindings land with the runtime-host and chrome
  consumers that quote this profile.
- Time-bounded auto-exit from safe mode after a clean launch — the
  beta surfaces only explicit, user-confirmed return paths.
- Cross-tenant policy-forced safe-mode reconciliation; the
  `policy_forced_profile` fixture covers single-tenant entry and exit.

[`SafeModeProfile`]: ../../../crates/aureline-support/src/safe_mode/mod.rs
[`SafeModeTransition`]: ../../../crates/aureline-support/src/safe_mode/mod.rs
[`SafeModeEvaluator`]: ../../../crates/aureline-support/src/safe_mode/mod.rs
[`SafeModeSupportPacket`]: ../../../crates/aureline-support/src/safe_mode/mod.rs
