# Privacy-history and lifecycle vocabulary contract

This contract freezes the vocabulary and row shapes used when Aureline
explains consent changes, support-bundle exports, privacy-affecting
policy overrides, export requests, deletion requests, and offboarding
outcomes. It prevents privacy controls from becoming a second hidden
policy language by forcing every surface to reuse the same lifecycle,
delete-state, signal-class, and AI memory-class terms.

Companion artifacts:

- [`/schemas/governance/privacy_history_event.schema.json`](../../schemas/governance/privacy_history_event.schema.json)
  defines one privacy-history row.
- [`/schemas/governance/export_delete_request_summary.schema.json`](../../schemas/governance/export_delete_request_summary.schema.json)
  defines one export, delete, or offboarding request summary.
- [`/fixtures/governance/privacy_history_cases/`](../../fixtures/governance/privacy_history_cases/)
  carries worked cases for consent toggles, policy-locked upload,
  support export, held deletion, and offboarding export exclusions.
- [`./record_state_and_policy_simulation_models.md`](./record_state_and_policy_simulation_models.md)
  remains the per-record state machine for local-only, managed-copy,
  held, delete-requested, delete-complete, and export-available state.
- [`./storage_and_retention_vocabulary.md`](./storage_and_retention_vocabulary.md)
  remains the storage, retention, and raw-secret-exclusion vocabulary.
- [`./telemetry_and_support_schema_registry.md`](./telemetry_and_support_schema_registry.md)
  remains the registry for telemetry, crash, support-export, usage,
  offboarding, and CLI/headless schema families.
- [`../ai/memory_and_reconciliation_contract.md`](../ai/memory_and_reconciliation_contract.md)
  remains the AI memory-class contract.
- [`../support/support_bundle_preview_contract.md`](../support/support_bundle_preview_contract.md)
  remains the support-bundle preview and redaction-review contract.

If this document and a schema disagree, the schema wins and this
document must be updated in the same change.

## Why this contract exists

The product already has class-level retention, record state, telemetry
schema, AI memory, support-bundle preview, and portability contracts.
Without a privacy-history layer, the live privacy settings screen, the
support-bundle preview, delete/export flows, and offboarding copy can
still drift into incompatible language:

- settings may say a signal is local, while a support preview says it
  is retained;
- a delete flow may say complete while a legal hold or policy retention
  still keeps a managed copy;
- an AI memory export may collapse conversations, derived caches, and
  retained evidence into one opaque "AI history" label;
- a support bundle may be uploaded under explicit consent, but later
  review cannot reconstruct which consent and policy state applied.

The contract below gives every surface one inspectable row shape for
privacy-history events and one inspectable summary shape for export,
delete, and offboarding requests. Reviewers must be able to understand
the state without reading raw logs, support payloads, or policy files.

## Canonical lifecycle vocabulary

Every privacy-history row, support-bundle preview row, export/delete
summary, offboarding checklist, and delete-honesty surface reuses the
following exact lifecycle terms. Schemas use lower-snake ids; product
surfaces render the required labels.

| Term id | Required label | Meaning |
|---|---|---|
| `local_only` | `Local only` | The bytes or state are reachable only from the current device or local workspace authority. No Aureline-managed hosted copy exists for the platform to retain, mine, export, or delete. |
| `uploaded` | `Uploaded` | A copy has left the device through an explicit user/admin export, upload, or managed-policy path named by an endpoint or export manifest. |
| `redacted` | `Redacted` | A representation exists, but sensitive fields or payload bodies were removed, tokenized, hashed, or replaced with reviewable omission markers. |
| `held` | `Held` | Deletion, expiry, compaction, or destruction is blocked by legal hold, support investigation, export-pending hold, policy freeze, or retention minimum. |
| `destroyed` | `Destroyed` | The payload bytes for the named scope were removed. A tombstone, receipt, or audit summary may remain, but it is not a hidden payload copy. |
| `pending` | `Pending` | A user/admin request or policy action has been accepted but has not reached a terminal export, upload, delete, or destruction state. |

The terms are orthogonal. A request can include a `redacted` export,
an `uploaded` support copy, and a `held` managed archive at the same
time. Surfaces must show the row-level terms that apply instead of
collapsing them into one success string.

## Stable delete and export states

Delete and offboarding surfaces reuse these stable state labels across
privacy settings, collaboration, support, and offboarding:

