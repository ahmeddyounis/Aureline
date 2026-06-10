# M5 Request-Workspace Documents, Environment Sets, and Auth-Source Inspectors Artifact Companion

This file is the artifact-level companion document for the checked-in M5 request-workspace qualification packet.

- **Canonical JSON**: `artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json`
- **Schema**: `schemas/data/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.schema.json`
- **Typed consumer**: `crates/aureline-api/src/materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors/mod.rs`

The packet is the single source of truth for M5 request-workspace depth lane qualification. All downstream surfaces ingest it directly.

## Object Coverage

| Object | Current proof |
| --- | --- |
| Request workspace document | Four document cards expose HTTP and GraphQL kinds, versioned identity, method/path, header/body refs, variable/assertion refs, diffability, and CLI reusability. |
| Environment set | Four environment sets show layered variables, base URL refs, secret-handle refs, effective fingerprints, and previewability without raw-secret export. |
| Auth-source inspector | Eight auth-source rows cover no-auth, secret-broker, delegated identity, policy-injected, managed service, mTLS, imported no-live, and policy-blocked modes with visible provenance. |
| Effective-request inspector | Two inspector rows show document values, workspace defaults, policy injection, ad hoc overrides, and secret handles before send with export-safe provenance. |
| Schema snapshot | Four snapshot rows cover OpenAPI and GraphQL introspection sources with digest, freshness, stale labeling, and live-truth guard. |

## Surface Qualification

| Surface | Claim | Displayed | Rationale |
| --- | --- | --- | --- |
| Request editor | Stable | Stable | Document kind, endpoint identity, auth source, environment layers, write posture, and schema freshness are visible. |
| Environment picker | Stable | Stable | Layered variables, base URL, secret-handle refs, and effective fingerprint are shown without exposing raw secrets. |
| Auth inspector | Stable | Stable | Strategy kind, broker handles, and provenance are visible without exposing raw credential material. |
| Send bar | Stable | Stable | Effective-request inspector visibility and write-posture disclosure are required before execution. |
| History row | Preview | Preview | Local-first and redactable but does not yet show full environment or auth-source detail in preview. |
| Export review | Stable | Stable | Redaction profile, safe-preview class, and provenance are shown before any portable handoff. |
| Schema inspector | Preview | Preview | Snapshot digest, source kind, and freshness are shown but is still below stable pending full UI parity. |
| Effective request inspector | Stable | Stable | Value sources (request file, workspace default, policy, ad hoc, secret broker) are shown before send. |

## Guardrails

- Raw endpoint URLs, raw secrets, raw credential bodies, and raw cookie or token values do not appear in this packet.
- Environment sets exclude raw secrets from portable export by default.
- Auth-source inspectors show broker handles and provenance, never raw secret material.
- Stale schema snapshots are visibly labeled and may not masquerade as live truth.
- Policy-blocked operations carry an explicit reason and remain non-executable until step-up is satisfied.

## Known Limits

This packet qualifies versioned request-workspace documents, environment sets, and auth-source inspectors for promoted M5 surfaces. It does not ship a live request runner, and imported request artifacts stay inspect-only unless they carry recoverable document, environment, and auth-source lineage.
