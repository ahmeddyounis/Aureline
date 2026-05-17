# Third-party Notice Generation Report

Generated from `artifacts/release/m3/third_party_import_manifest.json`.

| Field | Value |
|---|---|
| Manifest id | `release.third_party_import_manifest.beta` |
| Release candidate | `release_candidate:aureline.2_1_0_beta_1` |
| As of | `2026-05-17T12:00:00Z` |
| Source rows | 15 |
| Protected-path rows | 9 |
| Red-risk reviews | 8 |

## Target Coverage

| Publication target | Row count |
|---|---:|
| `cyclonedx_sbom` | 7 |
| `docs_pack_manifest` | 1 |
| `provenance_statement` | 15 |
| `spdx_sbom` | 9 |
| `third_party_notice` | 8 |

## Generated Notice Inputs

### `third_party_notice`

| Source id | Name | Version | License class | Owner | Gate |
|---|---|---|---|---|---|
| `dep.renderer.wgpu` | wgpu | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.renderer.winit` | winit | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.renderer.softbuffer` | softbuffer | `admitted_by_shell_frame_until_gpu_backend` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.rustybuzz` | rustybuzz-class shaper | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.swash` | swash-class rasterisation | `selected_by_renderer_tradeoff_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.fontdb` | fontdb-class discovery layer | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.accessibility.accesskit` | accesskit-class accessibility bridge | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `import.fonts.noto_subset` | Last-resort bundled Noto-class fallback subset | `not_yet_seeded` | `redistributable_font_expected_pending_import` | `@ahmeddyounis` | `on_first_binary_distribution` |

### `spdx_sbom`

| Source id | Name | Version | License class | Owner | Gate |
|---|---|---|---|---|---|
| `dep.renderer.wgpu` | wgpu | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.renderer.winit` | winit | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.renderer.softbuffer` | softbuffer | `admitted_by_shell_frame_until_gpu_backend` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.rustybuzz` | rustybuzz-class shaper | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.swash` | swash-class rasterisation | `selected_by_renderer_tradeoff_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.fontdb` | fontdb-class discovery layer | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.accessibility.accesskit` | accesskit-class accessibility bridge | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `import.fonts.noto_subset` | Last-resort bundled Noto-class fallback subset | `not_yet_seeded` | `redistributable_font_expected_pending_import` | `@ahmeddyounis` | `on_first_binary_distribution` |
| `dep.repo.rust_toolchain` | rustup-managed Rust toolchain | `rust-toolchain.toml#channel=1.84.0` | `permissive_toolchain_runtime` | `@ahmeddyounis` | `build_tooling_only` |

### `cyclonedx_sbom`

