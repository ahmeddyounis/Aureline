# M5 theme-package interop fixtures

These fixtures are the checked-in canonical truth for the M5 theme-import-parity
qualification audit. They conform to
[`schemas/ux/m5-theme-import-parity.schema.json`](../../../../schemas/ux/m5-theme-import-parity.schema.json)
and are validated by the CI gate at
[`tools/ci/m5/theme_import_parity_check.py`](../../../../tools/ci/m5/theme_import_parity_check.py).
See the companion doc
[`docs/m5/theme-package-and-appearance-objects.md`](../../../../docs/m5/theme-package-and-appearance-objects.md)
for the frozen object-model index and the compatibility/downgrade vocabulary.

| File | Record kind | Why it is here |
| ---- | ----------- | -------------- |
| `report.json` | `shell_m5_theme_import_parity_report_record` | The frozen object-model index plus the per-surface parity bindings across the five rows. |
| `support_export.json` | `shell_m5_theme_import_parity_support_export_record` | The support-export wrapper; `case_ids` quotes the report id and every surface id and descriptor revision. |
| `compact.txt` | (rendered summary) | One-line audit summary plus per-row coverage counts for quick CI/log inspection. |

The report is "interop" because the same five canonical appearance objects are
consumed across the M5 integration surfaces — shell chrome, docs/help and
service-health panes, support/export, extension-hosted and embedded surfaces,
and the marketplace/account/sync settings surface — through one shared object
model rather than per-surface theme code.

The clean checked-in report exercises the honest-downgrade scenarios the
contract is built to keep legible:

- A docs/help pane that needs a **restart-or-reload** to adopt a theme-package
  swap, disclosed before it applies.
- A support/export surface whose imported VS Code theme leaves **unsupported
  slots**, each disclosed with its fallback class and a rollback path.
- An extension-hosted panel with **partial inheritance** (reduced-motion gap
  disclosed in product, export, and diagnostics).
- An embedded webview that **declares a capture gap** because it cannot inherit
  the host reduced-motion posture.

All four remain `qualified` (or a declared gap) because the downgrade is
disclosed; the same conditions become blockers the moment they are hidden.
