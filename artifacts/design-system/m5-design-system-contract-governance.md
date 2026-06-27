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

## Proof and drift control

The matrix support export is the proof lane that blocks drift: the seed builder
is the single producer of the support export, the dashboard, the Markdown proof,
the component-gallery demo fixtures, and the drill fixtures, and the inline tests
assert the checked-in artifacts match the seed. A claimed M5 surface that lacks a
mapped contract object or current design-system proof fails Stable promotion
through the release gate. The foundation package above is governed the same way:
its checked-in fixtures, diff, and release packet are minted from the seed builder
and asserted by inline tests.

