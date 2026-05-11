# Proof packet: host-boundary cues and target-identity handoff on one certified wedge

Purpose: anchor proof captures for the M1 bounded prototype that keeps
**host-boundary cues and target identity** legible through the lifecycle
of one certified wedge — the bottom-panel terminal pane. The wedge
records each session lifecycle event as a typed `HostBoundaryCueHandoffStep`
that preserves source / current identity, surfaces the typed boundary cue
from the shared target/origin badge vocabulary, and keeps the cue visible
across degraded, reconnecting, and policy-blocked states.

Reviewer landing page:
[`docs/ux/m1_host_boundary_cues_seed.md`](../../../../docs/ux/m1_host_boundary_cues_seed.md).

## Canonical sources

- Crate (consumer + projection): `crates/aureline-shell/`
  - `src/host_boundary_cues/mod.rs` — `HostBoundaryCueWedge`,
    `HostBoundaryCueCardRecord`, `HostBoundaryCueHandoffStep`,
    `TargetIdentitySnapshot`, `HandoffKind`,
    `HostBoundaryClaimLimit`, `HostBoundaryInvariantViolation`,
    deterministic `render_plaintext` block, and the typed `WedgeError`
    rejection set (`NotInitialized`, `AlreadyClosed`,
    `HandoffFlattensTargetIdentity`, `ReconnectIdentityMismatch`).
  - `src/host_boundary_cues/tests.rs` — unit + fixture tests covering
    the protected walk on the trusted local-desktop seed, the
    failure-drill remote handoff, the typed flatten/reconnect-mismatch
    rejections, transport-loss → reconnect, quarantine, policy-blocked,
    container handoff, serde round-trip, and the deterministic
    plaintext quote.
- Crate (shared badge vocabulary): `crates/aureline-shell/`
  - `src/badges/target_origin/mod.rs` — `TargetOriginBadge`,
    `HostBoundaryCue`, `TargetBadgeClass`, `OriginBadgeClass`,
    `BadgeEntryPoint`. The wedge projects boundary cues through this
    module rather than minting a parallel vocabulary.
- Crate (shared chrome chip vocabulary): `crates/aureline-shell/`
  - `src/state_cards/degraded_state.rs` — `DegradedStateToken`
    (`Warming` / `Offline` / `PolicyBlocked` / `Limited`).
- Crate (execution-context truth): `crates/aureline-runtime/`
  - `src/execution_context/mod.rs` — `ExecutionContext`, `TargetClass`,
    `IdentityMode`, `TrustState`, `ReachabilityState`.
- Crate (terminal session truth): `crates/aureline-terminal/`
  - `src/pty_host/mod.rs` — `PtyHost`, `PtySession`, `SessionHeader`,
    `SessionLifecycleState`, `HostClass`.
- Fixtures: `fixtures/targets/m1_target_identity_cases/`
  - `protected_walk_local_terminal.json` — trusted local-desktop seed
    renders one clean `initial_open` step (hidden cue, no degraded
    chip, no honesty marker, no invariant violations).
  - `failure_drill_local_to_remote_handoff.json` — handoff from
    `local_host` to `ssh_remote` preserves the source identity and
    lights `local_to_remote` on the new step.
  - `transport_loss_keeps_remote_cue_visible.json` — after a remote
    handoff, transport_lost keeps `boundary_cue = local_to_remote`
    visible with a typed `Offline` degraded chip and an honesty marker.
- Reviewer doc: `docs/ux/m1_host_boundary_cues_seed.md`

## Upstream contracts the wedge projects against (without forking)

- `crates/aureline-shell/src/badges/target_origin/mod.rs` (M01-078) —
  the shared target / origin / boundary-cue badge vocabulary the wedge
  reuses on every step. Forking would break the M01-078 truth-source
  guarantee.
- `crates/aureline-runtime/src/execution_context/mod.rs` — the canonical
  execution-context object that supplies target identity, identity
  mode, trust posture, and degraded-field provenance.
- `crates/aureline-terminal/src/pty_host/mod.rs` — the canonical PTY
  host / session header / lifecycle state machine the wedge mirrors.
- `crates/aureline-shell/src/state_cards/degraded_state.rs` — the
  shared `DegradedStateToken` vocabulary the wedge maps chrome chips
  into.

