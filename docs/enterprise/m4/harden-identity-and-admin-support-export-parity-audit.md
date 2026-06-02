# Harden identity and admin support-export parity — audit-safe redaction and no-vendor-control-plane local governance paths

This lane makes every enterprise identity and admin surface — directory/provider
 cards, user/seat lifecycle, provisioning failures, policy-target dry runs, and
local governance paths — visible and verifiable enough that product, security
review, support export, and admin surfaces can all explain: which provisioning
class is active (OIDC, SCIM, signed file bundle, manual), what sync freshness
applies, what fallback or manual path exists, what remains local versus tenant-
scoped, and whether admin action/result lineage is recorded. The runtime owner
is `aureline_policy::harden_identity_and_admin_support_export_parity_audit`.

The packet does **not** re-derive raw tenant data, raw user emails, raw
provisioning payloads, raw policy bundle bodies, or raw credential material. The
upstream `aureline_auth::provisioning` beta audit remains canonical for its own
slice. This packet re-exports those qualification tokens verbatim and adds the
identity-admin hardening invariants needed for a single evidence packet.

## Contract

For the stable claim to hold, **all nine** of the following conditions must be
verified simultaneously:

1. **All five row classes covered** — at least one row exists for each of:
   `directory_provider_card`, `user_seat_lifecycle`, `provisioning_failure_log`,
   `policy_target_dry_run`, and `local_governance_path`.
2. **Raw secret material excluded** — every row carries
   `raw_secret_or_private_material_excluded: true`; no credential bodies,
   private keys, tenant raw data, or user emails cross this boundary.
3. **Provisioning class declared** — rows requiring a provisioning class carry a
   non-empty token from the closed vocabulary (`oidc`, `scim`,
   `signed_file_bundle`, `manual`).
4. **Sync freshness declared** — every row carries a non-empty
   `sync_freshness_token` from the closed vocabulary (`live`, `cached`, `stale`,
   `expired`, `missing`).
5. **Local vs tenant scope declared** — every row carries a non-empty
   `local_tenant_scope_token` explaining what remains local versus tenant-scoped.
6. **Specific failure kinds** — provisioning failure and applicable dry-run rows
   that carry a failure kind use a specific [`ProvisioningFailureKind`] token
   (`provider_outage`, `auth_drift`, `scope_mismatch`, `seat_loss`,
   `deprovisioning_impact`) rather than generic error copy.
7. **Local-core continuity explicit** — `local_governance_path` rows carry
   `local_core_continuity_explicit: true`.
8. **Fallback/manual path declared** — `directory_provider_card` rows carry a
   non-empty `fallback_manual_path_label`.
9. **Action lineage present** — rows requiring admin action/result lineage carry
   an [`AdminActionLineage`] block with action token, result token, actor ref,
   applied-at timestamp, and transaction id.

## Required behavior

`validate_harden_identity_admin_page` rejects a page when its `defects` list is
non-empty.

