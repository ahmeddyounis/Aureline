# Stable safe-preview trust classes release packet

Status: stable validation passes.

Canonical sources:

- `schemas/trust/safe-preview-trust-class.schema.json`
- `fixtures/trust/m4/stabilize-safe-preview-trust-classes/canonical_packet.json`
- `docs/trust/m4/stabilize-safe-preview-trust-classes.md`
- `aureline_content_safety::stable_safe_preview_trust`

Release evidence:

- The packet covers all four trust classes: `RawText`, `SanitizedRich`,
  `TrustedLocalActive`, and `IsolatedRemoteActive`.
- Stable consumer rows cover editor, docs/help preview, notebook rich output,
  preview/runtime, marketplace/account webviews, browser-runtime viewers,
  support/export, and trust-sensitive install/attach/approval/publish/delete
  reviews.
- Transfer cases cover raw-only, sanitized, trusted-local, isolated-remote,
  downgrade, and blocked paths.
- Active content downgrade triggers include trust loss, policy deny,
  disconnect, origin loss, unsupported host, blocked active capability, and
  support/export boundaries.
- Screenshot, support bundle, exported evidence, and diagnostics carriers retain
  trust-class and representation lineage for every Stable row.

Verification:

```sh
cargo test -p aureline-content-safety --test stable_safe_preview_trust
cargo run -q -p aureline-content-safety --bin stable_safe_preview_trust -- validate
```
