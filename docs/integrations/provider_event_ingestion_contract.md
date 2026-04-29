# Provider Event Ingestion, Webhook Replay, and Import Sessions

This contract freezes how external provider events enter Aureline,
how webhook redelivery and replay are reconciled, and how imported
provider state stays traceable. It turns webhooks, polling refreshes,
mirror syncs, browser-return callbacks, manual backfills, dry-run
imports, and operator replays into one governed event-ingestion model.

The machine-readable schemas live at:

- [`/schemas/integrations/provider_event.schema.json`](../../schemas/integrations/provider_event.schema.json)
  - `provider_event_record`
- [`/schemas/integrations/import_session.schema.json`](../../schemas/integrations/import_session.schema.json)
  - `import_session_record`
- [`/schemas/integrations/webhook_replay_record.schema.json`](../../schemas/integrations/webhook_replay_record.schema.json)
  - `webhook_replay_record`

Worked fixtures live at:

- [`/fixtures/integrations/provider_event_cases/`](../../fixtures/integrations/provider_event_cases/)

This contract composes with and does not replace the provider-mode,
callback-envelope, browser-handoff, approval-ticket, work-item,
hosted-review, AI-evidence, and support-bundle contracts. The
provider-mode callback envelope remains the broad callback wrapper;
the records here freeze the provider-event, replay-ledger, and
import-session boundary that downstream surfaces read.

Raw URLs, raw request headers, raw webhook bodies, raw provider
payloads, raw OAuth tokens, raw delegated tokens, and provider-private
profile bodies do not cross this boundary. Records carry opaque refs,
digests, closed vocabulary, and short reviewable summaries only.

This document does not ship live webhooks, polling jobs, mirror
adapters, queue drainers, or provider-specific import code. It freezes
the contract those implementations will read and write.

## Goals

The ingestion boundary must let any desktop, CLI, support, review,
or AI-evidence reader answer these questions without guessing from
timestamps alone:

1. Which provider, tenant, actor, subscription, scope, and delivery
   produced this row?
2. Was the event verified, denied, delayed, replayed, duplicated,
   imported from a mirror, dry-run inspected, or only partially
   applied?
3. Which import session materialized the current provider-derived
   state?
4. Which replay ledger entry suppressed, held, replayed, rewound, or
   denied the delivery?
5. Which raw payload reference was normalized into which local record
   refs?
6. Is the rendered state local live truth, live provider overlay,
   cached provider overlay, imported snapshot, replayed import, dry
   run, or denied/no-mutation evidence?

## Scope

Frozen at this revision:

- the `provider_event_record` envelope required before any provider
  event can influence local state;
- the `import_session_record` required for first sync, incremental
  catch-up, manual backfill, webhook replay, rewind reconstruction,
  dry-run inspection, mirror sync, and failed-partial import attempts;
- the `webhook_replay_record` ledger required for dedupe, replay,
  rewind, missing-page recovery, cursor reset, and manual operator
  replay;
- replay and reconciliation rules for duplicate suppression,
  out-of-order delivery, missing pages, signature failure, stale
  permissions, cursor reset, and operator replay;
- visibility rules for desktop, CLI, review, support, and AI evidence
  so imported and replayed rows never masquerade as local live truth;
- degraded-import vocabulary, callback-deny/audit references, and
  blast-radius notes.

Out of scope:

- provider-specific webhook signature libraries;
- live network listeners or import workers;
- provider mutation APIs;
- raw payload retention policy beyond opaque refs and redaction class.

## Event Envelope

Every incoming provider delivery is first normalized into exactly one
`provider_event_record`. The record is the durable attribution object
for both accepted and denied deliveries.

Required identity fields:

| Field | Meaning |
|---|---|
| `provider_identity` | Provider class, canonical host, tenant/org scope, environment, and provider-instance ref. |
| `acting_identity` | Provider actor class, actor ref, effective-scope ref, and permission snapshot ref. |
| `event_source_class` | Webhook, callback return, polling refresh, mirror sync, import file, manual backfill, operator replay, or provider stream. |
| `event_kind` | Closed event-kind vocabulary such as object update, comment change, check status, permission change, callback denial, cursor reset, or unknown event. |
| `original_delivery_id` | Provider delivery id exactly as represented by a redaction-safe opaque ref. |
| `verification_state` | Verified, unsigned-expected, signature denied, host mismatch, route denied, stale-permission denied, or pending review. |
| `cursor_or_sequence_hint` | Cursor token, page token, sequence number, timestamp-only event, cursor reset, or sequence gap. |
| `scope_refs` | Provider subscription, local object, provider object, import scope, permission scope, review anchor, support packet, or AI evidence scope refs. |
| `dedupe_key` | Idempotency key with scope class, first/last seen times, and occurrence count. |
| `raw_to_normalized_linkage` | Opaque raw payload/header refs plus normalized local record refs and parse status. |
| `import_session_ref` | Import session that materialized or attempted to materialize state from this event. |
| `replay_record_ref` | Replay ledger item that decided whether the delivery applied, deduped, held, replayed, or denied. |

