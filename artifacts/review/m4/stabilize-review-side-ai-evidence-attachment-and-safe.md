# Artifact: Stabilized review-side AI evidence attachment and safe suggestion apply without broadening authority

**Task:** Stabilize review-side AI evidence attachment and safe suggestion apply for daily-driver review lanes.
**Status:** Implemented
**Verification class:** Automated functional + Conformance

## Summary

The AI review evidence lane binds AI-generated review evidence and safe suggestion apply operations into a single coherent packet. Every AI evidence attachment carries explicit source, freshness, actor, and return path. Safe suggestion apply remains previewable, checkpointed, and reversible — never broadening the user's authority beyond what the local workspace and review context already permit.

## What changed

- New Rust module: `crates/aureline-review/src/stabilize_review_side_ai_evidence_attachment_and_safe/mod.rs`
- New fixtures: `fixtures/review/m4/stabilize-review-side-ai-evidence-attachment-and-safe/`
  - `attached_current_with_preview.json`
  - `applied_with_checkpoint.json`
  - `blocked_authority_broadening.json`
- New tests: `crates/aureline-review/tests/stabilize_review_side_ai_evidence_attachment_and_safe_alpha.rs`
- New docs: `docs/review/m4/stabilize-review-side-ai-evidence-attachment-and-safe.md`
- New schema: `schemas/review/ai_review_evidence.schema.json`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet for AI review evidence are current and referenced by the stable proof index.
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable in product copy, docs/help, and release packets.
- [x] Daily Git/review or migration workflows stay previewable, attributable, and reversible.
- [x] Provider-linked or browser-handoff behavior is explicit about freshness and ownership.
- [x] AI evidence attachments disclose exact source, model run, actor, and freshness.
- [x] Safe suggestion apply never broadens authority; blocked suggestions are explicitly flagged.
- [x] Applied suggestions carry checkpoints preserving base/head identity and worktree state.

## How to verify

```bash
cargo test -p aureline-review --test stabilize_review_side_ai_evidence_attachment_and_safe_alpha
```

## Risks / follow-ups

- The module currently consumes `ReviewWorkspaceBetaPacket` as input. When a unified review-state packet is introduced, the constructor should be updated to consume it directly.
- Suggestion apply checkpoints use opaque worktree state hashes. When the worktree patch stack crate stabilizes typed checkpoint records, these fields should be updated to consume them.
- AI evidence source classes are modeled as strings; when the AI and provider crates stabilize their enums, these should be narrowed to typed enums.
