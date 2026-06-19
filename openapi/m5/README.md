# M5 OpenAPI contract index

This directory is the M5 publication index for the **OpenAPI family** contract
form. The canonical optional-service API spec lives at
[`openapi/service_api_seed.yaml`](../service_api_seed.yaml).

The `service_optional_api` row of the M5 public-contract matrix
(`artifacts/contracts/m5-stability-lifecycle-map.json`,
`artifacts/contracts/m5-public-contract-matrix.md`) records the publication
requirements this contract form must satisfy before it can carry a Stable
contract claim: a JSON Schema home, the OpenAPI spec, a Markdown summary, example
payloads, migration notes, and a validator suite.

The OpenAPI family is currently published as a **seed** (a single spec), so the
matrix records the OpenAPI publication state as `partial` and narrows the
`service_optional_api` family to Beta until the full OpenAPI family is published.
That narrowing is the auto-downgrade behaviour the matrix freezes: a family whose
required contract form is only partially published may not hold a Stable contract
claim.

The compatibility-surface row for this contract is `service.optional_api_family`
(`artifacts/governance/compatibility_surfaces.yaml`) and the qualification row is
`compat_row:provider.service_api_and_browser_handoff`
(`artifacts/compat/qualification_matrix_seed.yaml`).
