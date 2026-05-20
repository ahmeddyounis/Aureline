# Extension appearance conformance (beta)

This page documents how extension-contributed UI declares and proves that
it inherits the host appearance contract, and how that truth reaches
marketplace rows, install and side-load review, mirrored/offline bundle
review, and post-install diagnostics.

The contract exists so a marketplace row, install sheet, or mirrored
catalog entry never *implies* that contributed UI inherits host theme,
density, focus, contrast, motion, or tokens unless the runtime can prove
it. Extensions may ship rich custom surfaces, but appearance parity is a
claim that must be backed by a host-side conformance probe, not by prose.

## Canonical records

- Rust source:
  `crates/aureline-extensions/src/appearance_conformance/`
- Cross-tool schema:
  `schemas/extensions/appearance_support.schema.json`
- Fixture corpus:
  `fixtures/extensions/m3/appearance_inheritance/`
- Human-readable report:
  `artifacts/extensions/m3/appearance_gap_review.md`

Every record carries `extensions:appearance_conformance_beta:v1`. Product
rows, support rows, docs, and the generated report cite the same row ids.

## Appearance axes

Each contributed surface declares an inheritance posture for every host
appearance axis:

| Axis | What it covers |
| --- | --- |
| `theme` | Color/theme class (light, dark, theme-package tokens). |
| `density` | Density scale (comfortable, compact). |
| `focus_ring` | Keyboard focus-ring tokens and visible-focus behavior. |
| `high_contrast` | High-contrast / forced-colors tokens. |
| `reduced_motion` | Reduced-motion tokens and animation suppression. |
| `host_token` | Design-system host tokens beyond the named axes. |

The declared posture uses the shared inheritance vocabulary `inherits`,
`partial`, `does_not_inherit`, or `not_disclosed`. An extension also
declares its **known unsupported states** (for example, fixed chart
colors under forced-colors dark) so a reduced row is honest about *what*
it cannot honor.

## Declaration joined with host proof

A declaration alone never earns a full-inheritance badge. The host runs a
conformance probe per axis and the runtime joins declaration with proof:

| Declared | Host probe | Effective support | Outcome |
| --- | --- | --- | --- |
| `inherits` | `proven_inherits` | `full_inheritance` | Conformant, badge may imply full inheritance. |
| `inherits` | `proven_reduced` | `reduced_support` | Needs review — claim exceeds proof. |
| `inherits` | `unproven` | `reduced_support` | Needs review — verify before badging. |
| `inherits` | `proven_unsupported` | `unsupported_private_styling` | **Refused** — overclaimed inheritance. |
| `partial` | `proven_inherits` / `proven_reduced` | `reduced_support` | Conformant with caveat. |
| `partial` | `proven_unsupported` | `unsupported_private_styling` | **Refused** — overclaimed inheritance. |
| `does_not_inherit` | any | `unsupported_private_styling` | Conformant, disclosed private styling. |
| `not_disclosed` | any | `undisclosed_gap` | Needs review — disclosure incomplete. |

The worst axis rolls up into the row's `overall_support_class`. A row is
only badged full inheritance when **every** axis is `full_inheritance`
and the decision is `conformant`. This is the exit-gate guarantee: rows
no longer imply full appearance inheritance unless parity is proven.

## Surfaces and persistent caveats

Each row carries a caveat for every consuming surface:

- `marketplace_result_row`
- `marketplace_detail_page`
- `install_review`
- `sideload_review`
- `mirrored_bundle_review`
- `post_install_diagnostics`

The post-install diagnostics caveat is marked `persists_after_install`,
so an appearance gap stays visible in diagnostics/help after the package
is enabled instead of disappearing once installed. No surface caveat may
set `implies_full_inheritance` unless the row is proven fully inherited.

## Host-stable labels are never hidden

Every row carries host-stable trust, severity, permission, and policy
labels plus `host_rendered_trust_and_severity`. Extension-local styling
can never hide them: if the host does not render the labels, or any label
is empty, the appearance claim is refused. Reduced or unsupported
appearance never downgrades these host-stable cues.

## Seeded coverage

The checked packet covers four contributed surfaces:

| Surface | Overall support | Decision | Reason |
| --- | --- | --- | --- |
| Markdown preview pane | `full_inheritance` | `conformant` | `full_inheritance_proven` |
| Insights dashboard panel | `reduced_support` | `conformant` | `reduced_support_disclosed` |
| Theme settings surface | `reduced_support` | `needs_review` | `needs_verification_before_badge` |
| Custom toolbar surface (mirrored) | `unsupported_private_styling` | `conformant` | `unsupported_private_styling_disclosed` |

The seeded summary has zero defects, one fully inherited row, two
reduced-support rows, one private-styling row, and zero overclaimed rows.

## Headless consumer

```text
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- packet
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- rows
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- support-rows
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- defects
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- support-export
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- validate
```

`validate` fails if an axis is undisclosed, a declared inheritance claim
is contradicted by a host probe, a surface caveat implies full
inheritance without proof, a known unsupported state contradicts a fully
inherited axis, host-stable labels stop being host-rendered, or support
export parity breaks.

## How to verify

```text
cargo test -p aureline-extensions appearance_conformance
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- validate
```
