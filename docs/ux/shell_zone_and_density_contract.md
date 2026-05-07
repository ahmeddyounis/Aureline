# Shell-zone, density, and window-restore contract

This document freezes the desktop shell shape — **zones, default
metrics, density modes, adaptive window classes, zone-priority /
collapse policy, and multi-window / multi-monitor / off-screen
restore rules** — at the level of detail the renderer, layout,
and later implementation lanes need in order to stop guessing
per surface.

It is the narrative companion to two machine-readable artifacts
and a worked-example corpus:

- [`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
  — the canonical zone map, minimum widths, density modes,
  adaptive window classes, collapse order, and restore rules.
- [`/artifacts/ux/zone_priority_rules.yaml`](../../artifacts/ux/zone_priority_rules.yaml)
  — cue-level priority ladders and collapse ladders for responsive fallback.
- [`/fixtures/ux/shell_layout_classes/`](../../fixtures/ux/shell_layout_classes/)
  — reviewable scenario fixtures for compact, standard,
  expanded, split-heavy, and multi-window states plus an
  off-screen-restore recovery fixture.
- [`/fixtures/ux/responsive_fallback_cases/`](../../fixtures/ux/responsive_fallback_cases/)
  — cue-focused stress cases for narrow width, split pressure, density/zoom,
  restore, and long-title/long-path conditions.
- [`/artifacts/ux/desktop_shell_boundary_matrix.yaml`](../../artifacts/ux/desktop_shell_boundary_matrix.yaml)
  — the ADR-0016 boundary rows this contract composes with.

If this document and the source UI/UX spec disagree, the spec
wins and this contract plus the companion YAML and fixtures
MUST update in the same change. If this document and a
downstream surface's private metric / density / collapse story
disagree, this contract wins and the surface is non-conforming.

This contract rides alongside — it does **not** re-mint — the
vocabularies already frozen in:

- [`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md)
  — shell-zone ids, adaptive classes, focus and command-entry
  rules, restore-vs-rebind boundary.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — `responsive_fallback_mode`, `required_visible_field_class`,
  `focus_return_state`, and the `chrome_hid_required_field`
  denial. This contract names **where** chrome may collapse;
  the interaction-safety contract names **what** must remain
  visible at commit.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  — workspace-authority / window-topology / profile-defaults /
  machine-or-display-hints separation, restore phases
  (`chooser` → `skeleton` → `hydrate` → `rebind` →
  `evidence_only_fallback`), and placeholder rules.
- [`/docs/qa/multi_window_verification.md`](../qa/multi_window_verification.md)
  — scenario-id family and claimed-profile notes for
  topology, mixed-DPI, suspend/resume, and reopen drills.
- [`/docs/architecture/input_adapter_failure_modes.md`](../architecture/input_adapter_failure_modes.md)
  — degraded-state posture for off-screen restore, wake,
  display reconnect, and removable-volume return.

## Who reads this document

- **Renderer and layout owners** needing one canonical set of
  shell-zone ids, minimum widths, density row heights, and
  collapse priorities to cite from scene composition and
  adaptive-layout code.
- **Shell / review / AI / extension / provider surface authors**
  deciding where a new surface attaches, which zone hosts it
  across adaptive classes, and what fallback it shares with
  sibling surfaces.
- **Product writers and designers** sizing density presets,
  compact-shell overflow copy, and off-screen-restore
  recenter copy without minting per-surface rules.
- **QA / parity tooling** reading the YAML rows and fixtures
  mechanically to check that a shell rehearsal preserves
  required identity cues under every adaptive class.

## 1. Scope

- Freeze one canonical shell-zone map covering the title /
  context bar, activity rail, left sidebar, main workspace,
  right inspector, bottom panel, status bar, and transient
  overlay — labelling each as **required** or **optional**
  on a given adaptive class.
- Freeze default metrics at 100 % zoom, standard density:
  minimum / recommended / maximum widths and heights per zone,
  minimum hit targets, and resize-handle sizes.
