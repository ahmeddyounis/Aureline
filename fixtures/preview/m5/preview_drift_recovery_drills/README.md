# M5 Preview Drift-Recovery Drill Fixtures

## drift_recovery_preserves_truth_across_hot_reload_stale_map_and_reconnect.json

A drift-and-recovery drill set fixture proving the claimed M5 preview lanes fail
honestly across the failures users actually hit: hot-reload reset, stale source
map, lost dev server, device reconnect, expired browser session, replaced
runtime, and data-posture flip. Every drift event in the closed vocabulary is
drilled exactly once.

Each drill carries a `before` and an `after` truth snapshot over the shared
source-sync, data-posture, freshness, target, device-capability,
source-mapping-quality, and runtime-origin chips. The set demonstrates the
preserved-truth and event-honesty invariants:

- The `stale_source_map` drill holds `drifted_from_source` with a `stale` map
  rather than keeping its prior `exact` claim, and recovers via
  `remap_source_then_reload` / `hold_inspect_only_until_remapped`.
- The `dev_server_lost` and `browser_session_expired` drills drop their
  `runtime_backed` claim and fall back to a captured last frame, recording a
  precise degraded label and trigger.
- The `device_reconnect` drill preserves the same `physical_device_target` and
  runtime identity while `reconnect_required` holds the view below current.
- The `runtime_replaced` drill marks the view `drifted_from_source` instead of
  carrying the prior runtime's in-sync claim forward.
- The `hot_reload_reset` and `data_posture_flip` drills recover cleanly: the
  former preserves source-sync and target, the latter changes only the governed
  data chip.

Every degraded drill records a `downgrade_trigger` and a precise non-generic
`degraded_label` that `survives_reopen_export`; the two clean-recovery drills
carry neither. Every recovery route is admissible for its drift event.

The fixture validates against
`schemas/preview/preview_drift_recovery_drill_set.schema.json` and is byte-aligned
with the in-crate builder via
`cargo run -p aureline-preview --example dump_m5_preview_drift_recovery_drills`.
