# Red-risk Dependency Review

Generated from `release.third_party_import_manifest.beta` into `artifacts/security/m3/red_risk_dependency_review.md`.

| Field | Value |
|---|---|
| Release candidate | `release_candidate:aureline.2_1_0_beta_1` |
| As of | `2026-05-17T12:00:00Z` |
| Review rows | 8 |

## Promotion Impact

| Dependency | Owner | Health status | Next review | Promotion effect | Manifest rows |
|---|---|---|---|---|---|
| `dep.renderer.wgpu` | `@ahmeddyounis` | `provisional_high_review_missing` | `2026-06-14` | `blocks_stable_promotion_until_scored_review` | `third_party.dep_renderer_wgpu` |
| `dep.renderer.winit` | `@ahmeddyounis` | `provisional_high_review_missing` | `2026-06-14` | `blocks_stable_promotion_until_scored_review` | `third_party.dep_renderer_winit` |
| `dep.renderer.softbuffer` | `@ahmeddyounis` | `scorecard_missing_for_protected_path` | `2026-06-14` | `blocks_stable_promotion_until_scored_review` | `third_party.dep_renderer_softbuffer` |
| `dep.text.rustybuzz` | `@ahmeddyounis` | `provisional_high_review_missing` | `2026-06-14` | `blocks_stable_promotion_until_scored_review` | `third_party.dep_text_rustybuzz` |
| `dep.text.swash` | `@ahmeddyounis` | `provisional_high_review_missing` | `2026-06-14` | `blocks_stable_promotion_until_scored_review` | `third_party.dep_text_swash` |
| `dep.text.fontdb` | `@ahmeddyounis` | `provisional_high_review_missing` | `2026-06-14` | `blocks_stable_promotion_until_scored_review` | `third_party.dep_text_fontdb` |
| `dep.accessibility.accesskit` | `@ahmeddyounis` | `provisional_high_review_missing` | `2026-06-14` | `blocks_stable_promotion_until_scored_review` | `third_party.dep_accessibility_accesskit` |
| `dep.text.noto_class_fallback_font` | `@ahmeddyounis` | `provisional_high_review_missing` | `2026-06-14` | `blocks_binary_distribution_until_import_terms_verified` | `third_party.dep_text_noto_class_fallback_font`, `third_party.import_fonts_noto_subset` |

## Review Notes

### `dep.renderer.wgpu`

- Risk state: `red`
- Review gaps: `backup_owner_not_named`, `upstream_review_provisional`, `upstream_scorecard_missing`, `license_pending_admission`
- Fork / replace / escalate trigger: Open replacement or local-fork review if a claimed-backend bug or hot-path budget blocker cannot be corrected inside one release family without waiting on upstream.

- Required follow-up: Complete scored upstream health, license, and backup-owner review before widening or stable promotion.
- Decision refs: `artifacts/release/m3/artifact_graph.json`, `artifacts/release/m3/release_center_pack/pack.json`, `docs/release/m3/release_center_beta.md`

### `dep.renderer.winit`

- Risk state: `red`
- Review gaps: `backup_owner_not_named`, `upstream_review_provisional`, `upstream_scorecard_missing`, `license_pending_admission`
- Fork / replace / escalate trigger: Open replacement or local-fork review if claimed-platform windowing, DPI, or accessibility-root requirements can no longer be met without per-platform shell forks.

- Required follow-up: Complete scored upstream health, license, and backup-owner review before widening or stable promotion.
- Decision refs: `artifacts/release/m3/artifact_graph.json`, `artifacts/release/m3/release_center_pack/pack.json`, `docs/release/m3/release_center_beta.md`

### `dep.renderer.softbuffer`

- Risk state: `red`
- Review gaps: `backup_owner_not_named`, `upstream_scorecard_missing`, `license_pending_admission`
- Fork / replace / escalate trigger: Replace the window-present stub once the GPU-backed renderer surface is admitted; keep the shell frame and zone registry contracts intact.

- Required follow-up: Complete scored upstream health, license, and backup-owner review before widening or stable promotion.
- Decision refs: `artifacts/release/m3/artifact_graph.json`, `artifacts/release/m3/release_center_pack/pack.json`, `docs/release/m3/release_center_beta.md`

### `dep.text.rustybuzz`

- Risk state: `red`
- Review gaps: `backup_owner_not_named`, `upstream_review_provisional`, `upstream_scorecard_missing`, `license_pending_admission`
- Fork / replace / escalate trigger: Reopen the shaping choice if a claimed script or locale requires fixes the Rust-native path cannot carry in time and the platform-native path is no longer sufficient.

- Required follow-up: Complete scored upstream health, license, and backup-owner review before widening or stable promotion.
- Decision refs: `artifacts/release/m3/artifact_graph.json`, `artifacts/release/m3/release_center_pack/pack.json`, `docs/release/m3/release_center_beta.md`

### `dep.text.swash`

- Risk state: `red`
- Review gaps: `backup_owner_not_named`, `upstream_review_provisional`, `upstream_scorecard_missing`, `license_pending_admission`
- Fork / replace / escalate trigger: Open replacement review if glyph raster correctness, color-font coverage, or atlas pressure on claimed hosts cannot be corrected without diverging locally.

- Required follow-up: Complete scored upstream health, license, and backup-owner review before widening or stable promotion.
- Decision refs: `artifacts/release/m3/artifact_graph.json`, `artifacts/release/m3/release_center_pack/pack.json`, `docs/release/m3/release_center_beta.md`

### `dep.text.fontdb`

- Risk state: `red`
- Review gaps: `backup_owner_not_named`, `upstream_review_provisional`, `upstream_scorecard_missing`, `license_pending_admission`
- Fork / replace / escalate trigger: Replace the discovery layer if host-specific path behavior or cache invalidation cannot be kept inside the VFS and shell contracts.

- Required follow-up: Complete scored upstream health, license, and backup-owner review before widening or stable promotion.
- Decision refs: `artifacts/release/m3/artifact_graph.json`, `artifacts/release/m3/release_center_pack/pack.json`, `docs/release/m3/release_center_beta.md`

### `dep.accessibility.accesskit`

- Risk state: `red`
- Review gaps: `backup_owner_not_named`, `upstream_review_provisional`, `upstream_scorecard_missing`, `license_pending_admission`
- Fork / replace / escalate trigger: Escalate to replacement or fork planning if accessibility parity on claimed hosts cannot be corrected within the protected release family.

- Required follow-up: Complete scored upstream health, license, and backup-owner review before widening or stable promotion.
- Decision refs: `artifacts/release/m3/artifact_graph.json`, `artifacts/release/m3/release_center_pack/pack.json`, `docs/release/m3/release_center_beta.md`

### `dep.text.noto_class_fallback_font`

- Risk state: `red`
- Review gaps: `backup_owner_not_named`, `upstream_review_provisional`, `upstream_scorecard_missing`, `license_pending_admission`
- Fork / replace / escalate trigger: Replace the source family or subset plan if redistribution terms, signature posture, or glyph coverage can no longer satisfy the renderer fallback contract on the release cadence.

- Required follow-up: Complete scored upstream health, license, and backup-owner review before widening or stable promotion.
- Decision refs: `artifacts/release/m3/artifact_graph.json`, `artifacts/release/m3/release_center_pack/pack.json`, `docs/release/m3/release_center_beta.md`

## Release Rule

A red-risk protected-path dependency cannot be treated as green while owner, backup-owner, license, or upstream-health evidence is missing. Promotion must either refresh the review, narrow the claim, or carry an explicit mitigation.
