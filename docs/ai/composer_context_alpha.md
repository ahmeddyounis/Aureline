# AI composer context alpha

This document is the reviewer-facing contract for the alpha prompt composer
and AI context inspector. The implementation is anchored in
`aureline_ai::context_inspector` and consumed by the shell context-inspector
projection.

## Owned artifacts

| Artifact | Role |
| --- | --- |
| `crates/aureline-ai/src/context_inspector/mod.rs` | Canonical pre-send snapshot, grouped context rows, docs/source-language truth, budget strip, review lock, validation, and evidence handoff rows. |
| `crates/aureline-shell/src/ai_context_inspector/mod.rs` | First shell projection that renders alpha snapshot rows without owning a second context model. |
| `fixtures/ai/composer_context_alpha/` | Protected cases for pinned docs citations, source-language fallback, mention ambiguity, blocked context, tainted context, and budget overflow. |

## Contract

The alpha snapshot must answer these questions before any material AI result
is generated:

- Which intent mode is active: ask, explain, plan, draft patch, review diff,
  generate tests, or run tool with approval.
- Which typed attachments and mentions will be used, and whether each resolves
  to stable workspace, docs, or runtime identity.
- Which context groups are included, pinned, omitted, blocked, stale, tainted,
  or summarized.
- Which provider/model route, quota state, and cost envelope are active, read
  from the routing packet rather than restated.
- Which docs or knowledge attachments have docs-node identity, source class,
  version/revision, locality, exact-anchor availability, and source-language
  fallback.

## Evidence handoff

`ComposerContextAlphaSnapshot::evidence_handoff()` emits
`AiContextEvidenceHandoff` rows for context that must survive into downstream
evidence: pinned, omitted, blocked, stale, tainted, docs-backed, and
attachment/mention-linked rows. The mutation evidence packet ingests that
handoff optionally and validates that the composer session, draft, and request
workspace match the packet before exposing the rows in support export.

## Protected cases

- `pinned_docs_citation_fallback.json`: proves pinned docs context keeps
  docs-node identity, exact anchor availability, version truth, mirrored-pack
  locality, and source-language fallback before send and in evidence handoff.
- `ambiguity_blocked_budget_overflow.json`: proves ambiguous mentions,
  blocked context, omitted context, tainted context, and overflow budget state
  remain visible before send and survive the handoff.

## Verification

```sh
cargo test -p aureline-ai --no-fail-fast
cargo test -p aureline-shell --lib ai_context_inspector --no-fail-fast
```
