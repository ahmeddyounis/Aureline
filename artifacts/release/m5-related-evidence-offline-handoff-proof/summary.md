# Related-evidence cards and offline-handoff packet cards

- Packet: `m5-related-evidence-offline-handoff-controls:stable:0001`
- Surface: `M5 related-evidence cards and offline-handoff packet cards: evidence cards summarize linked reviews, branches/worktrees, failing/passing tests, CI checks, incidents/runbooks, and docs/ADR references summary-first with derived freshness and an open-detail action; offline-handoff packet cards show packet type, included metadata/evidence, redaction state, and publish-later target, staying visible, retryable, and exportable after failure so a held, queued, or failed packet never implies the provider accepted it`
- Related-evidence cards: 6 (1 need attention)
- Offline-handoff packet cards: 8 (1 provider-accepted)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Related-evidence cards

- **evidence-checkout-rounding-tests** (test_result) [failing / current_evidence] → 3 of 128 checkout tests failing on the rounding path
- **evidence-checkout-ci** (ci_check) [passing / current_evidence] → Lint and build checks passing on the latest push
- **evidence-review-thread** (review_thread) [informational / stale_evidence] → Review 482 has 2 open threads from an earlier revision
- **evidence-local-change** (linked_change) [passing / local_only_evidence] → Local worktree change compiles and passes the smoke check
- **evidence-attached-runbook** (attached_artifact) [unknown_outcome / unknown_freshness] → Failover runbook attached; last verification unknown
- **evidence-adr-reference** (external_reference) [informational / current_evidence] → ADR-014 documents the signing-key rotation decision

## Offline-handoff packet cards

- **packet-local-draft-hold** [held_local_only] → `local_queue` boundary: metadata_safe
- **packet-queued-comment** [queued_not_yet_accepted] → `provider_publish` boundary: body_excluded
- **packet-publish-failed** [publish_failed_retryable] → `provider_publish` boundary: identifiers_masked
- **packet-exported-file** [exported_for_handoff] → `exported_packet` boundary: credentials_scrubbed
- **packet-provider-accepted** [provider_accepted] → `provider_publish` boundary: full_disclosure_blocked
- **packet-support-bundle** [exported_for_handoff] → `support_bundle` boundary: local_only
- **packet-another-device** [exported_for_handoff] → `another_device` boundary: metadata_safe
- **packet-discard-review** [exported_for_handoff] → `discard_after_review` boundary: body_excluded
