# M5 Execution-Lifecycle Component Accessibility & Auto-Narrowing

- Packet: `m5-execution-lifecycle-accessibility-fallback:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Families: 7 certified across 7 / 7 frozen families
- Status: 2 green / 5 yellow / 0 red

## Rows

- **a11y:run-attempt-header** (run_attempt_header) — family=run_attempt_header keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_interactive effective_claim=full_interactive status=parity
- **a11y:input-request-prompt** (input_request_prompt) — family=input_request_prompt keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_interactive effective_claim=inspect_only status=narrowed_disclosed
  - Auto-narrow: full_interactive → inspect_only (dimension=input_state, trigger=input_consequence_unknown) — Approval blocked by policy — prompt is inspect-only
- **a11y:artifact-publish-row** (artifact_publish_row) — family=artifact_publish_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_interactive effective_claim=read_only status=narrowed_disclosed
  - Auto-narrow: full_interactive → read_only (dimension=artifact_freshness, trigger=artifact_retention_expired) — Artifact retention expired — lineage copyable, re-open disabled
- **a11y:rerun-comparison-sheet** (rerun_comparison_sheet) — family=rerun_comparison_sheet keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=review_required effective_claim=review_required status=parity
- **a11y:debug-session-header** (debug_session_header) — family=debug_session_header keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_interactive effective_claim=review_required status=narrowed_disclosed
  - Auto-narrow: full_interactive → review_required (dimension=target_identity, trigger=connector_lost) — Attach target ambiguous — control gated behind target review
- **a11y:thread-process-tree** (thread_process_tree) — family=thread_process_tree keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_interactive effective_claim=inspect_only status=narrowed_disclosed
  - Auto-narrow: full_interactive → inspect_only (dimension=target_identity, trigger=connector_lost) — Live connector lost — tree is captured, inspect-only
- **a11y:dump-crash-artifact-card** (dump_crash_artifact_card) — family=dump_crash_artifact_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=read_only effective_claim=inspect_only status=narrowed_disclosed
  - Auto-narrow: read_only → inspect_only (dimension=mapping_quality, trigger=symbols_unavailable) — Symbols unavailable — frames shown unsymbolicated, inspect-only
