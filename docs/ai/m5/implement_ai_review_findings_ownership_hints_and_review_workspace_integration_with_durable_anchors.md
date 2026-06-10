# AI Review Findings, Ownership Hints, and Review-Workspace Integration with Durable Anchors

## Purpose

This document defines the M5 canonical packet for AI-assisted code review. An AI
review pass produces **findings** about a change, each one anchored to a location
by a **durable anchor** that survives edits, hints at the **owners** of the
affected areas, and integrates into the **review workspace** behind a human gate.
The pass is **read-only** — it never applies a change and never self-publishes
findings without human review. Every claim a finding makes must cite evidence by
id rather than asserting authority on its own. The packet binds four concerns
into one export-safe artifact:

- **Findings** — the issues the pass raised. Each finding carries a class
  (bug risk, security, performance, maintainability, style, test gap, or
  documentation), a severity (blocker, major, minor, nit), and a confidence
  (grounded, probable, speculative); binds to a durable anchor; cites the
  evidence refs that back it; and flags whether it needs human confirmation.
  Findings that cite no evidence are counted and surfaced rather than hidden, and
  no finding may claim authority beyond its cited evidence.
- **Durable anchors** — each finding binds to a location by an anchor strategy
  (symbol path, content hash, structural node, or line range) so it survives
  edits. When the anchored location drifts or is lost, the anchor discloses the
  drift and its rebind disposition rather than silently reattaching to the wrong
  place or vanishing.
- **Ownership hints** — advisory hints for the affected areas. Each hint is a
  typed reference to an owner drawn from a code-owners file, commit history, a
  declared team, or a heuristic, carrying its confidence and whether it is a
  reviewer suggestion. The AI never auto-assigns a reviewer or treats a hint as
  authority.
- **Review-workspace integration** — how the findings reach review: publish
  state, destination, the review-pack digest the lane projects against, and the
  evidence-packet lineage the findings inherit. Publishing into review requires a
  human gate; the AI never self-publishes.

## Scope

The packet covers:

1. **Finding classes** — bug risk, security, performance, maintainability, style,
   test gap, and documentation findings, each with their severity and confidence.
2. **Durable anchors** — symbol-path, content-hash, structural-node, and
   line-range anchors, each recorded with their lifecycle state (bound, drifted,
   rebound, lost), whether drift was detected, and whether the rebind disposition
   is disclosed.
3. **Ownership hints** — code-owners-file, commit-history, declared-team, and
   heuristic hints, each with their confidence (strong, moderate, weak) and
   whether they are a reviewer suggestion. Hints are advisory only.
4. **Review-workspace integration** — draft, staged, published-to-review, and
   withdrawn publish states into a review pack, inline thread, or pull-request
   comment set, gated by a human.
5. **Consumer surface parity** — cross-surface truth for the desktop review
   panel, desktop editor gutter, CLI/headless, browser companion, support export,
   and diagnostics.

## Evidence honesty

- A finding's `evidence_backed` flag must agree with whether it cites any
  evidence ref.
- A finding that cites no evidence must set `requires_human_confirmation`, and the
  block's `uncited_findings_count` must equal the actual count of uncited
  findings.
- No finding may claim authority beyond its cited evidence:
  `no_authority_beyond_evidence` must be true.

## Durable-anchor truth

Every anchor carries `durable = true`. When the anchored location moves or
vanishes — `state` is `drifted`, `rebound`, or `lost`, or `drift_detected` is
true — the anchor must set `rebind_disclosed`: the drift is surfaced rather than
hidden. A drifted or lost anchor that is not disclosed is a validation failure and
narrows the lane via the `anchor_lost_undisclosed` downgrade trigger.

## Ownership-hint posture

Ownership hints are advisory. The block's `hints_are_advisory` and
`no_auto_assignment` must both be true, and every hint row carries `advisory =
true`. The AI never auto-assigns a reviewer or treats a hint as an authority
source; doing so narrows the lane via the `ownership_hint_treated_as_authority`
downgrade trigger. Owner references are opaque refs — never a raw identity,
handle, or email address.

## Read-only and human-gate posture

The pass itself never applies a change: findings are produced before any apply
(`produced_before_apply` must be true). Publishing findings into review requires a
human gate: `human_gate_required` must be true, and whenever `publish_state` is
`published_to_review`, `human_gated` must be true. Every published finding id must
exist in the finding block. The AI never self-publishes; bypassing the gate
narrows the lane via the `published_without_human_gate` downgrade trigger.

## Downgrade and rollback posture

The packet carries explicit downgrade triggers:

- Proof stale
- Policy blocked
- Provider unavailable
- Trust narrowing
- Scope expansion unqualified
- Upstream dependency narrowed
- Anchor lost undisclosed
- Ownership hint treated as authority
- Published without human gate
- Uncited claim surfaced

An undisclosed anchor drift, an ownership hint treated as authority, a publish
that bypasses the human gate, or an uncited claim surfaced as authoritative
narrows the lane claim rather than hiding the deficiency.

## Source contracts

This packet projects against:

- `docs/ai/context_assembly_contract.md` — frozen context-assembly contract for evidence-citation and omitted-context truth
- `docs/ai/m4/ai-review-assist-and-publish-truth.md` — prior canonical AI review-assist truth lane for finding rows, scope selectors, publish-to-review sheets, and resolution memory
- `docs/review/m4/review-pack-evaluator-and-local-ci-parity.md` — stable review-pack evaluator contract for the review-pack digest
- `docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md` — evidence-rich patch review contract for evidence-packet lineage
- `docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md` — M5 AI workflow matrix contract
- `schemas/ai/implement-ai-review-findings-ownership-hints-and-review-workspace-integration-with-durable-anchors.schema.json` — boundary schema

## Privacy and redaction

The record is export-safe. It carries refs, state tokens, coarse classes, counts,
and review labels only. Raw source lines, raw symbol names, raw file paths, raw
diff bodies, raw owner identities, email addresses, raw prompt bodies, provider
payloads, endpoint URLs, credentials, raw token counts, exact prices, and
billing-account ids stay outside the support boundary.
