# AI Review Assist And Publish Truth

This document freezes the stable metadata contract for AI review-assist findings, scope selectors, publish-to-review sheets, and resolution memory. The Rust owner is `aureline_ai::ai_review_assist::AiReviewAssistTruthPacket`; the JSON boundary is `schemas/ai/ai-review-assist-and-publish-truth.schema.json`.

## Contract

AI review assist is evidence, not review authority. A finding can summarize or suggest, but it cannot approve, request changes, merge, or hide whether it remains local analysis, copied/exported text, or a provider-published comment/check annotation.

Every packet binds:

- `ReviewScopeSelector` rows for `selected_diff`, `uncommitted_changes`, and `hosted_review_object`, each with base/head identity, review-pack digest, freshness, rerun action, and material-diff-change refs when stale.
- `AiReviewFindingRow` rows with durable finding ids, affected file/hunk refs, repo instruction or check source, review-pack digest, evidence-packet lineage, scope freshness, and current resolution state.
- `PublishToReviewSheet` rows with provider/local/export destination, destination ref, provider write posture, exact redaction-aware outbound text preview, attribution state, redaction note, action class, and fallback actions.
- `ResolutionMemoryRow` rows with `open`, `dismissed`, `published`, `outdated`, or `suppressed` state, actor/source/timestamp, reopen lineage, and publish-sheet linkage when published.
- `AiReviewConsumerProjection` rows for desktop review workspace, CLI/headless, browser companion, and support export.

## Required Behavior

- Material diff changes mark prior findings `outdated` or force rerun before provider publication.
- Missing provider write access is represented as `missing_provider_write_access` and downgrades to `copy_only`, `export_local_packet`, or `keep_local`; it is not a dead-end error.
- Publish sheets preview the exact destination class and redaction-aware outbound text before any provider write.
- Low-confidence and outdated findings cannot publish to a provider destination.
- All consumer surfaces preserve analyzed scope, instruction/check source, outbound destination truth, resolution memory, and local/copy/export fallback.

## Canonical Artifacts

- Schema: `schemas/ai/ai-review-assist-and-publish-truth.schema.json`
- Fixture: `fixtures/ai/m4/ai-review-assist-and-publish-truth/truth_packet.json`
- Support export: `artifacts/ai/m4/ai-review-assist-and-publish-truth/support_export.json`
- Summary: `artifacts/ai/m4/ai-review-assist-and-publish-truth/summary.md`
