# Component-state registry certification for shell, editor, panel, popover, dialog, and extension surfaces — contract

This is the reviewer-facing companion for the stable lane that hardens the
**component-state registry** into a governed launch property: one governed record
per registry posture that binds **one registry value** behind every row,
**family coverage** across core controls / dense rows / tabs / trees / palettes /
popovers / dialogs / banners / job rows / inline notices, **normalized degraded
states** (disabled, blocked, policy-locked, reconnecting, warming, partial,
stale, recovering) that stay consistent across shell / review / settings /
support, **extension/embedded inheritance honesty** per appearance axis,
**token-driven shell-zoning semantics**, and **per-permutation state-fixture
coverage** — all to a public claim ceiling and an automatic
narrow-below-Stable verdict.

This lane is a settings-side certification that *projects* live contracts. It
does not render a component; it proves the shared component-state vocabulary
resolves and stays consistent across product and extension surfaces. It builds
on the design-system component-state registry
(`aureline_design_system::seeded_component_state_registry`), the live extension
appearance-conformance packet
(`aureline_extensions::appearance_conformance::seeded_appearance_conformance_packet`),
the design-system screenshot-diff packet
(`aureline_design_system::seeded_screenshot_diff_packet`), and the shared UI
taxonomy (`aureline_ui::components::ComponentStateClass`).

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity/`](../../../fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity/)
- Schema:
  [`/schemas/ux/harden-component-state-registry-and-theme-token-parity.schema.json`](../../../schemas/ux/harden-component-state-registry-and-theme-token-parity.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/harden-component-state-registry-and-theme-token-parity.md`](../../../artifacts/ux/m4/harden-component-state-registry-and-theme-token-parity.md)
- Typed source: `aureline_settings::component_state_registry_stable` (`model`, `corpus`)
- Headless emitter: `aureline_settings_component_state_registry_stable`
- Replay + invariant gate:
  `crates/aureline-settings/tests/component_state_registry_stable_fixtures.rs`

## Why one governed certification record

Component state is rendered by every launch-critical surface — and by
extension-contributed and webview-adjacent surfaces beside them. If each surface
re-invents what "disabled", "blocked", "policy-locked", "reconnecting",
"warming", "partial", "stale", or "recovering" looks like and says, the same
state forks by surface or theme: a focus ring vanishes in one place, a blocked
row reads as a spinner in another, an extension webview quietly drops the host's
high-contrast tokens, and the release packet ships a single happy-path screenshot
that hides all of it. The result is a green "component states are consistent"
claim that is really an average over surfaces that each diverge a little, with no
proof that extension gaps are disclosed or that every launch-critical permutation
was actually captured.

A `component_state_registry_certification_record` closes that gap. For one
registry posture it binds:

- **One registry value.** `registry_binding.value_ref` records the active registry
  id and revision and the shared `taxonomy_ref`. Every family, normalized-state,
  zone, and fixture row resolves against the same taxonomy, so no surface forks
  the vocabulary.
- **Family coverage.** One `component_families[]` entry per core control, dense
  row, tab, tree, palette, popover, dialog, banner, job row, and inline notice.
  Each declares its `supported_states`, `required_affordances`, and an
  `accessibility_note`, and proves it is `token_driven` with `focus_visible_preserved`
  and `screen_reader_semantics_preserved`. The union of `supported_states` must
  cover the full canonical vocabulary or the build refuses the record.
- **Normalized states.** One `normalized_states[]` entry per disabled, blocked,
  policy-locked, reconnecting, warming, partial, stale, and recovering state. Each
  resolves to the shared `taxonomy_ref`, carries a non-color cue, names a
  `narratable_reason` and an `action_path`, and is held `consistent_across_surfaces`
  for shell, review, settings, and support. Hue and animation alone are forbidden.
- **Extension inheritance honesty.** One `extension_inheritance[]` entry per
  appearance axis (theme, density, focus ring, high contrast, reduced motion, host
  token), projected from the live appearance-conformance packet. Each records the
  effective `support_class` and proves any gap surfaces in review
  (`gap_disclosed_in_review`), diagnostics (`gap_surfaced_in_diagnostics`), and
  support export (`gap_surfaced_in_support_export`). An undisclosed gap with no
  bounded `waiver_ref` is a hard build error.
- **Shell-zoning semantics.** One `shell_zones[]` entry per declared slot, proving
  the docked-versus-sheet `layout_mode`, the min/max chrome metrics, the density
  and reduced-motion semantics, and the placeholder card are token-driven rather
  than hard-coded. A zone that hard-codes a metric must carry a bounded
  `waiver_ref` and narrows the claim.
- **State-fixture coverage.** One `state_fixtures[]` entry per launch-critical
  surface/state permutation, projected from the screenshot-diff packet, proving
  every permutation has a stable `screenshot_ref` and `fixture_ref` with
  `focus_visible_preserved` and `screen_reader_semantics_preserved` through the
  transition — not one happy-path screenshot per component.

## The public claim ceiling and automatic narrowing

`pillars` are *derived* from the rows, never asserted. `claim_ceiling` records
what the posture is allowed to claim; the builder refuses to mint a record whose
ceiling exceeds its proof. `stable_qualification` then narrows the posture below
Stable with a named `narrowing_reasons[]` entry whenever a pillar fails or the
lowest family marker is below Stable — so a posture never inherits Stable by
adjacency.

| Narrowing reason | Meaning |
| --- | --- |
| `registry_family_coverage_incomplete` | A required family is missing or does not cover the vocabulary. |
| `state_normalization_inconsistent` | A normalized state's treatment forks by surface. |
| `extension_gap_undisclosed` | An extension axis gap is not disclosed everywhere. |
| `shell_zoning_hardcoded` | A shell zone hard-codes a chrome metric or placeholder. |
| `state_fixture_coverage_incomplete` | A launch-critical permutation lacks a conforming fixture. |
| `focus_or_screen_reader_regression` | Focus visibility or screen-reader semantics regress through a transition. |
| `hue_or_animation_only_cue` | A state relies on hue or animation alone. |
| `surface_not_yet_stable` | The lowest family marker is below Stable. |

## Reachability, accessibility, and availability

The same record is reachable from the settings / design-system panel, the command
palette, the status bar, and a menu command (`routes[]`), keyboard-first, and the
recovery routes (`recovery_routes[]`) hold across normal, high-contrast, and
zoomed layouts (`accessibility.layout_modes[]`). Every record stays available
without an account or managed services.

## Regenerating the records

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_component_state_registry_stable -- emit-fixtures \
  fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity
```

The replay gate
`crates/aureline-settings/tests/component_state_registry_stable_fixtures.rs` fails
if the on-disk JSON drifts from the in-code projection.
