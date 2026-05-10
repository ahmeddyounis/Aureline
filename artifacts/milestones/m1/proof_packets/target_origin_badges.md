# Proof packet: target / origin badges and host-boundary cues on terminal, task, debug-prep, and provider/auth entry points

Purpose: anchor proof captures for the M1 seed that surfaces target / origin
badges and host-boundary cues on every run-capable seed entry point. The seed
keeps one badge vocabulary across the bottom-panel terminal pane, the task
seed, the debug-prep seed, and the provider / auth entry point so a remote
host, a managed plane, a devcontainer, or a managed sign-in tenant cannot
look like an ordinary local desktop launch merely because the surface was
crowded.

Reviewer landing page: [`docs/ux/target_origin_badges.md`](../../../docs/ux/target_origin_badges.md).

Canonical sources:

- Crate: `crates/aureline-shell/`
  - `src/lib.rs` — exposes the `badges` module.
  - `src/badges/mod.rs` — module-level summary.
  - `src/badges/target_origin/mod.rs` — `TargetOriginBadge`,
    `TargetOriginBadgeSet`, `BadgeEntryPoint`, `TargetBadgeClass`,
    `OriginBadgeClass`, `HostBoundaryCue`, and the deterministic
    boundary-cue precedence (`unknown_boundary` >
    `policy_blocked` > `degraded_trust` > target-class boundary >
    `local_to_provider` for non-local-only provider entries > hidden).
  - `src/badges/target_origin/tests.rs` — protected walk + failure drills
    (target-class mirror, remote-target consistency, pending-trust honesty,
    managed provider entry, fixture replay).
- Upstream contracts (no fork; mirrored verbatim):
  - `crates/aureline-runtime/src/execution_context/mod.rs` —
    `ExecutionContext`, `TargetClass`, `IdentityMode`, `TrustState`,
    `DegradedFieldReason`.
  - `crates/aureline-auth/src/browser_callback/mod.rs` —
    `BrowserCallbackPacket`, `AccountBoundaryClass`, `IdentityModeAlias`.
- Seed fixtures:
  - `fixtures/runtime/target_origin_cases/local_terminal_protected_walk.json`
    — protected walk on a trusted local desktop seed.
  - `fixtures/runtime/target_origin_cases/remote_target_failure_drill.json`
    — failure drill on a remote SSH target with consistent boundary cue
    across terminal, task seed, and debug-prep seed.
  - `fixtures/runtime/target_origin_cases/pending_trust_honesty_drill.json`
    — honesty drill where local target + pending trust must surface
    `degraded_trust` rather than a stale `Hidden` chip.

Protected walk: project `TargetOriginBadgeSet::project(&context)` from a
trusted local-desktop execution context. Confirm every execution-entry
badge (terminal, task seed, debug-prep seed) renders `target_class =
local_desktop`, `origin_class = account_free_local`, `boundary_cue =
hidden`, and `honesty_marker_present = false`. Confirm
`execution_entries_consistent()` returns `true` and `any_honesty_marker()`
returns `false`. Evidence:
`crates/aureline-shell/src/badges/target_origin/tests.rs::protected_walk_local_seed_renders_hidden_boundary_on_every_entry_point`,
`crates/aureline-shell/src/badges/target_origin/tests.rs::fixture_protected_walk_replays_into_the_badge_projection`.

Failure drill A (consistency across entry points): override the resolver
target to `ssh_remote` and confirm the terminal, task seed, and debug-prep
seed badges all light `boundary_cue = local_to_remote`. Evidence:
`crates/aureline-shell/src/badges/target_origin/tests.rs::failure_drill_remote_target_lights_local_to_remote_consistently`,
`crates/aureline-shell/src/badges/target_origin/tests.rs::fixture_failure_drill_replays_consistent_remote_boundary_cue`.

Failure drill B (honesty marker on pending trust): resolve a local-desktop
context with `TrustState::PendingEvaluation` and confirm every badge surfaces
`boundary_cue = degraded_trust` and `honesty_marker_present = true`.
Evidence:
`crates/aureline-shell/src/badges/target_origin/tests.rs::pending_trust_lights_degraded_trust_cue_and_honesty_marker_on_every_badge`,
`crates/aureline-shell/src/badges/target_origin/tests.rs::fixture_pending_trust_replays_honesty_marker_on_every_badge`.

Failure drill C (managed provider entry on local target): attach a managed
`BrowserCallbackPacket` to a local-desktop context. Confirm the provider
badge lights `boundary_cue = local_to_provider` (because the auth tenant
boundary is crossed) while the execution-entry badges keep their local
`hidden` cue. Evidence:
`crates/aureline-shell/src/badges/target_origin/tests.rs::managed_provider_entry_on_local_target_lights_local_to_provider_cue`.

Validation commands:

```
cargo test -p aureline-shell --lib badges::target_origin
```
