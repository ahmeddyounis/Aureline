# Fixtures: publish-enterprise-self-hosted-and-air-gapped-docs

Reference fixtures for the enterprise, self-hosted, and air-gapped docs,
matrices, and known-limits publish packet
(`policy:enterprise_docs_matrices_known_limits:v1`).

## Files

| File | Description |
| --- | --- |
| `page.json` | Seeded publish page covering all five enterprise profiles; qualifies stable with zero defects. |
| `summary.json` | Summary record from the seeded page. |
| `defects.json` | Empty defect list for the seeded page. |
| `support_export.json` | Support-export envelope wrapping the seeded page. |
| `drill_local_core_blocked_withdrawn.json` | Failure drill: `local_core_posture: blocked_by_default` triggers immediate withdrawal. |
| `drill_aspirational_proof_withdrawn.json` | Failure drill: `proof_currency: aspirational` on a sovereignty profile triggers immediate withdrawal. |
| `drill_missing_profiles_preview.json` | Failure drill: missing enterprise profiles narrow the page to preview. |
| `drill_stale_docs_beta.json` | Failure drill: `docs_state: stale` on an enterprise row narrows toward beta. |
| `drill_partial_matrix_beta.json` | Failure drill: `matrix_state: partial` on an enterprise row narrows toward beta. |
| `drill_partially_disclosed_known_limits_beta.json` | Failure drill: `known_limits_state: partially_disclosed` on an enterprise row narrows toward beta. |

## Schema

All records conform to
`schemas/enterprise/publish-enterprise-self-hosted-and-air-gapped-docs.schema.json`.

## Canonical paths

- Doc: `docs/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs.md`
- Artifact: `artifacts/enterprise/m4/publish-enterprise-self-hosted-and-air-gapped-docs.md`
- Contract ref: `policy:enterprise_docs_matrices_known_limits:v1`
- Runtime owner: `aureline_policy::publish_enterprise_self_hosted_and_air_gapped_docs_matrices_and_known_limits`
