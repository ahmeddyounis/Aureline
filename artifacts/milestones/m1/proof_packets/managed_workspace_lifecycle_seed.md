# Proof packet: managed-workspace lifecycle labels on one bounded certified wedge

Purpose: anchor proof captures for the M1 bounded prototype that mints
**managed-workspace lifecycle labels** on one certified wedge — the
managed-workspace state card. The wedge records each managed-workspace
lifecycle event as a typed `ManagedWorkspaceLifecycleStep` that names
the lifecycle label, the workspace-copy class, the chrome degraded
chip, the recovery action offered, and the typed
`ManagedAuthorityLineage` quoted from the upstream
locality/tenancy/key-mode vocabulary. The wedge keeps the four
workspace-copy classes the spec requires visibly distinct
(`live_environment`, `snapshot_only_view`, `suspended_workspace`,
`fresh_reprovisioned_copy`) and refuses to skip lifecycle transitions.

Reviewer landing page:
[`docs/ux/m1_managed_workspace_lifecycle_seed.md`](../../../../docs/ux/m1_managed_workspace_lifecycle_seed.md).

## Canonical sources

- Crate (wedge module): `crates/aureline-shell/`
  - `src/managed_workspace_labels/mod.rs` — `ManagedWorkspaceLifecycleWedge`,
    `ManagedWorkspaceLifecycleCardRecord`,
    `ManagedWorkspaceLifecycleStep`, `ManagedLifecycleLabelClass`,
    `WorkspaceCopyClass`, `RecoveryActionClass`,
    `ManagedAuthorityLineage`, `ManagedLifecycleClaimLimit`,
    `ManagedLifecycleInvariantViolation`, deterministic
    `render_plaintext` block, and the typed `WedgeError` rejection set
    (`NotInitialized`, `AlreadyClosed`, `SkippedTransition`,
    `ReconnectWithoutPriorConnection`, `ReprovisionWithoutPriorPause`,
    `EmptyAuthorityLineage`).
  - `src/managed_workspace_labels/tests.rs` — unit + fixture tests
    covering the protected walk, the failure drill (reconnect →
    read-only degraded → suspended → reprovisioning → connecting →
    warming → ready on the fresh copy), the typed-invariant drill for
    a hand-patched skipped transition, API-side rejection drills for
    skipped transitions / reconnect-without-prior-connection /
    reprovision-without-prior-pause / empty-lineage, the
    snapshot-only-view drill that keeps writes blocked, the
    `closed`-seals-the-wedge drill, the deterministic plaintext quote,
    and serde round-trip.
- Crate (shared degraded-chip vocabulary): `crates/aureline-shell/`
  - `src/state_cards/degraded_state.rs` — `DegradedStateToken`
    (`Warming` / `Offline` / `PolicyBlocked` / `Limited` / `Stale`).
- Upstream contract (managed identity / sign-in / continuity):
  `docs/auth/managed_auth_and_session_continuity_contract.md` — the
  managed-session vocabulary the wedge labels lifecycle against. The
  wedge does not implement managed sign-in.
- Upstream contract (locality / tenancy / key-mode vocabulary):
  `schemas/governance/locality_tenancy_keymode.schema.json` — the
  closed `identity_mode_token` / `locality_class_token` tokens carried
  on every step's `ManagedAuthorityLineage` row.
- Upstream contract (local workspace lifecycle):
  `crates/aureline-workspace/src/lifecycle/mod.rs` —
  `WorkspaceLifecycleState`. The managed wedge labels the *managed*
  subset and does not duplicate the local lifecycle vocabulary.
- Fixtures: `fixtures/managed_workspace/m1_lifecycle_cases/`
  - `protected_walk_managed_lifecycle.json` — happy-path managed
    workspace seed renders four clean steps with every workspace-copy
    class flipping honestly from `not_yet_admitted_copy_class` to
    `live_environment` only once `ready` lands.
  - `failure_drill_reconnect_then_reprovision.json` — transport drop →
    read-only degraded → suspended → reprovisioning → connecting →
    warming → ready keeps all four workspace-copy classes visibly
    distinct and fires no invariant violations.
  - `snapshot_only_view_keeps_writes_blocked.json` — `snapshot_only_view`
    after `ready` keeps `admits_writes = false` and surfaces
    `continue_in_snapshot_view` with the `Stale` degraded chip.
- Reviewer doc: `docs/ux/m1_managed_workspace_lifecycle_seed.md`

## Upstream contracts the wedge projects against (without forking)

- `docs/auth/managed_auth_and_session_continuity_contract.md` — the
  managed-session vocabulary, system-browser-first sign-in posture, and
  local-continuity guarantees the wedge labels lifecycle against.
