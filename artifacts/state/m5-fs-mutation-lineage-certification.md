# M5 Filesystem and Mutation-Lineage Certification

- Packet id: `state.m5_fs_mutation_lineage_certification.v1`
- Checked artifact: `artifacts/state/m5-fs-mutation-lineage-certification.json`
- Schema: `schemas/state/m5-fs-mutation-lineage-certification.schema.json`
- Contract doc: `docs/state/m5-fs-mutation-lineage-certification.md`

## State distribution

- `qualified`: `notebook_document`, `request_workspace_document`
- `limited`: `request_response_snapshot`, `imported_archive_capture`
- `stale`: `notebook_output_artifact`, `database_export_artifact`, `profiler_trace_artifact`, `preview_output_artifact`
- `reconcile_required`: `sync_packet_artifact`, `provider_local_draft`, `infrastructure_overlay_document`

## Downgrade rules

- `canonical_identity_unavailable_narrows_claim`
- `canonical_save_target_unavailable_narrows_claim`
- `watch_truth_degraded_stales_generated_or_imported_rows`
- `mutation_lineage_scope_narrows_nonordinary_rows`
- `recovery_path_scope_narrows_claim`
- `deferred_intent_requires_review_or_revalidation`
- `consumer_binding_missing_blocks_broad_claim`

## Downstream bindings

- Help surface: `crates/aureline-shell/src/help/filesystem_continuity.rs`
- Diagnostics export: `crates/aureline-shell/src/diagnostics/filesystem_continuity.rs`
- Support bundle: `crates/aureline-support/src/m5_fs_mutation_lineage_certification/mod.rs::support_bundle_projection`
- Release center: `crates/aureline-shell/src/release_center/filesystem_continuity.rs`

## Notes

This packet is metadata-safe. It carries typed state, row ids, lineage root
refs, recovery family refs, and downgrade rules only. It does not embed raw
file payloads, provider payloads, or ambient authority.
