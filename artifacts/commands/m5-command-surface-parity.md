# M5 command-surface parity: menu, context-menu, and command-bar parity with canonical labels, shortcuts, and blocked-state reasons across every claimed M5 surface

Generated from the seeded packet in
[`crate::m5_command_surface_parity`](../../crates/aureline-shell/src/m5_command_surface_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- markdown > \
  artifacts/commands/m5-command-surface-parity.md
```

- Packet id: `m5-command-surface-parity:stable:0001`
- Source schema ref: `schemas/commands/m5-command-surface-parity.schema.json`
- Certifies matrix packet: `m5-discoverability-affordances:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required parity dimensions: `canonical_projection`, `target_guard`, `route_parity`, `support_export_parity`
- Affordance open modes certified: `pointer_opened`, `keyboard_opened`, `compact_layout`, `touch_context_action`, `policy_blocked`
- Surface families certified: 10
- Green (full conformance): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable (stable-claim gate): `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Surface family | Status | Canonical projection | Target guard | Route parity | Support/export | Headless | Waiver |
| -------------- | ------ | -------------------- | ------------ | ------------ | -------------- | -------- | ------ |
| Menu-bar item | `green` | `canonical_label_shortcut_reason_certified` | `stale_target_and_destructive_grouping_certified` | `every_action_has_palette_help_keyboard_route` | `command_id_label_reason_reconstructable` | `true` | — |
| Menu group / submenu | `green` | `canonical_label_shortcut_reason_certified` | `stale_target_and_destructive_grouping_certified` | `every_action_has_palette_help_keyboard_route` | `command_id_label_reason_reconstructable` | `true` | — |
| Context menu | `yellow` | `canonical_label_shortcut_reason_certified` | `disclosed_deferred_target_revalidation` | `every_action_has_palette_help_keyboard_route` | `command_id_label_reason_reconstructable` | `true` | — |
| Command / action bar | `yellow` | `disclosed_reduced_shortcut_hint` | `stale_target_and_destructive_grouping_certified` | `every_action_has_palette_help_keyboard_route` | `command_id_label_reason_reconstructable` | `true` | — |
| Keybinding resolver layer | `green` | `canonical_label_shortcut_reason_certified` | `stale_target_and_destructive_grouping_certified` | `every_action_has_palette_help_keyboard_route` | `command_id_label_reason_reconstructable` | `true` | — |
| Conflict review sheet | `green` | `canonical_label_shortcut_reason_certified` | `stale_target_and_destructive_grouping_certified` | `every_action_has_palette_help_keyboard_route` | `command_id_label_reason_reconstructable` | `true` | — |
| Import-bridge row | `yellow` | `canonical_label_shortcut_reason_certified` | `stale_target_and_destructive_grouping_certified` | `every_action_has_palette_help_keyboard_route` | `disclosed_partial_capture` | `true` | — |
| Disabled-command explainer | `green` | `canonical_label_shortcut_reason_certified` | `stale_target_and_destructive_grouping_certified` | `every_action_has_palette_help_keyboard_route` | `command_id_label_reason_reconstructable` | `true` | — |
| Leader / sequence help overlay | `yellow` | `canonical_label_shortcut_reason_certified` | `stale_target_and_destructive_grouping_certified` | `disclosed_architectural_route_exception` | `command_id_label_reason_reconstructable` | `true` | `waiver:leader-sequence-route-exception:0001` |
| Command-documentation surface | `green` | `canonical_label_shortcut_reason_certified` | `stale_target_and_destructive_grouping_certified` | `every_action_has_palette_help_keyboard_route` | `command_id_label_reason_reconstructable` | `true` | — |

## Auto-narrowed rows

- `context_menu` (`yellow`) — On a background-refresh surface the context menu takes a disclosed deferred stale-target revalidation — an item whose target may have moved is marked provisional and revalidated on next open while destructive items stay clearly grouped and every label and reason stays canonical — so the guard is narrowed and disclosed rather than silently misfiring.
- `command_bar` (`yellow`) — On a compact command-bar layout the surface takes a disclosed reduced shortcut hint — the resolved source-layer chip is folded into a tooltip while the chord, the canonical label, and the typed blocked-state reason stay visible on every action — so the shortcut truth is narrowed and disclosed rather than hidden or relabelled.
- `import_bridge_row` (`yellow`) — On the legacy import diagnostics export the import-bridge row takes a disclosed partial capture — the export captures the command id, the translated native binding, and the blocked-state reason but not the resolved shortcut hint, while still disclosing the gap — so the support/export parity is narrowed and disclosed rather than absent.
- `leader_sequence_help` (`yellow`) — The in-progress leader / multi-key sequence continuation is contextual-only under a disclosed, waivered architectural exception — a half-typed sequence prefix only makes sense while the leader is armed — but every resolved sequence still resolves to a canonical command with a palette, help, and keyboard route and its behaviour is documented, so the route parity is narrowed and disclosed rather than a hidden-only route.

## Exact conformance causes

- `context_menu` — `stale_target_not_invalidated` (disclosed: `true`) — On a background-refresh surface the affordance takes a disclosed deferred stale-target revalidation — an item whose target may have moved is marked provisional and revalidated on next open while destructive items stay clearly grouped — so the guard is narrowed and disclosed rather than silently misfiring.
- `command_bar` — `source_layer_hidden` (disclosed: `true`) — On a constrained layout the surface takes a disclosed reduced shortcut hint — the resolved source-layer chip is folded into a tooltip while the chord, the canonical label, and the blocked-state reason stay visible — so the shortcut truth is narrowed and disclosed rather than hidden.
- `import_bridge_row` — `proof_stale` (disclosed: `true`) — One legacy export surface takes a disclosed partial capture — a legacy diagnostics export captures the command id and blocked-state reason but not the resolved shortcut hint, while still disclosing the gap — so the support/export parity is narrowed and disclosed rather than absent.
- `leader_sequence_help` — `parity_surface_dropped` (disclosed: `true`) — One action is contextual-only under a disclosed, waivered architectural exception — the in-place affordance only makes sense against a live selection and its behaviour is still documented in help — so the route parity is narrowed and disclosed rather than a hidden-only route that widens no authority.

## Active waivers

- `waiver:leader-sequence-route-exception:0001` (`leader_sequence_help`, owner: Shell/keybinding owner, expires `2026-09-30T00:00:00Z`) — The in-progress leader / multi-key sequence continuation is contextual-only under a disclosed, waivered architectural exception — a half-typed sequence prefix only makes sense while the leader is armed, so it is not surfaced as a standalone palette row, but every resolved sequence still resolves to a canonical command with a palette, help, and keyboard route and its behaviour is documented in the command-documentation surface — so the route parity is narrowed and disclosed rather than a hidden-only route. The exception retires when the palette adopts sequence-prefix rows.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- validate
cargo test -p aureline-shell --test m5_command_surface_parity_fixtures
```
