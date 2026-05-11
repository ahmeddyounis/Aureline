# M1 managed-workspace lifecycle labels on one bounded certified wedge

This page is the reviewer-facing landing page for the bounded prototype
that mints **managed-workspace lifecycle labels** on one certified path.
The wedge lives at
[`crates/aureline-shell/src/managed_workspace_labels/`](../../crates/aureline-shell/src/managed_workspace_labels/)
and is exercised by the unit + fixture tests in
[`crates/aureline-shell/src/managed_workspace_labels/tests.rs`](../../crates/aureline-shell/src/managed_workspace_labels/tests.rs)
plus the fixture suite under
[`fixtures/managed_workspace/m1_lifecycle_cases/`](../../fixtures/managed_workspace/m1_lifecycle_cases/).

The wedge is bounded: it covers exactly one prototype path (the
managed-workspace state card), labels lifecycle truth supplied by the
caller without running a managed control-plane, and never implies broad
managed parity across unrelated surfaces. The
`ManagedLifecycleClaimLimit::SingleCertifiedWedgeOnly` claim row is
always rendered to keep the boundary explicit.

## Truth sources (do not fork)

- Managed-session vocabulary, system-browser auth handoff, and
  account/seat/deprovision posture come from the upstream
  [managed authentication and session-continuity contract](../auth/managed_auth_and_session_continuity_contract.md)
  and the
  [`crates/aureline-auth`](../../crates/aureline-auth/) browser-callback
  surface. The wedge does not implement managed sign-in; it labels the
  lifecycle the caller drives.
- Locality / tenancy / key-mode vocabulary mirrors
  [`schemas/governance/locality_tenancy_keymode.schema.json`](../../schemas/governance/locality_tenancy_keymode.schema.json)
  on every step's `ManagedAuthorityLineage` row. The
  `identity_mode_token` and `locality_class_token` fields quote that
  vocabulary verbatim.
- Local workspace lifecycle (`discovered → trust_evaluating → opening →
  partially_ready → ready → degraded → closing → closed`) is owned by
  [`aureline_workspace::WorkspaceLifecycleState`](../../crates/aureline-workspace/src/lifecycle/mod.rs).
  This wedge labels the *managed* lifecycle subset and does not
  duplicate the local lifecycle vocabulary.
