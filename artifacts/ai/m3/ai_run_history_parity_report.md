# AI Run History Parity Report

Parity report for the durable AI run history, approval timeline, and
rerun-review baseline owned by `aureline_ai::run_history`. One
[`AiRunHistoryEntry`] is the canonical row for a material AI run: it
carries the canonical run id that the AI thread, evidence panel, support
packet, and replay view all read; the actor and provider/model identity
that issued the run; the execution boundary it crossed; an ordered
approval timeline that preserves every approve, deny, revoke, expire, and
policy-block event with its own scope, policy epoch, and actor/object
identity; and a typed `evidence_completeness_class` that records when
provider, connector, or policy drift means the run can no longer be
reconstructed in full fidelity. One [`AiRerunReview`] compares the
original run against the current workspace, policy, provider, model, and
tool state, re-resolves the approvals it would need against the current
policy epoch, and exposes typed `Rerun`, `Cancel`, and `OpenAsRecipe`
action offers without hiding drift.

## Source contracts

- `schemas/ai/ai_run_history_entry.schema.json` — boundary schema for one
  durable run-history row.
- `schemas/ai/ai_rerun_review.schema.json` — boundary schema for one
  rerun-review sheet.
- `schemas/ai/evidence_packet.schema.json` — boundary schema the
  evidence-lineage refs project against.
- `schemas/ai/evidence_replay_packet.schema.json` — boundary schema the
  replay-view surface projects against.
- `schemas/ai/spend_receipt.schema.json` — boundary schema the
  cost/quota band rolls up.
- `schemas/ai/tool_call_timeline_entry.schema.json` — boundary schema the
  per-run tool-call lineage refs point at.
- `schemas/integration/approval_ticket.schema.json` — boundary schema the
  approval-timeline rows preserve refs into.

Reviewer fixtures live under `fixtures/ai/m3/run_history_and_replay/`.

## Canonical run coverage

The canonical parity packet covers the three identities the durable
history record MUST be able to preserve end-to-end:

- `ai-run:applied:local:0001` — Local reversible edit. Approval timeline
  records the pending prompt *and* the granted decision as separate rows.
  Evidence is reconstructible in full fidelity; rerun review admits Rerun
  after re-resolving the approval against the current policy epoch.
- `ai-run:denied:remote:0001` — Remote AI run denied because the admin
  approval ticket for the issues connector was missing. The approval
  timeline preserves the pending request and the policy-gate block as
  separate events instead of collapsing them into one final status.
  Rerun review reports `missing_required_approval` and denies Rerun with
  the typed reason `approval_missing`, but still offers
  `open_as_recipe` and `request_approval_renewal`.
- `ai-run:revoked:branch-agent:0001` — Background branch-agent dispatch
  whose admin ticket was revoked after the vendor provider was withdrawn.
  Evidence completeness drops to `evidence_incomplete_degraded_replay`
  with the typed reason `provider_withdrawn`. Rerun review records
  `removed_or_withdrawn` drift on the provider and model axes and denies
  Rerun with the typed reason `provider_withdrawn`.

## Approval timeline coverage

The packet validates the following approval decision classes as separate
timeline rows on at least one canonical run:

- `pending_user_review`
- `granted`
- `revoked`
- `blocked_by_policy`

Two further decision classes — `rejected` and `expired` — remain part of
the canonical vocabulary so the schema can record them without dropping
identity or note material.

## Rerun admission coverage

The canonical packet exercises one admit class and two deny classes so
each rerun-review code path is reviewer-visible:

- `admit_rerun` — Local reversible edit; no drift on any required axis;
  approval freshly re-resolved.
- `deny_rerun_approval_unresolved` — Admin approval is still missing in
  the current epoch.
- `deny_rerun_provider_unavailable` — Original provider was withdrawn
  and replay is degraded; rerun cannot inherit removed authority.

## Acceptance invariants

The parity packet validates the following invariants:

1. **Canonical run id is preserved across surfaces.** AI thread,
   evidence panel, support packet, and replay view all carry the same
   `canonical_run_id` set for every recorded run; the surface parity
   rows are mechanically checked against the packet's entry ids.
2. **Approval timelines never collapse into one final status.** Every
   approve, deny, revoke, expire, and policy-block event remains its
   own row with decision class, scope, actor identity, object identity,
   policy epoch, optional expiry, and optional revocation note. Revoked
   events MUST carry a reviewer-visible revocation note.
3. **Applied runs MUST carry a granted approval event.** The validator
   rejects an applied entry whose timeline has no `granted` row, so
   applied state can never be displayed without admissible approval
   lineage.
4. **Terminal rows MUST carry `completed_at`.** Applied, rejected, and
   cancelled rows always carry a terminal timestamp; active rows never
   do.
5. **Rerun never silently inherits expired authority.** Rerun admission
   is never `admit_rerun` while a drift row is blocking
   (`material_drift` or `removed_or_withdrawn`) or while approval
   resolution is anything other than
   `all_required_freshly_resolved` or
   `not_applicable_no_approvals_required`. Rerun denials always carry
   a typed reason class and a reviewer-visible note.
6. **Drift is shown verbatim.** Rerun review sheets always cover the
   workspace-revision, policy-epoch, provider-lifecycle,
   model-lifecycle, and tool-availability axes. Even "no drift" rows
   are recorded so the sheet shows verbatim what stayed stable; the
   sheet never hides that the workspace, policy, provider, or tool
   availability has changed.
7. **`Open as recipe` always remains an offer.** Every rerun review
   includes `open_as_recipe` in its action offers, so an operator can
   recover the original run as a recipe even when Rerun is denied.
8. **Evidence-incomplete states are typed and explained.** When a
   provider, connector, or policy change means Aureline cannot
   reconstruct full fidelity later, the entry MUST set
   `evidence_completeness_class` to a degraded or blocked value, carry
   a typed `evidence_incompleteness_reason_class`, and include a
   reviewer-visible note. The packet exercises `provider_withdrawn`.
9. **Export is metadata-only.** The deterministic JSON export is
   mechanically scanned for raw endpoint URLs, API keys, OAuth tokens,
   and bearer-prefixed strings; the validator rejects the packet if any
   forbidden material crosses the boundary.

## Out of scope

This baseline does not broaden into long-term analytics, full autonomous
background-agent orchestration, or stable cross-tenant sharing of AI
histories. The run-history and rerun-review records are additive over
the existing evidence, routing, spend-receipt, and tool-gateway records
and never accept raw prompts, raw provider payloads, raw diff bodies,
raw endpoint URLs, raw token counts, raw cost amounts, or credential
material.
