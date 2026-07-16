# M5 Imported / Offline Evidence Lineage Propagation

Row **M05-1253** — B149 cross-surface lineage-propagation lane.

This contract governs how imported / offline evidence descriptors and the shared
`Showing imported or offline evidence` label flow from the primary archive viewer into the first downstream
consumers that can ingest archived data — companion cards, browser / export handoffs, support packets, and
AI explanation / evidence consumers — so non-live data can never masquerade as current route, provider, or
service truth.

It builds directly on the frozen historical-reference matrix
([`m5-historical-reference-matrix`](../../artifacts/program/m5-historical-reference-matrix.md)) and its five
non-live-evidence object classes, reusing the matrix object, consumer-surface, role, and accessibility-route
vocabularies rather than restating them.

## What the packet proves

- **One non-live vocabulary across consumers (AC1).** Every binding carries the controlled non-live grammar —
  historical-role, snapshot-label, capture-time, provenance, mutation-blocked-posture, and the shared
  `Showing imported or offline evidence` label — identical for one profile across every surface. At least one
  companion / export surface and one support / AI consumer render the same vocabulary and lineage fields as the
  primary archive viewer.
- **No silent promotion to live truth (AC2).** The consumer action set (`inspect_lineage`, `export_lineage`, and
  — only when the lineage joins a validated live-target handoff — `open_current_live_object`) is closed and
  analysis-only. There is no rank / narrate / summarize-as-current affordance. The non-live boundary is always
  explicitly called out, and the guardrails `ranked_or_narrated_as_current_live_service_truth` and
  `presents_imported_offline_as_current_route_or_provider_state` must both be `false`.
- **Export-safe lineage (AC3).** Each descriptor joins its lineage back to a source snapshot descriptor and a
  controlled capture-context join, and names either a live-target handoff packet or a metadata-only exit **by
  controlled id** rather than embedding a live route, secret, or authority. The export is scrubbed for forbidden
  boundary material, and the `leaks_live_secret_or_stale_authority_through_lineage` guardrail must be `false`.

## Dispositions

| Disposition | Open-live? | Handoff ref | Metadata-only exit | Content | Parity |
| --- | --- | --- | --- | --- | --- |
| `live_target_joinable` | yes | required | — | available | `live_target_lineage_joined` |
| `imported_offline_only` | no | — | required | available | `non_live_boundary_disclosed` |
| `metadata_only_exit` | no | — | required | unavailable | `non_live_boundary_disclosed` |
| `exported_redacted_lineage` | no | — | required | available | `non_live_boundary_disclosed` |

When content is unavailable, the binding still renders capture time, provenance, and its non-live boundary note
instead of degrading to a dead link.

## Artifacts

- Boundary schema: [`schemas/program/m5-imported-offline-evidence-lineage-propagation.schema.json`](../../schemas/program/m5-imported-offline-evidence-lineage-propagation.schema.json)
- Support export: [`artifacts/support/m5-imported-offline-lineage/support_export.json`](../../artifacts/support/m5-imported-offline-lineage/support_export.json)
- Matrix CSV: [`artifacts/support/m5-imported-offline-lineage/matrix.csv`](../../artifacts/support/m5-imported-offline-lineage/matrix.csv)
- Markdown summary: [`artifacts/support/m5-imported-offline-lineage/summary.md`](../../artifacts/support/m5-imported-offline-lineage/summary.md)
- Narrowed fixtures: [`fixtures/recovery/m5-imported-offline-lineage/`](../../fixtures/recovery/m5-imported-offline-lineage/)

## Regenerating

The seed builder in `crates/aureline-ui/src/m5_imported_offline_evidence_lineage_propagation/seed.rs` is the only
mint-from-truth path. Re-emit with:

```text
cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- support-export
cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- csv
cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- report
cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- fixture-imported-offline-narrowed
cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- fixture-metadata-only-narrowed
```
