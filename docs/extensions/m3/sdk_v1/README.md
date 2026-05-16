# SDK v1 typed APIs, manifest authoring guides, and canonical sample-extension pack

This page is the reviewer-facing entrypoint for the SDK v1 beta starter
pack: the stable beta starting point extension authors build, validate,
publish, and recover against. It binds three previously-separate truth
sources into one record set so authors, reviewers, support, and
partners read one truth instead of inventing per-surface "this works"
copy:

- **Typed API surfaces.** One row per published SDK v1 typed-API
  surface (wasm component-model, wasm core-module, external-host
  supervised, helper-binary, compatibility-bridge, remote-side-
  component). Each surface pins its host contract family, the WIT
  world refs / external host contract ref it covers, and a closed
  availability vocabulary under the M3 beta admission lane.
- **Manifest authoring guides.** One row per authored walkthrough
  that pairs a typed guide class with the surfaces it covers and a
  non-empty repair-affordance label.
- **Canonical sample-extension pack.** One row per checked-in
  sample whose typed entry class and validation class make the
  wasm and external-host claims real rather than implied. Every
  sample carries opaque refs to its manifest baseline, permission
  manifest, runtime contract, and SDK release-bundle so the same
  row reviewers see in install / review chrome is the row authors
  build against.

The contract is governed. The canonical Rust source of truth lives in
[`crates/aureline-extensions/src/sdk_v1/`](../../../../crates/aureline-extensions/src/sdk_v1/);
the extension runtime / SDK / manifest / bridge compatibility matrix is
[`artifacts/compat/m3/bridge_matrix.yaml`](../../../../artifacts/compat/m3/bridge_matrix.yaml);
the cross-tool boundary schema is
[`schemas/extensions/sdk_v1_starter_pack.schema.json`](../../../../schemas/extensions/sdk_v1_starter_pack.schema.json);
the checked-in fixtures live under
[`fixtures/extensions/m3/sample_pack/`](../../../../fixtures/extensions/m3/sample_pack/).

## What the starter pack covers

One `SdkV1StarterPackRecord` binds the truth that every later surface
(install / review, permission inspector, support export, partner packet
template, CLI / headless review) needs to render a single, inspectable
answer to "is this SDK line ready for authors?":

- the SDK line id and semver, mirrored verbatim from the SDK
  publication contract in
  [`docs/extensions/sdk_publication_contract.md`](../../sdk_publication_contract.md);
- the list of claimed API surface rows, each typed by
  `SdkV1ApiSurfaceClass` and `SdkV1ApiAvailabilityClass`;
- the list of manifest authoring guides, each typed by
  `SdkV1ManifestGuideClass` with at least one covered API surface
  class;
- the list of sample-pack rows, each typed by
  `SamplePackEntryClass` and `SamplePackValidationClass` and bound
  to its manifest / permission / runtime / SDK release-bundle refs;
- derived counts for claimed surfaces, available-in-beta surfaces,
  preview surfaces, validated wasm samples, validated external-host
  samples, and authoring guides; and
- a closed `SdkV1StarterPackDecisionClass` paired with a closed
  `SdkV1StarterPackReasonClass` and a metadata-safe summary the
  install / review chrome reads verbatim.

The record always emits `RedactionClass::MetadataSafeDefault`. Raw SDK
release bytes, raw signing-key material, raw policy bodies, raw paths,
raw tokens, raw bridge-shim payloads, and raw publisher-private data
MUST NOT appear anywhere in the record set; every field is an opaque
ref or a closed vocabulary value.

## Why one record covers both wasm and external-host samples

The acceptance gate for this lane is that sample extensions cover both
Wasm and external-host patterns relevant to claimed beta lanes. Earlier
attempts kept two parallel sample registries: a Wasm sample list with
its own validation state, and an external-host sample list with its
own restart / supervision posture. The starter pack collapses those
into one record:

- `SamplePackEntryClass` resolves Wasm minimal / capability-negotiated
  rows, external-host minimal / capability-negotiated rows, helper-
  binary rows, and documentation walkthrough rows under one closed
  vocabulary.
- `SamplePackValidationClass` is the orthogonal axis: a row either
  must compile in CI, must validate in CI through the sample-validator
  kit, must run in CI against a reference host, or is documentation-
  only. The evaluator refuses closed if a runnable entry is tagged
  `documentation_only`, and refuses closed if a documentation
  walkthrough is tagged anything else.
- Each sample's `host_contract_family_class` MUST match the
  `SdkV1ApiSurfaceClass` it claims under, so a wasm sample cannot
  pretend to run under an external-host supervision model and an
  external-host sample cannot pretend to be a wasm component.

