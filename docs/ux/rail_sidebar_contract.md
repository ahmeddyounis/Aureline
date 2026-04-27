# Rail, sidebar, section budget, and reopen contract

This document freezes the shell-level contract for the activity rail,
left sidebar, top-level section budget, overflow behavior, remembered
section state, and reopen-last-section behavior. It exists so Explorer,
Search, Source Control, Run/Test, Extensions, Collaboration,
Support/Admin, extension-contributed views, and future product rows all
attach to one durable structural-navigation model instead of growing
parallel sidebars, duplicate routes, or private action-only chrome.

The machine-readable schema lives at:

- [`/schemas/ux/section_slot.schema.json`](../../schemas/ux/section_slot.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/sidebar_sections/`](../../fixtures/ux/sidebar_sections/)

This contract composes with and does not replace:

- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  for shell zones, adaptive classes, metrics, and focus behavior.
- [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  for route ids, escalation tiers, and route parity.
- [`/docs/commands/command_graph_and_ui_slots_seed.md`](../commands/command_graph_and_ui_slots_seed.md)
  for slot families, slot keys, direct projection, and
  mirror/handoff projection rules.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  for command descriptors, capability scope, preview and approval
  posture, disabled reasons, and invocation packets.
- [`/docs/ux/collection_view_contract.md`](./collection_view_contract.md)
  and [`/docs/ux/selection_and_scope_contract.md`](./selection_and_scope_contract.md)
  for durable lists, counts, filters, saved views, and batch scope.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for consequence-bearing actions, review, focus return, and
  responsive fallback.

Where this contract disagrees with the source PRD, architecture,
technical design, UI/UX spec, shell-zone contract, navigation contract,
or command contract, the source wins and this document plus its schema
and fixtures update in the same change. Where a downstream surface
mints a private rail, private sidebar, private section id, or
surface-local command route that conflicts with this document, this
contract wins and the surface is non-conforming.

## Who reads this contract

- **Shell authors** wiring the rail, left sidebar, compact sidebar
  sheet, overflow drawer, layout restore, and section commands.
- **Feature owners** deciding whether a new surface belongs in the
  rail, sidebar, bottom panel, inspector, main workspace, sheet, or
  command-only route.
- **Extension authors** contributing views or commands without adding
  duplicate global sidebars or new primary rail entries.
- **Design, accessibility, support, and parity tooling** reviewing
  section placement, command parity, no-hidden-only critical actions,
  focus return, and support-export reconstruction.

## 1. Scope

This contract freezes:

- one stable top-level section taxonomy and primary-section budget;
- one overflow policy for future rows and conditional sections;
- one duplicate-surface prevention rule set;
- one rail/sidebar/contextual placement model;
- one command-backed route set for hiding, showing, reopening, and
  jumping directly to sections;
- one remembered-section-state contract used by layout restore,
  compact sheet fallback, and reopen-last-section;
- one rule set for structural browsing, counts, durable lists, and
  one-off mutating actions.

This contract does not implement Explorer, Search, Source Control,
Run/Test, Extensions, Collaboration, Support/Admin, or any provider
logic. It freezes the shell contracts those implementations must read
before product code lands.

## 2. Out of scope

- Per-feature data models, provider protocols, and row schemas inside
  Explorer, Search, Source Control, Run/Test, Extensions,
  Collaboration, or Support/Admin.
- Final icons, copy strings, color tokens, and animation values.
- Command-registry descriptor bodies for each reserved shell command.
  This contract reserves the command ids and command parity
  requirements; launch-bearing product code must mint the matching
  descriptors before those commands ship.
- Telemetry event names. Telemetry may quote section ids and command
  ids, but this contract is not an analytics schema.

## 3. Frozen vocabulary

This contract re-exports these upstream values without redefining them:

- `shell_zone`: `activity_rail`, `left_sidebar`, `main_workspace`,
  `right_inspector`, `bottom_panel`, `status_bar`,
  `transient_overlay` from the shell-zone contract.
- `navigation_route_id`: `route.activity_rail`, `route.sidebar`,
  `route.command_palette`, `route.global_menu`,
  `route.context_menu`, `route.status_bar_link`,
  `route.inspector_link`, and related routes from the navigation
  contract.
- `slot_family_class` and `slot_key`: `activity_rail`,
  `rail.primary_route_entry`, `rail.handoff_entry`, `sidebar`,
  `sidebar.section_action`, and `sidebar.selection_action` from the
  command-graph slot taxonomy.
- `escalation_tier`, `disclosure_depth`, `issuing_surface`,
  `capability_scope_class`, `preview_class`,
  `approval_posture_class`, `disabled_reason_code`, `redaction_class`,
  and `client_scope` from the navigation and command contracts.

This contract introduces a small section-local vocabulary. Values are
closed at schema version 1; adding a value is additive-minor and bumps
`section_slot_schema_version`. Repurposing a value is breaking and
requires a governance decision row.

### 3.1 `section_slot_id`

Top-level section ids:

- `section.explorer`
- `section.search`
- `section.source_control`
- `section.run_test`
- `section.extensions`
- `section.collaboration`
- `section.support_admin`
- `section.future_overflow`

Rules:

1. These ids are product shell ids, not feature implementation module
   names. They remain stable across themes, profiles, local/remote
   operation, and extension sets.
2. `section.support_admin` is one section. Support, Project Doctor,
   policy/admin queues, service health, and diagnostics may be
   subsections, but they do not become separate primary rail entries
   by default.
3. `section.future_overflow` is a classification bucket for future
   product or extension rows. It is not a user-facing section label
   and it cannot be promoted to primary without updating the budget
   table in this contract.

### 3.2 `rail_slot_class`

- `primary` - occupies one of the stable primary rail ranks.
- `conditional_primary` - has a stable primary rank but may be hidden
  when the current client scope, profile, policy, or build does not
  admit it. Hidden conditional rows do not create a vacancy for future
  rows.
- `overflow_only` - appears in the rail overflow, command palette, or
  owning surface route. It does not occupy a primary rail rank.

### 3.3 `section_visibility_class`

- `visible_enabled` - visible and focusable; its sidebar section can
  open.
- `visible_degraded` - visible and focusable; opening shows degraded
  truth and repair routes.
- `visible_placeholder` - visible because state or policy requires a
  return path, but the provider/body is unavailable.
- `hidden_not_applicable` - hidden because the current client scope,
  profile, policy, or build does not admit the section.
- `hidden_by_user` - hidden from the sidebar body by explicit user
  action; the rail, command palette, or global menu still provides a
  return path.
- `overflowed` - present only through overflow because budget or
  policy denied primary placement.

### 3.4 `sidebar_content_class`

- `structural_tree` - hierarchical browsing such as roots, folders,
  symbols, or dependency trees.
- `result_collection` - query-backed results, saved queries, and
  scoped collections.
- `durable_queue` - review, work, support, admin, or provider queues
  that survive navigation.
- `durable_list` - installed extensions, runs, tests, sessions,
  participants, or other durable rows.
- `status_summary` - section-owned provider, freshness, or capability
  state that explains what the structural body can or cannot show.
- `contextual_group` - small groups of links into inspectors, bottom
  panels, sheets, or main workspace surfaces.

### 3.5 `placement_lane`

- `rail`
- `sidebar`
- `main_workspace`
- `right_inspector`
- `bottom_panel`
- `status_bar`
- `command_palette`
- `global_menu`
- `context_menu`
- `sheet`
- `review_surface`
- `contextual_link`
- `overflow_drawer`

### 3.6 `section_state_memory_class`

- `visible_active`
- `visible_background`
- `hidden_remembered`
- `hidden_no_prior_state`
- `compact_sheet_remembered`
- `unavailable_placeholder`
- `reset_to_default`

### 3.7 `surface_placement_verdict`

- `rail_primary`
- `rail_conditional_primary`
- `rail_overflow`
- `sidebar_section`
- `sidebar_subsection`
- `contextual_link_only`
- `bottom_panel_tab`
- `right_inspector_pane`
- `main_workspace_surface`
- `sheet_or_review_surface`
- `command_only`
- `denied_duplicate_surface`

## 4. Top-level section taxonomy and budget

The primary rail budget is **seven** top-level section ranks. The
budget is already allocated by this taxonomy:

| Rank | Section id | Rail slot class | Sidebar promise | Primary ownership |
|---:|---|---|---|---|
| 10 | `section.explorer` | `primary` | workspace roots, file tree, structural browse, root-scoped actions | file and workspace structure |
| 20 | `section.search` | `primary` | search inputs, result collections, saved queries, scope/freshness state | query-backed discovery and codebase lookup |
| 30 | `section.source_control` | `primary` | changes, branches, worktrees, review queues, provider state | version-control structure and review adjacency |
| 40 | `section.run_test` | `primary` | run configs, test tree, debug targets, task/debug/test state summaries | executable target browsing and session setup |
| 50 | `section.extensions` | `primary` | installed extensions, marketplace/mirror rows, activation status, permission summaries | extension inventory and extension-host state |
| 60 | `section.collaboration` | `conditional_primary` | sessions, participants, invites, follow/control grants, shared-state summaries | collaboration and shared-control state |
| 70 | `section.support_admin` | `conditional_primary` | Project Doctor, support bundles, diagnostics, policy/admin queues, service health | supportability, diagnostics, and admin-facing queues |
| 900+ | `section.future_overflow` | `overflow_only` | only after a placement decision proves structural need | future or extension rows |

Rules (frozen):

1. The rail exposes at most seven primary section ranks before
   overflow. The overflow control does not count as a section rank.
2. Primary ranks never reorder because a provider warms, an extension
   activates, a remote reconnects, or a policy overlay appears.
3. Conditional primary ranks may hide when not applicable, but the
   hidden rank is reserved. A future or extension row does not move up
   to fill the visual slot unless this table changes.
4. Future rows default to `section.future_overflow` and
   `overflow_only`. Promotion to a primary rank requires removing,
   merging, or demoting another rank while preserving the budget.
5. A surface spanning multiple sections declares one owning section
   for structural browsing and uses contextual links elsewhere. It
   must not duplicate durable lists across sections.
6. Extension-contributed structural views attach to an existing
   section or the overflow drawer. Extensions do not create a second
   activity rail, a second left sidebar, or a new primary rail rank by
   default.

## 5. Rail contract

The rail is a durable top-level route map. It answers "which major
structural mode am I entering?" It does not answer "which mutating
operation should run now?"

Rail entries may:

- focus or reopen the owning sidebar section;
- show stable section identity, coarse availability, and aggregate
  attention counts;
- expose overflow and handoff entries;
- seed the command palette or focus an existing slot through
  `rail.primary_route_entry` or `rail.handoff_entry`;
- preserve a keyboard path to each section jump.

Rail entries must not:

- dispatch destructive, credentialed, external, publish, install,
  admin, or broad mutation commands directly;
- become a scrolling secondary toolbar of one-off actions;
- host feature-local filters, result rows, review actions, run
  controls, or install buttons;
- invent rail-only command ids or rail-only authority classes;
- hide the only route back to a hidden sidebar section.

Allowed rail count and badge classes:

- `aggregate_attention_count`
- `provider_degraded_count`
- `pending_review_count`
- `changed_item_count`
- `running_or_failed_session_count`
- `blocked_policy_count`

Rules:

1. Rail counts are summaries only. Detailed counts, filters,
   selected-member counts, hidden-member counts, and saved-view truth
   live in the sidebar or owning collection.
2. Rail counts quote the same source records the sidebar uses. A rail
   badge cannot compute private truth.
3. Selecting a rail entry while the sidebar is hidden reopens the
   last remembered state for that section when one exists, otherwise
   it opens that section's default structural view.
4. Selecting an overflow entry uses the same reopen rules as a primary
   entry. Overflow is not a second sidebar and not a hidden execution
   surface.

## 6. Sidebar contract

The left sidebar is the durable structural-navigation lane for the
active top-level section. It hosts trees, result collections, durable
lists, queues, saved views, scope/freshness summaries, and the section
toolbar needed to browse those structures.

Sidebar sections may contain:

- file, symbol, root, dependency, or framework trees;
- search inputs, result lists, saved queries, and scope/freshness
  banners;
- source-control changes, branch/worktree lists, hosted review rows,
  and provider status;
- run configs, test trees, debug target lists, task summaries, and
  links into bottom-panel output;
- extension inventory, marketplace/mirror rows, permission summaries,
  activation/degradation state, and extension-host diagnostics;
- collaboration sessions, participants, invites, follow/control grant
  summaries, recording/retention state, and handoffs to review sheets;
- Project Doctor findings, support bundle rows, diagnostics, policy
  queues, service health, and admin-relevant status summaries.

Sidebar sections must not contain as their only path:

- critical destructive, credentialed, external, publish, install,
  policy-authoring, admin-waiver, or broad mutation actions;
- browser or OS handoff actions with no command descriptor;
- one-off review gates that need a sheet, modal, diff, or full review
  surface;
- private sidebars or feature-local global navigation systems;
- hover-only or pointer-only actions.

Rules:

1. Structural browsing belongs in the sidebar when the user scans,
   filters, expands, sorts, selects, or revisits a durable population.
2. The section toolbar may host low-risk structural controls such as
   filter toggles, refresh, collapse/expand, sort, view mode, and
   saved-view selection only when each control resolves to a command
   descriptor or inherited collection command.
3. One-off mutating actions belong in command, toolbar, context menu,
   sheet, review, bottom-panel, or main-workspace surfaces according
   to their consequence class. The sidebar may link to those surfaces
   but does not privately own them.
4. A sidebar subsection is allowed only when it is still part of the
   owning section's structural model. A "Git Review" subsection belongs
   under Source Control; it does not become its own primary rail row.
5. Sidebars preserve count truth. Visible, loaded, matching, selected,
   blocked, stale, provider-limited, and approximate counts use the
   collection and selection contracts rather than local synonyms.

## 7. Other shell lanes

Use this decision table before adding a new surface:

| User need | Default lane | Reason |
|---|---|---|
| Switch among top-level structural modes | `rail` | durable mode switching |
| Browse durable hierarchy, result set, queue, or list | `sidebar` | sustained structural scanning and counts |
| Edit, review, diff, dashboard, notebook, visual designer, or full evidence view | `main_workspace` | dominant task surface |
| Read contextual details about the active target | `right_inspector` | explanation and metadata, not primary navigation |
| Watch execution, terminal, logs, debug console, test output, or longitudinal job state | `bottom_panel` | output and live execution state |
| Jump from persistent low-urgency state to its owner | `status_bar` | contextual state route |
| Run or discover a known command | `command_palette` or `global_menu` | command graph entry |
| Choose or approve a consequence-bearing operation | `sheet` or `review_surface` | preview, approval, and focus-return truth |
| Point from one owner to another without duplicating data | `contextual_link` | cross-surface handoff |

Rules:

1. If the surface's primary object is a durable collection, it needs a
   sidebar or main-workspace owner.
2. If the surface's primary object is output over time, it belongs in
   the bottom panel unless it needs a full review/evidence surface.
3. If the surface explains the currently active object, it belongs in
   the inspector.
4. If the surface commits a consequence, it must resolve through a
   command descriptor and the shell-interaction-safety contract.
5. If the surface exists only to link elsewhere, it is a contextual
   link or handoff, not a new section.

## 8. Command-backed section routes

The rail and sidebar are routes into the command graph, not private
execution planes. These command ids are reserved by this contract:

| Purpose | Required command id |
|---|---|
| Hide the current sidebar | `cmd:shell.sidebar.hide` |
| Toggle sidebar visibility | `cmd:shell.sidebar.toggle` |
| Reopen the last sidebar section | `cmd:shell.sidebar.reopen_last` |
| Open Explorer | `cmd:shell.section.open.explorer` |
| Open Search | `cmd:shell.section.open.search` |
| Open Source Control | `cmd:shell.section.open.source_control` |
| Open Run/Test | `cmd:shell.section.open.run_test` |
| Open Extensions | `cmd:shell.section.open.extensions` |
| Open Collaboration | `cmd:shell.section.open.collaboration` |
| Open Support/Admin | `cmd:shell.section.open.support_admin` |
| Open rail overflow | `cmd:shell.section.open_overflow` |

Rules:

1. Launch-bearing product code must mint command descriptors for these
   ids before the corresponding UI ships.
2. Rail entries are `handoff_bearing`: they focus a sidebar section,
   open overflow, or seed the command palette. They do not widen
   command authority.
3. Sidebar hide, toggle, reopen, and direct section jumps must be
   reachable through at least two route classes, one of which is
   keyboard first. Acceptable pairs include rail plus command palette,
   global menu plus command palette, or keybinding plus command
   palette.
4. A critical action shown inside a sidebar row, section toolbar, or
   context menu must cite its canonical command id and an independent
   command-palette or menu route. If no descriptor exists, the action
   cannot ship as launch-bearing UI.
5. Private sidebar affordances may narrow visibility by selection,
   policy, lifecycle, trust, or client scope. They may not change
   preview, approval, capability, result contract, or authority class.

## 9. Hide, remember, and reopen-last-section

Hiding the sidebar is a layout action, not a data loss action. The
shell records the last meaningful section state and keeps a visible
return path.

### 9.1 State captured on hide

When the sidebar hides, the shell records:

- `last_active_section_id`;
- `active_view_ref` or default view id for that section;
- focus owner kind and the safest focus-return target;
- sidebar width or compact sheet posture;
- expanded tree nodes, selected saved view, filter, sort, scope, and
  scroll anchor where the owning collection says those values are
  restorable;
- current selection summary and hidden-selected disclosure when safe
  to restore;
- provider freshness/degraded state and disabled-reason refs;
- visible placeholder state when the body is unavailable.

The shell does not record:

- transient hover state;
- unsent credential material or raw secret text;
- an unreviewed destructive action staged only in a private sidebar
  control;
- a modal/sheet decision that should instead be owned by an
  interaction-safety packet;
- raw paths, raw query text, raw URLs, or raw provider payloads when
  the owning contract requires refs, hashes, or redacted labels.

### 9.2 Reopen algorithm

`cmd:shell.sidebar.reopen_last` and selecting the active rail entry
use this order:

1. If `last_active_section_id` is present and admitted by policy,
   lifecycle, trust, and client scope, reopen that section with its
   last meaningful state.
2. If the section is admitted but its provider/body is unavailable,
   reopen the same section as `visible_placeholder` with typed
   disabled reason and repair routes.
3. If the last section is no longer admitted, open the first admitted
   primary section in rank order and show a short non-blocking
   explanation that the prior section is unavailable.
4. If no prior state exists, open `section.explorer`.
5. If Explorer cannot open because no workspace is available, keep the
   rail/command-palette return path visible and route to the
   workspace-entry surface owned by the main workspace.

Rules:

1. Reopen never lands in an empty shell with no rail, palette, global
   menu, status link, or visible explanation.
2. Compact desktop may present the sidebar as a sheet. It still uses
   `left_sidebar`, the same section id, and the same remembered state.
3. Layout reset may discard remembered section state only after
   preserving documents, review state, dirty buffers, and durable
   evidence through their owning contracts.
4. A provider reconnection may refresh labels and availability, but it
   may not reorder primary rail ranks or silently replace the last
   section.

## 10. Duplicate-surface prevention

Every new surface declares one owner:

- one `section_slot_id` when it is structural;
- one shell lane when it is not structural;
- one command descriptor when it launches an action;
- one source-of-truth record family when it displays durable data.

Non-conforming patterns:

- a second activity rail for a feature area;
- a second left sidebar for a provider, extension, or admin bundle;
- a top-level route that duplicates an existing section's durable
  list with different state;
- a rail entry that exists only to run a command;
- a sidebar action with no command descriptor;
- an inspector pane used as the primary route for core navigation;
- a bottom-panel tab used as a primary workspace browser;
- an overflow row promoted visually into a hidden eighth primary
  section;
- an extension surface that hard-codes shell widths, focus rules, or
  token meanings outside the shell-zone contract.

Conforming alternatives:

- add a subsection to the owning section;
- add a saved view, filter, or provider group to the existing
  collection;
- add a contextual link from another section to the owning section;
- add a command-palette category or global-menu command;
- add a bottom-panel tab for output;
- add an inspector pane for active-target explanation;
- add a sheet or review surface for consequence-bearing decisions;
- place the feature in rail overflow with explicit owner, command
  route, and no claim to primary rank.

## 11. Placement rules by section

### 11.1 Explorer

Explorer owns workspace structural browsing. File trees, roots,
workspace manifests, worksets, generated-artifact lineage entry
points, and root-scoped durable lists belong here when the user's
primary task is to browse workspace structure.

Explorer does not privately own destructive file operations. Delete,
move, rename, import, generated-file cleanup, trust changes, and
large-scope mutations require command-backed routes and preview or
review when consequence classes require it.

### 11.2 Search

Search owns query-backed discovery, result collections, saved queries,
scope/freshness disclosure, and result navigation. It may show result
counts, hidden-scope notes, readiness state, and fallback truth.

Search does not own the only route to actions on results. Open,
reveal, copy, replace, export, batch select, and provider-backed
actions resolve through command descriptors, context menus, or review
surfaces.

### 11.3 Source Control

Source Control owns changes, branches, worktrees, stashes, hosted
review rows, provider state, and review adjacency. It may link to
diffs and review surfaces in the main workspace.

Source Control does not privately own publish, push, history edit,
merge, rebase, branch deletion, or hosted-provider mutation. Those
actions require command descriptors and the review/approval posture
defined by their consequence class.

### 11.4 Run/Test

Run/Test owns executable target browsing: run configurations, test
trees, debug targets, task lists, session summaries, and links into
execution output.

Terminal output, logs, debug console, and test output belong in the
bottom panel. Debug views, failure review, coverage dashboards, and
long evidence comparisons may claim the main workspace. Start/stop,
rerun, attach, debug, and remote execution actions resolve through
command descriptors and execution-context truth.

### 11.5 Extensions

Extensions owns installed-extension inventory, marketplace or mirror
rows, activation status, permissions, compatibility, quarantine, and
extension-host diagnostics.

Extension install, update, disable, trust, permission-grant,
publisher handoff, and rollback actions require command descriptors
and review/approval when their consequence class requires it.
Extension-provided views attach to an existing section, inspector,
bottom panel, main workspace, or overflow row; they do not add a new
primary rail rank by default.

### 11.6 Collaboration

Collaboration owns live sessions, participants, invites, presence,
follow/presenter state, shared-control grants, recording/retention
summaries, and degraded shared-state truth.

Join, invite, grant/revoke control, recording changes, shared debug,
route share, and archive/export actions require command descriptors
and consent/approval paths. Local editing and non-shared routes remain
available when collaboration degrades.

### 11.7 Support/Admin

Support/Admin owns Project Doctor, support bundle rows, repair
previews, service health, policy/admin queues, diagnostic summaries,
incident handoff, and managed-profile admin surfaces where the current
client scope admits them.

Repair apply, support export, policy authoring, admin waivers,
entitlement refresh, upload, publication, or managed-control actions
require command descriptors and the appropriate sheet, modal, review,
or main-workspace surface. Support/Admin may be hidden when not
applicable, but hidden state must not remove the command-backed route
to diagnostics or support export when those routes are available.

### 11.8 Future and extension rows

Future rows start as `section.future_overflow` and `overflow_only`.
They must include:

- owning team or extension identity;
- structural need statement;
- why an existing section/subsection is insufficient;
- command-backed direct route;
- duplicate-surface analysis;
- compact and keyboard behavior;
- degradation and disabled-reason handling;
- promotion plan if primary placement is requested.

Without those fields, the placement verdict is
`denied_duplicate_surface` or `contextual_link_only`.

## 12. Reviewer checklist

A new surface is conforming only when all answers are yes:

1. Does it declare exactly one owning section or shell lane?
2. If it is structural, does it attach to one of the seven section ids
   or to `section.future_overflow`?
3. If it wants primary rail placement, does the seven-section budget
   still hold without visual replacement tricks?
4. If it is only a link, status cue, or explanation, is it a
   contextual link, status-bar link, inspector pane, or command route
   rather than a new section?
5. Does every critical action cite a command descriptor and at least
   one independent keyboard-first route?
6. Does hide/reopen preserve the last meaningful state or show an
   understandable placeholder?
7. Can a keyboard-only user reopen the section after hiding it?
8. Are counts and selections using collection/selection vocabulary
   rather than local synonyms?
9. Is provider, policy, lifecycle, trust, or client-scope narrowing
   visible through typed disabled reasons?
10. Is duplicate durable state avoided across rail, sidebar,
   inspector, bottom panel, and main workspace?

## 13. Schema and fixture contract

The schema exports four record kinds:

- `section_slot_catalog_record` - the canonical taxonomy, budget,
  overflow, and command set.
- `section_slot_record` - one section's rail/sidebar placement,
  content classes, commands, and duplicate-surface guardrails.
- `sidebar_visibility_state_record` - hidden, remembered, compact
  sheet, unavailable placeholder, and reopen-last-section state.
- `section_surface_placement_decision_record` - a reviewer-facing
  placement decision for a new surface or critical action.

The fixtures cover:

- canonical top-level taxonomy and budget;
- Explorer as the default structural section;
- Run/Test as a split sidebar/bottom-panel/main-workspace section;
- hiding Search and reopening its last state;
- a future extension row denied primary placement and routed to
  overflow;
- a Source Control critical action requiring command-backed parity.

## 14. Reuse guarantee

A downstream surface may change visual treatment, row density,
provider grouping, or local copy. It may not change:

- section ids;
- primary rail ranks;
- budget rules;
- rail/sidebar/contextual ownership;
- command-backed hide/show/direct-jump requirements;
- reopen-last-section state semantics;
- duplicate-surface prevention rules;
- count, selection, consequence, preview, or approval vocabulary.

