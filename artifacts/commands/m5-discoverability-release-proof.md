# M5 discoverability release proof: menu-affordance, keybinding-resolver, leader-help, and command-documentation truth for every claimed M5 command surface

Generated from the seeded packet in
[`crate::m5_discoverability_release_proof`](../../crates/aureline-shell/src/m5_discoverability_release_proof/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_release_proof -- markdown > \
  artifacts/commands/m5-discoverability-release-proof.md
```

- Packet id: `m5-discoverability-release-proof:stable:0001`
- Source schema ref: `schemas/commands/m5-discoverability-release-proof.schema.json`
- Certifies matrix packet: `m5-discoverability-affordances:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Release-evidence index: `release_center.discoverability_release_evidence_index`
- Bundled proof lanes: `artifacts/release/m5-command-surface-parity-proof/packet.json`, `artifacts/release/m5-keybinding-resolver-inspectors-proof/packet.json`, `artifacts/release/m5-command-explainers-proof/packet.json`, `artifacts/release/m5-command-documentation-proof/packet.json`
- Required proof dimensions: `menu_affordance`, `keybinding_resolver`, `leader_help`, `command_documentation`
- Desktop profiles: `compact_desktop`, `standard_desktop`, `expanded_desktop`, `mixed_dpi`, `multi_monitor`, `dependency_missing_restore`
- Surface families certified: 10
- Green (full conformance): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable (stable-claim gate): `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Release-proof rows

| Surface family | Status | Menu affordance | Keybinding resolver | Leader help | Command documentation | Lifecycle | Headless | Waiver |
| -------------- | ------ | --------------- | ------------------- | ----------- | --------------------- | --------- | -------- | ------ |
| Menu-bar item | `green` | `menu_affordance_parity_certified` | `shortcut_resolution_inspectable` | `leader_and_blocked_explainer_certified` | `command_doc_record_certified` | `stable` | `true` | — |
| Menu group / submenu | `green` | `menu_affordance_parity_certified` | `shortcut_resolution_inspectable` | `leader_and_blocked_explainer_certified` | `command_doc_record_certified` | `stable` | `true` | — |
| Context menu | `green` | `menu_affordance_parity_certified` | `shortcut_resolution_inspectable` | `leader_and_blocked_explainer_certified` | `command_doc_record_certified` | `stable` | `true` | — |
| Command / action bar | `yellow` | `disclosed_reduced_affordance_hint` | `shortcut_resolution_inspectable` | `leader_and_blocked_explainer_certified` | `command_doc_record_certified` | `stable` | `true` | — |
| Keybinding resolver layer | `yellow` | `menu_affordance_parity_certified` | `disclosed_reduced_resolver_detail` | `leader_and_blocked_explainer_certified` | `command_doc_record_certified` | `stable` | `true` | — |
| Conflict review sheet | `green` | `menu_affordance_parity_certified` | `shortcut_resolution_inspectable` | `leader_and_blocked_explainer_certified` | `command_doc_record_certified` | `stable` | `true` | — |
| Import-bridge row | `green` | `menu_affordance_parity_certified` | `shortcut_resolution_inspectable` | `leader_and_blocked_explainer_certified` | `command_doc_record_certified` | `stable` | `true` | — |
| Disabled-command explainer | `green` | `menu_affordance_parity_certified` | `shortcut_resolution_inspectable` | `leader_and_blocked_explainer_certified` | `command_doc_record_certified` | `stable` | `true` | — |
| Leader / sequence help overlay | `yellow` | `menu_affordance_parity_certified` | `shortcut_resolution_inspectable` | `disclosed_reduced_explainer_detail` | `command_doc_record_certified` | `beta` | `true` | `waiver:release-proof-reduced-explainer:0001` |
| Command-documentation surface | `yellow` | `menu_affordance_parity_certified` | `shortcut_resolution_inspectable` | `leader_and_blocked_explainer_certified` | `disclosed_reduced_doc_detail` | `stable` | `true` | — |

## Auto-narrowed rows

- `command_bar` (`yellow`) — On the dense command / action bar one affordance renders a disclosed shortened shortcut hint — the modifier chord folds into a compact glyph — while still projecting the canonical label and blocked-state reason, so the menu-affordance parity is narrowed and disclosed rather than inventing an alternate label.
- `keybinding_resolver_layer` (`yellow`) — On the keybinding resolver layer the shadowed-candidate detail folds into an expandable inspector while the winning binding and its source layer stay named inline, so the shortcut resolution is narrowed and disclosed rather than hidden.
- `leader_sequence_help` (`yellow`) — On the space-constrained compact-layout profile the leader / sequence help overlay renders a disclosed, waivered reduced explainer — the next-safe-action detail folds into an expandable note while the typed prefix, available next keys, resulting command id, and blocker class stay present — so the blocked / in-progress intent stays explainable rather than silent.
- `command_documentation_surface` (`yellow`) — On one legacy documentation surface the canonical example set folds into a "see full docs" link while the command id, lifecycle state, aliases, and supported surfaces stay present, so the command-documentation truth is narrowed and disclosed rather than stale.

## Exact conformance causes

- `command_bar` — `source_layer_hidden` (disclosed: `true`) — One dense affordance renders a disclosed shortened shortcut / label hint while still projecting the canonical label and blocked-state reason, so the menu-affordance parity is narrowed and disclosed rather than inventing an alternate label.
- `keybinding_resolver_layer` — `source_layer_hidden` (disclosed: `true`) — One resolver surface folds the shadowed-candidate detail into an expandable inspector while still naming the winning binding and its source layer, so the shortcut resolution is narrowed and disclosed rather than hidden.
- `leader_sequence_help` — `source_layer_hidden` (disclosed: `true`) — One surface renders a disclosed, waivered reduced explainer — the next-safe-action detail folds into an expandable note while the blocker class and command id stay present — so the leader / blocked-command explanation is narrowed and disclosed rather than silent.
- `command_documentation_surface` — `proof_stale` (disclosed: `true`) — One legacy doc surface folds the example set into a "see full docs" link while the command id, lifecycle state, and supported surfaces stay present, so the command-documentation truth is narrowed and disclosed rather than stale.

## Active waivers

- `waiver:release-proof-reduced-explainer:0001` (`leader_sequence_help`, owner: Shell/command-discovery owner, expires `2026-09-30T00:00:00Z`) — On the space-constrained compact-layout profile the leader / sequence help overlay renders a disclosed, waivered reduced explainer — the next-safe-action detail folds into an expandable note while the typed prefix, available next keys, resulting command id, and blocker class stay present — so the blocked / in-progress intent stays explainable rather than silent. The exception retires when the overlay renders the full explainer inline on every claimed profile.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_release_proof -- validate
cargo test -p aureline-shell --test m5_discoverability_release_proof_fixtures
```
