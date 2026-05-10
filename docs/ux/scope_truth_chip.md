# Scope-truth chip on open and search foundations

This document is the reviewer-facing entry point for the scope-truth
chip rendered on the live shell's open and search surfaces. It freezes
the chip vocabulary, the `partial_scope` flag, the visible / loaded /
all-matching count disclosure, and the named runtime path the chip
flows through.

The companion artifacts are:

- [`/crates/aureline-shell/src/scope_truth/`](../../crates/aureline-shell/src/scope_truth/)
  - shared shell module that projects the canonical workspace
    [`ScopeClass`](../../crates/aureline-workspace/src/worksets/mod.rs)
    and [`WorksetArtifactRecord::project_chip`](../../crates/aureline-workspace/src/worksets/mod.rs)
    truth into the serializable `ScopeTruthChipCard` the chrome
    consumes.
- [`/crates/aureline-shell/src/search_shell/state.rs`](../../crates/aureline-shell/src/search_shell/state.rs)
  - the named protected-row consumer that wires the chip into the live
    workspace search surface card.
- [`/fixtures/workspace/scope_truth_cases/`](../../fixtures/workspace/scope_truth_cases/)
  - seeded cases for full workspace, narrowed workset, sparse-slice
    partial index, policy-limited admin view, and outside-current-scope
    rows.
- [`/crates/aureline-shell/tests/scope_truth_chip_card_cases.rs`](../../crates/aureline-shell/tests/scope_truth_chip_card_cases.rs)
  - fixture-driven test that asserts the projection contract for every
    case, plus a serde round-trip on every emitted card.

## Why this chip exists

Open and search surfaces in M1 frequently render results that look
workspace-wide even when the active scope is narrowed by:

- a `current_repo` selection in a multi-root workspace,
- a saved or ad-hoc workset (`Hot path`, `Backend only`, ...),
- a sparse slice driven by include/exclude globs,
- an admin policy that hides members from the view,
- a watcher / index that has not finished warming.

The scope-truth chip MUST surface the gap between *what is in scope*
and *what would match against the workspace*. Surfaces never invent a
private chip label, presentation state, or hidden-result class; they
project the canonical workspace truth through the shell-side scope
truth module.

## Chip vocabulary

The chip card carries five frozen token classes the chrome quotes
verbatim:

| Token field                   | Vocabulary                                                                                                                                                  |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scope_class_token`           | `current_repo` / `selected_workset` / `sparse_slice` / `full_workspace` / `policy_limited_view`                                                             |
| `presentation_state_token`    | `active_narrow_safe` / `active_partial` / `active_policy_limited` / `active_widened` / `outside_current_scope`                                              |
| `surface_class_token`         | `explorer` / `quick_open` / `search_shell` / `docs_browser` / `open_flow_sheet` / `support_packet`                                                          |
| `hidden_result_count_class`   | `partial_index` / `outside_scope_roots` / `policy_hidden` / `warming_index` / `remote_unreachable` (omitted when nothing is actually hidden)                |
| `counts_class_token`          | `globally_authoritative` / `partial_truth` / `not_computed`                                                                                                 |

The label string is `"<chip family> · <workset name>"` for narrowed
scopes (e.g. `Selected workset · Hot path`) and `"<chip family>"` for
the bare `current_repo` and `full_workspace` cases. Outside-scope rows
always render the literal label `"Outside current scope"`.

## `partial_scope` flag

`partial_scope` is `true` whenever any of the following holds:

- the active scope is narrower than the workspace
  (`current_repo`, `selected_workset`, `sparse_slice`, `policy_limited_view`),
- at least one member ref carries a `partial_truth` below `loaded`,
- the active scope's readiness is below `ready` (warming, partial,
  unavailable),
- the chip is an outside-current-scope marker.

Surfaces MUST render a `Partial` cue (badge, dotted border, or banner
hint) alongside the chip when this flag is true. They MUST NOT collapse
the flag into a generic "loading" badge, and they MUST NOT promote a
`current_repo` chip to globally authoritative just because its readiness
is `ready`.

## Visible / loaded / all-matching count disclosure

Every chip carries a `counts` block with three explicitly distinct
fields:

| Field                          | Meaning                                                                                                                                                         |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `visible_in_view`              | Rows currently rendered after viewport truncation. Always known.                                                                                                |
| `loaded_in_scope`              | Total rows produced by the active query against the loaded scope index, before viewport truncation. `None` when no query has run.                               |
| `all_matching_in_workspace`    | Rows the same query would match against `full_workspace`. `None` until widened-search lands. Surfaces MUST NOT default this to `loaded_in_scope`.               |

`counts_class_token` is derived from the triplet:

- `globally_authoritative` only when scope covers the workspace, the
  scope is ready, and the three counts collapse to the same value.
- `partial_truth` whenever any pair differs, the scope is narrowed, or
  the readiness is below ready.
- `not_computed` when the surface has not yet produced any counts (e.g.
  quick open before the user types).

In M1 the search shell never sets `all_matching_in_workspace`, so every
search-shell chip honestly reports `partial_truth` until widened-search
ships. This is intentional — it is the canonical disclosure that we do
not yet know whether widening would surface more rows.

## Named runtime path

```
aureline_workspace::ScopeClass         (canonical scope vocabulary)
aureline_workspace::WorksetArtifactRecord::project_chip
        |
        v
aureline_shell::scope_truth::project_scope_truth_chip_card_for_artifact
        |  (or project_scope_truth_chip_card / project_outside_scope_truth_chip_card)
        v
aureline_shell::scope_truth::ScopeTruthChipCard
        |
        v
aureline_shell::search_shell::WorkspaceSearchSurfaceCard.scope_truth_chip
```

The same card is consumed by the chrome and by support exports. Surfaces
never inspect the underlying workset artifact directly to render
additional labels; they render the card.

## Protected walk

1. Open a sparse-slice workspace using the `sparse_slice_partial_index`
   fixture row.
2. Type a query into the workspace search surface.
3. Observe the rendered card:
   - `scope_truth_chip.chip_label = "Sparse slice · Frontend-only slice"`.
   - `scope_truth_chip.presentation_state_token = "active_partial"`.
   - `scope_truth_chip.partial_scope = true`.
   - `scope_truth_chip.hidden_result_count_class = "partial_index"`.
   - `scope_truth_chip.counts.counts_class_token = "partial_truth"`.
4. Switch to a saved workset (`selected_workset_narrowed_view` row) and
   confirm the chip relabels to `Selected workset · Hot path` and
   offers `widen_with_review`, `widen_to_full_workspace`, and
   `open_scope_diff` actions.

## Failure drill

Drive a search inside the `outside_current_scope_search_row` fixture and
confirm the surfaced row chips itself as `Outside current scope` with
`partial_scope = true`, `outside_current_scope_marker_visible = true`,
and offered actions `widen_with_review` + `open_in_new_pane`. The chip
MUST NOT default to a generic "outside" cue or borrow a workset's
in-scope action set.

## Out of scope

- Widened-search count derivation (waiting on M2 search foundations).
- Remote / container scope chips that do not yet have a workset
  artifact (waiting on M2+ remote workspaces).
- Cross-window chip propagation; today every chip is bound to the
  emitting workspace.