That means one starter pack record covers every host shape the M3
wedge claims, and one support export panel quotes the same closed
tokens whether the row is a wasm component sample or an external LSP
host sample.

## Admission flow

[`evaluate_sdk_v1_starter_pack`](../../../../crates/aureline-extensions/src/sdk_v1/mod.rs)
is deterministic and fails closed. In strict precedence order:

1. If `starter_pack_id` is not prefixed `sdk_v1_starter_pack:`, the
   evaluator refuses with `refused_pack_id_unprefixed`.
2. If `sdk_line_id` is empty, the evaluator refuses with
   `refused_sdk_line_ref_missing`.
3. If `claimed_api_surfaces` is empty, the evaluator refuses with
   `refused_claimed_surfaces_empty`.
4. If any claimed surface's `sdk_release_bundle_ref` is missing the
   `sdk_release_bundle:` prefix, the evaluator refuses with
   `refused_sdk_line_ref_missing`.
5. If any claimed surface is `retired_pending_successor`, the evaluator
   refuses with `refused_surface_availability_retired`.
6. For each sample row in precedence order: a missing
   `manifest_baseline:`, `permission_manifest:`, or
   `runtime_v1_beta:` prefix refuses with the matching reason; a
   sample whose `host_contract_family_class` disagrees with its
   `api_surface_class` refuses with
   `refused_sample_host_family_disagrees_with_api_surface`; a runnable
   sample tagged `documentation_only` refuses with
   `refused_sample_validation_documentation_only_on_runnable_entry`.
7. If the pack claims any wasm API surface but ships no validated
   runnable wasm sample, the evaluator refuses with
   `refused_no_wasm_sample_for_claimed_wasm_lane`.
8. If the pack claims an `external_host_supervised_api` but ships no
   validated runnable external-host sample, the evaluator refuses
   with `refused_no_external_host_sample_for_claimed_external_host_lane`.
9. If any claimed API surface is not covered by at least one authoring
   guide, the evaluator refuses with
   `refused_authoring_guide_missing_for_claimed_surface`.
10. If any claimed API surface is `preview_in_beta` or
    `not_available_until_general`, the pack resolves to
    `partially_ready_preview_surfaces_only`.
11. Otherwise, the pack resolves to `ready_for_authors` with reason
    `all_claimed_surfaces_available_in_beta`.

## Closed vocabularies

The following vocabularies are closed. Adding a member is additive-
minor with an `sdk_v1_starter_pack_schema_version` bump; repurposing
an existing member is breaking and requires a new decision row.

| Vocabulary                              | Members                                                                                                                                                                   |
|-----------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `SdkV1ApiSurfaceClass`                  | `wasm_component_model_host_api`, `wasm_core_module_host_api`, `external_host_supervised_api`, `helper_binary_api`, `compatibility_bridge_api`, `remote_side_component_api` |
| `SdkV1ApiAvailabilityClass`             | `available_in_beta`, `preview_in_beta`, `not_available_until_general`, `retired_pending_successor`                                                                          |
| `SamplePackEntryClass`                  | `wasm_component_minimal`, `wasm_component_capability_negotiated`, `external_host_supervised_minimal`, `external_host_supervised_capability_negotiated`, `helper_binary_short_lived`, `manifest_authoring_walkthrough` |
| `SamplePackValidationClass`             | `must_compile_in_ci`, `must_validate_in_ci`, `must_run_in_ci`, `documentation_only`                                                                                        |
| `SdkV1ManifestGuideClass`               | `permission_declaration_walkthrough`, `host_contract_family_walkthrough`, `publisher_identity_walkthrough`, `runtime_budget_walkthrough`, `update_and_rollback_walkthrough`, `sdk_publication_walkthrough` |
| `SdkV1StarterPackDecisionClass`         | `ready_for_authors`, `partially_ready_preview_surfaces_only`, `refused_inconsistent_input`                                                                                 |

The `SdkV1ApiSurfaceClass` ↔ `HostContractFamilyClass` mapping is
total and closed; `host_contract_family_for_api_surface` is the only
authorized projection.

## Fixture catalogue

