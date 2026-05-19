# AI run history and replay fixtures

Canonical fixtures for the durable AI run history, approval timeline, and
rerun-review baseline exercised by `aureline_ai::run_history`.

## Files

- `ai_run_history_parity_packet.json` — one parity packet covering three
  canonical run identities and three matching rerun reviews. The packet
  preserves the same `canonical_run_id` set across the AI thread,
  evidence-panel, support-packet, and replay-view surfaces.

## Canonical runs

1. `ai-run:applied:local:0001` — Reversible local edit that landed after a
   pending → granted approval pair. Evidence is reconstructible in full
   fidelity. Paired with rerun review `rerun-review:applied:local:0001`,
   which records no drift on the five required axes, re-resolves the
   approval against the current policy epoch, and admits Rerun.
2. `ai-run:denied:remote:0001` — Remote AI run denied because an admin
   approval ticket for the issues connector was missing. Approval timeline
   records the pending request *and* the policy-gate block as separate
   events (the schema MUST NOT collapse this into one final status).
   Paired with rerun review `rerun-review:denied:remote:0001`, which
   reports `missing_required_approval` and denies Rerun with the typed
   reason `approval_missing` while still offering `open_as_recipe`.
3. `ai-run:revoked:branch-agent:0001` — Background branch-agent run that
   landed an applied change, then had its admin ticket revoked when the
   vendor provider was withdrawn. Evidence completeness drops to
   `evidence_incomplete_degraded_replay` with the typed reason
   `provider_withdrawn`. Paired with rerun review
   `rerun-review:revoked:branch-agent:0001`, which records
   `removed_or_withdrawn` drift on the provider and model lifecycle axes
   and denies Rerun with the typed reason `provider_withdrawn`.

## Invariants exercised

- Every run preserves the canonical run id across composer thread,
  evidence packet, support packet, and replay view (mechanically checked
  by the surface parity rows in the packet).
- Every approval/deny event remains a separate timeline row with its own
  decision class, scope, actor identity, object identity, policy epoch,
  optional expiry, and optional revocation note.
- Revoked approval events MUST carry a reviewer-visible revocation note.
- Applied runs MUST carry a granted approval event somewhere in their
  timeline.
- Terminal rows (`applied` / `rejected` / `cancelled`) MUST carry a
  `completed_at` timestamp; active rows MUST NOT.
- Rerun reviews always include all five required drift axes
  (`workspace_revision`, `policy_epoch`, `provider_lifecycle`,
  `model_lifecycle`, `tool_availability`) and always offer
  `open_as_recipe` so an operator can recover a recipe even if Rerun is
  denied.
- Rerun admission is never `admit_rerun` while any drift row is
  blocking or any required approval is unresolved.
- Rerun denials always carry a typed reason class and a reviewer-visible
  note.
- Incomplete evidence states (`evidence_incomplete_degraded_replay` /
  `evidence_incomplete_replay_blocked`) MUST carry a typed reason class
  and a reviewer-visible note, so degraded replay is never surfaced as
  if it were full-fidelity replay.
- Export-safe JSON contains no raw endpoint URLs, API keys, OAuth tokens,
  or bearer prefixes; the structural check is built into the validator.
