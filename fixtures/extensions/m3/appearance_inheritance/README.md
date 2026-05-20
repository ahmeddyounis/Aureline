# Extension appearance inheritance fixtures

This directory contains the checked fixture corpus for extension-UI
appearance conformance. The records are generated from the Rust seed in
`crates/aureline-extensions/src/appearance_conformance/`.

All generated records carry the shared contract ref
`extensions:appearance_conformance_beta:v1` so marketplace rows, install
and side-load review, mirrored/offline bundle review, post-install
diagnostics, headless output, support exports, and the generated report
pivot to the same row ids.

## Index

| Fixture | Coverage |
| --- | --- |
| `inputs.json` | Seed inputs for four contributed surfaces: a fully inherited preview pane, a reduced-support dashboard panel, an unverified inheritance claim, and a private-styling mirrored toolbar. |
| `rows.json` | Audited rows with per-axis conformance, known unsupported states, per-surface caveats, overall support class, decision, reason, and host-stable labels. |
| `support_rows.json` | Metadata-safe support rows paired 1:1 with product rows. |
| `defects.json` | Validator output for the seeded corpus. The expected value is `[]`. |
| `conformance_packet.json` | Full packet with summary, rows, support rows, and defects. |
| `support_export.json` | Support-export wrapper projected from the packet with raw private styling material excluded. |

## Fixture rules

- Every row discloses all six host appearance axes: theme, density,
  focus ring, high contrast, reduced motion, and host tokens.
- A row is only badged `full_inheritance` when each axis is declared
  `inherits` and a host probe returns `proven_inherits`.
- A claim the host cannot prove (`unproven`) downgrades to
  `reduced_support` and routes the row to `needs_review`; it never
  implies full inheritance on any surface.
- A claim the host contradicts (`proven_unsupported` against an
  `inherits` declaration) is an `overclaimed_inheritance` defect and
  refuses the row.
- An undisclosed axis is an `axis_disclosure_missing` defect.
- Host-stable trust, severity, permission, and policy labels stay
  host-rendered; if they are not, the row is refused.
- Every row carries a caveat for the marketplace result row, detail
  page, install review, side-load review, mirrored bundle review, and
  post-install diagnostics; the diagnostics caveat persists after enable.
- Support rows mirror product rows on identity, lifecycle, per-axis
  support tokens, overall support, decision, reason, host-stable trust
  and severity, defect tokens, and caveat summary.

## Regenerate

```text
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- inputs > fixtures/extensions/m3/appearance_inheritance/inputs.json
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- packet > fixtures/extensions/m3/appearance_inheritance/conformance_packet.json
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- rows > fixtures/extensions/m3/appearance_inheritance/rows.json
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- support-rows > fixtures/extensions/m3/appearance_inheritance/support_rows.json
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- defects > fixtures/extensions/m3/appearance_inheritance/defects.json
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- support-export > fixtures/extensions/m3/appearance_inheritance/support_export.json
```

## Verification

```text
cargo test -p aureline-extensions appearance_conformance
cargo run -q -p aureline-extensions --example dump_appearance_conformance_records -- validate
```
