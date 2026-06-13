# M5 Raw-versus-Rendered Representation & Handoff

- Packet: `m5-raw-rendered-handoff:stable:0001`
- Case: `case:m5-raw-rendered-handoff:stable`
- Diverging surfaces: 7
- Divergence kinds: rendered_applies_styling, rendered_normalizes_for_comparison, rendered_reflows_layout, rendered_summarizes_content

## Surfaces

- **docs_rendered_panel** (ordinary_browsing): transform `markdown_html_render`, divergence `rendered_reflows_layout`, raw copy reachable: true
- **notebook_rendered_output** (ordinary_browsing): transform `notebook_output_render`, divergence `rendered_applies_styling`, raw copy reachable: true
- **ai_summary_evidence** (ordinary_browsing): transform `ai_summarization`, divergence `rendered_summarizes_content`, raw copy reachable: true
- **review_structured_diff** (ordinary_browsing): transform `diff_normalization`, divergence `rendered_normalizes_for_comparison`, raw copy reachable: true
- **structured_artifact_viewer** (ordinary_browsing): transform `structured_pretty_print`, divergence `rendered_reflows_layout`, raw copy reachable: true
- **marketplace_install_review** (strong_decision_strict_identity): transform `manifest_render`, divergence `rendered_reflows_layout`, raw copy reachable: true
- **policy_review_overlay** (strong_decision_strict_identity): transform `policy_render`, divergence `rendered_reflows_layout`, raw copy reachable: true

## Handoff carriers

- `support_export`: declares rendered != raw: true, preserves note: true
- `screenshot_caption`: declares rendered != raw: true, preserves note: true
- `handoff_packet`: declares rendered != raw: true, preserves note: true