`audit_harden_identity_admin_page` runs the combined check and returns a typed
`Vec<HardenIdentityAdminDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A row with `raw_secret_or_private_material_excluded: false` (narrow reason:
  `raw_secret_or_private_material_exposed`). The function returns immediately
  with this single defect and skips all other checks.

A missing required row class narrows to `Preview` rather than `Beta` because the
coverage gap prevents any verifiable claim for that row class.

An empty `provisioning_class_token` on a required row narrows to `Beta`.

An empty `sync_freshness_token` on any row narrows to `Beta`.

A missing `local_tenant_scope_token` on any row narrows to `Beta`.

A generic or unknown failure kind token on a failure row narrows to `Beta`.

A missing `local_core_continuity_explicit` on a `local_governance_path` row
narrows to `Beta`.

A missing `fallback_manual_path_label` on a `directory_provider_card` row narrows
to `Beta`.

A missing `action_lineage` on a row that requires one narrows to `Beta`.

## Row classes

| Row class token | Description |
| --- | --- |
| `directory_provider_card` | Directory and provider card naming the identity provider, directory source, and provisioning class. |
| `user_seat_lifecycle` | User and seat lifecycle row covering creation, transfer, suspension, reactivation, and deprovision flows. |
| `provisioning_failure_log` | Provisioning failure log that distinguishes failure kinds rather than collapsing into generic admin error copy. |
| `policy_target_dry_run` | Policy-target dry run result showing what would change before commit. |
| `local_governance_path` | Local governance path row documenting no-vendor-control-plane fallback and local-artifact safety. |

## Provisioning classes

| Token | Description |
| --- | --- |
| `oidc` | OIDC-based identity provisioning and authentication. |
| `scim` | SCIM-based lifecycle provisioning. |
| `signed_file_bundle` | Signed file bundle import providing org lifecycle state. |
| `manual` | Manual admin action or file-based setup. |

## Sync freshness states

| Token | Description |
| --- | --- |
| `live` | Live sync within the accepted freshness window. |
| `cached` | Cached sync that may be slightly behind live. |
| `stale` | Stale sync past the acceptable window. |
| `expired` | Expired sync; the last known good is past its validity. |
| `missing` | Missing sync; no source has been successfully contacted. |

## Local vs tenant scope

| Token | Description |
| --- | --- |
| `local_state_only` | The row covers only local device state; no tenant-scoped data is involved. |
| `tenant_scoped` | The row covers only tenant-scoped state; local artifacts are not affected. |
| `hybrid_local_tenant` | The row covers both local and tenant state; the exact boundary is explained in the row's `local_artifact_safety_note`. |

## Provisioning failure kinds

| Token | Description |
| --- | --- |
| `provider_outage` | The identity provider or provisioning endpoint is unreachable or returning errors. |
| `auth_drift` | The local auth state has drifted from the provider's expected state. |
| `scope_mismatch` | The requested action exceeds the granted scope or entitlement. |
| `seat_loss` | A seat was removed, transferred, or otherwise lost. |
| `deprovisioning_impact` | The failure occurred during a deprovisioning or offboarding flow. |

## Local-only fallback

Enterprise-bearing rows (`directory_provider_card`, `user_seat_lifecycle`,
`provisioning_failure_log`) must explicitly declare a `fallback_manual_path_label`
— the path that takes over when the primary managed endpoint is unavailable —
and a human-readable `local_artifact_safety_note` describing what remains local.
This prevents silent fail-open behavior by requiring explicit documentation of
the continuity path.

## Boundary

The following material stays outside this packet's support boundary:

- Raw tenant names, raw user emails, raw subject identifiers.
- Raw provisioning payloads, raw SCIM responses, raw OIDC tokens.
- Raw policy bundle bodies, raw rule text, raw admin policy configurations.
- Raw credential tokens, bearer tokens, or session secrets.
- Raw signature blobs, private keys, or certificate material.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, a count, or a schema-version integer.

## Truth source

The seeded proof packet is `seeded_harden_identity_admin_page()` in
[`/crates/aureline-policy/src/harden_identity_and_admin_support_export_parity_audit/mod.rs`](../../../crates/aureline-policy/src/harden_identity_and_admin_support_export_parity_audit/mod.rs).

That function is the single inspectable record for this lane. Dashboards,
Help/About surfaces, and support exports should ingest it rather than cloning
status text or maintaining parallel identity-admin checks.

## Canonical paths

- Runtime owner: `aureline_policy::harden_identity_and_admin_support_export_parity_audit`
- Artifact: `artifacts/enterprise/m4/harden-identity-and-admin-support-export-parity-audit.md`
- Fixtures: `fixtures/enterprise/m4/harden-identity-and-admin-support-export-parity-audit/`
- Schema: `schemas/enterprise/harden-identity-and-admin-support-export-parity-audit.schema.json`
