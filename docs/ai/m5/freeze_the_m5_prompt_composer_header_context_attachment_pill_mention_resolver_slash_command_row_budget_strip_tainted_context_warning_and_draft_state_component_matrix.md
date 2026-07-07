# M5 prompt-composer component matrix contract

**Row:** M05-884 — Freeze the M5 prompt-composer-header, context-attachment-pill,
mention-resolver, slash-command-row, budget-strip, tainted-context-warning, and
draft-state component matrix (batch B104).

This contract freezes the reusable **pre-send prompt-composition component matrix**
so composer mode, scope, route, attachment, trust/taint, omitted-context, draft, and
send-review language stop drifting across M5 consumers. It is the composer analog of
the AI-execution/replay component freeze
(`freeze_the_m5_ai_action_state_banner_...`), the docs-browser
(`freeze_the_m5_docs_search_bar_...`), and the release-center
(`freeze_the_m5_release_candidate_card_...`) component freezes.

- **Crate / module:** `aureline-ai`,
  `freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`
- **Schema:** `schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json`
- **Support export (canonical truth):**
  `artifacts/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/support_export.json`
- **Matrix CSV / Markdown report:** same directory (`matrix.csv`) and sibling `.md`
- **Narrowed fixtures:**
  `fixtures/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/`
- **Headless emitter:** `cargo run -p aureline-ai --bin aureline_ai_prompt_composer_component_matrix -- <support-export|report|csv|fixture-*|validate>`

## Governed component families (9)

| Family | Owns (family-specific vocabulary) |
| --- | --- |
| `prompt_composer_header` | composer modes, composer scopes, route classes |
| `context_attachment_pill` | attachment kinds, attachment trust states |
| `mention_resolver` | mention-resolution states |
| `slash_command_row` | slash-command states |
| `budget_size_strip` | budget postures, omitted-context reasons |
| `tainted_context_warning` | taint sources, taint severities |
| `draft_state_row` | draft localities |
| `attachment_stale_banner` | staleness reasons |
| `send_review_control` | send postures, review requirements |

Every family also declares composer surface families, deployment lines, mandatory
plus truth labels, non-visual accessibility routes, consumer surfaces, and downgrade
triggers.

## Frozen controlled vocabularies

- **Composer mode:** `chat_ask`, `inline_edit`, `guided_patch`, `background_agent`,
  `review_first`, `headless_automation`
- **Composer scope:** `selection`, `active_file`, `open_files`, `workspace`,
  `repository`, `managed_org`
- **Route class:** `local_model`, `byok_direct`, `managed_route`, `self_hosted_route`,
  `mirrored_route`, `policy_pinned_route`
- **Attachment kind:** `file`, `symbol`, `selection_range`, `evidence_packet`,
  `external_paste`, `url_reference`
- **Attachment trust state:** `trusted_fresh`, `trusted_stale`, `unverified_source`,
  `tainted_external`, `redacted_scope`, `out_of_scope`
- **Mention resolution:** `resolved_unique`, `resolved_pinned`, `ambiguous_candidates`,
  `unresolved_missing`, `out_of_scope_denied`, `deferred_pending`
- **Slash-command state:** `available`, `disabled_unmet_precondition`,
  `requires_approval`, `deprecated_aliased`, `policy_hidden`, `unknown_command`
- **Budget posture:** `within_budget`, `near_limit`, `over_budget`,
  `truncation_pending`, `hard_blocked`, `unmetered_local`
- **Omitted-context reason:** `none_omitted`, `size_truncated`, `budget_capped`,
  `policy_excluded`, `dedup_collapsed`, `stale_dropped`
- **Taint source:** `pasted_external_text`, `tool_output`, `fetched_url_content`,
  `untrusted_file`, `third_party_connector`, `prior_model_output`
- **Taint severity:** `none`, `informational`, `elevated`, `quarantine_required`,
  `injection_suspected`
- **Draft locality:** `local_only`, `workspace_synced`, `account_synced`,
  `shared_thread`, `ephemeral_unsaved`, `retention_pending_purge`
- **Staleness reason:** `source_edited`, `source_moved`, `source_deleted`,
  `revision_superseded`, `permission_revoked`, `index_reindexed`
- **Send posture:** `ready_to_send`, `split_send_review`, `review_before_send`,
  `policy_blocked`, `over_budget_blocked`, `taint_blocked`
- **Review requirement:** `none`, `attachment_review`, `taint_ack`, `budget_ack`,
  `route_change_ack`

Shared/topology vocabularies: composer surface family (7), deployment line (5),
consumer surface (9), accessibility route (6), required label (6, with mandatory
`identity` / `state` / `keyboard_route` plus `composer_mode` / `route_provider_model`
/ `trust_or_taint`), qualification class (6), downgrade trigger (12).

## The six acceptance-criteria states are distinct, not collapsed

The matrix keeps each of the required pre-send states in its own token rather than
collapsing them into a generic send:

- **local-only** → `draft_locality = local_only`
- **managed-route** → `route_class = managed_route`
- **policy-blocked** → `send_posture = policy_blocked` (and `slash_command_state =
  policy_hidden`)
- **over-budget** → `budget_posture = over_budget` / `send_posture =
  over_budget_blocked`
- **unresolved-mention** → `mention_resolution = unresolved_missing`
- **tainted-context** → `attachment_trust_state = tainted_external` +
  `taint_severity ∈ {quarantine_required, injection_suspected}` / `send_posture =
  taint_blocked`

## Hard component invariants

Every component row must keep all four `false`:

1. `masks_mode_or_route` — never hide which composer mode or route/provider a
   component composes under.
2. `hides_taint_or_trust_state` — never present a tainted, unverified, or stale
   context as trusted.
3. `invents_private_composer_grammar` — never invent a second composer grammar
   outside this matrix.
4. `bypasses_send_review_gate` — never let a request that needs review or is blocked
   send as a plain ready action.

## Non-visual / CLI / export expectations

Every component declares a non-visual accessibility route set (keyboard focus,
screen-reader announcement, non-hover reachability, pointer-optional, high-contrast
safety, support-exportability). Composer primitives must never become panel-only or
chat-only affordances: the same mode/scope/route/attachment/trust/taint/draft/send
truth is reachable via keyboard, screen reader, CLI inspect, and the support export.

## Auto-narrowing

Qualification narrows below Stable when a downgrade trigger fires (e.g. composer
mode unstated, route/provider masked, attachment identity unstated, attachment
freshness masked, taint state hidden, omitted context undisclosed, mention left
unresolved, budget overrun hidden, draft locality masked, send-review gate bypassed,
attachment staleness undisclosed, proof stale). The two checked-in narrowed fixtures
demonstrate the pattern while keeping every family visible:
`tainted_context_warning` → Beta, `send_review_control` → Preview.

## Bound source contracts

`schemas/ai/prompt_composer_draft.schema.json`,
`schemas/ai/prompt_context_attachment.schema.json`,
`schemas/ai/tainted_context.schema.json`, and
`schemas/ai/context_assembly.schema.json` — this matrix hardens shared pre-send
composition components layered on top of those already-claimed systems; it does not
re-architect model routing, evidence storage, or branch-agent lifecycle.

## Consumer rule

Every claimed M5 inline / panel / patch-review / branch-agent / help / companion
composer consumer points at this one canonical component contract instead of
rewording composer mode, attachment, trust, taint, draft, or send truth locally.
Future AI composer implementation rows have an agreed field/state baseline and no
open ambiguity about what will be sent before a request leaves the shell.
