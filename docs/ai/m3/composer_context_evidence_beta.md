# AI composer context evidence beta

This beta lane promotes the prompt composer, AI context inspector, mutation
evidence packet, and spend receipt as one inspectable operator-truth chain.
The runtime owner is `aureline_ai::composer::beta`.

## Contract

`ComposerContextEvidenceBetaPacket` is built from the canonical runtime records
rather than from duplicated UI copy:

- `ComposerContextAlphaSnapshot` for pre-approval context rows.
- `AiContextRetrievalExport` for the retrieval inspector packet consumed by AI
  context.
- `AiMutationEvidencePacket` for post-run mutation, approval, rollback, and
  tool-call lineage.
- `SpendReceiptRecord` for route, cost-envelope, cost-visibility, and charge
  locus.

The packet is export-safe. It carries refs, state tokens, coarse cost classes,
and review labels only; raw prompt text, source file bodies, provider payloads,
credentials, endpoint URLs, raw token counts, exact prices, and billing-account
ids stay outside the support boundary.

## Required behavior

The context inspector must show the context state before approval or execution.
The beta packet requires coverage for `included`, `pinned`, `omitted`, `stale`,
and `trimmed`, and rows that are omitted, stale, blocked, tainted, summarized,
or trimmed must carry an omission reason.

Evidence must survive the run. A promoted packet cannot remain in
`review_pre_apply`, and post-run evidence must include approval-ticket refs,
route receipt refs, spend receipt refs, and tool-call lineage refs. Applied
mutations continue to require rollback checkpoint and mutation-journal lineage
through the evidence packet validator.

Retrieval state must be labeled. The packet only promotes a retrieval export
whose inspector packet is `promotable` and has no validation findings. Partial
or fallback retrieval remains visible through the retrieval packet ref instead
of becoming hidden composer state.

Spend truth must match route truth. The spend receipt cost-envelope and
cost-visibility tokens must match the evidence route lineage, and the spend
receipt must expose its run state and charge-locus token.

## Surfaces

The composer, context inspector, review workspace, docs/help, support export,
and CLI projections must all preserve the same context snapshot, evidence
packet, route receipt, and spend receipt refs. A projection that remints or
drops those refs is drift and blocks the beta claim.

## Export

The checked support export is:

`/artifacts/ai/m3/composer_context_evidence_beta_support_export.json`

The JSON schema is:

`/schemas/ai/composer_context_evidence_beta.schema.json`

The reviewer fixture corpus is:

`/fixtures/ai/m3/composer_context_evidence/`

## Verification

```sh
cargo test -p aureline-ai composer::beta --no-fail-fast
cargo run -q -p aureline-ai --example dump_composer_context_evidence_beta
```
