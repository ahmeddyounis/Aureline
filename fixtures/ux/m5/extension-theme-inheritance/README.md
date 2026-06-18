# Extension theme-inheritance fixtures

These fixtures are the checked-in canonical truth for the extension
appearance-inheritance descriptor audit. They conform to
[`schemas/ux/extension-appearance-descriptor.schema.json`](../../../../schemas/ux/extension-appearance-descriptor.schema.json)
and are validated by the CI gate at
[`tools/ci/m5/extension_appearance_descriptors_check.py`](../../../../tools/ci/m5/extension_appearance_descriptors_check.py).
See the companion doc
[`docs/m5/extension-appearance-inheritance.md`](../../../../docs/m5/extension-appearance-inheritance.md)
for how the extension-detail, embedded-pane, diagnostics, and support/export
surfaces consume the same audit object.

| File | Record kind | Why it is here |
| ---- | ----------- | -------------- |
| `audit.json` | `extension_appearance_audit_record` | Every extension-backed or embedded surface with its host id, package id, five governed posture axes (theme, focus, contrast, density, reduced motion), derived inheritance badge, parity-claim state, and known gaps, plus the recomputed summary. |
| `support_export.json` | `extension_appearance_support_export_record` | The support-export wrapper; `case_ids` quotes the audit id and every descriptor id, package id, and host id, and `raw_appearance_material_excluded` is asserted. |
| `compact.txt` | (rendered summary) | One-line audit header plus a per-descriptor badge/decision line for quick CI/log inspection. |

These fixtures are bit-for-bit equal to the output of
`seeded_extension_appearance_audit` in
[`crates/aureline-extensions/src/appearance_descriptors/mod.rs`](../../../../crates/aureline-extensions/src/appearance_descriptors/mod.rs),
enforced by the integration test
`crates/aureline-extensions/tests/extension_appearance_descriptors_fixtures.rs`.
Regenerate them with:

```sh
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- audit > \
  fixtures/ux/m5/extension-theme-inheritance/audit.json
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- support-export > \
  fixtures/ux/m5/extension-theme-inheritance/support_export.json
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- compact > \
  fixtures/ux/m5/extension-theme-inheritance/compact.txt
cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- markdown > \
  artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md
```

The corpus exercises the honesty spectrum the contract keeps legible — what
inherits, what only approximates, and what keeps private appearance logic:

- A **preview pane** that inherits every axis and is granted `claims_host_parity`,
  backed by accessibility evidence.
- An **embedded webview** dashboard that inherits theme and density but keeps a
  private focus ring and approximate contrast/motion — badged
  `partial_inheritance` and making `no_parity_claim`.
- A **provider panel** that ships a fixed private palette across every axis —
  badged `does_not_inherit`.
- An embedded **docs/help pane** that inherits every axis except a disclosed
  compact-density gap and makes a `partial_claim_with_gaps`, plus a
  **diagnostics pane** that partly inherits and makes `no_parity_claim`.

Every descriptor renders its badge on the extension-detail, embedded-pane,
diagnostics, and support/export surfaces, and no descriptor overclaims parity it
cannot back. Raw theme files, raw token values, raw screenshots, raw paths, and
raw user content never cross this boundary.
