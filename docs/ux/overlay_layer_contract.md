# Overlay Layer, Portal, Scrim, Focus-Trap, and Escalation Contract

This document is the shell-wide overlay contract for Aureline. It exists
so dialogs, sheets, menus, hovercards, peek panels, contextual review
surfaces, presentation overlays, and promoted panels share one layer
order, one portal-boundary model, one scrim vocabulary, one focus-trap
and focus-return rule set, one dismissal and back-stack model, and one
promotion ladder.

The contract is normative. Where this document disagrees with the UI /
UX Spec, UX Design System Style Guide, or the narrower contracts it
composes with, the source spec wins and this document must update in the
same change. Where this document disagrees with a downstream surface's
private z-order, portal, focus, or promotion rule, this document wins.

Companion artifacts:

- [`/schemas/ux/overlay_stack.schema.json`](../../schemas/ux/overlay_stack.schema.json)
  - boundary schema for one `overlay_stack_record`.
- [`/fixtures/ux/overlay_cases/`](../../fixtures/ux/overlay_cases/)
  - worked cases for denied product-owned nesting, stale preview
  promotion, boundary-change banners, focus return after dismissal,
  missing-provider fallback, and keyboard-only high-zoom promotion.

This contract composes with, and does not replace:

- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  and [`/artifacts/design/layer_and_scrim_tokens.yaml`](../../artifacts/design/layer_and_scrim_tokens.yaml)
  for the frozen layer order and scrim tokens.
- [`/docs/ux/transient_surface_contract.md`](./transient_surface_contract.md)
  for tooltip, hovercard, popover, peek, pinning, freshness, stale
  cues, and promotion identity preservation.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md)
  for modal, sheet, dedicated review surface, platform-auth exception,
  and product-owned nested overlay denial rules.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for consequence class, safe default focus, preview/apply drift, and
  durable handoff.
- [`/docs/ux/shell_close_reopen_contract.md`](./shell_close_reopen_contract.md)
  for transient overlay memory, placeholders, and focus-return fallback.

## Scope

This contract applies to every product-owned or extension-contributed
surface that renders above, beside, or temporarily in front of another
surface:

- tooltip, hovercard, popover, context menu, command palette, quick
  picker, completion list, and peek panel;
- modal dialog, side sheet, window-attached sheet, full sheet, dedicated
  review surface, and promoted panel;
- transient banner, toast, presentation spotlight, follow prompt,
  trust/auth/policy prompt, and contextual review overlay;
- host-owned platform auth or file-picker dialog when the product
  launches it and resumes from it.

Inline validation text, durable pane content, normal editor decorations,
and permanent layout slots are out of scope unless they open or promote
into an overlay.

## Overlay Classes And Layers

Overlay classes use the design-token layer order. Surfaces consume token
names, not raw z-index values.

| Overlay class | Layer token | Portal boundary | Scrim default | Focus default |
|---|---|---|---|---|
| Tooltip | `z_floating` | Anchor local | `scrim_none` | No trap |
| Hovercard | `z_floating` | Anchor or pane local | `scrim_none` | No trap; keyboard route required |
| Popover | `z_floating` | Anchor local | `scrim_none` or `scrim_weak` | Contained focus when actionable |
| Context menu / quick picker / command palette | `z_menu` | Window local | `scrim_none` or `scrim_weak` | Contained focus or roving tabindex |
| Peek panel | `z_floating` or `z_dialog` when blocking | Editor-group or pane local | `scrim_none` or `scrim_weak` | Contained focus when opened |
| Banner | `z_sticky` | Pane, window, or workspace local | `scrim_none` | No trap |
| Toast | `z_toast` | Window local | `scrim_none` | No trap; never covers dialog actions |
| Dialog / capability sheet / import-export sheet | `z_dialog` | Owning window | `scrim_strong` for blocking; `scrim_weak` for sheet isolation | Modal trap |
| Dedicated review surface / promoted panel | Durable shell surface | Not transient after promotion | Surface-owned | Owns normal focus order |
| Critical trust/auth/security prompt | `z_critical` | Owning window unless platform-owned | `scrim_strong` | Modal trap or platform trap |
| Platform auth or file picker | Platform host-owned | Platform host-owned | Host-owned | Host-owned trap |

Rules:

- `z_critical` is reserved for trust, auth, and security-critical
  first-party overlays. Extensions may request a host review surface;
  they may not stack content at the critical layer.
- Dialogs and critical prompts outrank menus, hovers, and peeks. Menus
  and hovercards close or suspend when a blocking dialog opens.
- Toasts render above working chrome but must not obscure the primary
  action area, cancel action, or consequence block of an active dialog
  or sheet.
