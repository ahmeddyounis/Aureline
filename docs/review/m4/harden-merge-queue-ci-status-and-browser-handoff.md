# Hardened merge-queue, CI/check-status, pipeline-overlay freshness, and browser-handoff audit on claimed provider rows

**Scope:** Harden merge-queue, CI/check-status, pipeline-overlay freshness, and browser-handoff audit on claimed provider rows for daily-driver review lanes.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Bind merge-queue truth, CI/check status, pipeline-overlay freshness, and browser-handoff audit into a single coherent audit packet on every claimed stable provider row. Provider overlays are normalized run/check objects with fetched-at freshness, provider/source class, artifact-link trust class, and explicit read-only versus run-control subset labeling. Any rerun, cancel, queue, or similar upstream-mutation affordance claimed in-product uses an auditable inline review or short confirm sheet naming provider scope, actor mode, target run identity, and browser-handoff fallback. Cached provider metadata never implies control authority.

## Design principles

1. **Normalized run/check objects** — CI/check audit records carry `fetched_at`, `provider_source_class`, `freshness_class`, and explicit `divergence_label_class` when local, CI, AI-review, or provider results disagree.
2. **Explicit read-only vs run-control labeling** — Pipeline overlay audits carry `read_only_subset_label` and `run_control_subset_label`. Unqualified overlays downgrade to `subset_unqualified_downgraded` rather than inheriting adjacent green rows.
3. **Explicit mutation mode disclosure** — Run-control audits state whether the control is `inspect_only`, `provider_controlled`, or `auditable_in_product`. The review-pack digest and base/head context that scoped the action are preserved.
4. **Browser-handoff audit on provider rows** — Every claimed provider row with a browser handoff carries an explicit audit record with `provider_class`, `target_object_ref`, `actor_ref`, `scope_disclosure`, and `return_path_present`.
5. **No hidden authority** — The `hidden_authority_detected` flag prevents provider mutations from masquerading as local chrome. When detected, the audit is degraded.
6. **Separable inspectable truths** — Merge queue, CI checks, pipeline overlays, run controls, and browser handoffs are all independently auditable and inspectable.
7. **Redaction-safe support export** — Raw URLs and raw provider payloads are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `merge_queue_ci_status_browser_handoff_audit_packet` | Top-level packet consumed by review surfaces and support exports. |
| `merge_queue_ci_status_browser_handoff_audit_record` | Core audit binding workspace, landing, stabilization, boundary hardening, and provider-linked stabilization. |
| `merge_queue_audit_record` | Merge-queue entry audit with explicit provider authority and divergence detection. |
| `ci_check_audit_record` | Normalized run/check object with fetched-at freshness and divergence labels. |
| `pipeline_overlay_audit_record` | Pipeline overlay audit with read-only versus run-control subset labeling. |
| `run_control_audit_record` | Run-control audit with explicit mutation mode and preserved review-pack/base/head context. |
| `browser_handoff_audit_record` | Browser-handoff audit on claimed provider rows. |
| `audit_command_record` | Command-graph operations (preview, approve, refresh, invalidate, handoff, rerun, cancel, retry). |
| `audit_support_export_packet` | Redaction-safe export with reopen context. |
| `audit_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Audit states
- `audit_passed`, `audit_degraded_merge_queue_divergence`, `audit_degraded_ci_check_stale`, `audit_degraded_pipeline_overlay_unqualified`, `audit_degraded_browser_handoff_untyped`, `audit_degraded_hidden_authority`, `audit_failed_provider_claim_unsupported`

### Merge-queue audit states
- `merge_queue_authoritative`, `merge_queue_diverged_from_provider`, `merge_queue_stale_local_estimate`, `merge_queue_downgraded_to_inspect_only`

### CI/check freshness classes
- `check_fresh`, `check_stale_within_grace`, `check_stale_blocks_mutation`, `check_freshness_unknown`

### CI/check divergence classes
- `no_divergence`, `local_ci_disagree`, `local_provider_disagree`, `ci_provider_disagree`, `ai_review_disagrees`, `all_three_disagree`

### Pipeline-overlay subset classes
- `read_only_inspect`, `run_control_subset`, `run_control_full`, `subset_unqualified_downgraded`

### Run-control mutation modes
- `inspect_only`, `provider_controlled`, `auditable_in_product`

### Browser-handoff audit classes
- `handoff_audited_reversible`, `handoff_audited_no_return_path`, `handoff_downgraded_untyped`, `handoff_not_required`

## Key invariants

- `audit_passed` is incompatible with `any_provider_row_downgraded`.
- `audit_passed` is incompatible with `hidden_authority_detected`.
- `inspect_only` run controls must not claim `run_control_permitted`.
- Reversible browser handoffs require `handoff_audited_reversible` class.
- All `raw_*_export_allowed` flags in support export must be `false`.
- Cross-packet consistency: workspace, landing, stabilization, boundary hardening, and provider-linked stabilization must share the same `review_workspace_id`.
- Run-control `review_pack_digest_ref` must match stabilization `review_pack_digest_ref`.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-review/src/harden_merge_queue_ci_status_and_browser_handoff/mod.rs` |
| Schema | `schemas/review/harden_merge_queue_ci_status_and_browser_handoff.schema.json` |
| Fixtures | `fixtures/review/m4/harden-merge-queue-ci-status-and-browser-handoff/` |
| Tests | `crates/aureline-review/tests/harden_merge_queue_ci_status_and_browser_handoff_alpha.rs` |

## Integration with existing lanes

- Consumes [`LandingCandidatePacket`] from the `landing` module.
- Consumes [`ReviewStabilizationPacket`] from the `stabilize_review_workspace_anchors_stale_base_labels_approval` module.
- Consumes [`ReviewBoundaryHardeningPacket`] from the `harden_browser_handoff_and_in_product_review_boundaries` module.
- Consumes [`ProviderLinkedReviewStabilizationPacket`] from the `stabilize_provider_linked_object_models_snapshot_freshness_and` module.
- Optionally consumes [`ReviewPackParityHarnessRecord`] from the `review_pack_parity_harness` module.
- Projects into the same inspector/CLI/support-export surfaces as the `landing` and `stabilize_review_workspace_anchors_stale_base_labels_approval` modules.

## Verification

```bash
cargo test -p aureline-review --test harden_merge_queue_ci_status_and_browser_handoff_alpha
```
