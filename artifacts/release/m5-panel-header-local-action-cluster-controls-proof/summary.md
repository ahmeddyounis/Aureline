# M5 Panel-Header and Local-Action-Cluster Controls

- Packet: `m5-panel-header-local-action-cluster-controls:stable:0001`
- Label: `M5 panel-header and local-action-cluster controls with stable title slots, active-context truth, cached/partial/stale/remote/provider-owned source-freshness cues at the pane boundary, structured-menu/overflow action placement, keyboard-reachable command-backed refresh and reveal/detail affordances, and compaction that preserves panel identity across shell, request/data, review, search, support, and product surfaces`
- Consumer surfaces: 6
- Source / freshness kinds: current, cached, partial, stale, remote, provider_owned, freshness_unknown
- Action placements: inline_primary, structured_menu, overflow_menu, mixed_primary_overflow, no_actions, placement_unknown
- Compaction modes: full_header, compact_header, collapsed_to_overflow, responsive_reflow, minimized_rail, compaction_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **data_ui**: `stable`
  - Owner: Request / data pane-header owner
  - Scope: The request/data pane header names a stable title and active context, labels a cached provider view at the boundary, points back to the canonical count/selection model, and keeps grid actions in structured menus rather than persistent clutter — degrading when the freshness cue is hidden, the header re-encodes counts, or advanced actions become persistent clutter
  - Panel-header examples: 4 / local-action-cluster examples: 2
- **review_ui**: `stable`
  - Owner: Review-queue pane-header owner
  - Scope: The review queue reuses the shared header grammar for remote-owned queues and keeps its compacted action cluster preserving panel identity and semantics — degrading when readiness is overstated, compaction re-instantiates a different surface, or an overflowed action is silently dropped
  - Panel-header examples: 2 / local-action-cluster examples: 3
- **search_ui**: `stable`
  - Owner: Search results pane-header owner
  - Scope: The search surface reuses the same header semantics for a current results pane and keeps its responsively-reflowed action cluster keyboard-reachable — degrading when the active context is unresolved or the actions are hover-only
  - Panel-header examples: 2 / local-action-cluster examples: 2
- **shell_ui**: `stable`
  - Owner: Governance / shell pane-header owner
  - Scope: Governance surfaces reuse the same header grammar, honestly naming a background provider-owned pane and keeping a minimized-rail action cluster that preserves panel identity — degrading when a background context reads as active, the title slot is unstable, or compaction loses the panel identity
  - Panel-header examples: 3 / local-action-cluster examples: 2
- **support_export**: `stable`
  - Owner: Support/export pane-header owner
  - Scope: The support export carries the same resolved header and cluster truth, so a partial pane labelled at the boundary, an unresolved source/freshness, a missing refresh command, lost keyboard access, or an unresolved action placement is visible in evidence rather than hidden behind compact chrome
  - Panel-header examples: 3 / local-action-cluster examples: 3
- **product_ui**: `stable`
  - Owner: In-product pane-header owner
  - Scope: In-product surfaces reuse the same header and action grammar a user sees in the shell and data panes, always offering the command-backed refresh and reveal/detail affordances — degrading when the title is unstated, the reveal path is missing, the budget is unresolved, or compaction loses the action semantics
  - Panel-header examples: 3 / local-action-cluster examples: 5
