# Artifact: Stabilized review workspace anchors, stale-base labels, approval invalidation, and mergeability truth

**Task:** Stabilize review workspace anchors, stale-base labels, approval invalidation, and mergeability truth for daily-driver review lanes.
**Status:** Implemented
**Verification class:** Automated functional + Conformance

## Summary

The review-stabilization lane binds workspace anchors, landing-candidate truths, review-pack digest, base/head identity, required-check vocabulary, and ownership signals into a single coherent packet. Stale-pack, partial-scope, and slice-omitted states are explicitly flagged and do not inherit green from adjacent provider rows. Ownership signals remain split between advisory and enforceable classes. Approval invalidation carries replayable evidence. Bundle export and offline handoff preserve review-pack version, divergence labels, and replayable evidence.

## What changed

- New Rust module: `crates/aureline-review/src/stabilize_review_workspace_anchors_stale_base_labels_approval/mod.rs`
- New fixtures: `fixtures/review/m4/stabilize-review-workspace-anchors-stale-base-labels-approval/`
  - `stabilized_current.json`
  - `stabilized_stale_pack.json`
  - `approval_invalidated_with_replay_evidence.json`
  - `ownership_conflict_with_offline_handoff.json`
- New tests: `crates/aureline-review/tests/stabilize_review_workspace_anchors_stale_base_labels_approval_alpha.rs`
- New docs: `docs/review/m4/stabilize-review-workspace-anchors-stale-base-labels-approval.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet for review stabilization are current and referenced by the stable proof index.
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable in product copy, docs/help, and release packets.
- [x] Daily Git/review or migration workflows stay previewable, attributable, and reversible.
- [x] Provider-linked or browser-handoff behavior is explicit about freshness and ownership.
- [x] Anchor stability records bind to the exact review-pack digest, base/head identity, and required-check vocabulary.
- [x] Ownership signals remain split between advisory/graph-derived and enforceable/CODEOWNERS-or-provider-policy classes.
- [x] Review bundle export/import and offline handoff preserve review-pack version, divergence labels, and replayable local-CI/AI-review evidence.

## How to verify

```bash
cargo test -p aureline-review --test stabilize_review_workspace_anchors_stale_base_labels_approval_alpha
```

## Risks / follow-ups

- The module currently consumes `LandingCandidatePacket` and `ReviewWorkspaceBetaPacket` as separate inputs. When a unified review-state packet is introduced, the constructor should be updated to consume it directly.
- Provider publish postures and queue authority classes are modeled as strings; when the provider and landing crates stabilize their enums, these should be narrowed to typed enums.
- Bundle export/import currently use opaque evidence refs. When the local-CI and AI-review crates provide strongly-typed evidence records, the replay evidence fields should be updated to consume them.
