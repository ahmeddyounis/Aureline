# Search Scope Truth Alpha

The alpha search lane uses one scope-count packet across search results,
graph-backed candidates, and AI context candidates.

## Vocabulary

User-visible scope labels are fixed to:

| Label | Meaning |
|---|---|
| `Current repo` | The current repository/root lane. |
| `Selected workset` | A named workset or sparse slice selected by the user. |
| `Full workspace` | The widened workspace lane. |
| `Remote cache` | A remote or imported cache boundary. |
| `Outside current scope` | A result or action outside the active scope. |
| `Policy-limited view` | A view narrowed or blocked by policy/trust. |

Surfaces must quote these labels from `ScopeTruthLabel` and must not invent
parallel labels for graph or AI context candidates.

## Count Contract

`SearchScopeCountsRecord` separates:

| Field | Meaning |
|---|---|
| `visible_rows` | Rows currently rendered after grouping/truncation. |
| `loaded_rows` | Rows loaded by the active scope before viewport or group truncation. |
| `all_matching_rows` | Rows the same query would match in the full workspace, when known. |
| `hidden_by_current_scope_rows` | Matching rows excluded by the active workset/sparse scope. |
| `hidden_by_policy_rows` | Matching rows hidden or blocked by policy/trust. |
| `hidden_by_remote_cache_rows` | Matching rows behind a remote-cache boundary. |

`visible_rows` must not be copied into `loaded_rows` or `all_matching_rows`.
Unknown values stay absent.

## Empty States

The first alpha search lane distinguishes these states:

| Token | Label | Use |
|---|---|---|
| `no_results` | `No results` | Full searched scope has no matching rows. |
| `no_results_in_this_workset` | `No results in this workset` | Matching rows exist outside the selected workset/sparse scope. |
| `index_not_built_for_excluded_roots` | `Index not built for excluded roots` | Excluded roots may contain matches but were not indexed. |
| `blocked_by_trust_or_policy` | `Blocked by trust or policy` | Policy/trust blocks the lane or view. |

The protected fixtures live in
`fixtures/search/scope_counts_alpha/` and are enforced by
`crates/aureline-search/tests/scope_counts_cases.rs`.