- `schemas/governance/locality_tenancy_keymode.schema.json` — the
  closed locality / tenancy / key-mode vocabulary mirrored on every
  step's authority lineage row.
- `crates/aureline-workspace/src/lifecycle/mod.rs` — the canonical
  *local* workspace lifecycle the managed wedge sits alongside without
  duplicating.
- `crates/aureline-shell/src/state_cards/degraded_state.rs` — the
  shared `DegradedStateToken` vocabulary the wedge maps chrome chips
  into.

## Protected walk

Drive `open_authenticating` → `record_connecting` → `record_warming` →
`record_ready` on a managed workspace seed. The wedge MUST mint four
steps in canonical order, flip `copy_class = live_environment` only
once `ready` lands, carry the prototype-label chip and the four
canonical claim-limit rows (`single_certified_wedge_only`,
`no_managed_control_plane_in_m1`, `no_tenancy_orchestration`,
`no_lifecycle_executor_productization`) verbatim, and report
`has_invariant_violations = false` plus `current_admits_writes = true`
on the final step.

Evidence:

- `crates/aureline-shell/src/managed_workspace_labels/tests.rs::protected_walk_renders_clean_card_with_visible_labels`
- `crates/aureline-shell/src/managed_workspace_labels/tests.rs::fixture_protected_walk_replays_into_the_wedge`
- Fixture: `fixtures/managed_workspace/m1_lifecycle_cases/protected_walk_managed_lifecycle.json`

## Failure drill — lifecycle advances honestly without skipped transitions

Drive the failure-drill sequence (transport drop → read-only degraded →
suspended → reprovisioning → re-connect → re-warm → re-ready on the
fresh copy). The wedge MUST visit all four workspace-copy classes
(`live_environment`, `snapshot_only_view`, `suspended_workspace`,
`fresh_reprovisioned_copy`) and carry a non-`none` recovery action on
every degraded posture row. A call that would skip a required
transition MUST be refused with `WedgeError::SkippedTransition`. A
reconnect without a prior `ready`/`read_only_degraded` step MUST be
refused with `WedgeError::ReconnectWithoutPriorConnection`. A
reprovision without a prior `suspended`/`read_only_degraded` step MUST
be refused with `WedgeError::ReprovisionWithoutPriorPause`. A buggy
caller that hand-patches a `connecting → ready` jump onto the rendered
card MUST surface the typed `SkippedTransition` invariant on the
serialized payload.

Evidence:

- `crates/aureline-shell/src/managed_workspace_labels/tests.rs::failure_drill_walks_reconnect_through_reprovisioning_back_to_ready`
- `crates/aureline-shell/src/managed_workspace_labels/tests.rs::skipping_warming_is_refused_at_the_api`
- `crates/aureline-shell/src/managed_workspace_labels/tests.rs::reconnect_without_prior_connection_is_refused`
- `crates/aureline-shell/src/managed_workspace_labels/tests.rs::reprovision_without_prior_pause_is_refused`
- `crates/aureline-shell/src/managed_workspace_labels/tests.rs::hand_patched_skipped_transition_surfaces_typed_invariant`
- `crates/aureline-shell/src/managed_workspace_labels/tests.rs::fixture_failure_drill_reconnect_then_reprovision_replays_into_the_wedge`
- Fixture: `fixtures/managed_workspace/m1_lifecycle_cases/failure_drill_reconnect_then_reprovision.json`

## Adjacent drills

- `snapshot_only_view_after_ready_keeps_writes_blocked` — moving a
  `ready` lane to `snapshot_only_view` keeps `admits_writes = false`,
  surfaces `continue_in_snapshot_view` as the recovery action, and
  carries the `Stale` degraded chip without firing any invariant.
- `degraded_step_must_carry_recovery_action` — a degraded posture step
  whose recovery action is stripped to `None` surfaces the typed
  `missing_recovery_action_for_degraded_state` invariant.
- `empty_lineage_is_refused` — `open_authenticating` against an empty
  authority lineage returns `WedgeError::EmptyAuthorityLineage`.
- `closed_step_seals_wedge` — closure is recorded with a `Limited`
  degraded chip; subsequent `record_*` calls return
  `WedgeError::AlreadyClosed`.

## Validation command

```
cargo test -p aureline-shell --lib managed_workspace_labels
```

## Evidence storage

- Crate sources: `crates/aureline-shell/src/managed_workspace_labels/`,
  `crates/aureline-shell/src/state_cards/degraded_state.rs`
- Reviewer doc: `docs/ux/m1_managed_workspace_lifecycle_seed.md`
- Fixtures: `fixtures/managed_workspace/m1_lifecycle_cases/`
