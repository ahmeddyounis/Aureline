# Battery, thermal, suspend-resume, and user-visible runtime-efficiency adaptation — release evidence

Reviewer-facing evidence packet for the lane that stabilizes **battery, thermal,
suspend-resume, and user-visible runtime-efficiency adaptation** on claimed-stable
desktop shell surfaces: one canonical record per efficiency posture that binds a
materialized runtime-efficiency state, background work shed before any foreground
regression, protected foreground paths within published latency bands,
hidden-pane quiescence, a surfaced queue-governor reason / paused lanes / resume
owner, preserved durable state, per-OS conformance, a public claim ceiling, an
automatic narrow-below-Stable verdict, recovery and route parity across the
activity center / command palette / status bar / menus, accessibility across
normal / high-contrast / zoomed layouts, and postures that stay available
without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible/`](../../../fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible/)
- Schema: [`/schemas/ux/stabilize-battery-thermal-suspend-resume-and-user-visible.schema.json`](../../../schemas/ux/stabilize-battery-thermal-suspend-resume-and-user-visible.schema.json)
- Companion doc: [`/docs/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible.md`](../../../docs/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible.md)
- Typed source: `aureline_shell::runtime_efficiency_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_runtime_efficiency_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/runtime_efficiency_stable_fixtures.rs`
- Projected from: `aureline_shell::efficiency` (power/thermal policy, workload-budget decisions, render-visibility audit), `aureline_shell::runtime_adaptation` (suspend-resume / power-posture page)

## The claimed-stable matrix

| Record | State | Governor reason | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- | --- |
| `nominal_ac_power_stable.json` | Nominal | none_nominal | **stable** | stable | — |
| `battery_saver_efficiency_aware_stable.json` | EfficiencyAware | battery_saver | **stable** | stable | — |
| `thermal_constrained_stable.json` | ThermalConstrained | thermal_clamp | **stable** | stable | — |
| `critical_battery_protect_core_stable.json` | ProtectCore | critical_battery | **stable** | stable | — |
| `suspend_resume_recovery_stable.json` | Recovery | suspend_resume | **stable** | stable | — |
| `foreground_latency_regression_drill.json` | EfficiencyAware | battery_saver | beta (narrowed) | stable | `foreground_exceeds_latency_band` |
| `hidden_pane_render_leak_drill.json` | ThermalConstrained | thermal_clamp | beta (narrowed) | stable | `hidden_panes_not_quiescent` |
| `low_disk_protect_core_preview_surface.json` | ProtectCore | low_disk | preview (narrowed) | preview | `surface_not_yet_stable` |

Coverage verdict: **5 Stable, 3 narrowed**, materializing all five
runtime-efficiency states (Nominal, EfficiencyAware, ThermalConstrained,
ProtectCore, Recovery) and the battery-saver, thermal-clamp, critical-battery,
suspend/resume, and low-disk governor reasons. Each narrowed row names a reason
and drops below the launch cutline rather than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **The five runtime-efficiency states are materialized with named shed-work,
  protected paths, resume conditions, and diagnostics.** Every record carries an
  `efficiency_state`, a `governor` block, the five `protected_paths[]` rows, and
  (where the state degrades work) named `shed_work[]` rows with a `resume_owner`
  and `resume_condition`. The matrix covers all five states; the
  `matrix_materializes_every_efficiency_state` gate enforces it.
- **Background work is paused or throttled before any foreground regression.**
  Every behavior-changing `shed_work[]` row proves `shed_before_foreground`; the
  rows are projected from the live `EfficiencyStateRuntime` workload-budget
  decisions for speculative prefetch, AI warmups, indexing refresh, extension
  polling, uploads, preview refresh, and graph enrichment. Stable rows prove
  `pillars.background_shed_before_foreground`.
- **Protected foreground paths stay within published latency bands.** Each
  `protected_paths[]` row carries `published_band_ms` and `observed_p99_ms` for
  editing, save, direct navigation, quick-open, and the command palette; Stable
  rows keep every path `within_band`. The
  `foreground_latency_regression_drill` deliberately lets typing p99 exceed its
  band and is narrowed below Stable with `foreground_exceeds_latency_band`.
- **Hidden panes do not paint, animate, or poll off-screen.** `hidden_pane_audit`
  is projected from the live render-visibility policy; Stable rows prove
  `passes_hidden_pane_policy` with zero violations. The
  `hidden_pane_render_leak_drill` injects a hidden pane that keeps committing
  paint and a background poll and is narrowed with `hidden_panes_not_quiescent`.
- **The queue-governor reason, paused lanes, and resume owner are surfaced.**
  Every pressured posture's `governor` is `surfaced_in_status_strip` and
  `surfaced_in_diagnostics` with `not_generic_slowness` and
  `not_stale_masquerade`, so battery saver, thermal clamp, low-disk, suspend, and
  resume transitions never masquerade as generic slowness or stale data.
- **No marketed row achieves efficiency by losing local durable state.** Every
  record's `durability` proves save durability, dirty buffers, and user-owned
  artifacts are preserved; the `durable_state_preserved_everywhere` gate enforces
  it across the whole matrix.
- **Suspend/resume keeps local work fast and authority honest.** The Recovery
  posture carries a `suspend_resume` continuity block projected from
  `aureline_shell::runtime_adaptation`: local work continues, privileged work is
  paused at the boundary, no silent rerun or authority reuse is admitted, and a
  user-visible resume summary is required.
- **Per-OS conformance covers macOS, Windows, and Linux.** Every record's
  `platform_conformance[]` covers the three profiles with current proof and named
  downgrade behaviors.
- **Below-Stable surfaces are narrowed, not inherited.**
  `low_disk_protect_core_preview_surface` proves every pillar but binds a
  diagnostics-review surface still in preview, so it is narrowed to Preview by its
  lowest binding surface marker.
- **Discover / operate / recover from keyboard and mouse, no account.** Every
  record exposes `recovery_routes[]` (open efficiency status, review paused work,
  resume / override where allowed, open diagnostics, export support), `routes[]`
  for the activity center / command palette / status bar / menu command (all
  keyboard reachable, all activating the same posture), an `accessibility` block
  holding across normal / high-contrast / zoomed layouts, and
  `available_without_account` + `available_without_managed_services`.

## Reproduce

```sh
# Stable corpus index — scenario id, state, governor, claim, marker.
cargo run -q -p aureline-shell --bin aureline_shell_runtime_efficiency_stable -- index

# Per-record plaintext truth blocks (support-export form).
cargo run -q -p aureline-shell --bin aureline_shell_runtime_efficiency_stable -- plaintext

# Refresh the on-disk fixtures.
cargo run -q -p aureline-shell --bin aureline_shell_runtime_efficiency_stable -- emit-fixtures \
  fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible

# Replay + invariant gate.
cargo test -p aureline-shell --test runtime_efficiency_stable_fixtures
```

## Guardrails honored

No hover-only routes, no focus ambiguity, no toast-only truth, no hard-coded
theme/state semantics, and no public-scope widening from this row alone. A
posture that proves a narrower claim than planned downgrades and names the reason
in the record rather than papering over the gap; the foreground-latency and
hidden-pane drills keep the "no silent degradation of active editing or durable
state" promise enforceable in CI.
