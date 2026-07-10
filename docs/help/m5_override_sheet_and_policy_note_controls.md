# M5 per-workspace override-sheet and override-policy note-row controls

This is the third implement lane over the frozen
[M5 efficiency component matrix](m5_efficiency_components_contract.md). It turns two of the
matrix's reusable adaptive-efficiency components — the **per-workspace override sheet** and the
**override-policy note row** — into resolvers that produce export-safe, honest projections instead
of dead or misleading override controls.

The canonical source of truth is the Rust in
`crates/aureline-shell/src/implement_the_m5_per_workspace_override_sheet_and_override_policy_note_row_current_mode_ceilings_expected_effect_reset_path_and_blocked_by_policy_primitive`.
The checked-in evidence under
`artifacts/release/m5-override-sheet-policy-note-controls-proof/` and the narrowed fixtures under
`fixtures/ui/m5-override-sheet-policy-note-controls/` are minted from the seed builders by the
headless emitter; the inline tests byte-lock the artifact to the seed.

## What the components explain

A **per-workspace override sheet** previews an adaptation before the user changes it and names:

- the **current efficiency mode** (nominal, efficiency-aware, thermal-constrained, …);
- the **allowed policy ceilings** — the limits an override may reach;
- the **expected effect** on indexing, AI, or extensions;
- the **exact reset path** back to the policy default; and
- the **performance-versus-freshness trade-off** the override implies.

An **override-policy note row** explains, next to the override, when an override is **blocked**,
**who owns the policy**, and **what remains changeable locally**.

## Acceptance criteria

- **AC1 — users never see a dead or misleading override control when policy disallows the
  requested behavior change.** A sheet or note that still presents an override as an actionable
  control while its posture blocks the override degrades to `dead_override_control_offered`. A
  clean surface facing a blocking policy shows the override as blocked-by-policy, names the owner,
  and states what remains changeable locally.
- **AC2 — override sheets are explicit about the performance-versus-freshness trade-off and never
  hide side effects behind generic efficiency language.** A sheet that omits the trade-off degrades
  to `performance_freshness_tradeoff_unstated`; a sheet that collapses the expected effect into
  generic low-power wording degrades to `side_effects_hidden_by_generic_language`.

Both criteria are proven by the resolved examples the packet carries, not merely asserted by
governance flags.

## Guardrails

Every controls row carries four hard invariants, all of which must be `false`:

- `presents_override_available_when_policy_blocks`
- `hides_side_effects_behind_generic_efficiency_language`
- `collapses_pressure_sources_into_generic_warning`
- `hides_what_remains_changeable_locally`

## Emitter

```text
cargo run -p aureline-shell --example dump_m5_override_controls -- support-export
cargo run -p aureline-shell --example dump_m5_override_controls -- report
cargo run -p aureline-shell --example dump_m5_override_controls -- csv
cargo run -p aureline-shell --example dump_m5_override_controls -- fixture-override-settings-beta-narrowed
cargo run -p aureline-shell --example dump_m5_override_controls -- fixture-activity-center-preview-narrowed
cargo run -p aureline-shell --example dump_m5_override_controls -- validate
```
