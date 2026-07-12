# M5 Tab-Strip and Breadcrumbs Controls

- Packet: `m5-tab-strip-breadcrumbs-controls:stable:0001`
- Label: `M5 tab-strip and breadcrumbs controls with active-context, per-tab item state, source-aware hierarchy ancestry, and no top-level navigation drift aligned across shell, explorer, search, review, help, and support surfaces`
- Consumer surfaces: 5
- Tab item states: pinned, preview, modified, read_only, blocked, shared, reopened, state_unknown
- Breadcrumb ancestry kinds: file_path, symbol_ancestry, logical_root, search_scope, mixed_ancestry, ancestry_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell workspace owner
  - Scope: The shell tab strip names the active context and per-tab pinned/preview/modified/read-only/blocked/shared/reopened state with no-color-only semantics, and degrades honestly when the active context is unstated or a blocked tab hides behind an ambiguous ellipsis
  - Tab-strip examples: 4 / breadcrumb examples: 2
- **explorer_ui**: `stable`
  - Owner: Explorer tree owner
  - Scope: The explorer breadcrumb trail names its file-path or logical-root ancestry and stays explicit across compact and expanded views, showing a partial hierarchy honestly and degrading when missing scope collapses into an ambiguous ellipsis
  - Tab-strip examples: 2 / breadcrumb examples: 4
- **search_ui**: `stable`
  - Owner: Search results owner
  - Scope: The search surface reuses the same tab and breadcrumb grammar for preview contexts and search-scope ancestry, and degrades honestly when a path is not explicit across views or an item state is encoded by color alone
  - Tab-strip examples: 3 / breadcrumb examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved tab and breadcrumb truth, so a masquerading tab, an invented surface-local badge, a partial-hierarchy-shown-complete trail, or an unresolved ancestry is visible in evidence rather than hidden behind compact chrome
  - Tab-strip examples: 3 / breadcrumb examples: 3
- **product_ui**: `stable`
  - Owner: In-product navigation owner
  - Scope: In-product surfaces reuse the same active-context and ancestry grammar a user sees in the shell, always offering the command-backed detail path and degrading honestly when the trace path is missing
  - Tab-strip examples: 2 / breadcrumb examples: 2
