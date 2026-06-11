# M5 component-state and design-token inheritance parity (companion doc)

This page is the companion to the M5 component-state and design-token
inheritance audit. It carries the stable v1 shell promise forward into
the M5 depth lanes: every new pane — notebook cell chrome, result-grid
rows, profiler and trace panels, pipeline cards, preview-route badges,
docs/browser panes, companion surfaces, sync status, and offboarding —
must inherit the same design-token system and the same normalized
component-state vocabulary as the v1 shell, and must never encode
severity, trust, or lifecycle meaning through a one-off color or a private
badge outside the shared registry.

The audit data, the per-row blocking findings, the per-state coverage
numbers, and the registry-anchor index all come from one mint-from-truth
path — the seeded audit in
[`crate::m5_component_registry`](../../crates/aureline-shell/src/m5_component_registry/mod.rs)
— so the live shell design-QA inspector, the CLI/headless inspector, the
support-export wrapper, the cross-surface hardening matrix, and the CI
gate never disagree on what styling and state vocabulary each M5 surface
inherits.

Authoritative artifacts:

- [`/artifacts/ux/m5/component-state-audit/m5_component_state_audit.md`](../../artifacts/ux/m5/component-state-audit/m5_component_state_audit.md)
  — the rendered audit generated from the seeded projection.
- [`/fixtures/ux/m5/theme-token-consumers/report.json`](../../fixtures/ux/m5/theme-token-consumers/report.json)
  — the JSON snapshot of the same record consumed by every surface.
- [`/fixtures/ux/m5/theme-token-consumers/support_export.json`](../../fixtures/ux/m5/theme-token-consumers/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/ux/m5-component-state.schema.json`](../../schemas/ux/m5-component-state.schema.json)
  — the boundary schema the fixtures conform to.

## The nine normalized component states

The audit covers exactly the nine normalized states every M5 surface must
render consistently:

| State | Meaning |
| ----- | ------- |
| `loading` | Work in flight; a determinate or indeterminate loading treatment. |
| `cached` | Content served from a local cache rather than a fresh fetch. |
| `stale` | Content known to be out of date relative to its source. |
| `partial` | A partial result set (truncated, sampled, or still streaming). |
| `policy_blocked` | The action or content is blocked by a policy or capability gate. |
| `degraded` | The surface is operating in a reduced-capability degraded mode. |
| `preview_only` | The content is preview-only and not yet applied or committed. |
| `sync_pending` | A durable sync or publish is pending for this surface. |
| `boundary_handoff` | The surface is handing off across an embedded or device boundary. |

For every registered M5 surface, each state carries a binding. The
binding status is one of:

- `inherited` — the surface inherits the shared registry token and state
  class and quotes the canonical descriptor verbatim (token group, token
  ref, style provenance, cue policy, registry anchor).
- `explicitly_narrowed` — the surface narrows this state but names a
  `narrowing_reason`.
- `not_applicable` — the state does not apply to this surface; a reason is
  named (e.g. `result_rows_do_not_cross_an_embedded_boundary`).
- `declared_inheritance_gap` — an extension- or provider-backed surface
  declares a known inheritance gap honestly, with a reason, instead of
  quietly drifting away from the shell token system.
- `platform_omitted` — the state is not surfaced on this client/platform;
  a reason is named.
- `unregistered_local_state` — the surface renders the state through
  ad-hoc local semantics outside the shared registry. **Always blocking.**
- `unknown_token_gap` — the required state has a missing or unknown token
  binding. **Always blocking** for any high-salience surface.

A surface is "high-salience" when its descriptor pins a semantic salience
of `lifecycle_bearing`, `trust_bearing`, or `severity_bearing` — i.e. it
conveys lifecycle, trust, or severity meaning. A high-salience surface
MUST stay registered, token-driven, and carry a non-color cue.

## Token inheritance and accessibility honesty

Every descriptor pins a `token_group`, a `style_provenance`, and a
`cue_policy` so the audit can prove each surface inherits the shared
token system and never encodes state meaning through color alone:

- Every `inherited` binding MUST project `token:<token_group>:<state>` —
  the canonical token ref derived from the surface's token group and the
  normalized state. A divergent ref is a `token_ref_drift` blocker, and a
  `null` ref is a `missing_projection` blocker, so a surface can never
  paint a state from a private token or a literal value.
- A binding that carries a `hardcoded_value` is a `hardcoded_theme_value`
  blocker, and a binding that falls back to an `unresolved_token_fallback`
  is an `unresolved_token_fallback` blocker.
- `cue_policy = non_color_cue_required` means state meaning MUST carry a
  non-color cue (`icon_and_text`, `text_label`, or `shape_or_pattern`).
  A `color_only` cue on a high-salience surface (or any surface that
  requires a non-color cue) is a `color_only_cue` blocker, and a
  high-salience surface that declares `color_allowed` is a
  `missing_non_color_cue_policy` blocker.
- `style_provenance` records whether a surface inherits the shell tokens
  directly, documents a scoped override, or is extension-contributed or
  provider-backed. Extension/provider surfaces use
  `declared_inheritance_gap` bindings to surface their gaps in review,
  diagnostics, and support export rather than drifting silently.

## What the validator rejects

The audit fails the gate when any blocking finding remains:

- `unknown_token_gap`, `unregistered_local_state` — unregistered or
  pane-local state semantics outside the shared registry.
- `token_group_drift`, `token_ref_drift`, `style_provenance_drift`,
  `cue_policy_drift`, `override_drift` — an inherited binding that
  disagrees with the canonical descriptor or the shared token system.
- `color_only_cue`, `missing_non_color_cue_policy` — inaccessible
  color-only cues.
- `hardcoded_theme_value`, `unresolved_token_fallback` — a hard-coded
  theme literal or an unresolved token fallback that would block
  certification.
- `missing_registry_anchor`, `descriptor_missing_registry_anchor` — an
  inherited binding or the descriptor that cannot point back to its
  shared-registry entry.
- `missing_accessibility_note` — a descriptor with no accessibility note.
- `surface_not_registered` — a surface not yet registered in the shared
  component-state registry.
- `missing_narrowing_reason`, `missing_projection` — a non-inherited row
  with no reason, or an inherited row missing a projected field.

## Consuming the audit

The cross-surface hardening matrix and later release-center, docs/help,
and support-export surfaces ingest the checked-in `report.json` directly
when qualifying or narrowing an M5 row instead of cloning status text.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- validate
cargo test -p aureline-shell --test m5_component_state_fixtures
python3 tools/ci/m5/component_state_check.py
```
