# M5 Review-Pack Surface Certification

- Packet: `m5-review-pack-surface-certification:stable:0001`
- As of: `2026-07-16T00:00:00Z`
- Canonical bundle: `artifacts/review/m5-review-pack-results/support_export.json`
- Profiles: 8 / 8 certified (2 green, 6 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 6
- Report clean: true

## Profiles

- **cert:certified-review-pack-lane** — profile=certified_review_pack_lane claimed=certified_review_pack_truth certified=certified_review_pack_truth status=green narrowed_axes=0
- **cert:reviewable-review-pack-record-structure** — profile=reviewable_review_pack_record_structure claimed=reviewable_review_pack_record certified=reviewable_review_pack_record status=green narrowed_axes=0
- **cert:stale-pack-version-digest-profile** — profile=stale_pack_version_digest_profile claimed=reviewable_review_pack_record certified=pack_version_digest_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-owner-provenance-profile** — profile=unverified_owner_provenance_profile claimed=reviewable_review_pack_record certified=owner_provenance_unverified_projection status=yellow narrowed_axes=1
- **cert:unevaluated-required-check-profile** — profile=unevaluated_required_check_profile claimed=reviewable_review_pack_record certified=evidence_check_unverified_projection status=yellow narrowed_axes=1
- **cert:local-only-parity-profile** — profile=local_only_parity_profile claimed=reviewable_review_pack_record certified=local_parity_unverified_projection status=yellow narrowed_axes=1
- **cert:undisclosed-ai-pack-binding-profile** — profile=undisclosed_ai_pack_binding_profile claimed=reviewable_review_pack_record certified=ai_pack_binding_unverified_projection status=yellow narrowed_axes=1
- **cert:stale-template-attribution-profile** — profile=stale_template_attribution_profile claimed=reviewable_review_pack_record certified=template_attribution_unverified_projection status=yellow narrowed_axes=1
