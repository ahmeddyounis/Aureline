# Work-Item Detail Headers, Transition Sheets, Comment Publish Review, and Offline Handoff Packets

This document is the contract for the M5 packet that keeps provider-linked
work-item mutation honest before anything leaves the machine. The packet is the
canonical export-safe source for detail-header truth, transition review,
comment publish review, and offline handoff continuity across desktop,
CLI/headless, companion, and support surfaces.

- Record kind: `ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews`
- Schema: [`schemas/review/ship-work-item-detail-headers-status-transition-sheets-comment-publish-review-and-offline-handoff-packets-with-side-effect-previews.schema.json`](../../../schemas/review/ship-work-item-detail-headers-status-transition-sheets-comment-publish-review-and-offline-handoff-packets-with-side-effect-previews.schema.json)
- Canonical support export: [`artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/support_export.json`](../../../artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/support_export.json)
- Summary artifact: [`artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews.md`](../../../artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews.md)
- Fixtures: [`fixtures/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/`](../../../fixtures/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/)
- Producer: `aureline_review::current_work_item_mutation_review_export`

## Pillars

### Detail headers

Each `detail_headers[]` row pins the provider boundary, canonical item identity,
state, owner or assignee, sync-state summary, write authority, publish posture,
and open-external truth for one tracked work item. The header stays explicit
about whether the visible state is provider-authoritative, local-draft,
queued-for-publish, or offline-captured rather than letting local continuity
masquerade as remote acceptance.

### Transition sheets

Each `transition_reviews[]` row keeps the current state, requested state, linked
branch or review note, actor authority, permission scope, publish mode, side
effects, and fallback label together in one place. Non-publish paths stay
obvious: `publish_now`, `deferred_publish`, `open_in_provider`, and
`local_draft` remain separate truths.

### Comment publish review

Each `comment_publish_reviews[]` row separates `local_draft_summary` from
`external_post_summary`, then adds `visibility_target_label`,
`notify_behavior_label`, `evidence_refs`, `side_effect_summaries`, and
`fallback_label`. That keeps local note capture, queued publish, browser
handoff, and provider-authoritative comment post from collapsing into one vague
“comment saved” story.

### Offline handoff packets

Each `offline_handoff_packets[]` row preserves the captured note summary, code
links, evidence refs, redaction class, expiry, publish target, retry and export
affordances, and restartability. The packet never treats an offline handoff as
accepted provider truth: `provider_acceptance_class` must stay on a non-final
lane for this contract.

## Track invariant

The `trust_review` block is the hard gate for this lane:

- detail headers disclose provider boundary, canonical identity, state, owner,
  sync state, and open-external truth;
- transition sheets disclose side effects, authority, and publish mode before
  confirm;
- comment review separates local draft from external post and names visibility,
  notification, and fallback truth;
- offline packets preserve redaction and expiry without claiming provider
  acceptance;
- provider outage or policy denial preserves local intent through a draft or
  handoff packet;
- downgrade narrows the lane instead of hiding the loss of proof.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp.
Supported downgrade triggers are `provider_authority_stale`,
`side_effect_preview_missing`, `comment_draft_boundary_missing`,
`publish_mode_ambiguous`, `offline_continuity_stale`,
`redaction_or_expiry_missing`, and `upstream_dependency_narrowed`. The
fixtures show both provider-outage continuity and policy-blocked local-draft
continuity staying valid because the packet preserves intent and labels the
degraded lane explicitly.

## Boundary

Raw provider payloads, raw URLs, raw comment bodies, credentials, browser
session tokens, and live provider responses never cross this boundary. The
packet carries only metadata, typed status and publish-mode truth, side-effect
summaries, evidence refs, redaction state, expiry, and restart/export/retry
affordances.
