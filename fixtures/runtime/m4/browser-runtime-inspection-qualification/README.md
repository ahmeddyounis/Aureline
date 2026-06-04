# Browser Runtime Inspection Qualification Fixtures

Fixture pack for the browser-runtime inspection qualification packet.

The packet reuses the existing browser-runtime and inspection corpora:

- `fixtures/runtime/browser_runtime_cases/`
- `fixtures/runtime/browser_inspection_cases/`
- `fixtures/preview/m3/preview_origin_and_browser_runtime/`

## Coverage

| Fixture | Purpose |
|---|---|
| `qualification_manifest.yaml` | Lists required target kinds, source-map states, inspection states, mutation actions, consumer surfaces, and downgrade rules. |
| `mutation_review_drills.yaml` | Proves replay, clear storage, override, reload, live style edit, and protocol override actions need review, target identity, rollback/export lineage, redaction, and no hidden side effects. |
| `downgrade_drills.yaml` | Proves missing target identity, stale source maps, protocol unavailable, cross-origin limits, stale sessions, unsafe redaction, and missing consumer bindings downgrade or block stable claims. |

No fixture contains raw URLs, cookies, storage values, request or response
payloads, DOM text, source-map bytes, source files, screenshots, secrets, or
ambient runtime authority.
