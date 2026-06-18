# M5 token-overlay sync/import fixtures

These fixtures are the checked-in canonical truth for the M5 token-overlay
round-trip portability audit. They conform to
[`schemas/ux/token-overlay.schema.json`](../../../../schemas/ux/token-overlay.schema.json)
and are validated by the CI gate at
[`tools/ci/m5/token_overlay_check.py`](../../../../tools/ci/m5/token_overlay_check.py).
See the companion doc
[`docs/m5/token-overlays-and-scope.md`](../../../../docs/m5/token-overlays-and-scope.md)
for how appearance overrides become scope-explicit, downgrade-safe,
round-trip-portable objects.

| File | Record kind | Why it is here |
| --- | --- | --- |
| `report.json` | `shell_m5_token_overlay_portability_report_record` | The per-scope overlays, the override entries, the winning-versus-shadowed resolution table, the export/import/sync round-trip proof, and the blocking-finding summary. |
| `support_export.json` | `shell_m5_token_overlay_portability_support_export_record` | The support-export wrapper a reviewer pivots on; its `case_ids` quote the report id, the appearance-session ref, every overlay id, every entry id, every resolved token ref, the proof id, and every stage id. |
| `compact.txt` | (rendered summary) | One-line-per-row audit summary the headless inspector prints. |

The fixtures are the **only mint-from-truth output** of the headless inspector
`aureline_shell_m5_token_overlays`; regenerate them with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- report > \
  fixtures/ux/m5/token-overlay-sync-import/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- support-export > \
  fixtures/ux/m5/token-overlay-sync-import/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- compact > \
  fixtures/ux/m5/token-overlay-sync-import/compact.txt
```

The clean checked-in report exercises every disclosed-overlay scenario the
contract is built to keep honest:

- Seven overlays — the inherited theme-package base plus the `imported_theme`,
  `extension_contributed`, `user_global`, `profile`, `workspace`, and
  `policy_managed` scopes — each kept structured rather than flattened into an
  opaque blob.
- Five resolved tokens make precedence inspectable: a workspace accent wins over
  the user-global override and theme default; a managed-policy danger colour caps
  the profile override; a profile row spacing wins over the user-global
  override; an extension code-role override wins with a disclosed deprecated
  alias; and an imported chart slot is unmapped and stays an inert placeholder.
- The export → import → sync round trip preserves four overrides unchanged and
  carries the deprecated alias and the unmapped chart slot forward as disclosed
  downgrades — neither is dropped, rewritten, or treated as fully supported, and
  no override loses its scope.

Every record stays clean because each override is scope-explicit, every
unsupported token survives as a disclosed downgrade, and every overlay stays
structured; the same conditions become blockers the moment an override loses its
scope, an unsupported token is dropped or treated as supported, a resolution
names the wrong winner or hides a shadowed entry, or an overlay is flattened
into an opaque profile blob.
