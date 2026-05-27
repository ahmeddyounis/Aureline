# Stabilized review-side AI evidence attachment and safe suggestion apply without broadening authority

**Scope:** Stabilize review-side AI evidence attachment and safe suggestion apply for daily-driver review lanes.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Bind AI-generated review evidence and safe suggestion apply operations into a single coherent packet. Every AI evidence attachment carries explicit source, freshness, actor, and return path. Safe suggestion apply remains previewable, checkpointed, and reversible — never broadening the user's authority beyond what the local workspace and review context already permit.

## Design principles

1. **Explicit AI evidence source** — Every evidence attachment carries an `evidence_source_class` from the closed vocabulary (`ai_model_local`, `ai_model_provider`, `ai_model_hybrid`, `human_curated_ai`). AI evidence never masquerades as human review or hosted authority.
2. **Safe suggestion apply without authority broadening** — Suggestions remain advisory until explicitly previewed and checkpointed. A suggestion that would broaden the user's scope or authority is blocked and flagged explicitly.
3. **Checkpointed reversibility** — Every applied suggestion creates a checkpoint preserving base/head revision and worktree state. Reversion is always possible while the checkpoint remains recoverable.
4. **Previewable, attributable, reversible** — All mutation commands support preview/dry-run. Actor identity and target row are preserved. Return paths for detachment and reversal are explicit.
5. **Redaction-safe support export** — Raw URLs and raw provider payloads are explicitly forbidden from crossing the support boundary.
6. **Inspection projection** — Compact boolean projection surfaces every truth axis (evidence state, suggestion state, checkpoint state, authority broadening) for CLI and inspector surfaces.

## Record kinds

| Record kind | Purpose |
|---|---|
| `ai_review_evidence_packet` | Top-level packet consumed by review surfaces and support exports. |
| `ai_review_evidence_record` | Core evidence packet binding workspace, attachments, and suggestion apply rows. |
| `ai_evidence_attachment_record` | One AI evidence item bound to a review row with explicit source, freshness, actor, and return path. |
| `safe_suggestion_apply_record` | Safe suggestion apply operation with explicit authority class, preview state, and reversibility. |
| `suggestion_apply_checkpoint_record` | Checkpoint before/after apply preserving exact base/head identity and worktree state. |
| `ai_evidence_command_record` | Command-graph operations (preview, apply, revert, detach, refresh, export). |
| `ai_evidence_support_export_packet` | Redaction-safe export with reopen context. |
| `ai_evidence_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### AI evidence states
- `attached_current`, `attached_stale`, `detached_invalidated`, `pending_verification`

### AI evidence source classes
- `ai_model_local`, `ai_model_provider`, `ai_model_hybrid`, `human_curated_ai`

### Suggestion apply states
- `preview_ready`, `applied_with_checkpoint`, `reverted`, `blocked_pending_review`, `blocked_scope_exceeded`

### Suggestion authority classes
- `advisory_only`, `previewable_local_apply`, `checkpointed_reversible`, `blocked_requires_human_approval`

### Checkpoint states
- `checkpoint_created`, `checkpoint_applied`, `checkpoint_reverted`, `checkpoint_failed`

### Command classes
- `preview_suggestion`, `apply_suggestion`, `revert_suggestion`, `detach_evidence`, `refresh_evidence`, `export_evidence`

## Key invariants

- `evidence_state` of `attached_current` requires every attachment to have `freshness_class` of `current`.
- A suggestion with `would_broaden_authority` set to `true` may not have `apply_state` of `applied_with_checkpoint`.
- A suggestion with `apply_state` of `applied_with_checkpoint` must have a non-null `checkpoint_ref`.
- All `raw_*_export_allowed` flags in support export must be `false`.
- Evidence attachments must cite a known `source_evidence_ref` from the same packet.
- Checkpoint dependency order must be valid: every `checkpoint_ref` on a suggestion must exist in the `checkpoints` array.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-review/src/stabilize_review_side_ai_evidence_attachment_and_safe/mod.rs` |
| Schema | `schemas/review/ai_review_evidence.schema.json` |
| Fixtures | `fixtures/review/m4/stabilize-review-side-ai-evidence-attachment-and-safe/` |
| Tests | `crates/aureline-review/tests/stabilize_review_side_ai_evidence_attachment_and_safe_alpha.rs` |

## Integration with existing lanes

- Consumes [`ReviewWorkspaceBetaPacket`] from the `workspace` module.
- Projects into the same inspector/CLI/support-export surfaces as the `landing` and `stabilize_review_workspace_anchors_stale_base_labels_approval` modules.
- Coexists with provider-linked stabilization; AI evidence does not inherit green from provider rows.

## Verification

```bash
cargo test -p aureline-review --test stabilize_review_side_ai_evidence_attachment_and_safe_alpha
```
