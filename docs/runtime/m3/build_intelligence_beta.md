# Build Intelligence Health, Receipts, and Discovery Diff (Beta)

This beta layer turns target-confidence labels into a release-grade build
intelligence packet. Runtime, shell, CLI/headless, and support exports now read
one record family for:

- adapter-health strips;
- target rows and run-config cards;
- build/test receipts;
- discovery refresh diff reviews.

Implementation:

- [`/crates/aureline-runtime/src/build_intelligence/`](../../../crates/aureline-runtime/src/build_intelligence/)
- [`/crates/aureline-shell/src/build_intelligence_beta/`](../../../crates/aureline-shell/src/build_intelligence_beta/)
- [`/schemas/runtime/adapter_health_strip.schema.json`](../../../schemas/runtime/adapter_health_strip.schema.json)
- [`/schemas/runtime/discovery_diff.schema.json`](../../../schemas/runtime/discovery_diff.schema.json)

## Closed Lane Model

Build intelligence uses five lane tokens:

| Lane | Meaning |
| --- | --- |
| `native_adapter` | first-party or certified build adapter |
| `structured_protocol` | BSP or comparable structured protocol |
| `build_event_stream` | BEP/BES or comparable structured event stream |
| `structured_output_import` | parsed machine-readable artifact without live adapter truth |
| `heuristic_fallback` | script, matcher, output, or history inference |

These tokens are preserved in adapter-health strips, target rows, run-config
cards, receipts, shell panels, plaintext summaries, and support exports.

## Health Reasons

Adapter-health strips use precise reason tokens:

- `transport_failure`
- `auth_failure`
- `workspace_mismatch`
- `version_skew`
- `unsupported_features`
- `parse_ambiguity`
- `stale_artifact`
- `control_plane_outage`
- `managed_workspace_lifecycle_change`

Partial remote or managed rows can expose both `continue_local` and
`inspect_only` actions without relabeling imported or partial truth as live.

## Imported Versus Live

Target rows, run-config cards, and receipts carry an
`imported_live_state_token`:

- `live_workspace_inspection`
- `imported_artifact`
- `replayed_receipt`
- `heuristic_inference`
- `mixed_live_and_imported`

Receipts also carry `artifact_source_token`, an explicit
`imported_or_replayed_note`, and `high_trust_action_posture_token`. Imported
or replayed truth remains inspectable and exportable, but rerun/publish paths
must refresh or review before treating it as current workspace state.

## Discovery Diff

`DiscoveryDiffReview` separates refresh changes into:

- `added`
- `removed`
- `renamed`
- `downgraded_confidence`
- `newly_heuristic`
- `newly_exact`
- `now_unresolved`

Stable target IDs are the join key. If a target keeps its ID but changes name,
the diff reports `renamed`; if confidence drops or the row becomes heuristic
or unresolved, those are separate buckets.

## Fixtures And Packets

Fixtures:

- [`/fixtures/runtime/m3/build_intelligence_confidence/all_lanes_refresh_diff.json`](../../../fixtures/runtime/m3/build_intelligence_confidence/all_lanes_refresh_diff.json)

Governed packet examples:

- [`/artifacts/runtime/m3/build_intelligence_packets/closed_vocabularies.yaml`](../../../artifacts/runtime/m3/build_intelligence_packets/closed_vocabularies.yaml)
- [`/artifacts/runtime/m3/build_intelligence_packets/support_export_projection.json`](../../../artifacts/runtime/m3/build_intelligence_packets/support_export_projection.json)

## Verify

```sh
cargo test -p aureline-runtime build_intelligence
cargo test -p aureline-runtime --test build_intelligence_beta
cargo test -p aureline-shell build_intelligence_beta
```
