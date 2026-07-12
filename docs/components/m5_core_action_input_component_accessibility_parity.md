# M5 core-action-input component accessibility & auto-narrowing parity (M05-1130)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5 core-action-input
component matrix (`m5_core_action_input_component_matrix`). Where the freeze matrix defines the reusable
**button, icon button, split button, text field, search field, combobox, checkbox-radio-switch toggle
control, and segmented control** primitives, and the 1125–1128 implementation lanes resolve their
per-surface truth, this lane certifies — per control family — that every action / input control claim
survives beyond the pointer-rich desktop view and **auto-narrows when its command / value / validation
proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_core_action_input_accessibility_parity_and_narrowing_when_control_truth_is_stale/`
- **Schema:** `schemas/ui/m5-core-action-input-component-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-core-action-input-component-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/ui/m5-core-action-input-component-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every family exposes a keyboard-complete,
   screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and CLI/headless-reachable path into
   the same control identity, interaction state / disposition, command binding, accessible name, value
   source, validation anchor, and immediate-versus-deferred toggle semantics the rich control shows — never
   a placeholder-as-label, a hover-only affordance, a color-only emphasis, or a motion-only cue. The
   support / release / CLI export reconstructs each control's meaning from typed tokens and opaque refs
   **without a raw payload**.

2. **Honest auto-narrowing.** When a command binding is stale / missing, an icon-only control has no
   accessible name, a split button's safe default cannot be confirmed, a validation anchor is stale, an
   immediate/deferred toggle semantic is unverified, or a search field can only disclose a partial
   retention / privacy posture, the claim auto-narrows from `trusted_control` / `reviewable_control` to the
   matching projection, discloses the narrowing with a precise trigger and binding dimension, and preserves
   the canonical identity / last-known state. A control with every dimension intact must **not** carry a
   spurious narrowing, and a weakened control can never keep a trusted, ready-to-operate claim — a loading
   button never relabels its action, and a riskier split alternate never quietly becomes the default.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the forms, settings, search, entry,
   review, repair, CLI-export, support-export, and product surfaces so product, help, and release
   publication stay aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_control` | Fully current, command-bound, accessibly-named, value-source-clear, validation-anchored, toggle-semantics-clear — ready to operate. |
| `reviewable_control` | Self-sufficient, reviewable read-only control (a combobox / segmented control a user can inspect), not an authoritative mutation surface. |
| `command_binding_unverified_projection` | Command binding stale / missing (button). |
| `accessible_name_unverified_projection` | Icon-only control has no confirmed accessible name (icon button). |
| `default_safety_unverified_projection` | Split button's safe default cannot be confirmed (split button). |
| `validation_unverified_projection` | Field validation anchor is stale (text field). |
| `toggle_semantics_unverified_projection` | Immediate-versus-deferred toggle semantic is unverified (toggle control). |
| `retention_disclosed_projection` | Search field can only disclose a partial / redacted retention posture — an **honest disclosed-absence**, not a truth overstatement (search field). |

## Weakening dimensions and their frozen triggers

Each family maps 1:1 to a claim dimension; a weak condition state narrows to the matching projection and
names the on-topic frozen matrix downgrade trigger:

| Dimension (family) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `command_binding_clarity` (button) | `command_binding_stale` | `command_binding_unstated` | yes |
| `accessible_name_clarity` (icon button) | `accessible_name_missing` | `icon_only_destructive_unlabeled` | yes |
| `default_safety_clarity` (split button) | `default_safety_stale` | `split_defaulted_to_riskier_alternate` | yes |
| `label_validation_clarity` (text field) | `validation_anchor_stale` | `validation_state_unstated` | yes |
| `toggle_semantics_clarity` (toggle control) | `toggle_semantics_unverified` | `switch_and_deferred_checkbox_blurred` | yes |
| `clear_submit_privacy_clarity` (search field) | `retention_disclosed_partial` | `proof_stale` | no (honest disclosed-absence) |
| `value_source_clarity` (combobox) | *(green — fully qualified reviewable)* | — | — |
| `selected_mode_clarity` (segmented control) | *(green — fully qualified trusted)* | — | — |

The `retention_disclosed_partial` state is deliberately **excluded** from
`cannot_be_shown_trusted`: a partial / redacted retention posture shown honestly with an inspectable
privacy note is a disclosed-absence operation, not a truth overstatement.

## Structure-heavy families

The **split button** (default action plus alternate menu), **combobox** (filterable option list), and
**segmented control** (segments) render a dense structured surface, so they must additionally bind their
structured layout to an equivalent flat list / textual path (a `structured` fallback modality **plus** a
non-visual list / textual / CLI path).

## Certified rows

Eight rows, one per family: **1 green** (segmented control — selected mode fully stated, trusted) and
**7 yellow** — the combobox stays a fully-qualified reviewable control but discloses a screen-reader
reduction, and the remaining six auto-narrow to their permitted projections. **No red rows may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To
regenerate after an intentional change:

```
GEN_CORE_ACTION_INPUT_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_core_action_input_accessibility_parity_and_narrowing_when_control_truth_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
