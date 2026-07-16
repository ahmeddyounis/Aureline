# M5 AI-Review-Assist Surface Certification

- Packet: `m5-ai-review-assist-surface-certification:stable:0001`
- As of: `2026-07-16T00:00:00Z`
- Canonical bundle: `artifacts/review/m5-ai-review-publish-packets/support_export.json`
- Profiles: 6 / 6 certified (2 green, 4 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 4
- Report clean: true

## Profiles

- **cert:certified-ai-review-lane** — profile=certified_ai_review_lane claimed=certified_ai_review_truth certified=certified_ai_review_truth status=green narrowed_axes=0
- **cert:reviewable-ai-review-record-structure** — profile=reviewable_ai_review_record_structure claimed=reviewable_ai_review_record certified=reviewable_ai_review_record status=green narrowed_axes=0
- **cert:disclosed-provider-freshness-partial-profile** — profile=disclosed_provider_freshness_partial_profile claimed=reviewable_ai_review_record certified=provider_freshness_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-diff-scope-profile** — profile=unverified_diff_scope_profile claimed=reviewable_ai_review_record certified=diff_scope_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-publish-target-profile** — profile=unverified_publish_target_profile claimed=reviewable_ai_review_record certified=publish_target_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-finding-lifecycle-profile** — profile=unverified_finding_lifecycle_profile claimed=reviewable_ai_review_record certified=finding_lifecycle_unverified_projection status=yellow narrowed_axes=1
