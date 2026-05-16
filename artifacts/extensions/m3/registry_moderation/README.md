# Registry moderation catalog descriptor packet

This directory contains the checked beta catalog descriptor packet for
`dev.aureline.samples/wasm-notes`.

- `catalog_descriptor_record.json` is the canonical moderated catalog row.
- `catalog_descriptor_support_export.json` is the metadata-safe support view.

Refresh or inspect the packet with:

```text
cargo run --example dump_registry_moderation_records -p aureline-extensions
```
