# Stabilized review workspace anchors, stale-base labels, approval invalidation, and mergeability truth

**Scope:** Stabilize review workspace anchors, stale-base labels, approval invalidation, and mergeability truth for daily-driver review lanes.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Bind review-workspace anchors, landing-candidate truths, review-pack digest, base/head identity, required-check vocabulary, and ownership signals into a single coherent stabilization packet. Every truth axis remains separable and inspectable; stale-pack, partial-scope, and slice-omitted states may not inherit green from adjacent provider rows.

## Design principles

1. **Exact review-pack digest binding** — Every anchor stability record carries the exact `review_pack_digest_ref`, `base_revision_ref`, `head_revision_ref`, and `required_check_ids` that produced the current result. Stale-pack states do not silently inherit exact binding semantics.
2. **Separable inspectable truths** — Stabilization state, anchor stability, stale-base label, approval invalidation, mergeability truth, and ownership signals are all independent fields. No single "status" column hides the underlying truth.
3. **Ownership signal split** — Advisory/graph-derived signals and enforceable/CODEOWNERS-or-provider-policy signals are modeled as separate records. Reviewer hints, risk badges, and merge warnings never masquerade as branch protection or hosted policy truth.
4. **Replayable approval invalidation** — When approval is invalidated, the record carries `replay_evidence_refs` and `replay_evidence_classes` (local-CI, AI-review, human-review, provider-check) so the same review truth can be reconstructed outside the GUI.
5. **Review bundle export/import and offline handoff** — Bundle export and offline handoff preserve `review_pack_version`, `divergence_labels`, and replayable evidence so support, bots, and enterprises can reopen the same review truth.
6. **Redaction-safe support export** — Raw URLs and raw provider payloads are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `review_stabilization_packet` | Top-level packet consumed by review surfaces and support exports. |
| `review_stabilization_record` | Core stabilization binding workspace, landing candidate, review pack, and ownership. |
| `review_anchor_stability_record` | Durable anchor bound to exact review-pack digest, base/head identity, and required-check vocabulary. |
| `review_stale_base_label_record` | Explicit stale-base label with divergence class and exact base revision identity. |
| `review_approval_invalidation_record` | Approval invalidation with triggering cause and replayable evidence. |
| `review_mergeability_truth_record` | Mergeability truth bound to required-check vocabulary and freshness state. |
| `review_ownership_signal_record` | Ownership split between advisory/graph-derived and enforceable/CODEOWNERS-or-provider-policy classes. |
| `review_bundle_export_record` | Offline review bundle preserving version, divergence labels, and replayable evidence. |
| `review_bundle_import_record` | Review bundle import with version-mismatch rejection. |
| `review_offline_handoff_record` | Offline handoff with explicit freshness, ownership, and return path. |
| `review_stabilization_command_record` | Command-graph operations (preview, approve, refresh, export, handoff, replay, publish). |
| `review_stabilization_support_export_packet` | Redaction-safe export with reopen context. |
| `review_stabilization_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Stabilization states
- `stabilized_current`, `stabilized_stale_pack`, `stabilized_partial_scope`, `stabilized_slice_omitted`, `stabilized_diverged_requires_review`

### Anchor stability classes
- `anchor_bound_exact`, `anchor_bound_stale_pack`, `anchor_bound_partial_scope`, `anchor_drifted_requires_review`

### Stale-base label classes
- `base_current`, `base_stale_within_grace`, `base_stale_blocks_landing`, `base_diverged_requires_rebase`, `base_diverged_requires_merge`

### Approval invalidation triggers
- `head_changed_after_approval`, `base_changed_after_approval`, `review_pack_changed_after_approval`, `check_failure_after_approval`, `manual_invalidation_by_reviewer`

### Mergeability truth classes
- `mergeable`, `not_mergeable_blocking`, `mergeable_pending_eligibility`, `mergeability_unknown_requires_review`

### Ownership signal classes
- `advisory_graph_derived`, `enforceable_codeowners_policy`, `enforceable_provider_policy`, `advisory_and_enforceable_match`, `advisory_and_enforceable_conflict`

### Replay evidence classes
- `local_ci_evidence`, `ai_review_evidence`, `human_review_evidence`, `provider_check_evidence`

### Divergence labels
- `no_divergence`, `local_ahead`, `remote_ahead`, `diverged_requires_rebase`, `diverged_requires_merge`

## Key invariants

- `stabilization_state` of `stabilized_current` requires every anchor stability class to be `anchor_bound_exact`.
- `stabilized_stale_pack` may not claim `all_anchors_bound_exact`.
- `base_revision_ref` and `head_revision_ref` in the stabilization input must match the landing candidate.
- `review_pack_digest_ref` in the stabilization input must match the landing candidate.
- Approval invalidation records must have matching lengths for `replay_evidence_refs` and `replay_evidence_classes`.
- Ownership signals must be at least one of advisory or enforceable.
- All `raw_*_export_allowed` flags in support export must be `false`.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-review/src/stabilize_review_workspace_anchors_stale_base_labels_approval/mod.rs` |
| Schema | `schemas/review/review_stabilization.schema.json` |
| Fixtures | `fixtures/review/m4/stabilize-review-workspace-anchors-stale-base-labels-approval/` |
| Tests | `crates/aureline-review/tests/stabilize_review_workspace_anchors_stale_base_labels_approval_alpha.rs` |

## Integration with existing lanes

- Consumes [`ReviewWorkspaceBetaPacket`] from the `workspace` module.
- Consumes [`LandingCandidatePacket`] from the `landing` module.
- Projects into the same inspector/CLI/support-export surfaces as the `landing` and `harden_merge_rebase_cherry_pick_revert_and_reset` modules.

## Verification

```bash
cargo test -p aureline-review --test stabilize_review_workspace_anchors_stale_base_labels_approval_alpha
```
