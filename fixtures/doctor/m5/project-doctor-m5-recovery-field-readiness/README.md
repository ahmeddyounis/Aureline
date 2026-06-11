# Fixtures: Project Doctor M5 recovery field-readiness

This directory contains fixture metadata for the
`project_doctor_m5_recovery_field_readiness` packet.

The canonical full corpus is checked in at:

`artifacts/doctor/m5/project-doctor-m5-recovery-field-readiness.json`

## Coverage

- **Every M5 recovery lane** has a blocked-user scenario: `notebook_kernel`,
  `request_api`, `database_target`, `profiler_replay`, `preview_route`,
  `sync_device_registry`, `companion_handoff`, and `incident_packet`.
- **Every recovery-ladder rung** is exercised: `safe_mode`, `quarantine`,
  `open_without_restore`, `cache_index_repair`, `restricted_reopen`, and
  `typed_repair`. Only the `typed_repair` rung carries a `repair.`-prefixed
  `repair_id`.
- **Every diagnosis-latency budget** declares `p50`/`p90`/`p95` thresholds with
  `target_ms` < `yellow_ms` < `red_ms`, and the corpus records observed
  latencies at each percentile. The p90 first-actionable observation is the
  headline gate input.
- **Every drill outcome** appears: `diagnosed_and_handed_off`,
  `diagnosed_not_handed_off`, and `not_diagnosed`.
- **Every promotion action and narrowing reason** appears: `publish_full`/`none`
  (notebook, request, companion), `narrow_to_advisory` for `stale_corpus`
  (database, 40d corpus vs 30d window), `latency_breached` (profiler, observed
  p90 7000ms over the 6000ms red threshold), `evidence_missing` (preview, empty
  durable-evidence refs), `drill_not_handed_off` (sync), and `block_promotion`
  for `drill_not_diagnosed` (incident).
- **The non-inheriting promotion gate** is provable: every scenario's published
  `published_promotion_action` and `published_narrowing_reason` equal the
  decision recomputed from its own drill outcome, freshness, p90 latency state,
  and escalation completeness. Tampering with any input (staling the corpus,
  breaching the latency budget, removing durable evidence) makes the published
  decision diverge and fails validation.
- **Support-bundle parity** is preserved: every scenario carries a
  `bundle_manifest_ref`, an `escalation_packet_ref`, `preserved_finding_ids`,
  and `preserved_scope_refs` (stable identity, kept even when narrowed or
  blocked), and is metadata-safe (`redaction_class: metadata_safe_default`,
  `raw_private_material_excluded: true`, `overcapture_excluded: true`).
- **Cross-surface parity**: every scenario renders on `desktop_pane`, `cli_row`,
  `headless_json`, `support_export`, `incident_packet`, and `public_truth`, and
  carries the locale-invariant `machine_meaning_keys` (`scenario_id`, `lane`,
  `recovery_rung`, `drill_outcome`, `promotion_action`).
