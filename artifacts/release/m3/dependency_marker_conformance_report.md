# Capability Dependency Marker Conformance Report

This report is the release-evidence packet for the dependency-marker
contract on every M3 artifact path. It exists so a reviewer reading
the release evidence can verify that every claimed beta artifact path
preserves capability markers through import, export, sync, backup /
restore, mirror-only, offline-cache-only, headless / CLI inspect, and
companion-targeted handoff, and that downgrade and missing-capability
scenarios warn before apply instead of silently failing after commit.

A row in this report is the proof product, docs, support, and
shiproom use to decide whether a path still deserves its claimed
lifecycle / support class.

## Contract refs

- Boundary schema for capability records: [`schemas/capabilities/capability_record.schema.json`](../../../schemas/capabilities/capability_record.schema.json)
- Boundary schema for artifact dependency markers: [`schemas/capabilities/artifact_dependency_marker.schema.json`](../../../schemas/capabilities/artifact_dependency_marker.schema.json)
- Runtime projection: [`crates/aureline-capabilities/src/dependency_markers/mod.rs`](../../../crates/aureline-capabilities/src/dependency_markers/mod.rs)
- Transport-lane replay: [`crates/aureline-capabilities/src/dependency_markers/transport_lanes.rs`](../../../crates/aureline-capabilities/src/dependency_markers/transport_lanes.rs)
- Downgrade / missing-capability evaluator: [`crates/aureline-capabilities/src/dependency_markers/downgrade.rs`](../../../crates/aureline-capabilities/src/dependency_markers/downgrade.rs)
- Reviewer-facing contract: [`docs/ux/m3/capability_dependency_marker_beta.md`](../../../docs/ux/m3/capability_dependency_marker_beta.md)
- Predecessor packet (single-marker fixture coverage): [`capability_dependency_marker_packet.md`](./capability_dependency_marker_packet.md)
- Import / export / sync / mirror / offline / headless / companion fixture corpus: [`fixtures/capabilities/m3/import_export_dependency_markers/`](../../../fixtures/capabilities/m3/import_export_dependency_markers/)
- Downgrade / missing-capability fixture corpus: [`fixtures/capabilities/m3/downgrade_and_missing_capability/`](../../../fixtures/capabilities/m3/downgrade_and_missing_capability/)
- Fixture-replay test: [`crates/aureline-capabilities/tests/dependency_markers_replay.rs`](../../../crates/aureline-capabilities/tests/dependency_markers_replay.rs)
- Migration guidance: [`docs/migration/m3/dependency_marker_downgrade_guidance.md`](../../../docs/migration/m3/dependency_marker_downgrade_guidance.md)

## Closed vocabularies extended by this packet

| Axis | Closed values |
| --- | --- |
| `transport_lane` | `import`, `export`, `sync`, `backup_restore`, `mirror_only`, `offline_cache_only`, `headless_cli_inspect`, `companion_handoff` |
| `downgrade_scenario` | `stable_to_preview`, `preview_to_stable`, `host_change`, `mirror_only`, `offline_cache_only`, `policy_disabled` |
| `effective_effect_on_import` | reuses the closed `effect_on_import` vocabulary (`block_apply_preserve_data`, `narrow_behavior_preserve_data`, `emulated_downgrade_preserve_data`, `hold_for_later_preserve_data`, `render_tombstone_preserve_data`) |
| `effective_support_promise` | reuses the closed `support_promise` vocabulary (`best_effort`, `community_supported`, `standard_support`, `extended_support`, `operator_only`, `no_support`) |

Each `effective_effect_on_import` ends in `_preserve_data`: the closed
vocabulary itself is the proof that no lane and no downgrade scenario
is allowed to drop user-authored data.

## Transport-lane coverage matrix

The lane fixture corpus exercises every entry of the closed
`TransportLane` set against the relevant artifact class. Each row is a
fixture under
[`fixtures/capabilities/m3/import_export_dependency_markers/`](../../../fixtures/capabilities/m3/import_export_dependency_markers/).

| Fixture | `transport_lane` | `artifact_class` | Pre-apply disclosure | Read-only | Companion-safe |
| --- | --- | --- | --- | --- | --- |
| `import_settings_and_workflow_bundle.json` | `import` | `migration_packet` | yes | no | yes |
| `export_profile_preview.json` | `export` | `profile` | no | yes | yes |
| `sync_artifact_host_specific.json` | `sync` | `sync_artifact` | yes | no | yes |
| `backup_restore_portable_state_policy_gated.json` | `backup_restore` | `portable_state_package` | yes | no | yes |
| `mirror_only_settings_labs.json` | `mirror_only` | `settings_export` | yes | no | yes |
| `offline_cache_only_workflow_bundle.json` | `offline_cache_only` | `workflow_bundle` | yes | no | yes |
| `headless_cli_inspect_support_export.json` | `headless_cli_inspect` | `support_export` | no | yes | yes |
| `companion_handoff_saved_view.json` | `companion_handoff` | `saved_view` | yes | no | yes |

