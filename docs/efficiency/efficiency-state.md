# Efficiency state

Aureline's low-power and thermal behavior is inspectable through a single typed
state object, not inferred from badges or logs. One snapshot is the source of
truth; the status bar, diagnostics, and support export all project from it, so
they can never disagree about what changed, why, or which subsystems were
affected.

## The canonical object

`aureline_shell::efficiency::EfficiencyStateSnapshot` is the canonical record.
It carries:

- **active_state** — the typed efficiency state (what changed).
- **pressure_sources** — the source-of-change (why it changed). The vocabulary
  keeps the four causes the contract must never collapse into one ambiguous
  "power saving" state distinct: an OS battery saver, thermal pressure, a
  user-selected low-power mode, and a policy-imposed cap.
- **override_posture** — whether and how the adaptation may be overridden,
  derived policy-aware from the state and source.
- **recovery_state** — how deferred work resumes as pressure clears.
- **affected_subsystems** — a compact summary of every subsystem whose behavior
  changed (which subsystems were affected), naming the owner, action, and the
  user impact.
- **throttled_capabilities** / **workload_decisions** — the full capability rows
  and budget decisions behind the summary.
- **hidden_pane_audit** — proof that hidden or off-screen panes suppressed
  render, animation, and polling. The per-surface, per-class version of this
  proof — for notebooks, traces, previews, docs/browser panes, pipelines, and
  incident workspaces — lives in the
  [hidden-surface policy](./hidden-surface-policy.md).
- **protected_interactions_preserved** / **durability_invariants** — the
  protected paths the adaptation may not narrow.

### Efficiency states

| State | Meaning |
| --- | --- |
| `Nominal` | Full published budgets, ordinary governor rules. |
| `EfficiencyAware` | Battery or power-saver pressure reduces speculative work. |
| `ThermalConstrained` | Thermal or sustained CPU pressure reduces background and visual work. |
| `ProtectCore` | Core interaction is protected by pausing or denying optional work. |
| `Recovery` | Pressure has cleared; deferred work resumes in stages. |

### Source-of-change

`ac_power`, `battery`, `os_battery_saver`, `user_low_power_mode`, `low_battery`,
`critical_battery`, `thermal_pressure`, `frame_miss_pressure`, `policy_cap`,
`pressure_cleared`.

### Override posture

Derived policy-aware from the cause:

- A `policy_cap` source ⇒ `policy_blocked`.
- `ProtectCore`, or a `critical_battery` cause ⇒ `not_overridable`.
- A user-controllable cause (battery, OS battery saver, low battery, user
  low-power mode) ⇒ `user_override_session_only`.
- Any other physical pressure ⇒ `not_overridable`.

## The three surfaces

All three derive from the same `EfficiencyStateSnapshot`:

1. **Shell status** — the status bar renders the `EfficiencyStatusSnapshot`
   embedded in the snapshot, with an open-details command into
   `surface.runtime.efficiency_state`.
2. **Diagnostics** — `efficiency::surfaces::EfficiencyDiagnosticsProjection`
   (materialized by `diagnostics::efficiency_posture`) gives operators the
   state, cause, affected subsystems, override posture, recovery state, and the
   hidden-pane audit result, plus a cross-link to the support export. It embeds
   the matrix-bound governance projection so its vocabulary is traceable to the
   frozen governance matrix.
3. **Support export** — `efficiency::surfaces::EfficiencyStateSupportExport` is a
   metadata-only packet. It carries `reconstructs_*` guarantees and
   `ui_text_scrape_required: false`, so support tooling reconstructs the
   low-power posture without scraping rendered UI text or reading raw logs. It
   exports no provider payloads or secret bodies.

## Fixtures and schema

`fixtures/efficiency/states/*.json` hold one case per representative posture,
each carrying the snapshot together with its diagnostics and support-export
projections. `schemas/efficiency/efficiency-state.schema.json` describes the
case shape. Regenerate the fixtures with:

```bash
cargo run -p aureline-shell --example dump_efficiency_state_surfaces
```

The fixtures are validated in `crates/aureline-shell/tests/efficiency_state_surfaces.rs`,
which round-trips each case back through the typed surfaces and asserts the three
surfaces agree on the same object.

## Surfaces that consume this state

- [Hidden-surface render suppression](./hidden-surface-policy.md) — what hidden,
  occluded, or off-screen panes shed before protected paths degrade.
- [Per-surface low-power disclosures](./low-power-disclosures.md) — how each
  affected product surface tells the user what still works, what is delayed, and
  how to inspect or override.
- [Active-session continuity under pressure](./session-pressure.md) — how active
  tasks, debug sessions, remote attaches, notebook kernels, traces, and captures
  stay correct while optional work sheds first and any material downgrade is
  warned about before it applies.
