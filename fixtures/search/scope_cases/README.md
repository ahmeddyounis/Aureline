# Search scope cases

Fixtures for the workset/slice-aware search scope resolver
(`aureline_search::scope`). Each fixture pins one canonical scope shape and
the partition it MUST produce when applied to a workspace-relative file
list, so the protected-row scope vocabulary cannot drift without a fixture
update.

The driver lives at `crates/aureline-search/tests/scope_cases.rs`; see
`docs/search/search_scope_contract.md` for the reviewer-facing contract.

## Cases

| Fixture | What it covers |
|---|---|
| `full_workspace_default.json` | Bare full-workspace scope (no workset artifact). Authoritative chip, all rows in scope, no partial cue. |
| `current_repo_default.json` | Bare current-repo scope. Chip family stays narrowed; pattern filter is a no-op. |
| `selected_workset_root_filter.json` | Selected workset with one include pattern. Rows outside the include drop from the index. |
| `sparse_slice_include_exclude.json` | Sparse slice with include + exclude patterns. Excludes win over includes; partial-index note flows through. |
| `policy_limited_view.json` | Policy-limited view (admin-narrowed). Chip surfaces the policy-limited presentation state and the filter still applies. |
| `workset_switch_failure_drill.json` | Failure drill: switch worksets mid-search. The chip MUST update to the new family + name and the previous workset's rows MUST NOT match the new pattern set. |

## Schema

Each fixture carries `record_kind: "search_scope_case"` and
`schema_version: 1`. The `scope_kind` field selects how the resolver is
constructed:

- `full_workspace_default` — bare `WorkspaceSearchScope::for_full_workspace`.
- `current_repo_default` — bare `WorkspaceSearchScope::for_current_repo`.
- `workset_artifact` — projected through `from_workset_artifact` from the
  embedded `workset_artifact` block.
- `workset_switch` — failure-drill case carrying both a
  `from_workset_artifact` and a `to_workset_artifact` block.
