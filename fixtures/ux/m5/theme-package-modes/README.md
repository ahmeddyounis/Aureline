# M5 theme-package modes fixtures

These fixtures are the checked-in canonical truth for the M5 theme-package
manifest audit. They conform to
[`schemas/ux/m5-theme-package-manifest.schema.json`](../../../../schemas/ux/m5-theme-package-manifest.schema.json)
and are validated by the CI gate at
[`tools/ci/m5/theme_package_manifest_check.py`](../../../../tools/ci/m5/theme_package_manifest_check.py).
See the companion doc
[`docs/m5/theme-package-manifests.md`](../../../../docs/m5/theme-package-manifests.md)
for how the audit binds versioned theme packages to every claimed M5 surface.

| File | Record kind | Why it is here |
| --- | --- | --- |
| `report.json` | `shell_m5_theme_package_manifest_report_record` | The registered theme-package manifests, the per-surface active-package bindings, per-package coverage, the provenance index, and the blocking-finding summary. |
| `support_export.json` | `shell_m5_theme_package_manifest_support_export_record` | The support-export wrapper a reviewer pivots on; its `case_ids` quote the report id, every package id and revision, and every surface id and descriptor revision. |
| `compact.txt` | (rendered summary) | One-line audit summary the headless inspector prints. |

The fixtures are the **only mint-from-truth output** of the headless inspector
`aureline_shell_m5_theme_packages`; regenerate them with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- report > \
  fixtures/ux/m5/theme-package-modes/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- support-export > \
  fixtures/ux/m5/theme-package-modes/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- compact > \
  fixtures/ux/m5/theme-package-modes/compact.txt
```

The clean checked-in report exercises the disclosed-appearance scenarios the
contract is built to keep honest:

- Six first-party surfaces (notebook, result grid, profiler timeline,
  preview/browser pane, docs/help pane, companion surface) ride the built-in
  default package, fully inherit its five appearance axes, and disclose their
  provenance with fresh evidence.
- An extension-backed panel rides a signed, extension-contributed package,
  honours only the dark modes that package supports, and **discloses** that it
  does not inherit the focus axis the package expects — a partial-inheritance
  posture, not a hidden gap.
- The provenance index reports each package's signature, build-compatibility,
  and most-degraded disclosed evidence state, so About/help, diagnostics, and
  support export read the same provenance truth.

Every binding stays clean because each downgrade is disclosed; the same
conditions become blockers the moment a surface claims an unsupported mode,
hides an inheritance gap, renders a disabled package without disclosure, keeps
stale evidence on a marketed surface, or paints its own appearance outside the
shared appearance-session model.
