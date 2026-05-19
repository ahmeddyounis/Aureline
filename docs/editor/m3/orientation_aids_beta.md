# Beta orientation aids across editor, diff, and review

This contract promotes the editor orientation-aid surface to beta. Multi-cursor
and column-selection state, fold summaries, breadcrumb path identity, the shared
gutter rail, and minimap or overview-ruler chrome project from one truth model
so editor, diff, and review surfaces cannot disagree about what is hidden, where
a jump leads, or how an aid degrades under large-file, low-resource,
reduced-motion, high-contrast, battery-saver, restricted, or
disabled-by-setting postures.

Records are emitted as
[`orientation_aid_state_record`](../../../schemas/editor/orientation_aid_state.schema.json)
payloads. Each fold summary inside the record also conforms to
[`fold_summary_state_record`](../../../schemas/editor/fold_summary_state.schema.json)
on its own, so support exports and reviewer tooling can quote one fold without
the full surface frame.

## Shared marker vocabulary

A closed marker-family vocabulary backs every orientation aid:

- `diagnostic_error`, `diagnostic_warning`, `diagnostic_info`, `diagnostic_hint`
- `merge_conflict`, `staged_hunk`, `vcs_change`
- `search_hit`, `review_thread`
- `trust_or_policy_warning`, `breakpoint`
- `fold_hidden_state`, `generated_or_read_only`, `freshness_or_stale`

The gutter rail, minimap, overview ruler, and breadcrumb-rooted orientation
aids must project a subset of `shared_marker_families`; the record is rejected
when any surface invents a family the record has not declared. The same
vocabulary is the canonical set assistive technology, themes, and support
exports use to talk about state meaning.

## Multi-cursor and column-selection attribution

`multi_cursor` carries the next-edit posture, caret count, primary caret label,
column-mode flag, undo-grouping class, and an accessibility label. When
`mode_posture` is `multiple_carets` or `column_selection`, the indicator must
be visible (`caret_count >= 2`, `group_undo_visible = true`); when the posture
is `column_selection`, `column_mode_active` must be `true`. Commands that
cannot apply to every caret atomically (refactor previews, structural actions
without a reviewed transaction) declare `unsupported_atomicity` and surface an
`unsupported_note` rather than silently skipping carets.

## Fold summaries preserve hidden critical state

Each fold summary records its hidden line count and per-family hidden marker
counts. The schema treats `diagnostic_error`, `diagnostic_warning`,
`merge_conflict`, `trust_or_policy_warning`, and `staged_hunk` as critical
families: when any of these have a non-zero hidden count the summary must set
`critical_state_preserved = true` and expose a `detail_route_ref`. A folded
region can never make a file look cleaner than it is.

## Breadcrumbs preserve continuity

`breadcrumb.back_forward_preserved` must be `true`, and the row must publish at
least one alternate keyboard command (typically `cmd:quick_open.toggle` or
`cmd:command_palette.open`) plus a `focus_return_route_ref` that points the
editor back to its source on dismissal. Symbol-path freshness is named in
`symbol_path_state` (for example `partial_index_current_file_exact`) so the
visible row never silently downgrades from semantic to syntactic identity.

## Degraded-state honesty

When any orientation aid is reduced or disabled, `degraded_mode_classes` must
list the active class and the gutter rail and overview aids must each carry a
non-empty `degraded_state_message`, at least one alternate or replacement
route, and an accessibility label. The supported degraded postures are:

- `disabled_large_file` (large-file mode)
- `disabled_low_resource` (low-resource mode)
- `disabled_reduced_motion` (reduced-motion accessibility)
- `disabled_high_contrast` (high-contrast / forced colors)
- `disabled_battery_saver` (battery-saver)
- `disabled_restricted_mode` (restricted workspace trust)
- `disabled_by_setting` (user or workspace setting)
- `reduced` (visible but simplified)

`Problems`, `Search`, `Review`, `Source Control`, and `Outline` are the
canonical replacement routes; they must remain reachable through keyboard
commands even when the visual aids are disabled.

## Fixtures

The fixture set under `fixtures/editor/m3/orientation_aids/` covers:

- `source_editor_beta.json` — beta source editor with multi-cursor, two folds,
  breadcrumb, gutter rail, minimap, and overview ruler.
- `diff_surface_beta.json` — beta diff surface with column-selection posture,
  staged-hunk fold, and review-thread overview parity.
- `review_surface_beta.json` — beta review thread surface with resolved-thread
  fold and review-thread first-class marker family.
- `large_file_degraded_beta.json` — large-file degraded posture; all overview
  aids and the gutter rail name `disabled_large_file` and route to Problems
  and Search.

Each fixture is consumed by the integration test
`crates/aureline-editor/tests/orientation_aids_beta.rs`, which round-trips the
record, asserts the shared vocabulary, fold-summary critical-state
preservation, breadcrumb continuity, multi-cursor attribution, and degraded
labeling, and verifies that the constructed beta record matches the source
editor fixture for the default posture.
