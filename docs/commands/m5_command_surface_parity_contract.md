# M5 command-surface parity certification contract

Menu, context-menu, and command-bar parity with canonical labels, shortcuts, and blocked-state reasons
across every claimed M5 command surface (task **M05-741**, batch B86).

This lane is the **parity capstone** over the frozen
[M5 menu-affordance, keybinding-resolver, and command-documentation matrix][matrix] (task M05-740). It
certifies, for every one of the ten governed command-surface families, that the same action keeps the
same canonical label, the same shortcut truth, the same blocked-state reason, and the same authority
posture regardless of whether it is reached from a menu, a context menu, a command bar, a keybinding
sheet, a help page, a leader overlay, a palette row, or a contextual affordance. It mints no parallel
command vocabulary — every surface's canonical command binding, qualification, owner, required labels,
feature families, cross-modality parity surfaces, declared consumer surfaces, stale-target states,
why-unavailable reasons, and applicable downgrade triggers are pulled straight from the frozen matrix.

[matrix]: ./m5_discoverability_affordances_contract.md

## Certified surface families

Row key is the frozen `M5CommandSurfaceFamily` — one certification row per family:

- `menu_item`
- `menu_group`
- `context_menu`
- `command_bar`
- `keybinding_resolver_layer`
- `conflict_review_sheet`
- `import_bridge_row`
- `disabled_command_explainer`
- `leader_sequence_help`
- `command_documentation_surface`

## Parity dimensions

Each row is certified on four tri-state parity dimensions (each maps to an acceptance criterion or
implementation requirement) plus a headless-parity hard invariant:

| Dimension | Full (green) | Disclosed narrowing (yellow) | Blocked (red) |
| --------- | ------------ | ---------------------------- | ------------- |
| **Canonical projection** (AC1) | `canonical_label_shortcut_reason_certified` | `disclosed_reduced_shortcut_hint` | `alternate_label_or_reason_invented` |
| **Target guard** (AC2) | `stale_target_and_destructive_grouping_certified` | `disclosed_deferred_target_revalidation` | `stale_target_not_invalidated_or_destructive_unseparated` |
| **Route parity** (AC3) | `every_action_has_palette_help_keyboard_route` | `disclosed_architectural_route_exception` (**requires an active waiver**) | `contextual_only_action_without_route` |
| **Support/export parity** | `command_id_label_reason_reconstructable` | `disclosed_partial_capture` | `blocked_reason_absent_from_capture` |

The derived green/yellow/red status is **recomputed**, never asserted: any hard blocker forces `red`,
any disclosed narrowing forces `yellow`, otherwise `green`. A disclosed architectural route exception —
making an action contextual-only — is the sensitive narrowing and stays `yellow` only when an active
waiver discloses it.

### Structural completeness lints (hard blockers)

A row also blocks (`red`) unless it certifies:

- every one of the **six cross-modality parity surfaces** (`keyboard`, `screen_reader`, `pointer`,
  `touch`, `cli_help`, `support_export`);
- fixtures for every one of the **five affordance open modes** (`pointer_opened`, `keyboard_opened`,
  `compact_layout`, `touch_context_action`, `policy_blocked`) — with the same reason strings and
  shortcuts;
- every **consumer surface** the matrix declares for the family; and
- the same command semantics preserved in **headless / CLI execution** (`headless_parity_preserved`).

## Seed posture

Six families are green; four auto-narrow to yellow (`command_bar` reduced shortcut hint on a compact
layout, `context_menu` deferred stale-target revalidation, `import_bridge_row` partial support/export
capture, and `leader_sequence_help` a waivered architectural route exception). No row is blocked, so the
packet is clean and every row is publishable. Five blocked fixtures prove each red failure mode:
`menu_item` alternate-label, `context_menu` stale-target, `command_bar` contextual-only route,
`command_documentation_surface` capture-absent, and `disabled_command_explainer` headless-parity-lost.

## Artifacts and evidence

The headless emitter `aureline_shell_m5_command_surface_parity` is the only mint-from-truth path:

- Boundary schema: `schemas/commands/m5-command-surface-parity.schema.json`
- Published packet: `artifacts/release/m5-command-surface-parity-proof/packet.json`
- Published dashboard: `artifacts/release/m5-command-surface-parity-proof/dashboard.json`
- Published support export: `artifacts/release/m5-command-surface-parity-proof/support_export.json`
- Published matrix CSV: `artifacts/release/m5-command-surface-parity-proof/matrix.csv`
- Markdown report: `artifacts/commands/m5-command-surface-parity.md`
- Protected fixtures: `fixtures/commands/m5-command-surface-parity/packet.json`,
  `dashboard.json`, `support_export.json`, `compact.txt`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- validate
cargo test -p aureline-shell --lib m5_command_surface_parity
cargo test -p aureline-shell --test m5_command_surface_parity_fixtures
```

Regenerate the artifacts and fixtures from the seed with the `packet` / `dashboard` /
`support-export` / `csv` / `markdown` / `compact` subcommands of the same bin.
