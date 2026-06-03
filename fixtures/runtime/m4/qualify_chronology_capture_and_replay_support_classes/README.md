# qualify_chronology_capture_and_replay_support_classes fixture corpus

Fixture corpus for the M4 chronology-capture and replay/reverse-debug support-class
qualification truth packet
(`schemas/debug/chronology-replay-support.schema.json`).

Each fixture is a `ChronologyReplaySupportTruthPacketInput` with an `expect` block
that pins the materialized packet's promotion state, finding count, lane and
row-class token sets, vocabulary token sets, and the support-export safety verdict.
Tests in `crates/aureline-debug` load each case and assert that
`ChronologyReplaySupportPacket::materialize` agrees.

## Cases

- `baseline_stable.json` — All four debug lanes (`local_lane`, `remote_helper_lane`,
  `container_lane`, `notebook_bridge_lane`) carry a full `chronology_quality` headline
  row at `launch_stable`, the full five `support_class_admission` rows (supported,
  limited, view_only, unsupported, policy_blocked), the full four
  `capture_state_admission` rows (recorded, not_recorded,
  restart_with_recording_available, capture_unsupported), the full six
  `mapping_quality_badge_admission` rows (exact, approximate, partial, unavailable,
  stale, mismatched), the full three `replay_scope_admission` rows (local_scope,
  remote_scope, notebook_bridge_scope), the full five `inspector_state_admission`
  rows (live, snapshot, stale, limited, unavailable), the full four
  `restart_posture_admission` rows (available, unavailable_unsupported_backend,
  unavailable_policy_blocked, unavailable_no_live_session), the full eight
  `replay_surface_binding` rows (all except support_export_surface attesting
  `attests_replay_read_only=true`), and a `lineage_admission` row. All 13 required
  consumer projections preserve all vocabularies verbatim. Promotion state: stable.

- `unbound_evidence_blocks_stable.json` — The `local_lane` quality row carries
  `evidence_class=evidence_unbound` while claiming `launch_stable`. Two findings
  fire: `missing_evidence_class` and `launch_stable_with_unbound_binding`. Blocks
  stable promotion.

- `missing_replay_support_class_blocks_stable.json` — The `local_lane` is missing
  its `support_class_admission` row for `replay_support_class=supported`. One finding
  fires: `missing_replay_support_class_coverage`. Blocks stable promotion.

- `missing_capture_state_blocks_stable.json` — The `local_lane` is missing its
  `capture_state_admission` row for `capture_state_class=recorded`. One finding
  fires: `missing_capture_state_coverage`. Blocks stable promotion.

- `missing_mapping_quality_badge_blocks_stable.json` — The `local_lane` is missing
  its `mapping_quality_badge_admission` row for `mapping_quality_badge_class=exact`.
  One finding fires: `missing_mapping_quality_badge_coverage`. Blocks stable
  promotion.

- `missing_inspector_state_blocks_stable.json` — The `local_lane` is missing its
  `inspector_state_admission` row for `inspector_state_class=live`. One finding fires:
  `missing_inspector_state_coverage`. Blocks stable promotion.

- `replay_surface_missing_read_only_attestation_blocks_stable.json` — The
  `local_lane` `timeline_scrubber_surface` replay-surface binding row carries
  `attests_replay_read_only=false`. One finding fires:
  `replay_surface_missing_read_only_attestation`. Blocks stable promotion. (The
  `support_export_surface` is the only surface exempt from this attestation
  requirement.)

- `raw_source_material_blocks_stable.json` — The `local_lane` quality row has
  `raw_source_material_excluded=false`, admitting raw debug payloads past the packet
  boundary. One finding fires: `raw_source_material_present`. Blocks stable
  promotion.
