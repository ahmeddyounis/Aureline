# M5 imported-theme corpus fixtures

These fixtures are the checked-in canonical truth for the M5 imported-theme
mapping & rollback report. They conform to
[`schemas/ux/m5-theme-import-report.schema.json`](../../../../schemas/ux/m5-theme-import-report.schema.json)
and are validated by the CI gate at
[`tools/ci/m5/theme_import_report_check.py`](../../../../tools/ci/m5/theme_import_report_check.py).
See the companion doc
[`docs/m5/theme-import-and-rollback.md`](../../../../docs/m5/theme-import-and-rollback.md)
for how the migration center, support/export, compatibility packets, and
sync/import flows consume the same report object.

| File | Record kind | Why it is here |
| ---- | ----------- | -------------- |
| `report.json` | `shell_m5_theme_import_report_record` | The imported-theme rows across ecosystems, with per-row source provenance, translated-token and unresolved-slot counts, syntax coverage, parity note, and rollback ref, plus the outcome and aggregate-token summaries. |
| `support_export.json` | `shell_m5_theme_import_report_support_export_record` | The support-export wrapper; `case_ids` quotes the report id and every row id, source-theme identifier (provenance), checkpoint ref, and rollback ref. |
| `compact.txt` | (rendered summary) | One-line report header plus a per-theme summary for quick CI/log inspection. |

These fixtures are bit-for-bit equal to the output of
`seeded_theme_import_report` in
[`crates/aureline-shell/src/theme_import_reports/mod.rs`](../../../../crates/aureline-shell/src/theme_import_reports/mod.rs),
enforced by the integration test
`crates/aureline-shell/tests/m5_theme_import_reports_fixtures.rs`. Regenerate
them with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- report > \
  fixtures/ux/m5/theme-import-corpus/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- support-export > \
  fixtures/ux/m5/theme-import-corpus/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- compact > \
  fixtures/ux/m5/theme-import-corpus/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- markdown > \
  artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md
```

The corpus exercises the full honesty spectrum the contract is built to keep
legible — what translated cleanly, what stayed approximate, and what did not map
at all:

- A **VS Code** dark theme that translated every slot and so claims full parity,
  backed by the report and reversible from an appearance checkpoint.
- A **JetBrains** Darcula scheme applied **with warnings**: most slots
  translated, a few fell back to disclosed neutral defaults, and two stayed
  unresolved, so parity is claimed only partially.
- A **Zed** theme that was applied and then **rolled back** because it recolored
  a protected trust cue with color alone — proving imported visual
  customizations stay reversible when they prove semantically misleading.
- A **Vim** colorscheme held for **review** because it leaves the IDE chrome
  unresolved.
- A legacy **TextMate** `tmTheme` that is **blocked** rather than shipped as a
  plausible-looking but mostly unmapped theme.

Every row carries its source provenance and an explicit unresolved-slot count,
and no row claims parity it cannot back. Raw theme files, raw token values, raw
screenshots, raw paths, and raw user content never cross this boundary.
