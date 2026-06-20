# M5 energy/thermal efficiency lab

Promotion-grade evidence for Aureline's low-power and thermal behavior. The lab
turns efficiency-state behavior from informal observation ("battery felt better")
into something a checked-in fixture can fail promotion over.

For each claimed M5 desktop profile the lab injects a deterministic schedule of
battery and thermal pressure, drives the canonical efficiency-state runtime
through the resulting transitions, and captures one **lab trace** — an ordered
series of steps, each recording the transition that fired, the subsystems it
throttled, the hidden-pane audit, and a content-free explanation of *why* each
surface slowed or paused. Two consumer packets derive from the same trace: the
[Project Doctor report and the support export](../../docs/efficiency/doctor-and-support.md).

The runtime owns the closed vocabulary
([`crates/aureline-shell/src/efficiency/energy_lab/`](../../crates/aureline-shell/src/efficiency/energy_lab/));
this lab is a downstream projection bound back to it by
[`crates/aureline-shell/tests/efficiency_energy_lab.rs`](../../crates/aureline-shell/tests/efficiency_energy_lab.rs)
so the two can never drift. The canonical efficiency-state contract is described
in [`docs/efficiency/efficiency-state.md`](../../docs/efficiency/efficiency-state.md);
the governance matrix that binds each M5 surface to its claim is at
[`artifacts/efficiency/m5-efficiency-governance.md`](./m5-efficiency-governance.md).

This lab is part of the M5 efficiency-governance and low-power truth lane,
registered alongside the governance matrix under the canonical M5 evidence index
(`artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`),
so release and shiproom evidence review can reach the lab traces, Doctor reports,
and support exports without raw log spelunking.

## Profiles exercised

Each profile names a distinct hardware-and-policy situation the contract keeps
separate, so the lab covers every cause of reduced behavior independently.

| Profile | Class | Pressure schedule | Ends in |
| --- | --- | --- | --- |
| Battery ultrabook | `battery_ultrabook` | AC nominal → OS battery saver (efficiency aware) → back on AC (recovery) | `Recovery` |
| Thermal workstation | `thermal_workstation` | nominal → sustained thermal pressure (thermal constrained) → thermals clearing (recovery) | `Recovery` |
| Policy-managed fleet | `policy_managed_fleet` | unmanaged nominal → admin policy caps background work (efficiency aware, **policy blocked**) | `EfficiencyAware` |
| Critical-battery field laptop | `critical_battery_field` | low battery (efficiency aware) → critical battery (**protect core**) → charging (recovery) | `Recovery` |

The profiles' workspaces, states, sources, and timestamps line up with the seeded
snapshots the status, diagnostics, support, and disclosure surfaces use, so the
lab evidence aligns with the rest of the low-power contract.

## Promotion gates

Each trace exposes four certifiable claims. A regression in any one fails the
checked-in fixtures (`EfficiencyLabTrace::promotion_gates_pass`):

- **`protected_paths_held`** — at every step, save durability and the protected
  interactions (typing, save, undo, local navigation, terminal correctness,
  current-task visibility) held.
- **`hidden_panes_passed`** — no hidden or off-screen pane painted, animated, or
  polled off-screen at any step.
- **`every_slowdown_explained`** — every reduced subsystem carried a content-free
  reason naming why it slowed and what stays correct.
- **`trace_is_content_free`** — the whole trace references only canonical
  vocabulary (states, source-of-change signals, subsystem tokens, surface
  *classes*, and authored labels), never document bodies, file paths, or provider
  payloads.

The policy-managed-fleet profile additionally proves an admin `policy_cap`
narrows the override posture to `policy_blocked`, and the critical-battery profile
proves `ProtectCore` is `not_overridable` while active — the two postures support
must never collapse into a generic "power saving" state.

## Pressure-injection fixtures

The lab is driven by deterministic pressure-injection schedules, not live
hardware. Each `PressureInjection` names a target state, the source-of-change
signals, the reason recorded on the transition, the background workloads observed,
and the hidden surfaces audited at that step. This is what lets a fixture fail
promotion when claimed behavior regresses: the injected pressure is fixed, so the
captured posture is the only variable.

## Files

- Machine-readable traces (one per profile):
  [`artifacts/efficiency/m5-efficiency-traces/`](./m5-efficiency-traces/).
- Fixtures (full lab case per profile: profile, trace, Doctor report, support
  export): [`fixtures/efficiency/lab/`](../../fixtures/efficiency/lab/).
- Conformance dump:
  `cargo run -p aureline-shell --example dump_efficiency_energy_lab`.
- Drift + invariant test: `cargo test -p aureline-shell efficiency_energy_lab`.
