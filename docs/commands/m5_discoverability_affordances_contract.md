# M5 menu-affordance, keybinding-resolver, and command-documentation matrix contract

Status: **frozen** (M05-740, batch B86 / wave W86)

This contract freezes the canonical matrix for Aureline's last-mile M5
command-discovery surfaces. It closes the gap between "the command exists" and
"every discoverability / help surface explains the *same* command truth."

## Why this lane exists

The command model, automation labels, search/preview, modal-keyboard, design
system, shell-zone, and lifecycle vocabularies are already frozen by earlier M5
rows. This matrix does **not** redefine the command model. It hardens the
discoverability and explanation surfaces *around* already-claimed M5 commands:
menus, context menus, command bars, keybinding inspectors, conflict sheets,
keymap import bridges, disabled-command explainers, leader/sequence overlays,
and command-documentation surfaces.

## Track invariant

Menus, context menus, command bars, keybinding inspectors, leader/sequence help,
and command-documentation surfaces all project from **one canonical command
record**. No surface:

- invents a second naming system (alternate label) for a stable command,
- widens authority beyond the canonical command,
- hides a disabled-state reason,
- drops preview / approval / lifecycle truth.

Keyboard, screen-reader, pointer, touch, CLI/help, and support/export paths can
all explain the same command semantics.

## Canonical command record reuse

Every surface row binds to the frozen M5 command descriptor
(`schemas/commands/command_descriptor.schema.json`), reusing the descriptor's
`lifecycle_label`, `preview_class`, `disabled_reason_mode`, `feature_family`,
and `discovery_channel` vocabulary verbatim (re-exported from
`crate::m5_command_registry`). This matrix mints **no** parallel command
vocabulary — only the vocabulary for the discoverability surfaces themselves.

## Governed command-surface families (10)

| Family | Purpose |
| --- | --- |
| `menu_item` | A single application/menu-bar item projecting one canonical command. |
| `menu_group` | A named menu section / submenu grouping related commands. |
| `context_menu` | A right-click / long-press menu for the focused object. |
| `command_bar` | A contextual action bar for the active surface. |
| `keybinding_resolver_layer` | The resolver inspector naming the winning source layer and shadowed losers. |
| `conflict_review_sheet` | The conflict sheet naming each collision with a controlled reason. |
| `import_bridge_row` | One foreign-keymap binding translated to a native command. |
| `disabled_command_explainer` | The why-unavailable packet for a greyed-out command. |
| `leader_sequence_help` | The leader / multi-key sequence help overlay. |
| `command_documentation_surface` | The command-detail / documentation surface. |

## Frozen vocabularies

These closed sets are the single source of truth so support, docs, and export
paths reuse them verbatim:

- **Shortcut-source classes** (precedence, lowest first): `platform_default`,
  `default_keymap`, `imported_keymap`, `extension_keybinding`,
  `workspace_keybinding`, `user_keybinding`, `leader_sequence`.
- **Conflict reasons**: `same_chord_different_command`, `higher_layer_shadowed`,
  `sequence_prefix_collision`, `context_scope_overlap`,
  `imported_binding_collision`, `platform_reserved_chord`.
- **Import-translation states**: `translated_exact`, `translated_approximated`,
  `unmapped_source_key`, `conflict_with_existing`, `rejected_unsafe`,
  `requires_manual_review`.
- **Stale-target invalidation states**: `target_live`, `target_moved_rebound`,
  `target_removed_unavailable`, `target_context_lost`,
  `target_replaced_by_deprecation`.
- **Why-unavailable reasons**: `no_active_selection`, `focus_required_elsewhere`,
  `preview_approval_required`, `policy_blocked`, `capability_missing`,
  `higher_scope_required`, `experimental_not_claimed`,
  `deprecated_use_replacement`, `upstream_dependency_unavailable`.
- **Mandatory labels** (every claimed surface must be able to show the first
  four): `command_id`, `source_layer`, `disabled_reason`,
  `lifecycle_or_deprecation` (+ optional `primary_label`, `preview_or_approval`).
- **Parity surfaces**: `keyboard`, `screen_reader`, `pointer`, `touch`,
  `cli_help`, `support_export`.

## Hard invariants (blockers)

The validator (`M5DiscoverabilityMatrixPacket::validate`) refuses to keep a
surface green if any of the following hold:

- a required surface family is missing;
- a row omits any of the four mandatory labels;
- a row's canonical command binding is incomplete;
- a shortcut-resolving surface declares no shortcut-source classes, a
  conflict-reviewing surface declares no conflict reasons, or an import-bridge
  row declares no import-translation states;
- a row declares no stale-target states, no why-unavailable reasons, no feature
  families, no parity surfaces, no consumer surfaces, or no downgrade triggers;
- a Stable surface carries no proof packet ref;
- a row `invents_alternate_label`, `masks_preview_or_approval`,
  `widens_authority`, or `hides_disabled_reason`;
- the governance review, consumer projection, proof freshness, or release
  posture is incomplete;
- the frozen vocabulary set drifts from the canonical token lists;
- the export carries raw sensitive material.

## Downstream consumers

This freeze packet is the single source of truth for downstream discoverability,
help, and parity rows and for the M5 evidence index. Product UI (menus / command
bar / context menus), the keybinding resolver inspector, help search / docs,
onboarding tours, CLI/headless help, and AI automation all read this matrix
rather than inventing parallel discoverability vocabulary.

## Outputs

- Schema: `schemas/commands/m5-discoverability-affordances.schema.json`
- Contract: `docs/commands/m5_discoverability_affordances_contract.md` (this file)
- Support export (canonical, checked-in):
  `artifacts/release/m5-discoverability-affordances-proof/support_export.json`
- Matrix CSV:
  `artifacts/release/m5-discoverability-affordances-proof/matrix.csv`
- Markdown report: `artifacts/commands/m5-discoverability-affordances.md`
- Narrowed fixtures: `fixtures/commands/m5-discoverability-affordances/`

## Regenerating

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- csv
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- report
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- validate
```

The seed builder (`seeded_m5_discoverability_matrix`) is the only mint-from-truth
producer; the inline round-trip tests assert the checked-in artifacts and
fixtures are bit-for-bit equal to it.
