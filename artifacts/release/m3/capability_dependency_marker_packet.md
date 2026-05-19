# Capability Dependency Marker Release-Evidence Packet

This packet collects the release-evidence rows the capability
dependency-marker contract publishes for stable- and beta-facing
artifacts. It exists so a reviewer reading the release evidence can
verify that every claimed artifact carries explicit dependency
markers, fallback notes, and typed import/downgrade behavior on
targets lacking Labs, Preview, Beta-only, policy-gated, or
host-specific capabilities.

## Contract refs

- Boundary schema for capability records: [`schemas/capabilities/capability_record.schema.json`](../../../schemas/capabilities/capability_record.schema.json)
- Boundary schema for artifact dependency markers: [`schemas/capabilities/artifact_dependency_marker.schema.json`](../../../schemas/capabilities/artifact_dependency_marker.schema.json)
- Runtime projection: [`crates/aureline-capabilities/src/dependency_markers/mod.rs`](../../../crates/aureline-capabilities/src/dependency_markers/mod.rs)
- Reviewer-facing contract: [`docs/ux/m3/capability_dependency_marker_beta.md`](../../../docs/ux/m3/capability_dependency_marker_beta.md)
- Fixture corpus: [`fixtures/capabilities/m3/dependency_markers/`](../../../fixtures/capabilities/m3/dependency_markers/)
- Fixture-replay test: [`crates/aureline-capabilities/tests/dependency_markers_fixtures.rs`](../../../crates/aureline-capabilities/tests/dependency_markers_fixtures.rs)

## Closed vocabularies

| Axis | Closed values |
| --- | --- |
| `artifact_class` | `settings_export`, `profile`, `workflow_bundle`, `portable_state_package`, `recipe`, `saved_view`, `migration_packet`, `support_export`, `sync_artifact` |
| `dependency_class` | `labs`, `preview`, `beta_only`, `policy_gated`, `host_specific` |
| `required_lifecycle_state` | `labs`, `preview`, `beta`, `stable`, `lts_facing`, `deprecated`, `disabled_by_policy`, `retired` |
| `support_promise` | `best_effort`, `community_supported`, `standard_support`, `extended_support`, `operator_only`, `no_support` |
| `effect_on_import` | `block_apply_preserve_data`, `narrow_behavior_preserve_data`, `emulated_downgrade_preserve_data`, `hold_for_later_preserve_data`, `render_tombstone_preserve_data` |
| `host_surface` (projection) | `settings_inspector`, `import_review_sheet`, `bundle_detail_page`, `downgrade_flow`, `headless_cli_inspect`, `docs_help_page` |

Every variant of `effect_on_import` ends in `_preserve_data`. The
closed vocabulary itself is the proof that the contract forbids
silent drop of user-authored data.

## Coverage matrix

The fixture corpus exercises every `artifact_class` against the
dependency classes listed in this packet. Each row is a fixture under
[`fixtures/capabilities/m3/dependency_markers/`](../../../fixtures/capabilities/m3/dependency_markers/).

| Fixture | `artifact_class` | `dependency_class` | `effect_on_import` | Kill switch |
| --- | --- | --- | --- | --- |
| `settings_export_labs_marker.json` | `settings_export` | `labs` | `narrow_behavior_preserve_data` | inactive |
| `profile_preview_marker.json` | `profile` | `preview` | `emulated_downgrade_preserve_data` | inactive |
| `workflow_bundle_beta_marker.json` | `workflow_bundle` | `beta_only` | `hold_for_later_preserve_data` | inactive |
| `portable_state_package_policy_gated_marker.json` | `portable_state_package` | `policy_gated` | `block_apply_preserve_data` | **active** |
| `recipe_host_specific_marker.json` | `recipe` | `host_specific` | `render_tombstone_preserve_data` | inactive |
| `saved_view_preview_marker.json` | `saved_view` | `preview` | `emulated_downgrade_preserve_data` | inactive |
| `migration_packet_beta_marker.json` | `migration_packet` | `beta_only` | `hold_for_later_preserve_data` | inactive |
| `support_export_labs_marker.json` | `support_export` | `labs` | `narrow_behavior_preserve_data` | inactive |
| `sync_artifact_host_specific_marker.json` | `sync_artifact` | `host_specific` | `render_tombstone_preserve_data` | inactive |

## Acceptance evidence

1. **Saved / exported / imported artifacts do not lose critical
   meaning silently when moved to a target lacking the required
   capability.** Every fixture in the coverage matrix uses an
   `effect_on_import` from the closed `_preserve_data` set; the
   fixture-replay test asserts
   `user_authored_data_preserved = true` for every projection on
   every host surface.
2. **Stable and beta product surfaces disclose hidden Labs / Preview /
   policy-gated dependencies before apply, not after failure.** The
   `MarkerHostProjection.blocks_apply_until_disclosed` flag is true
   on import-review sheets and downgrade flows; the validator
   rejects markers with empty `fallback_path` or `summary`.
3. **Docs, support exports, and product UI can reconstruct which
   dependent capability was present, missing, disabled, or
   downgraded.** Each fixture carries `required_capability_id`,
   `required_lifecycle_state`, `dependency_class`, `support_promise`,
   `effect_on_import`, `behavior_on_missing.summary`,
   `behavior_on_missing.fallback_path`, `kill_switch_active`, and
   (where applicable) `host_scope`. The shared projection guarantees
   the same vocabulary across every surface.

## Verification commands

Run the fixture-replay tests and the per-marker unit tests with:

```sh
cargo test -p aureline-capabilities
```

## Out of scope

- Hosted experimentation analytics or broad experimentation governance
  beyond marker fidelity, portability, and downgrade honesty.
- Live RPC of raw provider tokens, raw policy-bundle bytes, or raw
  kill-switch material; the marker carries ids and typed vocabulary
  only.
