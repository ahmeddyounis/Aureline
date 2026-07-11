# M5 adaptive-efficiency component surface certification contract (M05-1067)

This is the closing **surface-certification capstone** for the B126 adaptive-efficiency
component lane. Where the freeze matrix defines the eight reusable components, the
implement lanes narrow each one, the consumer lane adopts them, and the accessibility
lane proves keyboard / screen-reader / reduced-motion / high-contrast / CLI-export
parity, this capstone *certifies* that the shared component truth holds on every claimed
M5 adaptive-efficiency surface — and auto-narrows any surface that cannot sustain it.

- **Schema:** [`schemas/ui/m5-efficiency-component-certification.schema.json`](../../schemas/ui/m5-efficiency-component-certification.schema.json)
- **Canonical proof bundle:** `artifacts/release/m5-efficiency-component-certification-proof/support_export.json`
- **Cited efficiency-proof bundle (one, shared):** `artifacts/release/m5-efficiency-components-proof/support_export.json`
- **Accessibility evidence:** `artifacts/release/m5-efficiency-component-accessibility-proof/support_export.json`

## What is certified

The packet is keyed on the **claimed surface** a user, operator, or support engineer
reads adaptive-efficiency truth through, not on the reusable component family it renders.
The eight certified surfaces are:

| Surface | Reads |
| --- | --- |
| `shell_status_bar` | power-state indicator, throttled subsystems |
| `activity_center` | background-work rows and banners |
| `work_content_canvas` | slowed / paused work on notebook / preview / pipeline / graph |
| `policy_aware_settings` | per-workspace override sheets, policy notes |
| `incident_diagnostics` | constrained-state truth during incidents |
| `docs_help` | documented component truth |
| `support_export` | replayable support / export bundle |
| `cli_headless` | text / JSON / Markdown automation output |

Every frozen component family — power-state indicator, throttled-subsystem row,
background-work row, background-work banner, per-workspace override sheet,
override-policy note row, resume-summary card, and stale-result-continuity note — is
certified on at least one surface.

## The six truth axes

Each surface is scored on exactly six axes, each appearing once:

1. **`visual`** — source of change, active efficiency state, slowed-versus-paused work,
   what still works, override availability, policy owner, resumed backlog, and
   stale-result continuity are shown on-surface.
2. **`keyboard`** — the same inspect / override / resume actions are keyboard-reachable.
3. **`screen_reader`** — the same truth is announced non-visually, never color/glyph-only.
4. **`cli_export`** (always-on) — the surface state is reconstructable as text / JSON /
   Markdown for support and automation. This axis must always certify.
5. **`degraded_state`** — a stale, deferred, or partial reading honestly downgrades a
   `full_truth` / `resolved_truth` claim rather than reading current.
6. **`efficiency_truth`** — source of change, slowed-versus-paused work, override
   availability, policy owner, resumed-work backlog, and stale-result continuity stay
   explicit and never collapse into one generic low-power warning, hide paused work
   behind a toast, present a blocked override as available, or clear stale-result
   context on resume.

## The invariant: a degraded axis must produce a visible claim narrowing

The efficiency-support claim ceiling ranks, strongest first: `full_truth` (5),
`resolved_truth` (4), `degraded` (3), `deferred` (2), `stale_shown` (1),
`policy_blocked` (0).

- **Green** — every axis certified, the claimed ceiling delivered.
- **Yellow** — an axis is `disclosed_narrowed` with a bound reason and a frozen downgrade
  trigger, and the surface visibly narrows its claim from `claimed_claim` to a weaker
  `certified_claim` via `claim_auto_narrow` (bound to the narrowed, non-always-on axis,
  with a non-generic label).
- **Red** — a degraded axis is hidden behind a full claim (`undisclosed_drift`, or a
  disclosed narrowing with no claim reduction), the CLI/export axis drops, the copy /
  export parity is incomplete, the certified claim exceeds the claimed one, or the
  narrowing is inconsistent. Red surfaces are not publishable.

The stored `derived_status` is always recomputed and compared on validation, so the
verdict can never be hand-asserted.

## Metadata-only boundary

Raw device telemetry, credentials, and policy secrets never cross this boundary. The
validator rejects any export carrying obviously forbidden material.

## Regenerating the proof

```
GEN_EFFICIENCY_CERT_ARTIFACTS=1 cargo test -p aureline-shell \
  certify_power_state...::tests::generate_artifacts
```

This writes the canonical `support_export.json`, `matrix.csv`, and `report.md` under
`artifacts/release/m5-efficiency-component-certification-proof/` and mirrors them into
`fixtures/ui/m5-efficiency-component-certification/`. The `checked_in_export_matches_seeded_builder`
test byte-locks the on-disk export to the seeded builder.