## Protected walk

Open the wedge on a trusted local-desktop seed and drive
`HostBoundaryCueWedge::open_initial`. The wedge MUST render one step
with `kind = initial_open`, `target_class = local_desktop`,
`boundary_cue = hidden`, no degraded chip, no honesty marker, and no
invariant violations. The card MUST carry the prototype-label chip and
the four canonical claim-limit rows (`single_certified_wedge_only`,
`no_remote_orchestration_breadth`, `no_provider_parity_implied`,
`no_transport_orchestration`) verbatim in stable order.

Evidence:

- `crates/aureline-shell/src/host_boundary_cues/tests.rs::protected_walk_local_open_renders_hidden_cue_and_clean_invariants`
- `crates/aureline-shell/src/host_boundary_cues/tests.rs::fixture_protected_walk_replays_into_the_wedge`
- Fixture: `fixtures/targets/m1_target_identity_cases/protected_walk_local_terminal.json`

## Failure drill — handoff preserves source and refuses to flatten target identity

Open a local terminal and drive
`HostBoundaryCueWedge::record_target_handoff` onto an SSH remote
target. The wedge MUST mint a `target_handoff` step whose `source`
identity quotes the prior local-desktop snapshot, set
`current.target_class = remote_host` with a different
`canonical_target_id`, and light `boundary_cue = local_to_remote`.
A call that would map source and current onto the same
`canonical_target_id` MUST be refused with
`WedgeError::HandoffFlattensTargetIdentity` and the wedge state MUST
stay unchanged.

Evidence:

- `crates/aureline-shell/src/host_boundary_cues/tests.rs::failure_drill_handoff_preserves_source_and_lights_local_to_remote`
- `crates/aureline-shell/src/host_boundary_cues/tests.rs::handoff_that_flattens_target_identity_is_rejected`
- `crates/aureline-shell/src/host_boundary_cues/tests.rs::fixture_failure_drill_handoff_preserves_source_and_lights_remote_cue`
- Fixture: `fixtures/targets/m1_target_identity_cases/failure_drill_local_to_remote_handoff.json`

## Adjacent drills — boundary cues survive degraded states

- `transport_lost_then_reconnect_keeps_cue_visible_on_remote_lane` —
  after a remote handoff, transport drops. The wedge keeps
  `boundary_cue = local_to_remote` on the `transport_lost` step,
  surfaces a typed `Offline` degraded chip, and lights
  `honesty_marker_present = true`. A subsequent reconnect-same-identity
  preserves the canonical target id.
  Fixture:
  `fixtures/targets/m1_target_identity_cases/transport_loss_keeps_remote_cue_visible.json`.
- `quarantine_keeps_remote_boundary_cue_visible_and_lights_policy_blocked_chip`
  — supervisor quarantine on a remote lane surfaces a `PolicyBlocked`
  degraded chip without erasing the `local_to_remote` cue.
- `policy_blocked_step_switches_boundary_cue_to_policy_blocked` —
  reachability is denied; the cue moves to `policy_blocked` and the
  chrome surfaces a matching degraded chip.
- `closed_step_seals_the_wedge_and_preserves_prior_identity` — closure
  is recorded with a `Limited` degraded chip; subsequent record_*
  calls return `WedgeError::AlreadyClosed`.
- `handoff_to_container_lights_local_to_container_cue` — handoff onto a
  `container_local` target lights `local_to_container` instead of
  `local_to_remote`; the shared badge vocabulary handles all three
  classes.
- `reconnect_identity_mismatch_is_rejected` — a "reconnect" with a
  different canonical target id is refused with
  `WedgeError::ReconnectIdentityMismatch`.

## Validation command

```
cargo test -p aureline-shell --lib host_boundary_cues
```

## Evidence storage

- Crate sources: `crates/aureline-shell/src/host_boundary_cues/`,
  `crates/aureline-shell/src/badges/target_origin/`,
  `crates/aureline-shell/src/state_cards/degraded_state.rs`,
  `crates/aureline-runtime/src/execution_context/mod.rs`,
  `crates/aureline-terminal/src/pty_host/mod.rs`
- Reviewer doc: `docs/ux/m1_host_boundary_cues_seed.md`
- Fixtures: `fixtures/targets/m1_target_identity_cases/`
