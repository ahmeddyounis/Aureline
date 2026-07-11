# M5 Build/Remote-Boundary Component Accessibility & Auto-Narrowing

- Packet: `m5-build-remote-boundary-component-accessibility-parity:stable:0001`
- As of: `2026-07-11T00:00:00Z`
- Families: 8 certified across 8 / 8 frozen families
- Status: 2 green / 6 yellow / 0 red

## Rows

- **a11y:host-boundary-strip** (host_boundary_strip) — family=host_boundary_strip keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=full_truth status=parity
- **a11y:execution-origin-receipt-row** (execution_origin_receipt_row) — family=execution_origin_receipt_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=resolved_truth status=parity
- **a11y:adapter-confidence-chip** (adapter_confidence_chip) — family=adapter_confidence_chip keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: full_truth → degraded (dimension=discovery_confidence_truth, trigger=discovery_drift_hidden) — Adapter confidence partially resolved — chip shown degraded until the build adapter's confidence in the resolved target settles
- **a11y:discovery-diff-card** (discovery_diff_card) — family=discovery_diff_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=stale status=narrowed_disclosed
  - Auto-narrow: resolved_truth → stale (dimension=discovery_confidence_truth, trigger=discovery_drift_hidden) — Discovery proof stale — shown as a stale diff with its last-discovery time, not a fresh resolved target, pending re-discovery
- **a11y:managed-workspace-lifecycle-card** (managed_workspace_lifecycle_card) — family=managed_workspace_lifecycle_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: full_truth → degraded (dimension=lifecycle_state_truth, trigger=lifecycle_state_unstated) — Lifecycle state partially resolved — card shown degraded while reconnecting to the managed control plane, not a fully-live workspace
- **a11y:suspend-resume-rebuild-review-sheet** (suspend_resume_rebuild_review_sheet) — family=suspend_resume_rebuild_review_sheet keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=unverified status=narrowed_disclosed
  - Auto-narrow: resolved_truth → unverified (dimension=continuity_truth, trigger=exact_continuity_overclaimed) — Continuity unverified after rebuild — shown as material-change, not exact continuity, with the preserved-vs-lost state named for review
- **a11y:workspace-expiry-banner** (workspace_expiry_banner) — family=workspace_expiry_banner keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=stale status=narrowed_disclosed
  - Auto-narrow: resolved_truth → stale (dimension=expiry_timing_truth, trigger=expiry_timing_unstated) — Expiry timing stale — shown as a stale expiry clock with its last-known deadline, not a fresh countdown or a generic disconnect, pending control-plane refresh
- **a11y:local-safe-continuation-card** (local_safe_continuation_card) — family=local_safe_continuation_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=unsupported status=narrowed_disclosed
  - Auto-narrow: resolved_truth → unsupported (dimension=continuity_truth, trigger=exact_continuity_overclaimed) — Managed continuity unsupported on this profile — shown as local-safe continuation only, never exact continuity, with preserved files and lost live state named
