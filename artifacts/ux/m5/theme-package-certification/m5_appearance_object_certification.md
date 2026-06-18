# M5 appearance-object certification

Generated from the seeded report in
[`crate::appearance_object_certification`](../../../../crates/aureline-shell/src/appearance_object_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- markdown > \
  artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md
```

- Report id: `shell:m5_appearance_object_certification:audit:v1`
- Source schema ref: `schemas/ux/m5-appearance-object-certification.schema.json`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Surfaces certified: 10
- Certified (full): 7
- Auto-narrowed: 3
- Blocked: 0
- All surfaces publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-17T00:00:00Z`

## Canonical appearance-object index

| Family | Canonical schema | Source report | Contract |
| ------ | ---------------- | ------------- | -------- |
| Theme package | `schemas/ux/m5-theme-package-manifest.schema.json` | `shell:m5_theme_packages:audit:v1` | `shell:m5_theme_packages:v1` |
| Appearance session | `schemas/ux/appearance-session.schema.json` | `shell:m5_appearance_session:runtime:v1` | `shell:m5_appearance_session:v1` |
| Token overlay | `schemas/ux/token-overlay.schema.json` | `shell:m5_token_overlays:portability:v1` | `shell:m5_token_overlays:v1` |
| Imported-theme report | `schemas/ux/m5-theme-import-report.schema.json` | `shell:m5_theme_import_report:v1:default` | `shell:m5_theme_import_report:v1` |
| Extension appearance descriptor | `schemas/ux/extension-appearance-descriptor.schema.json` | `extensions:m5_appearance_descriptor:audit:v1` | `extensions:m5_appearance_descriptor:v1` |

## Per-surface certification

| Surface | Lifecycle | Scope | Theme package | Appearance session | Token overlay | Imported theme | Extension |
| ------- | --------- | ----- | ------------- | ------------------ | ------------- | -------------- | --------- |
| Notebook cell chrome | `stable` | `certified_full` | `qualified` | `qualified` | `qualified` | `qualified` | `not_applicable` |
| Result-grid row | `stable` | `certified_full` | `qualified` | `qualified` | `qualified` | `qualified` | `not_applicable` |
| Profiler panel | `stable` | `certified_full` | `qualified` | `qualified` | `qualified` | `qualified` | `not_applicable` |
| Trace panel | `stable` | `certified_full` | `qualified` | `qualified` | `qualified` | `qualified` | `not_applicable` |
| Pipeline card | `stable` | `certified_full` | `qualified` | `qualified` | `qualified` | `qualified` | `not_applicable` |
| Preview-route badge | `stable` | `certified_narrowed` | `qualified` | `qualified` / `restart_or_reload_required` | `qualified` | `qualified` | `qualified` |
| Docs / browser pane | `stable` | `certified_narrowed` | `qualified` | `qualified` | `qualified` | `qualified` | `qualified` / `partial_inheritance` |
| Companion surface | `stable` | `certified_full` | `qualified` | `qualified` | `qualified` | `qualified` | `not_applicable` |
| Sync status surface | `stable` | `certified_full` | `qualified` | `qualified` | `qualified` | `qualified` | `not_applicable` |
| Offboarding surface | `stable` | `certified_narrowed` | `qualified` | `qualified` | `explicitly_narrowed` / `unsupported_slot` | `qualified` | `not_applicable` |

## Auto-narrowed surfaces

- `preview_route_badge` (`certified_narrowed`) — Embedded preview certifies every appearance object, but its appearance session discloses a reload-required posture for forced-colors, so the surface markets the narrowed appearance story.
- `docs_browser_pane` (`certified_narrowed`) — The embedded docs/help pane certifies its host appearance objects and discloses partial extension density inheritance, so the surface markets the narrowed story.
- `offboarding_surface` (`certified_narrowed`) — The offboarding / export-and-wipe surface narrows token-overlay round-trip and markets only the narrowed appearance story.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- validate
cargo test -p aureline-shell --test m5_appearance_object_certification_fixtures
python3 tools/ci/m5/appearance_object_certification_check.py --repo-root .
```
