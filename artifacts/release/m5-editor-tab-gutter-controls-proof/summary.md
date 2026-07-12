# M5 Editor-Tab and Gutter Controls

- Packet: `m5-editor-tab-gutter-controls:stable:0001`
- Label: `M5 editor-tab and gutter controls with modified/preview/pinned/read-only/blocked/shared/generated/remote file-session state, breakpoint/change-marker/diagnostic/blame layering, and reopen/reveal continuity aligned across editor, diff, notebook, diagnostics, support, and product surfaces`
- Consumer surfaces: 6
- Tab item states: modified, preview, pinned, read_only, blocked, shared, generated, remote, state_unknown
- Gutter marker layers: diagnostic, breakpoint, change_marker, blame_or_trust_cue, fold_affordance, layer_unresolved
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor tab strip names the active document context and per-tab pinned/modified/read-only/shared/generated/remote state with no-color-only semantics, and the gutter layers breakpoints and change markers with stable precedence; both degrade honestly when identity is unstated or a marker is encoded by color alone
  - Editor-tab examples: 4 / gutter examples: 3
- **diff_ui**: `stable`
  - Owner: Diff / merge surface owner
  - Scope: The diff surface reuses the same tab and gutter grammar for read-only and split panes and added/modified change markers, and degrades honestly when reopen/reveal continuity is lost across panes or a marker kind cannot be resolved
  - Editor-tab examples: 3 / gutter examples: 3
- **notebook_ui**: `stable`
  - Owner: Notebook code-pane owner
  - Scope: The notebook code cell reuses the same preview tab grammar and fold-region gutter affordance a user sees in the editor, and degrades honestly when the pane context is unresolved or the marker layer cannot be resolved
  - Editor-tab examples: 2 / gutter examples: 2
- **diagnostics_ui**: `stable`
  - Owner: Diagnostics gutter owner
  - Scope: The diagnostics surface names problem severity on the diagnostic gutter layer with no-color-only semantics and keeps layer precedence readable in compact, high-zoom, and exported views, degrading honestly when precedence is lost, layering is unreadable, or severity is unresolved
  - Editor-tab examples: 2 / gutter examples: 5
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved tab and gutter truth, so an invented feature-local badge, a blocked tab hidden behind a color cue, or an unresolved anchor is visible in evidence rather than hidden behind compact chrome
  - Editor-tab examples: 3 / gutter examples: 3
- **product_ui**: `stable`
  - Owner: In-product editor owner
  - Scope: In-product surfaces reuse the same file/session and gutter state grammar a user sees in the editor, always offering the command-backed detail/reveal path and degrading honestly when the trace path is missing
  - Editor-tab examples: 2 / gutter examples: 2
