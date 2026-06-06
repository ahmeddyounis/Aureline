# Artifact: Stable review-pack evaluator, ownership classes, parity, and replay packets

**Status:** Implemented

## Summary

This artifact records the stable review-pack evaluator and replay packet contract. The implementation validates one normalized result model shared by review workspaces, hosted provider overlays, local CI, AI review, browser companion follow-up, CLI/headless replay, and support export.

## Checked-In Outputs

| Output | Path |
|---|---|
| Rust module | `crates/aureline-review/src/review_pack_evaluator_and_local_ci_parity/mod.rs` |
| Public exports | `crates/aureline-review/src/lib.rs` |
| JSON schema | `schemas/review/review-pack.schema.json` |
| Fixtures | `fixtures/review/m4/review-pack-evaluator-and-local-ci-parity/` |
| Integration test | `crates/aureline-review/tests/review_pack_evaluator_and_local_ci_parity.rs` |
| Reviewer doc | `docs/review/m4/review-pack-evaluator-and-local-ci-parity.md` |

## Acceptance Evidence

- Stable evaluation packets bind review-pack version, digest, base/head identity, scope selectors, required-check names, ownership classes, AI findings, publish previews, divergence labels, and support export refs.
- Every required surface must state pack/base/head truth before the row can claim parity.
- Digest mismatch, stale AI findings, missing provider write access, partial browser companion scope, provider outage, and unsupported evaluator capability all downgrade explicitly.
- Advisory ownership and enforced/provider-authoritative ownership remain separate classes in both evaluation and replay export.
- Replay packets preserve required-check vocabulary, ownership classes, divergence labels, AI finding refs, and headless support export refs.

## Verification

```bash
cargo test -p aureline-review --test review_pack_evaluator_and_local_ci_parity
```
