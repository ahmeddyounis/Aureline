# Component-state registry certification — release evidence

Reviewer-facing evidence packet for the lane that hardens the **component-state
registry** across shell, editor, panel, popover, dialog, and extension-adjacent
surfaces: one canonical record per registry posture that binds one registry value
behind every row, family coverage across the ten component families, normalized
degraded-state treatments held consistent across shell / review / settings /
support, extension/embedded inheritance honesty per appearance axis with every
gap surfaced in review / diagnostics / support export, token-driven shell-zoning
and responsive-fallback semantics, per-permutation state-fixture coverage, a
public claim ceiling, an automatic narrow-below-Stable verdict, recovery and route
parity across the settings / design-system panel / command palette / status bar /
menus, accessibility across normal / high-contrast / zoomed layouts, and rows that
stay available without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity/`](../../../fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity/)
- Schema: [`/schemas/ux/harden-component-state-registry-and-theme-token-parity.schema.json`](../../../schemas/ux/harden-component-state-registry-and-theme-token-parity.schema.json)
- Companion doc: [`/docs/ux/m4/harden-component-state-registry-and-theme-token-parity.md`](../../../docs/ux/m4/harden-component-state-registry-and-theme-token-parity.md)
- Typed source: `aureline_settings::component_state_registry_stable` (`model`, `corpus`)
- Headless emitter: `aureline_settings_component_state_registry_stable`
- Replay + invariant gate: `crates/aureline-settings/tests/component_state_registry_stable_fixtures.rs`

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `nominal.json` | nominal | **stable** | stable | — |
| `popover_family_in_preview.json` | popover family in preview | preview (narrowed) | preview | `surface_not_yet_stable` |
| `hardcoded_zoning_drill.json` | hard-coded shell-zoning drill | beta (narrowed) | stable | `shell_zoning_hardcoded` |
| `extension_gap_undisclosed_drill.json` | extension gap undisclosed drill | beta (narrowed) | stable | `extension_gap_undisclosed` |

Coverage verdict: **1 Stable, 3 narrowed**. Each narrowed row names a reason and
drops below the launch cutline rather than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **The component-state registry is checked in, consumed by core shell
  components, and referenced by extension-surface guidance and design QA packets.**
  The record is minted by `aureline_settings::component_state_registry_stable` from
  the live `aureline_design_system::seeded_component_state_registry`;
  `registry_binding` cites its source-of-truth refs and the shared `taxonomy_ref`,
  and `upstream` cites the design-system registry, the extension
  appearance-conformance packet, and the screenshot-diff packet it projects.
- **State-rich fixtures demonstrate parity for normal, disabled, loading, warning,
  error, blocked, and recovering states across shell, settings, and notification
  surfaces.** `component_families[]` covers all ten families and the union of
  `supported_states` covers the full canonical vocabulary; `normalized_states[]`
  proves disabled / blocked / policy-locked / reconnecting / warming / partial /
  stale / recovering each stay consistent across shell, review, settings, and
  support; `state_fixtures[]` covers every launch-critical surface/state
  permutation from the screenshot-diff packet.
- **Extension-bearing or webview-adjacent surfaces that cannot inherit stable
  tokens or state semantics are explicitly downgraded or labeled rather than
  silently drifting.** `extension_inheritance[]` records the effective
  `support_class` per appearance axis projected from the live conformance packet,
  and proves any gap surfaces in review, diagnostics, and support export. The
  `extension_gap_undisclosed_drill.json` posture shows the lane refusing a gap that
  is not surfaced in the support export and narrowing to `beta`.
- **Design QA and accessibility packets flag state-semantic drift as
  release-bearing defects on claimed stable rows.** `pillars` are derived from the
  rows; `claim_ceiling` cannot exceed the proof; `stable_qualification` narrows the
  posture below Stable with a named reason whenever a pillar fails or the lowest
  family marker is below Stable. `honesty_marker_present` is `true` on every
  narrowed posture.
- **State-fixture and token-parity audits prove shell metrics, density, motion,
  and placeholder states remain consistent across first-party, extension-contributed,
  and embedded surfaces.** `shell_zones[]` proves slot names, docked-versus-sheet
  state, min/max chrome metrics, density semantics, reduced-motion behavior, and
  placeholder cards are token-driven; the `hardcoded_zoning_drill.json` posture
  shows the lane detecting a hard-coded metric and narrowing to `beta`.

## Guardrails honored

- Component states cannot fork by surface or theme: `normalized_states[]` binds
  each state to the shared `taxonomy_ref` and requires consistency across all four
  surfaces, and `registry_binding` ties every row to one registry value.
- No state relies on hue or animation alone: every family and normalized state
  sets `hue_only_forbidden` and `animation_only_forbidden`, and the
  `no_hue_or_animation_only` pillar is derived from them.
- A surface, zone, or axis that cannot honor the registry narrows the claim (with a
  `waiver_ref`) instead of leaking bespoke styling or an undisclosed gap into
  Stable.
- The record stays available without an account or managed services
  (`available_without_account`, `available_without_managed_services`).

## Reproduce

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_component_state_registry_stable -- index

cargo run -q -p aureline-settings \
  --bin aureline_settings_component_state_registry_stable -- emit-fixtures \
  fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity

cargo test -p aureline-settings --test component_state_registry_stable_fixtures
```