Every fixture in this matrix is replayed by
[`every_lane_fixture_survives_all_lanes_with_zero_defects`](../../../crates/aureline-capabilities/tests/dependency_markers_replay.rs)
through every lane, not just its named lane. The test asserts
`LaneReplayOutcome::matches_source = true` for every (marker, lane)
pair.

## Downgrade scenario coverage matrix

The downgrade fixture corpus covers every entry of the closed
`DowngradeScenario` set. Each row is a fixture under
[`fixtures/capabilities/m3/downgrade_and_missing_capability/`](../../../fixtures/capabilities/m3/downgrade_and_missing_capability/).

| Fixture | `downgrade_scenario` | `artifact_class` | `effective_effect_on_import` | `effective_support_promise` | `support_claim_narrowed` | `kill_switch_active` |
| --- | --- | --- | --- | --- | --- | --- |
| `stable_to_preview_profile.json` | `stable_to_preview` | `profile` | `emulated_downgrade_preserve_data` | `best_effort` | yes | no |
| `preview_to_stable_saved_view.json` | `preview_to_stable` | `saved_view` | `emulated_downgrade_preserve_data` | `standard_support` | no | no |
| `host_change_recipe.json` | `host_change` | `recipe` | `render_tombstone_preserve_data` | `no_support` | yes | no |
| `mirror_only_settings_export.json` | `mirror_only` | `settings_export` | `narrow_behavior_preserve_data` | `best_effort` | no | no |
| `offline_cache_only_workflow_bundle.json` | `offline_cache_only` | `workflow_bundle` | `hold_for_later_preserve_data` | `best_effort` | yes | no |
| `policy_disabled_portable_state.json` | `policy_disabled` | `portable_state_package` | `block_apply_preserve_data` | `no_support` | yes | yes |

Every downgrade fixture is evaluated by
[`downgrade_fixtures_evaluate_the_named_scenario_and_match_expectations`](../../../crates/aureline-capabilities/tests/dependency_markers_replay.rs).
The test asserts the `CompareApplyReviewSheet` it builds carries
`apply_held_until_disclosed = true`, a non-empty
`portability_consequence`, a non-empty `safe_fallback`,
`user_authored_data_preserved = true`, and the expected effective
effect / support promise. The corpus also feeds
[`every_downgrade_marker_evaluates_through_all_scenarios_without_defects`](../../../crates/aureline-capabilities/tests/dependency_markers_replay.rs)
to prove no scenario silently narrows.

## Acceptance evidence

1. **Capability markers persist across export / import and are not
   stripped by mirror / offline / headless / companion paths.** The
   replay test
   [`every_lane_fixture_survives_all_lanes_with_zero_defects`](../../../crates/aureline-capabilities/tests/dependency_markers_replay.rs)
   round-trips every marker in the lane corpus through every
   [`TransportLane`] and asserts every vocabulary token (artifact
   class, dependency class, lifecycle state, support promise,
   effect-on-import, required capability id) matches the source
   marker bit-for-bit. A vocabulary drift on any lane fails the
   release.
2. **Downgrade and missing-capability scenarios warn before apply and
   preserve enough metadata for support / export reconstruction.**
   Every row of the downgrade matrix produces a
   `CompareApplyReviewSheet` with `apply_held_until_disclosed = true`
   and a non-empty `portability_consequence` plus `safe_fallback`.
   The
   [`assert_downgrade_review_sheets`](../../../crates/aureline-capabilities/src/dependency_markers/downgrade.rs)
   harness also rejects silent support-claim narrowing.
3. **Rows that fail the corpus downgrade their support or lifecycle
   claim instead of shipping silent portability breakage.** The
   `effective_support_promise` column of the downgrade matrix above
   shows the bounded support claim each scenario projects. The
   evaluator never elevates a recorded promise (the closed
   `support_rank` ordering enforces this); it only narrows when the
   target reduces guarantees, and it sets
   `support_claim_narrowed = true` whenever it does.
4. **Compare/apply review sheets always show the dependency marker,
   portability consequence, and safe fallback.** The
   [`CompareApplyReviewSheet`](../../../crates/aureline-capabilities/src/dependency_markers/downgrade.rs)
   struct requires every field; the harness rejects rows missing
   portability copy or safe fallback. The fixture corpus exercises
   every scenario and asserts the harness emits well-formed rows.

## Verification commands

Run the dependency-marker test suite (unit + fixture + replay) with:

```sh
cargo test -p aureline-capabilities
```

Run just the replay test:

```sh
cargo test -p aureline-capabilities --test dependency_markers_replay
```

## Out of scope

- Full ecosystem-wide dependency introspection beyond the closed
  Aureline-owned artifact families listed in the
  [`ArtifactClass`] vocabulary and the M3 companion / headless
  surfaces called out by the spec.
- Live RPC of raw provider tokens, raw policy-bundle bytes, or raw
  kill-switch material on any lane. The marker schema carries ids and
  typed vocabulary only; the replay harness reuses the schema's
  guarantee that companion lanes never receive raw bytes.
- Per-product UX rollout of the review sheet (the `CompareApplyReviewSheet`
  is data-only; downstream UI crates compose it with their existing
  import-review and downgrade surfaces).
