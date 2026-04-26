# Command-graph, UI-slot, and slot-token seed

This document closes the remaining gap between the frozen
command-descriptor contract and the frozen design-token export
contract.

`docs/commands/command_descriptor_contract.md` already owns the
stable command object: `command_id`, arguments, capability scope,
preview and approval posture, lifecycle metadata, discoverability
fields, result contract, and invocation-session packet.

`docs/design/design_token_component_state_vocabulary.md` already
owns the visual vocabulary: token families, component states,
semantic status, trust visuals, layer order, scrims, themes, and
motion posture.

What neither contract owned yet was the translation layer between:

- "this descriptor may surface in a toolbar or status bar", and
- "which concrete shell slots are allowed to render it, with which
  issuing-surface class and which token/state families."

This seed freezes that translation before the first real shell
surfaces land.

Companion artifacts:

- [`/schemas/commands/ui_slot_taxonomy.schema.json`](../../schemas/commands/ui_slot_taxonomy.schema.json)
  — machine-readable taxonomy and projection-policy boundary.
- [`/fixtures/commands/ui_slot_taxonomy_examples/`](../../fixtures/commands/ui_slot_taxonomy_examples/)
  — one taxonomy seed record and one cross-surface command
  projection example.
- [`/docs/commands/command_parity_diff.md`](./command_parity_diff.md)
  and
  [`/artifacts/commands/command_parity_seed.yaml`](../../artifacts/commands/command_parity_seed.yaml)
  — the reusable diff format and synthetic cross-surface seed
  corpus that compare palette, menu/button, keybinding-help,
  CLI/help, and AI-tool claims against the frozen command object.
- [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json),
  [`/fixtures/commands/seed_commands/`](../../fixtures/commands/seed_commands/),
  and
  [`/artifacts/commands/command_registry_seed.yaml`](../../artifacts/commands/command_registry_seed.yaml)
  — the seeded canonical command-registry object that keeps aliases,
  discoverability projections, current-shortcut display refs,
  disabled-state explainers, diagnostics, and machine-facing names
  attached to the canonical command id instead of reauthoring them per
  surface.
- [`/docs/commands/palette_row_contract.md`](./palette_row_contract.md),
  [`/schemas/commands/palette_result.schema.json`](../../schemas/commands/palette_result.schema.json),
  [`/schemas/commands/palette_action_footer.schema.json`](../../schemas/commands/palette_action_footer.schema.json),
  and
  [`/fixtures/commands/palette_rows/`](../../fixtures/commands/palette_rows/)
  — the governed command-palette row and action-footer projection
  contract that renders command rows in palette, docs/help, settings,
  keybinding help, migration guidance, automation explainers, and
  support export from the same registry/shareability records.
- [`/docs/commands/sequence_and_modal_discoverability_contract.md`](./sequence_and_modal_discoverability_contract.md),
  [`/schemas/commands/leader_overlay.schema.json`](../../schemas/commands/leader_overlay.schema.json),
  and
  [`/fixtures/commands/sequence_help_examples/`](../../fixtures/commands/sequence_help_examples/)
  — the governed modal / sequence discoverability projection for mode
  strips, leader overlays, sequence-help rows, shortcut teaching, and
  colon-style command parity over the same command graph.
- [`/docs/commands/command_descriptor_contract.md`](./command_descriptor_contract.md)
  — the command object this seed projects from.
- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  — the token/state vocabulary this seed projects into.

## Why this exists

Without a shared slot taxonomy:

- title/context-bar buttons, sidebar actions, editor-chrome
  affordances, inspector actions, status items, onboarding tips,
  and companion handoff affordances would each need to guess which
  descriptor `ui_slot_class` they count as;
- one surface could ship a command as a menu item while another
  silently ships the same command as an untracked title-bar button
  or companion-only launcher;
- shell authors would need to invent local token aliases per slot
  family, recreating the drift the design-token export was created
  to prevent.

This seed makes the translation explicit:

1. the command descriptor remains the canonical command object;
2. `ui_slot_hints` remain the coarse discoverability vocabulary;
3. this document and the schema translate those coarse hints into
   stable slot families, stable slot keys, direct-projection
   rules, mirror/handoff rules, and slot-token publication rules;
