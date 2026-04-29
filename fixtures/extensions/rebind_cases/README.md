# Extension rebinding + repro-export fixtures

These fixtures anchor the contracts frozen in:

- `/docs/extensions/dev_loop_rebinding_and_repro_contract.md`
- `/schemas/extensions/rebind_review.schema.json`
- `/schemas/extensions/repro_export.schema.json`

They also intentionally reuse the continuity vocabulary from:

- `/schemas/extensions/sideload_review.schema.json`

Raw source code, raw absolute filesystem paths, raw registry URLs, raw
wasm bytes, raw log bodies, raw crash dumps, and raw signing-key
material MUST NOT appear in any fixture.

## Fixtures

| File | Schema | Demonstrates |
|---|---|---|
| `local_to_registry_rebinding.yaml` | `rebind_review.schema.json` | Local/sideload package binding to a registry-backed identity requires explicit review and discloses namespace/signer/channel/trust promotion. |
| `source_moved_continuity.yaml` | `rebind_review.schema.json` | Source path hint changes without silently changing identity or trust posture; review stays explicit for any real trust/ABI/permission change. |
| `build_failed_repro_export.yaml` | `repro_export.schema.json` | Repro export packet after a build failure preserves digest/ABI/permissions/log summaries for mechanical comparison. |
| `manifest_widened_during_hot_reload.yaml` | `sideload_review.schema.json` | Permission widening during dev hot reload forces a re-review instead of silently widening authority. |

