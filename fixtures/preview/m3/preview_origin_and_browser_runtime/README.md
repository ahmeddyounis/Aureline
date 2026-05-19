# Preview-origin and browser-runtime fixture corpus

Worked corpus for the preview-origin, preview-target, hot-reload descriptor,
browser-runtime session-origin, and runtime-mutation-action-plan records
this milestone freezes.

Contract:
[`/docs/preview/m3/preview_target_and_hot_reload_beta.md`](../../../../docs/preview/m3/preview_target_and_hot_reload_beta.md)

Schemas:
- [`/schemas/preview/preview_target_descriptor.schema.json`](../../../../schemas/preview/preview_target_descriptor.schema.json)
- [`/schemas/preview/hot_reload_state.schema.json`](../../../../schemas/preview/hot_reload_state.schema.json)
- [`/schemas/browser_runtime/session_origin.schema.json`](../../../../schemas/browser_runtime/session_origin.schema.json)

Implementation:
[`/crates/aureline-preview/src/preview_origin/`](../../../../crates/aureline-preview/src/preview_origin/mod.rs)

Fixture naming:
- `*.positive.json` — a record the schema and the Rust `validate()` honesty
  rules both accept.
- `*.negative.json` — a record that exercises one named honesty rule. Each
  carries a `__fixture__.expected_violation_check_id` field naming the
  finding the validator must report.

Raw URLs, raw IPs, raw hostnames, raw device serial numbers, raw cookies,
raw selectors, and raw private app state never appear in these fixtures;
opaque refs and closed class labels do.