- A surface whose layer order is hard-coded or locally invented is
  non-conforming. It must emit `hard_coded_layer_order`.

## Portal Boundaries

Every overlay declares one portal boundary:

- `anchor_local` - attached to a concrete control, row, token, or range.
- `pane_local` - scoped to one pane or editor group.
- `editor_group_local` - scoped to the active editor group, including
  peeks and local review panels.
- `window_local` - scoped to one OS window. Focus history remains local.
- `workspace_window_attached` - scoped to the workspace in the owning
  window; dialogs and sheets name the workspace or workset they affect.
- `platform_host_owned` - launched by Aureline but owned by OS, browser,
  or identity provider.
- `dedicated_surface_owned` - the overlay has been promoted into a
  durable panel, sheet, tab, or review surface.

Rules:

- Overlays attach to the narrowest truthful boundary. Blocking a whole
  window is allowed only when the whole window cannot continue safely.
- Cross-window overlays are forbidden. A background event in another
  window may request attention in that window, but it must not warp
  focus or stack a prompt over the current window.
- If a provider, extension host, remote agent, or embedded runtime that
  owns the portal becomes unavailable, the overlay renders an attributed
  placeholder, promotes to a product-owned sheet, or disables risky
  actions. It may not disappear without a focus-return or history row.
- If display topology, monitor DPI, zoom, or window bounds changes, the
  portal must remain visible or expose a recenter/restore-layout path.
  An off-screen stranded prompt is non-conforming.

## Scrim Semantics

Scrims communicate depth and focus. They are never the only indicator of
disabled, blocked, stale, restricted, or destructive state.

| Scrim class | Use | Required behavior |
|---|---|---|
| `scrim_none` | Tooltip, hovercard, menu, toast, non-blocking banner | Underlying focus remains visible and semantics remain reachable. |
| `scrim_weak` | Lightweight sheet, popover with contained focus, local peek isolation | Inerts only the blocked region when possible; preserves orientation. |
| `scrim_strong` | Blocking dialog, permission sheet, critical review | Names the blocked scope and keeps focus ring, safe action, and consequence block visible. |
| `overlay_dim_presentation` | Presentation spotlight, follow prompt, teaching overlay | Preserves semantic order, active-region cues, and return-to-work action. |

Rules:

- A scrim must pair with an explicit blocked-region or active-region
  model. Decorative dimming without focus semantics is non-conforming.
- High-contrast and forced-colors modes preserve focus and state meaning
  with borders, icons, and labels, not opacity alone.
- At 400% zoom, the safe action, dismiss or return action, and required
  state label remain reachable without horizontal scrolling.

## Focus, Traps, And Return

Opening any overlay that steals focus records a focus-return target
first. Closing an overlay returns focus to the invoking control, its row,
the current batch/detail owner, or the nearest safe ancestor when the
exact target no longer exists.

Focus-trap classes:

- `none` - passive overlays such as tooltip, toast, and banner.
- `roving_tab_stop` - menus, quick pickers, dense command lists.
- `contained_focus` - popovers, peeks, and local panels with actionable
  controls.
- `modal_focus_trap` - dialogs, blocking sheets, and critical prompts.
- `platform_host_trap` - platform auth or file picker.

Rules:

- Focus order, visible focus, and screen-reader structure are part of
  the overlay contract. A visually correct overlay with an ambiguous
  keyboard path is non-conforming.
- `Escape` resolves the innermost transient state first: tooltip or
  hovercard, completion/menu, popover/peek, sheet/dialog, spotlight or
  follow prompt, then larger context transitions.
- Keyboard and assistive-technology users can open, traverse, pin,
  promote, and dismiss the same knowledge path as pointer users.
- A focus trap may not swallow platform escape, IME composition, or
  screen-reader navigation without a clear exit or cancel path.

## Dismissal And Back Stack

Dismissal behavior is deterministic and local to the owning boundary.

| Overlay | Dismissal | Back-stack policy |
|---|---|---|
| Tooltip / hovercard | Pointer leave, focus move, explicit dismiss, or `Escape` | Not in navigation history |
| Popover / menu / quick picker | `Escape`, outside click where safe, or explicit close | Consumes overlay back-stack entry only |
| Peek panel | `Escape` closes peek and returns to parent context | Optional overlay stack entry; does not become navigation unless promoted |
| Sheet / dialog | Explicit cancel, safe close, or reviewed discard | Owns one overlay stack entry; may preserve draft/review state |
| Platform host overlay | Host-owned cancel or return packet | Product resumes invoking surface or durable return target |
| Promoted panel / full sheet / tab | Normal shell back/close behavior | Durable navigation/history entry |

Rules:

- Dismissing presentation chrome never commits the underlying action
  unless the action label says so.
