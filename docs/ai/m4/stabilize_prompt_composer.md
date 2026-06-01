# Stabilize the AI prompt composer

This stable lane promotes the beta prompt-composer conformance packet to the
stable line. The runtime owner is `aureline_ai::stabilize_prompt_composer`.

The composer is a typed control surface, not a chat box. Before any model
execution begins, a claimed stable composer must be able to explain what the
operator asked, which exact objects are attached, what will be omitted or
summarized, and which route or approval class the send action will use.

## Contract

The beta `aureline_ai::prompt_composer::PromptComposerConformancePacket` remains
canonical for intent mode, mention resolution, attachment identity, slash-command
parity, budget pressure, draft retention, edge-case handling, and evidence
lineage. The stable packet does **not** clone that truth. It references the
conformance packet by id and adds the review-pass deltas the stable line
requires:

- **Thread header** — current scope, provider/model, retention mode, and
  memory access (save / delete / export). Any Remember/Save affordance previews
  what will be retained, where it will live, and who can reuse it.
- **Typed attachment semantics** — every attached object (files, symbols, docs
  references, diagnostics, tests, terminal/tool outputs, external text) shows
  origin, trust or taint class, freshness, and current inclusion posture before
  send.
- **Pinned but stale** — pinned context that changed on disk, in Git, or in
  live state reads as `pinned_but_stale` with a drift source and blocks silent
  reuse until the operator refreshes or removes it.
- **Omitted-context review** — omitted sources remain inspectable after send so
  replay, support, and audit flows can explain why they were excluded.
- **Forked-thread lineage** — parent thread/run id, inherited context snapshot,
  and divergence point for any forked thread.
- **Compare-answer truth** — each comparison records whether the answers reused
  the same context snapshot or differ because of hidden context drift, plus any
  provider/model or instruction-stack delta.
- **Context-drift banners** — when changed files, docs, tests, or live objects
  materially alter a previously reviewed composer state, a banner requires
  re-review so rerun/resend never implies the earlier snapshot still applies.
- **Surface consistency** — attachment pills, mention rows, omitted-context
  review, forked-thread comparison, and drift banners stay keyboard reachable
  and screen-reader describable across editor-attached, sidebar, and detached
  composers.

## Required behavior

`PromptComposerStabilizationPacket::validate` rejects a packet when:

- the promoted conformance packet is invalid or its session, draft, or context
  snapshot refs do not match;
- a typed source class (file, symbol, docs reference, diagnostic, test, terminal
  or tool output, external text) is not covered, or an attachment row lacks
  origin, trust, freshness, posture, keyboard reach, or screen-reader narration;
- a pinned object that drifted underneath does not read as `pinned_but_stale`
  with a drift source and a send block;
- an omitted source is not inspectable after send or cannot be explained by
  replay/support/audit;
- a forked thread lacks parent, inherited snapshot, or divergence point;
- a drifted comparison fails to flag hidden context drift, or a same-context
  comparison claims drift;
- a drift banner does not require re-review;
- a Remember/Save affordance that shares beyond the device does not name a reuse
  audience, locus, and retained summary;
- any of the editor-attached, sidebar, or detached surfaces is missing or not
  fully keyboard/screen-reader reachable.

## Boundary

The packet is export-safe. It carries refs, state tokens, coarse classes, and
review labels only. Raw prompt text, source file bodies, provider payloads,
credentials, endpoint URLs, raw token counts, exact prices, and billing-account
ids stay outside the support boundary.

## Truth source

The checked artifact at
`artifacts/ai/m4/prompt_composer_stabilization/support_export.json` is canonical
for this lane. Dashboards, docs, Help/About surfaces, and support exports should
ingest it instead of cloning status text. The boundary schema is
`schemas/ai/prompt_composer_stabilization.schema.json`; the protected fixture is
`fixtures/ai/m4/prompt_composer_stabilization/`.

Verify the checked packet with:

```sh
cargo test -p aureline-ai stabilize_prompt_composer
```
