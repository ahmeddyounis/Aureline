# M5 command explainers: leader/partial-sequence overlays, disabled-command and why-unavailable explainers, shared remediation, and copy-safe blocker export across every claimed M5 command surface

Generated from the seeded packet in
[`crate::m5_command_explainers`](../../crates/aureline-shell/src/m5_command_explainers/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_explainers -- markdown > \
  artifacts/commands/m5-command-explainers.md
```

- Packet id: `m5-command-explainers:stable:0001`
- Source schema ref: `schemas/commands/m5-command-explainers.schema.json`
- Certifies matrix packet: `m5-discoverability-affordances:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required explainer dimensions: `leader_overlay`, `blocked_explainer`, `remediation_parity`, `explainer_export`
- Leader-overlay fields narrated: `typed_prefix`, `current_mode`, `available_next_keys`, `resulting_command_label_and_id`, `timeout_cancel_posture`, `surface_unsupported_note`
- Blocker classes named: `context`, `trust`, `policy`, `lifecycle`, `missing_dependency`, `stale_target`, `mode_overlay`
- Remediation actions offered: `next_safe_action`, `copy_command_id`, `open_help`
- Reach modes: `pointer_default`, `keyboard_only`, `screen_reader`, `compact_layout`, `touch_context_action`
- Surface families certified: 10
- Green (full conformance): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable (stable-claim gate): `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Explainer rows

| Surface family | Status | Leader overlay | Blocked explainer | Remediation parity | Explainer export | Lifecycle | Headless | Waiver |
| -------------- | ------ | -------------- | ----------------- | ------------------ | ---------------- | --------- | -------- | ------ |
| Menu-bar item | `green` | `typed_prefix_next_keys_and_timeout_narrated` | `blocker_class_next_action_and_actions_certified` | `shared_reason_packet_across_all_surfaces` | `blocker_and_command_id_reconstructable` | `stable` | `true` | — |
| Menu group / submenu | `green` | `typed_prefix_next_keys_and_timeout_narrated` | `blocker_class_next_action_and_actions_certified` | `shared_reason_packet_across_all_surfaces` | `blocker_and_command_id_reconstructable` | `stable` | `true` | — |
| Context menu | `yellow` | `typed_prefix_next_keys_and_timeout_narrated` | `blocker_class_next_action_and_actions_certified` | `disclosed_surface_local_remediation_note` | `blocker_and_command_id_reconstructable` | `stable` | `true` | — |
| Command / action bar | `yellow` | `typed_prefix_next_keys_and_timeout_narrated` | `disclosed_reduced_explainer_detail` | `shared_reason_packet_across_all_surfaces` | `blocker_and_command_id_reconstructable` | `stable` | `true` | — |
| Keybinding resolver layer | `green` | `typed_prefix_next_keys_and_timeout_narrated` | `blocker_class_next_action_and_actions_certified` | `shared_reason_packet_across_all_surfaces` | `blocker_and_command_id_reconstructable` | `stable` | `true` | — |
| Conflict review sheet | `green` | `typed_prefix_next_keys_and_timeout_narrated` | `blocker_class_next_action_and_actions_certified` | `shared_reason_packet_across_all_surfaces` | `blocker_and_command_id_reconstructable` | `stable` | `true` | — |
| Import-bridge row | `yellow` | `typed_prefix_next_keys_and_timeout_narrated` | `blocker_class_next_action_and_actions_certified` | `shared_reason_packet_across_all_surfaces` | `disclosed_partial_capture` | `stable` | `true` | — |
| Disabled-command explainer | `green` | `typed_prefix_next_keys_and_timeout_narrated` | `blocker_class_next_action_and_actions_certified` | `shared_reason_packet_across_all_surfaces` | `blocker_and_command_id_reconstructable` | `stable` | `true` | — |
| Leader / sequence help overlay | `yellow` | `disclosed_reduced_sequence_overlay` | `blocker_class_next_action_and_actions_certified` | `shared_reason_packet_across_all_surfaces` | `blocker_and_command_id_reconstructable` | `beta` | `true` | `waiver:command-explainer-reduced-sequence:0001` |
| Command-documentation surface | `green` | `typed_prefix_next_keys_and_timeout_narrated` | `blocker_class_next_action_and_actions_certified` | `shared_reason_packet_across_all_surfaces` | `blocker_and_command_id_reconstructable` | `stable` | `true` | — |

## Auto-narrowed rows

- `context_menu` (`yellow`) — On the space-constrained context menu one blocked action appends a disclosed short surface-local remediation note while still projecting the shared reason packet and remediation language — so the remediation is narrowed and disclosed rather than an invented surface-local error prose.
- `command_bar` (`yellow`) — On the constrained command / action bar the disabled-command explainer takes a disclosed reduced detail — the next-safe-action guidance is folded into an expandable section while the blocker class and the copy-command-id / open-help actions stay visible — so the explainer is narrowed and disclosed rather than failing silently or showing only generic copy.
- `import_bridge_row` (`yellow`) — On the legacy import export the copy-safe explainer export takes a disclosed partial capture — the export captures the blocker class and command id but not the full remediation-action list, while still disclosing the gap — so the copy-safe export parity is narrowed and disclosed rather than absent.
- `leader_sequence_help` (`yellow`) — On the space-constrained leader / sequence help overlay one deep prefix renders a disclosed, waivered reduced form — the resulting command labels / ids are folded into an expandable hint while the typed prefix, current mode, available next keys, and timeout / cancel posture stay visible — so the overlay is narrowed and disclosed rather than hiding next-available actions behind hover.

## Exact conformance causes

- `context_menu` — `alternate_label_invented` (disclosed: `true`) — One constrained surface appends a disclosed short surface-local remediation note while still projecting the shared reason packet and remediation language, so the remediation is narrowed and disclosed rather than an invented surface-local error prose.
- `command_bar` — `disabled_reason_hidden` (disclosed: `true`) — On a constrained surface the disabled-command explainer takes a disclosed reduced detail — the next-safe-action guidance is folded into an expandable section while the blocker class and the copy-command-id / open-help actions stay visible — so the explainer is narrowed and disclosed rather than failing silently or showing only generic copy.
- `import_bridge_row` — `proof_stale` (disclosed: `true`) — One legacy explainer export takes a disclosed partial capture — the export captures the blocker class and command id but not the full remediation-action list, while still disclosing the gap — so the copy-safe export parity is narrowed and disclosed rather than absent.
- `leader_sequence_help` — `source_layer_hidden` (disclosed: `true`) — On a constrained surface the leader / sequence overlay takes a disclosed, waivered reduced form — the resulting command labels / ids are folded into an expandable hint while the typed prefix, current mode, available next keys, and timeout / cancel posture stay visible — so the overlay is narrowed and disclosed rather than hiding next-available actions behind hover.

## Active waivers

- `waiver:command-explainer-reduced-sequence:0001` (`leader_sequence_help`, owner: Shell/keyboard owner, expires `2026-09-30T00:00:00Z`) — On the space-constrained leader / sequence help overlay one deep prefix renders a disclosed, waivered reduced form — the resulting command labels / ids are folded into an expandable hint while the overlay still shows the typed prefix, the current mode, the available next keys, and the timeout / cancel posture, and the command palette and keybinding UI keep the full resulting-label detail — so the overlay is narrowed and disclosed rather than hiding next-available actions behind hover. The exception retires when the overlay renders the full resulting-label detail on every claimed prefix depth.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_explainers -- validate
cargo test -p aureline-shell --test m5_command_explainers_fixtures
```
