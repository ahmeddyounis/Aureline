# ADR 0016 - Shell windowing, input, and accessibility boundary

- **Decision id:** D-0022 (must exist in `artifacts/governance/decision_index.yaml`)
- **Status:** Accepted
- **Decision date:** 2026-04-22
- **Freeze deadline:** 2026-08-15
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council
- **Related requirement ids:** `none`

## Context

The renderer ADR, layout-restore contract, interaction-safety contract,
desktop-platform matrix, and multi-window verification seed already
freeze adjacent parts of the desktop shell, but they still leave one
dangerous gap: there is no single contract that says where the desktop
shell stops, where the renderer stops, where the platform adapter
starts, how focus and text input move across shell zones, and what
desktop lifecycle events may restore automatically versus what must
revalidate. That gap is large enough for the shell spike, future
multi-window work, native-menu integration, input-method work, and
accessibility bridging to drift into parallel local rules.

The failure mode is not cosmetic. Without one boundary contract, the
title/context bar, rail, sidebar, workspace, inspector, bottom panel,
status bar, dialogs, menus, deep links, notifications, embedded
surfaces, and OS integrations can each invent their own focus return
story, their own shortcut exceptions, their own IME and AltGr handling,
their own fallback for off-screen restore, and their own idea of when a
wake-from-sleep or display reconnect is allowed to silently resume live
authority. That would make multi-window verification, platform
conformance, support export, accessibility review, and command parity
impossible to cite mechanically.

## Decision

Aureline freezes one desktop shell boundary contract for windowing,
input, focus routing, adaptive layout, accessibility publication, and
platform-adapter seams. The desktop shell process owns the canonical
zone model, adaptive classes, per-window event loop integration, focus
chain, command-dispatch entry points, text-input normalization policy,
and accessibility-tree lifecycle for host-rendered surfaces; the
renderer owns per-window scene composition and accessibility-node
publication hooks; platform adapters own native window handles, raw OS
event translation, OS accessibility bridges, IME and dead-key plumbing,
default-browser and protocol-handler registration, notifications,
clipboard/drag/drop, file dialogs, trust-store and keychain state, and
display-topology facts; and later conformance suites prove the claimed
rows rather than inventing fallback behavior locally. Shell-specific
shortcuts, menus, deep links, notification reopen paths, and embedded
surface handoffs are all command-dispatch entry points only: they may
request work, but they may not bypass preview, approval, trust review,
or typed degraded-state disclosure.

### Canonical ownership split

The shell boundary is frozen at these layers:

| Subject | Shared shell owns | Renderer owns | Platform adapter owns | Later conformance suite proves |
|---|---|---|---|---|
| Zone model and adaptive layout | canonical shell-zone ids, open/close/reset behavior, adaptive collapse order, placeholder semantics, remembered open-zone state | zone-to-scene mapping for active render surfaces | any native chrome constraints that affect available client area | compact/standard/expanded shell behavior on claimed desktop rows |
| Windowing and event loop | per-window Aureline lifecycle, modal ownership, focus-chain state, restore posture, dispatch ordering | per-window surface root, frame submission, dirty-region routing | native window creation, activation, fullscreen/snap/Spaces hooks, display topology facts | multi-window, wake/display-reconnect, off-screen recovery, and dialog ownership drills |
| Focus and command routing | logical focus graph, typed focus-return fallback, command entry normalization | focused-surface visual affordances and accessibility-node deltas | native focus notifications and menu invocation callbacks | keyboard-only, screen-reader, and mixed input-path verification |
| Text input normalization | shared text-input policy, composition lifecycle, multi-cursor degradation rules, command-vs-text arbitration | composition overlay rendering, grapheme/bidi-aware paint | IME, dead keys, compose sequences, AltGr, emoji picker, dictation, layout-switch facts | IME/bidi/dead-key/AltGr and locale regression suites |
| Accessibility publication | accessibility-tree lifecycle, host-owned semantic nodes, host-owned boundary chrome for embeds | accessibility node generation from the same surface/view truth | UIA / NSAccessibility / AT-SPI bridge transport | accessibility regression suites on claimed rows |
| Native affordances | host-visible command route, disabled reason, preview/approval gate, restore/reopen target | visual state only when host-rendered | menus, dialogs, browser handoff, notifications, clipboard, drag/drop, deep-link registration, trust-store/keychain, removable-volume and sleep/display events | platform conformance packets and support/export drills |

