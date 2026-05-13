# Git Change List Alpha

The shell change-list surface is a projection over the canonical Git status
snapshot from `aureline-git`. It does not run Git itself or invent a second
status model.

## Contract

- The source-control view always exposes two top-level groups:
  `staged` and `unstaged`.
- Paths with both index and worktree changes appear in both groups with the
  same file-state token and separate group context.
- Untracked files and unresolved conflicts live in the `unstaged` group.
- Rows carry one file-state chip set reused by shell change lists, editor tabs,
  and review entry points.
- Large groups are materialized through a bounded viewport with total counts,
  hidden-before counts, and hidden-after counts so the UI can scale without
  claiming that only visible rows exist.

## File-State Vocabulary

The shared chip vocabulary is:

| Token | Label | Tone |
|---|---|---|
| `modified` | Modified | accent |
| `added` | Added | positive |
| `deleted` | Deleted | danger |
| `renamed` | Renamed | attention |
| `copied` | Copied | attention |
| `type_changed` | Type changed | attention |
| `untracked` | Untracked | neutral |
| `conflicted` | Conflict | critical |
| `ignored` | Ignored | muted |

Each visible row contains a `GitFileStateChipSet` with `shell`, `editor_tab`,
and `review_entry` chips. Those chips must share the same `state_token`.

## Protected Fixtures

Fixtures live under `fixtures/git/change_list_alpha/`.

- `mixed_groups.yaml` validates staged and unstaged grouping, rename metadata,
  untracked rows, and a dual-state path.
- `large_virtualized.yaml` validates large change-list windows and hidden-row
  disclosure.

Run the protected proof path with:

```sh
cargo test -p aureline-shell --test git_change_list_alpha
```
