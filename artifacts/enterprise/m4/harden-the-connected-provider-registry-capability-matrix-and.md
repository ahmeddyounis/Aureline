# Connected-Provider Registry Hardening — Stable Packet

- Packet: `remote:provider_registry:default`
- Schema version: `1`
- Contract ref: `remote:provider_registry_harden:v1`
- Qualification: `stable` (derived, not asserted)
- Audit defects: 0
- Withdrawn rows: 0
- Stable rows: all (9)

## Pair coverage

| Provider family | Actor identity | Callback path | Dependency | Snapshot | Local-core | Object support |
|---|---|---|---|---|---|---|
| `code_host` | `human_account` | `public_saas` | `network` | `fresh` | ✓ | PR, branch |
| `code_host` | `installation_grant` | `public_saas` | `network` | `fresh` | ✓ | PR (mutate), branch (inspect) |
| `code_host` | `delegated_credential` | `public_saas` | `network` | `fresh` | ✓ | PR (inspect) |
| `issue_tracker` | `human_account` | `public_saas` | `network` | `fresh` | ✓ | issue/work item |
| `issue_tracker` | `installation_grant` | `public_saas` | `network` | `fresh` | ✓ | issue/work item (publish-later) |
| `issue_tracker` | `delegated_credential` | `public_saas` | `network` | `fresh` | ✓ | issue/work item (inspect) |
| `ci_checks` | `human_account` | `public_saas` | `network` | `fresh` | ✓ | check run, pipeline run (browser handoff) |
| `ci_checks` | `installation_grant` | `public_saas` | `network` | `fresh` | ✓ | check run, pipeline run (publish-later) |
| `ci_checks` | `delegated_credential` | `public_saas` | `network` | `fresh` | ✓ | check run, pipeline run (inspect) |

## Key invariants verified

1. All nine required (provider family, actor identity) pairs are covered by typed `ProviderDescriptorRecord` entries.
2. No raw private material is exposed on any descriptor (`raw_private_material_excluded: true` on all records).
3. Every descriptor explicitly declares `local_core_continuity_allowed: true`; local editing is never blocked by provider-connectivity failures.
4. Every descriptor carries an explicit `dependency_class_token`.
5. Every descriptor names a `callback_path_token` for the ingress/callback path.
6. Every descriptor covers ≥1 object kind with explicit mutation posture.

## Hard guardrail — withdrawal condition

The following forces `Withdrawn` immediately and cannot be overridden:

- Any descriptor with `raw_private_material_excluded: false`
  (narrow reason: `raw_private_material_present`).

## Failure / recovery drill coverage

| Scenario | Pair(s) | Expected behavior |
|---|---|---|
| Missing required pair | any of the 9 | Narrows to `Preview`; `required_pair_missing` defect for each absent pair |
| Raw credential on descriptor | any | Immediate `Withdrawn` qualification; `raw_private_material_present` defect |
| No local-core continuity | any descriptor | Narrows to `Beta`; `local_core_continuity_missing` defect |
| Empty object support | any descriptor | Narrows to `Beta`; `object_support_missing` defect |
| Stale-expired snapshot | any descriptor | Snapshot `is_usable()` returns false; descriptor usability degraded |

## Canonical paths

- Doc: `docs/enterprise/m4/harden-the-connected-provider-registry-capability-matrix-and.md`
- Runtime owner: `aureline_remote::harden_the_connected_provider_registry_capability_matrix_and`
- Fixtures: `fixtures/enterprise/m4/harden-the-connected-provider-registry-capability-matrix-and/`
- Schema: `schemas/enterprise/harden-the-connected-provider-registry-capability-matrix-and.schema.json`
