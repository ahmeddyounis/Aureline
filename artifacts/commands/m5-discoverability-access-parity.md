# M5 discoverability access parity: keyboard, screen-reader, touch, and support-export parity for menu, keybinding-help, and command-doc surfaces across every claimed M5 desktop profile

Generated from the seeded packet in
[`crate::m5_discoverability_access_parity`](../../crates/aureline-shell/src/m5_discoverability_access_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- markdown > \
  artifacts/commands/m5-discoverability-access-parity.md
```

- Packet id: `m5-discoverability-access-parity:stable:0001`
- Source schema ref: `schemas/commands/m5-discoverability-access-parity.schema.json`
- Certifies matrix packet: `m5-discoverability-affordances:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required access dimensions: `non_pointer_reach`, `support_export_evidence`, `profile_stability`, `release_evidence`
- Non-pointer reach channels: `pointer_default`, `keyboard_path`, `screen_reader_narration`, `focus_return`, `touch_context_action`
- Accessibility-incident fields: `command_id`, `source_layer`, `conflict_or_blocker_reason`, `lifecycle_state`, `help_anchor_ref`
- Desktop access profiles: `reduced_motion`, `high_zoom`, `compact_layout`, `multi_window`
- Surface families certified: 10
- Green (full conformance): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable (stable-claim gate): `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Access-parity rows

| Surface family | Status | Non-pointer reach | Support-export evidence | Profile stability | Release evidence | Lifecycle | Headless | Waiver |
| -------------- | ------ | ----------------- | ----------------------- | ----------------- | ---------------- | --------- | -------- | ------ |
| Menu-bar item | `green` | `keyboard_screen_reader_and_touch_parity_certified` | `structured_incident_evidence_reconstructable` | `reachable_and_stable_across_all_profiles` | `parity_checks_gate_release_evidence` | `stable` | `true` | — |
| Menu group / submenu | `green` | `keyboard_screen_reader_and_touch_parity_certified` | `structured_incident_evidence_reconstructable` | `reachable_and_stable_across_all_profiles` | `parity_checks_gate_release_evidence` | `stable` | `true` | — |
| Context menu | `green` | `keyboard_screen_reader_and_touch_parity_certified` | `structured_incident_evidence_reconstructable` | `reachable_and_stable_across_all_profiles` | `parity_checks_gate_release_evidence` | `stable` | `true` | — |
| Command / action bar | `yellow` | `disclosed_reduced_touch_fallback` | `structured_incident_evidence_reconstructable` | `reachable_and_stable_across_all_profiles` | `parity_checks_gate_release_evidence` | `stable` | `true` | `waiver:access-parity-reduced-touch:0001` |
| Keybinding resolver layer | `green` | `keyboard_screen_reader_and_touch_parity_certified` | `structured_incident_evidence_reconstructable` | `reachable_and_stable_across_all_profiles` | `parity_checks_gate_release_evidence` | `stable` | `true` | — |
| Conflict review sheet | `green` | `keyboard_screen_reader_and_touch_parity_certified` | `structured_incident_evidence_reconstructable` | `reachable_and_stable_across_all_profiles` | `parity_checks_gate_release_evidence` | `stable` | `true` | — |
| Import-bridge row | `yellow` | `keyboard_screen_reader_and_touch_parity_certified` | `disclosed_partial_capture` | `reachable_and_stable_across_all_profiles` | `parity_checks_gate_release_evidence` | `stable` | `true` | — |
| Disabled-command explainer | `green` | `keyboard_screen_reader_and_touch_parity_certified` | `structured_incident_evidence_reconstructable` | `reachable_and_stable_across_all_profiles` | `parity_checks_gate_release_evidence` | `stable` | `true` | — |
| Leader / sequence help overlay | `yellow` | `keyboard_screen_reader_and_touch_parity_certified` | `structured_incident_evidence_reconstructable` | `disclosed_reduced_profile_coverage` | `parity_checks_gate_release_evidence` | `beta` | `true` | — |
| Command-documentation surface | `yellow` | `keyboard_screen_reader_and_touch_parity_certified` | `structured_incident_evidence_reconstructable` | `reachable_and_stable_across_all_profiles` | `disclosed_partial_evidence_refresh` | `stable` | `true` | — |

## Auto-narrowed rows

- `command_bar` (`yellow`) — On a constrained touch surface the command / action bar's hover affordance falls back to a disclosed, waivered reduced form — the hover detail collapses into a tap-to-open sheet while the keyboard path and screen-reader narration stay present and the focus returns predictably — so the reach is narrowed and disclosed rather than hover-only.
- `import_bridge_row` (`yellow`) — On the legacy import export the copy-safe support-export takes a disclosed partial capture — the export captures the command id and blocker reason but not the full incident-field set, while still disclosing the gap — so the support-export evidence is narrowed and disclosed rather than absent.
- `leader_sequence_help` (`yellow`) — On the space-constrained compact-layout profile the leader / sequence help overlay renders a disclosed reduced form — the resulting-label detail folds into an expandable hint while the overlay stays reachable and stable and keeps its keyboard path and screen-reader narration — so the profile coverage is narrowed and disclosed rather than unreachable.
- `command_documentation_surface` (`yellow`) — On one legacy release-evidence surface the command-documentation parity check refreshes on a disclosed delayed cadence while still gating the claim — a stale help anchor or missing narration still narrows the claim on the next refresh — so the release-evidence freshness is narrowed and disclosed rather than stale.

## Exact conformance causes

- `command_bar` — `parity_surface_dropped` (disclosed: `true`) — On a constrained touch surface the affordance falls back to a disclosed, waivered reduced form while the keyboard path and screen-reader narration stay present — so the reach is narrowed and disclosed rather than hover-only.
- `import_bridge_row` — `proof_stale` (disclosed: `true`) — One legacy support-export takes a disclosed partial capture — the export captures the command id and blocker reason but not the full incident-field set, while still disclosing the gap — so the support-export evidence is narrowed and disclosed rather than absent.
- `leader_sequence_help` — `parity_surface_dropped` (disclosed: `true`) — On one constrained desktop profile the surface renders a disclosed reduced form while still staying reachable and stable — so the profile coverage is narrowed and disclosed rather than unreachable.
- `command_documentation_surface` — `proof_stale` (disclosed: `true`) — One release-evidence surface refreshes on a disclosed delayed cadence while still gating the claim, so the release-evidence freshness is narrowed and disclosed rather than stale.

## Active waivers

- `waiver:access-parity-reduced-touch:0001` (`command_bar`, owner: Shell/accessibility owner, expires `2026-09-30T00:00:00Z`) — On a constrained touch surface the command / action bar's hover affordance falls back to a disclosed, waivered reduced form — the hover detail collapses into a tap-to-open sheet while the keyboard path and screen-reader narration stay present and the focus still returns predictably — so the reach is narrowed and disclosed rather than hover-only. The exception retires when the bar renders the full touch-equivalent affordance on every claimed touch surface.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- validate
cargo test -p aureline-shell --test m5_discoverability_access_parity_fixtures
```
