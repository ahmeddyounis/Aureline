# Extension SDK publication example fixtures

These fixtures anchor the publication vocabulary frozen in
[`/docs/extensions/sdk_publication_contract.md`](../../../docs/extensions/sdk_publication_contract.md)
and validated by the seed schemas at
[`/schemas/extensions/sdk_release_bundle.schema.json`](../../../schemas/extensions/sdk_release_bundle.schema.json)
and
[`/schemas/extensions/conformance_result.schema.json`](../../../schemas/extensions/conformance_result.schema.json).

The SDK publication seed is at `Status: Proposed`. These fixtures
exercise the reserved field sets, the SDK-line identity envelope,
the build-identity content-address shape, the mirror-availability,
support-window, and compatibility-badge vocabularies, and the
schema `allOf` gates so the later SDK lane, docs lane,
conformance lane, and air-gapped mirror lane can be built against
one publication contract rather than invent SDK-shaped fields ad
hoc.

**Scope rules**

- Each `*_release_*.yaml` and `*_bundle_*.yaml` and `*_kit_*.yaml`
  fixture validates against
  `schemas/extensions/sdk_release_bundle.schema.json` as one of
  `wit_package_release_row`,
  `generated_binding_row`,
  `docs_pack_export_row`,
  `tutorial_sample_bundle_row`, or
  `conformance_kit_release_row`.
- Each `conformance_result_*.yaml` fixture validates against
  `schemas/extensions/conformance_result.schema.json` as a
  `conformance_result_record`.
- A fixture MUST exercise at least one frozen
  `release_artifact_class`, `host_abi_window_class`,
  `mirror_availability_class`, `support_window_class`,
  `compatibility_badge_class`, `target_language_class`,
  `binding_kind_class`, `docs_pack_kind_class`,
  `docs_format_class`, `bundle_kind_class`,
  `sample_validation_state`, `tooling_class`, `result_class`,
  `coverage_class`, `failure_reason_class`,
  `sdk_audit_event_id`, or `sdk_denial_reason` and MUST name the
  seed section that motivates it.
- Raw artifact bytes, raw signing-key material, raw bindgen
  inputs, raw docs export bytes, raw sample / template bytes, raw
  conformance run outputs, raw stack frames, and raw target-host
  secrets MUST NOT appear; refs stand in for every field that
  would otherwise carry raw material.
- Ids, refs, aliases, and monotonic timestamps are opaque; they
  are chosen to read well rather than to reflect any real
  deployment.

**Index**

| Fixture                                                                                                              | Record kind                       | Key classes exercised                                                                                                                                                  | Seed section                                                                                                          |
|----------------------------------------------------------------------------------------------------------------------|-----------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| [`wit_package_release_general_support.yaml`](./wit_package_release_general_support.yaml)                             | `wit_package_release_row`         | `wit_package_release` / `component_model_abi_window_general_1` / `public_and_approved_mirror` / `general_long_window` / `compatible_on_declared_targets`                | SDK-line identity envelope; Build identity; Support window                                                            |
| [`generated_binding_rust_typed_bindings.yaml`](./generated_binding_rust_typed_bindings.yaml)                         | `generated_binding_row`           | `generated_binding` / `rust` / `typed_bindings_full` / `offline_mirror_bundle_eligible` / `general_long_window`                                                         | Per-class fields :: generated_binding_row; Mirror availability and offline bundles                                    |
| [`docs_pack_export_air_gapped_static_site.yaml`](./docs_pack_export_air_gapped_static_site.yaml)                     | `docs_pack_export_row`            | `docs_pack_export` / `api_reference_html` / `html_static_site` / `air_gapped_mirror_only` / `general_long_window`                                                       | Per-class fields :: docs_pack_export_row; Static docs exports and air-gapped mirror bundles                           |
| [`tutorial_sample_bundle_must_compile_in_ci.yaml`](./tutorial_sample_bundle_must_compile_in_ci.yaml)                 | `tutorial_sample_bundle_row`      | `tutorial_sample_bundle` / `tutorial_pack` / `must_compile_in_ci` / `public_and_approved_mirror` / `general_short_window`                                               | Per-class fields :: tutorial_sample_bundle_row; Sample-validation rule                                                |
| [`conformance_kit_release_host_tester.yaml`](./conformance_kit_release_host_tester.yaml)                             | `conformance_kit_release_row`     | `conformance_kit_release` / `host_conformance_tester` / `public_and_approved_mirror` / `general_long_window` / `compatible_on_declared_targets`                         | Per-class fields :: conformance_kit_release_row                                                                       |
| [`conformance_result_host_pass_full_matrix.yaml`](./conformance_result_host_pass_full_matrix.yaml)                   | `conformance_result_record`       | `pass_full_matrix` / `full_declared_matrix` / `compatible_on_declared_targets` / `reference_host_in_repo`                                                               | Conformance result envelope; Compatibility badges                                                                     |
| [`conformance_result_fail_host_abi_drift.yaml`](./conformance_result_fail_host_abi_drift.yaml)                       | `conformance_result_record`       | `fail_blocking` / `partial_subset_documented` / `unsupported_pending_qualification` / `host_abi_drift_detected` / `qualification_lab_host`                              | Conformance result envelope; Denial reasons reserved                                                                  |

**Cross-row linkage**

Fixtures intentionally cross-link by stable opaque ref so the
publication-row → conformance-result → docs-pack / mirror-bundle
chains reviewers walk in the contract are exercised end to end:

- `wit_package_release_general_support` and
  `docs_pack_export_air_gapped_static_site` cite
  `conf-result-aureline-sdk-worlds-0-1-0-host-pass-full-matrix`
  (the `conformance_result_host_pass_full_matrix` row) as the
  passing run that backs their `general_long_window` /
  `compatible_on_declared_targets` claim.
- `generated_binding_rust_typed_bindings` cites
  `sdk-rel-aureline-sdk-worlds-0-1-0-wit` (the WIT package
  release) as `source_wit_release_ref`, plus its own
  `conf-result-aureline-sdk-worlds-0-1-0-rust-bindings-pass-full-matrix`
  conformance ref (a hypothetical sibling pass row not seeded
  here, which downstream lanes will land alongside Rust binding
  CI).
- `tutorial_sample_bundle_must_compile_in_ci` cites
  `conf-result-aureline-sdk-worlds-0-1-0-tutorial-pack-pass-subset`
  (a hypothetical sample-validator pass row) so the
  `must_compile_in_ci` claim is structurally backed by an
  evidence ref.
- `conformance_kit_release_host_tester` cites a self-published
  bootstrap pass run; later lanes replace it with a multi-host
  matrix as more `target_host_class` rows are seeded.
- `conformance_result_fail_host_abi_drift` cites a hypothetical
  `sdk-rel-aureline-sdk-worlds-0-2-0-rust-bindings-candidate`
  row whose support-window promotion would be denied with
  `support_window_unbacked_by_conformance_result` until a
  passing run replaces this failing one.
