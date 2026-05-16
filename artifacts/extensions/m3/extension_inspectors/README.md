# Extension inspector packet

This directory contains the checked beta extension inspector packet for
`dev.aureline.samples/wasm-notes`.

- `inspector_page.json` is the canonical shell page packet.
- `inspector_support_export.json` is the metadata-safe support export.

Refresh or inspect the packet with:

```text
cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- page
cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- support-export
```