- Freeze three density modes (`compact`, `standard`,
  `comfortable`) with row and control heights and the
  explicit non-effect list (command semantics, focus order,
  information architecture, and state vocabulary MUST NOT
  change with density).
- Freeze five adaptive window classes (`compact_desktop`,
  `standard_desktop`, `expanded_desktop`, `split_heavy_desktop`,
  `multi_window_desktop`) with width bands, required collapse
  order, and non-negotiables.
- Freeze the zone-priority and collapse policy for
  breadcrumbs, status indicators, tabs, terminal headers,
  inspector headers, and other identity cues so fallback
  preserves truth instead of merely hiding controls.
- Freeze multi-window, multi-monitor, and off-screen-restore
  rules so layout intent and visible identity survive monitor
  changes without a silent pixel-perfect restore.

## 2. Out of scope

- Polished first-beta visuals. This contract names logical
  metrics and tokens; the UX design-system style guide owns
  final visual specifications.
- Final platform-specific chrome behavior. Per-OS titlebar,
  fullscreen, snap, and Spaces integration stays in the
  platform-adapter contract referenced in ADR 0016.
- The eventual shell crate's Rust types. This contract
  freezes the vocabulary and boundary shape the later crate
  must plug into.
- The production layout-restore engine. This contract names
  the zone-shape / density / adaptive rules the restore
  engine reads; `layout_serialization_contract.md` owns the
  phases and placeholder payloads.

## 3. Frozen vocabulary (re-exported)

This contract mints no new shell-zone ids, adaptive classes,
or restore phases. Every row resolves to values from the
adjacent contracts.

- Shell zones (`title_context_bar`, `activity_rail`,
  `left_sidebar`, `main_workspace`, `right_inspector`,
  `bottom_panel`, `status_bar`, `transient_overlay`) —
  ADR 0016 + `artifacts/ux/desktop_shell_boundary_matrix.yaml`.
- Adaptive classes (`compact_desktop`, `standard_desktop`,
  `expanded_desktop`) — ADR 0016. This contract additionally
  names `split_heavy_desktop` and `multi_window_desktop` as
  **shell-state classes**, not new width bands, to capture
  split-rich and secondary-window postures without widening
  adaptive authority.
- Density modes (`compact`, `standard`, `comfortable`) —
  UI/UX spec §6.4.
- Responsive-fallback modes (`full_chrome`, `compact_shell`,
  `split_shell`, `narrow_width_sheet`, `very_narrow_compare`,
  `zoom_400_overflow`, `missing_extension_placeholder`,
  `presentation_overlay_dimmed`) —
  `shell_interaction_safety_contract.md` §Core axes.
- Required-visible-field classes (`target_identity`,
  `actor_identity`, `authority_class_label`,
  `consequence_class_label`, `scope_statement`,
  `recovery_class_label`, `policy_source_label`,
  `expiry_or_revocation_claim`,
  `blocked_or_hidden_member_count`,
  `representation_class_label`, `basis_snapshot_freshness`) —
  `shell_interaction_safety_contract.md` §Required-visible-field set.
- Restore phases (`chooser` → `skeleton` → `hydrate` →
  `rebind` → `evidence_only_fallback`) —
  `layout_serialization_contract.md` §4.
- Multi-window scenario ids
  (`split_layout_detached_auxiliary_focus`,
  `display_detach_dock_safe_bounds`,
  `mixed_dpi_cross_monitor_reflow`,
  `fullscreen_snapped_restore_intent`,
  `offscreen_dialog_owner_recenter`,
  `suspend_resume_remote_rebind`,
  `restart_reopen_live_surface_rebind`,
  `restart_reopen_missing_dependency_placeholder`) —
  `multi_window_verification.md` + `artifacts/qa/window_display_matrix.yaml`.

## 4. Truthfulness posture (normative)

Every rule below is normative. A shell rehearsal, renderer
spike, or later implementation that violates any of them is
non-conforming regardless of how the violation is painted.