4. launch-bearing chrome, docs/help renderers, onboarding
   surfaces, and companion handoff surfaces therefore reuse one
   slot vocabulary instead of inventing local names.

## Scope

Frozen here:

- one command-graph relation model tying `command_descriptor_record`
  and `invocation_session_packet_record` to discoverability,
  issuing surfaces, slot families, and concrete slot keys;
- one stable UI-slot taxonomy for shell chrome, menus, palette,
  help surfaces, onboarding affordances, and companion handoff
  surfaces;
- one slot-token publication strategy that maps slot families to
  the design-token export's token and state families without
  re-publishing raw token values; and
- one example projection corpus showing a command represented
  consistently in palette, menu, keybinding-help, and CLI-help
  contexts.

Out of scope:

- live shell layout implementation;
- a finished palette router or keybinding resolver;
- per-feature control design inside a published slot key;
- adding new `ui_slot_class` or `issuing_surface` enum values to
  the command-descriptor contract; and
- re-exporting raw token values. This seed owns slot-to-family
  mapping, not the token values themselves.

## Command-graph relation model

| Graph object | Canonical owner | Owns | Must not own |
|---|---|---|---|
| `command_descriptor_record` | command registry | stable `command_id`, arguments, capability scope, preview/approval posture, lifecycle metadata, result contract | concrete shell slot placement |
| Discoverability fields on the descriptor | same descriptor | labels, aliases, docs/help anchor, palette visibility, coarse `ui_slot_hints` | per-surface copies of labels, help anchors, or shortcut wording |
| `ui_slot_taxonomy_record` | slot taxonomy registry | stable slot families, stable slot keys, direct-projection rules, mirror/handoff rules, slot-token publication rules | command capability or result semantics |
| `invocation_session_packet_record` | issuing surface + command runtime | actual `issuing_surface`, authority, enablement decision, execution intent, outcome, evidence refs | invented command ids or invented slot semantics |
| `design_token_export_manifest_record` | design-system registry | token families, component states, semantic status, trust visuals, layer order, scrims, themes, motion posture | shell-slot ownership or command placement |

Rules:

- every launch-bearing surface starts from `command_id` plus
  `command_revision_ref`; no shell surface may bind a widget-local
  callback or local-only verb;
- a descriptor may request surfacing only through its frozen
  `ui_slot_hints`; the taxonomy may narrow or translate those
  hints into stable slot keys, but it may never widen them into a
  new direct command path;
- the slot taxonomy owns concrete slot names; a surface may not
  invent `top_nav_button`, `sidebar_cta`, `footer_action`, or
  similar private aliases when a published slot key already
  exists;
- `issuing_surface` always comes from the command-descriptor
  contract; if a location does not have a dedicated issuing value
  yet, it must route through an existing value or remain
  mirror/handoff-only; and
- onboarding and companion surfaces are mirror/handoff families in
  this seed. They may quote a command and deep-link to a canonical
  route, but they may not mint a new invocation-session
  `issuing_surface` value.

## Direct projection vs mirror/handoff

Two projection modes exist at this seed:

| Mode | Meaning |
|---|---|
| **Direct projection** | the slot launches the descriptor directly and the invocation packet uses an existing `issuing_surface` value such as `command_palette`, `global_application_menu`, `context_menu`, `toolbar_button`, or `ai_tool_surface` |
| **Mirror / handoff projection** | the slot quotes the same command metadata but transfers control to a direct surface such as the palette, a toolbar button, or a desktop handoff because no dedicated issuing-surface value exists yet |

A slot family marked `handoff_bearing`,
`narrates_existing_route`, or `documents_existing_route` is not
allowed to invent a new direct execution path.

## Slot families and stable slot keys

The seed families are:

- `title_context_bar` — `title_context.context_actions`
- `activity_rail` — `rail.primary_route_entry`,
  `rail.handoff_entry`
- `sidebar` — `sidebar.section_action`,
  `sidebar.selection_action`
- `editor_chrome` — `editor_chrome.breadcrumb_action`,
  `editor_chrome.inline_action`, `editor_chrome.gutter_action`
- `bottom_panel` — `bottom_panel.panel_tab_action`,
  `bottom_panel.panel_toolbar_action`
