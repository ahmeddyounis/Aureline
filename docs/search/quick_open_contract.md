# Quick-open query session: runtime contract

The quick-open query session is the first universal lookup surface in M1.
It merges three canonical sources behind one mental model so the dogfood
flow can move around the workspace before deeper indexing exists:

- recent navigation targets (files, places — served from hot local state),
- commands (projected from the canonical command registry), and
- lexical filename / path hits (projected from the canonical lexical
  search shell).

This document is the reviewer-facing entry point for the runtime path that
backs the surface. It composes with — and does **not** replace — these
contracts:

- [`docs/ux/quick_open_contract.md`](../ux/quick_open_contract.md) for the
  renderer-facing row anatomy, banner vocabulary, and focus rules;
- [`search_query_session_contract.md`](./search_query_session_contract.md)
  for the canonical search query-session record;
- [`search_readiness_vocabulary.md`](./search_readiness_vocabulary.md) for
  the readiness, freshness, partial-truth, and source-class tokens that
  flow into and out of the lexical lane;
- [`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  for the canonical command identity, disabled-reason, and invocation
  preview vocabularies that command rows quote.

If those contracts already define a vocabulary axis, this document and the
runtime path quote that axis instead of minting a synonym.

## Canonical sources

The session does not invent its own command registry, recent-work store,
or lexical index. It projects from three upstream truths:

| Source | Owns | Quick open projects |
|---|---|---|
| `aureline_commands::CommandRegistry` | command identity, disabled-reason classes, invocation preview class | `command_id`, `title`, `summary`, `disabled_reason_class`, `invocation_preview_class`, `dominant_side_effect_class` |
| recent-work registry / consumer-supplied list | recent target identity (paths, routes, buffer anchors) | `recent_id`, `display_label`, `secondary_label`, optional `relative_path`, `target_kind_token` |
| `aureline_search::LexicalShell` | lexical filename/path rows, readiness class, partial-truth causes | `lexical_filename` and `lexical_path` row classes, `match_kind_token`, partial-truth causes |

Quick open MUST NOT relabel a lexical hit as semantic just because a
richer surface ships alongside it. Quick open MUST NOT rename a command
because the command title looks similar to a recent target.

## Source-class vocabulary (`QuickOpenSourceClass`)

| Token | Lane | When to surface it |
|---|---|---|
| `recent_target` | Recent | Row originated from the recents lane (hot local state). |
| `command` | Commands | Row originated from the canonical command registry. |
| `lexical_filename` | Filenames | Lexical match on the basename of a workspace file. |
| `lexical_path` | Paths | Lexical match on the full workspace-relative path. |

Each row carries exactly one source class. Future semantic / symbol /
graph lanes MUST mint their own source class — they MUST NOT reuse
`lexical_filename` or `lexical_path`.

## Source-state vocabulary (`QuickOpenSourceState`)

| Token | Meaning |
|---|---|
| `not_requested` | Source has not been activated for the current session/query. |
| `warming` | Source is preparing to answer; it has not produced rows yet. |
| `partial` | Source has rows, but coverage is incomplete (e.g., index still scanning). |
| `ready` | Source is ready (or fully populated) for the current query. |
| `unavailable` | Source cannot answer right now (watcher down, policy blocked, out of scope). |

The chrome MUST surface these tokens directly per source. Collapsing
`warming` and `partial` into a generic "loading" badge is forbidden — the
failure drill ("query quick-open before all sources are ready") relies on
the chrome showing the correct partial cause per lane.

## Row identity rules

Every snapshot row carries the identity field required for its row kind:

- recents carry the upstream recent-target identity in the source list,
- commands quote `command_id`,
- lexical files quote `relative_path`.

The palette runtime resolves selection through its canonical
`PaletteItemKey`; quick-open snapshots expose the persisted command and file
identity fields directly so support exports do not scrape labels. Each rendered
snapshot row also carries:

- `result_id`, minted from the shared search result-ID helpers;
- `ranking_reason_classes`, ordered from the source row or quick-open lane;
- `result_truth_class`, so ready rows and heuristic or partial rows remain
  distinguishable;
- `partiality_class`, so row-level partial, warming, stale, or unavailable
  caveats survive re-render and support export.

### Winning-source attribution

When the same workspace-relative path appears in both recents and lexical
results _and_ the recent target matches the current query, the recent
wins. The row appears once with `source_class = recent_target` and
`row_kind = recent_target`. Lexical rows with that path are skipped in
the same materialization pass.

If the recent does **not** match the current query, it does not claim its
path — the lexical row is free to surface. This keeps "I just visited
this file" from hiding lexical hits the user is actively typing for.

### Command-row invariants

Command rows MUST quote canonical command identity directly:

- `command_id` is non-empty and matches a registry entry's `command_id`;
- `invocation_preview_class` is the registry entry's `preview_class`
  verbatim (`none`, `summary`, `diff`, …);
- `disabled_reason_class` is the registry entry's first
  `disabled_reason_records` token, when the command is disabled, and
  `null` when enabled.

These three fields MUST appear on the same row across palette, quick
open, command diagnostics, and support exports. A command row that
exposes a disabled reason only in debug output, only in the palette, or
only in an audit packet violates the cross-surface invariant and the
quick-open exit gate cannot close.

## Scope chip projection

The scope chip is projected through `aureline_search::ScopeClass`,
which mirrors `aureline_workspace::ScopeClass` token-for-token. The chip
label family follows the workspace vocabulary:

- `current_repo` → `Current repo`,
- `selected_workset` → `Selected workset` (suffix `· <name>` when a
  workset name is supplied),
- `sparse_slice` → `Sparse slice` (same suffix rule),
- `full_workspace` → `Full workspace`,
- `policy_limited_view` → `Policy-limited view` (same suffix rule).

Quick open does not mint a parallel chip vocabulary.

## Held modifiers

The session stores a closed set of modifier tokens (e.g.,
`files_only_filter`, `command_palette_shortcut`). The session does not
interpret modifier semantics — surfaces and downstream search/palette
consumers read the same set so the three surfaces converge on one mental
model. The token vocabulary is intentionally open-ended at M1; future
contracts will freeze specific tokens.

## Snapshot record

`QuickOpenSnapshot` is the byte-replayable export of a session. It
carries:

- workspace identity, scope class token, scope chip label;
- the active query and held-modifier set;
- per-source state summaries (`recent_target`, `command`,
  `lexical_filename`, `lexical_path`);
- partial-truth causes from the lexical lane (verbatim from the
  upstream `LexicalShell`);
- the materialized rows in display order, including `result_id`,
  `ranking_reason_classes`, `result_truth_class`, and `partiality_class`;
- `available_source_classes` (verbatim taxonomy) so future surfaces
  cannot relabel a captured row.

Support bundles cite the snapshot directly; they do not scrape the
rendered chrome.

## Failure drill

> Query quick-open before all sources are ready and confirm recent /
> command / file sections show truthful warming or partial state.

The failure drill is locked in by
`fixtures/search/quick_open_cases/lexical_warming_partial.json` and
`fixtures/search/quick_open_cases/lexical_unavailable.json`. The
integration test in
`crates/aureline-shell/tests/quick_open_query_session_tests.rs` asserts
that:

- when the lexical lane is warming, the per-source state reads
  `warming`, partial-truth causes are surfaced, and recents/commands
  remain usable;
- when the lexical lane is unavailable, the lexical lanes report
  `unavailable` directly and quick open does not invent file rows.

## Cannot close if

The exit gate stays open if any of these are violated:

- a command row exposes its `command_id`, `disabled_reason_class`, or
  `invocation_preview_class` only in debug output;
- the same command row disagrees about identity across palette, quick
  open, and command diagnostics;
- a lexical row claims a higher-confidence source class than
  `lexical_filename` / `lexical_path`;
- a `warming` lexical lane is collapsed into a generic "loading" badge
  with no partial-truth cause;
- the chrome relabels a captured recent target as a lexical hit because
  its path happens to match a workspace file.
