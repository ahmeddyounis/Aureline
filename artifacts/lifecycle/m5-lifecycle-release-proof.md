# M5 lifecycle release proof: lifecycle-state, checkpoint, and recovery-affordance truth across every claimed M5 profile and exported truth surface

Generated from the seeded packet in
[`crate::m5_lifecycle_release_proof`](../../crates/aureline-shell/src/m5_lifecycle_release_proof/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_release_proof -- markdown > \
  artifacts/lifecycle/m5-lifecycle-release-proof.md
```

- Packet id: `m5-lifecycle-release-proof:stable:0001`
- Source schema ref: `schemas/lifecycle/m5-lifecycle-release-proof.schema.json`
- Certifies matrix packet: `m5-lifecycle-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required proof dimensions: `lifecycle_state_truth`, `checkpoint_truth`, `recovery_affordance_truth`, `exported_proof_parity`
- Claimed profiles certified: `compact_desktop`, `standard_desktop`, `expanded_desktop`, `mixed_dpi`, `multi_monitor`, `dependency_missing_restore`
- Object families certified: 13
- Green (full conformance): 9
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable (stable-promotion gate): `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Object family | Status | Lifecycle-state truth | Checkpoint truth | Recovery truth | Exported proof | Headless | Waiver |
| ------------- | ------ | --------------------- | ---------------- | -------------- | -------------- | -------- | ------ |
| Workspace / window session | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Installed extension | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Remote / tunnel session | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Collaboration session | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| AI assistant action | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Update / rollback | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Notebook runtime | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Request / API run | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Preview / live-server session | `yellow` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `disclosed_partial_export_refresh` | `true` | — |
| Pipeline / task run | `yellow` | `explicit_state_truth_certified` | `disclosed_compacted_checkpoint_truth` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Data / database session | `green` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Profiler / trace capture | `yellow` | `disclosed_reduced_state_truth` | `named_checkpoint_truth_certified` | `named_recovery_and_reason_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Companion / paired-device session | `yellow` | `explicit_state_truth_certified` | `named_checkpoint_truth_certified` | `disclosed_reduced_recovery_truth` | `exported_surfaces_reflect_current_proof` | `true` | `waiver:companion-reduced-recovery-truth:0001` |

## Auto-narrowed rows

- `preview_session` (`yellow`) — On the legacy preview diagnostics surface one exported truth surface takes a disclosed partial refresh cadence — the legacy diagnostics export refreshes on a slower cadence than the live UI while still disclosing the lag and still exporting the same status code and last-failure reason — so the exported parity is narrowed and disclosed rather than stale or divergent.
- `pipeline_run` (`yellow`) — On a long fan-out pipeline the run shows a disclosed compacted checkpoint sequence — two adjacent stage milestones are folded into one disclosed compacted milestone while each terminal checkpoint is still named — so the checkpoint truth is narrowed and disclosed rather than collapsed into an anonymous spinner.
- `profiler_capture` (`yellow`) — On a constrained trace-capture build the profiler exposes a disclosed reduced lifecycle-state truth — a handful of intermediate capture states are grouped into one disclosed grouped state while the terminal controlled state (ready, partial_ready, or recoverable_failure) is still named — so the state truth is narrowed and disclosed rather than collapsed into a generic loading or error behavior.
- `companion_session` (`yellow`) — On the small companion / paired-device surface a degraded session exposes a disclosed, waivered reduced recovery truth — the named recovery affordance is deferred to a linked reattach-on-desktop action while the controlled last-failure reason is still named inline — so the recovery truth is narrowed and disclosed rather than dropped.

## Exact conformance causes

- `preview_session` — `upstream_dependency_narrowed` (disclosed: `true`) — One exported truth surface takes a disclosed partial refresh cadence on a legacy surface — a legacy diagnostics export refreshes on a slower cadence while still disclosing the lag — so the exported parity is narrowed and disclosed rather than stale or divergent.
- `pipeline_run` — `upstream_dependency_narrowed` (disclosed: `true`) — On a long-running journey the object shows a disclosed compacted checkpoint sequence — two adjacent milestones are folded into one disclosed compacted milestone while each terminal checkpoint is still named — so the checkpoint truth is narrowed and disclosed rather than collapsed into an anonymous spinner.
- `profiler_capture` — `upstream_dependency_narrowed` (disclosed: `true`) — On a constrained build the object exposes a disclosed reduced lifecycle-state truth — a handful of intermediate states are grouped into one disclosed grouped state while the terminal controlled state is still named — so the state truth is narrowed and disclosed rather than collapsed into a generic loading or error behavior.
- `companion_session` — `upstream_dependency_narrowed` (disclosed: `true`) — On a constrained surface the object exposes a disclosed, waivered reduced recovery truth — the named recovery affordance is deferred to a linked action while the controlled last-failure reason is still named — so the recovery truth is narrowed and disclosed rather than dropped.

## Active waivers

- `waiver:companion-reduced-recovery-truth:0001` (`companion_session`, owner: Companion owner, expires `2026-09-30T00:00:00Z`) — On the small companion / paired-device surface a degraded session exposes a disclosed reduced recovery truth — the named recovery affordance is deferred to a linked reattach-on-desktop action while the controlled last-failure reason is still named inline — so the recovery truth is narrowed and disclosed rather than dropped. The full in-place recovery affordance is restored the moment the companion reattaches to a standard-width surface.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_release_proof -- validate
cargo test -p aureline-shell --test m5_lifecycle_release_proof_fixtures
```
