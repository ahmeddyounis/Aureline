# M5 design-system style-drift lint and state-semantic audit

The **style-drift lint** is the conformance gate for Aureline's most trust-bearing
M5 shell surfaces. It is the checked-in answer to the residual launch risk: not
missing themes, but *surface-local styling and state drift* on the flows users
trust most. Where the
[contract matrix](m5-design-system-contract-matrix.md) governs *which*
design-system objects exist, the [foundation package](m5-foundation-package.md)
ships the *tokens*, the [component manifests](m5-component-manifest.md) ship the
durable *component contracts*, and the
[reference-layout package](m5-reference-layout-package.md) ships the *layouts*,
this lane ships the *gate*: a report that declares what each protected surface
consumes, and a lint pass that blocks promotion when a surface forks the design
system or lets a degraded state go unlabeled.

- Schema: [`schemas/design-system/m5-style-drift-lint.schema.json`](../../schemas/design-system/m5-style-drift-lint.schema.json)
- Canonical report: [`fixtures/ui/m5-style-drift-lint/lint-report.json`](../../fixtures/ui/m5-style-drift-lint/lint-report.json)
- Drill reports: [`fixtures/ui/m5-style-drift-lint/`](../../fixtures/ui/m5-style-drift-lint/)
- Lint-outcome proof: [`artifacts/release/m5-design-system-proof/style-drift-lint-outcome.json`](../../artifacts/release/m5-design-system-proof/style-drift-lint-outcome.json)
- Release packet: [`artifacts/release/m5-design-system-proof/style-drift-lint-release.json`](../../artifacts/release/m5-design-system-proof/style-drift-lint-release.json)
- Producer: `cargo run -p aureline-design-system --bin aureline_design_system_m5_style_drift_lint`

## Protected surfaces

The lint covers the four surface families most exposed to local-style and state
drift. The `surface_class` token is stable and shared with shell code and support
exports:

| `surface_class` | Surface |
| --------------- | ------- |
| `trust_prompt` | Trust / permission / capability prompt sheet |
| `onboarding_flow` | First-use and return-to-work onboarding flow |
| `notification_activity` | Notification envelope and durable activity-center surface |
| `embedded_boundary` | Embedded-surface boundary naming route, trust, and capability |

Each surface declares the foundation `token_usages` it consumes, the
`local_style_forks` it carries (none on a conformant surface), the
`state_bindings` it renders for the protected states, and any `waivers` in play.

## What the lint flags

The lint pass over a report emits one finding per drift, each a blocking error
unless suppressed by an active waiver:

| Check id | Catches |
| -------- | ------- |
| `style_drift.unmanaged_token_value` | A `token_ref` that is a raw literal (a hex color, a raw dimension, an `rgb(...)` / `hsl(...)` call) or that does not resolve into a governed foundation namespace |
| `style_drift.forbidden_local_style_fork` | Any local style override on a protected surface |
| `style_drift.missing_state_semantic_binding` | A protected state (`loading`, `pending`, `degraded`, `blocked`) the surface does not bind |
| `state_semantic.unlabeled_state` | A protected state without a visible label and a screen-reader label |
| `state_semantic.color_only_state_meaning` | A protected state whose only cue is color (no non-color cue) |
| `state_semantic.spinner_only_state` | A `pending` / `degraded` / `blocked` state carried by a spinner alone (`loading` is the exempt spinner affordance) |
| `state_semantic.hover_only_critical_action` | A protected state hiding a critical action or its reason behind hover only |
| `waiver.unused` | A well-formed waiver that suppresses no finding — a non-blocking warning so reviewers prune stale waivers |

A token reference is **managed** when it resolves into a governed foundation
namespace (`al.color.*`, `space.*`, `typography.*`, `icon.*`, `motion_*`, …); any
raw inline value is unmanaged and rejected.

## Gate decision

The lint resolves an overall and a per-surface `gate_decision`:

| Gate | When |
| ---- | ---- |
| `pass` | No findings |
| `pass_with_disclosed_gap` | Every error finding is suppressed by an active, proof-tied waiver |
| `warn` | Only non-blocking warnings remain (e.g. an unused waiver) |
| `block` | Any unwaived error finding remains |

`block` fails CI: the producer's `lint` subcommand exits non-zero and names the
blocked surfaces, so a new local-style fork or an unlabeled degraded state on a
protected flow produces a reviewable failure rather than shipping silently.

## Waivers are explicit, time-bounded, and proof-tied

A finding is suppressed only by an `M5StyleDriftWaiver` that:

- names exactly one suppressible `waived_check_id` (and may narrow to a
  `waived_state_class` or `waived_subject_id`),
- carries an `expires_at` — the waiver stops suppressing once the report's
  `evaluated_at` reaches it, so a waiver cannot outlive its window, and
- carries a `proof_packet_ref` under
  `artifacts/release/m5-design-system-proof/`, tying the exception to a
  design-system proof packet.

A waiver missing its proof packet or carrying an unknown check id is rejected by
validation; an **expired** waiver is structurally valid but suppresses nothing,
so the surface still blocks. The checked-in
[`lint-report-waived.json`](../../fixtures/ui/m5-style-drift-lint/lint-report-waived.json)
and
[`lint-report-expired-waiver.json`](../../fixtures/ui/m5-style-drift-lint/lint-report-expired-waiver.json)
drills exercise both paths.

## Drift drills

Three checked-in drills accompany the conformant report so the gate behavior is
itself proven:

| Drill | Gate |
| ----- | ---- |
| [`lint-report.json`](../../fixtures/ui/m5-style-drift-lint/lint-report.json) (conformant) | `pass` |
| [`lint-report-drift.json`](../../fixtures/ui/m5-style-drift-lint/lint-report-drift.json) (unmanaged token, local fork, dropped + regressed states) | `block` |
| [`lint-report-waived.json`](../../fixtures/ui/m5-style-drift-lint/lint-report-waived.json) (drift + active proof-tied waivers) | `pass_with_disclosed_gap` |
| [`lint-report-expired-waiver.json`](../../fixtures/ui/m5-style-drift-lint/lint-report-expired-waiver.json) (drift + expired waivers) | `block` |

The report, the drills, the outcome proof, and the release packet are minted by
the same seed builder and asserted by inline tests, so the checked-in artifacts
and the in-code gate never drift.
