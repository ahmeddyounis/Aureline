# Prompt Composer Beta

The prompt composer is a typed control surface, not a plain chat box. Before a
turn leaves the composer, the UI must expose mode, scope, execution boundary,
mentions, attachments, slash-command identity, budget pressure, draft retention,
and evidence lineage from the same runtime packet.

## Required Rows

| Row | Required truth |
| --- | --- |
| Intent | One of `ask`, `explain`, `plan`, `draft_patch`, `review_diff`, `generate_tests`, or `run_tool_with_approval`, plus current scope and execution boundary. |
| Mentions | `@file`, `@symbol`, `@root`, `@run`, and object references resolve to stable ids. Ambiguity is shown before send with candidate refs. |
| Attachments | Every pill has stable object identity, source class, freshness, trust, preview, open, remove, and keyboard reachability. |
| Slash commands | Rows reuse canonical command ids, capability classes, approval posture, result schema refs, help anchors, and disabled reasons. |
| Budget | Overflow shows what is omitted, summarized, trimmed, blocked, or route-switched. Raw tokenizer trivia is not enough. |
| Draft state | Drafts are local-first by default. Managed-policy or collaboration changes to retention/sync are visible in the row. |
| Evidence | Inline stub, operator, support, and compliance packet classes preserve composer, route, spend, redaction, and replay refs. |

## Failure Handling

Stale attachments, unresolved mentions, budget overflow, policy-blocked routes,
and offline local-only degradation must keep the current draft intact. Each
state needs a visible safe fallback such as manual edit/search, local-only
review, CLI/headless continuation, or deferred review/export.

Preview-only branch or worktree rows must show cumulative budget posture and
route receipts. They must not become autonomous apply, merge, or push controls.

## Export

The runtime owner is `aureline_ai::prompt_composer`. The export-safe packet is
validated by `PromptComposerConformancePacket::validate` and is published at:

- `schemas/ai/prompt_composer_draft.schema.json`
- `schemas/ai/prompt_context_attachment.schema.json`
- `fixtures/ai/m3/prompt_composer_drills/`
- `artifacts/ai/m3/prompt_composer_conformance/support_export.json`
