# Publish OpenAPI specs, lifecycle labels, and example packs for M5 service APIs

Evidence record for the canonical **M5 OpenAPI publication catalog**: the one
index that turns the optional managed-service OpenAPI document into a published
contract family by binding every service-API operation to a lifecycle label, an
auth-source class, an entitlement and policy-override posture, a mutability
posture, a preview/dry-run support class, an offline/cache behaviour, a
deprecation lane and sunset posture, a compatibility note, and a checked-in
example request/response pack.

## What shipped

- A checked-in catalog mapping every OpenAPI operation to its auth, mutability,
  preview, offline, deprecation, lifecycle, and example-pack metadata:
  [`/artifacts/contracts/m5-openapi-catalog.json`](../contracts/m5-openapi-catalog.json)
  (18 operations across 17 service surfaces and 12 api families).
- The boundary schema:
  [`/schemas/public/m5-contracts/m5_openapi_catalog.schema.json`](../../schemas/public/m5-contracts/m5_openapi_catalog.schema.json).
- The OpenAPI document the catalog indexes:
  [`/openapi/service_api_seed.yaml`](../../openapi/service_api_seed.yaml), with the
  publication index at [`/openapi/m5/README.md`](../../openapi/m5/README.md).
- The per-operation example request/response packs:
  [`/examples/contracts/m5-openapi/`](../../examples/contracts/m5-openapi/).
- The SDK index and narrative companion:
  [`/docs/sdk/m5-service-api-catalog.md`](../../docs/sdk/m5-service-api-catalog.md)
  and
  [`/docs/m5/publish-openapi-specs-lifecycle-labels-and-example-packs-for-m5-service-apis-registry-mirror-endpoints-admin-ai-usage-export-routes-and-managed-control-plane-surfaces.md`](../../docs/m5/publish-openapi-specs-lifecycle-labels-and-example-packs-for-m5-service-apis-registry-mirror-endpoints-admin-ai-usage-export-routes-and-managed-control-plane-surfaces.md).
- The typed product object plus its tests:
  `crates/aureline-release/src/publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces/`.
- The single source of truth (regenerator) and the validator:
  [`/tools/regenerate_m5_openapi_catalog.py`](../../tools/regenerate_m5_openapi_catalog.py)
  and
  [`/tools/validate_m5_openapi_catalog.py`](../../tools/validate_m5_openapi_catalog.py).
- Negative fixtures and CI capture:
  [`/fixtures/contracts/m5-openapi/`](../../fixtures/contracts/m5-openapi/) and
  [`/artifacts/release/captures/publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces_validation_capture.json`](../release/captures/publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces_validation_capture.json).

## Endpoints covered

The registry/mirror catalog read and install request, signed policy-bundle fetch,
user-scope settings read and write, the OIDC session front door and SCIM
provisioning identity routes, the runtime-catalog fetch, the managed AI-broker
turn route, the collaboration relay session open and review-evidence archive,
telemetry ingest, support-bundle attach, the entitlement-snapshot and
usage-export reads, the managed control-plane offboarding exit-packet assemble and
append-only destruction-receipt ledger, and the docs-pack manifest fetch. Each
operation resolves to a real path/method/operationId in
`openapi/service_api_seed.yaml` and a real surface row in
`artifacts/service/api_surface_rows.yaml`.

## Acceptance, met

- **Every claimed M5 service API or endpoint family has a checked-in OpenAPI spec,
  example pack, and lifecycle label.** All 18 operations carry an
  `openapi_document_ref`, a checked-in `example_pack_ref` under
  `examples/contracts/m5-openapi/`, and the family lifecycle label
  (`stable`, the publication matrix's effective published label for the
  `service_optional_api` family).
- **OpenAPI publication covers auth/ownership class, mutability, preview/dry-run
  support, and compatibility notes.** Each endpoint declares an
  `auth_source_class`, an `entitlement_class`, a `policy_override_posture`, a
  `mutability_posture`, a `preview_support_class`, and a `compatibility_note`; the
  validator proves the auth/offline/deprecation/sunset postures match the surface
  rows.
- **Mirror/self-host/offline packaging can ship the same specs and examples without
  live vendor services.** `offline_bundle.requires_runtime_service` is `false`; the
  catalog, the OpenAPI document, the boundary schema, the example packs, and the
  validator bundle into mirror artifact sets and validate offline.

## Guardrails honored

- **No endpoint is surfaced in docs/help/admin/export flows without the same
  published OpenAPI contract.** The SDK doc and the OpenAPI README resolve every
  surface from the catalog; the catalog is the single source consumed downstream.
- **No example pack implies broader authority than the endpoint contract allows.**
  Each example pack records the endpoint's `auth_source_class`, validates against
  the OpenAPI document's component schemas, and carries no raw credentials,
  signatures, bytes, or live service URLs (validator-enforced).

## Auto-narrowing wired into publication logic

Publishing the full OpenAPI family flips the `service_optional_api` row of the
public-contract publication matrix
([`/artifacts/contracts/m5-stability-lifecycle-map.json`](../contracts/m5-stability-lifecycle-map.json))
from `partial` to `published`, un-narrowing the family from Beta to its Stable
contract claim. The catalog and the validator are listed in that row's example,
OpenAPI, and validator-suite requirements, so if the catalog, the OpenAPI
document, the example packs, the validator, or the matrix linkage go missing or
stale, the row narrows below the launch cutline again automatically.

## Reuse, not restatement

The catalog reuses the established governance sources: the auth-source,
entitlement, policy-override, offline, deprecation, and sunset vocabularies are
drawn verbatim from `/artifacts/service/api_surface_rows.yaml`; the example payload
shapes are conformant with the component schemas in
`/openapi/service_api_seed.yaml`; and the family lifecycle label is the effective
`published_label` from `/artifacts/contracts/m5-stability-lifecycle-map.json`. The
validator cross-checks all three joins.

## Proof

Automated proof lives in
`crates/aureline-release/src/publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces/tests.rs`
and
`crates/aureline-release/tests/publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces.rs`:

- the checked-in catalog parses and validates with zero violations;
- the family publishes at the stable cutline and every endpoint inherits the
  family label;
- every endpoint binds an auth-source class, a mutability posture, and a readable
  example pack, and read-only operations never carry a request body or a preview;
- the recomputed summary agrees with the checked-in catalog;
- the model matches the frozen CI validation capture;
- negative gates reject a duplicate endpoint id, an off-vocabulary auth class, a
  widened lifecycle label, a drifted summary, and a read-only endpoint with a
  request body;
- the checked-in fixtures are rejected by the typed model.

`tools/validate_m5_openapi_catalog.py` validates the catalog against the boundary
schema, the semantic invariants, the regenerator (no hand-edit drift), the matrix
lifecycle join, every operation/method/path/auth posture against the OpenAPI
document and the surface rows, every example request/response against the
document's component schemas, the no-credential/no-URL example guard, every
referenced path, and the negative fixtures. It runs in the shared
`ci/contract_validation.sh` lane and as the dedicated `check_m5_openapi_catalog`
workflow.

## Reuse surfaces

`endpoint(...)` and `endpoints_for_surface(...)` (SDK/docs/support inspection),
`read_only_endpoints()` (mirror packaging), and `publishes_stable()` /
`computed_summary()` (claim manifests and dashboards). Part of the canonical M5
evidence train; the row narrows if its catalog, boundary schema, example packs,
validator, or proof drift.
