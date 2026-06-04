# Preview, designer, and publish qualification

A preview, designer, share/export, or publish/deploy surface shows its lifecycle
label from:

[`artifacts/release/m4/preview-designer-publish-surface-qualification.json`](../../artifacts/release/m4/preview-designer-publish-surface-qualification.json)

Use the row state directly:

- `displayed_label` is the label product surfaces render.
- `source_mapping_quality` says whether the surface maps back to canonical
  source or is approximate, unsupported, generated-only, or snapshot-only.
- `source_sync_state` says whether the projection is in sync with source.
- `generated_source_truth` distinguishes canonical source, live output, mock or
  imported output, cached output, generated projection, and preview snapshot.
- `exported_artifact_truth` tells support and downstream consumers what an
  export or publish packet actually contains.

Stable preview does not imply browser-runtime inspection. Browser DOM/CSS,
console, network/storage, source-map drift, and live runtime mutation are
separate contracts.