- `inspector` — `inspector.header_action`,
  `inspector.footer_action`
- `status_bar` — `status_bar.primary_status_item`,
  `status_bar.quick_action`
- `global_menu` — `global_menu.command_item`
- `context_menu` — `context_menu.editor_item`,
  `context_menu.explorer_item`, `context_menu.review_item`,
  `context_menu.source_control_item`,
  `context_menu.terminal_item`, `context_menu.search_item`
- `command_palette` — `command_palette.result_row`,
  `command_palette.inline_action`
- `keybinding_help` — `keybinding_help.command_row`
- `cli_help` — `cli_help.command_row`
- `onboarding_affordance` — `onboarding.tip_action`,
  `onboarding.guided_step_action`
- `companion_surface` — `companion_surface.primary_action`,
  `companion_surface.desktop_handoff`
- `ai_tool_surface` — `ai_tool_surface.tool_entry`

Family notes:

- `activity_rail` is a mirror/handoff family in this seed. It may
  focus a sidebar section or seed the command palette, but it does
  not own a new direct command path.
- `sidebar`, `editor_chrome`, `bottom_panel`, and `inspector`
  only own action slots. Durable lists, trees, tables, editors,
  logs, and viewers rendered inside those regions remain owned by
  their own surface contracts.
- `onboarding_affordance` and `companion_surface` never widen
  authority. They quote the same descriptor and help path as the
  canonical surface they hand off to.
- `cli_help` is included because docs/help and export consumers
  need stable slot names too, even though the eventual terminal
  renderer is not a themed shell slot.

## Descriptor-slot mapping rules

Direct projection rules from the command descriptor contract into
the slot taxonomy are:

| Descriptor `ui_slot_class` | Allowed slot keys | Invocation posture |
|---|---|---|
| `command_palette` | `command_palette.result_row`, `command_palette.inline_action` | direct; `issuing_surface = command_palette` |
| `global_application_menu` | `global_menu.command_item` | direct; `issuing_surface = global_application_menu`; `menu_path_refs` required |
| `editor_context_menu` | `context_menu.editor_item` | direct; `issuing_surface = context_menu`; `menu_path_refs` and contextual filter required |
| `explorer_context_menu` | `context_menu.explorer_item` | direct; `issuing_surface = context_menu`; `menu_path_refs` and contextual filter required |
| `review_context_menu` | `context_menu.review_item` | direct; `issuing_surface = context_menu`; `menu_path_refs` and contextual filter required |
| `source_control_context_menu` | `context_menu.source_control_item` | direct; `issuing_surface = context_menu`; `menu_path_refs` and contextual filter required |
| `terminal_context_menu` | `context_menu.terminal_item` | direct; `issuing_surface = context_menu`; `menu_path_refs` and contextual filter required |
| `search_context_menu` | `context_menu.search_item` | direct; `issuing_surface = context_menu`; `menu_path_refs` and contextual filter required |
| `primary_toolbar` | `title_context.context_actions`, `editor_chrome.inline_action`, `bottom_panel.panel_toolbar_action` | direct; `issuing_surface = toolbar_button` |
| `secondary_toolbar` | `sidebar.section_action`, `sidebar.selection_action`, `editor_chrome.gutter_action`, `inspector.header_action`, `bottom_panel.panel_tab_action` | direct; `issuing_surface = toolbar_button` |
| `status_bar_control` | `status_bar.primary_status_item`, `status_bar.quick_action` | direct; `issuing_surface = toolbar_button` |
| `keybinding_help` | `keybinding_help.command_row` | narrates an existing key route; the help row itself does not mint a new `issuing_surface` |
| `cli_help` | `cli_help.command_row` | documents an existing CLI route; the help row itself does not mint a new `issuing_surface` |
| `ai_tool_surface` | `ai_tool_surface.tool_entry` | direct; `issuing_surface = ai_tool_surface` |

Derived-family rules:

- `activity_rail` may mirror commands already published to
  `command_palette` or `primary_toolbar`; it must focus the owning
  route or palette category instead of inventing a rail-specific
  command path.
- `onboarding_affordance` may mirror commands already published to
  `command_palette`, `global_application_menu`,
  `primary_toolbar`, `secondary_toolbar`, `status_bar_control`,
  `keybinding_help`, or `cli_help`; it must hand off to one of
  those canonical routes.
