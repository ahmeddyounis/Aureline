# M5 appearance and density parity (companion doc)

This page is the companion to the M5 appearance-and-density qualification
audit. It carries the stable v1 shell promise forward into the M5 depth
lanes: every new pane — notebook cell chrome, result-grid rows, profiler
and trace panels, pipeline cards, preview-route badges, docs/browser panes,
companion surfaces, sync status, and offboarding — must stay legible,
controllable, and semantically consistent across the supported appearance
and density rows Aureline already claims, instead of only looking correct in
one default desktop theme. No appearance mode may hide trust, severity, or
lifecycle state, lose focus visibility, or corrupt layout on a live
appearance change.

The audit data, the per-surface blocking findings, the per-row coverage
numbers, the appearance-anchor index, and the narrowable-marketed-rows list
all come from one mint-from-truth path — the seeded audit in
[`crate::m5_appearance_parity`](../../crates/aureline-shell/src/m5_appearance_parity/mod.rs)
— so the live shell design-QA inspector, the CLI/headless inspector, the
support-export wrapper, the cross-surface hardening matrix, the
release-center packets, and the CI gate never disagree on what each M5
surface certifies across the appearance and density rows.

Authoritative artifacts:

- [`/artifacts/ux/m5/appearance-parity/m5_appearance_parity_audit.md`](../../artifacts/ux/m5/appearance-parity/m5_appearance_parity_audit.md)
  — the rendered audit generated from the seeded projection.
- [`/fixtures/ux/m5/dark-light-hc-density-reduced-motion/report.json`](../../fixtures/ux/m5/dark-light-hc-density-reduced-motion/report.json)
  — the JSON snapshot of the same record consumed by every surface.
- [`/fixtures/ux/m5/dark-light-hc-density-reduced-motion/support_export.json`](../../fixtures/ux/m5/dark-light-hc-density-reduced-motion/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/ux/m5-appearance-qualification.schema.json`](../../schemas/ux/m5-appearance-qualification.schema.json)
  — the boundary schema the fixtures conform to.

## The eight appearance rows

The audit certifies exactly the eight appearance rows every marketed M5
surface must pass, grouped by the appearance dimension Aureline already
claims:

| Row | Dimension | Meaning |
| --- | --------- | ------- |
| `theme_dark` | theme | The surface is legible and semantically intact in the dark theme. |
| `theme_light` | theme | The surface is legible and semantically intact in the light theme. |
| `theme_high_contrast` | theme | The surface meets contrast in the high-contrast theme. |
| `density_compact` | density | The surface stays usable at compact density. |
| `density_standard` | density | The surface stays usable at standard density. |
| `density_comfortable` | density | The surface stays usable at comfortable density. |
| `reduced_motion` | motion | The surface downgrades animation when reduced motion is requested. |
| `live_appearance_change` | live_change | A live appearance change does not corrupt layout, state, or focus. |

These are the appearance modes Aureline already claims — the audit certifies
and hardens them; it does not introduce new appearance modes.

For every registered M5 surface, each row carries a qualification binding.
The qualification status is one of:

- `qualified` — the row is qualified with captured appearance evidence and
  quotes the canonical evidence fields (screenshot pack, focus visibility,
  state semantics, keyboard and screen-reader checks, reopen affordance,
  boundary cue, evidence freshness, plus a contrast result for the theme
  rows, a motion downgrade for `reduced_motion`, and intact layout for
  `live_appearance_change`).
- `explicitly_narrowed` — the surface narrows this row but names a
  `narrowing_reason`.
- `not_applicable` — the row does not apply to this surface; a reason is
  named.
- `platform_omitted` — the row is not surfaced on this client/platform; a
  reason is named.
- `declared_capture_gap` — an extension- or provider-backed surface declares
  a known capture gap honestly, with a reason, instead of silently shipping
  an un-qualified row.
- `unqualified_local_appearance` — the surface paints this row through ad-hoc
  local styling outside the shared appearance-session model. **Always
  blocking.**
- `missing_evidence` — a marketed row is claimed with no captured evidence.
  **Always blocking.**

A surface is "high-salience" when its descriptor pins a semantic salience of
`lifecycle_bearing`, `trust_bearing`, or `severity_bearing` — i.e. it conveys
lifecycle, trust, or severity meaning. A high-salience surface MUST keep a
present high-risk boundary cue and preserved state semantics on every
qualified row, so no appearance mode can hide that meaning.

## Captured evidence and accessibility honesty

Every qualified row projects the captured evidence the row requires so the
audit can prove the surface was certified against the row, not just the
default theme:

- A screenshot pack ref, focus visibility, preserved state semantics, and
  captured keyboard and screen-reader checks are required on **every**
  qualified row. A missing field is a `missing_projection` blocker; a red
  result (`focus_not_visible`, `state_semantics_lost`, `keyboard_check_failed`,
  `screen_reader_check_failed`, `reopen_target_lost`, `boundary_cue_hidden`)
  is a blocker in its own class.
- The theme rows additionally require a contrast result; a `below_threshold`
  result is a `contrast_below_threshold` blocker.
- The `reduced_motion` row additionally requires a motion treatment; an
  `animated` treatment is a `motion_not_downgraded` blocker.
- The `live_appearance_change` row additionally requires intact layout; a
  `corrupted` layout, a lost focus, or lost state semantics become the
  `live_change_layout_corruption`, `live_change_focus_loss`, and
  `live_change_state_corruption` blockers so a runtime appearance change can
  never silently break a surface.
- Stale evidence on a marketed row is a `stale_evidence_on_marketed_row`
  blocker, so a marketed row whose captures have aged out narrows instead of
  shipping as implicitly stable.

## What the validator rejects

The audit fails the gate when any blocking finding remains:

- `unqualified_local_appearance`, `missing_evidence` — ad-hoc local
  appearance outside the shared appearance-session model, or a marketed row
  with no evidence.
- `contrast_below_threshold`, `focus_not_visible`, `state_semantics_lost`,
  `keyboard_check_failed`, `screen_reader_check_failed`, `reopen_target_lost`,
  `boundary_cue_hidden`, `motion_not_downgraded` — a red captured result.
- `live_change_layout_corruption`, `live_change_focus_loss`,
  `live_change_state_corruption` — a live appearance change that corrupts
  layout, focus, or state.
- `stale_evidence_on_marketed_row` — stale evidence on a marketed row.
- `dimension_drift`, `missing_narrowing_reason`, `missing_projection` — a
  binding whose dimension disagrees with its row, a non-qualified row with no
  reason, or a qualified row missing a required captured-evidence field.
- `descriptor_missing_appearance_anchor`, `missing_accessibility_note`,
  `surface_not_on_appearance_session` — a descriptor with no appearance
  anchor or accessibility note, or a surface that paints its own appearance
  outside the shared appearance-session model.

## Consuming the audit and narrowing marketed rows

The cross-surface hardening matrix and the release-center packets ingest the
checked-in `report.json` directly when qualifying or narrowing a marketed M5
row instead of cloning status text. The report's `narrowable_marketed_rows`
list names every marketed surface row whose appearance evidence is stale or
red, so release tooling can narrow that marketed M5 row before publication
instead of shipping the surface as implicitly stable. In the clean checked-in
audit that list is empty.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_parity -- validate
cargo test -p aureline-shell --test m5_appearance_parity_fixtures
python3 tools/ci/m5/appearance_parity_check.py
```