Rules (frozen):

1. The shell owns policy-bearing behavior. The renderer and platform
   adapters may surface state, but they may not mint new preview,
   approval, trust, or restore rules.
2. One Aureline top-level window maps to one native top-level window,
   one renderer surface root, and one accessibility root. Many windows
   may still share one workspace authority.
3. Focus is a shell truth, not paint order. A surface that paints last
   does not therefore own focus.
4. Platform adapters may translate native events, but they may not
   classify an action as safe to bypass the command graph.

### Canonical shell zones and adaptive classes

The canonical shell-zone ids are the product zones from the UI/UX spec
and command-slot taxonomy, not the narrower renderer-spike trace zones.

| Shell zone | Primary role | Restorable posture | Focus rule | Slot family anchor |
|---|---|---|---|---|
| `title_context_bar` | identity and global state | restore open state and high-level route state | one deterministic tab stop before delegating to any embedded controls | `title_context_bar` |
| `activity_rail` | durable top-level mode switching | restore selected route only | rail focus never depends on hover; overflow remains keyboard reachable | `activity_rail` |
| `left_sidebar` | structural navigation | restore active view and rough width | focus lands on the active structural view, not on a hidden toolbar affordance | `sidebar` |
| `main_workspace` | dominant task surface | restore occupant, tab/group structure, and local focus target when safe | the default return target for most task flows | `editor_chrome` plus workspace-local surfaces |
| `right_inspector` | contextual explanation and detail | restore open/closed state and selected inspector when still valid | inspector focus is optional; if invalid it returns to the main workspace | `inspector` |
| `bottom_panel` | execution, output, and longitudinal state | restore active tab and last meaningful height | panel tabs may collapse, but the active durable state remains reachable | `bottom_panel` |
| `status_bar` | persistent low-urgency state and toggles | restore durable state items only | status focus must narrate current state and route to the owning surface | `status_bar` |
| `transient_overlay` | scoped interruption or quick access | not restored as ambient shell state | overlays record a focus-return target first and remain window-local | `command_palette`, `global_menu`, `context_menu`, sheets, dialogs |

Adaptive classes are frozen exactly as shell-wide layout classes. Density
is separate and must not be used as an implicit layout class.

| Adaptive class | Width band | Required collapse order | Non-negotiables |
|---|---|---|---|
| `compact_desktop` | 1024-1279 px | inspector to sheet first, secondary bottom-panel tabs to overflow second, low-frequency side tools to drawers third | title/context identity, dominant task surface, trust/recovery state, and visible focus remain reachable |
| `standard_desktop` | 1280-1599 px | sidebar persists, inspector is on demand, bottom panel may persist | no critical action becomes hidden-only, and no focus jump occurs when a surface docks or sheets |
| `expanded_desktop` | 1600+ px | two side surfaces may coexist if the main workspace stays dominant | extra space may not become duplicated chrome or a second truth location |

Rules (frozen):

1. Adaptive collapse narrows detail surfaces before it narrows identity,
   preview, approval, trust, recovery, or the active task surface.
2. When a zone moves between docked and sheet presentation, the logical
   focus chain and command route stay the same.
3. A compact shell menu may summarize overflowed state, but it may not
   become the only route to a trust-critical or recovery-critical action.

### Windowing and event-loop model

The per-window shell event loop is frozen as one ordered pipeline:

1. A platform adapter receives a native event for one native window.
2. The adapter translates it into a typed Aureline event without
   collapsing text input, command invocation, focus movement, or display
   topology changes into one generic callback.
3. The shell resolves the target window, current adaptive class, and
   logical focus owner before dispatching the event.
4. The shell decides whether the event is text input, a command-route
   candidate, a zone/layout event, or a lifecycle event.
5. The owning surface consumes the event through the shared command,
   input, or restore contracts.
6. The renderer and accessibility tree publish deltas after state
   changes, not as independent sources of truth.

Rules (frozen):

1. The shell event loop must remain live even if PTY, language-host,
   remote, AI, or other non-shell processes fail.