1. **Zones are identity, not chrome.** Zones keep their ids
   and slot family across density changes, adaptive classes,
   split-rich and multi-window postures, and restore phases.
   A surface that collapses a zone's **slot** instead of its
   **contents** is non-conforming.
2. **Required zones never silently disappear.** Zones marked
   required on an adaptive class MUST remain reachable. A
   fallback MAY collapse the zone into an overflow menu,
   summary chip, or keyboard shortcut; it MUST NOT render the
   zone as absent with no re-entry affordance.
3. **Density is presentation, not architecture.** Density
   changes affect row height, control height, padding,
   chrome thickness, and inline-action overflow threshold.
   Density MUST NOT change command meaning, information
   architecture, focus order, keyboard routes, or state
   vocabulary. The shell MUST NOT silently switch density
   because a provider loaded, a theme changed, or a workflow
   changed.
4. **Collapse narrows detail, not identity.** Adaptive
   collapse moves right-inspector detail, low-frequency
   side-tools, and secondary bottom-panel tabs out first;
   it MUST preserve title / context identity, the dominant
   task surface, trust / recovery state, the required-
   visible-field set for the active consequence class, and
   visible focus.
5. **Fallback preserves truth, not just controls.** Every
   required identity cue (breadcrumbs, status indicators,
   tabs, terminal headers, inspector headers) has a declared
   fallback surface — summary chip, overflow menu,
   placeholder card, or status-bar aggregate — and the
   fallback stays reachable by keyboard. A compact shell that
   hides a critical action with no disclosure is non-
   conforming (`chrome_hid_required_field`).
6. **Layout intent over pixels.** Multi-window,
   multi-monitor, and off-screen restore preserves window
   role, pane order, active tab, visible focus, and reachable
   recovery actions. Stale monitor coordinates are best-
   effort metadata; safe-bounds remap beats pixel-perfect
   restore.
7. **Restore structure before authority.** `skeleton` and
   `hydrate` may run automatically, replacing unavailable
   panes with placeholders in the original slot. `rebind`
   requires explicit revalidation wherever authority,
   credentials, remote routes, or privileged live surfaces
   changed. A silent re-attach is non-conforming.
8. **Off-screen restore is a typed event.** A window whose
   last-known bounds are no longer reachable recenters with
   a `safe_bounds_remap` note recorded on restore provenance
   and keeps owner dialogs attached. A stranded window or
   orphaned sheet is non-conforming.

## 5. Canonical shell-zone map

The eight shell-zone ids are the product zones from the UI/UX
spec and the ADR-0016 canonical model, not the narrower
renderer-spike trace zones. Every row names:

- **Primary role** — what the zone exists to say.
- **Slot family anchor** — the stable slot id the shell,
  renderer, and extension surfaces attach to.
- **Required on** — adaptive classes where the zone is
  non-negotiable (the zone MUST render and remain reachable).
- **Optional on** — adaptive classes where the zone MAY
  collapse into overflow / sheet / drawer provided the
  fallback surface named in §8 exists.
- **Focus rule** — how the focus graph treats the zone.

