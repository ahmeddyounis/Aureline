# M5 author-and-publish-preview matrix — human-readable rendering

Human-readable rendering of the canonical M5 author-side and publish-preview matrix.
This row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at
`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`.

## Per-family author lane

| Family | Runtime / host-ABI | Workspace | Signing | Declared → published trust | Hot reload | Conformance | Anti-abuse | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| first_party_framework_pack | wasm_capability_sandbox / local_machine | source_present_built | signed_verified | enterprise_approved → enterprise_approved | no_widening | conformant | disclosed_clean | ready_to_publish |
| docs_pack | passive_package / no_code_execution | source_present_built | signed_verified | verified_publisher → verified_publisher | relaunch_only | not_run | publisher_loss_history_disclosed | publishable_with_warnings |
| local_model_pack | external_host / external_process | source_present_built | unsigned_local_dev | verified_publisher → **unsigned_local_only** | no_widening | partial | disclosed_clean | publishable_with_warnings |
| signed_recipe_pack | declarative_host_rendered_view / managed_host | source_present_built | signed_verified | verified_publisher → verified_publisher | permissions_widened_pending_review | bridge_conformant | disclosed_clean | blocked_from_publish |
| template_artifact | passive_package / browser_runtime | build_failed | signed_unverified | registry_bound → registry_bound | runtime_class_widened_pending_review | retest_pending | disclosed_clean | blocked_from_publish |
| bridge_backed_package | compatibility_bridge / managed_host | source_present_built | signed_verified | verified_publisher → verified_publisher | external_executable_added_pending_review | bridge_conformant | disclosed_clean | blocked_from_publish |
| side_loaded_package | wasm_capability_sandbox / local_machine | source_missing | unsigned_sideload | registry_bound → **unsigned_local_only** | no_widening | failed | undisclosed | blocked_from_publish |
| mirrored_registry_variant | remote_side_component / remote_target | source_present_built | revoked_signature | enterprise_approved → **unsigned_local_only** | no_widening | conformant | quarantined | withheld_quarantined |

## Findings (blockers vs warnings)

- **first_party_framework_pack** — none (ready to publish).
- **docs_pack** — `conformance_incomplete` (warning), `publisher_loss_history`
  (warning).
- **local_model_pack** — `provenance_unverified` (warning), `conformance_incomplete`
  (warning).
- **signed_recipe_pack** — `hot_reload_permissions_widened` (blocker).
- **template_artifact** — `build_failed` (blocker), `provenance_unverified` (warning),
  `hot_reload_runtime_widened` (blocker), `conformance_incomplete` (warning).
- **bridge_backed_package** — `hot_reload_external_executable_added` (blocker).
- **side_loaded_package** — `source_missing` (blocker), `provenance_unverified`
  (warning), `conformance_failed` (blocker), `anti_abuse_undisclosed` (blocker).
- **mirrored_registry_variant** — `signature_revoked` (blocker),
  `anti_abuse_quarantined` (blocker).

## Summary

- 8 families, one author row each.
- 1 ready to publish, 2 publishable with warnings, 4 blocked, 1 withheld
  (quarantined).
- 5 families carry at least one blocker; 4 carry at least one warning.
- 3 families publish as local-only; 4 publish with a verified-publisher or
  enterprise-approved badge.
- The unsigned local-model pack and the revoked mirrored variant both requested a
  trusted badge and were capped to `unsigned_local_only`, proving author-side packages
  never inherit end-user registry trust.