| Fixture                                                                                            | Decision class                              | Reason class                                                              |
|----------------------------------------------------------------------------------------------------|---------------------------------------------|---------------------------------------------------------------------------|
| [`ready_for_authors_wasm_and_external_host.json`](../../../../fixtures/extensions/m3/sample_pack/ready_for_authors_wasm_and_external_host.json) | `ready_for_authors`                         | `all_claimed_surfaces_available_in_beta`                                  |
| [`partially_ready_preview_surface.json`](../../../../fixtures/extensions/m3/sample_pack/partially_ready_preview_surface.json)                   | `partially_ready_preview_surfaces_only`     | `some_claimed_surfaces_preview_in_beta`                                   |
| [`refused_missing_wasm_sample.json`](../../../../fixtures/extensions/m3/sample_pack/refused_missing_wasm_sample.json)                           | `refused_inconsistent_input`                | `refused_no_wasm_sample_for_claimed_wasm_lane`                            |
| [`refused_authoring_guide_missing.json`](../../../../fixtures/extensions/m3/sample_pack/refused_authoring_guide_missing.json)                   | `refused_inconsistent_input`                | `refused_authoring_guide_missing_for_claimed_surface`                     |
| [`refused_retired_surface.json`](../../../../fixtures/extensions/m3/sample_pack/refused_retired_surface.json)                                   | `refused_inconsistent_input`                | `refused_surface_availability_retired`                                    |

## Manifest authoring guides

The starter pack ships canonical authoring guides authors read against
the surfaces they target. Each guide lives in this directory and is
also referenced by the runtime, permission, supervision, and SDK
publication contracts:

- [`manifest_authoring_permission_declaration.md`](./manifest_authoring_permission_declaration.md)
- [`manifest_authoring_host_contract_family.md`](./manifest_authoring_host_contract_family.md)
- [`manifest_authoring_publisher_identity.md`](./manifest_authoring_publisher_identity.md)
- [`manifest_authoring_update_and_rollback.md`](./manifest_authoring_update_and_rollback.md)

## Consumer expectations

The downstream surfaces below MUST read this record set rather than
invent SDK-v1-shaped fields:

- **Install / review and permission inspector.** Project
  `decision_class`, `reason_class`, `available_in_beta_surface_count`,
  and `preview_in_beta_surface_count` from the starter pack record
  the install resolves through. A surface that hides any of these is
  denied with `review_disclosure_incomplete` per the SDK publication
  contract.
- **Support export and partner packet template.** Read
  `SdkV1StarterPackSupportExportRecord` verbatim; never invent a
  "this SDK is ready" string. The export pins
  `blocks_authoring` and `preview_disclosure_required` flags so the
  partner packet template stays bound to the typed decision.
- **CLI / headless review.** Quotes the same closed tokens as the
  install / review chrome; refuse the install with the same reason
  class the chrome would show.
- **Compatibility matrix.** Native, bridge, shimmed, partial, and
  unsupported author paths cite
  [`docs/extensions/m3/compatibility_matrix_beta.md`](../compatibility_matrix_beta.md)
  and the matching `extension_bridge_row:*` id. Bridge or shimmed paths
  cannot be documented as exact SDK parity.
- **Extension validator.** Validates author manifests against the beta
  SDK, permission, lifecycle, compatibility, and conformance-fixture
  expectations before registry ingest. The command and fixture suite
  are documented in
  [`docs/extensions/m3/conformance_kit_beta.md`](../conformance_kit_beta.md).
- **Air-gapped mirror / offline-bundle review.** Re-emits the same
  record set when the SDK line is republished into a sealed bundle;
  the same closed vocabulary applies regardless of the origin source.

## How to verify

```text
cargo build -p aureline-extensions
cargo test -p aureline-extensions sdk_v1
cargo run --example dump_sdk_v1_starter_pack_records -p aureline-extensions
```

The `dump_sdk_v1_starter_pack_records` example emits every fixture's
evaluator output for independent JSON-schema validation against
[`schemas/extensions/sdk_v1_starter_pack.schema.json`](../../../../schemas/extensions/sdk_v1_starter_pack.schema.json).

## Guardrails

- No beta widening on opaque publisher identity. Every sample row
  cites a permission-manifest ref that already pins the publisher
  identity through the manifest baseline; the runtime contract refuses
  to admit a sample whose publisher identity is opaque.
- No beta widening on missing diff reports. The starter pack delegates
  the declared-vs-effective permission diff to the permission-manifest
  beta lane, which already refuses closed when the diff is missing.
- No beta widening on unbounded host authority. Sample rows MUST cite
  a `runtime_v1_beta:` ref, and the runtime contract refuses
  to admit a row whose host placement and supervision do not match
  the declared host contract family.

## Out of scope

- Quantitative support-window lengths per surface class. Lengths
  remain in the SDK publication contract.
- A marketplace recommendation engine or vanity catalog surface.
- A public-mirror service for the sample pack; the seed pins the row
  shape, the mirror service is a successor concern.
