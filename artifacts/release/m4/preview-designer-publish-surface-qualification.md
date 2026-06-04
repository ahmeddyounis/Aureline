# Preview, designer, and publish surface qualification packet

This packet is the reviewer-facing companion for:

- [`artifacts/release/m4/preview-designer-publish-surface-qualification.json`](./preview-designer-publish-surface-qualification.json)
- [`schemas/release/preview-designer-publish-surface-qualification.schema.json`](../../../schemas/release/preview-designer-publish-surface-qualification.schema.json)
- [`docs/release/preview-designer-publish-surface-qualification.md`](../../../docs/release/preview-designer-publish-surface-qualification.md)

The JSON packet is canonical. Docs, Help/About, release packets, product labels,
share/export sheets, and support exports should ingest rows by `surface_id` and
render `displayed_label`, `source_mapping_quality`, `source_sync_state`,
`generated_source_truth`, and `exported_artifact_truth`.

## Qualification result

| Surface | Displayed label | Mapping | Source truth | Required fallback |
|---|---:|---|---|---|
| Source-mapped preview runtime | Stable | Canonical source mapping | Live runtime output | Open source, open diff, rollback lineage |
| Device and viewport preview rows | Preview | Approximate mapping | Generated projection | Open source, open diff, raw source |
| Visual designer canvas and property inspector | Preview | Unsupported construct | Generated projection | Open source, open diff, raw source |
| Preview share and export sheet | Beta | Snapshot only | Preview snapshot | Open source, open diff, rollback lineage |
| Publish and deploy review sheet | Beta | Generated only | Generated projection | Open source, open diff, rollback lineage |

## Boundary proof

- Preview runtime qualification does not imply browser-runtime inspection. The
  green preview row cites runtime identity but leaves DOM/CSS, console,
  network/storage, source-map drift, and live runtime mutation to
  [`docs/runtime/browser_runtime_contract.md`](../../../docs/runtime/browser_runtime_contract.md).
- Designer surfaces stay below Stable until source-to-canvas round trip is
  proven for unsupported framework constructs. Source remains the canonical
  writable artifact.
- Share/export/publish sheets disclose whether the downstream artifact is
  canonical source, a preview snapshot, a generated projection, an external
  browser capture, or a publish dry-run packet.
- Publish/deploy rows require dry run plus preview/apply/revert lineage before
  side effects, and rollback lineage must remain exportable.

## Packet-freshness SLO

The current green proof uses a 180-day max age and 30-day warning window. A row
that lacks a current captured packet narrows below Stable until a fresh packet is
captured and owner-signed.

## Evidence refs

- [`docs/architecture/preview_runtime_contract.md`](../../../docs/architecture/preview_runtime_contract.md)
- [`docs/security/safe_preview_trust_classes.md`](../../../docs/security/safe_preview_trust_classes.md)
- [`docs/framework/framework_certainty_and_source_sync_contract.md`](../../../docs/framework/framework_certainty_and_source_sync_contract.md)
- [`docs/runtime/browser_runtime_contract.md`](../../../docs/runtime/browser_runtime_contract.md)
- [`fixtures/runtime/browser_runtime_cases/README.md`](../../../fixtures/runtime/browser_runtime_cases/README.md)