The envelope has no raw provider payload. A raw payload ref is a
redaction-aware pointer into the narrowest permitted storage boundary,
or a digest-only placeholder when policy denies retention.

### Verification States

Only `verified` and `unsigned_expected` may advance to reconciliation
without a deny path. `unsigned_expected` is allowed only for provider
sources that contractually do not sign the event and have an alternate
route proof.

Denied verification states must:

- set `processing_outcome_class` to a denied or failed-partial class;
- cite at least one `callback_deny_audit_event_ref`;
- retain `original_delivery_id`, `provider_identity`,
  `acting_identity`, `scope_refs`, and `dedupe_key`;
- keep `visibility_truth_class` at `denied_no_local_mutation`.

## Import Sessions

Every row imported from a provider must cite an `import_session_record`.
The import session is not a generic "synced" marker; it is the durable
lineage object explaining how provider state was materialized.

Session intents:

| Intent | Use |
|---|---|
| `first_sync` | Initial materialization of linked provider objects. |
| `incremental_catch_up` | Normal catch-up after a known cursor or freshness floor. |
| `manual_backfill` | User/admin requested historical import for a bounded scope. |
| `webhook_replay` | Import caused by provider redelivery or replay ledger action. |
| `rewind_reconstruction` | Rebuild local provider-derived state from earlier cursor or snapshot. |
| `dry_run_inspection` | Inspect what would import without writing provider-derived rows. |
| `failed_partial_import` | Attempt failed after some bounded state was imported or after policy blocked materialization. |
| `mirror_sync` | Customer-controlled or offline mirror ingest using the same vocabulary. |

Session states:

| State | Meaning |
|---|---|
| `pending` | Admitted but not yet fetching or replaying. |
| `running` | Active ingest is in progress. |
| `completed_full` | All in-scope objects materialized for the declared scope. |
| `completed_partial_degraded` | Some objects materialized; omissions and degraded classes explain limits. |
| `failed_partial` | Session failed after partial materialization or after capturing denial evidence. |
| `denied_policy_blocked` | No imported state materialized because policy or proof failed. |
| `dry_run_completed_no_write` | Inspection completed without writing imported provider rows. |
| `replay_hold_pending_operator` | Held until a human or admin chooses replay/backfill/skip. |
| `rewind_completed` | Earlier provider state was reconstructed and marked replayed/imported. |
| `superseded` | Another session replaces this one. |

Each import session names object scope, provider identity, actor scope,
source event refs, snapshot time, freshness state, partiality class,
rate-limit posture, cursor state, omitted scopes, imported object refs,
and reconciliation summary.

### Degraded Import Vocabulary

Degraded imports use explicit classes rather than a generic failed-sync
badge:

- `missing_pages`
- `partial_rate_limit`
- `provider_unreachable`
- `stale_permissions`
- `cursor_reset`
- `signature_denied`
- `policy_blocked`
- `raw_payload_quarantined`
- `replay_hold`
- `dry_run_no_write`
- `unknown_fields_preserved_raw_ref`

Any degraded import must render as partial, stale, denied, dry-run, or
replayed according to its `visibility_truth_class`; it may not render
as local live truth.

## Replay Ledger

Every delivery that can be redelivered, replayed, rewound, or manually
inspected carries a `webhook_replay_record`. The replay record is the
idempotency and blast-radius ledger, not an implementation log.

Replay reasons:

- `duplicate_delivery`
- `out_of_order_delivery`
- `missing_page_backfill`
- `manual_operator_replay`
- `cursor_reset_rewind`
- `signature_failure_denied`
- `stale_permission_recheck`
- `policy_reconciliation`
- `provider_redelivery`

Replay dispositions:

| Disposition | Rule |
|---|---|
| `applied_once` | First verified delivery applies exactly once. |
| `deduped_noop` | Duplicate delivery is suppressed; freshness may refresh but user-visible mutation cannot repeat. |
| `held_pending_sequence` | Out-of-order delivery waits for gap/backfill or operator review. |
| `replayed_read_only` | Replay can inspect/import but cannot mutate provider or local authored truth. |
| `replayed_with_write_after_revalidation` | Replay wrote imported provider state only after fresh proof, scope, cursor, and policy revalidation. |
| `denied_no_mutation` | Verification, host, scope, or policy failed; denial is auditable. |
| `failed_partial_import` | Replay/import stopped after bounded partial state or denial evidence. |
| `rewind_no_live_claim` | Rewind reconstructed historical state and labels it replayed/imported, not live. |

