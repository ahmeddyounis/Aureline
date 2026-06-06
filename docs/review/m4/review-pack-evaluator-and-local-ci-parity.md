# Stable review-pack evaluator, ownership classes, parity, and replay packets

**Scope:** Stable review-pack result model for review workspaces, hosted overlays, local CI, AI review, browser companion follow-up, CLI/headless replay, and support export.

**Status:** Implemented in `aureline-review`.

## Goal

The stable review-pack evaluator turns repo-defined review policy into one replayable contract. Every claimed review row carries the same review-pack version, digest, base/head identity, scope selector set, required-check vocabulary, ownership signal classes, AI review finding state, publish preview intent, divergence labels, and replay/export refs.

The evaluator is declarative and arbitrary-code-free. It validates already-normalized truth from local review, CI, AI review, hosted provider overlays, and browser companion follow-up; it does not run commands or mutate branches.

## Record Families

| Record | Purpose |
|---|---|
| `review_pack_stable_evaluation_packet` | Top-level packet consumed by review surfaces and support export. |
| `review_pack_stable_evaluation_record` | Normalized result with pack identity, scope, checks, ownership, AI findings, publish previews, and export posture. |
| `review_pack_replay_export_packet` | Headless replay packet preserving version, digest, base/head, check names, ownership classes, divergence labels, and AI finding refs. |
| `review_pack_evaluation_inspection_record` | Boolean projection for stable-row gates and fixture tests. |

## Stable Invariants

- All required surfaces are present: local review workspace, local CI run, hosted provider overlay, AI review, browser companion follow-up, and CLI/headless support.
- Full parity is rejected when any surface is stale, partial, digest-mismatched, unsupported, unavailable, or not evaluated.
- Review-pack digest, base identity, and head identity are carried by every surface observation and by replay export.
- Required-check names are preserved in order by the replay packet.
- Ownership signals remain split between `advisory_owner`, `enforced_owner`, and `provider_authoritative_owner`.
- AI findings use one lifecycle vocabulary: `open`, `dismissed`, `published`, `outdated`, `suppressed`, and `rerun_recommended`.
- Material diff changes force prior AI findings to `outdated` or `rerun_recommended`.
- Missing provider write access must expose local copy/export fallback and cannot claim hidden publish.
- Raw paths, glob bodies, command lines, and check outputs stay outside support export.

## Files

| Artifact | Path |
|---|---|
| Rust implementation | `crates/aureline-review/src/review_pack_evaluator_and_local_ci_parity/mod.rs` |
| Boundary schema | `schemas/review/review-pack.schema.json` |
| Fixtures | `fixtures/review/m4/review-pack-evaluator-and-local-ci-parity/` |
| Tests | `crates/aureline-review/tests/review_pack_evaluator_and_local_ci_parity.rs` |
| Evidence packet | `artifacts/review/m4/review-pack-evaluator-and-local-ci-parity.md` |

## Verification

```bash
cargo test -p aureline-review --test review_pack_evaluator_and_local_ci_parity
```
