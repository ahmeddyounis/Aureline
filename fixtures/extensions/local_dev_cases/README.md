# Local extension development, sideload, runtime inspector, and publish preview fixtures

These fixtures anchor the contract frozen in:

- `/docs/extensions/local_dev_and_sideload_contract.md`
- `/schemas/extensions/sideload_review.schema.json`
- `/schemas/extensions/publish_preview.schema.json`

They are deliberately synthetic and exist to exercise required local
dev and sideload trust flows: unsigned local packages, permission
widening that forces re-review, missing source while a last-loaded build
remains active, and publish preview separating blockers vs warnings.

Raw source code, raw absolute filesystem paths, raw wasm bytes, raw log
bodies, raw crash dumps, and raw signing-key material MUST NOT appear in
any fixture.

## Fixtures

| File | Record kind | Demonstrates |
|---|---|---|
| `unsigned_local_package.yaml` | `local_extension_workspace_strip_record` | Local-only, unsigned package never inherits a high-trust publisher badge. |
| `permission_widening_requires_rereview.yaml` | `sideload_review_sheet_record` | Permission widening triggers a fresh review; update binding stays local until explicitly changed. |
| `source_missing_last_loaded_active.yaml` | `runtime_inspector_snapshot_record` | Source removed from disk while a last loaded build remains active; inspector preserves logs and failure history. |
| `publish_preview_blockers_vs_warnings.yaml` | `publish_preview_sheet_record` | Publish preview shows exact outbound destination and separates blockers from warnings with typed origins. |

