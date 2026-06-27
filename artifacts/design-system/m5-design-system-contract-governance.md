# M5 design-system contract governance

This is the checked-in human-readable **design-system contract matrix**: the
authoritative inventory of every governed design-system object Aureline ships
for the claimed M5 surfaces, each named with its accountable owner, first
consumer, canonical artifact, the release packet that keeps it current, and the
proof lane that blocks drift. It is generated from the same checked-in matrix
the product ships (`artifacts/release/m5-design-system-proof/support_export.json`);
the machine-readable source of truth is that packet, and shell, help,
onboarding, presentation, the extension SDK, release center, QA, support
exports, and the stable-claim matrix consume it directly.

Later M5 families point at one object id in this inventory instead of
re-describing shell/component behavior in local docs. Stable promotion fails
when a claimed M5 surface lacks a mapped contract object or current
design-system proof — see the coverage gate below.

- Matrix id: `m5-design-system-contract:stable:0001`
- Governed objects: 11
- Claimed surfaces: 4
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Governed contract objects

| Object id | Kind | Owner | First consumer | Canonical artifact | Proof lane |
| --------- | ---- | ----- | -------------- | ------------------ | ---------- |
| `design-system:foundation:tokens` | `foundation` | Design system owner | `shell` | `fixtures/ui/m5-component-gallery/foundations.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:layout:shell-reference` | `reference_layout` | Design system owner | `shell` | `fixtures/ui/m5-component-gallery/reference-layout.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:state:canonical-states` | `state_semantic_family` | Design system owner | `shell` | `schemas/design-system/m5-design-system-contract-matrix.schema.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:fixture:component-gallery` | `demo_fixture` | Design QA owner | `qa` | `fixtures/ui/m5-component-gallery/` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:proof:token-conformance` | `proof_packet` | Design QA owner | `release_center` | `artifacts/release/m5-design-system-proof/support_export.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:proof:component-screenshot-diff` | `proof_packet` | Design QA owner | `release_center` | `artifacts/release/m5-design-system-proof/support_export.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:proof:appearance-session` | `proof_packet` | Design QA owner | `release_center` | `artifacts/release/m5-design-system-proof/support_export.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:component:shell_chrome` | `component_contract` | Component owner | `shell` | `fixtures/ui/m5-component-gallery/component-contract-shell_chrome.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:component:command_palette` | `component_contract` | Component owner | `shell` | `fixtures/ui/m5-component-gallery/component-contract-command_palette.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:component:trust_prompt` | `component_contract` | Component owner | `shell` | `fixtures/ui/m5-component-gallery/component-contract-trust_prompt.json` | `artifacts/release/m5-design-system-proof/support_export.json` |
| `design-system:component:notification_envelope` | `component_contract` | Component owner | `shell` | `fixtures/ui/m5-component-gallery/component-contract-notification_envelope.json` | `artifacts/release/m5-design-system-proof/support_export.json` |

Every object also names the release packet that keeps it current
(`evidence:m5-design-system-release-packet`) and the extension-SDK guidance
extenders read.

## Claimed-surface coverage gate

Each claimed M5 surface maps the contract objects it must point at. The status
is derived from the inventory: a surface that lacks a mapped object is blocked
from Stable promotion; a surface whose mapped object has stale proof
auto-narrows below Stable.

| Surface | Class | Status | Claim &rarr; effective | Gate | Required objects |
| ------- | ----- | ------ | ---------------------- | ---- | ---------------- |
| `design-system-surface:shell_chrome` | `shell_chrome` | conformant | `stable` &rarr; `stable` | `certified_promote` | `design-system:foundation:tokens`, `design-system:component:shell_chrome`, `design-system:layout:shell-reference`, `design-system:state:canonical-states`, `design-system:fixture:component-gallery`, `design-system:proof:token-conformance` |
| `design-system-surface:command_palette` | `command_palette` | conformant | `stable` &rarr; `stable` | `certified_promote` | `design-system:foundation:tokens`, `design-system:component:command_palette`, `design-system:layout:shell-reference`, `design-system:state:canonical-states`, `design-system:fixture:component-gallery`, `design-system:proof:token-conformance` |
| `design-system-surface:trust_prompt` | `trust_prompt` | conformant | `stable` &rarr; `stable` | `certified_promote` | `design-system:foundation:tokens`, `design-system:component:trust_prompt`, `design-system:layout:shell-reference`, `design-system:state:canonical-states`, `design-system:fixture:component-gallery`, `design-system:proof:token-conformance` |
| `design-system-surface:notification_envelope` | `notification_envelope` | conformant | `stable` &rarr; `stable` | `certified_promote` | `design-system:foundation:tokens`, `design-system:component:notification_envelope`, `design-system:layout:shell-reference`, `design-system:state:canonical-states`, `design-system:fixture:component-gallery`, `design-system:proof:token-conformance` |

## Foundation package (the `foundation` object's content)

The `design-system:foundation:tokens` object's content is the versioned,
machine-readable **foundation package** — the actual semantic tokens, density /
motion / contrast rows, and controlled component-state family the design system
ships, so density, reduced-motion, power-saving, and high-contrast rows cannot
drift by surface family. The package, its diff fixture, and its release-packet
proof are minted by the same seed builder and asserted by inline tests.

| Artifact | Path |
| -------- | ---- |
| Schema | `schemas/design-system/m5-foundation-package.schema.json` |
| Doc | `docs/design-system/m5-foundation-package.md` |
| Canonical package (v1.0.0) | `fixtures/ui/m5-foundation-package/foundation-package.json` |
| Next package (v1.1.0) | `fixtures/ui/m5-foundation-package/foundation-package-next.json` |
| Version diff | `fixtures/ui/m5-foundation-package/foundation-package-diff.json` |
| Release packet | `artifacts/release/m5-design-system-proof/foundation-package-release.json` |

The package versions each family independently, represents the controlled states
(`empty`, `loading`, `pending`, `degraded`, `blocked`, `error`, `completed`) as a
shared family rather than pane-local enums, and keeps unsupported or deprecated
entries inspectable and explicitly downgraded — export, import, diff, and the
release packet all preserve the downgrade target rather than silently dropping it.

## Reference-layout package (the `reference_layout` object's content)

The `design-system:layout:shell-reference` object's content is extended by the
versioned, machine-readable **reference-layout package** — one descriptor per
dominant M5 workspace (notebooks, data grids, the profiler, pipelines, docs,
preview, incident, and companion surfaces) naming how the workspace occupies the
governed shell zones, collapses responsively, degrades when a dependency is
missing, and reopens or resets. The descriptors use the same zone, slot,
fallback-placement, and placeholder-class tokens shell code consumes, and a
shell-slot conformance packet projects the slot-keyed layout truth feature
implementations test against.

| Artifact | Path |
| -------- | ---- |
| Schema | `schemas/design-system/m5-reference-layout-package.schema.json` |
| Doc | `docs/design-system/m5-reference-layout-package.md` |
| Canonical package (v1.0.0) | `fixtures/ui/m5-reference-layout/reference-layout-package.json` |
| Per-workspace fixtures | `fixtures/ui/m5-reference-layout/workspace-*.json` |
| Release packet | `artifacts/release/m5-design-system-proof/reference-layout-release.json` |
| Shell-slot conformance packet | `artifacts/release/m5-design-system-proof/reference-layout-conformance.json` |

Each workspace claims and marks `required` the `main_workspace` work surface and
the `status_bar`; the persistent zones never collapse; every zone declares the
placeholder it shows before content resolves; and every missing-dependency rule
names the placeholder class and governed message the affected zone shows instead
of a blank pane. The package, its per-workspace fixtures, the release packet, and
the conformance packet are minted by the same seed builder and asserted by inline
tests.

## Style-drift lint and state-semantic audit (the conformance gate)

The residual launch risk the matrix above does not by itself close is
surface-local styling and state drift on the most trust-bearing shell flows. The
**style-drift lint** closes it: a checked-in report that declares, per protected
surface, the foundation tokens it consumes, the local style forks it carries, and
the protected-state semantic bindings it renders, plus a lint pass that blocks
Stable promotion when a surface forks the design system, uses an unmanaged token
value, or lets a `loading` / `pending` / `degraded` / `blocked` state go
unlabeled, color-only, spinner-only, or hover-only. The lane covers the trust
prompt, the onboarding flow, the notification / activity center, and the
embedded-surface boundary.

| Artifact | Path |
| -------- | ---- |
| Schema | `schemas/design-system/m5-style-drift-lint.schema.json` |
| Doc | `docs/design-system/m5-style-drift-lint.md` |
| Canonical report (v1.0.0) | `fixtures/ui/m5-style-drift-lint/lint-report.json` |
| Drift / waived / expired drills | `fixtures/ui/m5-style-drift-lint/lint-report-*.json` |
| Lint-outcome proof | `artifacts/release/m5-design-system-proof/style-drift-lint-outcome.json` |
| Release packet | `artifacts/release/m5-design-system-proof/style-drift-lint-release.json` |

A finding is suppressed only by a waiver that is **explicit** (it names one
suppressible check id), **time-bounded** (it carries an `expires_at` and stops
suppressing once the report's `evaluated_at` reaches it), and **tied to a
design-system proof packet** (its `proof_packet_ref` lives under this proof
directory). An expired or proof-less waiver does not suppress its finding, so the
surface still blocks. The gate decision (`pass`, `pass_with_disclosed_gap`,
`warn`, `block`) is the CI signal: the producer's `lint` subcommand exits non-zero
and names the blocked surfaces on `block`. The report, the drift / waived /
expired drills, the outcome proof, and the release packet are minted by the same
seed builder and asserted by inline tests.

## Surface qualification (the integrating gate)

The matrix above governs *which* design-system objects exist; the
**surface-qualification packet** is the layer that qualifies each claimed M5
workspace surface against all four lanes at once — the foundation package, the
component manifests, the reference layouts, and the evidence pack — and derives a
green/yellow/red verdict from them. A surface whose bound contract artifact is
missing is **disqualified** and blocked from Stable promotion; a surface whose
evidence proof is stale or whose token/state conformance fails is **provisional**
and auto-narrows below Stable before promotion (floored at Beta). A blocking gap
can be accepted under a disclosed, time-bounded waiver scoped to one gap subject,
which ships the surface narrowed to the waived claim while its true status stays
red. Help/About, the release center, shiproom, support exports, and the
stable-claim matrix consume the same packet and dashboard.

| Artifact | Path |
| -------- | ---- |
| Packet schema | `schemas/design-system/m5-surface-qualification.schema.json` |
| Dashboard schema | `schemas/design-system/m5-surface-qualification-dashboard.schema.json` |
| Doc | `docs/design-system/m5-surface-qualification.md` |
| Support export | `artifacts/release/m5-design-system-proof/surface-qualification.json` |
| Published dashboard | `artifacts/design-system/m5-surface-qualification-dashboard.json` |
| Markdown proof | `artifacts/release/m5-design-system-proof/surface-qualification-proof.md` |
| Stale / token-drift / missing-manifest / waiver drills | `fixtures/ui/m5-surface-qualification/*.json` |

Every verdict is derived from the same checked-in lane packets, so the
qualification can never outrun the contract behind it. The support export, the
dashboard, the Markdown proof, and the four drill fixtures are minted by the same
seed builder and asserted by inline tests.

## Proof and drift control

The matrix support export is the proof lane that blocks drift: the seed builder
is the single producer of the support export, the dashboard, the Markdown proof,
the component-gallery demo fixtures, and the drill fixtures, and the inline tests
assert the checked-in artifacts match the seed. A claimed M5 surface that lacks a
mapped contract object or current design-system proof fails Stable promotion
through the release gate. The foundation package above is governed the same way:
its checked-in fixtures, diff, and release packet are minted from the seed builder
and asserted by inline tests.