- Chrome degraded chips reuse the shared
  [`crate::state_cards::DegradedStateToken`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
  vocabulary (`Warming` / `Offline` / `Limited` / `Stale`).

When this page disagrees with those sources or the upstream PRD / TAD
under `.t2/docs/`, the upstream sources win and this page must update in
the same change.

## What the wedge owns

- One canonical
  [`ManagedWorkspaceLifecycleCardRecord`](../../crates/aureline-shell/src/managed_workspace_labels/mod.rs)
  carrying `record_kind`, `schema_version`, `prototype_label_token`
  (always `m1_prototype_managed_workspace_lifecycle_labels`),
  `workspace_id`, `wedge_id`, the ordered `steps` list, the current
  lifecycle label / copy class / recovery action / degraded chip / write
  admissibility, the `claim_limits` set, and the typed invariant rows.
- A closed
  [`ManagedLifecycleLabelClass`](../../crates/aureline-shell/src/managed_workspace_labels/mod.rs)
  vocabulary the chrome quotes verbatim: `authenticating`,
  `connecting`, `warming`, `ready`, `reconnecting`, `read_only_degraded`,
  `suspended`, `reprovisioning`, `snapshot_only_view`, `closed`. Only
  `ready` admits writes; every other label sets `admits_writes = false`.
- A closed
  [`WorkspaceCopyClass`](../../crates/aureline-shell/src/managed_workspace_labels/mod.rs)
  vocabulary that distinguishes the four copies the spec requires the
  wedge to keep visibly distinct: `live_environment`,
  `snapshot_only_view`, `suspended_workspace`, `fresh_reprovisioned_copy`
  (plus the `not_yet_admitted_copy_class` sentinel used during
  authenticating / connecting / warming).
- A closed
  [`RecoveryActionClass`](../../crates/aureline-shell/src/managed_workspace_labels/mod.rs)
  vocabulary: `reauthenticate_via_system_browser`, `retry_connection`,
  `wait_for_warm`, `resume_from_suspension`,
  `accept_reprovisioned_state`, `continue_in_snapshot_view`,
  `export_local_safe_artifacts`, `none`. Every degraded posture row
  carries a non-`none` recovery action; the invariant validator surfaces
  `missing_recovery_action_for_degraded_state` if a buggy caller drops
  it.
- A typed
  [`ManagedAuthorityLineage`](../../crates/aureline-shell/src/managed_workspace_labels/mod.rs)
  row carried on every step: `workspace_id` + `managed_tenant_ref` +
  `identity_mode_token` + `locality_class_token`, all required, all
  quoted from the upstream locality/tenancy/key-mode vocabulary rather
  than minted locally.
- A closed
  [`ManagedLifecycleInvariantViolation`](../../crates/aureline-shell/src/managed_workspace_labels/mod.rs)
  vocabulary: `missing_prototype_label`,
  `claim_limits_missing_or_out_of_order`, `skipped_transition`,
  `missing_recovery_action_for_degraded_state`, `copy_class_missing`,
  `label_and_copy_class_disagree`, `missing_authority_lineage`,
  `reconnecting_without_prior_connection`,
  `reprovisioning_without_prior_pause`.
- A deterministic
  [`ManagedWorkspaceLifecycleCardRecord::render_plaintext()`](../../crates/aureline-shell/src/managed_workspace_labels/mod.rs)
  block that quotes every step, lifecycle label, copy class, recovery
  action, degraded chip, lineage row, claim limit, and invariant in
  stable order so support exports and proof captures can quote the same
  payload across hosts.

## Protected walk

Drive the wedge through the canonical happy-path sequence:
`open_authenticating` → `record_connecting` → `record_warming` →
`record_ready`. The wedge MUST:

1. Mint four steps in canonical order with no skipped transitions.
2. Carry `copy_class = not_yet_admitted_copy_class` on the
   authenticating / connecting / warming rows and flip to
   `copy_class = live_environment` only once `ready` lands.
3. Carry the recovery actions
   `reauthenticate_via_system_browser` →
   `retry_connection` → `wait_for_warm` → `none` in row order.
4. Carry the prototype-label chip and the four canonical claim-limit
   rows verbatim:
   - `single_certified_wedge_only`,
   - `no_managed_control_plane_in_m1`,
   - `no_tenancy_orchestration`,
   - `no_lifecycle_executor_productization`.
5. Report `has_invariant_violations = false` and
   `current_admits_writes = true` on the final `ready` step.

Exercised by
[`protected_walk_renders_clean_card_with_visible_labels`](../../crates/aureline-shell/src/managed_workspace_labels/tests.rs)
and the fixture
[`fixtures/managed_workspace/m1_lifecycle_cases/protected_walk_managed_lifecycle.json`](../../fixtures/managed_workspace/m1_lifecycle_cases/protected_walk_managed_lifecycle.json).

## Failure drill — lifecycle advances honestly through every degraded state

After the protected walk lands `ready`, drive the wedge through:
`record_reconnecting` (transport drop) →
`record_read_only_degraded` (snapshot fallback) →
`record_suspended` (user pauses) →
`record_reprovisioning` (tenant cuts a fresh copy) →
`record_connecting` → `record_warming` → `record_ready`. The wedge
MUST:

- Visit all four workspace-copy classes the spec requires to remain
  visibly distinct (`live_environment`, `snapshot_only_view`,
  `suspended_workspace`, `fresh_reprovisioned_copy`).
- Carry a non-`none` recovery action on every degraded posture row
  (`retry_connection`, `continue_in_snapshot_view`,
  `resume_from_suspension`, `accept_reprovisioned_state`).
- Refuse to skip ahead: a call that requests `record_ready` on top of
  `record_connecting` (skipping `warming`) returns
  `WedgeError::SkippedTransition { from: connecting, to: ready }` and
  the wedge state stays unchanged.
- Refuse reconnect without a prior `ready`/`read_only_degraded` step
  (`WedgeError::ReconnectWithoutPriorConnection`).
- Refuse reprovision without a prior `suspended`/`read_only_degraded`
  step (`WedgeError::ReprovisionWithoutPriorPause`).

Exercised by
[`failure_drill_walks_reconnect_through_reprovisioning_back_to_ready`](../../crates/aureline-shell/src/managed_workspace_labels/tests.rs),
the API-rejection drills
[`skipping_warming_is_refused_at_the_api`](../../crates/aureline-shell/src/managed_workspace_labels/tests.rs),
[`reconnect_without_prior_connection_is_refused`](../../crates/aureline-shell/src/managed_workspace_labels/tests.rs),
and
[`reprovision_without_prior_pause_is_refused`](../../crates/aureline-shell/src/managed_workspace_labels/tests.rs),
plus the typed-invariant drill
[`hand_patched_skipped_transition_surfaces_typed_invariant`](../../crates/aureline-shell/src/managed_workspace_labels/tests.rs)
and the fixture
[`fixtures/managed_workspace/m1_lifecycle_cases/failure_drill_reconnect_then_reprovision.json`](../../fixtures/managed_workspace/m1_lifecycle_cases/failure_drill_reconnect_then_reprovision.json).

## Adjacent drill — snapshot-only view keeps writes blocked

The wedge admits the `snapshot_only_view` label after `ready` /
`read_only_degraded` so the chrome can offer a read-only review path
without implying the live environment is admissible. The wedge MUST:

- Set `copy_class = snapshot_only_view` and
  `admits_writes = false` on the step.
- Surface `continue_in_snapshot_view` as the recovery action and the
  `Stale` degraded chip on the row.
- Keep `has_invariant_violations = false` — the path is honest, not
  buggy.

Exercised by
[`snapshot_only_view_after_ready_keeps_writes_blocked`](../../crates/aureline-shell/src/managed_workspace_labels/tests.rs)
and the fixture
[`fixtures/managed_workspace/m1_lifecycle_cases/snapshot_only_view_keeps_writes_blocked.json`](../../fixtures/managed_workspace/m1_lifecycle_cases/snapshot_only_view_keeps_writes_blocked.json).

## Bounded scope (deliberately)

- **One certified wedge only.** The wedge runs on the managed-workspace
  state card. Task / debug / provider / terminal / install entry points
  do not own a managed-workspace lifecycle card here.
- **No managed control-plane in M1.** The wedge labels lifecycle truth
  supplied by the caller; it does not authenticate, connect, warm, or
  reprovision a managed workspace. The shared
  `no_managed_control_plane_in_m1` claim row is always rendered.
- **No tenancy orchestration.** The wedge does not issue seats, mint
  managed tenants, or run admin-console flows. The
  `no_tenancy_orchestration` claim row is always rendered.
- **No lifecycle-executor productization.** The wedge mints labels for
  one prototype path, not a managed runtime. The
  `no_lifecycle_executor_productization` claim row is always rendered.
- **Out of scope:** managed-workspace control planes, tenancy
  orchestration, lifecycle-executor productization, broad
  managed-workspace availability claims across unrelated surfaces.

## Validation command

```
cargo test -p aureline-shell --lib managed_workspace_labels
```