2. Dialogs, sheets, permission prompts, and transient overlays are
   window-local objects. They may not float as global process state.
3. Display topology changes may change safe bounds, scale bucket, or
   fullscreen/snap state, but they may not silently change window
   ownership or focus owner.
4. Wake, restart, or reopen may restore shell structure before live
   authority. `skeleton` and `hydrate` may run automatically; `rebind`
   requires explicit revalidation where authority changed.

### Focus-chain ownership

The `focus_chain` is a window-topology object, not a renderer detail and
not an OS-only detail.

| Focus case | Shell requirement | Must never happen |
|---|---|---|
| zone-to-zone movement | deterministic next/previous order across title/context, rail, sidebar, workspace, panel, inspector, and overlays | hover-dependent or timing-dependent focus order |
| overlay open | record the invoking target before the overlay steals focus | open an overlay with no return target |
| overlay close | restore to the recorded target when it is still visible and valid | dump focus at the window root or a hidden widget |
| target lost during close | route focus to the nearest visible owner in the same window and emit the typed focus-loss reason | silently lose focus or jump to a different window |
| pane or inspector degraded | keep focus in the surrounding visible topology and preserve command reachability | focus a placeholderless missing surface or collapse the whole zone |
| dialog or sheet on moved/off-screen window | recenter the owned prompt with the owner window and return focus there on dismiss | orphan a dialog on a detached or hidden display |

Rules (frozen):

1. Focus return reuses `docs/ux/shell_interaction_safety_contract.md`
   for typed focus-return records; this ADR does not mint a second
   outcome vocabulary.
2. Hidden, filtered, blocked, or unreachable surfaces are not valid
   focus targets.
3. Embedded surfaces may receive focus inside their declared boundary,
   but host-owned chrome remains in the host focus graph and retains
   product-owned review/approval routes.

### Command-dispatch entry points

Every shell-facing route into an action is normalized to the same
command-dispatch boundary.

| Entry point | Required route | Special rule |
|---|---|---|
| keybinding / shortcut | command descriptor + invocation session | text-producing chords lose to active composition state |
| application menu or native menu item | same command descriptor and disabled reason as other surfaces | no command may exist only in an OS menu on a claimed row |
| toolbar, status item, sidebar action, panel action | same command descriptor and preview/approval posture | button location never changes consequence class |
| command palette | canonical direct route | palette does not gain bypass authority over preview or trust review |
| context menu | canonical direct route scoped to focused context | context menu cannot hide a wider blast radius than the descriptor declares |
| deep link or protocol handler | deep-link resolver plus command route | origin, expected action, scope, and trust state revalidate before dispatch |
| notification reopen target | durable object reopen plus optional command route | notification clicks may reopen context, but they may not skip review or approval |
| embedded surface host handoff | host-native command route only | embedded content may request; it may not invoke protected actions directly |

Rules (frozen):

1. Shell-specific shortcuts, menus, deep links, notification clicks, and
   embedded surfaces may not bypass preview, approval, trust prompts, or
   typed degraded-state disclosure.
2. Native menu bars, app menus, and protocol handlers are integration
   surfaces, not alternate business-logic stacks.
3. Any command reachable from OS-only chrome must also be reachable from
   a host-rendered route that exposes the same labels, disabled reasons,
   and accessibility narration.

### Text input normalization

Platform adapters report raw input facts; the shell owns the shared
normalization rules.

| Surface family | Shared-shell input contract | Required degradation when the full path is unavailable |
|---|---|---|
| editor | full IME preedit, grapheme-aware movement/deletion, bidi-preserving committed text, multi-cursor composition only when coherent | reduce to one visible composition target or block the action explicitly |
| terminal | composition-aware text entry, paste review, and command-key routing only when composition is inactive | keep terminal view usable, but deny unsafe input replay or hidden control |
| palette and search | composition survives filtering and result churn; filtering never silently commits or cancels composition | pause filtering or keep the current preedit target visible |
| settings, dialogs, and rename fields | same text-input policy as editor text fields, with explicit focus-return target | block submit/close routes that would silently discard composition |
| notebooks and other text-entry panels | same normalization as editor text-entry surfaces | narrow to one visible target or explicit unsupported state |

