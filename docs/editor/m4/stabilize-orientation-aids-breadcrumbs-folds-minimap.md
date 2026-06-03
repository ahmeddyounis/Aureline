# Stabilize breadcrumbs, multi-cursor, fold summaries, minimap/overview markers, and orientation-aid truth

## Scope

This document defines the M04-192 stable contract for orientation aids in the Aureline editor. It covers the multi-cursor indicator, fold summaries with hidden-state preservation, breadcrumb continuity across file moves and renames, minimap and overview-ruler marker semantics, gutter-rail marker vocabulary, and explicit degraded-mode labeling under constrained profiles.

## Goals

- **Multi-cursor state is visible before any transform.** Users can tell how many insertion points will be affected, what undo grouping will apply, and which commands reach equivalent actions.
- **Folded regions preserve summarized cues.** Diagnostics, search hits, merge conflicts, suspicious text, comments, and trust-relevant markers inside a fold remain discoverable through the summary and a detail route.
- **Minimap and overview-ruler markers are inspectable and optional.** They correspond only to canonical sources such as diagnostics, search hits, comments, conflicts, or breakpoints. They remain suppressible in reduced-distraction, power-saver, or assistive contexts.
- **Breadcrumb continuity is preserved.** Path state stays aligned with canonical file identity and semantic freshness across moved files, renamed symbols, and back/forward history.
- **Constrained-profile downgrades are explicit.** Large-file mode, reduced-motion, high-contrast, battery-saver, and assistive-technology postures that narrow orientation aids must carry a visible degraded-state label and keep alternate routes available.
- **No aid is the sole carrier of critical state.** If a profile suppresses an aid, Problems, Search, Review, and Outline remain canonical.

## Surfaces

| Surface | Purpose | Minimum state |
|---|---|---|
| **Multi-cursor indicator** | Declare how many carets will receive the next edit | Caret count, posture (multiple carets / column selection), undo grouping class, primary caret label |
| **Fold summary** | Keep hidden region state honest | Hidden line count, hidden marker counts by family, critical-state preserved flag, toggle command, detail route |
| **Breadcrumb row** | Preserve path and navigation continuity | File identity ref, symbol path freshness, back/forward preserved, alternate command refs, visible state note |
| **Gutter rail** | Share marker vocabulary across all surfaces | Marker families, availability, degraded-state message, alternate routes |
| **Minimap** | Coarse document preview with inspectable markers | Marker families, availability, degraded-state message, replacement routes |
| **Overview ruler** | Thin semantic marker rail | Marker families, availability, degraded-state message, replacement routes |

## Honesty invariants

1. **Multi-cursor count and undo grouping are visible.** The indicator shows caret count > 1, the mode posture, the undo grouping class, and an accessibility label.
2. **Fold summaries preserve hidden critical state.** Any fold that hides diagnostics, conflicts, trust warnings, or staged hunks must set `critical_state_preserved: true` and expose a non-empty `detail_route_ref`.
3. **Breadcrumb jumps preserve continuity.** `back_forward_preserved` is true, alternate command refs are non-empty, and a visible state note explains partial or degraded paths.
4. **Degraded aids name alternate routes.** Every reduced or disabled minimap, overview ruler, or gutter rail must carry a degraded-state message, at least one replacement route, and an accessibility label.
5. **Marker vocabulary is consistent.** Gutter, minimap, overview ruler, and breadcrrow share one `shared_marker_families` vocabulary; no aid projects a family outside the shared set.
6. **Degraded mode classes are explicit.** Whenever any aid is narrowed, the active `degraded_mode_classes` list contains the corresponding availability class.
7. **Performance budgets are bounded.** Orientation-aid rendering must not materially regress typing (≤ 500 µs), scrolling (≤ 1,000 µs), or file-switch (≤ 2,000 µs) on claimed stable rows.

## Surface downgrade matrix

| Availability class | When it applies | Visible behavior | Alternate routes |
|---|---|---|---|
| `disabled_large_file` | Document is in large-file mode | Minimap, overview ruler, and gutter rail disabled | Problems, Search, Review, and Outline panels |
| `disabled_low_resource` | Host is in low-resource mode | Minimap, overview ruler, and gutter rail disabled | Problems, Search, Review, and Outline panels |
| `disabled_reduced_motion` | Reduced-motion is active | Animations suppressed; static markers remain | Static Problems, Search, Review, and Outline routes |
| `disabled_high_contrast` | High-contrast or forced colors | Color-channels suppressed; shape/position markers remain | Non-color channels and canonical panels |
| `disabled_battery_saver` | Battery-saver is active | Overview aids reduced to conserve power | Problems, Search, Review, and Outline panels |
| `disabled_restricted_mode` | Restricted workspace policy | Trust-policy warnings may narrow marker sets | Policy-allowed Problems, Search, and Review state |

## Schema

The boundary schema is `schemas/editor/orientation_aid_state.schema.json`.

The canonical record is [`OrientationAidsStabilityPacket`]:
- `record_kind`: `"orientation_aids_stability_packet"`
- `schema_version`: `1`
- `schema_ref`: `"schemas/editor/orientation_aid_state.schema.json"`

It composes:
- `orientation_aid_state`: an [`OrientationAidStateRecord`] with multi-cursor attribution, fold summaries, breadcrumb continuity, gutter rail, and overview aids.
- `latency_budget_micros`: claimed latency budget for orientation-aid rendering (≤ 1,000 µs on stable rows).
- `typing_budget_micros`: claimed typing latency budget (≤ 500 µs).
- `scroll_budget_micros`: claimed scroll latency budget (≤ 1,000 µs).
- `file_switch_budget_micros`: claimed file-switch latency budget (≤ 2,000 µs).

## Fixtures

Deterministic claimed-stable fixtures live under:
`fixtures/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap/`

The corpus is generated by:
```sh
cargo run --bin aureline_orientation_aids_stability
```

Scenarios cover:
- `source_editor_full_fidelity.json` — baseline full-fidelity source editor surface with multiple carets.
- `diff_surface_column_selection.json` — diff surface with column-selection posture.
- `review_surface_full_fidelity.json` — review surface with review-thread markers first-class.
- `large_file_degraded.json` — large-file mode disables overview aids.
- `low_resource_degraded.json` — low-resource mode disables overview aids.
- `reduced_motion_degraded.json` — reduced-motion simplifies overview aids.
- `high_contrast_degraded.json` — high-contrast simplifies overview aids.
- `battery_saver_degraded.json` — battery-saver reduces overview aids.
- `restricted_mode_degraded.json` — restricted-mode narrows available marker families.
- `fold_with_critical_hidden_state.json` — explicit fold that preserves hidden diagnostics and conflicts.

## Integration touchpoints

- **crates/aureline-editor** — owns the orientation-aid state record and stability packet.
- **crates/aureline-search** — provides search-hit markers that feed the shared marker vocabulary.
- **crates/aureline-graph** — provides navigation-target and freshness-propagation markers that feed breadcrrow continuity.
- **docs/help** — exposes alternate routes and degraded-state explanations.
- **fixtures/editor** — pins deterministic corpus for regression testing.
- **fixtures/accessibility** — verifies screen-reader announcements for every degraded posture and fold summary.

## Performance

Orientation aids must not add noticeable latency tax to typing, scrolling, or file-switch on claimed stable rows. The current budgets are:

- **Typing**: ≤ 500 µs
- **Scrolling**: ≤ 1,000 µs
- **File-switch**: ≤ 2,000 µs
- **Orientation-aid render**: ≤ 1,000 µs
