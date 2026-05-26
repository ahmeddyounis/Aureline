# navigation_target_truth_packet fixture corpus

Fixture corpus for the M4 stable navigation-target truth packet
(`schemas/search/navigation_target_truth_packet.schema.json`).

Each fixture is a `NavigationTargetTruthPacketInput` with an `expect`
block pinning the materialized packet's promotion state, finding count,
row/access/downgrade token sets, and support-export posture. Tests in
`crates/aureline-graph/tests/navigation_target_truth_packet.rs` load
each case and assert that `NavigationTargetTruthPacket::materialize`
agrees.

Cases:

- `baseline_stable.json` — Rows cover every required row class
  (`definition`, `declaration`, `implementation`, `reference`,
  `call_hierarchy_edge`, `type_hierarchy_edge`, `related_object`,
  `rename_preview`). Reference rows preserve every required access kind
  (`read`, `write`, `call`, `inherit`, `import`, `export`,
  `route_binding`, `test_only`, `generated`, `runtime_observed`).
  The seven required consumer projections preserve the packet
  verbatim, so the packet materializes as `stable`.
- `silent_relation_alias_blocks_stable.json` — A definition row
  reports `relation_kind=declaration` while staying `canonical`, so
  `Go to Definition` would silently fall back to a declaration jump
  without an exportable reason. The validator emits
  `silent_relation_alias_present` and blocks the stable claim.
- `reference_missing_access_context_blocks_stable.json` — The `read`
  reference row drops its `reference_context`, so the find-references
  surface would lose the access-kind label. The validator emits
  `reference_missing_access_context` plus `missing_access_kind_coverage`
  and blocks the stable claim.
- `aliased_due_to_shallow_provider_missing_context_blocks_stable.json`
  — A definition row downgrades to `aliased_due_to_shallow_provider`
  because its provider is `syntax_fallback`, but the row drops its
  `aliasing_context` (no aliased_to_relation, reason token, or
  evidence ref). The validator emits
  `aliasing_context_missing_for_downgrade` and blocks the stable claim.
- `consumer_projection_drops_access_kind_blocks_stable.json` — The
  `ai_context` consumer projection drops the access-kind vocabulary,
  so the AI evidence pane could collapse `read`/`write`/`call`/etc.
  into one generic occurrence label. The validator emits
  `access_kind_vocabulary_dropped` plus `consumer_projection_drift`
  and blocks the stable claim.
- `rename_preview_missing_context_blocks_stable.json` — The
  rename-preview row drops its `rename_preview_context`, so
  blocked-candidate counts, sparse-scope omissions, and conflict
  notes would disappear from review and support surfaces. The
  validator emits `rename_preview_missing_context` and blocks the
  stable claim.
