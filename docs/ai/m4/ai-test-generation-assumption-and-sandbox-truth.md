# AI test-generation assumption and sandbox truth

This stable lane keeps AI-generated tests review-first. The runtime owner is
`aureline_ai::ai_test_generation`.

An AI-generated test proposal is not trusted coverage proof. It is a draft
candidate that must explain why it exists, what it assumes, what type of diff it
would create, what actually ran in a sandbox, and whether its coverage impact is
measured, estimated, stale, unsupported, or not comparable.

## Contract

The packet references `schemas/testing/ai_test_generation_gate.schema.json` for
selector/protected-path admission and adds the stable review/export contract:

- **Test-generation brief** - cites a concrete trigger such as a bug report,
  uncovered branch, failing example, changed symbol, or release-facing
  regression gap. The brief carries target refs, requested test type,
  framework target, confidence, flaky-risk labels, and export lineage.
- **Assumption review sheet** - lists fixture creation, mocks, clock/randomness,
  environment variables, file-system and network expectations, runtime
  dependencies, and unsupported paths when they affect trust.
- **Generated-test diff record** - separates logic assertions,
  helper/fixture additions, and snapshot/golden updates so high-risk changes do
  not hide inside one flat patch.
- **Sandbox validation record** - binds the exact target/environment lineage,
  network/file/secret policy, pass/fail/timeout/skipped/blocked outcome,
  trust-blocker reason, rerun action, and log action.
- **Coverage-impact note** - states measured, estimated, stale, unsupported, or
  not-comparable impact by target family. Candidate impact does not count toward
  release or benchmark truth while it remains AI-generated draft material.
- **Consumer projection** - proves suggestion cards, test explorer overlays,
  coverage views, CLI/headless output, support exports, and release packets
  preserve draft state, AI-generated source, diff classes, sandbox state, and
  coverage-impact vocabulary.

## Required behavior

`AiTestGenerationTruthPacket::validate` rejects a packet when:

- a candidate is promoted out of draft/review-only posture;
- the current client cannot inspect assumptions, diff classes, or sandbox state;
- bulk apply is available;
- a brief lacks concrete trigger evidence;
- low-confidence or flaky-risk labels are dropped after a sandbox pass;
- assumption rows are missing, hidden, or omit the unsupported-path note;
- generated-test diff classes are flattened or snapshot/golden rows lack a
  baseline ref;
- sandbox validation lacks target/environment lineage, network/file/secret
  policy, logs, rerun/open-log actions, or a trust-blocker reason;
- measured coverage lacks a measured coverage ref and delta summary;
- candidate coverage is counted as release or benchmark truth; or
- any support/export boundary carries raw generated source, raw diffs, raw
  prompts, raw logs, provider payloads, credentials, URLs, absolute local paths,
  or secrets.

## Truth source

The checked artifact at
`artifacts/ai/m4/ai-test-generation-assumption-and-sandbox-truth/support_export.json`
is canonical for this lane. Dashboards, Help/About cards, test explorer
overlays, CLI/headless exports, release packets, and support bundles should
ingest it directly. The boundary schema is
`schemas/ai/ai-test-generation-assumption-and-sandbox-truth.schema.json`; the
protected fixture is
`fixtures/ai/m4/ai-test-generation-assumption-and-sandbox-truth/`.

Verify the checked packet with:

```sh
cargo test -p aureline-ai ai_test_generation
```