Each replay record carries an idempotency policy, duplicate suppression
state, replay window, replay count, provider event refs, import session
ref, operator review ref when applicable, audit refs, and a blast-radius
block.

### Blast Radius

The `blast_radius` block states the maximum user-visible effect a replay
may have:

- `no_user_visible_mutation`
- `freshness_refresh_only`
- `imported_snapshot_replace`
- `queued_local_reconciliation`
- `provider_mutation_requires_new_authority`
- `support_export_only`

Manual operator replay must include an operator review ref and a
blast-radius note before it can apply. A replay record cannot borrow
authority from the original event if target identity, policy epoch,
scope, actor, or cursor has drifted.

## Reconciliation Rules

Duplicate delivery:

- Match by `dedupe_key.dedupe_scope_class` plus `scope_ref`.
- Increment occurrence/replay counts.
- Do not duplicate comments, state transitions, check rows, or review
  artifacts.
- Refresh freshness only if the envelope is still verified and in scope.

Out-of-order delivery:

- Detect sequence gaps or cursor/page regressions.
- Hold the event or partially apply only non-conflicting read-only
  freshness updates.
- Emit missing-page or backfill session refs.
- Render affected rows as partial/backfill pending.

Missing pages:

- Mark cursor state as `missing_page_detected` or
  `page_gap_detected`.
- Start an incremental catch-up or manual backfill session.
- Keep affected overlays degraded until the missing range is resolved
  or explicitly waived.

Signature failure or host mismatch:

- Deny before materializing imported state.
- Emit callback-deny/audit refs.
- Preserve original delivery id, provider identity, route class, and
  dedupe key.
- Do not retry silently.

Stale permissions:

- Deny or delay mutation-shaped reconciliation.
- Start a stale-permission import session if read-only inspection is
  still permitted.
- Require fresh grant resolution before any queued local edit publishes.

Cursor reset:

- Treat reset as an explicit event kind or cursor-state class.
- Start rewind reconstruction or bounded backfill.
- Label results as replayed/imported until a live provider refresh
  proves current state.

Manual operator replay:

- Requires operator review ref, reason, scope, blast-radius note, and
  new policy/scope validation.
- Can inspect, backfill, or replace imported provider overlays.
- Cannot silently spend the original provider mutation authority.

## Visibility Rules

The boundary has one job: no imported, replayed, cached, dry-run, or
denied provider state may masquerade as local live truth.

Every rendered row derived from provider ingestion must retain:

- provider event ref;
- import session ref;
- replay record ref when replay/redelivery affected disposition;
- visibility truth class;
- freshness class;
- actor scope and permission snapshot ref;
- redaction class and export visibility.

Desktop surfaces must show imported, cached, replayed, partial, denied,
or dry-run state in the row or panel that uses the data. CLI and
headless output must expose the same classes as structured fields.
Support packets and AI evidence packets must cite event, session, and
replay refs rather than collapsing them into a generic sync timestamp.

Allowed visibility truth classes:

| Class | Meaning |
|---|---|
| `live_provider_overlay` | Fresh provider-derived overlay with current proof. Still not local authored truth. |
| `cached_provider_overlay` | Provider overlay is cached or stale but still useful for read-only context. |
| `imported_snapshot` | Imported through a session and traceable to provider event refs. |
| `replayed_import` | Materialized through replay, rewind, or backfill; never shown as live. |
| `dry_run_only` | Inspection result; no imported state was written. |
| `denied_no_local_mutation` | Denied delivery preserved as audit/support evidence only. |

Local edits, local review packs, and local command outputs use their own
local-truth contracts. A provider event may point at them as scope refs
or reconciliation targets, but it cannot relabel them.

## Audit and Support Minimum

Accepted, denied, delayed, duplicate, and replayed events must remain
distinguishable in exported evidence. At minimum:

- accepted events cite delivery id, provider ref, object refs, import
  session, replay record, normalized refs, and freshness class;
- duplicate events cite replay count and duplicate suppression state;
- denied events cite denial reason, route/proof class, provider host,
  actor class, policy epoch, and callback-deny/audit refs;
- delayed or held events cite sequence/cursor notes, retryability,
  missing page scope, and operator or backfill refs;
- replayed and rewound events cite blast radius, operator review when
  present, and visibility truth class.

Support exports may redact human labels and provider-private data, but
must keep provider class, canonical host, tenant/org scope, original
delivery id ref, event/session/replay refs, disposition, and degraded
state.

## Worked Fixtures

The fixture directory includes scenario bundles where each `records`
entry is valid against one of the three schemas:

- valid verified delivery applied once;
- duplicate delivery suppressed by the replay ledger;
- out-of-order delivery held for missing-page backfill;
- invalid signature denied with callback-deny/audit refs;
- stale permissions producing a failed-partial import;
- cursor reset plus manual operator replay and dry-run inspection.
