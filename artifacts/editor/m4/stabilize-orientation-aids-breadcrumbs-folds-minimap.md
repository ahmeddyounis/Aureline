# M04-192 — Stabilize breadcrumbs, multi-cursor, fold summaries, minimap/overview markers, and orientation-aid truth

## Release evidence

This artifact documents the governed, export-safe orientation-aids stability packet produced by `crates/aureline-editor/src/stabilize_orientation_aids_breadcrumbs_folds_minimap/`.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `OrientationAidsStabilityPacket` | `orientation_aids_stability_packet` | `schemas/editor/orientation_aid_state.schema.json` | 1 |
| `OrientationAidStateRecord` | `orientation_aid_state_record` | (embedded in above) | 1 |
| `FoldSummaryStateRecord` | `fold_summary_state_record` | (embedded in above) | 1 |

## Honesty invariants (all must pass)

1. Multi-cursor or column-selection count, posture, and undo grouping are visible.
2. Fold summaries preserve hidden critical-state cues and expose detail routes.
3. Breadcrumb jumps preserve back/forward continuity and expose alternate routes.
4. Reduced or disabled orientation aids name an alternate route and accessibility label.
5. Gutter, minimap, overview ruler, and breadcrrow share one marker-family vocabulary.
6. Degraded mode classes are explicit whenever any orientation aid is narrowed.
7. Orientation-aid latency budget is ≤ 1,000 µs on claimed stable rows.
8. Typing budget is ≤ 500 µs.
9. Scroll budget is ≤ 1,000 µs.
10. File-switch budget is ≤ 2,000 µs.

## Claimed-stable corpus

Generated and pinned under `fixtures/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap/`.

| Scenario | Surface | Degraded modes | Carets |
|---|---|---|---|
| source_editor_full_fidelity | editor_source | 0 | 3 |
| diff_surface_column_selection | editor_diff | 0 | 5 |
| review_surface_full_fidelity | review_thread | 0 | 3 |
| large_file_degraded | editor_source | 1 | 3 |
| low_resource_degraded | editor_source | 1 | 3 |
| reduced_motion_degraded | editor_source | 1 | 3 |
| high_contrast_degraded | editor_source | 1 | 3 |
| battery_saver_degraded | editor_source | 1 | 3 |
| restricted_mode_degraded | editor_source | 1 | 3 |
| fold_with_critical_hidden_state | editor_source | 0 | 3 |

## Verification

Build and run the corpus emitter:

```sh
cargo run --bin aureline_orientation_aids_stability
```

Run the integration test:

```sh
cargo test --test stabilize_orientation_aids_replay
```

Check contract validity for every scenario:

```sh
cargo test -p aureline-editor stabilize_orientation_aids
```

## Risks and follow-ups

- **Performance budgets are claims, not measured guarantees.** The 500 µs typing, 1,000 µs scroll, and 2,000 µs file-switch budgets need benchmark-lab backing before they can be promoted to published performance claims.
- **Large-file and low-resource downgrades disable rather than simplify.** A future iteration may support simplified minimap rendering instead of full suppression.
- **Column-selection posture is modeled but not yet exercised by the live selection module.** The diff-surface corpus scenario projects the expected stable shape; wiring to the live column-selection implementation requires follow-up in the selection/viewport system.
- **Breadcrumb continuity across moved files is a structural claim.** The current model records `back_forward_preserved: true`; verifying this against the live navigation history graph requires integration with the graph-backed navigation target system.
