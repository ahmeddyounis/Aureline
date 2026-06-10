# Branch or Worktree Agent Lifecycle, Side-Branch Review Packs, and Merge-Back Handoff

## Purpose

This document defines the M5 canonical packet for the branch or worktree agent
lifecycle. It implements an AI agent run isolated to a side branch or worktree,
the reviewable pack it produces, and the human-gated handoff that lands the
work. The packet binds three concerns into one export-safe artifact:

- **Agent lifecycle** — one execution locus plus a stage timeline (launch review
  → planning → isolated editing → validation → approval → review-ready →
  merge-back handoff → completion) where every mutating stage carries preview,
  approval, and checkpoint evidence and never writes outside its admitted
  isolation.
- **Side-branch review pack** — the produced change bound to the upstream
  evidence-rich patch review lane by id (diff packet, validation receipt, and
  rollback handle refs) plus an evidence packet ref and disclosed findings,
  produced inside isolation and reviewable before any merge.
- **Merge-back handoff** — the landing bound to a human: the agent may never
  self-merge or self-push to a protected destination, the merge requires human
  approval, and reviewable artifacts survive cleanup.

## Scope

The packet covers:

1. **Execution locus separation** — local-only assist, isolated side worktree,
   side branch without worktree, ephemeral workspace, and managed remote
   workspace stay distinct. The locus vocabulary is reused from the background
   branch-agent lifecycle lane rather than re-invented.
2. **Lifecycle stages** — each stage row records its checkpoint, preview,
   approval, artifact preservation, and whether it stayed inside isolation.
3. **Review packs** — the diff packet, validation receipt, and rollback handle
   are referenced by id into the evidence-rich patch review lane, never embedded.
4. **Merge-back handoff** — the merge state has no agent-driven landing variant;
   only `merged_by_human` reaches the destination, and only with human approval.
5. **Consumer surface parity** — cross-surface truth for the desktop agent
   workspace, desktop review workspace, CLI/headless, browser/companion, support
   export, and diagnostics.

## Downgrade and rollback posture

The packet carries explicit downgrade triggers:

- Proof stale
- Policy blocked
- Provider unavailable
- Trust narrowing
- Scope expansion unqualified
- Upstream dependency narrowed
- Isolation breach
- Review pack missing
- Merge-back unapproved

An isolation breach, a missing review pack, or an unapproved merge-back
automatically narrows the lane claim rather than hiding the deficiency.

## Guardrails

- The agent never self-merges and never self-pushes to a protected destination.
- Merge-back always requires human approval; reaching the destination without it
  is a validation failure.
- Reviewable artifacts (diff, validation, rollback, evidence, findings) survive
  cleanup so a human can always inspect what the run did.

## Source contracts

This packet projects against:

- `docs/ai/background_branch_agent_lifecycle.md` — frozen branch-agent lifecycle base contract
- `docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md` — evidence-rich patch review contract
- `docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md` — M5 AI workflow matrix contract
- `schemas/ai/implement-branch-or-worktree-agent-lifecycle-side-branch-review-packs-and-merge-back-handoff.schema.json` — boundary schema

## Privacy and redaction

The record is export-safe. It carries refs, state tokens, coarse classes,
counts, and review labels only. Raw branch names, raw worktree paths, raw diff
bodies, raw patch text, raw logs, raw prompt bodies, source file bodies, provider
payloads, endpoint URLs, credentials, raw token counts, exact prices, and
billing-account ids stay outside the support boundary.
