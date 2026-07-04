# M5 Execution-Lifecycle Component Surface Certification

- Packet: `m5-execution-lifecycle-surface-certification:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Bundle: `artifacts/release/m5-execution-lifecycle-component-proof/support_export.json`
- Surfaces: 12 certified across 12 / 12 claimed surfaces
- Status: 7 green / 5 yellow / 0 red

## Surfaces

- **cert:task-execution** (Task execution) — surface=task_execution run_attempt=certified input=certified artifact=certified rerun=certified debug=certified export=certified declared=full_interactive effective=full_interactive status=certified
- **cert:test-execution** (Test execution) — surface=test_execution run_attempt=certified input=not_applicable artifact=certified rerun=certified debug=certified export=certified declared=full_interactive effective=full_interactive status=certified
- **cert:notebook-execution** (Notebook execution) — surface=notebook_execution run_attempt=certified input=certified artifact=certified rerun=not_applicable debug=certified export=certified declared=full_interactive effective=full_interactive status=certified
- **cert:publish-execution** (Publish execution) — surface=publish_execution run_attempt=certified input=not_applicable artifact=certified rerun=certified debug=not_applicable export=certified declared=full_interactive effective=full_interactive status=certified
- **cert:request-execution** (Request execution) — surface=request_execution run_attempt=certified input=certified artifact=certified rerun=disclosed_narrowed debug=not_applicable export=certified declared=full_interactive effective=review_required status=narrowed_disclosed
  - Auto-narrow: full_interactive → review_required (group=rerun_review, trigger=rerun_context_drift) — Remote request context drifted — rerun gated behind context review
- **cert:database-execution** (Database execution) — surface=database_execution run_attempt=certified input=certified artifact=disclosed_narrowed rerun=not_applicable debug=not_applicable export=certified declared=full_interactive effective=read_only status=narrowed_disclosed
  - Auto-narrow: full_interactive → read_only (group=artifact_publish, trigger=artifact_retention_expired) — Managed result retention expired — lineage copyable, re-open disabled
- **cert:ai-execution** (AI-mediated execution) — surface=ai_execution run_attempt=certified input=disclosed_narrowed artifact=certified rerun=certified debug=certified export=certified declared=full_interactive effective=review_required status=narrowed_disclosed
  - Auto-narrow: full_interactive → review_required (group=input_request, trigger=input_consequence_unknown) — Provider-backed approval consequence deferred — answer gated behind review
- **cert:preview-execution** (Preview execution) — surface=preview_execution run_attempt=certified input=not_applicable artifact=disclosed_narrowed rerun=not_applicable debug=not_applicable export=certified declared=full_interactive effective=read_only status=narrowed_disclosed
  - Auto-narrow: full_interactive → read_only (group=artifact_publish, trigger=artifact_retention_expired) — Container preview build stale — artifact read-only pending refresh
- **cert:debug-execution** (Debug execution) — surface=debug_execution run_attempt=certified input=not_applicable artifact=not_applicable rerun=certified debug=disclosed_narrowed export=certified declared=full_interactive effective=inspect_only status=narrowed_disclosed
  - Auto-narrow: full_interactive → inspect_only (group=debug_hierarchy, trigger=connector_lost) — Remote debug connector dropped — hierarchy captured, inspect-only
- **cert:support-export-replay** (Support / export replay) — surface=support_export_replay run_attempt=certified input=certified artifact=certified rerun=certified debug=certified export=certified declared=read_only effective=read_only status=certified
- **cert:docs-help-embeds** (Docs / help embeds) — surface=docs_help_embeds run_attempt=certified input=not_applicable artifact=not_applicable rerun=certified debug=not_applicable export=certified declared=inspect_only effective=inspect_only status=certified
- **cert:release-proof** (Release proof) — surface=release_proof run_attempt=certified input=not_applicable artifact=certified rerun=not_applicable debug=certified export=certified declared=read_only effective=read_only status=certified
