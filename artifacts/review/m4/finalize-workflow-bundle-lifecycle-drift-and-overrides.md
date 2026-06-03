# Release evidence: Finalize workflow-bundle lifecycle with drift, overrides, and certified truth

## Acceptance criteria status

| Criterion | Status | Evidence |
|-----------|--------|----------|
| Bundle lifecycle fixtures cover install, update, remove, drift, rebase, and mirror/offline cases | PASS | 6 fixtures under `fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/` |
| Exact visibility into changed assets, preserved overrides, and rollback checkpoints | PASS | `update_with_drift_and_override.json`, `rebase_with_dependency_change.json` |
| Support export and diagnostics can name active bundle, local drift, certification freshness, and current claim class | PASS | `BundleLifecycleInspectionRecord` exposes all fields; `support_export_safe` invariant enforced |
| Start Center, CLI/headless, docs, and release packets show the same bundle ID, signer/source, compatible range, and archetype class | PASS | `BundleLifecycleOperationRecord` binds `review_packet_id` and `certification_packet_id`; consumer surfaces list enforced |
| Bundle removal and downgrade drills prove user-owned state survives and stale certification narrows claims automatically | PASS | `remove_with_asset_provenance.json`, `downgrade_when_scorecard_stale.json` |
| Dependency markers (Preview/Beta, managed-only, org-mirrored, Labs) remain visible across all surfaces | PASS | `BundleDependencyMarkerRecord` with closed `capability_class` vocabulary |
| Scorecard linkage preserved in drift summaries and local overrides | PASS | `ScorecardLinkedDriftSummaryRecord` requires `scorecard_linkage_preserved = true` on every entry |
| Trust/egress/control-plane changes surfaced distinctly from ordinary package churn | PASS | `TrustEgressChangeDisclosureRecord` with `alters_authority_boundary` and closed `severity_class` vocabulary |

## Implementation summary

- **Rust module:** `crates/aureline-workspace/src/finalize_workflow_bundle_lifecycle_drift_and_overrides/mod.rs`
- **Schema:** `schemas/review/finalize-workflow-bundle-lifecycle-drift-and-overrides.schema.json`
- **Fixtures:** `fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/*.json`
- **Docs:** `docs/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides.md`
- **Tests:** Module-level tests in `mod.rs` covering projection, validation, removal safety, hidden boundary change rejection, scorecard linkage, and fixture round-trips.

## Verification

```bash
cd crates/aureline-workspace
cargo test finalize_workflow_bundle_lifecycle_drift_and_overrides
```

## Risks and follow-ups

1. **Integration with actual bundle review packet builder:** The current module
   validates and projects standalone finalization records. Future work should
   wire the builder to ingest live `WorkflowBundleReviewRecord` and
   `BundleArchetypeCertificationPacket` instances rather than manual input.
2. **UI surface binding:** Start Center and bundle detail pages must be updated
   to read `BundleLifecycleFinalizationRecord` verbatim instead of cloning
   status text.
3. **CLI/headless parity:** The `project_bundle_lifecycle_finalization` entry
   point should be invoked by the CLI install/update/remove commands.
4. **Support export pipeline:** Support export tooling should serialize the
   finalization record into redaction-safe packets.
