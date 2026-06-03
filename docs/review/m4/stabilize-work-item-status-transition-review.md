# Stabilize work-item status-transition review

## Overview

This document describes the stable work-item status-transition review packet that
turns beta work-item detail, transition, and offline-handoff records into one
inspectable review packet.

## Work-item detail headers

Every claimed work-item lane surfaces a `StableWorkItemDetailRecord` with:

- `canonical_work_item_id` — the canonical provider-owned ID visible to the user.
- `provider_owned_id` — the opaque provider object ID.
- `write_mode_disclosure_class` — read-only, comment/link, full-edit,
  offline-capture-only, policy-blocked.
- `row_posture_class` — provider-authoritative, cached-stale, read-only,
  policy-blocked, local-draft, queued, offline-captured.
- `freshness_class` — live, warm, degraded, unverifiable, imported, local-draft.
- State visibility flags — provider-authoritative, local-draft, sync-pending,
  offline-captured.

## Status-transition sheets

Before any mutation, a `StableStatusTransitionSheetRecord` previews exact provider
side effects:

- Comment creation.
- Status change.
- Assignee change.
- Label add/remove.
- Branch link create/update.
- Review link create/update.

Each side effect carries a `side_effect_class`, `target_scope`, and
`summary_label` so the user can inspect the full effect before confirming.

## Offline handoff packets

Offline handoff packets are first-class and durable. Every
`StableOfflineHandoffRecord` preserves:

- `canonical_work_item_ids` — the provider-owned IDs.
- `selected_evidence_refs` — evidence selected for the handoff.
- `redaction_choices` — redactions applied before publish.
- `queued_actions` — actions queued for later publish.
- `publish_target` — the target provider project/board/space.
- `expires_at` — the expiry timestamp.
- `retry_semantics`, `export_semantics`, `discard_semantics` — lifecycle actions.
- `survives_restart`, `survives_reconnect`, `survives_export_import` — durability
  flags.

## Publish-later continuity

`StablePublishLaterContinuityRecord` preserves the queued intent across restart
and reconnect with:

- `continuity_id` — stable continuity identity.
- `work_item_detail_id_ref` — link back to the work-item detail.
- `continuity_state` — queued_pending, queued_ready, queued_published,
  queued_failed, queued_cancelled.
- `queued_action_refs` — the queued actions.
- `expires_at` — the expiry timestamp.

## Inspection record

The `StableWorkItemInspectionRecord` exposes compact boolean projections so CLI
and inspector surfaces can answer:

- Is provider-authoritative state visible?
- Is local-draft state visible?
- Is offline-captured state visible?
- Is confirm action available?
- Does any offline handoff survive restart/reconnect?
- Is any publish-later continuity pending?
- Is the review actionable?

## Schema

`schemas/review/stable_work_item_status_transition_review.schema.json`

## Fixture

`fixtures/review/m4/stabilize-work-item-status-transition-review/packet.json`
