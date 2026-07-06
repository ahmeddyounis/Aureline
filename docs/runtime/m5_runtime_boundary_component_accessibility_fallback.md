# M5 runtime-boundary component accessibility & auto-narrowing (M05-858)

This lane is the accessibility-and-auto-narrowing capstone over the frozen
[M5 runtime-boundary component matrix](../../schemas/ui/m5-runtime-boundary-components.schema.json).
Where the freeze matrix defines the reusable terminal tab / header strip, remote
target pill, environment status strip, toolchain pin row, presence avatar stack, and
repair action card primitives, and the 853–857 implementation lanes resolve their
per-surface truth, this lane certifies — **per component family** — that
runtime-boundary and repair claims stay keyboard-complete, screen-reader-reachable,
CLI/export-safe, and honestly self-narrowing.

- Boundary schema: [`schemas/ui/m5-runtime-boundary-component-accessibility-fallback.schema.json`](../../schemas/ui/m5-runtime-boundary-component-accessibility-fallback.schema.json)
- Release proof: [`artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/`](../../artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/)
- Fixtures: [`fixtures/ui/m5-runtime-boundary-component-accessibility-fallback/`](../../fixtures/ui/m5-runtime-boundary-component-accessibility-fallback/)
- Implementation: `crates/aureline-shell/src/implement_keyboard_screen_reader_cli_export_parity_and_runtime_boundary_claim_auto_narrowing/`

## What it certifies

Each row keys on one frozen `M5RuntimeBoundaryComponentFamily` and reuses that frozen
family vocabulary plus the frozen `M5RuntimeBoundaryRequiredLabel` and
`M5RuntimeBoundaryDowngradeTrigger` and the shared `M5ShellConsumerSurface` consumer
surfaces, so the certified labels stay byte-identical to the matrix and the sibling
primitive packets.

### Keyboard / screen-reader / CLI reach

Every family exposes a keyboard-complete, screen-reader-reachable, and
CLI/headless-reachable path into the same session title, host boundary,
shell-integration quality, winning runtime / toolchain, collaboration role / follow
state, and repair blast-radius / reversibility the rich surface shows — never a
view-only card that strands assistive-tech or headless users. The hierarchy-heavy
family (the toolchain pin row's precedence inspector with its ordered shadowed
layers) additionally binds its tree to a flat list / textual path.

### Export parity

The support / release export reconstructs each component's meaning from typed tokens
and opaque refs **without a screenshot**, preserving the same host boundaries,
runtime sources, roles, and reversal classes shown in-product. Copy / export is
offered as text, JSON, and Markdown.

### Honest auto-narrowing

When a runtime dimension — host identity, shell-integration confidence, context
precedence, collaboration role, or repair reversibility — is partial, reconnecting,
restored, or policy-blocked, the component's **runtime-support claim** auto-narrows
from `live` / `ready` to the permitted ceiling and names the binding dimension and
frozen trigger while preserving canonical identity. A component with every dimension
intact carries **no** spurious narrowing, so an old `Live` or `Ready` label never
lingers on degraded runtime state.

| Condition state | Permitted support ceiling |
| --- | --- |
| `intact` | `live` |
| `partial` | `degraded` |
| `reconnecting` | `reconnecting` |
| `restored` | `restored` |
| `policy_blocked` | `policy_blocked` |

The binding dimension names the on-topic frozen trigger it governs:

| Binding dimension | Frozen trigger |
| --- | --- |
| `host_identity` | `host_boundary_masked` |
| `shell_integration_confidence` | `shell_integration_quality_hidden` |
| `context_precedence` | `runtime_source_unexplained` |
| `collaboration_role` | `collaboration_role_masked` |
| `repair_reversibility` | `reversibility_overstated` |

The effective support claim is the weakest ceiling across all modeled dimensions,
capped at the family's full claim. A stale, partial, reconnecting, restored, or
policy-blocked runtime can therefore no longer keep an old `Live` / `Ready` label.

### Cross-surface disclosure

The same narrowed state surfaces in shell chrome, side panels, docs/help, headless
CLI, release proof, and support/admin exports (`M5ShellConsumerSurface`), so claim
publication and field triage stay aligned on runtime-boundary downgrade behavior.
Every narrower rendering surface discloses its reduced interactivity and preserves
its labels; nothing is silently dropped.

## Certified rows

Six families, one row each: **2 green / 4 yellow / 0 red**.

| Row | Family | Effective claim | Binding dimension |
| --- | --- | --- | --- |
| `a11y:remote-target-pill` | remote target pill | `live` (green) | — |
| `a11y:environment-status-strip` | environment status strip | `ready` (green) | — |
| `a11y:terminal-tab` | terminal tab / header strip | `restored` | shell-integration confidence (restored) |
| `a11y:toolchain-pin-row` | toolchain pin row | `degraded` | context precedence (partial) |
| `a11y:presence-avatar-stack` | presence avatar stack | `reconnecting` | collaboration role (reconnecting) |
| `a11y:repair-action-card` | repair action card | `policy_blocked` | repair reversibility (policy-blocked) |

## Metadata-only boundary

Raw terminal buffers, credentials, connection secrets, mirror URLs, and provider
cursors never cross this boundary. The packet carries only typed class tokens, opaque
summary / evidence refs, booleans, and redacted labels.

## Regenerating the proof

The checked-in `support_export.json`, `matrix.csv`, and `report.md` are the one
source of truth, byte-aligned with the `seeded_m5_runtime_a11y_fallback_packet()`
builder. Regenerate with:

```
GEN_RUNTIME_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts
```
