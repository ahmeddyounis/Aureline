# M5 draft-state-row, attachment-stale-banner, and send-review-control primitive contract

Task: **M05-888** — Ship draft-state rows, offline-local-only banners, attachment-stale warnings,
and split-send-or-review controls with no-hidden-sharing and no-ambiguous-send truth across the
claimed M5 composer surfaces.

This lane narrows the `draft_state_row`, `attachment_stale_banner`, and `send_review_control`
families from the frozen
[prompt-composer component matrix](./freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix.md)
(M05-884) into three reusable primitives: a draft-state row, an offline-local-only /
attachment-stale banner, a split-send-or-review control, and one shared parity matrix. A user can
tell — from the row, the banner, or the control alone — where a draft lives and how long it is
retained, whether a stale attachment or an offline route still preserves the current draft and
what refresh or local-safe path resolves it, and, for any route that widens authority, which
explain-only / review / mutating send path they are choosing instead of one ambiguous send.

## Primitives

- Draft-state row: `resolve_draft_state_row(&M5DraftStateRowResolutionInput) -> Result<M5ResolvedDraftStateRow, M5DraftStateRowResolutionError>`.
- Attachment-stale banner: `resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput) -> Result<M5ResolvedAttachmentStaleBanner, M5AttachmentStaleBannerResolutionError>`.
- Send-review control: `resolve_send_review_control(&M5SendReviewControlResolutionInput) -> Result<M5ResolvedSendReviewControl, M5SendReviewControlResolutionError>`.
- Parity matrix packet: `M5DraftSendPacket`, one row per claimed send-capable composer consumer,
  each carrying worked draft, banner, and send resolution cases.

### Draft retention posture and no-hidden-sharing

The retention posture is derived one-to-one from the draft locality
(`local_only_ephemeral` / `local_only_persisted` / `workspace_retained` / `account_retained` /
`shared_to_thread` / `purge_pending`), so a row never leaves the retention posture implicit. A
draft that leaves the device — workspace-synced, account-synced, shared to a thread, or retained
pending purge — must disclose its sharing / retention exception (`shared_or_retained` plus a
non-empty note), or the resolver rejects it with `shared_draft_without_disclosure`. A
purge-pending draft must carry its retention note. The row always offers a
`view_retention_detail` action and the `save_locally` / `clear_draft` / `delete_draft` /
`stop_sharing` follow-ups its state allows.

### Attachment-stale / offline-local-only banner (specific-first)

1. reason `source_deleted` → **stale_source_gone** (unrecoverable; requires a recovery note).
2. reason `permission_revoked` → **stale_access_revoked** (unrecoverable; requires a recovery note).
3. reason `revision_superseded` → **stale_superseded_review** (review the newer revision).
4. reason `source_edited` / `source_moved` / `index_reindexed` → **stale_refreshable**.
5. no reason and `offline_local_only` → **offline_local_only** (must offer a refresh or a
   local-safe alternative, or the resolver rejects it with
   `offline_without_refresh_or_alternative`).
6. otherwise → **fresh**.

The current draft is always preserved (`draft_preserved`), the banner always keeps a
`keep_draft_local` action so the draft is never dropped, and it offers a `refresh_attachment` /
`review_attachment` / `use_local_safe_alternative` / `detach_attachment` resolution path instead
of a silent retry loop.

### Send-review control (blocking-first, split high authority)

1. `policy_blocked` → **policy_blocked** (no path until resolved).
2. `taint_blocked` → **taint_blocked** (no path until resolved).
3. `over_budget` → **over_budget_blocked** (no path until resolved).
4. `widens_authority` and `is_mutating_route` → **split_send_review** (paths `explain_only`,
   `review_then_send`, `direct_send`).
5. `widens_authority` or a pending review → **review_before_send** (paths `explain_only`,
   `review_then_send`).
6. otherwise → **ready_to_send** (paths `explain_only`, `direct_send` when mutating, or a single
   `direct_send`).

A send that widens authority always offers more than one qualified path (`is_split`), so it never
collapses into one unqualified send (`no_ambiguous_send`); the resolver rejects a widened,
unblocked, single-path send with `ambiguous_widening_send`.

## Invariants

Each matrix row asserts four hard invariants (all `false`):

- `masks_draft_locality_or_retention`
- `assumes_hidden_sharing`
- `invents_private_send_grammar`
- `collapses_high_authority_send`

## Acceptance-criterion coverage

- **Draft locality and retention posture remain visible without hidden-sharing assumptions.**
  `draft_locality_disclosure_unproven` fires unless a worked draft proves a non-local draft that
  discloses its sharing, and `draft_hidden_sharing_found` fires if any worked draft leaves the
  device without disclosing it.
- **Offline-local-only and attachment-stale states preserve the draft and offer refresh or
  safe-local alternatives instead of silent retry loops.** `stale_preserves_draft_unproven` fires
  unless a worked banner proves a stale-or-offline state that preserves the draft and offers a
  resolution path, and `stale_condition_coverage_unproven` fires unless both the offline-local-only
  and the attachment-stale conditions are proven.
- **High-authority send paths are reviewable and no longer collapse into a single unqualified send
  action.** `send_split_no_ambiguous_unproven` fires unless a worked control proves a
  widened-authority send that stays split, unambiguous, and review-gated before send.

## Boundary

Raw prompts, draft bodies, attachment bodies, raw paths, raw URLs, credentials, and private
endpoints never cross this boundary; every draft id, banner id, control id, draft label,
attachment label, and note is carried only as an opaque, export-safe representation. The
`raw_material_in_export` violation and the resolver forbidden-material errors reject obviously
sensitive strings.

## Source contracts

- Boundary schema: `schemas/ai/m5-draft-state-row-attachment-stale-banner-and-send-review-control.schema.json`.
- Frozen component matrix: `schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json`.
- Prompt-composer draft: `schemas/ai/prompt_composer_draft.schema.json`.
- Prompt-context attachment: `schemas/ai/prompt_context_attachment.schema.json`.

## Artifacts

- Support export: `artifacts/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/support_export.json`.
- Matrix CSV: same directory, `matrix.csv`.
- Markdown report: `artifacts/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces.md`.
- Narrowed fixtures: `fixtures/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/`.

All are minted from the seed builders by the headless emitter
`aureline_ai_draft_state_row_attachment_stale_banner_send_review_control_primitive`; the inline
tests assert the checked artifacts never drift from the seed.
