# M5 discoverability-affordance-parity certification contract

Buttons, inline affordances, tooltips, onboarding tips, AI/voice hints, and companion handoffs reuse one
command record across every claimed M5 action (task **M05-745**, batch B86).

This lane is the **convenience-affordance parity capstone** over the frozen
[M5 menu-affordance, keybinding-resolver, and command-documentation matrix][matrix] (task M05-740). Where the
sibling capstones certify the *command-discovery surfaces* (menus, keybinding inspectors, command
documentation, blocked-state explainers), this lane certifies the *convenience affordances* around those
commands: a primary / action button's text, an inline quick-action-card affordance, a tooltip / hovercard
label, an onboarding tip reference, an AI hint string, a voice hint string, and a companion / browser handoff
affordance. Each one must reuse the same one command record rather than inventing a convenience-specific
label, lifecycle language, side-effect story, or authority shortcut.

Each convenience affordance **drives** exactly one governed matrix surface family and pulls its canonical
command binding, qualification, owner, required labels, lifecycle label, preview class, feature families,
declared consumer surfaces, and applicable downgrade triggers straight from that family's frozen row, so the
lane mints no parallel command vocabulary and cannot certify an affordance the matrix does not anchor.

[matrix]: ./m5_discoverability_affordances_contract.md

## Certified convenience affordances

Row key is the new `M5ConvenienceAffordance` — one parity row per affordance, each driving one matrix surface
family:

| Affordance | Drives surface family |
| ---------- | --------------------- |
| `button` | `menu_item` |
| `inline_affordance` | `command_bar` |
| `tooltip` | `context_menu` |
| `onboarding_tip` | `leader_sequence_help` |
| `ai_hint` | `command_documentation_surface` |
| `voice_hint` | `disabled_command_explainer` |
| `companion_handoff` | `import_bridge_row` |

## Parity dimensions

Each row is certified on four tri-state parity dimensions (each maps to an acceptance criterion or
implementation requirement) plus a headless-parity hard invariant:

| Dimension | Full (green) | Disclosed narrowing (yellow) | Blocked (red) |
| --------- | ------------ | ---------------------------- | ------------- |
| **Label reuse** (AC1 / impl req 1) | `canonical_label_alias_and_lifecycle_reused` | `disclosed_shortened_affordance_label` | `private_label_or_lifecycle_invented` |
| **Side-effect truth** (AC2) | `side_effect_and_preview_truth_preserved` | `disclosed_summarized_side_effect_note` | `side_effect_or_preview_truth_weakened` |
| **Authority reach** (impl req 3) | `focus_equivalent_and_bounded_authority` | `disclosed_reduced_hover_fallback` (**requires an active waiver**) | `hover_only_or_authority_overreach` |
| **Origin export** (impl req 4) | `origin_command_identity_reconstructable` | `disclosed_partial_capture` | `originating_command_absent_from_capture` |

The derived green/yellow/red status is **recomputed**, never asserted: any hard blocker forces `red`, any
disclosed narrowing forces `yellow`, otherwise `green`. A disclosed reduced hover fallback — a hover
affordance collapsing to a reduced touch form — is the sensitive narrowing and stays `yellow` only when an
active waiver discloses it.

## Canonical record fields

The parity fixtures compare all six canonical command-record fields the implementation requirements name
across visible affordances and help/export surfaces; a row blocks unless it reuses every one:

- `canonical_label`
- `alias_set`
- `shortcut_hint`
- `side_effect_class`
- `preview_requirement`
- `lifecycle_badge`

## Reach modes

A hover-only affordance must still have focus / context-action equivalents, so each row stays reachable in
all five reach modes — `pointer_default`, `keyboard_focus`, `screen_reader`, `compact_layout`,
`touch_context_action`.

## Structural completeness lints (hard blockers)

A row blocks (`red`) unless it certifies:

- every one of the **six canonical record fields**;
- every one of the **five reach modes**;
- every **consumer surface** the matrix declares for the driving surface family; and
- the same parity preserved in **headless / CLI execution** (`headless_parity_preserved`).

## Records

- **Packet** (`AffordanceParityPacket`): the full set of per-affordance rows, derived counts, active waivers,
  exact conformance causes, and blocking findings. Boundary schema:
  `schemas/commands/m5-discoverability-affordance-parity.schema.json`.
- **Dashboard** (`AffordanceParityDashboard`): the light projection the button / tooltip / onboarding / AI /
  voice / companion tooling and Support Center / CLI reads to auto-narrow a convenience affordance's parity
  claim.
- **Support export** (`AffordanceParitySupportExport`): the packet + dashboard + copy-safe case ids a support
  bundle, doc, or migration packet pivots on so the originating command identity is reconstructable even when
  the action was triggered from a convenience affordance.

## Seed posture

Three affordances are green (`button`, `onboarding_tip`, `ai_hint`); four auto-narrow to yellow: the
`inline_affordance` discloses a shortened label, the `tooltip` discloses a summarized side-effect note, the
`voice_hint` discloses a partial copy-safe export capture, and the `companion_handoff` carries a waivered
reduced hover fallback. No row is blocked, so the packet is clean and every row is publishable.

## Published artifacts

- Markdown report: `artifacts/commands/m5-discoverability-affordance-parity.md`
- Packet: `artifacts/release/m5-discoverability-affordance-parity-proof/packet.json`
- Dashboard: `artifacts/release/m5-discoverability-affordance-parity-proof/dashboard.json`
- Support export: `artifacts/release/m5-discoverability-affordance-parity-proof/support_export.json`
- CSV: `artifacts/release/m5-discoverability-affordance-parity-proof/matrix.csv`
- Protected fixtures: `fixtures/commands/m5-discoverability-affordance-parity/packet.json`, `dashboard.json`,
  `support_export.json`, `compact.txt`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- validate
cargo test -p aureline-shell --lib m5_discoverability_affordance_parity
cargo test -p aureline-shell --test m5_discoverability_affordance_parity_fixtures
```

Regenerate the artifacts and fixtures (the headless emitter is the only mint-from-truth path) with the
`packet`, `dashboard`, `support-export`, `csv`, `markdown`, and `compact` subcommands of
`aureline_shell_m5_discoverability_affordance_parity`.
