# Secret-class matrix (storage, reveal, audit, and export-denial)

This document publishes the **secret-class matrix** that every Aureline surface
uses to store, reference, reveal, audit, and *deny export* of secret-bearing
material. The goal is one shared policy vocabulary so auth, providers, remote
connectors, registries, databases, signing, support bundles, and portable
profiles do not invent conflicting “token handling” rules.

Authoritative design anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.11 and Appendix BT
  (secret broker + class matrix).
- `.t2/docs/Aureline_PRD.md` §5.21 and the secrets/supportability requirements
  (exports omit secrets by default).

Companion artifacts:

- [`/artifacts/security/secret_classes.yaml`](../../artifacts/security/secret_classes.yaml)
  — machine-readable class rows (canonical values).
- [`/schemas/security/secret_class_rows.schema.json`](../../schemas/security/secret_class_rows.schema.json)
  — boundary schema for the frozen vocabulary.
- [`/artifacts/security/redaction_posture_matrix.yaml`](../../artifacts/security/redaction_posture_matrix.yaml)
  — per-surface redaction defaults (logs, traces, support bundles, profile export, etc).
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — full secret-broker contract (handles, projection modes, denial reasons, audit events).

Downstream consumers this matrix must stay consistent with:

- [`/docs/auth/credential_picker_contract.md`](../auth/credential_picker_contract.md) and
  [`/docs/auth/credential_state_and_secret_prompt_contract.md`](../auth/credential_state_and_secret_prompt_contract.md)
  (credential rows, picker, and prompts).
- [`/docs/support/diagnostic_artifact_matrix.md`](../support/diagnostic_artifact_matrix.md)
  and [`/artifacts/support/support_evidence_pack_matrix.yaml`](../../artifacts/support/support_evidence_pack_matrix.yaml)
  (support bundle inclusion and prohibited classes).
- [`/docs/profile/profile_sync_and_conflict_contract.md`](../profile/profile_sync_and_conflict_contract.md)
  (portable profiles are non-widening and alias/handle-only for secrets).
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  (policy may narrow projection/reveal/export; never widen).
- [`/docs/security/threat_model_and_audit_stream_contract.md`](./threat_model_and_audit_stream_contract.md)
  (audit export is metadata-only and class-labelled).

Out of scope: implementing a secret broker, keychain integration, vault adapters,
or a policy engine. This is the contract those implementations must satisfy.

## Principles (frozen)

1. **Raw secret material is exceptional.** Every durable and exportable surface
   (profiles, recipes, logs, traces, support bundles, audit exports) carries
   *aliases/handles and posture metadata*, not raw values.
2. **Storage authority is declared.** Every secret class names which trust-store
   authorities may hold it, and which fallback (if any) is admissible.
3. **Reveal is explicit, narrow, and audited.** When reveal exists at all it is
   per-handle, user initiated, time-bounded, and emits audit events; bulk reveal
   is forbidden.
4. **Export is mechanically decidable.** Support bundles and portable profile
   exports decide inclusion/omission based on `export_posture` for the secret
   class plus the per-surface redaction matrix.
5. **Display posture is consistent.** UI, CLI, admin, and support surfaces all
   render the same safe identifiers (class + alias + store/source posture) and
   none render raw secret bytes by default.

## Secret classes (published)

The canonical rows live in
[`/artifacts/security/secret_classes.yaml`](../../artifacts/security/secret_classes.yaml).
This table is a reviewer-oriented summary; when in doubt, the YAML rows control.

| Secret class | Storage authority (preferred) | Default consumption posture | Raw reveal default | Portable profile export | Support bundle export | Audit minimum (examples) |
|---|---|---|---|---|---|---|
| `ai_provider_token` | enterprise vault / OS keychain | alias/handle + broker projection | denied by default; admitted only for protocol-required header | alias-only | metadata-only | issue/revoke, projection use/fail, (if used) reveal start/end |
| `code_host_token` | enterprise vault / OS keychain | alias/handle + broker projection | denied by default; admitted only for protocol-required header | alias-only | metadata-only | issue/revoke, projection use/fail, (if used) reveal start/end |
| `package_registry_token` | enterprise vault / OS keychain | alias/handle + broker projection | denied by default; admitted only for protocol-required header | alias-only | metadata-only | issue/revoke, projection use/fail |
| `database_credential` | enterprise vault / OS keychain | connection-scoped projection | denied | alias-only | metadata-only | issue/revoke, projection use/fail, denial |
| `ssh_key_material` | platform agent / OS keychain / HSM/KMS | sign/decrypt projection; no private-byte export | metadata reveal only (fingerprint/public key) | alias-only | metadata-only | issue/revoke, projection use/fail, trust-store lock/unlock |
| `client_certificate` | platform agent / OS keychain / HSM/KMS | sign/decrypt projection; no private-byte export | metadata reveal only (subject/fingerprint/expiry) | alias-only | metadata-only | issue/revoke, projection use/fail |
| `signing_key_material` | HSM/KMS-backed only | service-mediated sign/decrypt only | metadata reveal only (proof/signature), never key bytes | excluded | proof/signature only | issue/revoke, projection use/fail, rotation outcomes |
| `provider_session` | OS keychain / enterprise vault | session store + token exchange | denied | alias-only | metadata-only | issue/renew/expire/revoke, rotation outcomes, denial |
| `device_secret` | platform agent / OS keychain / HSM/KMS | platform-mediated sign-only | denied | excluded | metadata-only | issue/revoke, projection use/fail, denial |
| `ephemeral_operation_token` | session-only cache / policy injector | operation-scoped, expiring handle | denied | excluded | metadata-only | issue/expire, projection use, denial |

## Mechanical export rules (published)

Support bundle export and portable profile export MUST be able to answer these
questions *without* inspecting raw material:

1. **May this class appear at all?**
   - Use `export_posture.*_inclusion` on the class row.
2. **If it may appear, what representation is allowed?**
   - `metadata_only` means class + alias + posture + audit refs only.
   - `alias_only` means portable exports may carry alias references and posture
     metadata but not handle ids or values.
   - signature/proof-only modes mean “carry the proof artifact, not the key”.
3. **Which redaction pass applies?**
   - Use the surface’s `redaction_class` in
     [`/artifacts/security/redaction_posture_matrix.yaml`](../../artifacts/security/redaction_posture_matrix.yaml)
     (for example `support_bundle`, `profile_export`, `logs_local`).

Rules:

- A surface MUST NOT “upgrade” a secret class representation (e.g. from
  alias-only to raw) without an explicit, audited, narrowing-only policy rule.
- A surface MUST be able to prove absence: omissions are recorded with class
  labels and denial reasons rather than pretending the secret never existed.

## Display posture (published)

To keep behavior consistent across UI, CLI, admin, and support:

- **User-facing lists (credential pickers, settings, inspectors):** show alias,
  class, store/source posture, scope, expiry, and audit refs; never show raw
  bytes; do not require handle id display.
- **Portable artifacts (profiles, recipes, automation manifests):** alias-only
  or excluded per class; never raw values; never durable handle ids as a
  substitute for a secret.
- **Support bundles and audit exports:** metadata-only; handle ids may be
  omitted or hashed per the redaction matrix; raw values are forbidden.

## Example coverage

Worked examples live under:
[`/fixtures/security/secret_class_examples/`](../../fixtures/security/secret_class_examples/).
They focus on the states reviewers most often need to reason about:

- session-only fallback when a secure store is unavailable;
- enterprise vault-backed storage authority;
- delegated identity / policy-injected authority; and
- explicit policy denial of raw projection/reveal.

