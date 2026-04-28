# Menu, Context-Menu, Command-Bar, and Inline-Toolbar Contract

This document freezes the projection rules for pointer-invoked action
surfaces: application menus, context menus, inline toolbars, command
bars, and their overflow menus. These surfaces are compact views onto
the command graph. They do not own command names, command IDs,
enablement rules, stale-target policy, shortcut wording, side-effect
semantics, or disabled-reason copy.

Machine-readable companions:

- [`/schemas/commands/menu_item.schema.json`](../../schemas/commands/menu_item.schema.json)
  defines `menu_case_record` and `menu_item_record` projections.
- [`/fixtures/commands/menu_cases/`](../../fixtures/commands/menu_cases/)
  contains parity, stale-target, destructive-group, toggle/choice, and
  provider-backed examples.

The command descriptor remains the canonical product object. Menu and
command-bar records are materialized projections used by shell chrome,
docs/help, support export, command diagnostics, and parity audits. If a
menu row disagrees with the command descriptor, command registry,
diagnostic row, keybinding resolver, or palette row, the menu row is
wrong.

## Scope

Frozen here:

- item classes for command, toggle, choice, destructive, and dynamic
  provider-backed rows;
- the row anatomy every menu, context menu, inline toolbar, command bar,
  and overflow menu can render without inventing a second interaction
  language;
- target-freshness and invalidation rules for context menus and inline
  toolbars;
- parity rules tying every launch-bearing row back to one command ID,
  one canonical label ref, one shortcut hint, one side-effect cue, and
  one disabled-reason model;
- destructive grouping, nested-depth limits, and provider source/stale
  disclosure; and
- fixture expectations for the seed corpus.

Out of scope:

- native OS menu integration details;
- final visual styling, animation, and platform accelerator notation;
- the live command router; and
- provider-specific command bodies. Provider-backed menu rows carry
  opaque provider refs and route through command descriptors.

## Canonical Source Chain

| Source | Owns | Menu / command-bar projection may carry |
|---|---|---|
| `command_descriptor_record` | stable command ID, canonical verb, capability scope, preview/approval posture, docs/help anchor, shortcut narration hint, coarse UI slot hints | command identity, command posture, shortcut narration ref, docs/help ref |
| `command_registry_entry_record` | user label, summary, origin/target badges, dominant side-effect class, automation labels, disabled-reason records | canonical label, subtitle/detail, side-effect cue, badge/source refs |
| keybinding resolver output | winning shortcut, shortcut state, source layer, platform conflicts | current shortcut hint or explicit unassigned/blocked state |
| enablement / diagnostics projection | enabled/disabled/hidden decision, typed disabled reason, repair hook, target/origin badge, policy source | disabled reason chip, why-unavailable route, protected-entry badge refs |
| `menu_item_record` | surface placement, item class, grouping, nesting, target snapshot, freshness state | compact row anatomy and surface-local placement only |

The projection may materialize user-readable text for rendering, but it
must also carry source refs proving where the text came from.

## Surface Families

| Surface family | Role | Direct invocation posture |
|---|---|---|
| `global_application_menu` | Stable application-level menu tree and host-rendered menu equivalents. | Direct; `issuing_surface = global_application_menu`. |
| `context_menu` | Target-scoped transient menu opened by pointer, keyboard menu key, or platform equivalent. | Direct; `issuing_surface = context_menu`; must carry target snapshot. |
| `command_bar` | Horizontal or compact command strip for high-frequency local actions. | Direct; `issuing_surface = toolbar_button`; may show only a subset. |
| `inline_toolbar` | Target-adjacent action strip for selection, editor, review, diff, or generated-artifact actions. | Direct; `issuing_surface = toolbar_button`; must carry target snapshot. |
| `overflow_menu` | Compact overflow for command bars, status items, breadcrumbs, tabs, and other constrained chrome. | Direct only when the source slot is direct; otherwise mirrors the canonical route. |

Keyboard invocation (`Shift+F10`, a platform menu key, command-bar focus
traversal, or equivalent) must reach the same command IDs and disabled
reasons as pointer invocation.

## Item Classes

| Item class | Required presentation | Contract rule |
|---|---|---|
| `command` | Canonical label, optional icon, shortcut hint, side-effect cue when meaningful, optional secondary scope. | Maps to exactly one `command_id` and one descriptor revision. |
| `toggle` | Canonical label, checkmark or state glyph, current value label when ambiguity exists, shortcut hint if present. | Must declare whether activation is immediate safe, staged, object-scoped, or preview-required. |
| `choice` | Canonical label plus one selected option visible within the group, or a submenu/overflow containing mutually exclusive options. | Exactly one option in the choice set may be current unless the state is explicitly `mixed`. |
| `destructive` | Canonical label, stronger caution cue, separated destructive group, preview/approval cue where required. | Cannot sit adjacent to safe navigation items without grouping; never bypasses preview/approval. |
| `dynamic_provider_backed` | Canonical command label or provider action label, provider/source hint, limited/stale state where applicable, shortcut only if declared by command metadata. | Provider action refs are opaque inputs to a command; the row still routes through one command descriptor and disabled-reason model. |