- `companion_surface` may mirror commands already published to
  `command_palette`, `primary_toolbar`, `secondary_toolbar`,
  `status_bar_control`, or `cli_help`; until the command contract
  grows a dedicated companion issuing-surface value, companion
  slots are handoff-only.

These rules are what prevent a launch-bearing surface from
inventing an untracked command path: every direct projection must
resolve through an existing descriptor `ui_slot_class`, an
existing `issuing_surface`, and one of the published slot keys
above.

## Slot-token publication strategy

The slot taxonomy does not publish raw token values. It publishes
which token families and state families are admissible for each
slot family so shell surfaces, docs/help renderers, onboarding,
and companion consumers all read the same semantic contract.

Publication rules:

1. A slot family resolves the design-token export manifest by
   `manifest_id` and then consumes only the families and state
   families admissible for that slot family.
2. Slot chrome consumes semantic families. Raw primitives,
   screenshot-local colors, and feature-local aliases are never
   admissible at the slot boundary.
3. Domain families such as `color_syntax`, `color_diff`, and
   `color_chart` belong to embedded content surfaces, not to shell
   slot chrome, unless a later published slot-family row admits
   them explicitly.
4. Overlay families (`command_palette`, `global_menu`,
   `context_menu`, `onboarding_affordance`) must also quote
   layer-order and scrim families from the design-token export.
5. `cli_help` consumes the same status/trust vocabulary for docs
   and export parity, but it does not imply a graphical shell
   renderer.
6. `companion_surface` consumes a semantic mirror subset; it may
   narrow motion, elevation, or overlay richness, but it may not
   rename token families or state meanings.

Grouped publication posture:

| Slot-family group | Token posture | State families |
|---|---|---|
| `title_context_bar`, `activity_rail`, `sidebar`, `editor_chrome`, `bottom_panel`, `inspector`, `status_bar`, `global_menu`, `context_menu`, `command_palette`, `keybinding_help`, `onboarding_affordance`, `ai_tool_surface` | semantic families plus optional component-token indirection | `component_state`, `semantic_status`, `trust_visual_state`, `accessibility_posture` |
| `cli_help` | semantic/status/trust text vocabulary only | `component_state`, `semantic_status`, `trust_visual_state` |
| `companion_surface` | semantic mirror subset only | `component_state`, `semantic_status`, `trust_visual_state`, `accessibility_posture` |

The schema and fixtures carry the per-family allow-lists; the
table above is the human summary.

## Example projection corpus

The projection example in
[`/fixtures/commands/ui_slot_taxonomy_examples/workspace_open_folder_surface_projection.json`](../../fixtures/commands/ui_slot_taxonomy_examples/workspace_open_folder_surface_projection.json)
shows one command represented consistently in:

- command palette,
- global application menu,
- keybinding help, and
- CLI help.

The example intentionally reuses the same:

- `command_id`,
- `command_revision_ref`,
- `primary_label_ref`, and
- `docs_help_anchor_ref`

already frozen in
[`/fixtures/commands/command_descriptor_examples/workspace_open_folder.json`](../../fixtures/commands/command_descriptor_examples/workspace_open_folder.json).

That is the core parity rule: a surface may project the command
differently, but it may not mint a different label path, help
anchor, or command identity.

## Change discipline

- Adding a new slot family, slot key, or projection rule is
  additive-minor.
- Repurposing an existing slot key is breaking.
- Adding a new direct command path requires the command-descriptor
  contract and this taxonomy to change together.
- A new shell, onboarding, or companion surface must extend this
  taxonomy in the same change that introduces the new surface; no
  unclassified launch-bearing surface is allowed.

## Reuse guarantee

This seed is reusable by shell chrome, docs/help, onboarding,
design-system consumers, and future companion surfaces without
inventing local command placement rules. A conforming consumer
must:

1. resolve command identity from the command descriptor contract;
2. resolve concrete placement from the slot taxonomy, not from
   surface-local heuristics;
3. resolve token and state allow-lists from the design-token
   export, not from raw color or spacing aliases; and
4. refuse to widen a command into a direct slot or issuing
   surface the published taxonomy does not already allow.
