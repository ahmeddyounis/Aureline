# Publish enterprise, self-hosted, and air-gapped docs, matrices, and known-limits with current proof

This lane makes enterprise, self-hosted, managed, and air-gapped documentation,
capability matrices, and known-limit disclosures visible and verifiable for every
claimed enterprise deployment profile. Product copy, security review, support
exports, and release packets can all answer: are the docs current for this
profile, is the capability matrix complete, are known limits fully disclosed,
is local-core continuity explicit, and is the proof current rather than
aspirational. The runtime owner is
`aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits`.

The packet does **not** re-derive raw doc artefacts, raw hostnames, raw tenant
identifiers, raw key bytes, or raw credentials. All references are opaque tokens
or export-safe labels. This packet adds the publish invariants needed for a
single evidence packet that can be ingested by dashboards, docs, Help/About
surfaces, and support exports without cloning status text.

## Contract

For the stable claim to hold, **all** of the following conditions must be
verified simultaneously:

1. **All five enterprise profiles covered** — at least one row exists for each
   of: `individual_local`, `self_hosted`, `enterprise_online`, `air_gapped`,
   and `managed_cloud`.
2. **No local-core posture blocks local-core work** — no row carries
   `local_core_posture: blocked_by_default`; enterprise features must not block
   local editing, save, search, or Git by default.
3. **Docs are current for enterprise profiles** — every non-`individual_local`
   row carries `docs_state: current`; `stale` or `missing` states narrow the row.
4. **Capability matrices are complete for enterprise profiles** — every
   non-`individual_local` row carries `matrix_state: complete`; `partial` or
   `missing` states narrow the row.
5. **Known limits are fully disclosed for enterprise profiles** — every
   non-`individual_local` row carries `known_limits_state: fully_disclosed`;
   `partially_disclosed` or `undisclosed` states narrow the row.
6. **Proof is current for sovereignty profiles** — every `self_hosted` and
   `air_gapped` row carries `proof_currency: current`; `stale` narrows to beta
   and `aspirational` withdraws the row immediately.
7. **Tenant/region ownership and policy source declared** — every
   non-`individual_local` row carries non-empty `tenant_region_owner_ref`,
   `policy_source_ref`, and `dependency_class_token`.
8. **Local-core continuity explicitly stated** — every row carries a non-empty
   `local_core_posture_token`; no row carries `local_core_posture: blocked_by_default`.

## Required behavior

`validate_enterprise_docs_matrices_known_limits_page` rejects a page when its
`defects` list is non-empty.

`audit_enterprise_docs_matrices_known_limits_page` runs the combined check and
returns a typed `Vec<EnterpriseDocsMatricesKnownLimitsDefect>`. Each defect
carries a closed `narrow_reason_token` and an export-safe `note`. The absence
of defects is the stable claim.

Two conditions force `Withdrawn` immediately and cannot be overridden:

- A row where `local_core_posture: blocked_by_default` is declared (narrow
  reason: `local_core_blocked_by_default`). Enterprise features must not block
  local-core work by default.
- A self-hosted or air-gapped row where `proof_currency: aspirational` is
  declared (narrow reason: `aspirational_proof_on_sovereign_profile`).
  Sovereignty claims require current evidence, not roadmap promises.

A missing required enterprise profile narrows to `Preview` rather than `Beta`
because the coverage gap prevents any verifiable claim for that profile.

## Enterprise profiles

| Profile token | Description |
| --- | --- |
| `individual_local` | Desktop-local, single-user, no managed control plane. Docs, matrices, and known limits are not applicable. |
| `self_hosted` | Customer-operated control plane with customer-managed keys and region. |
| `enterprise_online` | Hybrid remote-attach with vendor-provided managed services. |
| `air_gapped` | Offline-capable air-gapped mirror; no public egress. |
| `managed_cloud` | Vendor-operated SaaS with vendor-managed keys by default. |

All five profiles must be covered for a stable claim.

## Docs completeness tokens

| Token | Description |
| --- | --- |
| `current` | Docs are current and cover all claimed capabilities for this profile. |
| `stale` | Docs exist but are stale (last update outside the declared freshness window). |
| `missing` | Docs are missing for one or more claimed capability areas. |
| `not_applicable` | No enterprise docs scope exists for this profile. |

## Matrix completeness tokens

| Token | Description |
| --- | --- |
| `complete` | Matrix is complete and covers all claimed capabilities. |
| `partial` | Matrix is partially complete; some claimed capabilities lack matrix rows. |
| `missing` | Matrix is missing for this profile. |
| `not_applicable` | No enterprise matrix scope exists for this profile. |

## Known-limit completeness tokens

| Token | Description |
| --- | --- |
| `fully_disclosed` | All known limits are fully disclosed for this profile. |
| `partially_disclosed` | Some known limits are disclosed, but at least one area lacks disclosure. |
| `undisclosed` | Known limits are missing or undisclosed for this profile. |
| `not_applicable` | No enterprise known-limit scope exists for this profile. |

## Local-core continuity posture tokens

| Token | Description |
| --- | --- |
| `preserved` | The local editing floor is fully preserved for this profile. |
| `impaired_managed_dependency` | A managed dependency may degrade some capabilities, but the local editing floor is intact. |
| `blocked_by_default` | The profile blocks local-core capabilities by default. **Hard guardrail — withdraws the row.** |

## Proof currency tokens

| Token | Description |
| --- | --- |
| `current` | Proof is current and verified. |
| `stale` | Proof exists but is stale (outside the declared validity window). |
| `aspirational` | Proof is aspirational — a roadmap promise without current evidence. **Hard guardrail for sovereignty profiles — withdraws the row.** |
| `not_applicable` | No proof scope applies. |

## Seeded coverage

The seeded page covers all five profiles (5 rows total). Each row carries a
fully declared docs state, matrix state, known-limits state, local-core
continuity posture, and proof currency. The seeded page qualifies stable with
zero defects.

## Canonical paths

- Runtime owner: `aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits`
- Artifact: `artifacts/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs.md`
- Contract ref: `policy:enterprise_docs_matrices_known_limits:v1`
- Fixtures: `fixtures/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs/`
- Schema: `schemas/enterprise/publish-enterprise-self-hosted-and-air-gapped-docs.schema.json`
