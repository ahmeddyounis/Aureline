# Prompt-Composer Draft And Session-Artifact Records, Attachment-And-Mention Provenance, Context Add-Or-Remove Receipts, And Durable Replay-Safe Evidence Packets

- Packet: `prompt-session-artifact:stable:0001`
- Schema: `schemas/ai/ship-prompt-composer-draft-and-session-artifact-records-attachment-and-mention-provenance-context-add-or-remove-receipts.schema.json`
- Support export: `artifacts/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/support_export.json`
- Fixture: `fixtures/ai/m5/ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts/`

## Coverage

The packet turns prompt-composer drafts into durable-but-scoped session
artifacts, one row per draft or session, covering the desktop composer, review
workspace, and browser companion surfaces. Every artifact carries the
attachment-and-mention provenance that explains where its context came from, the
context add-or-remove receipts that record what was added, omitted, removed, or
policy-filtered, the scope/locality/retention/delete posture that keeps it
inspectable without drifting into hidden memory, and the replay-safe evidence
lineage that lets users, support, and compliance inspect the turn without raw
prompt text.

- The inline assist edit resolves to the desktop composer at Stable: a
  workspace-scoped, workspace-local active session artifact, durable until
  deleted with a workspace-scoped delete/export posture. It carries a primary
  workspace attachment and a docs-pack reference, a resolved symbol mention plus a
  scope-excluded mention that stays out of scope, and replay-visible add and
  budget-omission receipts. Its evidence is fully replay-safe.
- The patch review session resolves to the review workspace at Beta: retained
  until the user revokes consent with a user-scoped delete posture and a redacted
  replay-safe evidence lineage.
- The docs companion draft resolves to the browser companion at Preview: a
  thread-scoped, session-only draft-in-progress that stays scoped rather than
  drifting into durable memory. Its evidence is redacted replay-safe and
  raw-prompt-free.
- The deleted composer session resolves to a deleted tombstone at Held: its body
  is gone but its scope-revocation context receipt stays replay-visible so support
  and audit can still explain what was filtered out.

## What this proves

- Prompt-composer drafts and session artifacts preserve attachment origin,
  mention resolution, context addition or removal, and replay-safe evidence IDs
  across the first real M5 AI surfaces.
- Users, support, and compliance exports can inspect what context was added,
  omitted, removed, or policy-filtered without raw prompt text in every packet.
- Draft and session artifacts stay scoped, deletable, and locality-safe rather
  than drifting into hidden durable memory, and recall never crosses a workspace
  or tenant boundary by default.