| State id | Required label | Meaning |
|---|---|---|
| `delete_requested` | `Delete requested` | A delete request was submitted, accepted, or queued. No completion is implied. |
| `policy_retention` | `Policy retention` | A stricter retention policy keeps at least one managed or audit subset. This is not successful deletion. |
| `legal_hold` | `Legal hold` | A hold blocks destructive lifecycle steps for at least one matching managed record. This is not successful deletion. |
| `delete_completed` | `Delete completed` | The requested destructive action completed for the named scope. If receipts or tombstones remain, the summary names them as metadata-only evidence. |
| `exported_copy_remains_local` | `Exported copy remains local` | The product produced or downloaded an export copy that remains under user/device control and is outside the managed delete job. |

`policy_retention` and `legal_hold` can never be rendered as
`delete_completed`. A terminal delete summary with retained subsets
must say `Policy retention` or `Legal hold` and cite the blocker.

## Privacy-history event rows

Every privacy-affecting change emits one
`privacy_history_event_record`. These rows are user/admin inspectable
and safe for support/export linkage; raw payload bodies, raw prompts,
raw file paths, raw terminal contents, raw credentials, and raw policy
bundle bytes do not cross this boundary.

Seeded event types:

| Event type | Required use |
|---|---|
| `telemetry_consent_changed` | User, admin, or policy changed product telemetry opt-in/out. |
| `crash_upload_consent_changed` | Crash upload posture changed, including local-only, upload-capable, policy-denied, or redacted upload states. |
| `ai_usage_sharing_changed` | AI usage sharing, model-improvement sharing, or retained AI usage reporting changed. |
| `support_bundle_exported` | A support bundle or preview was exported, uploaded, redacted, or retained local-only. |
| `managed_analytics_toggle_changed` | A managed analytics toggle changed under org policy or admin action. |
| `policy_enforced_override_applied` | Effective privacy posture differs from the requested user/admin posture because a policy override applied. |

Required fields:

- `event_id`, `event_type`, `occurred_at`, and `source` identify what
  happened, when, and which surface or policy path produced it.
- `actor` names the actor class and safe actor reference. Actor rows
  never carry raw emails, names, tokens, or directory identifiers.
- `prior_control` and `effective_control` show the before and after
  control states. If a policy override applies, the effective state
  carries a policy ref and lock reason.
- `signal_class_resolutions` resolves the event through the shared
  signal-class matrix: install/update/version counts,
  crash/panic reports, performance metrics, feature usage counters,
  support-bundle transfer events, or managed analytics aggregates.
- `ai_memory_class_resolutions` resolves the event through the AI
  memory-class matrix where AI state is affected: turn state,
  conversation history, derived cache, embedding row, reusable repo
  fact, retained evidence copy, or explicit saved memory. Events with
  no AI effect carry a `not_applicable_no_ai_memory` projection row.
- `retention` names retention class, hold refs, delete state, export
  state, destruction receipt refs, and local artifact refs.
- `audit_linkage` connects the row to governed-record ids, support
  bundle ids, export request ids, deletion job ids, policy decision
  refs, schema-registry entries, and evidence packet refs.
- `vocabulary_terms` lists the lifecycle terms a renderer may show for
  this event. Renderers must not invent synonyms.

## Export/delete request summaries

Every export, delete, export-before-delete, support purge, or
offboarding flow emits one `export_delete_request_summary_record`.
The summary is inspectable without raw logs and is safe to attach to
support packets or offboarding exports.

Required fields:

- `request_id`, `request_type`, requester, source flow, request time,
  and update time.
- `current_status`, `stable_delete_state`, and `stable_state_label`.
  The state id and label must agree.
- `request_scope`, including subject scope, scope refs, optional time
  range, requested record classes, requested signal classes, and
  requested AI memory classes.
- `record_class_results`, one row per included, excluded, redacted,
  held, policy-retained, destroyed, pending, uploaded, local-only, or
  outside-platform-scope class.
- `blockers`, including legal hold, policy retention, export-pending
  hold, entitlement expiry, service unavailable, manual local capture,
  redaction policy, user-declined, not-found, and outside-platform
  scope blockers.
- `remaining_local_artifacts`, so an exported `.zip`, local support
  bundle, local history file, or manually captured artifact cannot be
  mistaken for a managed copy the service deleted.
- `post_exit_notes`, covering retained subsets, destruction receipts,
  local copy notes, manual-capture notes, and scheduled expiry notes.
