# M5 resume-summary card and stale-result continuity note controls

This is the fourth and final implement lane over the frozen
[M5 efficiency component matrix](m5_efficiency_components_contract.md). It turns the last two of the
matrix's reusable adaptive-efficiency components — the **resume-summary card** and the
**stale-result continuity note** — into resolvers that produce export-safe, honest projections
instead of recovery a user has to infer.

The canonical source of truth is the Rust in
`crates/aureline-shell/src/implement_the_m5_resume_summary_card_and_stale_result_continuity_note_resumed_work_backlog_state_stale_results_visible_and_next_safe_action_primitive`.
The checked-in evidence under
`artifacts/release/m5-resume-summary-stale-note-controls-proof/` and the narrowed fixtures under
`fixtures/ui/m5-resume-summary-stale-note-controls/` are minted from the seed builders by the
headless emitter; the inline tests byte-lock the artifact to the seed.

## What the components explain

A **resume-summary card** is the durable summary a user sees after pressure clears. It names:

- the **recovery state** (staged resume, recovered, …);
- **what resumed** — the workloads brought back from their deferred backlog;
- **what backlog remains** after resume;
- **whether stale results are still visible**; and
- the **safest next action** for the current task.

A **stale-result continuity note** appears wherever previously paused work can leave cached or
partially refreshed outputs visible after recovery. It states that a still-visible result is
**retained** or **refreshing**, that it is **based on a prior constrained state**, and — while
refreshing — the **refresh path**.

## Acceptance criteria

- **AC1 — returning to nominal conditions never silently removes evidence that a result is still
  stale, partial, or based on a prior constrained state.** A card or note that drops a live stale
  result from view on resume degrades to `stale_result_evidence_dropped` / `stale_evidence_silently_removed`.
  A clean surface keeps a retained or refreshing result visible and states that it is based on a
  prior constrained state.
- **AC2 — users get one durable summary of resumed work instead of having to infer recovery from
  disappearing banners or background queue motion.** A card whose summary is not durable degrades to
  `recovery_summary_not_durable`; a card that hides the resumed-work backlog degrades to
  `resume_backlog_hidden`.

Both criteria are proven by the resolved examples the packet carries, not merely asserted by
governance flags.

## Guardrails

Every controls row carries four hard invariants, all of which must be `false`:

- `clears_stale_result_context_on_resume`
- `requires_inferring_recovery_from_transient_banners`
- `hides_resumed_work_backlog`
- `collapses_pressure_sources_into_generic_warning`

## Emitter

```text
cargo run -p aureline-shell --example dump_m5_resume_controls -- support-export
cargo run -p aureline-shell --example dump_m5_resume_controls -- report
cargo run -p aureline-shell --example dump_m5_resume_controls -- csv
cargo run -p aureline-shell --example dump_m5_resume_controls -- fixture-activity-center-beta-narrowed
cargo run -p aureline-shell --example dump_m5_resume_controls -- fixture-background-work-preview-narrowed
cargo run -p aureline-shell --example dump_m5_resume_controls -- validate
```