Shared normalization rules (frozen):

1. Dead keys, compose sequences, AltGr, emoji pickers, and IME sessions
   remain platform-owned at the raw-input layer. The shell does not
   emulate them with synthetic key sequences.
2. AltGr and dead-key composition may not be misclassified as command
   modifiers when the adapter reports that they are text production.
3. The shell preserves committed Unicode text exactly as delivered by
   the platform input path. Rendering and review surfaces may annotate
   bidi or invisible controls, but they do not normalize them away.
4. Focus changes, filtering, adaptive collapse, and command-preview
   open/close cycles may not silently commit or cancel composition.
5. Restore, reconnect, and shared-session join never replay latent text
   input or composition state.

### Accessibility-tree ownership and bridge strategy

Accessibility is a host-owned semantic contract rooted in the shell and
renderer, then bridged by the platform adapter.

| Layer | Ownership |
|---|---|
| host semantic tree | shell owns the tree lifecycle and the semantic identity of host-rendered surfaces |
| node publication | renderer publishes node deltas from the same surface/view truth used for paint |
| OS bridge | platform adapter carries the tree into UIA, NSAccessibility, or AT-SPI |
| embedded boundary chrome | host owns the owner/origin/trust chrome and its semantic nodes even when the embedded body is degraded |

Rules (frozen):

1. A visible host-owned control that is missing from the accessibility
   tree is a correctness bug, not an optimization.
2. The focus model is independent of paint order and accessibility-tree
   update timing.
3. Host-owned review, approval, trust, update-verification, and
   rollback/restore surfaces remain host-native and semantically
   addressable even when a nearby embedded or OS-integrated surface is
   involved.
4. When the accessibility bridge is degraded or unavailable, the shell
   must surface a truthful degraded state and keep keyboard-complete
   routes available where possible; it may not continue claiming normal
   assistive-tech parity.

### Platform-adapter boundaries

The adapter split is frozen by subject, not by crate name.

| Subject | Shared-shell boundary | Platform-adapter boundary |
|---|---|---|
| native window and chrome | zone model, adaptive class, focus ownership, command reachability | native window handle, titlebar/system-menu integration, fullscreen/snap/Spaces bindings |
| menus and accelerators | command ids, labels, disabled reasons, preview/approval gates | OS menu bar/app menu/system-menu projection and accelerator notation |
| text input | text-vs-command arbitration, composition lifecycle, surface degradation rules | raw key events, IME services, dead keys, AltGr, compose sequences, emoji and dictation entry |
| accessibility | tree ownership and semantic node identity | OS accessibility bridge transport |
| browser handoff and deep links | command route, origin validation, trust review, reopen target | default-browser invocation, callback registration, native protocol/file-handler hooks |
| dialogs, clipboard, drag/drop, notifications | command route, object identity, reopen/repair route, preview gates | native dialog, clipboard, drag/drop, notification-center APIs |
| keychain, secret store, and trust store | degraded-state copy, command disablement, inspect-only/session-only fallback | OS credential-store and trust-store APIs plus native failure reasons |
| sleep, display reconnect, removable volume, restore hints | lifecycle policy, restore-vs-rebind boundary, placeholder rules | native event delivery, display facts, mount/volume facts, safe-bounds metadata |

This split is mirrored in
`artifacts/ux/desktop_shell_boundary_matrix.yaml`.

### Desktop lifecycle, restore, and degraded-state rules

The shell may restore structure and evidence before it restores live
authority.

| Event or case | May restore automatically | Must revalidate explicitly | Must never happen silently |
|---|---|---|---|
| app restart / reopen | window shells, zone openness, pane topology, titles, cwd hints, transcripts, outputs, focus chain, inspector state | routes, credentials, trust/policy state, remote/debug/notebook/terminal authority | rerun commands, resume hidden input, rejoin privileged control |
| wake from sleep | local-safe shell structure and durable evidence | callbacks, remote routes, browser handoff state, live collaboration control, privileged execution | replay publish, terminal input, debug continue, or approval tickets |
| display reconnect / topology change | safe-bounds remap, scale-bucket update, reachable owner dialogs | none beyond display facts unless authority also changed | strand windows, sheets, or focus off-screen |
| removable volume return / missing root | layout and placeholders around the missing root | root identity and any privileged write path | silently remap to a different target or present data loss as local deletion |
| keychain or trust-store loss | local-safe shell and read-only or session-only work where allowed | any action needing durable secret or trusted transport | fallback to plaintext secret storage or bypass trust validation |

