# AI Refactor Planner with Impact Sets, Candidate Previews, and Multi-File Safety Classes

## Purpose

This document defines the M5 canonical packet for the AI refactor planner. The
planner plans a refactor, computes the set of impacted sites with multi-file
safety classification, and presents preview candidates that bind to the
evidence-rich patch review lane before any apply. The planner is **preview-only**
— it never applies a change itself. The packet binds three concerns into one
export-safe artifact:

- **Refactor plan** — one refactor kind, intent, and scope, with a plan state and
  the assertion that a preview is required before any apply.
- **Impact set** — every impacted site with its site class, resolution
  confidence, and multi-file safety class, plus the disclosed worst-case safety
  class across the set and whether the analysis is complete. When the impact set
  is partial — for example dynamic or reflective references that cannot be
  statically resolved — the reason stays visible rather than being silently
  dropped.
- **Candidate previews** — preview candidates that each reference a diff packet,
  validation receipt, and rollback handle into the evidence-rich patch review
  lane by id, carry their multi-file safety class, require human review before
  apply, and block auto-apply for any unsafe safety class.

## Scope

The packet covers:

1. **Refactor kinds** — rename symbol, extract function/variable, inline symbol,
   move item, change signature, introduce parameter, replace pattern, and
   organize imports stay distinct.
2. **Impact sites** — definition, reference, test, doc, generated, public-API
   boundary, cross-crate boundary, and dynamic/reflective sites are each recorded
   with their resolution confidence (resolved, heuristic, ambiguous).
3. **Multi-file safety classes** — `mechanical_single_file`,
   `mechanical_multi_file`, `semantic_local`, `semantic_cross_boundary`,
   `behavior_affecting`, and `ambiguous_unsafe`, ordered from least to most
   risky. Only the mechanical classes are auto-applicable; every other class
   requires a human review boundary.
4. **Candidate previews** — the diff packet, validation receipt, and rollback
   handle are referenced by id into the evidence-rich patch review lane, never
   embedded. A selected candidate is handed to that apply flow; the planner has
   no agent-driven apply state of its own.
5. **Consumer surface parity** — cross-surface truth for the desktop refactor
   panel, desktop review workspace, CLI/headless, browser/companion, support
   export, and diagnostics.

## Safety honesty

- The disclosed `highest_safety_class` is at least as risky as every site in the
  impact set. Understating the worst case is a validation failure.
- A candidate whose safety class is not auto-applicable
  (`semantic_local` and above) must set `auto_apply_blocked_for_unsafe_class`.
  Letting an unsafe class auto-apply is a validation failure.
- Every candidate requires human review before apply, and candidates are produced
  before any apply.

## Omitted-context truth

When `analysis_complete` is false, `partial_reason_disclosed` must be true: the
gap in the impact set is surfaced rather than hidden. Every impacted site carries
`disclosed = true`; a site that reaches the set without disclosure is a
validation failure.

## Downgrade and rollback posture

The packet carries explicit downgrade triggers:

- Proof stale
- Policy blocked
- Provider unavailable
- Trust narrowing
- Scope expansion unqualified
- Upstream dependency narrowed
- Impact set incomplete
- Candidate preview missing
- Unsafe class auto-applied

An incomplete impact set, a missing candidate preview, or an unsafe class that
reached auto-apply narrows the lane claim rather than hiding the deficiency.

## Source contracts

This packet projects against:

- `docs/ai/context_assembly_contract.md` — frozen context-assembly contract for impact-set and omitted-context truth
- `docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md` — evidence-rich patch review contract
- `docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md` — M5 AI workflow matrix contract
- `schemas/ai/add-the-ai-refactor-planner-with-impact-sets-candidate-previews-and-multi-file-safety-classes.schema.json` — boundary schema

## Privacy and redaction

The record is export-safe. It carries refs, state tokens, coarse classes, counts,
and review labels only. Raw symbol names, raw file paths, raw diff bodies, raw
patch text, raw source bodies, raw prompt bodies, provider payloads, endpoint
URLs, credentials, raw token counts, exact prices, and billing-account ids stay
outside the support boundary.
