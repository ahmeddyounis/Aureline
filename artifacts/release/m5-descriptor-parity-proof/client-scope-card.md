# M5 client-scope card parity

- Registry: `m5-client-scope-card-registry:stable:0001`
- Label: `M5 client-scope card parity across discovery, deep-link, handoff, and companion surfaces`
- Cards: 6
- Minted: `2026-07-06T00:00:00Z`
- Surface classes: desktop, browser companion, headless, unsupported
- Disclosures: discovery, deep link, handoff, companion
- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion

## Cards

| Card | Surface | Client | Authority | Handoff | Claim | Blocked | Caveats |
|------|---------|--------|-----------|---------|-------|---------|--------|
| `client-scope-card:desktop-full` | `desktop` | `desktop_full` | `full_authority` | `not_required` | `fully_supported` | 0 | 0 |
| `client-scope-card:browser-companion` | `browser_companion` | `mobile_companion` | `scoped_authority` | `desktop_handoff_required` | `unsupported_client` | 2 | 3 |
| `client-scope-card:browser-reference` | `browser_companion` | `browser_reference` | `reference_only` | `console_handoff_required` | `unsupported_client` | 3 | 3 |
| `client-scope-card:headless` | `headless` | `companion_scoped` | `scoped_authority` | `desktop_handoff_required` | `unsupported_client` | 2 | 3 |
| `client-scope-card:unsupported-handoff` | `unsupported` | `handoff_only` | `handoff_only` | `desktop_handoff_required` | `unsupported_client` | 4 | 3 |
| `client-scope-card:unsupported-not-provided` | `unsupported` | `handoff_only` | `not_provided` | `not_provided` | `unsupported_client` | 4 | 3 |

## Disclosure parity

### `client-scope-card:desktop-full` → surface `desktop` (full authority)

Copy-safe summary: `client-scope-card:desktop-full · surface desktop · client desktop_full · authority full_authority · handoff not_required · claim fully_supported · 0 blocked · 0 caveat(s)`

_No blocked actions — full desktop parity._

| Disclosure | Authority | Handoff disclosed | No broader authority |
|------------|-----------|-------------------|----------------------|
| `discovery` | `full_authority` | n/a | yes |
| `deep_link` | `full_authority` | n/a | yes |
| `handoff` | `full_authority` | n/a | yes |
| `companion` | `full_authority` | n/a | yes |

### `client-scope-card:browser-companion` → surface `browser_companion` (narrowed)

Copy-safe summary: `client-scope-card:browser-companion · surface browser_companion · client mobile_companion · authority scoped_authority · handoff desktop_handoff_required · claim unsupported_client · 2 blocked · 3 caveat(s)`

**Blocked actions (recover via handoff):**

- `approve` → recover via `desktop_handoff_required`
- `administer` → recover via `desktop_handoff_required`

**Parity caveats:**

- `client_kind` (`mobile_companion`)
- `authority_class` (`scoped_authority`)
- `handoff_requirement` (`desktop_handoff_required`)

| Disclosure | Authority | Handoff disclosed | No broader authority |
|------------|-----------|-------------------|----------------------|
| `discovery` | `scoped_authority` | yes | yes |
| `deep_link` | `scoped_authority` | yes | yes |
| `handoff` | `scoped_authority` | yes | yes |
| `companion` | `scoped_authority` | yes | yes |

### `client-scope-card:browser-reference` → surface `browser_companion` (narrowed)

Copy-safe summary: `client-scope-card:browser-reference · surface browser_companion · client browser_reference · authority reference_only · handoff console_handoff_required · claim unsupported_client · 3 blocked · 3 caveat(s)`

**Blocked actions (recover via handoff):**

- `mutate_in_place` → recover via `console_handoff_required`
- `approve` → recover via `console_handoff_required`
- `administer` → recover via `console_handoff_required`

**Parity caveats:**

- `client_kind` (`browser_reference`)
- `authority_class` (`reference_only`)
- `handoff_requirement` (`console_handoff_required`)

| Disclosure | Authority | Handoff disclosed | No broader authority |
|------------|-----------|-------------------|----------------------|
| `discovery` | `reference_only` | yes | yes |
| `deep_link` | `reference_only` | yes | yes |
| `handoff` | `reference_only` | yes | yes |
| `companion` | `reference_only` | yes | yes |

### `client-scope-card:headless` → surface `headless` (narrowed)

Copy-safe summary: `client-scope-card:headless · surface headless · client companion_scoped · authority scoped_authority · handoff desktop_handoff_required · claim unsupported_client · 2 blocked · 3 caveat(s)`

**Blocked actions (recover via handoff):**

- `approve` → recover via `desktop_handoff_required`
- `administer` → recover via `desktop_handoff_required`

**Parity caveats:**

- `client_kind` (`companion_scoped`)
- `authority_class` (`scoped_authority`)
- `handoff_requirement` (`desktop_handoff_required`)

| Disclosure | Authority | Handoff disclosed | No broader authority |
|------------|-----------|-------------------|----------------------|
| `discovery` | `scoped_authority` | yes | yes |
| `deep_link` | `scoped_authority` | yes | yes |
| `handoff` | `scoped_authority` | yes | yes |
| `companion` | `scoped_authority` | yes | yes |

### `client-scope-card:unsupported-handoff` → surface `unsupported` (narrowed)

Copy-safe summary: `client-scope-card:unsupported-handoff · surface unsupported · client handoff_only · authority handoff_only · handoff desktop_handoff_required · claim unsupported_client · 4 blocked · 3 caveat(s)`

**Blocked actions (recover via handoff):**

- `observe` → recover via `desktop_handoff_required`
- `mutate_in_place` → recover via `desktop_handoff_required`
- `approve` → recover via `desktop_handoff_required`
- `administer` → recover via `desktop_handoff_required`

**Parity caveats:**

- `client_kind` (`handoff_only`)
- `authority_class` (`handoff_only`)
- `handoff_requirement` (`desktop_handoff_required`)

| Disclosure | Authority | Handoff disclosed | No broader authority |
|------------|-----------|-------------------|----------------------|
| `discovery` | `handoff_only` | yes | yes |
| `deep_link` | `handoff_only` | yes | yes |
| `handoff` | `handoff_only` | yes | yes |
| `companion` | `handoff_only` | yes | yes |

### `client-scope-card:unsupported-not-provided` → surface `unsupported` (narrowed)

Copy-safe summary: `client-scope-card:unsupported-not-provided · surface unsupported · client handoff_only · authority not_provided · handoff not_provided · claim unsupported_client · 4 blocked · 3 caveat(s)`

**Blocked actions (recover via handoff):**

- `observe` → recover via `desktop_handoff_required`
- `mutate_in_place` → recover via `desktop_handoff_required`
- `approve` → recover via `desktop_handoff_required`
- `administer` → recover via `desktop_handoff_required`

**Parity caveats:**

- `client_kind` (`handoff_only`)
- `authority_class` (`not_provided`)
- `handoff_requirement` (`not_provided`)

| Disclosure | Authority | Handoff disclosed | No broader authority |
|------------|-----------|-------------------|----------------------|
| `discovery` | `not_provided` | yes | yes |
| `deep_link` | `not_provided` | yes | yes |
| `handoff` | `not_provided` | yes | yes |
| `companion` | `not_provided` | yes | yes |

