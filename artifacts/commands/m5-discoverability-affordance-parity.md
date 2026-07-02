# M5 discoverability affordance parity: buttons, inline affordances, tooltips, onboarding tips, AI/voice hints, and companion handoffs reuse one command record across every claimed M5 action

Generated from the seeded packet in
[`crate::m5_discoverability_affordance_parity`](../../crates/aureline-shell/src/m5_discoverability_affordance_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- markdown > \
  artifacts/commands/m5-discoverability-affordance-parity.md
```

- Packet id: `m5-discoverability-affordance-parity:stable:0001`
- Source schema ref: `schemas/commands/m5-discoverability-affordance-parity.schema.json`
- Certifies matrix packet: `m5-discoverability-affordances:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required parity dimensions: `label_reuse`, `side_effect_truth`, `authority_reach`, `origin_export`
- Canonical record fields reused: `canonical_label`, `alias_set`, `shortcut_hint`, `side_effect_class`, `preview_requirement`, `lifecycle_badge`
- Reach modes: `pointer_default`, `keyboard_focus`, `screen_reader`, `compact_layout`, `touch_context_action`
- Convenience affordances certified: 7
- Green (full conformance): 3
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable (stable-claim gate): `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Parity rows

| Affordance | Drives | Status | Label reuse | Side-effect truth | Authority reach | Origin export | Lifecycle | Preview | Headless | Waiver |
| ---------- | ------ | ------ | ----------- | ----------------- | --------------- | ------------- | --------- | ------- | -------- | ------ |
| Primary / action button | `menu_item` | `green` | `canonical_label_alias_and_lifecycle_reused` | `side_effect_and_preview_truth_preserved` | `focus_equivalent_and_bounded_authority` | `origin_command_identity_reconstructable` | `stable` | `no_preview_required` | `true` | — |
| Inline quick-action affordance | `command_bar` | `yellow` | `disclosed_shortened_affordance_label` | `side_effect_and_preview_truth_preserved` | `focus_equivalent_and_bounded_authority` | `origin_command_identity_reconstructable` | `stable` | `structured_diff_preview` | `true` | — |
| Tooltip / hovercard | `context_menu` | `yellow` | `canonical_label_alias_and_lifecycle_reused` | `disclosed_summarized_side_effect_note` | `focus_equivalent_and_bounded_authority` | `origin_command_identity_reconstructable` | `stable` | `structured_diff_preview` | `true` | — |
| Onboarding tip | `leader_sequence_help` | `green` | `canonical_label_alias_and_lifecycle_reused` | `side_effect_and_preview_truth_preserved` | `focus_equivalent_and_bounded_authority` | `origin_command_identity_reconstructable` | `beta` | `no_preview_required` | `true` | — |
| AI hint | `command_documentation_surface` | `green` | `canonical_label_alias_and_lifecycle_reused` | `side_effect_and_preview_truth_preserved` | `focus_equivalent_and_bounded_authority` | `origin_command_identity_reconstructable` | `stable` | `no_preview_required` | `true` | — |
| Voice hint | `disabled_command_explainer` | `yellow` | `canonical_label_alias_and_lifecycle_reused` | `side_effect_and_preview_truth_preserved` | `focus_equivalent_and_bounded_authority` | `disclosed_partial_capture` | `stable` | `structured_diff_preview` | `true` | — |
| Companion / browser handoff | `import_bridge_row` | `yellow` | `canonical_label_alias_and_lifecycle_reused` | `side_effect_and_preview_truth_preserved` | `disclosed_reduced_hover_fallback` | `origin_command_identity_reconstructable` | `stable` | `policy_authoring_or_waiver_preview` | `true` | `waiver:affordance-parity-reduced-hover:0001` |

## Auto-narrowed rows

- `inline_affordance` (`yellow`) — On the space-constrained inline quick-action card the label renders a disclosed shortened form while the card still links the canonical command id, alias set, shortcut hint, and lifecycle badge — so the label is narrowed and disclosed rather than an invented convenience-specific label.
- `tooltip` (`yellow`) — On the constrained tooltip / hovercard the full side-effect prose is folded into a disclosed summary while the preview / approval requirement and side-effect class stay visible — so the side-effect truth is narrowed and disclosed rather than softened into a one-tap convenience.
- `voice_hint` (`yellow`) — On the legacy voice-transcript export the copy-safe origin export takes a disclosed partial capture — the export captures the affordance and command id but not the full canonical record, while still disclosing the gap — so the origin-export parity is narrowed and disclosed rather than absent.
- `companion_handoff` (`yellow`) — On a touch / narrow companion surface the desktop hover affordance falls back to a disclosed, waivered reduced form — the hovercard detail collapses into a tap-to-open sheet while the companion keeps a keyboard-focus and context-action equivalent and still names the same canonical command id within the desktop command's authority — so the reach is narrowed and disclosed rather than hover-only.

## Exact conformance causes

- `inline_affordance` — `alternate_label_invented` (disclosed: `true`) — On a space-constrained affordance the label renders a disclosed shortened form while the affordance still links the canonical command id, alias set, shortcut hint, and lifecycle badge — so the label is narrowed and disclosed rather than an invented convenience-specific label.
- `tooltip` — `preview_approval_masked` (disclosed: `true`) — On a constrained affordance the full side-effect prose is folded into a disclosed summary while the preview / approval requirement and side-effect class stay visible — so the side-effect truth is narrowed and disclosed rather than softened into a one-tap convenience.
- `voice_hint` — `proof_stale` (disclosed: `true`) — One legacy export takes a disclosed partial capture — the export captures the affordance and command id but not the full canonical record, while still disclosing the gap — so the origin-export parity is narrowed and disclosed rather than absent.
- `companion_handoff` — `parity_surface_dropped` (disclosed: `true`) — On a touch / narrow surface a hover affordance falls back to a disclosed, waivered reduced form while still keeping a keyboard-focus and context-action equivalent — so the reach is narrowed and disclosed rather than hover-only.

## Active waivers

- `waiver:affordance-parity-reduced-hover:0001` (`companion_handoff`, owner: Shell/companion owner, expires `2026-09-30T00:00:00Z`) — On a touch / narrow companion surface the desktop hover affordance falls back to a disclosed, waivered reduced form — the hovercard detail collapses into a tap-to-open sheet while the companion keeps a keyboard-focus and context-action equivalent, and the companion hint still names the same canonical command id and stays within the desktop command's authority — so the reach is narrowed and disclosed rather than hover-only. The exception retires when the companion renders the full hovercard equivalent on every touch surface.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- validate
cargo test -p aureline-shell --test m5_discoverability_affordance_parity_fixtures
```