All classes carry the same identity block:

- `command_id`;
- `command_revision_ref`;
- `canonical_verb`;
- `label_ref`;
- `docs_help_anchor_ref`; and
- `enablement_decision` plus `disabled_reason_code` when unavailable.

## Row Anatomy

A menu row has these ordered zones. A command bar may compress them, but
it may not drop mandatory semantics; compressed rows expose the same
information through focus narration, why-unavailable details, or the
overflow row.

| Zone | Contents | Rule |
|---|---|---|
| Leading affordance | Optional icon, checkmark, radio mark, provider badge, or destructive cue. | State indicators must not be color-only. |
| Primary label | Materialized canonical label plus `label_ref`. | Surface-local renames for the same action are non-conforming. |
| Secondary detail | Scope, target summary, provider source, current value, stale/limited note, or destructive scope summary. | Required when the row would otherwise hide target or source ambiguity. |
| Shortcut hint | Current shortcut display state and narration ref. | Blank is not a synonym for unassigned, unsupported, shadowed, or policy-blocked. |
| Side-effect / current-state cue | Dominant side effect, preview/approval cue, toggle state, choice current value, or provider freshness. | High-risk rows must show this inline or in one-step details. |
| Disabled reason | Compact disabled reason or why-unavailable affordance. | Required when discoverability matters or the row is the only obvious route. |
| Submenu affordance | Chevron or equivalent plus `submenu_depth`. | Depth limits below apply; deep command trees must become search/sheet/palette routes. |

Tooltip-only disclosure is insufficient for shortcut state, destructive
posture, provider-limited state, stale target, or why unavailable.

## Target Freshness

Context menus and inline toolbars are opened against a target snapshot.
The snapshot is a typed boundary, not a cached pointer to mutable UI
state. A projection records:

- focused entity ref;
- selection ref or selected object refs;
- target count and scope summary ref;
- buffer/tree/review/provider revision refs where applicable;
- policy epoch and trust state;
- execution context id; and
- basis snapshot ref used by any preview or invocation session.

Material target changes include:

- focused object identity changed;
- selection identity, range, or selected count changed;
- buffer, file, tree node, review thread, notebook cell, or generated
  artifact revision changed;
- provider result set, provider freshness, or provider capability state
  changed;
- policy epoch, trust state, workspace mode, or execution context
  changed;
- descriptor revision changed; or
- destructive scope, target count, preview class, or approval posture
  changed.

When a material change occurs while the surface is open:

| Affected row | Required outcome |
|---|---|
| Unaffected read-only row | May remain enabled after revalidating the enablement snapshot. |
| Safe local command whose target changed but can be refreshed exactly | Refresh target summary and enablement in place; announce the refresh for keyboard/screen-reader users. |
| Mutating command, destructive row, externally visible row, provider-backed row, or preview-required row | Disable the stale row with `basis_snapshot_drifted`, `freshness_floor_unmet`, or the typed provider reason; require refresh, close/reopen, or re-preview before apply. |
| Row whose descriptor, policy, trust, or execution context changed | Re-evaluate from the descriptor and diagnostics projection; do not preserve stale enabled state. |

Activation always revalidates `basis_snapshot_ref`. If the snapshot no
longer matches the target and the item is not explicitly revalidated, the
invocation is denied with `basis_snapshot_drifted`; it must not apply
against stale context.

## Parity Rules

1. Every launch-bearing row starts from `command_id` plus
   `command_revision_ref`; widget-local callbacks or menu-only verbs are
   non-conforming.
2. The same high-frequency action uses one canonical label ref across
   palette, global menu, context menu, inline toolbar, command bar,
   keybinding help, CLI/help where applicable, and docs/help.
3. Shortcut hints come from resolver output or descriptor narration
   refs. Menus may format accelerators differently per platform, but
   they may not change the command identity or narration meaning.
4. Disabled rows use the command descriptor and diagnostics disabled
   reason vocabulary. Local labels such as "not allowed" or "not here"
   must resolve to a typed `disabled_reason_code` and `repair_hook_ref`
   where a repair route exists.
5. Side-effect and preview/approval cues come from command metadata. A
   command bar button cannot hide destructive, external, credential, or
   policy-authoring posture because it is visually compact.
6. A surface may narrow availability for context, policy, lifecycle, or
   client-scope reasons, but it must carry the typed narrowing reason.
7. Command bars and inline toolbars may pin a subset for speed, but they
   may not become the only route to a high-frequency or critical command.
   The same command must remain discoverable from the palette, menu,
   docs/help, keybinding help, or a visible overflow route.
8. Overflow menus inherit the command-bar rows they overflow. Moving a
   row into overflow must preserve label, command ID, disabled reason,
   shortcut state, side-effect cue, and provider freshness.

## Dynamic Provider-Backed Items

Provider-backed rows include code actions, refactors, external review
actions, package actions, generated-artifact actions, or extension
contributions whose options are not fully known until a provider
responds. They must still route through a command descriptor such as an
apply-provider-action command.

