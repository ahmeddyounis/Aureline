# M1 host-boundary cues and target-identity handoff on one certified wedge

This page is the reviewer-facing landing page for the bounded prototype that
keeps **host-boundary cues and target identity** legible through the
lifecycle of one certified wedge — the bottom-panel terminal pane. The wedge
lives at
[`crates/aureline-shell/src/host_boundary_cues/`](../../crates/aureline-shell/src/host_boundary_cues/)
and is exercised by the unit + fixture tests in
[`crates/aureline-shell/src/host_boundary_cues/tests.rs`](../../crates/aureline-shell/src/host_boundary_cues/tests.rs)
plus the fixture suite under
[`fixtures/targets/m1_target_identity_cases/`](../../fixtures/targets/m1_target_identity_cases/).

The wedge is bounded: it covers exactly one entry point (the terminal
pane), reuses the shared
[`crate::badges::target_origin`](../../crates/aureline-shell/src/badges/target_origin/mod.rs)
projection rather than minting a parallel badge vocabulary, and does not
imply remote / provider parity across unrelated surfaces. The
`HostBoundaryClaimLimit::SingleCertifiedWedgeOnly` claim row is always
rendered to keep the boundary explicit.

## Truth sources (do not fork)

- Target class, identity mode, trust posture, working directory, and
  degraded-field provenance come from
  [`aureline_runtime::ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs).
- Session lifecycle (Requested → Starting → Active → LostTransport →
  ReconnectedSameIdentity → Closed → Quarantined) and the canonical
  session header come from
  [`aureline_terminal::PtyHost`](../../crates/aureline-terminal/src/pty_host/mod.rs).
- The badge / origin / boundary-cue vocabulary mirrors
  [`crate::badges::target_origin`](../../crates/aureline-shell/src/badges/target_origin/mod.rs)
  (`HostBoundaryCue`, `TargetBadgeClass`, `OriginBadgeClass`,
  `BadgeEntryPoint`).
- Chrome degraded chips reuse the shared
  [`crate::state_cards::DegradedStateToken`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
  vocabulary (`Warming` / `Offline` / `PolicyBlocked` / `Limited`).

When this page disagrees with those sources or the upstream PRD / TAD
under `.t2/docs/`, the upstream sources win and this page must update in
the same change.

## What the wedge owns

- One canonical
  [`HostBoundaryCueCardRecord`](../../crates/aureline-shell/src/host_boundary_cues/mod.rs)
  carrying `record_kind`, `schema_version`, `prototype_label_token`
  (always `m1_prototype_host_boundary_cues_and_target_handoff`),
  `workspace_id`, `wedge_id`, `entry_point`, the ordered `steps` list,
  the current boundary cue + degraded chip, the `claim_limits` set, and
  the typed invariant rows.
- A closed
  [`HandoffKind`](../../crates/aureline-shell/src/host_boundary_cues/mod.rs)
  vocabulary: `initial_open`, `target_handoff`,
  `reconnected_same_identity`, `transport_lost`, `quarantined`,
  `policy_blocked`, `closed`.
- A typed
  [`TargetIdentitySnapshot`](../../crates/aureline-shell/src/host_boundary_cues/mod.rs)
  shape that mirrors the upstream execution-context / session-header
  truth and is carried on every step (source + current).
- A typed
  [`HostBoundaryInvariantViolation`](../../crates/aureline-shell/src/host_boundary_cues/mod.rs)
  vocabulary: `missing_prototype_label`,
  `claim_limits_missing_or_out_of_order`,
  `missing_source_identity_on_handoff`,
  `handoff_flattens_target_identity`,
  `reconnect_identity_mismatch`,
  `boundary_cue_disappears_in_degraded_state`,
  `missing_execution_context_ref`.
- A deterministic
  [`HostBoundaryCueCardRecord::render_plaintext()`](../../crates/aureline-shell/src/host_boundary_cues/mod.rs)
  block that quotes every step, cue, degraded chip, claim limit, and
  invariant row in stable order so support exports and proof captures
  can quote the same payload.

## Protected walk

Open the terminal-pane wedge on a trusted local-desktop seed. Drive the
[`HostBoundaryCueWedge::open_initial`](../../crates/aureline-shell/src/host_boundary_cues/mod.rs)
step once. The wedge MUST:

1. Mint a single step with `kind = initial_open` and no source identity.
2. Project `target_class = local_desktop`, `origin_class =
   account_free_local`, `boundary_cue = hidden`, and
   `honesty_marker_present = false` from the shared badge.
3. Carry the prototype-label chip and the four canonical claim-limit
   rows verbatim:
   - `single_certified_wedge_only`,
   - `no_remote_orchestration_breadth`,
   - `no_provider_parity_implied`,
   - `no_transport_orchestration`.
4. Report `has_invariant_violations = false` and `has_honesty_marker =
   false`.

Exercised by
[`protected_walk_local_open_renders_hidden_cue_and_clean_invariants`](../../crates/aureline-shell/src/host_boundary_cues/tests.rs)
and the fixture
[`fixtures/targets/m1_target_identity_cases/protected_walk_local_terminal.json`](../../fixtures/targets/m1_target_identity_cases/protected_walk_local_terminal.json).

## Failure drill — handoff across host or target boundaries

Open a local terminal, then drive
[`HostBoundaryCueWedge::record_target_handoff`](../../crates/aureline-shell/src/host_boundary_cues/mod.rs)
onto an SSH remote target. The wedge MUST:

- Mint a `target_handoff` step whose `source` identity quotes the prior
  local-desktop snapshot (`target_class = local_desktop`,
  `canonical_target_id = localhost:...`).
- Set the step's `current.target_class = remote_host` with a different
  `canonical_target_id`.
- Light `boundary_cue = local_to_remote` and `boundary_cue_visible =
  true`.
- Refuse to flatten the identity: a call that would map source and
  current onto the same `canonical_target_id` returns
  `WedgeError::HandoffFlattensTargetIdentity` and the wedge state stays
  unchanged.

Exercised by
[`failure_drill_handoff_preserves_source_and_lights_local_to_remote`](../../crates/aureline-shell/src/host_boundary_cues/tests.rs),
the rejection drill
[`handoff_that_flattens_target_identity_is_rejected`](../../crates/aureline-shell/src/host_boundary_cues/tests.rs),
and the fixture
[`fixtures/targets/m1_target_identity_cases/failure_drill_local_to_remote_handoff.json`](../../fixtures/targets/m1_target_identity_cases/failure_drill_local_to_remote_handoff.json).

## Adjacent drills — boundary cues survive degraded states

The spec requires the cue to remain visible through degraded,
reconnecting, and policy-blocked states. The wedge enforces this through
both the boundary-cue selection logic and the
`BoundaryCueDisappearsInDegradedState` invariant:

- `transport_lost_then_reconnect_keeps_cue_visible_on_remote_lane` —
  after a remote handoff, drop transport. The `transport_lost` step
  keeps `boundary_cue = local_to_remote`, surfaces a typed `Offline`
  degraded chip, and lights `honesty_marker_present = true`. A
  subsequent reconnect-same-identity preserves the canonical target id.
  Fixture:
  [`transport_loss_keeps_remote_cue_visible.json`](../../fixtures/targets/m1_target_identity_cases/transport_loss_keeps_remote_cue_visible.json).
- `quarantine_keeps_remote_boundary_cue_visible_and_lights_policy_blocked_chip`
  — supervisor quarantine surfaces `PolicyBlocked` next to the row
  without erasing the `local_to_remote` cue.
- `policy_blocked_step_switches_boundary_cue_to_policy_blocked` —
  reachability is denied: the cue moves to `policy_blocked` and the
  chrome surfaces a matching degraded chip.
- `handoff_to_container_lights_local_to_container_cue` — handoff onto a
  `container_local` target lights `local_to_container` instead of
  `local_to_remote`; the shared badge vocabulary handles all three
  classes.
- `reconnect_identity_mismatch_is_rejected` — a "reconnect" with a
  different canonical target id is refused with
  `WedgeError::ReconnectIdentityMismatch` so chrome cannot smuggle a
  fresh open past the reconnect path.

## Bounded scope (deliberately)

- **One certified wedge only.** The wedge instantiates on the bottom-panel
  terminal pane. Task / debug-prep / provider entry points consume the
  shared
  [`crate::badges::target_origin::TargetOriginBadgeSet`](../../crates/aureline-shell/src/badges/target_origin/mod.rs)
  projection but do not own a per-step handoff record here.
- **No transport orchestration.** The wedge records identity through the
  lifecycle; it does not spawn, attach, or reconnect real PTY transport.
  The canonical `aureline_terminal::PtyHost` drives lifecycle
  transitions; the wedge mirrors them.
- **No provider/auth parity implied.** The shared badge module already
  projects a provider-entry slot from
  [`aureline_auth::BrowserCallbackPacket`](../../crates/aureline-auth/src/browser_callback/mod.rs).
  This wedge does not duplicate that surface; the
  `no_provider_parity_implied` claim row is always rendered to make the
  boundary explicit.
- **Out of scope:** remote-runtime / multi-host orchestration depth,
  fleet discovery, autonomous reconnect strategies, AI apply on remote
  targets, and broader market-facing "remote development" claims.

## Validation command

```
cargo test -p aureline-shell --lib host_boundary_cues
```
