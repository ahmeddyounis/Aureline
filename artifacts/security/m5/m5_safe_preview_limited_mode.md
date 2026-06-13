# M5 Safe-Preview Limited Mode & Expensive-Render Guards

- Packet: `m5-safe-preview-limited-mode:stable:0001`
- Case: `case:m5-safe-preview-limited-mode:stable`
- Limited-mode artifacts: 6 of 6
- Oversized: 2 · generated: 5 · guarded render: 4
- Budgets: 262144 bytes / 5000 lines

## Artifacts

- **build_log** (`pipeline:run:128:log:build`): 2400000 bytes / 84000 lines → open `safe_preview_limited`
  - Canonical source: `pipeline:run:128` · generated: false · guarded: true
  - Banners: oversized, limited_preview, expensive_render_guarded
  - Actions: open_raw (available_immediately), open_canonical_source (available_immediately), expand_full_render (requires_explicit_opt_in)
- **dependency_lockfile** (`workspace:lockfile:Cargo.lock`): 61000 bytes / 1200 lines → open `safe_preview_limited`
  - Canonical source: `workspace:manifest:Cargo.toml` · generated: true · guarded: true
  - Banners: generated_artifact, limited_preview, expensive_render_guarded
  - Actions: open_raw (available_immediately), open_canonical_source (available_immediately), expand_full_render (requires_explicit_opt_in)
- **test_snapshot** (`test:snapshot:ui:home:1`): 12000 bytes / 300 lines → open `safe_preview_limited`
  - Canonical source: `test:case:ui_home_renders` · generated: true · guarded: false
  - Banners: generated_artifact, limited_preview
  - Actions: open_raw (available_immediately), open_canonical_source (available_immediately), expand_full_render (available_immediately)
- **distribution_bundle** (`release:bundle:app@2.0.0`): 52000000 bytes / 1 lines → open `safe_preview_limited`
  - Canonical source: `build:inputs:app@2.0.0` · generated: true · guarded: true
  - Banners: oversized, generated_artifact, limited_preview, active_content_guarded
  - Actions: open_raw (available_immediately), open_canonical_source (available_immediately), expand_full_render (requires_explicit_opt_in)
- **evidence_packet** (`incident:evidence:packet:42`): 92000 bytes / 2100 lines → open `safe_preview_limited`
  - Canonical source: `incident:42:underlying-records` · generated: true · guarded: true
  - Banners: generated_artifact, limited_preview, expensive_render_guarded
  - Actions: open_raw (available_immediately), open_canonical_source (available_immediately), expand_full_render (requires_explicit_opt_in)
- **generated_artifact** (`codegen:output:api_client.rs`): 8400 bytes / 210 lines → open `safe_preview_limited`
  - Canonical source: `codegen:spec:openapi.yaml` · generated: true · guarded: false
  - Banners: generated_artifact, limited_preview
  - Actions: open_raw (available_immediately), open_canonical_source (available_immediately), expand_full_render (available_immediately)