Provider-backed rows must disclose:

- provider ref and provider kind;
- source label ref or provider badge;
- provider freshness state (`ready`, `partial`, `stale`, `limited`,
  `policy_blocked`, or `unavailable`);
- result limit or truncation when the provider returned only a subset;
- stale reason or provider-disabled reason when applicable;
- opaque provider action ref; and
- the command ID that will apply or inspect the action.

If provider state is stale, partial, limited, unavailable, or blocked by
policy, the row must not look identical to a local core command. It may
remain discoverable, but activation must revalidate provider state and
deny with the typed command disabled reason if the provider cannot
produce a fresh action.

## Destructive Grouping

Destructive rows include destructive local cleanup, irreversible publish,
externally mutating actions, destructive bulk mutations, policy-authoring
actions, managed-workspace controls, and any command whose descriptor
requires destructive preview or explicit approval.

Rules:

- destructive rows belong in a `destructive` group separated from safe
  navigation and ordinary edit actions;
- a destructive group may contain explanatory disabled rows, but safe
  navigation rows must not be placed inside it;
- destructive rows quote preview and approval posture from the
  descriptor and never claim direct safe apply when preview/approval is
  required;
- destructive context-menu rows must show affected scope or target count
  before activation when the scope is broader than the visible target;
- bulk destructive actions must not be reachable only from a context menu
  or inline toolbar; they need a palette, menu, review, sheet, or command
  diagnostics route as appropriate; and
- unavailable destructive rows that matter for discoverability remain
  visible with a disabled reason rather than disappearing into ambiguity.

## Nesting And Overflow

Menus stay shallow so they remain compact command projections:

- application menus may use at most two submenu levels below the top
  menu before switching to a searchable sheet, command palette result,
  or dedicated panel;
- context menus may use at most one submenu level;
- inline toolbars and command bars do not own nested submenus directly;
  they use an overflow menu with the same row contract;
- provider-backed result sets that exceed the visible budget use a
  "more from provider" command, sheet, or palette route rather than deep
  cascading submenus; and
- nesting is justified only by stable mutually exclusive choice groups,
  provider source grouping, or platform menu conventions. It is not a
  dumping ground for low-confidence commands.

The schema records `submenu_depth` and `max_depth_policy` so parity
audits can flag over-nested surfaces before implementation.

## Disabled Disclosure

Disabled or hidden-with-reason rows must answer:

- which command is unavailable;
- which boundary owns the block: trust, policy, lifecycle, dependency,
  client scope, context, provider, target freshness, or platform;
- which typed disabled reason applies;
- whether a repair hook, refresh, re-preview, docs/help, or command
  diagnostics route exists; and
- whether the command remains reachable through another surface.

Rows may be hidden in novice or compact contexts only when another
discoverability route can explain the reason. A context menu or inline
toolbar must not hide the only route to a critical command.

## Keyboard And Accessibility

- The menu keyboard route and pointer route produce the same rows after
  target filtering.
- Focus returns to the invoking surface or a safe ancestor when the
  surface closes or invalidates.
- Disabled rows that remain visible are focusable when they expose
  why-unavailable details.
- Screen-reader narration includes label, role, current state, shortcut
  state, provider/stale state, destructive posture, and disabled reason
  where present.
- Inline toolbars must not trap focus. Escape closes transient chrome and
  returns to the target if the target is still valid.

## Fixture Expectations

Fixture records under
[`/fixtures/commands/menu_cases/`](../../fixtures/commands/menu_cases/)
exercise:

- an application-menu command item that preserves command ID, label,
  shortcut, docs/help anchor, and palette/toolbar parity;
- a context-menu item invalidated by target snapshot drift;
- command-bar toggle and choice rows carrying current state without
  changing command identity;
- a destructive row separated from safe source-control actions with
  preview/approval cues; and
- a dynamic provider-backed row that discloses provider source and
  partial/stale state.

The corpus is intentionally small. It proves shape, parity, and
freshness rules; it does not assert final ranking, visual density, or
native OS menu integration.

## Source Anchors

- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  §7.11 and §16.16 — command parity, menu item grammar, target
  freshness, and toolbar parity.
- [`.t2/docs/Aureline_UX_Design_System_Style_Guide.md`](../../.t2/docs/Aureline_UX_Design_System_Style_Guide.md)
  §16.16 and §17 — menu anatomy and command palette parity.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md)
  §11.9 and Appendix BZ — one command graph and surface parity.
- [`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  — command identity, preview/approval, disabled-reason vocabulary, and
  invocation-session packet.
- [`docs/commands/command_graph_and_ui_slots_seed.md`](../commands/command_graph_and_ui_slots_seed.md)
  — slot-family and issuing-surface mapping.
- [`docs/commands/palette_row_contract.md`](../commands/palette_row_contract.md)
  — palette row parity, current shortcut display, and action footer
  projection.
- [`docs/ux/command_diagnostics_contract.md`](./command_diagnostics_contract.md)
  — disabled-reason, remediation, target/origin badge, and
  why-unavailable projection.
