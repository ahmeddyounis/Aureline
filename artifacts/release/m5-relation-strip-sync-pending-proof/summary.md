# Relation strips and sync-pending pills

- Packet: `m5-relation-strip-sync-pending-controls:stable:0001`
- Surface: `M5 relation strips and sync-pending pills: linked branch/review/test/incident context with derived stale/broken relation labeling and metadata-safe copy/open actions, plus pending comment/transition/link/field-edit/create pills that read visibly differently from provider-confirmed state and stay recoverable via retry or export when publish fails or the provider is offline`
- Relation strips: 2 (3 non-current relations)
- Sync-pending pills: 5 (4 not provider-confirmed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Relation strips

- **strip-checkout-rounding** (PROJ-1421):
  - linked_branch → `feature/checkout-rounding (3 commits ahead)` [current]
  - linked_review → `review #482 (2 unresolved threads)` [stale]
  - linked_test_run → `ci run 9921 checkout-suite` [broken]
  - unmapped_relation → `imported link (unresolved target)` [unmapped]
- **strip-failover-incident** (INC-3390):
  - linked_incident → `incident bridge #77` [current]
  - linked_pull_request → `PR #1290 failover hotfix` [current]

## Sync-pending pills

- **pill-confirmed-comment** pending_comment [synced_with_provider] → `provider_confirmed`
- **pill-pending-transition** pending_transition [queued_for_publish] → `pending_publish`
- **pill-failed-link** pending_link [publish_failed] → `recoverable_failure`
- **pill-offline-field-edit** pending_field_edit [local_only_draft] → `offline_held`
- **pill-policy-blocked-create** pending_create [conflict_held] → `policy_blocked`
