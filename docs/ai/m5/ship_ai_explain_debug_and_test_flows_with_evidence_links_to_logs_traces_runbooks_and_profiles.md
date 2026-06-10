# AI Explain, Debug, and Test Flows with Evidence Links to Logs, Traces, Runbooks, and Profiles

## Purpose

This document defines the M5 canonical packet for the AI explain, debug, and
test work loops. Each flow answers a question — explain this behavior, debug
this failure, or propose a test — and every claim it makes must cite evidence by
id rather than asserting authority on its own. The flow is **read-only** — it
never applies a change itself. When a test flow produces a candidate edit it
hands that candidate to the evidence-rich patch review apply lane by id and keeps
the human-review boundary intact. The packet binds three concerns into one
export-safe artifact:

- **Flow** — one flow kind (explain, debug, or test), intent, target, and scope,
  with a flow state, the assertion that every claim must cite evidence, and the
  assertion that the flow never self-applies.
- **Evidence links** — the links the flow consumed into logs, traces, runbooks,
  and profiles. Each link carries its source surface, freshness, provenance, and
  trust labels, plus whether it resolved. When a link cannot be resolved or is
  stale, the gap stays visible rather than being silently dropped.
- **Findings** — the answers the flow produced. Each finding cites the evidence
  link ids that back it, carries a confidence class, and flags whether it needs
  human confirmation. Findings that cite no evidence are counted and surfaced
  rather than hidden, and no finding may claim authority beyond its cited
  evidence.

## Scope

The packet covers:

1. **Flow kinds** — explain, debug, and test stay distinct.
2. **Evidence kinds** — log, trace, runbook, and profile evidence are each drawn
   from their canonical source surface (log store, trace store, runbook registry,
   profile store) and recorded with their freshness (fresh, aging, stale,
   unknown), provenance (recorded run, live capture, imported artifact,
   synthesized summary), and trust (trusted, unverified, untrusted) labels.
3. **Findings** — explanation, root-cause hypothesis, repro step, fix
   recommendation, test case, and caveat findings are each recorded with their
   confidence (grounded, probable, speculative) and the evidence link ids that
   back them.
4. **Apply handoff** — a test flow hands a selected candidate to the
   evidence-rich patch review lane by id; the flow has no agent-driven apply state
   of its own.
5. **Consumer surface parity** — cross-surface truth for the desktop explain
   panel, desktop debug console, desktop test runner, CLI/headless, support
   export, and diagnostics.

## Evidence honesty

- Every claim must cite evidence: `evidence_required_for_claims` must be true.
- A finding's `evidence_backed` flag must agree with whether it cites any
  evidence link, and every cited link id must exist in the evidence-link block.
  A dangling citation is a validation failure.
- A finding that cites no evidence must set `requires_human_confirmation`, and the
  block's `uncited_findings_count` must equal the actual count of uncited
  findings.
- No finding may claim authority beyond its cited evidence:
  `no_authority_beyond_evidence` must be true.

## Omitted-context truth

When not every evidence link resolves, `unresolved_reason_disclosed` must be
true: the gap in the evidence set is surfaced rather than hidden. When stale
evidence is present, `stale_evidence_present` and `stale_disclosed` must both be
true. Every evidence link carries `disclosed = true`; a link that reaches the set
without disclosure is a validation failure.

## Read-only and apply posture

The flow itself never applies a change: `read_only` must be true, and findings
are produced before any apply. A test flow that yields a candidate edit sets
`apply_handoff_ref` to a reference into the evidence-rich patch review apply lane;
that lane owns the diff packet, validation receipt, rollback handle, and
human-review boundary.

## Downgrade and rollback posture

The packet carries explicit downgrade triggers:

- Proof stale
- Policy blocked
- Provider unavailable
- Trust narrowing
- Scope expansion unqualified
- Upstream dependency narrowed
- Evidence unresolved
- Evidence stale undisclosed
- Uncited claim surfaced

Unresolved evidence, undisclosed stale evidence, or an uncited claim surfaced as
authoritative narrows the lane claim rather than hiding the deficiency.

## Source contracts

This packet projects against:

- `docs/ai/context_assembly_contract.md` — frozen context-assembly contract for evidence-link and omitted-context truth
- `docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md` — evidence-rich patch review contract for the test-flow apply handoff
- `docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md` — M5 AI workflow matrix contract
- `schemas/ai/ship-ai-explain-debug-and-test-flows-with-evidence-links-to-logs-traces-runbooks-and-profiles.schema.json` — boundary schema

## Privacy and redaction

The record is export-safe. It carries refs, state tokens, coarse classes, counts,
and review labels only. Raw log lines, raw trace bodies, raw runbook text, raw
profile samples, raw symbol names, raw file paths, raw diff bodies, raw prompt
bodies, provider payloads, endpoint URLs, credentials, raw token counts, exact
prices, and billing-account ids stay outside the support boundary.
