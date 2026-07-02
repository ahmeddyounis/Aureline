# M5 tooltip, hovercard & peek-panel representation, promotion, reach & stale-labeling

Generated from the seeded packet in
[`crate::m5_transient_inspect_certification`](../../crates/aureline-shell/src/m5_transient_inspect_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_transient_inspect_certification -- markdown > \
  artifacts/shell/m5-transient-inspect-certification.md
```

- Packet id: `m5-transient-inspect-certification:stable:0001`
- Source schema ref: `schemas/shell/m5-transient-inspect-certification.schema.json`
- Certifies matrix packet: `m5-shell-primitives:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 7
- Green: 3
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification dimensions

- `representation_truth`
- `promotion_continuity`
- `non_hover_reach`
- `stale_preview_labeling`

## Certification rows

| Context | Status | Qualification | Representation | Promotion | Reach | Stale | Tooltip | Waiver |
| ------- | ------ | ------------- | -------------- | --------- | ----- | ----- | ------- | ------ |
| Search results tooltips & peek | `green` | `stable` | `identity_source_freshness_representation_labeled` | `pin_open_paths_preserve_identity_and_state` | `keyboard_focus_context_reachable` | `stale_labeled_and_export_reconstructable` | `true` | — |
| Docs / help hovercards | `green` | `stable` | `identity_source_freshness_representation_labeled` | `pin_open_paths_preserve_identity_and_state` | `keyboard_focus_context_reachable` | `stale_labeled_and_export_reconstructable` | `true` | — |
| Review / change hovercards & peek | `yellow` | `stable` | `disclosed_reduced_representation_detail` | `pin_open_paths_preserve_identity_and_state` | `keyboard_focus_context_reachable` | `stale_labeled_and_export_reconstructable` | `true` | — |
| Editor symbol tooltips & peek | `green` | `stable` | `identity_source_freshness_representation_labeled` | `pin_open_paths_preserve_identity_and_state` | `keyboard_focus_context_reachable` | `stale_labeled_and_export_reconstructable` | `true` | — |
| Data grid cell hovercards & peek | `yellow` | `stable` | `identity_source_freshness_representation_labeled` | `pin_open_paths_preserve_identity_and_state` | `keyboard_focus_context_reachable` | `disclosed_partial_capture` | `true` | — |
| Profiler flame-graph peek | `yellow` | `stable` | `identity_source_freshness_representation_labeled` | `disclosed_reduced_promotion_path` | `keyboard_focus_context_reachable` | `stale_labeled_and_export_reconstructable` | `true` | `waiver:profiler-reduced-promotion-path:0001` |
| Operator console tooltips & peek | `yellow` | `stable` | `identity_source_freshness_representation_labeled` | `pin_open_paths_preserve_identity_and_state` | `disclosed_reduced_reach_route` | `stale_labeled_and_export_reconstructable` | `true` | — |

## Auto-narrowed rows

- `review_change` (`yellow`) — Under compact review width the change hovercard falls back to a disclosed, shorter representation of the diff hunk while the target identity, the provider-attributed source, and the freshness stay labeled; the reduction is disclosed and the row is narrowed below green.
- `data_grid` (`yellow`) — The data-grid cell peek's support export reconstructs the visible preview and discloses a partial capture of the pinned cached-snapshot set while the API run re-fetches; the partial capture is disclosed and the row is narrowed below green.
- `profiler` (`yellow`) — The profiler flame-graph peek pins and opens while preserving identity, state, and sampled-approximate freshness, but its detach-to-window promotion path is disclosedly deferred behind a waiver while the profiler window host stabilizes; the row is narrowed below green while the reduction is in force.
- `operator` (`yellow`) — On the seeded compact operator console one non-hover reach route (the hover info affordance) is temporarily reduced while the console re-lays-out; keyboard focus and the explicit context action still resolve every tooltip and peek, and the reduction is disclosed, so the row is narrowed below green.

## Exact certification causes

- `review_change` — `source_freshness_hidden` (disclosed: `true`) — Under compact width the hovercard falls back to a disclosed, shorter representation while the target identity, source/provider class, and freshness stay labeled; the reduction is disclosed and the row is narrowed below green.
- `data_grid` — `stale_preview_mistaken_for_live` (disclosed: `true`) — The support export reconstructs the visible preview and discloses a partial capture of the promoted / pinned set while a refresh is in flight; the partial capture is disclosed and the row is narrowed below green.
- `profiler` — `promotion_dropped_truth` (disclosed: `true`) — One promotion path (detach-to-window) is disclosedly deferred while pin and open still preserve the target identity, state, and representation truth; the reduction is disclosed and waivered, and the row is narrowed below green.
- `operator` — `hover_only_critical_truth` (disclosed: `true`) — One non-hover reach route is temporarily reduced; at least one route (keyboard focus, context action, or info affordance) still resolves and the reduction is disclosed.

## Active waivers

- `waiver:profiler-reduced-promotion-path:0001` (`profiler`, owner: Profiler surface owner, expires `2026-09-30T00:00:00Z`) — Under the seeded profiler capture the flame-graph peek can be pinned and opened to a full panel — both preserving its target identity, sampled-approximate freshness, and representation truth — but the detach-to-its-own-window promotion is deferred while the profiler window host stabilizes. The reduction is disclosed, never hidden, and the pinned preview stays reconstructable from the support export.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_transient_inspect_certification -- validate
cargo test -p aureline-shell --test m5_transient_inspect_certification_fixtures
```
