# M5 OpenAPI contract index

This directory is the M5 publication index for the **OpenAPI family** contract
form. The canonical optional-service API spec lives at
[`openapi/service_api_seed.yaml`](../service_api_seed.yaml), and the
machine-readable publication index that turns it into a contract family is the
**M5 OpenAPI publication catalog**
([`artifacts/contracts/m5-openapi-catalog.json`](../../artifacts/contracts/m5-openapi-catalog.json),
human index at
[`docs/sdk/m5-service-api-catalog.md`](../../docs/sdk/m5-service-api-catalog.md)).

The `service_optional_api` row of the M5 public-contract matrix
(`artifacts/contracts/m5-stability-lifecycle-map.json`,
`artifacts/contracts/m5-public-contract-matrix.md`) records the publication
requirements this contract form must satisfy before it can carry a Stable
contract claim: a JSON Schema home, the OpenAPI spec, a Markdown summary, example
payloads, migration notes, and a validator suite.

## What the OpenAPI publication catalog adds

The catalog binds every OpenAPI operation in the document — registry/mirror,
marketplace publication, identity, AI broker, collaboration relay, telemetry
ingest, support export, usage/metering export, managed control-plane offboarding,
and docs-pack routes — to a lifecycle label, an auth-source class, an entitlement
and policy-override posture, a mutability posture, a preview/dry-run support
class, an offline/cache behaviour, a deprecation lane and sunset posture, a
compatibility note, and a checked-in example request/response pack under
[`examples/contracts/m5-openapi/`](../../examples/contracts/m5-openapi/). The
auth, offline, deprecation, and sunset postures are drawn verbatim from the
optional-service API surface rows
(`artifacts/service/api_surface_rows.yaml`), and the per-endpoint example packs
validate against the OpenAPI document's component schemas, so self-hosted,
mirrored, enterprise, and support tooling can reason about the same contract
without reading server code.

With the full OpenAPI family published — the OpenAPI document, the publication
catalog, the per-endpoint example packs, the boundary schema, the validator, and
the migration notes — the matrix records the OpenAPI publication state as
`published` and the `service_optional_api` family holds its Stable contract
claim. If the catalog, the OpenAPI document, the example packs, the validator, or
the matrix linkage go missing or stale, the row narrows below the launch cutline
again automatically.

## Mirror and offline use

The catalog, the OpenAPI document, its boundary schema
(`schemas/public/m5-contracts/m5_openapi_catalog.schema.json`), the per-endpoint
example packs, and the validator
(`tools/validate_m5_openapi_catalog.py`) bundle into offline/mirror artifact sets
and validate without live vendor service access; no example pack carries raw
credentials, signatures, bytes, or live service URLs.

The compatibility-surface row for this contract is `service.optional_api_family`
(`artifacts/governance/compatibility_surfaces.yaml`) and the qualification row is
`compat_row:provider.service_api_and_browser_handoff`
(`artifacts/compat/qualification_matrix_seed.yaml`).
