# High-contrast signoff and live-change audit

Generated from `fixtures/ux/m3/appearance_evidence/appearance_evidence_packet.json`. Do not edit by hand; refresh the upstream truth and re-run the generator.

## High-contrast signoff

- Signoff state: `passed`
- Build identity: `build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`
- Version: `2.1.0-beta.1`
- Minimum text-contrast targets: high-contrast dark `7.0`, high-contrast light `7.0`

### First-party high-contrast rows

| Surface | Theme | State | Focus legible | Color-only absent |
| --- | --- | --- | --- | --- |
| command_palette | high_contrast_dark | pending | yes | yes |
| search_surface | high_contrast_light | completed | yes | yes |
| notification_envelope | high_contrast_dark | degraded | yes | yes |
| help_about_row | high_contrast_light | completed | yes | yes |

### Protected cues under high contrast / forced colors

| Cue | High contrast | Forced colors |
| --- | --- | --- |
| trust | yes | yes |
| policy_lock | yes | yes |
| severity | yes | yes |
| source_integrity | yes | yes |

### Contributed high-contrast support

| Extension | Surface | High-contrast support | Version |
| --- | --- | --- | --- |
| dev.aureline.samples/markdown-lens | Markdown preview pane | `full_inheritance` | 2.1.0 |
| com.acme.dashboards | Insights dashboard panel | `reduced_support` | 4.3.0-beta.2 |
| io.contrib.theme-extras | Theme settings surface | `reduced_support` | 0.9.0-beta.1 |
| net.legacy.toolbar | Custom toolbar surface | `unsupported_private_styling` | mirror:offline-bundle |

## Live OS appearance-change audit

- Audit state: `passed`
- Build identity: `build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`
- Claimed profiles: 6
- Matrix rows: 90
- Live (no review) axes: mode_theme_class, accent_source
- Live (checkpointed) axes: contrast_mode, density_class, reduced_motion_posture
- Confirm-required axes: text_scale, follow_system_posture
- Embedded reload-required rows: 26
- Full-restart-required rows: 0
- All reload/restart rows disclosed: yes
- Forced-colors rows explicit: yes

## Importer coverage

- Coverage state: `partial_with_visible_gaps`
- Source: vscode 1.86 -> light_parity
- Translated slots: 2
- Substituted with fallback: 1
- Unsupported slots: 1
- Unresolved mappings: 2
- Unresolved-slot total: 3
- Syntax coverage: 88%
- Syntax unresolved scopes: 5
- Protected cues preserved: yes

## Verify

```sh
python3 ci/check_m3_appearance_evidence.py --repo-root . --check
```
