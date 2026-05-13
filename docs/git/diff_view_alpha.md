# Git Diff View Alpha

The diff viewer alpha is the first local review surface over the canonical Git
status and shell change-list projections.

## Contract

- `aureline-review` owns the `diff_open_target` and
  `diff_view_surface_packet` records.
- Change-list rows expose a public `DiffOpenTarget` with the command id,
  launcher row ref, compare target, path-truth ref, status code, and file-state
  token needed to open a diff.
- The diff viewer consumes that open target plus explicit hunk input. It does
  not infer compare targets from private shell state.
- Protected rows repeat visible path and compare-target labels so review,
  copy, support export, and reopen paths all quote the same target truth.
- Syntax labels are attached from the language id or path extension. Structured
  config/document classes use a structure-aware label when available.
- Suspicious text uses the shared detector from `aureline-content-safety`.
  Warnings do not normalize source bytes, and suspicious rows keep raw and
  escaped copy paths visible.
- Copy actions distinguish raw source, plain text, rendered hunk context, and
  escaped source where suspicious text is present.
- Closed diff sessions store compare target, path-truth ref, scroll anchor, and
  selected hunk/row. Reopen restores the diff surface instead of falling back to
  a generic file open when those refs are available.

## Records

- `diff_open_target`: public entry point from a source-control row into a local
  diff.
- `diff_view_surface_packet`: path truth, compare target, syntax projection,
  hunks, row warnings, and row copy actions.
- `diff_closed_session_record`: close-time state needed for reopen continuity.
- `diff_reopen_projection`: restored diff surface, target refs, scroll anchor,
  and selected hunk/row.

## Protected Fixtures

Fixtures live under `fixtures/git/diff_view_alpha/`.

- `rust_suspicious_safe_copy.yaml` validates Rust syntax labeling, visible
  path/target truth, suspicious bidi cues, and raw/plain/context/escaped copy
  choices.
- `reopen_closed_diff.yaml` validates staged compare-target continuity when a
  closed diff is reopened.

Run the protected proof path with:

```sh
cargo test -p aureline-review --test diff_view_alpha
```
