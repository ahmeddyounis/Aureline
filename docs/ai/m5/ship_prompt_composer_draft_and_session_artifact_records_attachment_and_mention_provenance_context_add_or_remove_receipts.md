# Prompt-composer draft and session-artifact records, attachment-and-mention provenance, context add-or-remove receipts, and durable replay-safe evidence packets across M5 AI surfaces

This contract turns a prompt-composer draft into a durable-but-scoped session
artifact and exposes it as one export-safe truth packet whose unit of truth is a
session-artifact row. Shell, docs, support export, and release tooling consume
the packet directly instead of re-describing draft, mention, context-change, or
replay state by hand.

- Packet type: `aureline_ai::ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts::PromptSessionArtifactPacket`
- Schema: [`schemas/ai/ship-prompt-composer-draft-and-session-artifact-records-attachment-and-mention-provenance-context-add-or-remove-receipts.schema.json`](../../../schemas/ai/ship-prompt-composer-draft-and-session-artifact-records-attachment-and-mention-provenance-context-add-or-remove-receipts.schema.json)
- Support export: [`artifacts/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/support_export.json`](../../../artifacts/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/support_export.json)
- Fixtures: [`fixtures/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/`](../../../fixtures/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/)

## The session-artifact row

Each `SessionArtifactRow` binds, for one prompt-composer draft or session:

| Field | Meaning |
| --- | --- |
| `session_ref`, `draft_ref`, `context_snapshot_ref` | Composer session, draft, and context-snapshot identity the artifact captures. |
| `surface` | Desktop composer, review workspace, CLI, browser companion, support export, or diagnostics. |
| `artifact_class` | Draft-in-progress, active session artifact, archived session artifact, or deleted tombstone. |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `scope`, `locality`, `retention`, `delete_export_posture` | Turn/thread/workspace/org scope, the locality the data lives in, the retention class, and the delete/export posture. |
| `attachment_provenance` | Per-attachment origin, source class, semantic role, provenance class, and trust posture. |
| `mention_provenance` | Per-mention kind, resolution state, resolved target, and in-scope flag. |
| `context_receipts` | Context add-or-remove receipts: what was added, omitted, removed, or policy-filtered, with a precise reason and replay visibility. |
| `evidence` | Replay-safe evidence lineage: replay id, lineage, redaction manifest, route/spend/operator/support/compliance refs, replay safety, and raw-prompt-free flag. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `rollback_posture`, `rollback_verified` | Reversal posture for a scope/retention change and whether it was drilled. |

## Invariants enforced by validation

- **Attachment-and-mention provenance is complete.** Every attachment names its
  origin, stable object identity, source class, and trust posture; a resolved
  mention names a non-empty target, only a resolved mention is in scope, and a
  scope-excluded mention is never silently in scope.
- **Context add-or-remove receipts stay inspectable and precise.** Every context
  change carries a replay-visible receipt so support and audit can see what was
  added, omitted, removed, or policy-filtered; a removal or omission carries a
  precise reason rather than collapsing into an unspecified catch-all; and a
  reversible change names its restore action.
- **Drafts stay scoped and artifacts stay locality-safe.** A draft-in-progress
  must stay session-scoped rather than drifting into durable memory; anything
  durable declares an actionable delete/export posture; a workspace artifact
  never sits in a tenant-wide pinned store; and an org artifact stays
  tenant-pinned so recall never crosses a workspace or tenant boundary by default.
- **Evidence is durable and replay-safe.** Every artifact carries a replay-safe
  evidence id, replay lineage, and redaction manifest; replay never demands raw
  prompt text on the boundary; and a claimed artifact additionally carries route,
  spend, operator, support, and compliance evidence refs plus a replay-safe
  posture and a verified rollback path where one exists.
- **Narrow rather than hide.** Every artifact carries the `proof_stale` and
  `trust_narrowing` downgrade triggers, each narrowing to a strictly lower
  qualification.
- **Export-safe only.** Raw prompt bodies, source file bodies, provider payloads,
  endpoint URLs, credentials, raw token counts, and exact spend amounts never
  cross the boundary; the packet carries refs, coarse classes, state tokens, and
  review-safe labels only.

## Reused vocabularies

The packet grows additively on the frozen M5 lanes rather than forking truth. It
reuses the attachment semantic-role and provenance vocabularies from the richer
prompt composer; the mention-kind, mention-resolution, source-class, trust, and
context-item-state vocabularies from the composer and context inspector; the
scope, locality, retention, delete/export, and omission-reason vocabularies from
the AI memory class lane; and the qualification, consumer-surface,
downgrade-trigger, and rollback-posture vocabularies from the M5 AI workflow
matrix. The spend-and-route receipt schema is cited so the evidence lineage's
route and spend refs project onto the same receipt truth.
