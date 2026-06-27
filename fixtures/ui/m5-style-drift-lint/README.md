# M5 design-system style-drift-lint fixtures

These fixtures are valid, export-safe style-drift-lint reports for the four
trust-bearing M5 shell surfaces (trust prompt, onboarding flow, notification /
activity center, and embedded-surface boundary). Each declares the foundation
tokens the surface consumes, the local style forks it carries, the protected-state
semantic bindings it renders, and any waivers. They are minted from the same seed
builder as the lint-outcome proof by `aureline_design_system_m5_style_drift_lint`,
and the inline tests assert the checked-in files match the seed and gate as the
table below expects. See [`docs/design-system/m5-style-drift-lint.md`](../../../docs/design-system/m5-style-drift-lint.md).

## lint-report.json

The canonical, conformant report. Every surface consumes only governed foundation
tokens, carries no local style forks, and binds each protected state (`loading`,
`pending`, `degraded`, `blocked`) with a label and a non-color cue. The lint pass
is green (`pass`). This is the report `style-drift-lint-outcome.json` and
`style-drift-lint-release.json` are computed from.

## lint-report-drift.json

The trust-prompt surface develops drift: two unmanaged token values (a raw hex
color and a raw dimension), a forbidden local style fork, a dropped `degraded`
binding, and a `blocked` binding regressed to color-only, spinner-only, and
hover-only. The report is structurally valid — the gate, not validation, rejects
the drift — and the lint pass `block`s, naming the trust-prompt surface. The drift
is named, not hidden. Demonstrates the failure a protected-surface drift produces.

## lint-report-waived.json

The same drift as `lint-report-drift.json`, accepted under explicit waivers that
are active (unexpired as of `evaluated_at`) and tied to a design-system proof
packet. Every finding is suppressed by a named waiver, so the lint passes with a
disclosed gap (`pass_with_disclosed_gap`) without hiding the drift. Demonstrates
the time-bounded, proof-tied waiver path.

## lint-report-expired-waiver.json

The same drift and the same waivers as `lint-report-waived.json`, except the
waivers have already expired as of `evaluated_at`. An expired waiver is
structurally valid but suppresses nothing, so the lint still `block`s.
Demonstrates that a waiver cannot outlive its window.
