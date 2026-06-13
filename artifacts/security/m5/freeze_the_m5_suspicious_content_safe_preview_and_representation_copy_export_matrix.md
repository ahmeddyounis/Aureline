# M5 Suspicious-Content, Safe-Preview, and Copy/Export Matrix

- Packet: `m5-content-integrity-matrix:stable:0001`
- Label: `M5 Suspicious-Content, Safe-Preview, and Representation-Labeled Copy/Export Matrix`
- Families: 10 (7 stable)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-10T00:00:00Z)

## Families

- **notebook_rich_output**: `beta`
  - Scope: Notebook rich-output blocks climb a trust-class ladder from raw text to sanitized rich and isolated remote active; suspicious text stays annotated, raw cells stay inspectable, and active output never executes outside its declared isolated runtime
  - Raw/rendered: raw_and_rendered_distinct_both_reachable · Active content: isolated_remote_sandbox_only
  - Copy/export: raw_and_escaped_labeled_distinct · Safe preview: limited_mode_available · Display: ordinary_browsing
- **docs_browser_panel**: `stable`
  - Scope: Docs and in-product browser panels render sanitized rich content by default with raw source reachable on demand; remote active content is confined to an isolated runtime and never crosses into the trusted-local class
  - Raw/rendered: rendered_default_raw_on_demand · Active content: isolated_remote_sandbox_only
  - Copy/export: escaped_default_raw_reachable · Safe preview: full_preview · Display: ordinary_browsing
- **ai_evidence_viewer**: `stable`
  - Scope: AI evidence and finding-card viewers present raw and sanitized representations as distinctly labeled forms; quoted model and tool output is inert, never executes, and raw inspection of the underlying evidence stays reachable
  - Raw/rendered: raw_and_rendered_distinct_both_reachable · Active content: inert_never_executes
  - Copy/export: raw_and_escaped_labeled_distinct · Safe preview: full_preview · Display: ordinary_browsing
- **pipeline_artifact_browser**: `stable`
  - Scope: Pipeline run and artifact browsers render logs and artifact previews through the safe-preview boundary with raw download reachable; untrusted artifact bodies stay inert and are blocked rather than executed
  - Raw/rendered: rendered_default_raw_on_demand · Active content: inert_never_executes
  - Copy/export: escaped_default_raw_reachable · Safe preview: limited_mode_available · Display: ordinary_browsing
- **provider_overlay**: `stable`
  - Scope: Provider account and policy overlays render owner and origin identity in strong-decision strict mode; embedded provider content is inert and suspicious identifiers are annotated rather than silently normalized
  - Raw/rendered: rendered_default_raw_on_demand · Active content: inert_never_executes
  - Copy/export: escaped_default_raw_reachable · Safe preview: full_preview · Display: strong_decision_strict_identity
- **marketplace_install_update**: `stable`
  - Scope: Marketplace install and update surfaces render publisher identity in strong-decision strict mode; active payloads are blocked pending review, raw and rendered manifests are labeled distinctly, and confusable publisher names are surfaced
  - Raw/rendered: raw_and_rendered_distinct_both_reachable · Active content: blocked_pending_review
  - Copy/export: raw_and_escaped_labeled_distinct · Safe preview: limited_mode_default · Display: strong_decision_strict_identity
- **remote_preview_target**: `beta`
  - Scope: Remote preview targets render in strong-decision strict mode with limited preview by default; remote active content is confined to an isolated runtime and the raw target identity stays reachable for inspection
  - Raw/rendered: rendered_default_raw_on_demand · Active content: isolated_remote_sandbox_only
  - Copy/export: escaped_default_raw_reachable · Safe preview: limited_mode_default · Display: strong_decision_strict_identity
- **incident_export_packet**: `stable`
  - Scope: Incident and support/export packets carry redaction-safe metadata only; no raw suspicious body crosses the export boundary and quoted content is inert with no executable form
  - Raw/rendered: raw_only_no_rendering · Active content: inert_never_executes
  - Copy/export: metadata_only_no_raw_body · Safe preview: limited_mode_default · Display: ordinary_browsing
- **generated_artifact**: `beta`
  - Scope: Generated artifacts preserve raw and rendered forms as distinctly labeled representations; generated content is inert, never executes on view, and raw copy stays reachable whenever the rendered form differs
  - Raw/rendered: raw_and_rendered_distinct_both_reachable · Active content: inert_never_executes
  - Copy/export: raw_and_escaped_labeled_distinct · Safe preview: limited_mode_available · Display: ordinary_browsing
- **structured_compare_view**: `stable`
  - Scope: Structured compare and diff views render raw and rendered forms side by side with both reachable; compared content is inert, never executes, and rendered copy never masquerades as raw bytes
  - Raw/rendered: raw_and_rendered_distinct_both_reachable · Active content: inert_never_executes
  - Copy/export: raw_and_escaped_labeled_distinct · Safe preview: full_preview · Display: ordinary_browsing
