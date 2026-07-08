# Pending-Review Trays and Approval-Invalidation Banners: Reviewer-Scope and Reopen-Safe Truth

This document is the contract for the M5 packet that implements the reusable
pending-review tray and approval-invalidation banner. It narrows the
`pending_review_tray` and `approval_invalidation_banner` components frozen in the
[review-request/checks/merge-queue component matrix](../../../schemas/ui/m5-review-request-check-queue-component-matrix.schema.json)
into an implemented tray and banner contract. The packet is the canonical M5
control source for this lane: the review workspace, review lists, companion queues,
handoff packets, CLI/headless output, diagnostics, Help/About, notifications
inboxes, and support exports ingest the checked-in packet rather than cloning
review pills or a single provider-specific status number.

The goal is to preserve **who still owes action** and **when prior approval or
readiness truth became invalid**.

- Record kind: `pending_review_tray_and_approval_invalidation_banner_truth`
- Schema: [`schemas/ui/m5-pending-review-tray.schema.json`](../../../schemas/ui/m5-pending-review-tray.schema.json)
- Canonical support export: [`artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth/support_export.json`](../../../artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth/support_export.json)
- Summary artifact: [`artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth.md`](../../../artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth.md)
- Protected fixtures: [`fixtures/ui/m5-pending-review-trays/`](../../../fixtures/ui/m5-pending-review-trays/)

## Pending-review tray

A `PendingReviewTray` summarizes what the current owner still owes on a review:

- **Reviewer scope** (`reviewer_scope`) — awaiting my review, awaiting other
  reviewers, awaiting author revision, changes requested, or nothing outstanding.
  The scope is never omitted, and a tray that claims nothing is outstanding must not
  still list an outstanding reviewer or unresolved threads
  (`reviewer_scope_misrepresented`).
- **Requested reviewers** (`requested_reviewers`) — each reviewer's identity,
  review state (requested, approved, changes requested, commented, dismissed), and
  whether they are a required approver.
- **Unresolved threads** (`unresolved_thread_count`).
- **Local draft comments** (`local_draft_comments`) — the reviewer's own in-flight,
  unpublished comments.
- **Publish-later packets** (`publish_later_packets`) — offline follow-up packets
  queued for later publication.
- **Exact next-action verb** (`next_action` + `next_action_label`).

### Local evidence stays visible (AC2)

Local draft comments and offline follow-up packets remain visible even when provider
freshness is degraded or unavailable. If a tray carries any local drafts or
publish-later packets but flags `local_evidence_visible = false`, the packet fails
with `local_drafts_or_follow_up_hidden`. A degraded provider additionally requires a
local-continue note (`tray_local_continue_note_missing`), and an unreachable provider
requires an explicit browser-handoff boundary
(`tray_browser_handoff_boundary_missing`). Freshness is reused verbatim from the
frozen matrix (`M5ReviewComponentStaleProviderState`).

## Approval-invalidation banner

An `ApprovalInvalidationBanner` preserves when prior approval or readiness truth
became invalid:

- **Cause** (`invalidation_cause`) — stale base, rebased stack, rewritten series,
  changed queue state, or policy drift.
- **Cause detail** (`cause_detail`) and **prior approval state**
  (`prior_approval_state_label`) — required when approvals were actually invalidated.
- **Reopen-safe follow-up** (`reopen_safe` + `reopen_note`) — a reopen note is
  required whenever a reopen-safe follow-up is offered.
- **Actions** (`actions`) — compare, re-review, reopen, and export are all required
  on an invalidating banner (`required_invalidation_actions_missing`).

### Approval invalidation is kept separate (AC1)

Approval invalidation is kept separate from generic warning and queue-block banners.
The banner declares its `banner_kind`. A banner whose `approvals_were_invalidated` is
true must present as `approval_invalidation`; collapsing it into a `generic_warning`
or `queue_block` pill — or, conversely, labeling a non-invalidating banner
`approval_invalidation` — fails with `approval_invalidation_not_separated`. This
catches the masquerade in both directions.

## Derivation, not assertion

Two free functions derive the disclosures a tray or banner must carry, so honesty is
computed rather than trusted:

- `resolve_pending_tray_disclosure(provider_freshness)` — a stale, unreachable,
  conflicting, or local-only provider forces keeping local evidence visible and a
  local-continue note; only an unreachable provider forces a browser-handoff
  boundary.
- `resolve_approval_banner_disclosure(approvals_were_invalidated, provider_freshness)`
  — an invalidating banner must carry its cause, prior approval state, and required
  actions; freshness drives the same local-continue and handoff boundaries.

## Guardrails

- Ordinary triage never forces raw-provider navigation: every banner keeps at least
  one in-product action (`forced_raw_provider_navigation`).
- The invalidation banners span every cause
  (`invalidation_cause_coverage_missing`).
- Raw provider responses, credentials, and live provider payloads never cross the
  support boundary (`raw_boundary_material_in_export`).
- Stale sync degrades one tray or banner without collapsing the whole review lane.

## Source contracts

The packet references, by id, the frozen component matrix, the review-workspace and
review-pack contracts (review, reviewer, and thread identity), the publish-later /
offline follow-up contract, the approval-invalidation / stale-base contract, and the
landing-candidate contract (readiness identity). It embeds none of their content.
