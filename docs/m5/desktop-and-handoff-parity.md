# M5 desktop and handoff parity (companion doc)

This page is the companion to the M5 desktop-and-handoff qualification audit.
It carries the stable v1 shell promise forward into the M5 depth lanes: every
new pane — notebook cell chrome, result-grid rows, profiler and trace panels,
pipeline cards, preview-route badges, docs/browser panes, companion surfaces,
sync status, offboarding, and incident packets — must behave like a
first-class desktop citizen across the real desktop conditions Aureline
already claims, instead of only working on a single window, a single monitor,
one DPI class, and one uninterrupted happy path. No window, display,
power-state, or handoff change may silently lose target identity, break layout
continuity, corrupt typing or save, or drop an authority context.

The audit data, the per-surface blocking findings, the per-row coverage
numbers, the reopen-anchor index, and the narrowable-marketed-rows list all
come from one mint-from-truth path — the seeded audit in
[`crate::m5_desktop_conformance`](../../crates/aureline-shell/src/m5_desktop_conformance/mod.rs)
— so the live shell platform-conformance inspector, the CLI/headless
inspector, the support-export wrapper, the cross-surface hardening matrix, the
release-center packets, and the CI gate never disagree on what each M5 surface
certifies across the desktop scenario rows.

Authoritative artifacts:

- [`/artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md`](../../artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md)
  — the rendered audit generated from the seeded projection.
- [`/fixtures/platform/m5_depth_surfaces/report.json`](../../fixtures/platform/m5_depth_surfaces/report.json)
  — the JSON snapshot of the same record consumed by every surface.
- [`/fixtures/platform/m5_depth_surfaces/support_export.json`](../../fixtures/platform/m5_depth_surfaces/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/platform/m5-surface-desktop-qualification.schema.json`](../../schemas/platform/m5-surface-desktop-qualification.schema.json)
  — the boundary schema the fixtures conform to.

## The nine desktop scenario rows

The audit certifies exactly the nine desktop scenario rows every marketed M5
surface must pass, grouped by the desktop dimension Aureline already claims:

| Row | Dimension | Meaning |
| --- | --------- | ------- |
| `multi_window` | window_topology | The surface stays coherent across multiple top-level windows. |
| `multi_monitor` | window_topology | The surface stays coherent across multiple monitors. |
| `mixed_dpi` | window_topology | The surface stays coherent across mixed-DPI displays. |
| `suspend_resume` | power_state | The surface reopens its exact target truthfully after a suspend/resume cycle. |
| `battery_saver` | power_state | Battery saver throttles background work before it can corrupt foreground work. |
| `thermal_pressure` | power_state | Thermal pressure throttles background work before it can corrupt foreground work. |
| `deep_link` | handoff | A deep link reopens the exact target with its authority context and handoff reason. |
| `file_association` | handoff | A file association reopens the exact target with its authority context and handoff reason. |
| `system_open_return` | handoff | A system-open or browser return lands truthfully with its authority context preserved. |

These are the desktop conditions Aureline already claims — the audit certifies
and hardens them; it does not broaden the supported desktop-platform matrix.

For every registered M5 surface, each row carries a qualification binding. The
qualification status is one of:

- `qualified` — the row is qualified with captured desktop evidence and quotes
  the canonical evidence fields (evidence pack, reopen fidelity, layout
  continuity, interruption safety, placeholder honesty, evidence freshness,
  plus a background-adaptation result on the power rows and a handoff-reason
  and authority-context result on the handoff rows).
- `explicitly_narrowed` — the surface narrows this row but names a
  `narrowing_reason`.
- `not_applicable` — the row does not apply to this surface; a reason is named.
- `platform_omitted` — the row is not surfaced on this client/profile; a reason
  is named.
- `declared_capture_gap` — an extension- or provider-backed surface declares a
  known capture gap honestly, with a reason, instead of silently shipping an
  un-qualified row.
- `unqualified_local_platform_path` — the surface drives this row through an
  ad-hoc local window/restore path outside the shared platform-conformance
  harness. **Always blocking.**
- `missing_evidence` — a marketed row is claimed with no captured evidence.
  **Always blocking.**

A surface is "high-salience" when its descriptor pins a semantic salience of
`lifecycle_bearing`, `trust_bearing`, or `severity_bearing` — i.e. it conveys
lifecycle, trust, or severity meaning. A high-salience surface MUST keep a
present high-risk boundary cue on every qualified row, so no desktop scenario
can hide that meaning.

## Captured evidence and continuity honesty

Every qualified row projects the captured evidence the row requires so the
audit can prove the surface was certified against the real desktop condition,
not just the happy path:

- An evidence-pack ref, reopen fidelity, layout continuity, interruption
  safety, and placeholder honesty are required on **every** qualified row. A
  missing field is a `missing_projection` blocker; a red result
  (`reopen_target_lost`, `layout_continuity_broken`, `interruption_unsafe`,
  `placeholder_misleading`, `boundary_cue_hidden`) is a blocker in its own
  class.
- The power rows (`battery_saver`, `thermal_pressure`) additionally require a
  background-adaptation result; a `not_throttled` result is a
  `background_not_throttled` blocker, proving battery or thermal adaptation
  slows background work before it can corrupt typing, save, reopen, or
  exact-target attention behaviour.
- The handoff rows (`deep_link`, `file_association`, `system_open_return`)
  additionally require a handoff-reason and an authority-context result; a
  `dropped` reason is a `handoff_reason_dropped` blocker and a `lost` authority
  context is an `authority_context_lost` blocker, so a reopen or return can
  never silently lose target identity or authority context.
- Stale evidence on a marketed row is a `stale_evidence_on_marketed_row`
  blocker, so a marketed row whose drills have aged out narrows instead of
  shipping as implicitly stable.

## What the validator rejects

The audit fails the gate when any blocking finding remains:

- `unqualified_local_platform_path`, `missing_evidence` — an ad-hoc local
  window/restore path outside the shared platform-conformance harness, or a
  marketed row with no evidence.
- `reopen_target_lost`, `layout_continuity_broken`, `interruption_unsafe`,
  `placeholder_misleading`, `authority_context_lost`, `background_not_throttled`,
  `handoff_reason_dropped`, `boundary_cue_hidden` — a red captured result.
- `stale_evidence_on_marketed_row` — stale evidence on a marketed row.
- `dimension_drift`, `missing_narrowing_reason`, `missing_projection`,
  `missing_evidence_pack` — a binding whose dimension disagrees with its row, a
  non-qualified row with no reason, or a qualified row missing a required
  captured-evidence field.
- `descriptor_missing_reopen_anchor`, `missing_continuity_note`,
  `missing_claimed_profiles`, `surface_not_on_platform_conformance` — a
  descriptor with no reopen anchor, no continuity note, or no claimed desktop
  profile, or a surface that drives its own window/restore path outside the
  shared platform-conformance harness.

## Consuming the audit and narrowing marketed rows

The cross-surface hardening matrix and the release-center packets ingest the
checked-in `report.json` directly when qualifying or narrowing a marketed M5
row instead of cloning status text. The report's `narrowable_marketed_rows`
list names every marketed surface row whose desktop evidence is stale or red,
so release tooling can narrow that marketed M5 row on the affected profiles
before publication instead of shipping the surface as implicitly stable. In the
clean checked-in audit that list is empty.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_conformance -- validate
cargo test -p aureline-shell --test m5_desktop_conformance_fixtures
python3 tools/ci/m5/desktop_conformance_check.py
```
