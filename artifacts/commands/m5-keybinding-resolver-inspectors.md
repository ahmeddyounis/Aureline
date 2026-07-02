# M5 keybinding resolver inspectors: winning/shadowed shortcut inspection, controlled bridge outcomes, and copy-safe resolver export across every claimed M5 command surface

Generated from the seeded packet in
[`crate::m5_keybinding_resolver_inspectors`](../../crates/aureline-shell/src/m5_keybinding_resolver_inspectors/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- markdown > \
  artifacts/commands/m5-keybinding-resolver-inspectors.md
```

- Packet id: `m5-keybinding-resolver-inspectors:stable:0001`
- Source schema ref: `schemas/commands/m5-keybinding-resolver-inspectors.schema.json`
- Certifies matrix packet: `m5-discoverability-affordances:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required inspection dimensions: `resolver_inspection`, `bridge_outcome`, `leader_sequence_inspection`, `resolver_export`
- Inspector fields revealed: `source_layer`, `scope`, `current_mode`, `active_winner`, `losing_candidates`, `reserved_unavailable_state`, `fallback_command_path`
- Controlled bridge outcomes: `exact`, `translated`, `alias_only`, `partial`, `shimmed`, `unsupported`
- Migration actions: `open_docs`, `manual_fix`, `no_action_needed`
- Surface families certified: 10
- Green (full conformance): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable (stable-claim gate): `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Inspection rows

| Surface family | Status | Resolver inspection | Bridge outcome | Leader sequence | Resolver export | Winner | Headless | Waiver |
| -------------- | ------ | ------------------- | -------------- | --------------- | --------------- | ------ | -------- | ------ |
| Menu-bar item | `green` | `winner_shadowed_source_and_fallback_certified` | `controlled_states_and_migration_actions_certified` | `precedence_timeout_cancel_narration_certified` | `command_id_and_winning_source_reconstructable` | `user_keybinding` | `true` | — |
| Menu group / submenu | `green` | `winner_shadowed_source_and_fallback_certified` | `controlled_states_and_migration_actions_certified` | `precedence_timeout_cancel_narration_certified` | `command_id_and_winning_source_reconstructable` | `none` | `true` | — |
| Context menu | `green` | `winner_shadowed_source_and_fallback_certified` | `controlled_states_and_migration_actions_certified` | `precedence_timeout_cancel_narration_certified` | `command_id_and_winning_source_reconstructable` | `user_keybinding` | `true` | — |
| Command / action bar | `green` | `winner_shadowed_source_and_fallback_certified` | `controlled_states_and_migration_actions_certified` | `precedence_timeout_cancel_narration_certified` | `command_id_and_winning_source_reconstructable` | `user_keybinding` | `true` | — |
| Keybinding resolver layer | `yellow` | `disclosed_reduced_inspector_detail` | `controlled_states_and_migration_actions_certified` | `precedence_timeout_cancel_narration_certified` | `command_id_and_winning_source_reconstructable` | `leader_sequence` | `true` | — |
| Conflict review sheet | `green` | `winner_shadowed_source_and_fallback_certified` | `controlled_states_and_migration_actions_certified` | `precedence_timeout_cancel_narration_certified` | `command_id_and_winning_source_reconstructable` | `leader_sequence` | `true` | — |
| Import-bridge row | `yellow` | `winner_shadowed_source_and_fallback_certified` | `disclosed_partial_bridge_coverage` | `precedence_timeout_cancel_narration_certified` | `command_id_and_winning_source_reconstructable` | `leader_sequence` | `true` | — |
| Disabled-command explainer | `green` | `winner_shadowed_source_and_fallback_certified` | `controlled_states_and_migration_actions_certified` | `precedence_timeout_cancel_narration_certified` | `command_id_and_winning_source_reconstructable` | `none` | `true` | — |
| Leader / sequence help overlay | `yellow` | `winner_shadowed_source_and_fallback_certified` | `controlled_states_and_migration_actions_certified` | `disclosed_reduced_sequence_hint` | `command_id_and_winning_source_reconstructable` | `leader_sequence` | `true` | `waiver:leader-sequence-reduced-hint:0001` |
| Command-documentation surface | `yellow` | `winner_shadowed_source_and_fallback_certified` | `controlled_states_and_migration_actions_certified` | `precedence_timeout_cancel_narration_certified` | `disclosed_partial_capture` | `leader_sequence` | `true` | — |

## Auto-narrowed rows

- `keybinding_resolver_layer` (`yellow`) — On a constrained resolver surface the inspector takes a disclosed reduced inspector detail — the full losing-candidate list is folded into an expandable "N shadowed" summary while the winning source layer, the fallback command path, the scope, the current mode, and the reserved/unavailable state stay visible — so the shadowed truth is narrowed and disclosed rather than hidden.
- `import_bridge_row` (`yellow`) — One slice of imported bindings takes a disclosed partial bridge coverage — it is reported with a controlled `partial` / `shimmed` state and an open-docs / manual-fix action while manual review completes — so the import outcome is narrowed and disclosed with the controlled bridge-outcome vocabulary rather than generic imported wording.
- `leader_sequence_help` (`yellow`) — The armed leader / multi-key sequence overlay renders a disclosed, waivered reduced sequence hint — a half-typed continuation folds its next-key list into a compact hint while the precedence model, the timeout / cancel hints, and the narration stay available and every resolved sequence still names its winning source and fallback — so the sequence availability is narrowed and disclosed rather than requiring hidden knowledge.
- `command_documentation_surface` (`yellow`) — On the legacy documentation export the resolver/export surface takes a disclosed partial capture — the export captures the command id and the winning source but not the full shadowed list, while still disclosing the gap — so the resolver/export parity is narrowed and disclosed rather than absent.

## Exact conformance causes

- `keybinding_resolver_layer` — `source_layer_hidden` (disclosed: `true`) — On a constrained surface the resolver inspector takes a disclosed reduced inspector detail — the full losing-candidate list is folded into an expandable "N shadowed" summary while the winning source layer, the fallback command path, and the reserved/unavailable state stay visible — so the shadowed truth is narrowed and disclosed rather than hidden.
- `import_bridge_row` — `import_translation_untruthful` (disclosed: `true`) — One slice of imported bindings takes a disclosed partial bridge coverage — it is reported with a controlled `partial` / `shimmed` state and an open-docs / manual-fix action while manual review completes — so the import outcome is narrowed and disclosed rather than generic imported wording.
- `leader_sequence_help` — `source_layer_hidden` (disclosed: `true`) — A half-typed leader / multi-key sequence continuation renders a disclosed, waivered reduced sequence hint — the armed-sequence overlay folds the next-key list into a compact hint while the precedence, timeout / cancel hints, and narration stay available — so the sequence availability is narrowed and disclosed rather than requiring hidden knowledge.
- `command_documentation_surface` — `proof_stale` (disclosed: `true`) — One legacy resolver/export surface takes a disclosed partial capture — the export captures the command id and the winning source but not the full shadowed list, while still disclosing the gap — so the resolver/export parity is narrowed and disclosed rather than absent.

## Active waivers

- `waiver:leader-sequence-reduced-hint:0001` (`leader_sequence_help`, owner: Shell/keybinding owner, expires `2026-09-30T00:00:00Z`) — The armed leader / multi-key sequence overlay renders a reduced next-key hint under a disclosed, waivered exception — a half-typed sequence continuation folds its next-key list into a compact hint while the precedence model, the timeout / cancel hints, and the accessibility narration stay available and every resolved sequence still names its winning source and fallback command path — so the sequence availability is narrowed and disclosed rather than requiring hidden knowledge. The exception retires when the overlay renders the full next-key list on every claimed family.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- validate
cargo test -p aureline-shell --test m5_keybinding_resolver_inspectors_fixtures
```
