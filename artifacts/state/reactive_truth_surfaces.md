# Reactive-truth surfaces — evidence report

Every derived M5 surface ships one canonical reactive-truth cue — source
authority, freshness, completeness, invalidation reason, backpressure, the
narrowed claim, and an action gate — instead of feature-local stale-state
prose. The cue layer is implemented in
[`crates/aureline-reactive-state/src/reactive_truth_surfaces/mod.rs`](../../crates/aureline-reactive-state/src/reactive_truth_surfaces/mod.rs)
and serialized to
[`artifacts/state/reactive_truth_surfaces.json`](./reactive_truth_surfaces.json).

It is derived from the canonical governance matrix in
[`artifacts/state/m5_reactive_governance.json`](./m5_reactive_governance.json),
so the gate, invalidation reason, and resubscribe cue can never fork the
narrowing engine.

## Invariants

- Every derived M5 surface renders one canonical reactive-truth cue carrying source authority, freshness, completeness, invalidation reason, backpressure, and the narrowed claim instead of feature-local stale-state prose.
- No derived surface presents exact current truth; the strongest cue is a consistent snapshot at its authoritative epoch.
- Dangerous derived actions stay live only at a consistent snapshot; coalesced or cached truth must revalidate, partial or warming truth narrows to read-only, and stale, replayed, imported, policy-limited, or provider-unavailable truth blocks them.
- Terminal streams, snapshot-required backpressure, and unavailable scopes set a resubscribe-required cue instead of hiding behind a generic spinner.
- The gate, invalidation reason, and resubscribe cue are derived from the canonical narrowing engine, so UI, CLI/headless, activity-center, accessibility, diagnostics, and support/export channels narrow identically.

## Per-surface action gating

| surface | authority | view class | healthy claim | healthy gate | gated rules |
| --- | --- | --- | --- | --- | --- |
| `ai_context_panel` | `derived_knowledge` | `ephemeral_projection` | `consistent_snapshot` | `enabled` | 6 |
| `companion_panel` | `provider_overlay` | `managed_replicated_view` | `consistent_snapshot` | `enabled` | 10 |
| `docs_browser` | `derived_knowledge` | `durable_local_materialization` | `consistent_snapshot` | `enabled` | 6 |
| `editor_buffer_outline` | `buffer_editor` | `ephemeral_projection` | `consistent_snapshot` | `enabled` | 5 |
| `graph_neighborhood` | `derived_knowledge` | `ephemeral_projection` | `consistent_snapshot` | `enabled` | 5 |
| `headless_workspace_mirror` | `workspace_vfs` | `ephemeral_projection` | `consistent_snapshot` | `enabled` | 5 |
| `policy_trust_banner` | `policy_entitlement` | `ephemeral_projection` | `consistent_snapshot` | `enabled` | 4 |
| `preview_output` | `execution` | `exportable_snapshot` | `consistent_snapshot` | `enabled` | 3 |
| `review_workspace` | `provider_overlay` | `managed_replicated_view` | `consistent_snapshot` | `enabled` | 11 |
| `search_results` | `derived_knowledge` | `durable_local_materialization` | `consistent_snapshot` | `enabled` | 7 |
| `shell_activity_center` | `execution` | `durable_local_materialization` | `consistent_snapshot` | `enabled` | 8 |
| `shell_workspace_tree` | `workspace_vfs` | `durable_local_materialization` | `consistent_snapshot` | `enabled` | 6 |
| `support_export_view` | `derived_knowledge` | `exportable_snapshot` | `consistent_snapshot` | `enabled` | 3 |

## Deterministic audit projection

```
Gated reactive-truth cues shipped across the derived M5 surfaces
surface | authority | view_class | healthy_claim | healthy_gate | gated_rules
ai_context_panel | derived_knowledge | ephemeral_projection | consistent_snapshot | enabled | 6
companion_panel | provider_overlay | managed_replicated_view | consistent_snapshot | enabled | 10
docs_browser | derived_knowledge | durable_local_materialization | consistent_snapshot | enabled | 6
editor_buffer_outline | buffer_editor | ephemeral_projection | consistent_snapshot | enabled | 5
graph_neighborhood | derived_knowledge | ephemeral_projection | consistent_snapshot | enabled | 5
headless_workspace_mirror | workspace_vfs | ephemeral_projection | consistent_snapshot | enabled | 5
policy_trust_banner | policy_entitlement | ephemeral_projection | consistent_snapshot | enabled | 4
preview_output | execution | exportable_snapshot | consistent_snapshot | enabled | 3
review_workspace | provider_overlay | managed_replicated_view | consistent_snapshot | enabled | 11
search_results | derived_knowledge | durable_local_materialization | consistent_snapshot | enabled | 7
shell_activity_center | execution | durable_local_materialization | consistent_snapshot | enabled | 8
shell_workspace_tree | workspace_vfs | durable_local_materialization | consistent_snapshot | enabled | 6
support_export_view | derived_knowledge | exportable_snapshot | consistent_snapshot | enabled | 3

```
