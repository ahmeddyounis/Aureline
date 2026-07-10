# M5 background-work-row and background-work-banner controls

This is the second implement lane over the frozen
[M5 efficiency component matrix](m5_efficiency_components_contract.md). It turns two of the
matrix's reusable adaptive-efficiency components — the **background-work row** and the
**background-work banner** — into resolvers that produce export-safe, honest projections instead
of transient toast noise.

The canonical source of truth is the Rust in
`crates/aureline-shell/src/implement_the_m5_background_work_row_and_background_work_banner_affected_work_class_state_what_still_works_resume_condition_and_override_primitive`.
The checked-in evidence under
`artifacts/release/m5-background-work-row-banner-controls-proof/` and the narrowed fixtures under
`fixtures/ui/m5-background-work-row-banner-controls/` are minted from the seed builders by the
headless emitter; the inline tests byte-lock the artifact to the seed.

## What the components explain

A **background-work row** describes one adapting job and names:

- the **affected work class** (indexing refresh, AI warmups, uploads, prefetch, …);
- its current **slowed-versus-paused disposition** from the single controlled work-disposition
  vocabulary;
- **what still works** — the protected tasks that remain preserved;
- the **resume condition** — when or how the work may resume; and
- whether an **override** exists, and its policy owner.

A **background-work banner** coalesces broad or repeated pressure across many jobs into one
durable surface, naming the aggregate slowed and paused work explicitly.

## Acceptance criteria

- **AC1 — paused indexing, AI warmups, docs sync, prebuild refresh, or package metadata refresh
  remain reviewable after the user looks away.** A row that became user-visible but is only
  carried in a transient toast degrades to `toast_only_not_durable`. A clean row stays reviewable
  in a durable shell or activity surface.
- **AC2 — broad or repeated pressure events never degrade into duplicate toast spam or generic
  service-failure copy.** A banner that would emit one toast per event degrades to
  `duplicate_toast_spam`; a banner that collapses adaptive truth into a generic "something went
  wrong" message degrades to `generic_service_failure_copy`.

Both criteria are proven by the resolved examples the packet carries, not merely asserted by
governance flags.

## Guardrails

Every controls row carries four hard invariants, all of which must be `false`:

- `collapses_pressure_into_generic_service_failure`
- `hides_paused_work_behind_toast_only`
- `presents_override_available_when_policy_blocks`
- `drops_background_work_after_toast_dismissal`

## Emitter

```text
cargo run -p aureline-shell --example dump_m5_background_work_controls -- support-export
cargo run -p aureline-shell --example dump_m5_background_work_controls -- report
cargo run -p aureline-shell --example dump_m5_background_work_controls -- csv
cargo run -p aureline-shell --example dump_m5_background_work_controls -- fixture-activity-center-beta-narrowed
cargo run -p aureline-shell --example dump_m5_background_work_controls -- fixture-background-work-preview-narrowed
cargo run -p aureline-shell --example dump_m5_background_work_controls -- validate
```
