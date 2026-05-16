# Extension publication pipeline packet

This directory contains the checked beta publication packet generated
by `tools/extensions/m3/publish_extension.py`.

The packet demonstrates the governed publication path for
`dev.aureline.samples/wasm-notes`:

- `publication_pipeline_record.json` is the canonical packet.
- `publication_support_export.json` is the metadata-safe support view.
- `registry_manifest_row.json` is the registry row for the published
  artifact.
- `promotion_rows.json` shows the monotone channel moves.
- `rollback_manifest.json` preserves the prior installable artifact.
- `catalog_snapshot.json` is the final catalog pointer written only
  after all sidecar rows validate.

Refresh the packet with the command documented in
`docs/extensions/m3/publication_pipeline_beta.md`.
