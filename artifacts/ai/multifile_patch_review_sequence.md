# AI multi-file patch review sequence (review packet + validation attachment + apply audit)

Reviewer-side packet describing the **request → review → approve → apply** sequence for AI-generated multi-file patches. The goal is to make it mechanically impossible for an AI-driven workspace mutation to:

- bypass policy evaluation,
- bypass review presentation,
- apply anything beyond the exact diff hunks the user approved, or
- claim validation evidence that is not attributable to the exact proposed patch.

This packet composes with (and does not replace) upstream contracts. If this packet disagrees with upstream sources, upstream sources win and this packet plus its companion fixtures update in the same change:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — critical sequence “AI multi-file patch with explicit approval”.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` and `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` — AI patch review expectations, evidence export fields, and stable review-state vocabulary.
- `docs/ai/context_assembly_contract.md`, `schemas/ai/context_assembly.schema.json` — task staging, assembly, tool-call lineage.
- `schemas/ai/evidence_packet.schema.json` — evidence packet and tainted-fence requirements for AI claims.
- `docs/ai/spend_and_route_receipt_contract.md`, `schemas/ai/provider_route_receipt.schema.json`, `schemas/ai/spend_receipt.schema.json` — provider/model/route/spend truth.
- `docs/ux/shell_interaction_safety_contract.md`, `schemas/ux/interaction_safety.schema.json` — preview/apply/validate/keep-or-revert phases and no-basis-drift apply rules.
- `docs/workspace/mutation_lineage_model.md`, `schemas/workspace/mutation_journal.schema.json` — mutation group, checkpoint refs, and reversal/undo honesty.
- `schemas/integration/approval_ticket.schema.json` — approval tickets; AI flows may request but never mint authority.
- `schemas/security/audit_stream_record.schema.json` — security audit stream entries for apply actions.
- `schemas/ai/patch_review_summary.schema.json` — the review-packet summary, validation attachment, approval record, and apply audit record defined by this packet.

## 1. Scope

This sequence applies to any AI-assisted workflow that:

1. produces a **proposed multi-file patch** intended to mutate a workspace, and
2. requires an explicit user review/approval step before the apply engine mutates the live tree.

It covers:

- task intent capture,
- policy evaluation output as an inspectable boundary (redaction + tool allowance + scope limits),
- context package assembly,
- provider plan + proposed patch capture,
- optional validation execution and its attach-to-patch rule,
- review presentation,
- approval outcomes (approve / reject / amend / rerun / narrow),
- apply execution (approved-only, no scope widening),
- undo/checkpoint capture, and
- audit artifacts that prove applied scope matches approved scope.

Out of scope:

- provider-specific model quality tuning,
- any workflow that mutates provider-owned remote state (those route through provider mutation contracts),
- end-user feature breadth beyond the protected patch flow.

## 2. Invariants (frozen)

The sequence is non-conforming unless all invariants below hold:

1. **Policy first.** A policy evaluation result (trust + egress + tools + quotas + redaction stance) is obtained before any provider dispatch and before any apply attempt.
2. **Review is evidence, not authority.** The AI output is never treated as self-authorizing. Apply requires an explicit approval record plus an approval ticket where required.
3. **Proposed patch is content-addressed.** The review surface renders a diff derived from a content-addressed patch artifact; the patch digest is part of the review packet summary.
4. **Validation is patch-attributable.** A validation summary MUST bind to the patch artifact digest (and the assembly it was derived from). Validation summaries must not float as “background” or “generic good news”.
5. **Approval is scope-specific.** The approval record must enumerate either “all hunks” or an explicit hunk allowlist; an approval with no scope is non-conforming.
6. **Apply is approved-only.** The apply engine may only materialize hunks that are explicitly approved. Any attempt to apply an unapproved hunk must fail closed (or roll back) and mint an apply audit record showing the denied set.
7. **No silent basis drift.** If the diff basis changed (workspace drift, base revision drift, file identity drift), the apply engine must invalidate approval and reopen review rather than applying “best effort”.
8. **Undo/checkpoint is captured before commit.** Any apply attempt that could touch multiple files captures an undo checkpoint before the first live-tree mutation is committed.
9. **Audit proves lineage.** The apply audit record must cite (by id) the review packet summary, the approval record, the patch artifact digest, the mutation group id, and the checkpoint ref(s).

## 3. Canonical sequence (records and stage boundaries)

The high-level control flow matches the architecture critical sequence. This packet freezes the **record trail** that makes the flow reviewable and auditable.

### 3.1 Task intent and policy evaluation

- Mint (or resolve) a `prompt_composer_turn_draft_record` and `request_workspace_record` describing the patch-flow staging scope.
- Evaluate policy and trust for the intended action class (AI workspace mutation), producing:
  - policy context (`policy_epoch`, `trust_state`, `execution_context_id`),
  - a redaction stance (`redaction_class`),
  - an allowlist of data classes and tools (directly or through the composer plan’s narrowing posture),
  - any typed denials or required approvals.

### 3.2 Context package assembly

- Assemble context into one `ai_context_assembly_record` that enumerates included/omitted/pinned/redacted/policy-blocked/tainted segments.
- Ensure tool-call lineage is preserved via `ai_tool_call_lineage_record` rows when tool outputs influence the context package.

### 3.3 Provider plan and proposed patch

- Dispatch to the provider under a routed path that mints:
  - one `provider_route_receipt_record`,
  - one `spend_receipt_record`,
  - and one `ai_evidence_packet_record` that can later cite sources for claims.
- Capture the provider’s plan as a reviewable plan summary (step labels + constraints), and capture the proposed patch as a content-addressed patch artifact.

### 3.4 Optional validation (pre-approval)

- If validation is attempted (tests/lint/build), capture validation as:
  - a `patch_validation_summary_record` that binds to the patch artifact digest,
  - plus references to the tool-call lineage ids and any execution run summary ids.
- Validation may fail or be skipped, but it must do so with typed outcomes. A missing validation run must not be represented as a pass.

### 3.5 Review presentation (pre-apply)

- Mint exactly one `patch_review_summary_record` that binds:
  - task intent refs,
  - policy context + redaction stance + tool allowance posture,
  - context assembly refs,
  - provider route/spend receipt refs,
  - proposed patch artifact ref + digest + hunk inventory,
  - validation summary refs (when present),
  - and the review surface’s declared scope counts (files/hunks, included/blocked).

The review summary is the **frozen review packet**: the apply engine and support/audit tooling treat it as the sole “what exactly was reviewed?” reference.

### 3.6 Approval outcomes

An approval action mints exactly one `patch_review_approval_record` that:

- cites the reviewed packet summary id,
- names exactly one approval action class (approve/reject/amend/rerun/narrow),
- enumerates the approved hunk allowlist (or explicitly asserts “all hunks”),
- and cites any approval ticket required by policy for the apply attempt.

### 3.7 Apply execution, undo checkpoint, and audit

- Apply engine receives the approval record and MUST:
  - capture an undo/checkpoint boundary before commit,
  - apply only the approved hunk set,
  - mint a mutation group (`group_kind = ai_patch`) in the mutation journal,
  - and mint a `patch_apply_audit_record` that proves reviewed → approved → applied lineage.

## 4. Degraded modes (no bypass)

### 4.1 Provider failure

If the provider fails to return a plan or patch:

- a review packet may still be minted as “proposal failed”, but it MUST NOT include a patch digest/hunk inventory and MUST NOT permit apply.
- the user may rerun under current policy or narrow scope; both paths mint a fresh review summary id (no silent “same packet, different patch” reuse).

### 4.2 Tool / validation failure

If validation tools are unavailable, denied, or fail:

- the review summary must show validation as skipped/failed with typed cause,
- apply is still permitted only if policy allows (validation is evidence, not authority),
- apply audit must record the validation posture used (e.g. “not run”, “failed”, “partial”).

## 5. Fixture-backed conformance

Conformance for this packet is demonstrated by the cases under:

- `fixtures/ai/multifile_patch_cases/`

Each case is non-conforming unless it demonstrates:

- a content-addressed proposed patch with a stable hunk inventory,
- a validation summary bound to that patch digest (or a typed skip/failure),
- an approval record that enumerates approved scope,
- an apply audit record that proves the applied hunk set is a subset of the approved set, and
- mutation journal + checkpoint refs sufficient to support undo/restore claims.

