# SDK v1 canonical sample-extension pack fixtures

These fixtures anchor the SDK v1 starter pack contract owned by
[`crates/aureline-extensions/src/sdk_v1/`](../../../../crates/aureline-extensions/src/sdk_v1/)
and validated by the cross-tool boundary schema at
[`schemas/extensions/sdk_v1_starter_pack.schema.json`](../../../../schemas/extensions/sdk_v1_starter_pack.schema.json).

Each fixture exercises one decision class / reason class of the
[`evaluate_sdk_v1_starter_pack`](../../../../crates/aureline-extensions/src/sdk_v1/mod.rs)
evaluator. The reviewer-facing landing page is
[`docs/extensions/m3/sdk_v1/`](../../../../docs/extensions/m3/sdk_v1/).

**Scope rules**

- Every fixture deserializes into an `SdkV1StarterPackInput` and the
  evaluator's output MUST reproduce the
  `expected_decision_class` / `expected_reason_class` / counter
  values declared under `__fixture__`.
- Every record carries `redaction_class = metadata_safe_default`.
  Raw manifest bytes, raw signing-key material, raw policy bodies,
  raw paths, raw tokens, raw bridge-shim payloads, and raw
  publisher-private data MUST NOT appear; refs stand in for every
  field that would otherwise carry raw material.
- Ids and refs are opaque; they are chosen to read well rather than
  to reflect any real deployment.

**Index**

| Fixture                                                                                            | Decision class                              | Reason class                                                              |
|----------------------------------------------------------------------------------------------------|---------------------------------------------|---------------------------------------------------------------------------|
| [`ready_for_authors_wasm_and_external_host.json`](./ready_for_authors_wasm_and_external_host.json) | `ready_for_authors`                         | `all_claimed_surfaces_available_in_beta`                                  |
| [`partially_ready_preview_surface.json`](./partially_ready_preview_surface.json)                   | `partially_ready_preview_surfaces_only`     | `some_claimed_surfaces_preview_in_beta`                                   |
| [`refused_missing_wasm_sample.json`](./refused_missing_wasm_sample.json)                           | `refused_inconsistent_input`                | `refused_no_wasm_sample_for_claimed_wasm_lane`                            |
| [`refused_authoring_guide_missing.json`](./refused_authoring_guide_missing.json)                   | `refused_inconsistent_input`                | `refused_authoring_guide_missing_for_claimed_surface`                     |
| [`refused_retired_surface.json`](./refused_retired_surface.json)                                   | `refused_inconsistent_input`                | `refused_surface_availability_retired`                                    |
