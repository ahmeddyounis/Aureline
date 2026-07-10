# M5 power-state-indicator and throttled-subsystem-row controls

The first implement lane over the frozen [M5 efficiency component matrix](m5_efficiency_components_contract.md). It turns the two top-of-funnel adaptive-efficiency components — the **power-state indicator** and the **throttled-subsystem row** — into resolvers that produce export-safe, honest projections, so every M5 shell surface explains *why* Aureline adapted and *which* subsystems were affected instead of leaving a user to infer it from missing behavior.

- Controls packet schema: `schemas/ui/m5-power-state-throttled-subsystem-controls.schema.json`
- Support export: `artifacts/release/m5-power-state-throttled-subsystem-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-power-state-throttled-subsystem-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-power-state-throttled-subsystem-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-power-state-throttled-subsystem-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_power_state_indicator_and_throttled_subsystem_row_...`)

## Reused, not re-minted

The lane binds directly to the frozen efficiency object model so it can never fork its own low-power or thermal wording:

- **Source of change** reuses `EfficiencyPressureSource` (AC power, battery, OS battery saver, user low-power mode, low/critical battery, thermal pressure, frame-miss pressure, policy cap, pressure cleared).
- **Active efficiency state** reuses `EfficiencyState` (Nominal, EfficiencyAware, ThermalConstrained, ProtectCore, Recovery).
- **Affected subsystem** reuses `WorkloadFamily` (AI warmups, prefetch, uploads, non-essential animation, indexing refresh, extension polling, preview refresh, graph enrichment, remote/session helpers).
- **Work disposition** reuses the single controlled `M5EfficiencyWorkDisposition` vocabulary from the matrix (running_full, slowed, paused, policy_blocked, override_available, override_blocked, resuming, stale_result_shown, not_evaluated).

## Power-state indicator resolver

`resolve_power_state_indicator` degrades first rather than ever letting an ambiguous indicator read as a clean generic "low power" state:

| Condition | Degrade reason |
| --- | --- |
| Pressure signal unavailable | `pressure_signal_unavailable` |
| Source of change unstated | `source_of_change_unstated` |
| Multiple distinct causes collapsed into one warning | `causes_collapsed_into_generic` |
| No inspect path offered | `inspect_path_missing` |
| Proof stale | `proof_stale` |

A clean indicator names each distinct cause, the active state, and an inspect path, and reports `distinguishable_cause = true` — the AC1 guarantee that a user can tell *why* Aureline adapted without opening logs.

## Throttled-subsystem row resolver

`resolve_throttled_subsystem_row` enumerates exactly which lanes slowed or paused and which protected tasks remain preserved:

| Condition | Degrade reason |
| --- | --- |
| No affected subsystem named | `no_affected_subsystem_named` |
| Slowed work hidden after it became user-visible | `slowed_work_silently_hidden` |
| Same lane both slowed and paused | `slowed_versus_paused_ambiguous` |
| What still works unstated | `what_still_works_unstated` |
| Proof stale | `proof_stale` |

The `slowed_work_silently_hidden` degrade is the **AC2** guarantee: once adaptive behavior became user-visible, no surface may silently widen or hide slowed background work — it degrades visibly instead.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- collapses battery saver, thermal pressure, low-power mode, and policy cap into one generic warning;
- hides slowed work once adaptive behavior became user-visible;
- leaves what-still-works unstated on a throttled row;
- invents an alternate label for a governed state.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
