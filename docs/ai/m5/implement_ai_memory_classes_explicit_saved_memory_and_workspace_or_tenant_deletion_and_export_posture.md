# AI Memory Classes, Explicit Saved Memory, and Workspace/Tenant Deletion and Export Posture

## Purpose

This document defines the M5 canonical packet that locks how the AI's memory
classes are deleted and exported at workspace and tenant scope into one
export-safe artifact. It builds on the stable AI memory-state lane
(`docs/ai/ai-memory-delete-export.md`, `schemas/ai/ai-memory-state.schema.json`),
which froze the six product AI memory classes and their per-class delete/export
posture, and adds the operator-facing dimension that lane left to later work:
actually executing a workspace-scoped or tenant-scoped delete or export across
every memory class, with disclosed retention holds, accountable explicit saved
memory, per-class fan-out completeness, verified receipts, and honest
partial/blocked states. The packet binds three blocks:

- **Memory-class coverage** — asserts that every product AI memory class (turn
  state, conversation thread, prompt/result cache, reusable repo facts and
  summaries, retained evidence copy, and explicit saved memory) is addressed by
  the deletion and export fan-out. Each row reuses the frozen memory-class
  vocabulary and records whether delete and export fan-out cover the class, the
  retention hold (if any) that keeps an evidence-governed copy beyond a delete
  request, and whether that hold is disclosed rather than silently skipped.
- **Explicit saved memory** — the explicit saved-memory entries in scope. Each
  carries the scope it was saved at (user, repo, or org), the accountable actor
  who saved it (end user, repo owner, or org admin), the consent posture
  (explicitly consented or disclosed policy default), and whether it is
  revocable. Saved memory is never anonymous, never unconsented, and always
  revocable.
- **Scoped deletion/export operations** — the workspace- or tenant-scoped delete
  and export operations. Each carries the operation kind (deletion or export),
  the scope it ran at (workspace, repository, tenant, account, or organization),
  its per-class fan-out completeness, the receipt handle it produced, and its
  receipt verification state. A completed operation addresses every class and
  carries a verified receipt; a partial operation states its incompleteness
  honestly rather than claiming completion.

## Scope

The packet covers:

1. **Memory-class coverage** — delete/export fan-out coverage and retention-hold
   disclosure for every product memory class.
2. **Explicit saved memory** — scope, accountable actor, consent, and revocability
   for every saved entry.
3. **Scoped operations** — operation kind, scope, fan-out completeness, receipt,
   and verification for every workspace- or tenant-scoped delete or export.
4. **Consumer surface parity** — cross-surface truth for the memory inspector,
   delete/export review, admin console, CLI/headless, support export, and
   diagnostics.

## Memory-class coverage posture

Every product memory class must appear in the coverage rows
(`all_classes_covered` must be true and the required-coverage set must be
present), and every row carries `disclosed = true`. A class that is not covered
by the delete fan-out (`delete_fan_out_covered = false`) must carry a retention
hold (`evidence_hold` or `legal_hold`) that is disclosed (`hold_disclosed =
true`); otherwise it narrows the lane via the `delete_fan_out_incomplete`
trigger. A retention hold applied without disclosure narrows the lane via the
`retention_hold_undisclosed` trigger. Every class must be covered by the export
fan-out (`export_fan_out_covered = true`); an uncovered class narrows the lane
via the `export_fan_out_incomplete` trigger.

## Explicit saved-memory posture

Saved memory is never anonymous, never unconsented, and always revocable. Every
entry carries an accountable actor (`end_user`, `repo_owner`, or `org_admin`,
never `unattributed`), a permitted consent posture (`explicitly_consented` or
`policy_default`, never `not_consented`), and `revocable = true`. An entry
missing an accountable owner narrows the lane via the
`saved_memory_without_accountable_owner` trigger. The block-level
`owner_accountable` and `all_revocable` flags must be consistent with every row.

## Scoped deletion/export posture

Each operation carries a receipt handle (`receipt_ref`) when receipts are
required (`receipts_required = true`) and is disclosed. An operation that claims
a complete fan-out (`fan_out_completeness = all_classes_covered`) must address
every class (`all_classes_addressed = true`) and carry a verified receipt
(`receipt_verification = verified_receipt`); a claimed-complete operation that
does not address every class overstates completion, and one without a verified
receipt narrows the lane via the `deletion_receipt_unverified` trigger. A partial
operation states its incompleteness with `partial_pending_retention_hold` rather
than claiming completion.

## Downgrade and rollback posture

The packet carries explicit downgrade triggers:

- Proof stale
- Policy blocked
- Provider unavailable
- Trust narrowing
- Scope expansion unqualified
- Upstream dependency narrowed
- Delete fan-out incomplete
- Export fan-out incomplete
- Saved memory without accountable owner
- Deletion receipt unverified
- Retention hold undisclosed

An incomplete delete or export fan-out, saved memory without an accountable
owner, an unverified completion receipt, or an undisclosed retention hold narrows
the lane claim rather than hiding the deficiency.

## Source contracts

This packet projects against:

- `docs/ai/ai-memory-delete-export.md` — stable AI memory delete/export contract for the per-class delete/export posture this lane executes
- `docs/ai/memory_and_reconciliation_contract.md` — lower-level memory/reconciliation contract for memory-class semantics and provenance bindings
- `schemas/ai/memory_object.schema.json` — lower-level memory-object boundary schema
- `artifacts/ai/memory_classes.yaml` — machine-readable memory-class matrix and boundary invariants
- `docs/ai/context_assembly_contract.md` — frozen context-assembly contract for evidence-citation and omitted-context truth
- `docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md` — M5 AI workflow matrix contract
- `schemas/ai/implement-ai-memory-classes-explicit-saved-memory-and-workspace-or-tenant-deletion-and-export-posture.schema.json` — boundary schema

## Privacy and redaction

The record is export-safe. It carries refs, state tokens, coarse classes, counts,
and review labels only. Raw saved-memory bodies, raw prompts, responses, terminal
transcripts, raw vectors, raw provider payloads, raw support bodies, credentials,
endpoint URLs, raw file paths, and billing-account ids stay outside the support
boundary.
