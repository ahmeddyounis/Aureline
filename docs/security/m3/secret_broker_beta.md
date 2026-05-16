# Secret broker, vault/keychain integration, and consumer-identity audit beta

This document is the reviewer-facing landing page for the beta projection that
delivers a handle-only secret-broker contract and a consumer-identity audit
stream across the four claimed beta profiles. It builds on the alpha
record vocabulary frozen in
[`/docs/security/secret_broker_alpha.md`](../secret_broker_alpha.md) and the
class matrix in
[`/docs/security/secret_class_matrix.md`](../secret_class_matrix.md).

The projection is owned by
[`/crates/aureline-auth/src/secret_broker/mod.rs`](../../../crates/aureline-auth/src/secret_broker/mod.rs)
and consumed by
[`/crates/aureline-shell/src/secret_broker_beta/mod.rs`](../../../crates/aureline-shell/src/secret_broker_beta/mod.rs).
The headless inspector lives at
[`/crates/aureline-shell/src/bin/aureline_shell_secret_broker_beta.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_secret_broker_beta.rs).

## What the projection covers

Every claimed beta row carries:

- **Profile** — one of `connected`, `mirror_only`, `offline`,
  `enterprise_managed`. All four profiles must have at least one claimed row.
- **Vault/keychain adapter** — `os_keychain`,
  `enterprise_vault_managed`, `enterprise_vault_self_hosted_mirror`,
  `enterprise_vault_air_gapped_snapshot`, `platform_agent`,
  `hsm_or_kms_backed`, `session_memory_cache`, or `managed_policy_injector`.
  Managed-authority adapters fail closed without a verified signature
  posture.
- **Signature posture** — `verified_live`, `verified_mirror`,
  `verified_manual_import`, `verified_air_gapped`, or
  `not_required_local_origin`.
- **Reference mode** — `handle` (default), `delegated`, or visibly degraded
  `session_only`. Raw secret bytes and runtime handle ids never travel on
  any reference mode.
- **Projection mode** — `alias_only`, `broker_callback`,
  `request_header_signer`, `ephemeral_fd`, `bounded_mount`,
  `env_var_isolated_child`, `sign_only`, `decrypt_only`, `token_exchange`,
  `policy_materialised`, or `inspect_metadata`.
- **Lifecycle state** — `live`, `pending_rotation`, `expiring`, `expired`,
  `revoked`, `paused_trust_changed`, `paused_vault_locked`, or
  `paused_vault_unavailable`. Non-live states hold managed authority
  closed.
- **Consumer identity** — `consumer_id`, `consumer_label`, and a
  `capability_hash_ref` that ties the row to the consumer's manifest.

Every projection request is logged as a
`security_secret_broker_beta_consumer_audit_record` event naming consumer,
target, workspace scope, secret class, projection mode, and a typed outcome
(`granted_handle`, `granted_delegated`, `granted_session_only`, or one of ten
typed denial reasons including `denied_by_plaintext_requested`,
`denied_by_silent_in_memory_promotion`, `denied_by_stale_handle_reuse`,
`denied_by_public_endpoint_fallback`, `denied_by_policy`,
`denied_by_trust_state`, `denied_by_lifecycle_state`,
`denied_by_missing_approval`, `denied_by_expiry`, and
`denied_by_revocation`). Support and audit packets preserve this lineage
verbatim.

## Acceptance posture

- **Handle-only projection.** Consumers receive a handle, a scoped delegated
  credential, or a visibly degraded session-only reference through an
  admitted projection mode. Beta rows never carry raw secret material, never
  admit plaintext persistence, never admit silent in-memory promotion, never
  admit stale handle reuse, and never admit undeclared public-endpoint
  fallback.
- **Consumer-identity audit.** The broker can explain which consumer, target,
  workspace scope, secret class, and projection mode requested a secret
  projection and what the typed outcome was. The audit stream is preserved
  verbatim across `connected`, `mirror_only`, `offline`, and
  `enterprise_managed` profiles.
- **Lineage-preserved support and audit packets.**
  [`SecretBrokerBetaSupportExport`](../../../crates/aureline-auth/src/secret_broker/mod.rs)
  carries the full page plus a defect-kind roll-up. Raw secret values and
  raw runtime handle ids are excluded; vault adapter, signature posture,
  projection mode, consumer lineage, target/scope refs, and typed audit
  outcomes are preserved.
- **Fail-closed managed authority.** When a managed-authority vault adapter
  loses its verified signature posture (mirror outage, expired bundle,
  air-gapped snapshot past its `valid_until`), the row's lifecycle state
  fails closed and the validator emits
  `managed_authority_missing_signature`. Local editing continues.

| Profile               | Authority source                                                            |
| --------------------- | --------------------------------------------------------------------------- |
| `connected`           | OS credential store or live enterprise vault available.                    |
| `mirror_only`         | Signed enterprise-vault mirror is the only authority.                       |
| `offline`             | Air-gapped enterprise vault snapshot.                                       |
| `enterprise_managed`  | Managed policy injector materialises narrow authority per call.             |

## Failure-mode drills

The headless inspector emits typed defects for four deliberate failure
modes. The drill fixtures live under
[`/fixtures/security/m3/secret_broker/`](../../../fixtures/security/m3/secret_broker/):

- `drill_raw_secret_material.json` — a row claims raw secret material is
  present; the validator surfaces `raw_secret_material_present`.
- `drill_managed_authority_missing_signature.json` — a managed-authority
  vault adapter loses its verified signature posture; the validator surfaces
  `managed_authority_missing_signature`.
- `drill_consumer_audit_missing.json` — a row is left without any consumer
  audit events; the validator surfaces `consumer_audit_missing`.
- `drill_denied_audit_missing_reason.json` — a denied audit event drops its
  denial note; the validator surfaces `denied_audit_missing_reason`.

## Headless inspector commands

```sh
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- page
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- handle-rows
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- consumer-audit
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- defects
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- summary
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- validate
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- drill-raw-secret-material
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- drill-managed-authority-missing-signature
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- drill-consumer-audit-missing
cargo run -q -p aureline-shell --bin aureline_shell_secret_broker_beta -- drill-denied-audit-missing-reason
```

## Stable contract refs

- Record kind: `security_secret_broker_beta_page_record`
- Shared contract ref: `security:secret_broker_beta:v1`
- Source matrix:
  [`/artifacts/security/m3/secret_broker/secret_broker_matrix.yaml`](../../../artifacts/security/m3/secret_broker/secret_broker_matrix.yaml)
- Baseline support export:
  [`/artifacts/security/m3/secret_broker/baseline_support_export.json`](../../../artifacts/security/m3/secret_broker/baseline_support_export.json)
- Schema:
  [`/schemas/security/secret_handle.schema.json`](../../../schemas/security/secret_handle.schema.json)

## Support playbook reuse

Support bundles, admin console renderings, reviewer fixtures, and the
auditing surface share the same defect-kind vocabulary. A defect emitted by
the headless inspector is identical (record kind, kind token, subject id,
field, note) to the defect admin and support surfaces would report. Support
playbooks therefore reference the same exported vocabulary as the auditing
surface — there is no parallel admin-only language for secret handling.
