# AI Review Assist And Publish Truth

Stable AI review assist now has one typed truth packet:

- `AiReviewAssistTruthPacket` in `aureline_ai::ai_review_assist`
- `schemas/ai/ai-review-assist-and-publish-truth.schema.json`
- `artifacts/ai/m4/ai-review-assist-and-publish-truth/support_export.json`

The packet covers selected diff, uncommitted changes, and hosted review scopes; preserves review-pack digest and evidence-packet lineage; previews provider/local/export publication; records missing provider write access as local/copy/export fallback; and stores resolution memory for open, dismissed, published, outdated, and suppressed findings.
