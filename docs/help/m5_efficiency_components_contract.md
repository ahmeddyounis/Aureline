# M5 adaptive-efficiency component matrix contract

Aureline locks its reusable adaptive-efficiency UI components into **one canonical component
matrix** so every M5 surface that adapts under battery, thermal, or policy pressure reuses the
same vocabulary instead of improvising its own low-power or thermal messaging.

- **Truth source:** `crates/aureline-shell` module
  `freeze_the_m5_power_state_indicator_..._component_matrix` (seed builders + validator).
- **Combined schema:** `schemas/ui/m5-efficiency-component-matrix.schema.json`
- **Support export:** `artifacts/release/m5-efficiency-components-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-efficiency-components-proof/matrix.csv`
- **Design report:** `artifacts/design/m5-efficiency-component-matrix.md`
- **Narrowed fixtures:** `fixtures/ui/m5-efficiency-components/`
- **Bound object model:** `schemas/efficiency/m5-efficiency-governance.schema.json` and
  `artifacts/efficiency/m5-efficiency-governance.json` — the frozen efficiency-state governance
  matrix whose source-of-change, active-state, workload, override, and recovery vocabularies
  these components reuse verbatim.

## The eight governed components

Each component has its own canonical single-instance schema under `schemas/ui/`; a claimed M5
surface references one of these (or the combined matrix schema) rather than restating its truth.

| Component | Canonical schema | What it must always name |
| --- | --- | --- |
| `power_state_indicator` | `m5-power-state-indicator.schema.json` | source of change + active efficiency state |
| `throttled_subsystem_row` | `m5-throttled-subsystem-row.schema.json` | which subsystem's work is slowed / what still works |
| `background_work_row` | `m5-background-work-row.schema.json` | one job's slowed-versus-paused disposition |
| `background_work_banner` | `m5-background-work-banner.schema.json` | aggregate paused/slowed work, never toast-only |
| `per_workspace_override_sheet` | `m5-efficiency-override-sheet.schema.json` | override availability + policy owner |
| `override_policy_note_row` | `m5-override-policy-note-row.schema.json` | the accountable policy owner |
| `resume_summary_card` | `m5-resume-summary-card.schema.json` | resumed-work backlog + recovery state |
| `stale_result_continuity_note` | `m5-stale-result-continuity-note.schema.json` | stale-result continuity across resume |

## The one controlled work-disposition vocabulary

Every component row carries the frozen `work_dispositions` vocabulary — consumers bind to these
exact tokens and never invent a parallel word:

`running_full`, `slowed`, `paused`, `policy_blocked`, `override_available`, `override_blocked`,
`resuming`, `stale_result_shown`, `not_evaluated`.

## Bound family-specific vocabularies

These are reused from the efficiency-state governance object model, not re-minted:

- **Source of change** — `pressure_sources`: `ac_power`, `battery`, `os_battery_saver`,
  `user_low_power_mode`, `low_battery`, `critical_battery`, `thermal_pressure`,
  `frame_miss_pressure`, `policy_cap`, `pressure_cleared`.
- **Active efficiency state** — `efficiency_states`: `Nominal`, `EfficiencyAware`,
  `ThermalConstrained`, `ProtectCore`, `Recovery`.
- **Affected workload** — `affected_workloads`: the nine `WorkloadFamily` tokens.
- **Override posture** — `override_postures`: `not_overridable`, `user_override_session_only`,
  `user_override_persistent`, `policy_blocked`, `admin_controlled`.
- **Policy owner** — `policy_owners`: `user_controlled`, `local_policy`, `admin_policy`,
  `provider_policy`, `no_owner_resolved`.
- **Recovery state** — `recovery_states`: the six `EfficiencyRecoveryState` tokens.
- **Stale-result continuity** — `stale_result_states`: `fresh_result`, `stale_result_retained`,
  `stale_result_refreshing`, `stale_result_superseded`, `continuity_unknown`.

## Hard invariants (guardrails)

Every row asserts these are `false`, and the validator rejects any packet that violates them:

1. `collapses_pressure_sources_into_generic_warning` — battery saver, thermal pressure,
   user-selected low-power mode, and policy cap are never collapsed into one generic warning.
2. `hides_paused_work_behind_toast_only` — paused work is never hidden behind toast-only
   messaging.
3. `presents_override_available_when_policy_blocks` — an override never reads as available when
   policy blocks it.
4. `clears_stale_context_on_resume` — stale-result context is never cleared merely because
   background work resumed.

## Acceptance criteria

- **Every claimed M5 adaptive-efficiency surface references this one canonical matrix, or is
  explicitly narrowed with current evidence.** Narrowed surfaces stay visible: the two checked
  fixtures narrow one component each (override sheet → `beta`, stale-result note → `preview`)
  while keeping all eight component rows present.
- **The matrix drives schemas, fixtures, accessibility reviews, support/export packets, and
  release certification without feature-local reinterpretation** — the seed builder is the single
  producer of the export, CSV, report, and fixtures, and a test asserts the checked-in export and
  fixtures never drift from it.

## Regenerating the artifacts

```text
cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- support-export
cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- csv
cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- report
cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- fixture-override-sheet-beta-narrowed
cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- fixture-stale-result-note-preview-narrowed
cargo run -p aureline-shell --example dump_m5_efficiency_component_matrix -- validate
```
