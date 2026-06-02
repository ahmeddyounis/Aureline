# Enterprise, Self-Hosted, and Air-Gapped Docs, Matrices, and Known-Limits — Publish Packet

- Packet: `policy:enterprise-docs-matrices-known-limits:seeded:0001`
- Schema version: `1`
- Contract ref: `policy:enterprise_docs_matrices_known_limits:v1`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all 5

## Lane coverage

| Row | Enterprise profile | Docs state | Matrix state | Known limits | Proof currency | Local-core posture |
|---|---|---|---|---|---|---|
| `enterprise-docs-matrices-known-limits:individual_local` | `individual_local` | `not_applicable` | `not_applicable` | `not_applicable` | `not_applicable` | `preserved` |
| `enterprise-docs-matrices-known-limits:self_hosted` | `self_hosted` | `current` | `complete` | `fully_disclosed` | `current` | `preserved` |
| `enterprise-docs-matrices-known-limits:enterprise_online` | `enterprise_online` | `current` | `complete` | `fully_disclosed` | `not_applicable` | `preserved` |
| `enterprise-docs-matrices-known-limits:air_gapped` | `air_gapped` | `current` | `complete` | `fully_disclosed` | `current` | `preserved` |
| `enterprise-docs-matrices-known-limits:managed_cloud` | `managed_cloud` | `current` | `complete` | `fully_disclosed` | `not_applicable` | `preserved` |

## Key invariants verified

1. All five required enterprise profiles (`individual_local`, `self_hosted`,
   `enterprise_online`, `air_gapped`, `managed_cloud`) have rows.
2. No row carries `local_core_posture: blocked_by_default`; the hard guardrail
   is clean.
3. Every non-`individual_local` row carries `docs_state: current` with a
   declared freshness window.
4. Every non-`individual_local` row carries `matrix_state: complete` with a
   published matrix artefact ref.
5. Every non-`individual_local` row carries `known_limits_state: fully_disclosed`
   with a known-limit index ref.
6. Every sovereignty profile (`self_hosted`, `air_gapped`) carries
   `proof_currency: current` with a verified proof packet ref.
7. Every non-`individual_local` row carries non-empty `tenant_region_owner_ref`,
   `policy_source_ref`, and `dependency_class_token`.
8. Every row carries `local_core_posture: preserved`, making the local-editing
   floor explicit on every profile.

## Hard guardrails — withdrawal conditions

Two conditions force `Withdrawn` immediately and cannot be overridden:

- A row where `local_core_posture: blocked_by_default` is declared (narrow
  reason: `local_core_blocked_by_default`). Enterprise features must not block
  local-core work by default.
- A self-hosted or air-gapped row where `proof_currency: aspirational` is
  declared (narrow reason: `aspirational_proof_on_sovereign_profile`).
  Sovereignty claims require current evidence, not roadmap promises.

## Canonical paths

- Doc: `docs/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs.md`
- Runtime owner: `aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits`
- Fixtures: `fixtures/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs/`
- Schema: `schemas/enterprise/publish-enterprise-self-hosted-and-air-gapped-docs.schema.json`