| Shell zone | Primary role | Slot family | Required on | Optional on | Focus rule |
|---|---|---|---|---|---|
| `title_context_bar` | identity and global state (workspace, repo / branch, trust class, remote target, profile) | `title_context_bar` | every adaptive class | — | one deterministic tab stop before delegating to embedded controls |
| `activity_rail` | durable top-level mode switching (Explorer / Search / Source / Run / Extensions / Collaboration / Support) | `activity_rail` | every adaptive class; MAY narrow to icon-only overflow on `compact_desktop` | — | rail focus never depends on hover; overflow remains keyboard reachable |
| `left_sidebar` | structural navigation (tree, search, source-control lists, dependency views, admin queues) | `sidebar` | `standard_desktop`, `expanded_desktop`, `multi_window_desktop` primary window | `compact_desktop` (may sheet on demand), `split_heavy_desktop` (one primary side surface at a time) | focus lands on the active structural view, not a hidden toolbar |
| `main_workspace` | dominant task surface (editors, diff / merge, notebooks, dashboards, review packets, visual designers) | `editor_chrome` + workspace-local surfaces | every adaptive class | — (never optional) | default return target; focus goes here when no other safe target exists |
| `right_inspector` | contextual explanation and detail (docs, symbol details, AI context, run metadata, provider state) | `inspector` | — | every adaptive class (sheet-first on `compact_desktop`, `narrow_width_sheet`) | optional context focus; if invalid, returns to main workspace |
| `bottom_panel` | execution, output, and longitudinal state (terminal, problems, logs, debug, tests, activity) | `bottom_panel` | `standard_desktop`, `expanded_desktop`, `split_heavy_desktop`, `multi_window_desktop` | `compact_desktop` (collapsed or tabbed) | active panel tab owns focus; durable state remains reachable when secondary tabs collapse |
| `status_bar` | persistent low-urgency state and toggles (language mode, execution target, sync state, background jobs) | `status_bar` | every adaptive class | — | narrates current state and routes to the owning surface; MAY summarize overflowed items on `compact_desktop` but MUST NOT hide trust / recovery state |
| `transient_overlay` | scoped interruption or quick access (palette, dialogs, sheets, quick pickers, context menus) | `command_palette`, `global_menu`, `context_menu`, sheets, dialogs | every adaptive class | — | records a focus-return target before opening; remains window-local |

Rules (frozen):

1. **One zone, one slot family.** A shell surface attaches to
   exactly one slot family; a surface that registers to two
   slot families to dodge collapse policy is non-conforming.
2. **Required zones remain reachable.** A required zone MAY
   compress (icon-only rail, hidden labels, status-bar
   summary) but MUST remain reachable by keyboard and
   announced by the accessibility tree.
3. **Optional zones keep identity.** A zone that sheets or
   drawers on collapse retains its id, active occupant, and
   last meaningful size until the user intentionally discards
   it. A collapsed inspector that forgets its target is non-
   conforming.
4. **No undeclared shell chrome.** A new product or extension
   surface MUST attach to a declared slot family. Floating
   global buttons, duplicate sidebars, or hidden keyboard-only
   destinations are non-conforming.

## 6. Default metrics

