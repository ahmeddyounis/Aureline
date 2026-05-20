# Appearance parity evidence packet

Generated from `fixtures/ux/m3/appearance_evidence/appearance_evidence_packet.json`. Do not edit by hand; refresh the upstream truth and re-run the generator.

- Packet state: `current_packet`
- As of: `2026-05-20`
- Release candidate: `release_candidate:aureline.2_1_0_beta_1`
- Version: `2.1.0-beta.1`
- Build identity: `build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`
- Source revision: `commit:b7ee32adb5ebf319adbc0c20155550926ffe90f2`

## Source packets

| Source | Ref | Captured | Freshness | Age (days) |
| --- | --- | --- | --- | ---: |
| appearance_session | `fixtures/ux/m3/theme_import_and_live_change/appearance_session_beta_contract.json` | 2026-04-29T09:00:00Z | `current` | 21 |
| component_state_token | `artifacts/ux/m3/component_state_screenshot_diff/packet.json` | 2026-05-20T00:00:00Z | `current` | 0 |
| extension_inheritance | `fixtures/extensions/m3/appearance_inheritance/conformance_packet.json` | 2026-05-20T00:00:00Z | `current` | 0 |

## Summary

| Metric | Value |
| --- | ---: |
| Appearance rows | 14 |
| First-party rows | 10 |
| Contributed rows | 4 |
| Appearance-safe rows | 11 |
| Appearance-safe (with caveats) rows | 1 |
| Unverified rows | 1 |
| Private-styling rows | 1 |
| Downgraded rows | 0 |
| Extension gap rows | 4 |
| Extension needs-review rows | 1 |
| High-contrast signoff | `passed` |
| Live-update audit | `passed` |
| Importer coverage | `partial_with_visible_gaps` |

## Appearance rows

| Surface | Owner | Decision | Themes proven | Freshness | Packet state |
| --- | --- | --- | --- | --- | --- |
| Activity-center row | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Command palette | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Dialog sheet | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Help / About row | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Notification envelope | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Search surface | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Settings root | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Shell chrome | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Start Center | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Trust prompt | first_party | `appearance_safe` | dark_reference, light_parity, high_contrast_dark, high_contrast_light | `current` | `current_packet` |
| Markdown preview pane | contributed | `appearance_safe` | theme=full_inheritance, hc=full_inheritance | `current` | `current_packet` |
| Insights dashboard panel | contributed | `appearance_safe_with_caveats` | theme=full_inheritance, hc=reduced_support | `current` | `current_packet` |
| Theme settings surface | contributed | `appearance_unverified` | theme=full_inheritance, hc=reduced_support | `current` | `current_packet` |
| Custom toolbar surface | contributed | `appearance_private_styling` | theme=full_inheritance, hc=unsupported_private_styling | `current` | `current_packet` |

## Downgrade propagation

When the packet goes stale or red, these surfaces downgrade their claimed appearance rows:

| Surface | Class | Downgrades on | Ref |
| --- | --- | --- | --- |
| help:release_truth_surfaces | help | stale_evidence, red_evidence | `docs/help/m3/release_truth_surfaces.md` |
| docs:appearance_evidence_consumption_guide | docs | stale_evidence, red_evidence | `docs/ux/m3/appearance_evidence_consumption_guide.md` |
| docs:appearance_session_beta_contract | docs | stale_evidence, red_evidence | `docs/ux/m3/appearance_session_beta_contract.md` |
| docs:component_state_and_token_beta_contract | docs | stale_evidence, red_evidence | `docs/ux/m3/component_state_and_token_beta_contract.md` |
| marketplace:appearance_conformance_beta | marketplace | stale_evidence, red_evidence, needs_review | `docs/extensions/m3/appearance_conformance_beta.md` |
| marketplace:marketplace_fact_grid_beta | marketplace | stale_evidence, red_evidence, needs_review | `docs/extensions/m3/marketplace_fact_grid_beta.md` |
| marketplace:marketplace_truth_beta | marketplace | stale_evidence, red_evidence, needs_review | `docs/ux/m3/marketplace_truth_beta.md` |
| support:support_bundle_contract | support | stale_evidence, red_evidence | `docs/support/support_bundle_contract.md` |

## Verify

```sh
python3 ci/check_m3_appearance_evidence.py --repo-root . --check
```
