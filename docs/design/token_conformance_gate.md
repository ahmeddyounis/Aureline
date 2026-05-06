# Semantic-token conformance gate and raw-color exception policy

This document defines the **semantic-token conformance gate**: a mechanical
rule set that prevents first-party surfaces from shipping hard-coded colours or
cross-domain token borrowing that blurs meaning (syntax vs diff vs chart vs
diagnostic vs state cues).

This contract is normative. Where it disagrees with the upstream UX design
system style guide, UI/UX spec, or the design-token vocabulary, those sources
win and this document plus the gate tooling MUST be updated in the same change.

## Companion artifacts

- [`/tools/ci/check_semantic_token_conformance.py`](../../tools/ci/check_semantic_token_conformance.py)
  — CI/local gate runner. Scans first-party code for raw-color usage and
  validates the exception registry and worked violation fixtures.
- [`/artifacts/design/raw_color_exception_registry.yaml`](../../artifacts/design/raw_color_exception_registry.yaml)
  — the time-bounded, first-party exception registry.
- [`/fixtures/design/raw_color_violation_cases/`](../../fixtures/design/raw_color_violation_cases/)
  — worked violation and enforcement examples that regression-test the gate.

## Composition, not duplication

This gate composes with existing owners; it does not redefine their vocabulary:

- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  — token families, namespaces, and the shared “no raw colors on stable
  surfaces” intent.
- [`/docs/design/semantic_token_domains_and_palette_contract.md`](./semantic_token_domains_and_palette_contract.md)
  and [`/artifacts/design/semantic_token_domains.yaml`](../../artifacts/design/semantic_token_domains.yaml)
  — semantic token-domain meaning ownership for syntax/diff/chart/status/trust.
- [`/docs/design/appearance_evidence_packet_template.md`](./appearance_evidence_packet_template.md)
  and
  [`/artifacts/design/appearance_row_coverage_matrix.yaml`](../../artifacts/design/appearance_row_coverage_matrix.yaml)
  — appearance evidence routing, including the raw-color exception status
  vocabulary used by evidence packets.

## 1. Gate scope

### 1.1 What the gate enforces

The gate enforces two invariants for **first-party product code** and
**first-party extensions**:

1. **Raw colors are forbidden on stable surfaces.** Consuming code may not
   embed hex / rgb[a] / hsl[a] / integer color literals in UI styling.
2. **Meaning domains must not blur.** When a surface declares a color-domain
   intent (syntax vs diff vs chart vs diagnostic vs state), it may not borrow
   token families from another domain as a substitute for the correct domain’s
   tokens.

### 1.2 What the gate does not enforce

- Third-party extension enforcement beyond published guidance.
- Pixel correctness, rendering implementation, or final theme value selection.
- Detecting every possible form of “raw color” (for example named CSS colors)
  in arbitrary languages; the initial gate targets the most common literal
  formats and can be extended over time.

## 2. Definitions

### 2.1 Raw color literals

A raw color literal is any literal value that encodes color directly rather
than via the semantic token contract. The gate detects:

- hex literals: `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`
- CSS functional forms: `rgb(...)`, `rgba(...)`, `hsl(...)`, `hsla(...)`
- integer literals: `0xRRGGBB` or `0xAARRGGBB` (where applicable)

Raw colors are permitted in **design system sources of truth** (style guide,
token ledgers, palette examples, schemas, and fixtures) because those artifacts
define the canonical mapping from tokens to values. Raw colors are not
permitted in consuming surfaces.

### 2.2 Semantic token references

Consuming code must refer to colors by **semantic token id** (for example
`al.color.*`, `status.*`, and `trust.*`) rather than by literal value. The
canonical meaning and override rules for these ids are owned by the token-domain
ledger and palette contract.

## 3. Exception policy (first-party only)

When first-party code cannot comply immediately, it must route through the
exception registry:

- Registry path: [`/artifacts/design/raw_color_exception_registry.yaml`](../../artifacts/design/raw_color_exception_registry.yaml)
- Exceptions are **time-bounded** (they MUST carry an expiry timestamp).
- Exceptions MUST name:
  - an owner (team/role contact),
  - a plain-language rationale,
  - the scope (path globs + a locate hint),
  - the follow-up path to remove the exception, and
  - at least one evidence reference (appearance packet, conformance packet, or
    a review artifact) so the exception is reviewable by QA, design-system, and
    release review.

The gate refuses:

- expired exceptions;
- exceptions with missing owner, rationale, expiry, scope, follow-up, or
  evidence linkage; and
- exceptions that do not enumerate which raw literals or non-semantic token
  references they cover.

## 4. Domain-specific token-family rules

Token domains exist so “meaning ownership” is mechanical rather than
interpretive. A surface that declares a domain intent MUST NOT use another
domain’s tokens as a substitute:

- **Syntax domain** MUST use `al.color.syntax.*` for language roles and must not
  repurpose diff, chart, status, trust, or state-hue tokens as syntax colors.
- **Diff domain** MUST use `al.color.diff.*` for change meaning and must not use
  syntax tokens, state-hue tokens, or interactive-accent tokens as a substitute
  for diff roles.
- **Chart domain** MUST use `al.color.chart.*` for series roles and must not use
  syntax tokens, diff tokens, state-hue tokens, or interactive-accent tokens as
  a substitute for chart roles.
- **Diagnostic domain** uses the severity/trust vocabulary (status/trust/state
  cues) and must not repurpose syntax, diff, or chart color families for
  severity meaning.
- **State domain** (non-language, non-diff UI state) uses semantic theme +
  severity/trust cues and must not repurpose syntax/diff/chart families.

The gate’s initial enforcement is mechanical and conservative:

- It enforces the raw-color ban by scanning for literals in first-party code.
- It enforces domain separation only when a file opts in by declaring a domain
  intent tag (`aureline-token-domain: <domain>`), so migration can be staged
  without false positives.
