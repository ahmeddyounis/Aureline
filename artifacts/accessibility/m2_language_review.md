# Launch-Language Surface Accessibility Review

Status: seeded

Canonical packet:
`fixtures/accessibility/m2_language_surfaces/language_surface_parity.yaml`

First consumer:
`crates/aureline-shell/src/help/language_surface_accessibility.rs`

## Scope

This review covers diagnostics, completion assistance, and rename/refactor
preview across the TypeScript / JavaScript and Python launch wedges. It proves
that the protected alpha surfaces do not rely on pointer-only actions,
color-only state, or motion-only cues, and that preview/source labels remain
visible before broad mutation.

## Evidence Rows

| Surface | Keyboard | Screen reader | Reduced motion | Preview/source integrity |
|---|---|---|---|---|
| Diagnostics inline markers and Problems rows | Passed: diagnostic navigation, quick-fix entry, exact focus return | Passed: severity, source, freshness, scope, and diagnostic live-region messages | Passed: static severity labels, source badges, and problem counts replace pulse/transition cues | Passed: source/provider labels and quick-fix preview posture remain visible |
| Completion list, signature help, and snippet session | Passed: accept/dismiss plus snippet traversal and cancel routes | Passed: item source, additional edits, active parameter, and placeholder labels | Passed: static active-row, source, parameter, placeholder, and preview-required labels | Passed: LSP, fallback, and snippet sources remain labeled before acceptance |
| Rename and refactor preview | Passed: preview tree traversal, apply, narrow-scope, export, and cancel routes | Passed: changed/generated/protected counts, scope, warnings, and apply posture | Passed: static preview rows and count labels replace expand/collapse and warning motion | Passed: structured preview, blocked counts, scope labels, and safe-preview refs prevent silent broad mutation |

## Remaining Narrowness

Known-limit refs:

- `known_limit:external_alpha.scope.claimed_wedges_only`
- `known_limit:external_alpha.language_surface_accessibility_synthetic_only`

The review is current for protected synthetic fixtures and source-model
validation. It does not claim full assistive-technology coverage on partner
repositories, full framework-expert refactor breadth, or all-language parity.

## Verification

```sh
cargo test -p aureline-shell --test language_surface_accessibility
```
