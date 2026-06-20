# Efficiency state in Project Doctor and support export

Support and operators must be able to explain low-power and thermal behavior
without raw log spelunking. Two packets make that possible, and both derive from
the **same** energy/thermal lab trace, so Project Doctor, support export, and a
release reviewer can never disagree about the posture they describe.

The canonical instant-in-time object is the
[`EfficiencyStateSnapshot`](./efficiency-state.md). The packets here add the
*over-time* dimension: how the state got here, and what is still reduced.

- `aureline_shell::efficiency::energy_lab::EfficiencyLabTrace` — the ordered
  evidence one claimed M5 profile produces under an injected battery/thermal
  pressure schedule. See the [energy-lab artifact](../../artifacts/efficiency/m5-energy-lab.md).
- `aureline_shell::efficiency::energy_lab::EfficiencyDoctorReport` — the Project
  Doctor projection of a finished trace.
- `aureline_shell::efficiency::energy_lab::EfficiencyLabSupportExport` — a
  metadata-only support/export packet for a finished trace.

## What Project Doctor reports

The Doctor report answers the four operator questions the efficiency-state
contract keeps separate, ranked next to Doctor's other probes under the probe id
`probe.runtime.efficiency_state`:

| Field | Question it answers |
| --- | --- |
| `current_state` | **What is the efficiency state now?** A typed state token. |
| `recent_transitions` | **How did we get here?** The ordered transition history (oldest first), each naming the previous and new state, the source-of-change, a reason, and the top throttled contributors. |
| `throttled_subsystems` | **Which subsystems are reduced?** A compact per-subsystem summary: token, label, owner, action, visible state, and user impact. |
| `override_posture` | **Can it be overridden, and how?** The policy-aware posture token. |

It also carries a `finding_code`, a `finding_severity` (`ok`, `notice`, or
`degraded`), and a one-sentence `summary_label` so the posture can be ranked, plus
a `primary_command_id` / `opens_surface_ref` pair so an operator can open the full
state surface. `EfficiencyDoctorReport::names_state_transitions_subsystems_and_override`
gates that all four contract fields are present and resolve against the closed
vocabulary.

### Finding codes

| State | Finding code | Severity |
| --- | --- | --- |
| `Nominal` | `efficiency_nominal` | `ok` |
| `EfficiencyAware` | `efficiency_reducing_optional_work` | `notice` |
| `ThermalConstrained` | `efficiency_thermal_constrained` | `notice` |
| `ProtectCore` | `efficiency_protect_core_active` | `degraded` |
| `Recovery` | `efficiency_recovering` | `notice` |

## What support export carries

`EfficiencyLabSupportExport` is **metadata-only by construction**. It quotes the
same `current_state`, `recent_transitions`, `throttled_subsystems`,
`override_posture`, and `recovery_state` as the Doctor report, so support reads an
identical posture. It carries no provider payloads, secret bodies, or user
content, and never requires scraping rendered prose:

- `redaction_safe()` ⇒ no UI-text scrape, no raw provider payloads, no raw secret
  values, no named user content.
- `reconstructs_posture_without_logs()` ⇒ the transition history, throttled
  subsystems, and override posture are all reconstructable from structured fields
  (`support_field_refs`), not from raw logs.

The flags `ui_text_scrape_required`, `raw_provider_payloads_exported`,
`raw_secret_values_exported`, and `names_user_content` are all `false` and are
asserted by the fixtures, so a regression that starts leaking content fails
promotion.

## Why a surface slowed, without leaking content

Each trace step records a `SurfaceSlowdownExplanation` for every reduced
subsystem. The explanation reuses the canonical capability-row sentences — the
`why_label` is *why the surface slowed*, the `what_stays_correct` is *what stays
correct while it is reduced* — so it can never disagree with the status and
disclosure surfaces, and can never carry document bodies, file paths, or provider
payloads. Every explanation is `content_free` and names no user content; the
`EfficiencyLabTraceStep::every_slowdown_explained` gate fails if a surface slows
without a recorded, content-free reason.

## One source of truth

```text
EfficiencyLabTrace
  ├─ EfficiencyDoctorReport::from_trace(&trace)
  └─ EfficiencyLabSupportExport::from_trace(&trace)
```

Both projections quote the trace's `final_state`, `transitions`, final throttled
subsystems, and `final_override_posture`. The fixtures assert the Doctor report
and support export agree with each other and with the trace they derive from, so a
later release, support, or help surface should consume these packets instead of
re-deriving low-power prose.

## Provenance

- Runtime: [`crates/aureline-shell/src/efficiency/energy_lab/`](../../crates/aureline-shell/src/efficiency/energy_lab/).
- Conformance dump: `cargo run -p aureline-shell --example dump_efficiency_energy_lab`.
- Fixtures: [`fixtures/efficiency/lab/`](../../fixtures/efficiency/lab/), re-derived
  and drift-checked by `crates/aureline-shell/tests/efficiency_energy_lab.rs`.
- Exported traces: [`artifacts/efficiency/m5-efficiency-traces/`](../../artifacts/efficiency/m5-efficiency-traces/).
