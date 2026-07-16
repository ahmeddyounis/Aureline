# M5 Write-Review Sheets: Reviewed Fallback Transitions Across Flows

This lane operationalizes the B150 write-review sheet: when the current object cannot be written directly, a
reviewed sheet is shown **before commit** instead of a silent best-effort fallback. It is the write-capable
consumer of the frozen constrained-file-state matrix (`schemas/program/m5-constrained-file-state.schema.json`
and siblings) and reuses one sheet model across every originating flow that can hit a constrained object.

- **Schema:** `schemas/program/m5-write-review-sheet-fallback-paths.schema.json`
- **Support export:** `artifacts/support/m5-write-review-sheet-fallback-paths/support_export.json`
- **Matrix CSV:** `artifacts/support/m5-write-review-sheet-fallback-paths/matrix.csv`
- **Summary:** `artifacts/support/m5-write-review-sheet-fallback-paths/summary.md`
- **Fixtures:** `fixtures/editor/m5-write-review-sheet-fallback-paths/`
- **Record kind:** `m5_write_review_sheet_fallback_path_registry`

## The five reviewed fallback transitions

Every blocked write on a constrained current object routes to exactly one of these reviewed transitions rather
than a lossy direct write:

| Fallback action | Write disposition | Recovery / undo class |
| --- | --- | --- |
| `duplicate_to_editable_copy` | `read_only_blocked` | `new_copy_leaves_original_intact` |
| `detach_from_managed_source` | `detach_required` | `detach_checkpoint_restorable` |
| `create_overlay_patch` | `detach_required` | `overlay_patch_revertible` |
| `request_approval` | `approval_gated` | `approval_request_withdrawable` |
| `regenerate_with_preview` | `regenerate_only` | `regenerate_preview_discardable` |

## What every sheet shows

Each review binding carries one `review_content`: the exact write target, the side effects, the
preserved-versus-lost sync or regenerate path (retained items, lost items, and the sync / regenerate path),
required approvals, the checkpoint / undo class, an export-safe explanation, and the labels for any co-applicable
state classes. The content is identical for a profile across every flow that reviews it, so an AI apply and a
direct save that land on the same constrained object get the same reviewed transition.

## Reused across flows

One sheet model serves the `direct_save`, `code_action`, `ai_apply`, `importer`, `repair`, and `batch_edit`
flows. The AI, importer, and repair flows are actor-parity mutation paths and never get a hidden bypass around
the sheet. Each fallback action is reviewed through two or more distinct flows.

## Postures and narrowing

- `full_review_sheet` — the interactive sheet shown before commit; offers `commit_reviewed_transition`.
- `precondition_notice_compact` — a compact notice (status chip, code-action lightbulb) that routes to the full
  sheet; carries an explicit narrow note.
- `export_redacted` — an export-safe rendering in the support packet; carries a narrow note plus an
  export-detail note and points at the canonical contracts.

Narrowing changes only which actions remain; it never rewords the reviewed-transition content.

## Invariants

- **AC1** — At least one duplicate, one detach, one overlay, one request-approval, and one regenerate-first path
  is reviewable before commit with explicit retained-versus-lost behaviour.
- **AC2** — No constrained write path silently mutates the current object through a lossy fallback; the action
  set is closed and safe (no direct-write variant), and every binding is reviewed before commit.
- **AC3** — A recovery / undo class is visible before commit on every seeded fallback path.

Guardrails (each must be `false` on every binding): silent lossy fallback, hidden bypass for
AI / automation / import / repair, unstated exact write target or preserved-versus-lost sync, hidden recovery /
undo class before commit, and one state class hiding another.

Raw secret values, credentials, and private endpoints stay outside the support boundary; the packet references
upstream contracts by id.
