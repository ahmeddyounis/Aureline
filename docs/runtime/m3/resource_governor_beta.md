# Resource Governor Beta

The beta resource-governor surface exposes overload and throttling truth across
runtime, shell, status, and diagnostics. It does not implement a new scheduler.
It freezes the shape every scheduler, shell row, support export, and diagnostic
drill projects when background work is slowed, paused, coalesced, checkpointed,
or denied.

## Runtime Record

`aureline-runtime::resource_governor` owns:

- `ResourceGovernorSnapshot` for the current workspace/profile sample.
- `QueueLaneState` for the five shared lanes: `foreground`,
  `interactive_background`, `maintenance`, `provider_overlay`, and
  `upload_replication`.
- `PressureInput` for CPU, memory, disk, battery/thermal, network, and
  optional-service quota.
- `AdmissionControlDecision` for admitted foreground work and deferred or
  denied background work.
- `OverrideSheet` for workspace/profile override controls, including blocked
  override explanations.
- `ResourceGovernorSupportExport` for support bundles and diagnostics.

The snapshot validator requires every pressure dimension, every queue lane,
protected foreground actions, named affected work, collapse counts, checkpoint
metadata, and blocked override explanations.

## Shell Projection

`aureline-shell::background_work_status` projects the runtime snapshot into:

- a status-bar item with the governor state, most affected lane, queue depth,
  and detail command;
- lane rows with queue age, collapse count, checkpoint label, affected work,
  and what remains usable;
- deferred-work banners for paused or denied lanes;
- pressure cards that explain what was shed and what stayed protected;
- override rows that explain allowed or blocked controls;
- a shell support export that mirrors the visible rows and cards.

## Protected Foreground Contract

The beta packet always names the protected foreground actions that outrank
speculative or maintenance work:

- editing;
- save;
- explicit cancellation;
- quick open;
- navigation.

Core interaction and core navigation admission decisions must remain `admit`.
Background and optional work may be narrowed, paused, deferred, or denied only
with visible reason, lane class, checkpoint/replay note where applicable, and
support-export parity.

## Evidence

- Schema: `schemas/runtime/queue_lane_state.schema.json`
- Fixtures: `fixtures/runtime/m3/resource_governor_and_queue_truth/`
- Packet: `artifacts/runtime/m3/queue_and_pressure_packets/packet.json`
- Support projection:
  `artifacts/runtime/m3/queue_and_pressure_packets/support_export_projection.json`
