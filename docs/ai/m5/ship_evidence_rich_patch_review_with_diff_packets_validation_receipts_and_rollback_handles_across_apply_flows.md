# Evidence-Rich Patch Review with Diff Packets, Validation Receipts, and Rollback Handles Across Apply Flows

## Purpose

This document defines the M5 canonical packet for evidence-rich patch review. It binds structured diff packets, validation receipts, and rollback handles into one export-safe artifact for the three AI apply flows:

- **Inline assist** — composer-assisted quick edits and small-scope patches.
- **Patch review** — AI-assisted review of proposed changes, finding publication, and resolution.
- **Branch or worktree agent** — background long-running tasks isolated to a branch or worktree.

## Scope

The packet covers:

1. **Diff packets** — content-addressed hunk-level diff inventory with per-file and per-hunk disclosure, approval, and scope honesty.
2. **Validation receipts** — attributable validation evidence (lint, type-check, test, security-scan, format, build) bound to the exact patch digest.
3. **Rollback handles** — recoverable checkpoint lineage and revert availability scoped to the apply flow.
4. **Apply-flow bindings** — per-flow state and parity across inline assist, patch review, and branch or worktree agents.
5. **Consumer surface parity** — cross-surface truth for desktop composer, desktop review workspace, CLI/headless, browser/companion, support export, and diagnostics.

## Downgrade and rollback posture

The packet carries explicit downgrade triggers:

- Proof stale
- Policy blocked
- Provider unavailable
- Trust narrowing
- Scope expansion unqualified
- Upstream dependency narrowed
- Validation failed or missing
- Rollback unavailable

A stale proof, failed validation, or missing rollback handle automatically narrows the lane claim rather than hiding the deficiency.

## Source contracts

This packet projects against:

- `artifacts/ai/multifile_patch_review_sequence.md` — multi-file patch review sequence contract
- `docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md` — M5 AI workflow matrix contract
- `docs/commands/alpha_preview_apply_revert.md` — preview/apply/revert enforcement contract
- `schemas/ai/ship-evidence-rich-patch-review-with-diff-packets-validation-receipts-and-rollback-handles-across-apply-flows.schema.json` — boundary schema

## Privacy and redaction

The record is export-safe. It carries refs, state tokens, coarse classes, counts, digests, and review labels only. Raw diff bodies, raw patch text, raw prompt bodies, source file bodies, provider payloads, endpoint URLs, credentials, raw token counts, exact prices, and billing-account ids stay outside the support boundary.