Detailed rows live in
`docs/architecture/input_adapter_failure_modes.md`.

## Consequences

- The canonical shell-zone model is now frozen and distinct from the
  renderer-spike trace-zone vocabulary.
- Multi-window, platform-conformance, shell-metrics, and verification
  work now have one contract for focus routing, adaptive collapse,
  restore-vs-rebind, and adapter ownership.
- Native menus, deep links, notifications, and embedded surfaces are
  explicitly constrained to the same preview/approval/trust gates as
  every other command entry point.
- Later shell work may still refine command-registry placement,
  keyboard-scheme breadth, and accessibility semantic-surface packets,
  but those lanes must now cite this boundary rather than re-litigating
  ownership.
- Detailed command-system home, full keyboard-complete command-graph
  semantics, and richer accessibility packet families remain follow-on
  decisions; this ADR freezes the boundary they must plug into.

## Alternatives considered

- **Let the renderer ADR, layout contract, and platform matrix keep
  local shell rules.** Rejected because those documents each own only
  part of the problem and would leave multiple competing sources for
  focus, input, and restore behavior.
- **Let per-OS adapters own more shell behavior.** Rejected because it
  would turn macOS, Windows, and Linux into parallel shell products and
  break parity auditing.
- **Allow native menus, deep links, notifications, or embedded surfaces
  to special-case protected actions.** Rejected because it would create
  hidden bypasses around preview, approval, and trust review.
- **Leave the row unresolved and freeze shell broadening.** This was the
  default-if-unresolved posture. It would have kept multi-window,
  native-menu, deep-link, and accessibility-bridge widening frozen at
  the spike boundary until a shared shell contract landed. That
  narrowing did not land because this ADR closes the row.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md:770` - "Desktop shell process ... input, rendering, layout, accessibility tree, command routing"
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1506` - "every user-visible action reachable by mouse should resolve to a stable command ID and keyboard route"
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4452` - "semantic accessibility tree for custom-rendered surfaces"
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4479` - "IME composition across editor, palette, settings, terminal, and rename inputs"
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4520` - "IME composition, dead keys, compose sequences, AltGr, emoji input, bidi text"
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4521` - "correct DPI scaling, fractional scaling, multi-monitor moves, fullscreen, spaces/desktops, and restore across topology changes"
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4524` - "custom-rendered surfaces bridge into OS accessibility APIs"
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:554` - "stable zone model"
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:658` - "Compact desktop ... inspector becomes sheet or overlay"
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:8714` - "focus changes, filtering, or command previews must not silently commit or cancel composition"
- `.t2/docs/Aureline_Technical_Design_Document.md:12797` - "restore layout, titles, and cwd hints only; user reaffirms execution intent"
- `.t2/docs/Aureline_Technical_Design_Document.md:13272` - "Resume ... Must not do silently recreate the workspace"
- `.t2/docs/Aureline_Technical_Design_Document.md:14508` - "silent invocation from recipes, AI, or deep links"
- `.t2/docs/Aureline_Milestones_Document.md:803` - "Dialogs, sheets, and permission prompts ... return focus to the logical origin"
- `.t2/docs/Aureline_Milestones_Document.md:1076` - "OS / companion notifications ... no hidden bypass around preview/approval rules"

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0022`
- Machine-readable boundary matrix: `artifacts/ux/desktop_shell_boundary_matrix.yaml`
- Failure and degraded-state table: `docs/architecture/input_adapter_failure_modes.md`
- Related contracts: `docs/ux/shell_interaction_safety_contract.md`, `docs/platform/desktop_platform_conformance_matrix.md`, `docs/qa/multi_window_verification.md`, `docs/workspace/layout_serialization_contract.md`, `docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`
- Affected packages / lanes: `crates/aureline-shell-spike`, `crates/aureline-render`, `governance_lane:shell_command_system`, `governance_lane:accessibility_input_review`

## Supersession history

None.
