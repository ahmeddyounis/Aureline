# Language Surface Accessibility Alpha

Status: seeded

This contract validates the first launch-language assistance surfaces that can
affect daily coding flow: diagnostics, completion assistance, and rename /
refactor preview. It composes with the existing language and accessibility
contracts instead of redefining diagnostic, completion, or refactor truth.

Contract identity:

- `language_surface_accessibility_contract_id:
  aureline.accessibility.language_surface_alpha`
- `language_surface_accessibility_schema_version: 1`
- Runtime consumer:
  `crates/aureline-shell/src/help/language_surface_accessibility.rs`
- Protected fixture:
  `fixtures/accessibility/m2_language_surfaces/language_surface_parity.yaml`
- Review packet:
  `artifacts/accessibility/m2_language_review.md`

## Source Anchors

- Product principles require keyboard-first workflows, previewable automated
  edits, and first-class screen-reader and reduced-motion paths.
- The launch-language design requires stable completions, diagnostics,
  rename, go-to-definition, references, and explicit downgrade labels.
- The screen-reader contract defines diagnostic announcements and durable
  fallback rows.
- The focus/zoom contract defines focus-return states and pointer
  independence for transient editor surfaces.
- The visual-adaptation contract requires reduced-motion substitutions to keep
  diagnostic, trust, command, and support meaning intact.

## Protected Surfaces

| Surface | Upstream truth consumed | Accessibility proof |
|---|---|---|
| Diagnostics | `diagnostic_bus` snapshot and diagnostic surface projection | Keyboard diagnostic navigation, source/severity/freshness labels, polite live-region messages, static severity and count cues |
| Completion assistance | `assist` completion/signature/snippet snapshot | Accept/dismiss/snippet traversal routes, source labels, active parameter and placeholder labels, static active-row and preview-required markers |
| Refactor preview | TS/JS and Python rename-preview records | Preview/apply/revert routes, scope and blocked/generated/read-only counts, static preview tree cues, no silent broad mutation |

## Acceptance Rules

Every protected row must:

- expose stable command ids and a keyboard route equivalent to pointer actions;
- preserve a valid focus-return state from the focus contract;
- expose accessible names, message ids, source labels, and durable fallback
  rows;
- preserve meaning in reduced-motion mode through static labels, counts,
  borders, icons, or preview rows;
- keep preview, source, raw/rendered, and no-silent-mutation cues visible before
  accepting broad or risky changes; and
- cite the current known-limit row while evidence remains synthetic fixture
  proof rather than partner-repository assistive-technology proof.

## Known Limit

The current packet cites
`known_limit:external_alpha.language_surface_accessibility_synthetic_only`.
That limit narrows the claim to protected synthetic fixtures and source-model
validation for the TypeScript / JavaScript and Python wedges. It is not full
partner-repository, all-assistive-technology, or replacement-grade
accessibility certification.

## Verification

Run:

```sh
cargo test -p aureline-shell --test language_surface_accessibility
```
