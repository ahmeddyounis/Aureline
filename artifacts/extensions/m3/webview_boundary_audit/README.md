# Extension webview boundary audit packet

This directory mirrors the generated packet and support export for the
extension webview boundary audit.

- `audit_packet.json` is the canonical generated packet.
- `support_export.json` is the metadata-safe support export projected
  from the same rows.

Refresh or inspect the packet with:

```text
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- packet
cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- support-export
```
