# Proof packet: scope-truth chip on open and search foundations

Purpose: anchor proof that Aureline surfaces a canonical scope-truth chip on
its open and search foundations, that the chip honestly labels partial scope
truth, and that visible / loaded / all-matching counts are disclosed
separately rather than collapsed into a single number.

Canonical sources (non-exhaustive):

- `docs/ux/scope_truth_chip.md`
- `crates/aureline-shell/src/scope_truth/`
- `crates/aureline-shell/src/search_shell/state.rs`
- `crates/aureline-workspace/src/worksets/mod.rs`
- `fixtures/workspace/scope_truth_cases/`
- `crates/aureline-shell/tests/scope_truth_chip_card_cases.rs`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`
- Smoke outputs (optional): `artifacts/milestones/m1/smoke_outputs/`
- Screenshots (optional): `artifacts/milestones/m1/screenshots/`

Suggested manual drill (protected walk):

- Open the protected dogfood workspace and confirm the search-shell
  card carries `scope_truth_chip.chip_label = "Current repo"` with
  `partial_scope = true` because the scope is narrower than the
  workspace.
- Switch to a saved workset (`Hot path`) and confirm the chip relabels
  to `"Selected workset · Hot path"` with the `widen_with_review`,
  `widen_to_full_workspace`, and `open_scope_diff` actions offered.
- Activate a sparse-slice workset and confirm the chip's
  `hidden_result_count_class` is `partial_index` and that the
  `partial_index_note` quotes the workset's reason verbatim.

Failure drill:

- Run a query inside a partial-scope workset that contains rows whose
  owning root is outside the active scope. Confirm those rows render
  the `outside_current_scope` chip with `outside_current_scope_marker_visible = true`,
  partial scope set, and only `widen_with_review` + `open_in_new_pane`
  actions offered. The chip MUST NOT default to a generic "outside" cue
  or borrow the active scope's action set.

Automated proof:

- `cargo test -p aureline-shell --test scope_truth_chip_card_cases`
  drives every fixture row in `fixtures/workspace/scope_truth_cases/`
  and asserts the projection contract (chip label, presentation state,
  partial-scope flag, action set, hidden-result class, count
  disclosure, and serde round-trip) for each case.
- `cargo test -p aureline-shell --lib scope_truth` exercises the
  shared shell module's count-class derivation and bare-scope chip
  projection paths.
