# Queue Governor And Admission Control

This document freezes the M4 stable queue-governor and admission-control
truth packet. The Rust contract is
[`crates/aureline-runtime/src/queue_governor_and_admission_control/`](../../../crates/aureline-runtime/src/queue_governor_and_admission_control/),
the boundary schema is
[`schemas/runtime/queue-governor-and-admission-control.schema.json`](../../../schemas/runtime/queue-governor-and-admission-control.schema.json),
and the checked-in packet is
[`artifacts/runtime/m4/queue_governor_and_admission_control_packet.json`](../../../artifacts/runtime/m4/queue_governor_and_admission_control_packet.json).

## Stable Vocabulary

The packet reuses the shared governor lanes and health states from the
resource-governor contract: `foreground`, `interactive_background`,
`maintenance`, `provider_overlay`, `upload_replication`, and
`nominal`, `constrained`, `degraded`, `protect_core`, `recovery`.

Every background job carries `job_kind`, `workspace_id`, optional
`slice_id`, root or target scope, initiating source, collapse key, lane,
budget-domain set, checkpoint policy, staleness policy, and cancellation
contract. Jobs self-invalidate when workspace revision, workset or slice
manifest, execution-context hash, or policy epoch changes.

## Stable Rules

- Background jobs may not borrow `hot_path_interactive_budget`.
- Provider overlay and upload/replication lanes use retry budgets separate
  from local interactive and knowledge-refresh work.
- Duplicate jobs collapse by collapse key; superseded provider and index
  work is replaced or restarted from a checkpoint.
- Runtime health projections, shell/activity-center rows, diagnostics,
  CLI/headless inspection, and support export use the same pause reason,
  resume owner, and affected-lane tokens.
- Queue summaries expose queue depth, oldest age, collapse count, last
  checkpoint, shed work, protected data class, and resume condition without
  raw user content.

## Pressure Labs

The fixture corpus covers low memory, low disk, battery saver, thermal
pressure, provider quota exhaustion, and offline transitions. Each case
asserts that typing, save, navigation, quick open, and explicit cancellation
remain protected while paused or coalesced background work is named.

## Accessibility And Design Contract

Queue strips, per-lane rows, deferred-work banners, and budget-pressure cards
must show text labels for state, pause reason, resume condition, oldest age,
and checkpoint status. The state may not be toast-only, color-only, or
mouse-only.
