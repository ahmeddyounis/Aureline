# M5 live-appearance change & evidence-linkage report

Generated from the seeded report in
[`crate::live_appearance_evidence`](../../../../crates/aureline-shell/src/live_appearance_evidence/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- markdown > \
  artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md
```

- Report id: `shell:m5_live_appearance_evidence:audit:v1`
- Source schema ref: `schemas/ux/m5-live-appearance-evidence.schema.json`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows: 13
- Marketed rows: 12
- Rows needing reload/restart: 2
- Live change demonstrated: `true`
- All captures build-attributed: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-17T00:00:00Z`

## Live OS appearance changes

| Platform | OS signal | Axis | Qualifies | Posture | Capture | Golden | Status |
| -------- | --------- | ---- | --------- | ------- | ------- | ------ | ------ |
| `macos` | `system_theme_flip` | `follow_system` | theme_dark | `applies_live` | `live_transition` | `matched` | `qualified` |
| `windows` | `system_theme_flip` | `follow_system` | theme_light | `applies_live` | `live_transition` | `matched` | `qualified` |
| `linux` | `system_theme_flip` | `follow_system` | theme_dark | `applies_live` | `live_transition` | `matched` | `qualified` |
| `macos` | `contrast_increased` | `contrast` | theme_high_contrast | `applies_live` | `live_transition` | `matched` | `qualified` |
| `windows` | `forced_colors_enabled` | `contrast` | theme_high_contrast | `requires_surface_reload` | `live_transition` | `matched` | `qualified` |
| `linux` | `contrast_increased` | `contrast` | theme_high_contrast | `applies_live` | `live_transition` | `diff_within_tolerance` | `qualified` |
| `macos` | `reduced_motion_enabled` | `reduced_motion` | reduced_motion | `applies_live` | `live_transition` | `matched` | `qualified` |
| `windows` | `reduced_motion_enabled` | `reduced_motion` | reduced_motion | `applies_live` | `live_transition` | `matched` | `qualified` |
| `macos` | `accent_color_changed` | `accent` | — | `applies_live` | `live_transition` | `matched` | `qualified` |
| `windows` | `accent_color_changed` | `accent` | — | `applies_live` | `live_transition` | `matched` | `qualified` |
| `macos` | `text_scale_increased` | `text_scale` | — | `applies_live` | `live_transition` | `matched` | `qualified` |
| `linux` | `text_scale_increased` | `text_scale` | — | `requires_app_restart` | `live_transition` | `matched` | `qualified` |
| `windows` | `forced_colors_enabled` | `contrast` | theme_high_contrast | `platform_signal_unavailable` | `—` | `—` | `platform_omitted` |

## Evidence attribution

| Row | Build | Theme package | Session | Checkpoint |
| --- | ----- | ------------- | ------- | ---------- |
| `live-appearance:macos:system-theme-flip` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@macos` | `appearance-session:macos:system-theme-flip` | `appearance-checkpoint:macos:system-theme-flip` |
| `live-appearance:windows:system-theme-flip` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@windows` | `appearance-session:windows:system-theme-flip` | `appearance-checkpoint:windows:system-theme-flip` |
| `live-appearance:linux:system-theme-flip` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@linux` | `appearance-session:linux:system-theme-flip` | `appearance-checkpoint:linux:system-theme-flip` |
| `live-appearance:macos:contrast-increased` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@macos` | `appearance-session:macos:contrast-increased` | `appearance-checkpoint:macos:contrast-increased` |
| `live-appearance:windows:forced-colors-enabled` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@windows` | `appearance-session:windows:forced-colors-enabled` | `appearance-checkpoint:windows:forced-colors-enabled` |
| `live-appearance:linux:contrast-increased` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@linux` | `appearance-session:linux:contrast-increased` | `appearance-checkpoint:linux:contrast-increased` |
| `live-appearance:macos:reduced-motion-enabled` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@macos` | `appearance-session:macos:reduced-motion-enabled` | `appearance-checkpoint:macos:reduced-motion-enabled` |
| `live-appearance:windows:reduced-motion-enabled` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@windows` | `appearance-session:windows:reduced-motion-enabled` | `appearance-checkpoint:windows:reduced-motion-enabled` |
| `live-appearance:macos:accent-color-changed` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@macos` | `appearance-session:macos:accent-color-changed` | `appearance-checkpoint:macos:accent-color-changed` |
| `live-appearance:windows:accent-color-changed` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@windows` | `appearance-session:windows:accent-color-changed` | `appearance-checkpoint:windows:accent-color-changed` |
| `live-appearance:macos:text-scale-increased` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@macos` | `appearance-session:macos:text-scale-increased` | `appearance-checkpoint:macos:text-scale-increased` |
| `live-appearance:linux:text-scale-increased` | `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2` | `theme-package:aureline.default@linux` | `appearance-session:linux:text-scale-increased` | `appearance-checkpoint:linux:text-scale-increased` |
| `live-appearance:windows:forced-colors-portable-omitted` | `—` | `theme-package:aureline.default@windows` | `appearance-session:windows:forced-colors-portable-omitted` | `appearance-checkpoint:windows:forced-colors-portable-omitted` |

## Cross-platform axis coverage

| Axis | Platforms |
| ---- | --------- |
| `follow_system` | `linux`, `macos`, `windows` |
| `contrast` | `linux`, `macos`, `windows` |
| `reduced_motion` | `macos`, `windows` |
| `accent` | `macos`, `windows` |
| `text_scale` | `linux`, `macos` |

## Surface coverage

Qualified rows exercise: `companion_surface`, `docs_browser_pane`, `notebook_cell_chrome`, `pipeline_card`, `preview_route_badge`, `profiler_panel`, `result_grid_row`, `trace_panel`.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- validate
cargo test -p aureline-shell --test m5_live_appearance_evidence_fixtures
python3 tools/ci/m5/live_appearance_evidence_check.py --repo-root .
```
