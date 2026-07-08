# Artifact Identity Bars & Diff-Mode Switchers

- Packet: `artifact-identity-diff-mode-controls:stable:0001`
- Surface: `Artifact identity bars and diff-mode switchers`
- Identity bars: 4 (1 generated, 1 writable targets)
- Diff-mode switchers: 4
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Identity bars

- **Jupyter notebook** [`artifact:notebooks/analysis.ipynb`]: origin `authored_in_repo`, parser `structured_faithful`, writable `true` — Canonical source: notebooks/analysis.ipynb in this repo
- **generated API client** [`artifact:gen/api_client.rs`]: origin `generated_from_source`, parser `structured_faithful`, writable `false` — Canonical source: openapi/spec.yaml; this file is generated output
- **imported SBOM (SPDX)** [`artifact:sbom/spdx.json`]: origin `imported_external`, parser `structured_partial`, writable `false` — Canonical source: build pipeline SBOM export (imported snapshot)
- **design snapshot (media)** [`artifact:design/home.snapshot`]: origin `policy_owned`, parser `render_untrusted`, writable `false` — Canonical source: design system snapshot governed by design policy

## Diff-mode switchers

- **Jupyter notebook** [`artifact:notebooks/analysis.ipynb`]: active `structured_semantic` — structured_semantic=available, side_by_side=available, three_way_merge=available, raw_text_fallback=available
- **generated API client** [`artifact:gen/api_client.rs`]: active `structured_semantic` — structured_semantic=available, side_by_side=available, raw_text_fallback=available
- **imported SBOM (SPDX)** [`artifact:sbom/spdx.json`]: active `structured_semantic` — structured_semantic=available, three_way_merge=unavailable_policy_blocked, raw_text_fallback=available
- **design snapshot (media)** [`artifact:design/home.snapshot`]: active `media_visual` — media_visual=available, structured_semantic=unavailable_schema_unrecognized, raw_text_fallback=available
