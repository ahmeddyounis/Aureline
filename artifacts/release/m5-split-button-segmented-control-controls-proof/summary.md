# M5 Split-Button and Segmented-Control Controls

- Packet: `m5-split-button-segmented-control-controls:stable:0001`
- Label: `M5 split-button and segmented-control controls with safe-by-default primary actions, visible adjacent-menu alternates that never widen risk on stale state, and small mode/view toggles with explicit selected-mode truth and keyboard cycling aligned across forms, settings, search, review, support, and product surfaces`
- Consumer surfaces: 6
- Split default postures: primary_default_safe, explicit_alternate, confirm_required, destructive_guarded, all_disabled, posture_unknown
- Segmented modes: mode_toggle, view_switch, single_select_small_set, exclusive_options, not_navigation, mode_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **forms_ui**: `stable`
  - Owner: Forms surface owner
  - Scope: The forms surface offers a split button whose safe default click submits with alternates visible in the adjacent menu, and a segmented control toggling a compact layout mode with explicit selected-mode truth; both degrade honestly when the primary or group label is unstated
  - Split-button examples: 2 / segmented-control examples: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface keeps confirm-required and locked split defaults distinct rather than behind generic disabled chrome, and keeps a single-select density toggle small and keyboard-cyclable; both degrade honestly when a lock hides behind disabled
  - Split-button examples: 3 / segmented-control examples: 3
- **search_ui**: `stable`
  - Owner: Search surface owner
  - Scope: The search surface keeps split-button alternates reachable only by explicit selection and never hides an alternate behind the default click, and keeps a results-view toggle a small view switch rather than stealth navigation; both degrade honestly when an alternate is hidden or a toggle masquerades as navigation
  - Split-button examples: 2 / segmented-control examples: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review sheet keeps a destructive-guarded merge safe by default with any broadened batch scope disclosed, and keeps a diff-mode toggle keyboard-cyclable with mode-scope continuity preserved; both degrade honestly when stale state promotes a riskier default, a broadened scope is undisclosed, keyboard cycling is missing, or mode-scope continuity breaks
  - Split-button examples: 3 / segmented-control examples: 3
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved default-action and selected-mode truth, so an unstated command binding or an unstated selected segment is visible in evidence rather than hidden behind generic chrome
  - Split-button examples: 2 / segmented-control examples: 2
- **product_ui**: `stable`
  - Owner: In-product control owner
  - Scope: In-product surfaces reuse the same safe-default and selected-mode grammar a user sees in forms and settings, always offering the command-backed detail path and degrading honestly when the trace path is missing, an oversized set reads as navigation, or the selected state is color-only
  - Split-button examples: 3 / segmented-control examples: 4
