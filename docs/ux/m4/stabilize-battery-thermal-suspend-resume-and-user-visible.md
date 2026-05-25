# Battery, thermal, suspend-resume, and user-visible runtime-efficiency adaptation — contract

This is the reviewer-facing companion for the stable lane that hardens
**runtime-efficiency adaptation** — battery, thermal, low-disk, and
suspend/resume pressure — to Aureline's durable truth model: one governed record
per efficiency posture that binds a **materialized runtime-efficiency state**,
**background work shed before any foreground regression**, **protected
foreground paths within published latency bands**, **hidden-pane quiescence**, a
**surfaced queue-governor reason / paused lanes / resume owner**, **preserved
durable state**, **per-OS conformance**, and a **public claim ceiling** with an
automatic narrow-below-Stable verdict.

This lane stabilizes the alpha efficiency runtime (`aureline_shell::efficiency`)
and the beta runtime-adaptation page (`aureline_shell::runtime_adaptation`) into
a Stable governed record. Where those modules own the power/thermal policy, the
workload-budget decisions, and the suspend-resume continuity vocabulary, this
lane proves that *every* claimed-stable desktop posture materializes its
efficiency state honestly: it names what it sheds, keeps active editing fast,
keeps hidden panes quiet, and never lets a battery/thermal/disk/suspend
transition read as generic slowness or stale data.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible/`](../../../fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible/)
- Schema:
  [`/schemas/ux/stabilize-battery-thermal-suspend-resume-and-user-visible.schema.json`](../../../schemas/ux/stabilize-battery-thermal-suspend-resume-and-user-visible.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible.md`](../../../artifacts/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible.md)
- Typed source: `aureline_shell::runtime_efficiency_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_runtime_efficiency_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/runtime_efficiency_stable_fixtures.rs`

## Why one governed efficiency record

Power, thermal, low-disk, and suspend/resume pressure all converge on the same
risk: the product quietly trades the user's active work for headroom and never
says so. A competitor reduces typing responsiveness "to save battery", keeps a
hidden pane animating off-screen, lets indexing or uploads starve the
foreground, or shows stale data after a resume with no label — and the user reads
it as generic slowness or a broken app, with the truth living only in a
transient toast.

This lane mints one governed `runtime_efficiency_adaptation_record` per posture.
It does **not** reinvent the power/thermal policy, the workload-budget classes,
the render-visibility audit, or the suspend-resume continuity vocabulary: each
record is a genuine projection of the live efficiency runtime
(`aureline_shell::efficiency`) — a real `EfficiencyStateRuntime` is driven into
the posture's state and its workload-budget and render-visibility decisions are
read back — and the suspend-resume / power-posture page
(`aureline_shell::runtime_adaptation`). The record binds, for one
efficiency-state identity:

1. **A materialized runtime-efficiency state.** `efficiency_state` is one of
   `Nominal`, `EfficiencyAware`, `ThermalConstrained`, `ProtectCore`, or
   `Recovery`. Each posture carries named shed-work classes (`shed_work[]`),
   protected foreground paths (`protected_paths[]`), resume conditions, and
   export-safe diagnostics. A Stable posture proves
   `pillars.efficiency_state_materialized`.
2. **Background shed before foreground.** Every behavior-changing `shed_work[]`
   row proves `shed_before_foreground` — speculative indexing, extension warmup,
   AI background jobs, uploads, and provider-overlay refresh pause or throttle
   before typing, save, navigation, or quick-open ever regress.
3. **Protected foreground latency bands.** Each `protected_paths[]` row carries a
   `published_band_ms` and an `observed_p99_ms`; a Stable posture proves every
   path stays `within_band` and `preserved_under_posture`.
4. **Hidden-pane quiescence.** `hidden_pane_audit` is projected from the live
   render-visibility policy; a Stable posture proves
   `passes_hidden_pane_policy` with zero `hidden_pane_render_violation_count` —
   no hidden, occluded, or off-screen pane commits paint or runs a speculative
   poll.
5. **A surfaced queue-governor reason.** `governor` names the `reason` (battery
   saver, thermal clamp, low-disk, suspend/resume, …), the `paused_lane_tokens`,
   and the `resume_owner`; a pressured posture proves it is
   `surfaced_in_status_strip` and `surfaced_in_diagnostics` and never
   masquerades as generic slowness (`not_generic_slowness`) or stale data
   (`not_stale_masquerade`).
6. **Preserved durable state.** `durability` proves save durability, dirty
   buffers, and user-owned artifacts are preserved — adaptation may pause optional
   work but never loses local durable state.
7. **Per-OS conformance.** `platform_conformance[]` covers macOS, Windows, and
   Linux, each with current proof and named downgrade behaviors.
8. **A public claim ceiling and automatic narrowing.** `claim_ceiling.asserts_*`
   may never exceed the proven pillars, and a posture that cannot prove a pillar,
   or whose lowest binding surface marker is below Stable, narrows below Stable
   with a named `stable_qualification.narrowing_reasons[]` entry instead of
   inheriting an adjacent green row.

## Binding surfaces read the shared record

`surface_projections[]` enumerates the five binding surfaces that ingest this
record verbatim rather than cloning prose:

- `shell_status_strip` — the shell status strip / status overflow efficiency pill.
- `diagnostics_review` — the in-product efficiency / diagnostics review surface.
- `cli_inspect` — the `aureline_shell_runtime_efficiency_stable` headless
  inspector (`scenario`, `all`, `plaintext`, `index`).
- `help_about` — the Help/About efficiency posture.
- `support_export` — the redacted diagnostics support export (the per-record
  `support_export_lines()` plaintext block).

The lowest binding-surface marker drives `surface_lifecycle_marker`; a binding
surface still in preview narrows the posture to Preview.

## The claimed-stable matrix

See the release-evidence packet for the full table. The matrix materializes all
five runtime-efficiency states and spans a deliberate span of Stable and
narrowed rows, including two adversarial drills — a foreground-latency
regression and a hidden-pane render leak — that the lane narrows below Stable
with a named reason.

## Reading a record

Each fixture is one `runtime_efficiency_adaptation_record`. Start from
`efficiency_state`, `stable_qualification` (claim class + narrowing reasons), and
`pillars`. The `governor`, `shed_work[]`, `protected_paths[]`,
`hidden_pane_audit`, and `durability` blocks carry the per-pillar evidence;
`suspend_resume` carries the resume continuity when the posture is driven by a
sleep/wake cycle; `platform_conformance[]` carries the per-OS proof;
`surface_projections[]`, `recovery_routes[]`, `routes[]`, and `accessibility`
carry the discover/operate/recover parity. `honesty_marker_present` is set
whenever there is anything narrowed, below-Stable, pressured, or
resume-continuity-bearing to disclose.

## Guardrails

No hover-only routes, no focus ambiguity, no toast-only truth, and no
hard-coded theme/state semantics. The lane does not widen public scope from this
row alone: if delivery proves a narrower claim than planned, the posture
downgrades and names the reason in the record rather than papering over the gap.
A row may never achieve "efficiency" by silently slowing active editing or
losing local durable state — the foreground-latency and hidden-pane drills exist
precisely to keep that promise enforceable.
