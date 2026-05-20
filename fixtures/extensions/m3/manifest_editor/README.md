# Extension manifest editor session fixtures

Input + expectation fixtures for the native extension manifest/schema editor
(`aureline_extensions::manifest_editor`). Each file carries:

- `input` — a `ManifestEditorSessionInput` (session id, timestamp, manifest ref,
  connectivity class, and the manifest payload under edit).
- `__fixture__` — the expected publish-readiness decision, result class, blocker
  and advisory counts, migration-hint count, and (where useful) the expected
  blocker check ids and red-flag classes.

The fixtures are replayed by `crates/aureline-extensions/src/manifest_editor/tests.rs`
and dumped by `cargo run --example dump_manifest_editor_records -p aureline-extensions`.

| Fixture | What it proves |
| --- | --- |
| `ready_to_publish_wasm.json` | A complete, valid wasm manifest opens with zero blockers and zero advisories. |
| `advisories_deprecated_state_eager_startup.json` | A manifest that clears every publish blocker still surfaces a lifecycle-driven migration hint for the deprecated `lifecycle.state` value plus UX/performance advisories, without blocking publication. |
| `blocked_invalid_identity_and_vocabulary.json` | An anonymous publisher and out-of-vocabulary support class / lifecycle state produce three field-anchored must-fix blockers and a non-publishable session. |

Blocker findings reuse the same stable `check_id`, `suite`, `status`, and
`severity` vocabulary as the beta extension validator CLI
(`tools/extensions/m3/validator_cli/aureline_extension_validator.py`) and the
conformance kit report schema
(`schemas/extensions/conformance_kit_report.schema.json`). Editor-only
recommended improvements live in the `editor_advisory` suite under the
`advisory.*` check-id namespace. Migration hints are derived from the canonical
lifecycle metadata packet (`artifacts/extensions/m3/lifecycle_metadata_packet.json`),
so validation, permission reasoning, and migration guidance stay available
offline.
