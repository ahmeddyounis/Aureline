# M5 Manifest / Build Component Accessibility Fallback

- Packet: `m5-manifest-build-component-accessibility-fallback:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Families: 10 certified across 10 / 10 frozen families
- Status: 6 green / 4 yellow / 0 red
- Claim publication + field triage aligned: true

## Rows

- **a11y:manifest-editor-header** (manifest_editor_header) — family=manifest_editor_header target=target:cluster/prod-us-east/deploy.yaml keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=auto_narrowed_disclosed granted=review_required export=reconstructable_without_screenshot status=narrowed_disclosed
  - Auto-narrow: trigger=schema_stale — The manifest schema is stale for this target, so apply is narrowed to review-required and the target context stays pinned rather than presenting as directly executable
- **a11y:schema-validator-row** (schema_validator_row) — family=schema_validator_row target=target:cluster/prod-us-east/deploy.yaml keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=matches_truth granted=read_only export=reconstructable_without_screenshot status=parity
- **a11y:target-context-chip-group** (target_context_chip_group) — family=target_context_chip_group target=target:cluster/prod-us-east keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=matches_truth granted=inspect_only export=reconstructable_without_screenshot status=parity
- **a11y:resource-link-row** (resource_link_row) — family=resource_link_row target=target:cluster/prod-us-east/svc/api-gateway keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=auto_narrowed_disclosed granted=read_only export=reconstructable_without_screenshot status=narrowed_disclosed
  - Auto-narrow: trigger=connector_loss — The live connector is lost for this resource, so the link opens the rendered truth read-only and marks the live view unavailable rather than implying a live open
- **a11y:resource-explorer-row** (resource_explorer_row) — family=resource_explorer_row target=target:cluster/staging/svc/worker keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=matches_truth granted=fully_executable export=reconstructable_without_screenshot status=parity
- **a11y:adapter-source-badge** (adapter_source_badge) — family=adapter_source_badge target=target:build///app:server keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=matches_truth granted=inspect_only export=reconstructable_without_screenshot status=parity
- **a11y:target-graph-row** (target_graph_row) — family=target_graph_row target=target:build///app:server_test keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled claim=auto_narrowed_disclosed granted=review_required export=reconstructable_without_screenshot status=narrowed_disclosed
  - Auto-narrow: trigger=low_confidence_discovery — The target graph came from a low-confidence heuristic parse, so the run action narrows to review-required and the screen reader gets a labeled node/edge table instead of the canvas
- **a11y:capability-matrix** (capability_matrix) — family=capability_matrix target=target:build///app:server keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=auto_narrowed_disclosed granted=inspect_only export=reconstructable_without_screenshot status=narrowed_disclosed
  - Auto-narrow: trigger=policy_block — A policy block prevents invoking this capability, so the matrix cell narrows to inspect-only and keeps the supported/partial/unsupported state visible rather than offering a blocked run
- **a11y:raw-event-drawer** (raw_event_drawer) — family=raw_event_drawer target=target:build///app:server keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=matches_truth granted=read_only export=reconstructable_without_screenshot status=parity
- **a11y:fallback-confidence-drawer** (fallback_confidence_drawer) — family=fallback_confidence_drawer target=target:build///app:server keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled claim=matches_truth granted=read_only export=reconstructable_without_screenshot status=parity