- Unsaved input, partially reviewed consequences, or changed targets
  require a reviewed discard, refresh, or revalidate path.
- Long-running work launched from an overlay must mint a durable job,
  history, mutation-journal, or inspected-object state before the
  initiating overlay disappears.

## Allowed Nesting

Product-owned nested dialogs and nested sheets are forbidden. The parent
surface updates in place, replaces itself with a larger surface, promotes
to a dedicated review surface, or delegates to a platform host overlay.

Allowed:

- tooltip or hovercard over ordinary content when it is passive;
- menu over ordinary content when it is command-derived and
  keyboard-reachable;
- platform-auth or file-picker dialog above a product sheet when the
  product explains the handoff and resumes with origin preserved;
- local peek inside an editor group when it does not trap the entire
  window;
- banner above a sheet when it explains a boundary or freshness change.

Denied:

- product confirmation dialog over a product permission sheet;
- second destructive modal over a first destructive modal;
- hidden popover that contains the only path to approve, export, delete,
  publish, or recover;
- extension-owned webview that impersonates a first-party trust,
  permission, or destructive-action prompt.

## Promotion And Escalation

The standard knowledge path is:

`tooltip` -> `hovercard` -> `peek_panel` -> `pinned_panel`,
`side_sheet`, `full_sheet`, `dedicated_panel`, or canonical open.

Rules:

- Promotion preserves canonical object identity, provider attribution,
  source subsystem, freshness, stale cue, mapping quality, and AI
  confidence disclosure where present.
- Pinning a transient surface converts it into a durable panel, tab, or
  sheet. A pinned item is not an orphaned floating layer.
- A stale, cached, generated, remote, or approximate preview remains
  labeled after promotion. Promotion may add detail; it may not erase
  the stale or approximate cue.
- If the underlying target vanishes, moves, or changes scope while the
  overlay is open, risky actions freeze and the surface shows a banner
  or inline review invalidation before commit.

## Critical Action Rule

A transient surface may never be the only route to complete a critical
action. Critical actions include destructive, export/share, provider
mutation, trust/auth/policy, permission, secret access, remote attach,
cross-boundary publish, recovery, and rollback paths.

Every critical action must have at least one durable or structural route:

- command palette, menu, or keyboard command bound to the same command
  id;
- dialog, sheet, dedicated review surface, or durable panel;
- canonical object detail route; or
- activity, history, job, or support/evidence route after handoff.

Tooltip-only, hover-only, pointer-only, color-only, and motion-only
critical routes are non-conforming.

## Multi-Window, Providers, Boundaries, And Zoom

- Focus history is window-local. A prompt in one window cannot steal
  focus from another window.
- When a provider or extension host is missing, the shell must preserve
  attribution and show what still works. Risky actions disable; inspect
  and copy-safe actions may remain.
- When workspace, tenant, region, trust, remote-target, or policy
  boundary changes while an overlay is open, the overlay shows a
  boundary-change banner, records the changed basis, and requires
  revalidation before commit.
- High zoom, high contrast, reduced motion, forced colors, RTL, IME, and
  screen-reader flows are first-class overlay states. The same contract
  fields must remain reachable and announced.

## Record Shape

Every conforming overlay stack emits one `overlay_stack_record` when an
overlay opens, promotes, dismisses, denies a nested child, responds to a
boundary change, or hands off to a durable destination. The schema
freezes these groups:

- `overlays` - overlay class, layer token, portal boundary, scrim,
  focus-trap, dismissal, back-stack, nesting, provider, freshness, and
  route metadata;
- `portal_boundary` - owning window, provider state, missing-provider
  fallback, off-screen recovery, boundary-change banner requirement, and
  cross-window focus policy;
- `promotion_chains` - source overlay, canonical object, promotion
  target, identity preservation, stale cue preservation, pinning policy,
  and keyboard parity;
- `focus_and_dismissal` - focus owner, initial focus, escape order,
  return target, announcement, and stack close order;
- `nesting_rules` - admitted or denied child overlay, platform exception,
  denial reason, and required alternative;
- `contract_checks` - machine-readable conformance verdicts.

## Non-Conforming Patterns

- Locally invented z-index or "always on top" values.
- Product-owned dialog or sheet stacked over another product dialog or
  sheet.
- Tooltip, hovercard, context menu, or popover as the only path to a
  critical action.
- Stale preview promotion that drops freshness, source, mapping quality,
  or canonical identity.
- Scrim-only blocked state with no semantic disabled/blocked reason.
- Focus trap that strands focus on document body, off-screen chrome, or
  another window.
- Boundary change that leaves a previously valid confirm action enabled
  without revalidation.
- Provider-owned or extension-owned overlay that impersonates a
  first-party trust, permission, auth, or irreversible-action prompt.
