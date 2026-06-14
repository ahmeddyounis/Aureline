# API-collection, contract-source, request-origin, and persisted-operation matrix

## Scope

This document describes the canonical matrix that freezes the vocabulary and
states for operation collections, request lists, contract-source badges, schema
freshness, request origins, persisted-operation bindings, and retention modes
across the API request-workspace surfaces. The matrix covers REST, GraphQL, and
plugin-owned contract rows; local, remote, container, managed, and
browser-companion origins; and collection and history retention classes with
offline and mirror behavior.

## Truth sources

- Implementation: `crates/aureline-api/src/freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix/mod.rs`
- Schema: `schemas/data/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.schema.json`
- Checked-in packet: `artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json`
- Fixtures: `fixtures/data/m5/freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix/`

## Locked vocabulary

| Term | Family | Meaning |
|---|---|---|
| `live_contract` | source / freshness | Contract fetched live from the target and current as of the last send. |
| `cached_schema` | source / freshness | Schema served from a previous fetch and within its freshness window. |
| `schema_stale` | freshness | Cached schema is stale and explicitly labeled as such. |
| `imported_snapshot` | source / freshness | Contract loaded from a file or workspace artifact, not live truth. |
| `plugin_provided` | source | Contract owned and supplied by an extension. |
| `contract_unavailable` | source / freshness | No contract truth is available for the target. |
| `origin_changed` | origin drift | Request origin changed since it was last resolved. |
| `persisted_operation_drift` | persisted binding | Persisted-operation id no longer matches the operation text. |

## Consumer surfaces

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Request workspace collections and badges | stable | stable | Workspaces show collections, the request list, contract-source and freshness badges, origin, persisted-operation binding, and retention posture. |
| CLI and headless request output | stable | stable | CLI/headless runs print contract source, freshness, origin, and persisted-operation state so a green exit code never hides drift. |
| Diagnostics contract and origin drift | stable | stable | Diagnostics report stale schema, contract-unavailable, origin-changed, and persisted-operation drift as first-class findings. |
| Support export matrix bundle | stable | stable | Support exports carry matrix truth with metadata-only history retention. |
| Certification scorecard matrix row | stable | stable | The scorecard ingests the matrix and narrows API-client maturity rows when evidence is missing or stale. |

## Contract rows

| Contract | Kind | Source | Freshness | Origin | Persisted op |
|---|---|---|---|---|---|
| REST users | rest | live_contract | live_contract | remote | — |
| REST billing | rest | cached_schema | cached_schema | container | — |
| GraphQL core | graphql | live_contract | live_contract | remote | bound_current |
| GraphQL orders | graphql | cached_schema | schema_stale | managed | drift |
| GraphQL partner | graphql | imported_snapshot | imported_snapshot | local | — |
| Plugin gateway | plugin_owned | plugin_provided | cached_schema | container | — |
| Plugin (none) | plugin_owned | contract_unavailable | contract_unavailable | browser_companion | — |

## Request origins

| Origin | Kind | Drift | Inherits local trust | Trust class |
|---|---|---|---|---|
| Localhost loopback | local_host | origin_stable | yes | — |
| Remote api host | remote | origin_stable | no | — |
| Compose service name | container | origin_stable | no | — |
| Managed workspace endpoint | managed | origin_stable | no | `trust:managed_service` |
| Browser companion private DNS | browser_companion | origin_changed | no | `trust:browser_companion` |

Localhost, service names, and private DNS stay explicit and never silently
retarget between local, remote, container, managed, and browser-companion paths.
Browser-companion and managed origins never inherit desktop-local trust or
naming.

## Retention classes

| Class | Scope | Mode | Excludes bodies/headers by default | Offline/mirror |
|---|---|---|---|---|
| Collection text-first | collection | text_first_versioned | yes | mirror_maintained |
| History metadata | history | metadata_only | yes | mirror_maintained |
| History redacted replay | history | redacted_replayable | yes | offline_degraded |
| History full capture | history | opt_in_full_capture | yes | no_mirror |

Request files stay text-first and versionable. Request history never retains
raw bodies or headers by default; full body and header capture is available
only behind an explicit opt-in and is never the default for compare UX.

## Downgrade and claim-narrowing rules

- All promoted surfaces have `downgrade_if_missing: true`; missing proof on a
  stable claim narrows the surface to `preview` rather than inheriting a generic
  label.
- `ApiMatrixQualificationPacket::narrowing_contract_ids` returns contracts whose
  freshness is `schema_stale` or `contract_unavailable`; any claim that depends
  on those contracts being live must narrow.
- `persisted_operation_drift_ids` and `changed_origin_ids` surface drift that
  feeds diagnostics and downstream narrowing.

## Guardrails

- Request history does not move toward unsafe body/header retention by default
  to support compare UX.
- Persisted-operation or schema drift never silently falls back to raw request
  execution; drift blocks raw fallback until reviewed.
- Browser-companion and managed origins never inherit desktop-local trust or
  naming assumptions.
- API-client maturity claims always carry a named origin and contract-freshness
  story.
- Persisted-operation bindings and raw local text stay distinct truth objects.

## Redaction and privacy

- The matrix never includes raw endpoint URLs, raw secrets, raw request bodies,
  raw headers, or raw schema payloads. Rows carry stable IDs, closed posture
  vocabularies, opaque refs, and reviewable summaries only.