Values below are **logical pixels at 100 % zoom in standard
density**. System zoom, display scaling, and accessibility
settings take precedence. The machine-readable values live in
[`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
and win on disagreement.

| Zone or control | Minimum | Recommended default | Recommended maximum | Notes |
|---|---:|---:|---:|---|
| Title / context bar height | 32 px | 32 px | 40 px | grows only where platform chrome or accessibility settings require |
| Activity-rail width | 44 px | 48 px | 56 px | icon column is visually stable across sections; overflow lands in a drawer, not a second rail |
| Left-sidebar width | 220 px | 260–320 px | 420 px | preserves labels without forcing early truncation |
| Main-workspace minimum width | 420 px | 720 px and up | n/a | compare views drop to tabbed or staged mode before violating this |
| Right-inspector width | 280 px | 320–360 px | 420 px | sheets on `compact_desktop` before starving the workspace |
| Bottom-panel height | 180 px | 240–320 px | 45 % of window height | last meaningful size persists per workspace when possible |
| Status-bar height | 24 px | 24 px | 28 px | dense but still legible at high zoom |
| Tab minimum width | 96 px | 120–160 px | n/a | tabs overflow before labels become meaningless |
| Resize-handle hit area | 4 px | 6–8 px | n/a | visual line MAY remain thinner than the actual hit area |
| Icon-only control hit target | 28 px | 28–32 px | 36 px | logical hit target matters more than visible glyph size |

Rules (frozen):

1. **Extensions inherit shell metrics from tokens.** A
   surface that sets arbitrary widths that fracture the
   layout is non-conforming.
2. **Minimums are floors, not targets.** When a layout cannot
   satisfy the minimum, the shell MUST collapse or sheet a
   secondary zone instead of allowing a visually broken
   arrangement.
3. **Hit targets survive density.** Density MAY reduce
   padding and chrome thickness; it MUST NOT reduce hit
   targets below the minimum in this table.
4. **Truncate before you grow.** Truncation, overflow menus,
   and text wrapping are preferred over unbounded shell
   growth that hides adjacent zones.

## 7. Density modes

Density is a **presentation choice**, not a new information
architecture.

| Mode | Row height | Control height | Typical use |
|---|---:|---:|---|
| `compact` | 24 px | 28 px | large repos, expert users, smaller laptops, dense queue work |
| `standard` | 28 px | 32 px | default |
| `comfortable` | 32 px | 36 px | accessibility, notebook / data work, onboarding, review-heavy flows |

Density affects:

- row height in lists, trees, tables, and result sets;
- tab, chip, and badge spacing;
- panel padding and inspector breathing room;
- gutter spacing and some chrome thickness;
- how many inline row actions may persist before moving to
  overflow.

Density MUST NOT change:

- command semantics or command-graph identity;
- focus order or keyboard routes;
- information architecture or shell zoning;
- state vocabulary, icon meaning, or badge categories;
- the required-visible-field set at commit on any
  consequence class;
- the requirement for clear focus visibility and accessible
  hit targets.

Rules (frozen):

1. **Density is a profile default.** Preference is profile-
   level by default and survives restarts.
2. **Local presentation requires explanation.** A surface
   MAY opt into a more spacious local presentation only when
   it explains why (presentation mode, accessibility-specific
   viewer). A silent local override is non-conforming.
3. **Dense remains operable under zoom.** At 400 % zoom or
   equivalent assistive use, dense surfaces remain operable
   even when the profile prefers `compact`.
4. **Extensions declare reduced support truthfully.**
   Extensions that cannot honor supported density tokens
   declare reduced support rather than rendering an
   inconsistent private scale.

## 8. Adaptive window classes and responsive fallback

Five shell-state classes are frozen. `compact_desktop`,
`standard_desktop`, and `expanded_desktop` are **width-
bucketed** adaptive classes. `split_heavy_desktop` and
`multi_window_desktop` are **shell-state** classes that may
coexist with any width bucket; they capture split-rich and
secondary-window postures rather than widening adaptive
authority.

| Class | Width / state | Required collapse order | Non-negotiables |
|---|---|---|---|
| `compact_desktop` | 1024–1279 px | right-inspector → sheet first; secondary bottom-panel tabs → overflow second; low-frequency side tools → drawer third | title / context identity, main-workspace dominance, trust / recovery state, visible focus, status-bar summary of overflowed critical state |
| `standard_desktop` | 1280–1599 px | right-inspector on demand; sidebar and optional bottom panel persist | active-task dominance; no hidden-only commands; no focus jump on dock-to-sheet transitions |
| `expanded_desktop` | 1600+ px | no forced collapse when readability holds; two side surfaces may coexist | main-workspace dominance; no duplicated truth surfaces; no oversized empty chrome |
| `split_heavy_desktop` | any width; ≥ 2 simultaneous editor groups or compare / diff views | tabbed compare > split compare when a new group would violate the main-workspace minimum width; right-inspector sheets before a compare pane shrinks below minimum | identity of each editor group; per-group tab overflow before filename truncation; focus preserved across dock-to-sheet transitions |
| `multi_window_desktop` | any width; ≥ 2 Aureline top-level windows backed by the same workspace authority | window-local adaptive class applies per window; each window collapses independently; dialogs stay attached to the owning window | canonical command graph, workspace identity, trust state, remote / host identity, profile, and recovery-critical status preserved on every window |

Responsive-adaptation priority (frozen):

1. Move optional detail from the right inspector into a
   sheet or inline disclosure.
2. Collapse secondary bottom-panel tabs before reducing the
   main workspace below its minimum useful width.
3. Preserve path, branch, trust, and execution-target
   identity before preserving promotional or optional
   content.
4. Convert low-frequency side tools into overflow or on-
   demand drawers before collapsing primary navigation.
5. Preserve focus and keyboard continuity when surfaces move
   between docked and sheet presentations.

Rules (frozen):

1. **Palette, tabs, and status reach every width.** Command
   palette, title / context state, editor tabs, and
   recovery-critical status remain visible at every
   supported width including 400 % zoom.
2. **No hover-only reveals on narrow widths.** Essential
   actions MUST NOT become hover-only as width narrows.
3. **Identity-stable presentation swap.** A surface that
   moves from docked to sheet is the same task surface with
   the same state, not a fresh instance.
4. **Second-group violation degrades explicitly.** When
   opening a second editor group, diff, or compare view
   would violate the minimum group width, the shell falls
   back to tabbed compare, staged peek, or an explicit user
   choice rather than silently producing unusable narrow
   panes.
5. **Multi-window parity.** Every window in `multi_window_desktop`
   retains the same command graph, trust state, and
   recovery-critical status as the primary shell. Window-
   local layout, density, and active surface MAY differ;
   workspace-global risk and policy state MUST remain
   visible on every affected window.

## 9. Zone-priority and collapse policy for identity cues

Fallback preserves truth, not just controls. This section
freezes, per identity cue, what stays visible, where it
collapses, and what the fallback surface looks like. Every
cue has a declared fallback; a compact shell that hides a
critical cue with no disclosure is non-conforming.

| Identity cue | Where it lives | Fallback under `compact_desktop` / `narrow_width_sheet` | Forbidden |
|---|---|---|---|
| **Breadcrumbs** (root, folder path, file, symbol path) | main-workspace header near the active editor | overflow menu in the breadcrumb strip preserves root identity, current file, and current symbol; intermediate segments collapse into a middle ellipsis chip that expands on keyboard focus | hiding the root or current-item segment; silent collapse with no expand affordance |
| **Tabs** (per editor group) | editor-group header | tab overflow menu preserves pinned, dirty, and active tab identity; label truncation preferred to tab removal | dropping the active or dirty tab from the visible set |
| **Status indicators** (language mode, encoding, line endings, execution target, sync state, trust class, background jobs) | status bar | compact status menu summarizes low-priority items but MUST keep trust class, execution target, and any blocked / suppressed counts visible; `Status overflow / compact shell menu` names the hidden items | silent loss of trust / execution-target cues; mixing transient alerts into the summary |
| **Terminal headers** (session label, host, exit state, freshness) | bottom-panel terminal tab | tab title retains host and label; overflow reveals full session identity and exit state; `View transcript` remains reachable | replaying latent PTY input, hiding exit state, or renaming the session silently |
| **Inspector headers** (target pane ref, inspector kind, freshness, dock position) | right-inspector header | sheet header shows the same identity; `Pin back to docked` action remains reachable | opening a sheet without the target pane ref; silent re-targeting on sheet close |
| **Trust / policy / recovery-critical state** | title / context bar + status bar + per-surface chips | elevated to a compact-shell summary chip in the title / context bar and a status-bar overflow row; both remain reachable by keyboard | collapsing trust loss, broken restore, policy block, or failed publish into an undisclosed overflow |
| **Collaboration / presentation role badges** (presenter, co-presenter, observer, driver, approver) | title / context bar + window topology | sheet variant preserves the badge and shared-control chrome; badge MUST remain visible after restore | silently dropping a role badge when the zone sheets |
| **Command-palette entry** | overlay over any zone | keyboard shortcut survives every adaptive class; rail overflow entry remains available | no hiding the palette entry behind a hover or timing-dependent affordance |

Rules (frozen):

1. **Every cue has a typed fallback.** A responsive fallback
   MAY collapse secondary chrome; it MUST NOT drop any cue
   in the `required_visible_field_class` set for the active
   consequence class, and it MUST disclose collapsed cues
   through one of the fallback surfaces above.
2. **Fallback is keyboard-complete.** Every collapse has a
   keyboard-reachable re-entry affordance — overflow menu,
   sheet open, pin-back action, or command-palette route.
3. **Overflow explains its hidden items.** A compact shell
   menu names its hidden items (e.g. `3 more status items`),
   never presents an empty `…` trigger.
4. **Identity before ornament.** When space runs out, the
   shell truncates labels and drops ornament (icons-only,
   badge hidden) in that order; it does not drop identity
   text until the fallback surface takes over.

## 10. Multi-window, multi-monitor, and off-screen-restore rules

Multi-window work is a first-class workspace mode. Detached
documents, compare views, presentation windows, and secondary
displays preserve the same product truth as the primary
shell.

Per-window rules (frozen):

1. **One window → one native top-level window → one renderer
   surface root → one accessibility root** (ADR 0016 §Decision).
   Many windows MAY share one workspace authority
   (`workspace_authority_ref` in
   `layout_serialization_contract.md` §2).
2. **Every window is a full shell.** Every window retains
   the canonical command graph, workspace identity, trust
   state, remote / host identity, profile, and recovery-
   critical status cues. Window-local state MAY differ in
   layout, density, and active surface.
3. **Cross-window operations advertise their verb.** Cross-
   window drag-and-drop advertises the resulting verb
   (`Move tab`, `Copy editor`, `Open compare here`,
   `Create window`) before drop.
4. **Closing never orphans durable state.** Closing a
   secondary window MUST NOT orphan dirty buffers, active
   approvals, shared-session control, or long-running
   evidence review without an explicit continuation path.

Multi-monitor and topology rules (frozen):

1. **Layout intent over pixels.** Session restore preserves
   window role, editor-group composition, pinned / dirty
   tabs, panel visibility, and dominant monitor rather than
   brittle pixel-perfect coordinates. Stale monitor
   coordinates are best-effort metadata only.
2. **Safe-bounds remap on topology change.** When monitor
   topology, DPI, fullscreen / space state, snapped layout,
   or window-manager rules change, the shell remaps windows
   into reachable bounds and records the adjustment on
   restore provenance.
3. **Scale-bucket update stays silent on identity.** A
   display-scale change updates safe bounds, scale bucket,
   and owner-dialog placement without changing window
   ownership, focus owner, or command routes.
4. **Dialogs stay window-local.** Dialogs, permission
   sheets, trust prompts, and destructive confirmations
   attach to the owning window, name the workspace and
   target surface they affect, and return focus to the
   invoker on dismiss.
5. **Notifications route to the owner.** Notifications
   route to the owning window when context matters and
   degrade to badges or activity-center entries instead of
   stealing focus from the active typing surface.

Off-screen-restore rules (frozen):

1. **Recentre, don't strand.** A window whose last-known
   bounds are no longer reachable recenters on the nearest
   reachable display with a `safe_bounds_remap` note on
   restore provenance.
2. **Owner dialogs follow their window.** An orphaned
   dialog, sheet, or prompt recenters with its owning
   window; it MUST NOT survive on a detached or hidden
   display.
3. **Typed lifecycle, not silent resume.** Off-screen
   restore records the typed lifecycle event
   (`display_reconnect`, `wake_from_sleep`, `removable_volume_return`)
   and preserves restore provenance. Live authority
   (remote, debug, terminal, notebook, callback) requires
   explicit rebind; automatic re-attach is non-conforming.
4. **Recovery actions stay reachable.** After recentre,
   the active pane, owner dialog, and safe-recovery
   actions (`Move to primary display`, `Review layout`,
   `Restore detached window`) remain reachable by keyboard.
5. **Provenance is inspectable.** Topology adjustments,
   missing dependencies, and continuity level remain
   inspectable after reopen or wake
   (`layout_restore_provenance_record` in
   `layout_serialization_contract.md` §4).

## 11. Relationship to adjacent contracts

- **ADR 0016** is the authoritative source for shell-zone
  ids, adaptive classes, and focus / command-entry rules.
  This contract extends ADR 0016 with the metric values,
  density modes, collapse policy, and restore rules the
  renderer / layout / restore lanes need mechanically.
- **Shell-interaction-safety contract** is the authoritative
  source for `responsive_fallback_mode`,
  `required_visible_field_class`, `focus_return_state`, and
  the `chrome_hid_required_field` denial. This contract names
  **where** chrome may collapse; that contract names **what**
  must remain visible at commit and **how** focus returns.
- **Layout-serialization contract** is the authoritative
  source for workspace-authority / window-topology / profile-
  defaults / machine-or-display-hints separation, restore
  phases, and placeholder payloads. This contract is the
  zone-and-metric view the restore engine reads; it does not
  re-mint restore phases.
- **Multi-window verification seed** is the authoritative
  source for scenario-id rows and claimed-profile notes. The
  fixtures in
  [`/fixtures/ux/shell_layout_classes/`](../../fixtures/ux/shell_layout_classes/)
  reference the same scenario ids without duplicating the
  claimed-profile notes.
- **Input-adapter failure modes** is the authoritative
  source for degraded-state recovery posture (wake, display
  reconnect, keychain / trust-store loss). This contract
  reserves the adaptive-class entry points those recoveries
  plug into.

## 12. Schema-of-record posture

The eventual shell crate's Rust types are the source of
truth. The YAML export at
[`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
is the cross-tool boundary every non-owning surface reads.
Adding a new shell zone, adaptive class, density mode,
responsive-fallback mode, or restore rule is additive-minor
and bumps `shell_metrics_schema_version`. Repurposing an
existing value is breaking and requires a new decision row
in `artifacts/governance/decision_index.yaml`.

## 13. Non-goals at this milestone

Out of scope until a superseding decision row opens:

- Final visual specification of compact / standard /
  comfortable density chrome (the UX design-system style
  guide owns that).
- Per-OS titlebar, fullscreen, snap, and Spaces behavior
  (the platform adapter contract owns those).
- The production layout-restore engine, crash-loop detector,
  or multi-window runtime.
- The eventual shell crate's Rust types.

These lines move only by opening a new decision row, not by
editing this contract.

## 14. Reuse guarantee

This contract is reusable by renderer, layout, restore,
review, AI, extension, and provider-bearing flows without
redefining core shell semantics. A new shell surface MUST:

1. Attach to exactly one slot family from §5 and cite its
   zone id verbatim.
2. Honor the metric floors in §6 and the density rules in
   §7; a surface that mints a private scale is non-
   conforming.
3. Declare its adaptive-class posture (`compact_desktop`,
   `standard_desktop`, `expanded_desktop`, and how it
   behaves under `split_heavy_desktop` /
   `multi_window_desktop`) and honor the collapse order in
   §8.
4. Declare a typed fallback surface for every identity cue
   in §9 the surface owns; a surface that drops a cue under
   collapse without a fallback is non-conforming.
5. Emit a `layout_restore_provenance_record` for every
   restore it owns and a `focus_return_record` for every
   dismiss it closes, per the adjacent contracts.

## 15. Source anchors

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.1 — stable zone model.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.3 — default shell metrics.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.4 — density modes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.5 — window classes and responsive behavior.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.6 — multi-window and multi-monitor behavior.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.7 — platform conventions and desktop lifecycle failure modes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix EP — shell zone contract matrix and responsive fallback ladder.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` §12.1–§12.6 — shell, metrics, density, multi-window, and responsive behavior.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §12.7 — workspace-window, split-layout, and session-restore architecture.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.1.11 — workspace-window, split-layout, and session-restore architecture.

## 16. Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0022` (covers ADR 0016).
- Canonical shell-metrics artifact: `artifacts/ux/shell_metrics.yaml`.
- Worked-example fixtures: `fixtures/ux/shell_layout_classes/`.
- Companion boundary matrix: `artifacts/ux/desktop_shell_boundary_matrix.yaml`.
- Related contracts: `docs/adr/0016-shell-windowing-input-accessibility-boundary.md`, `docs/ux/shell_interaction_safety_contract.md`, `docs/workspace/layout_serialization_contract.md`, `docs/qa/multi_window_verification.md`, `docs/architecture/input_adapter_failure_modes.md`.