| Source id | Name | Version | License class | Owner | Gate |
|---|---|---|---|---|---|
| `dep.renderer.wgpu` | wgpu | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.renderer.winit` | winit | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.renderer.softbuffer` | softbuffer | `admitted_by_shell_frame_until_gpu_backend` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.rustybuzz` | rustybuzz-class shaper | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.swash` | swash-class rasterisation | `selected_by_renderer_tradeoff_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.fontdb` | fontdb-class discovery layer | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.accessibility.accesskit` | accesskit-class accessibility bridge | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |

### `provenance_statement`

| Source id | Name | Version | License class | Owner | Gate |
|---|---|---|---|---|---|
| `dep.renderer.wgpu` | wgpu | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.renderer.winit` | winit | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.renderer.softbuffer` | softbuffer | `admitted_by_shell_frame_until_gpu_backend` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.rustybuzz` | rustybuzz-class shaper | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.swash` | swash-class rasterisation | `selected_by_renderer_tradeoff_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.fontdb` | fontdb-class discovery layer | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.accessibility.accesskit` | accesskit-class accessibility bridge | `selected_by_adr_0002_until_manifest_admission` | `permissive_oss_expected_pending_admission` | `@ahmeddyounis` | `on_first_manifest_admission` |
| `dep.text.noto_class_fallback_font` | Noto-class fallback font source | `selected_by_adr_0002_until_first_import` | `redistributable_font_expected_pending_import` | `@ahmeddyounis` | `on_first_import` |
| `import.fonts.noto_subset` | Last-resort bundled Noto-class fallback subset | `not_yet_seeded` | `redistributable_font_expected_pending_import` | `@ahmeddyounis` | `on_first_binary_distribution` |
| `dep.repo.rust_toolchain` | rustup-managed Rust toolchain | `rust-toolchain.toml#channel=1.84.0` | `permissive_toolchain_runtime` | `@ahmeddyounis` | `build_tooling_only` |
| `dep.repo.rustup` | rustup | `host_runtime_unpinned` | `permissive_toolchain_runtime` | `@ahmeddyounis` | `host_runtime_record_only` |
| `dep.repo.git_cli` | Git CLI | `host_runtime_unpinned` | `strong_copyleft_runtime` | `@ahmeddyounis` | `host_runtime_record_only` |
| `dep.repo.bash` | bash | `host_runtime_unpinned` | `strong_copyleft_runtime` | `@ahmeddyounis` | `host_runtime_record_only` |
| `dep.benchmark.python3` | python3 runtime | `host_runtime_unpinned` | `permissive_runtime` | `@ahmeddyounis` | `host_runtime_record_only` |
| `import.docs.mirrored_official_pack` | Mirrored official docs pack source | `not_yet_seeded` | `upstream_pack_license_to_verify_on_first_mirror` | `@ahmeddyounis` | `on_pack_publication` |

### `docs_pack_manifest`

| Source id | Name | Version | License class | Owner | Gate |
|---|---|---|---|---|---|
| `import.docs.mirrored_official_pack` | Mirrored official docs pack source | `not_yet_seeded` | `upstream_pack_license_to_verify_on_first_mirror` | `@ahmeddyounis` | `on_pack_publication` |

## Held Until Admission Or First Import

| Source id | Admission state | Notice action | Promotion effect |
|---|---|---|---|
| `dep.renderer.wgpu` | `selected_not_admitted` | `hold_pending_first_admission` | Blocks stronger release claims until dependency admission, license evidence, and upstream health review are complete. |
| `dep.renderer.winit` | `selected_not_admitted` | `emit_third_party_notice_and_sbom_entries` | Blocks stronger release claims until dependency admission, license evidence, and upstream health review are complete. |
| `dep.renderer.softbuffer` | `selected_not_admitted` | `hold_pending_first_admission` | Blocks stronger release claims until dependency admission, license evidence, and upstream health review are complete. |
| `dep.text.rustybuzz` | `selected_not_admitted` | `hold_pending_first_admission` | Blocks stronger release claims until dependency admission, license evidence, and upstream health review are complete. |
| `dep.text.swash` | `selected_not_admitted` | `hold_pending_first_admission` | Blocks stronger release claims until dependency admission, license evidence, and upstream health review are complete. |
| `dep.text.fontdb` | `selected_not_admitted` | `hold_pending_first_admission` | Blocks stronger release claims until dependency admission, license evidence, and upstream health review are complete. |
| `dep.accessibility.accesskit` | `selected_not_admitted` | `hold_pending_first_admission` | Blocks stronger release claims until dependency admission, license evidence, and upstream health review are complete. |
| `dep.text.noto_class_fallback_font` | `selected_not_admitted` | `hold_pending_first_admission` | Blocks stronger release claims until dependency admission, license evidence, and upstream health review are complete. |
| `import.fonts.noto_subset` | `reserved_not_yet_imported` | `hold_pending_first_admission` | Blocks binary distribution until first import records source archive digest, notice text, and redistribution review. |
| `import.docs.mirrored_official_pack` | `reserved_not_yet_imported` | `emit_docs_pack_manifest_attribution` | Docs-pack publication remains attribution-only until mirror metadata and upstream terms are verified. |

## Generation Rule

Notice, SBOM, docs-pack, and provenance rows are rendered from `artifacts/release/m3/third_party_import_manifest.json`. Hand-maintained notice lists are not admitted for the beta release family.