- `audit_linkage`, connecting the request back to privacy-history
  events, governed-record rows, deletion jobs, support bundles, export
  manifests, policy refs, and destruction receipts.
- `honesty_assertions`, including a mandatory assertion that held or
  policy-retained data is not reported as destroyed.

## Vocabulary reuse rules

The lifecycle terms and stable delete states above are the only terms
these surfaces may render for the corresponding state:

- Live privacy settings use `Local only`, `Uploaded`, `Redacted`,
  `Held`, `Destroyed`, and `Pending` for the privacy-history detail
  row and for current effective state.
- Support-bundle preview uses the same terms next to item-level
  storage, redaction, upload, and policy-lock state.
- Offboarding/export flows use the same terms in class summaries and
  in generated export manifests.
- Delete-honesty surfaces use `Delete requested`, `Policy retention`,
  `Legal hold`, `Delete completed`, and `Exported copy remains local`
  instead of feature-specific alternatives.
- CLI/headless summaries use the lower-snake ids and may include the
  required labels, but must not substitute localized prose for the
  canonical ids in machine-readable output.

Non-conforming substitutions include "removed" for held data,
"private" for local-only state, "sent" for uploaded state, "complete"
for policy-retained data, and "not included" without an exclusion or
redaction reason.

## Auditability and support/export linkage

A support reviewer must be able to reconstruct which privacy and
consent state was in effect when a packet was produced. Therefore:

1. A support bundle, export manifest, or offboarding packet that
   contains or references privacy-governed data must cite the relevant
   `privacy_history_event_record` ids.
2. Each event cites the schema-registry entry and record-class ids that
   governed the produced packet.
3. Each export/delete summary cites the event ids, governed-record ids,
   deletion job ids, export manifest ids, support bundle ids, policy
   refs, and destruction receipt refs that explain its outcome.
4. A row may reference raw payloads by opaque ref, digest, or manifest
   member id only. It does not embed raw bodies.
5. If a policy bundle, legal hold, retention minimum, or support
   investigation blocks deletion, the summary cites the blocker and
   renders `Policy retention` or `Legal hold`.

## Signal and AI memory matrix resolution

Consent and lifecycle states must resolve through the same signal and
AI memory matrices used elsewhere in the product.

Signal classes:

- `install_update_active_version_counts`
- `crash_panic_reports`
- `performance_metrics`
- `feature_usage_counters`
- `support_bundle_transfer_events`
- `managed_analytics_aggregates`

AI memory classes:

- `turn_state_ephemeral`
- `conversation_history_user_visible`
- `derived_cache_regeneratable`
- `embedding_row_regeneratable`
- `reusable_repo_fact_workspace_scope`
- `retained_evidence_packet_copy`
- `explicit_saved_memory_user_owned`
- `not_applicable_no_ai_memory`

Every affected class row carries lifecycle terms, data class,
collection posture, record-class refs, and schema-registry refs. A
timeline that says "AI sharing disabled" but fails to say which AI
memory classes remain local-only, destroyed, held, retained, or
pending is incomplete.

## Honesty rules

- `Local only` means no Aureline-managed hosted copy exists. If a
  managed copy exists, even as a reference-only archive, the row must
  use `Uploaded`, `Held`, `Pending`, `Redacted`, or another truthful
  lifecycle term instead.
- `Redacted` means a representation exists and omission/redaction
  reasons are visible. It does not mean the original source no longer
  exists.
- `Held` always cites at least one hold or policy ref. A held row can
  have a pending delete request, but it cannot be shown as destroyed.
- `Destroyed` applies only to the named payload scope. Receipts,
  tombstones, audit stubs, and local export copies remain separately
  listed where they exist.
- `Exported copy remains local` is required when a user downloaded or
  generated an export copy and a managed delete job cannot touch that
  local artifact.
- Policy-enforced overrides must show both the requested posture and
  the effective posture. A lock is not a consent change by the user.

## Fixture coverage

The privacy-history cases demonstrate the minimum contract checks:

- a telemetry consent toggle that records uploaded signal classes and
  no applicable AI memory class;
- a policy-locked crash-upload attempt where crash data stays local
  and redacted, with a policy override event;
- a support-bundle export event with redacted and uploaded lifecycle
  terms plus support/export linkage;
- a delete request blocked by legal hold that cannot masquerade as
  successful deletion;
- an offboarding export with included and excluded classes, local
  export copies, AI memory class resolution, and explicit remaining
  local artifacts.
