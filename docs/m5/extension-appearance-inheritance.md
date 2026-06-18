# Extension appearance inheritance

Extension-backed and embedded surfaces — the extension detail page, a contributed
webview, a provider panel, a preview pane, an embedded docs/help pane, or a
post-install diagnostics pane — can render rich custom UI. That is allowed. What
is **not** allowed is implying first-party parity with Aureline's appearance when
the surface does not actually inherit it. A user must be able to tell whether a
surface inherits Aureline's theme, focus, contrast, density, and reduced-motion
semantics or only approximates them, and that answer must be inspectable in the
product and in diagnostics/export — not hidden in docs-only notes or manifest
comments the runtime cannot read.

This lane makes appearance inheritance a governed, machine-readable descriptor.
It is the product-facing runtime twin of the design-side audited record frozen in
[`schemas/design/extension_ui_appearance_descriptor.schema.json`](../../schemas/design/extension_ui_appearance_descriptor.schema.json)
and reuses that contract's inheritance-axis and parity-claim vocabulary instead
of minting parallel values.

## Canonical records

The Rust source of truth is
[`crates/aureline-extensions/src/appearance_descriptors/mod.rs`](../../crates/aureline-extensions/src/appearance_descriptors/mod.rs).
It mints the same records every consuming surface reads:

- the boundary schema:
  [`schemas/ux/extension-appearance-descriptor.schema.json`](../../schemas/ux/extension-appearance-descriptor.schema.json);
- the checked fixtures:
  [`fixtures/ux/m5/extension-theme-inheritance/audit.json`](../../fixtures/ux/m5/extension-theme-inheritance/audit.json)
  and `support_export.json`;
- the published report:
  [`artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md`](../../artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md);
- the fail-closed gate:
  [`tools/ci/m5/extension_appearance_descriptors_check.py`](../../tools/ci/m5/extension_appearance_descriptors_check.py);
  and
- the contract test
  [`crates/aureline-extensions/tests/extension_appearance_descriptors_fixtures.rs`](../../crates/aureline-extensions/tests/extension_appearance_descriptors_fixtures.rs).

The records carry no raw theme files, token values, screenshots, paths, or user
content — only opaque refs, closed vocabulary, short labels, and counts.

## The five governed axes

Each descriptor declares an inheritance posture for the five host appearance
axes. The axis vocabulary matches the frozen appearance contracts; the posture
vocabulary is reused from the webview-boundary lane:

| Token | Meaning |
| ----- | ------- |
| `inherits` | The surface inherits this host axis. |
| `partial` | The surface partially inherits and names the limitation. |
| `does_not_inherit` | The surface keeps private logic for this axis. |
| `not_disclosed` | The surface failed to disclose this axis (a defect). |

| Axis | What it covers |
| ---- | -------------- |
| `theme` | Color/theme tokens (light, dark, theme-package palettes). |
| `focus` | Keyboard focus-ring and focus-token posture. |
| `contrast` | High-contrast / forced-colors tokens. |
| `density` | Density scale (compact, standard, comfortable) tokens. |
| `reduced_motion` | Reduced-motion tokens and animation suppression. |

## Visible badge

The four postures derive one visible badge, rendered identically in extension
details, embedded panes, diagnostics, and support/export packets:

| Badge | When |
| ----- | ---- |
| `full_inheritance` | Every axis `inherits`. |
| `partial_inheritance` | Some axes inherit and at least one does not. |
| `does_not_inherit` | No axis inherits; private appearance logic. |
| `undisclosed` | At least one axis is `not_disclosed`. |

Only `full_inheritance` is consistent with a first-party-parity claim.

## Parity claims are blocked unless backed

A surface may set `claims_first_party_parity`, but the descriptor resolves it
fail-closed into the frozen `parity_claim_state` vocabulary:

| State | When |
| ----- | ---- |
| `no_parity_claim` | The surface does not claim parity; its badge stands alone. |
| `claims_host_parity` | Claimed, every axis inherits, no gaps, and accessibility evidence backs it. |
| `partial_claim_with_gaps` | Claimed on the inherited axes, partially inheriting, every gap disclosed, and accessibility evidence present. |
| `denied_claim` | Claimed, but full inheritance, gap disclosure, or accessibility evidence is missing. |

A `denied_claim` is an `overclaimed_parity` defect, so release and public-truth
packets cannot restate host parity for a surface that keeps private appearance
logic.

## Defect vocabulary

The gate refuses on a closed defect vocabulary:

- `undisclosed_axis` — an axis posture is `not_disclosed`.
- `overclaimed_parity` — a host-parity claim is not backed by full inheritance,
  gap disclosure, and accessibility evidence.
- `hidden_inheritance_gap` — a full-inheritance badge that also discloses a gap.
- `host_badge_chrome_hidden` — the host badge is suppressed or not rendered on a
  required surface.
- `support_export_parity_drift` — a support row or summary disagrees with the
  descriptors.
- `raw_appearance_material_exported` — raw appearance material crossed the
  support-export boundary.

## Seeded coverage

The seeded corpus covers the honesty spectrum:

- a **preview pane** that inherits every axis and is granted `claims_host_parity`,
  backed by accessibility evidence;
- an **embedded webview** dashboard that inherits theme and density but keeps a
  private focus ring and approximate contrast/motion — badged
  `partial_inheritance` and making `no_parity_claim`;
- a **provider panel** that ships a fixed private palette across every axis —
  badged `does_not_inherit`;
- an embedded **docs/help pane** that inherits every axis except a fixed compact
  density it discloses, makes a `partial_claim_with_gaps`, and backs it with
  accessibility evidence; and
- a **diagnostics pane** that partly inherits and makes `no_parity_claim`.

## Headless consumer

```sh
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- audit
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- support-export
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- compact
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- markdown
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- validate
```

## How to verify

```sh
python3 tools/ci/m5/extension_appearance_descriptors_check.py --repo-root .
cargo test -p aureline-extensions --test extension_appearance_descriptors_fixtures
cargo test -p aureline-extensions appearance_descriptors
```
