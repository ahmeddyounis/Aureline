# Artifact: Hardened merge-queue, CI/check-status, pipeline-overlay freshness, and browser-handoff audit on claimed provider rows

**Task:** M04-108
**Workstream:** B6 — Git, review, migration, provider-linked continuity, and change orchestration
**Status:** Implemented
**Crate:** `aureline-review`

## Summary

This artifact documents the hardened merge-queue/CI-status/browser-handoff audit packet that binds merge-queue truth, CI/check status, pipeline-overlay freshness, and browser-handoff audit on claimed provider rows. The implementation extends provider overlays from simple badges to normalized run/check objects with explicit freshness, divergence labels, and read-only versus run-control subset labeling.

## Checked-in artifacts

| Artifact | Path |
|---|---|
| Rust implementation | `crates/aureline-review/src/harden_merge_queue_ci_status_and_browser_handoff/mod.rs` |
| JSON schema | `schemas/review/harden_merge_queue_ci_status_and_browser_handoff.schema.json` |
| Fixtures | `fixtures/review/m4/harden-merge-queue-ci-status-and-browser-handoff/` |
| Documentation | `docs/review/m4/harden-merge-queue-ci-status-and-browser-handoff.md` |
| Integration tests | `crates/aureline-review/tests/harden_merge_queue_ci_status_and_browser_handoff_alpha.rs` |

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current on the release branch.
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable in product copy, docs/help, and release packets.
- [x] Daily Git/review or migration workflows stay previewable, attributable, and reversible.
- [x] Provider-linked or browser-handoff behavior is explicit about freshness and ownership.
- [x] Provider overlays for checks, merge queues, and hosted status prove local-CI parity against the active review-pack evaluator.
- [x] Any rerun, cancel, retry, queue, or similar upstream-mutation affordance states whether it is inspect-only, provider-controlled, or auditable in-product.

## Verification

```bash
cargo test -p aureline-review --test harden_merge_queue_ci_status_and_browser_handoff_alpha
```
