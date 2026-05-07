# Key hierarchy, tenant encryption, and signing-authority contract

This document turns managed-service, sovereignty, and “tenant encryption” claims
into **an explicit key hierarchy with explicit failure posture**. The goal is to
ensure reviewers can answer *which key family controls which boundary* without
hand-wavy “encrypted at rest” wording, and to keep **local-first editing
legible** under key outages, rotation, or revoke events.

Companion artifacts:

- [`/artifacts/security/key_scopes.yaml`](../../artifacts/security/key_scopes.yaml)
  — canonical key-scope rows (authority + storage + export + outage posture).
- [`/schemas/security/key_state.schema.json`](../../schemas/security/key_state.schema.json)
  — strict boundary for key-scope rows, key-state records, and worked cases.
- [`/fixtures/security/key_hierarchy_cases/`](../../fixtures/security/key_hierarchy_cases/)
  — worked cases showing rotation, revoke, outage, import-root mismatch, and
  publication failure posture.

Downstream contracts this MUST stay consistent with:

- [`/docs/service/operating_mode_and_capacity_contract.md`](../service/operating_mode_and_capacity_contract.md)
  and [`/schemas/service/region_key_state.schema.json`](../../schemas/service/region_key_state.schema.json)
  (tenant/key truth cards and bounded action-family failures).
- [`/docs/admin/admin_audit_export_contract.md`](../admin/admin_audit_export_contract.md)
  (admin audit exports covering key rotation/revoke/import actions without console dependence).
- [`/docs/security/secret_class_matrix.md`](./secret_class_matrix.md) and
  [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  (device-session secret handling and export posture; signing-key material rules).
- [`/docs/release/mirror_integrity_and_offline_verification_contract.md`](../release/mirror_integrity_and_offline_verification_contract.md)
  (mirror/offline trust roots and import verification posture).
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  (signing-class and build-identity linkage for published artifacts).
- [`/docs/security/advisory_surface_contract.md`](./advisory_surface_contract.md)
  and [`/docs/security/emergency_distribution_policy.md`](./emergency_distribution_policy.md)
  (advisory/emergency records are signed artifacts with consistent key vocabulary).
- [`/docs/release/assurance_center_and_regulated_claim_contract.md`](../release/assurance_center_and_regulated_claim_contract.md)
  (key-ownership and key-mode claims are gated on explicit rows and evidence).

Normative source alignment:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.11 (key management and
  tenant encryption model).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6 (identity/session
  and key-management architecture).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` key-ownership and boundary truth
  surfaces (key mode and “who owns the keys?” cues).

If this contract disagrees with those sources, those sources win and this file,
the companion schema, fixtures, and key-scope rows update in the same change.

## Scope

Frozen here:

1. A closed set of **key scopes** (device-session secrets, delegated credentials,
   tenant/org managed-data keys, signing authorities, mirror/import roots).
2. The **required metadata** a scope must expose (key epoch, rotation/revoke
   state, export posture, audit linkage).
3. **Tenant-scoped envelope-encryption rules** for managed data.
4. Explicit **degradation rules**: key outages block only the managed data or
   managed action family that cannot be bounded safely; local edit/save/search/Git
   remain available.
5. A signing-authority map that states what halts publication rather than
   silently downgrading trust.

Out of scope:

- implementing a specific KMS/HSM adapter, keychain integration, or signing service;
- automating bulk rewrap/rotation jobs; and
- defining vendor- or customer-specific operational runbooks.

## Key scopes (published)

The canonical rows live in
[`/artifacts/security/key_scopes.yaml`](../../artifacts/security/key_scopes.yaml).
This table is reviewer-oriented; when in doubt, the YAML rows control.

| Key scope | Controls | Authority owners (variants) | Storage backend class (variants) | Failure posture (summary) |
|---|---|---|---|---|
| `device_session_secret_material` | managed-session secrets on a device | OS / customer vault | OS keychain / vault adapter | local editing continues; identity refresh may re-auth or degrade |
| `short_lived_delegated_credentials` | issuer keys for scoped tokens/assertions | vendor or customer operator | token service store / KMS-HSM | fail closed for the specific managed operation; local continues |
| `tenant_org_managed_data_keys` | envelope keys for managed data | vendor or customer operator | KMS/HSM | blocks only protected managed data (sync/policy/audit/etc); local continues |
| `artifact_signing_and_release_keys` | signing keys for release/policy/emergency artifacts | vendor or customer operator | signing service / HSM | publication halts; installed artifacts remain inspectable |
| `mirror_and_sovereign_import_roots` | trust roots for mirror/offline import verification | customer operator | KMS/HSM or offline root store | imports fail closed under mismatch/unknown roots; local continues |

## Required metadata (per scope)

Every scope row declares a list of required metadata keys. Reviewers should be
able to find these fields in any product surface that claims the scope:

- **Key epoch** (`key_epoch`): an opaque ordering label (does not reveal key
  bytes or cloud identifiers).
- **Rotation / revoke state** (`key_state_class` plus timestamps): explicit
  planned/started/completed state or explicit mismatch/unreachable/revoked.
- **Audit linkage**: which audit/export surfaces must carry safe, attributable
  events for issuance/rotation/revoke/import actions.
- **Exportability posture**: public verification material is exportable; private
  material is not.

## Tenant-scoped envelope encryption (managed data)

This contract uses “envelope encryption” narrowly and mechanically:

1. A managed object (policy bundle cache entry, sync payload, audit record,
   managed artifact metadata) is encrypted with a fresh per-object **DEK**
   (data-encryption key).
2. The DEK is wrapped by the tenant/org’s **KEK** (key-encryption key) under the
   `tenant_org_managed_data_keys` scope and **key epoch**.
3. The ciphertext MUST carry metadata sufficient to answer:
   - which scope protects it (`tenant_org_managed_data_keys`);
   - which tenant boundary it is bound to (opaque `tenant_ref`);
   - which key epoch wrapped it (`key_epoch`);
   - what algorithm family applies (`algorithm_family`);
   - where the wrapped DEK lives (`wrapped_dek_ref`, opaque); and
   - what export forms are admissible when the key path is unavailable.

Rules:

- **Tenant binding is not optional.** A ciphertext envelope MUST be tenant-scoped
  and MUST fail closed across tenant boundaries. Tenant identity is represented
  as opaque refs, not raw names.
- **Local-first degradation is mandatory.** If the KEK path is unreachable,
  managed features that require decryption block only the protected managed data.
  Local files and local workflows remain available.
- **Exports remain honest.** When keys are unavailable, exports MAY still emit
  ciphertext envelopes and metadata-only manifests, but MUST NOT claim plaintext
  export succeeded.

## Customer-managed key behavior (promises)

When a deployment claims `key_mode = customer_managed` for a key scope, the
product promise is the behavior below (not the mechanism of a specific KMS):

- **Rotation**
  - rotation creates a new `key_epoch`;
  - new writes use the new epoch immediately once admitted;
  - old epochs remain readable until explicitly retired; and
  - a completed rotation that occurred out-of-band transitions to
    `rotation_completed_recheck_required` until the boundary is rechecked.
- **Revoke**
  - revoke is a fail-closed state for the affected managed action families;
  - revoke does not erase local files or local editing capability; and
  - surfaces must render the bounded impact (never whole-product outage copy).
- **Restore**
  - restoring key reachability clears `*_unreachable`/mismatch states after
    explicit recheck and returns affected families to normal operation.
- **Export**
  - plaintext export of protected managed data requires key availability;
  - metadata-only export and ciphertext export remain admissible without keys;
  - exports never include raw private key material.
- **Outage**
  - outages are scoped (per key scope) and always preserve local-first editing.

## Signing authority (what halts publication)

The `artifact_signing_and_release_keys` scope exists to keep publication honest:

- if signing authority is unavailable, publication fails closed;
- the product must not ship or mirror unsigned artifacts under “official” or
  “verified” wording; and
- advisories/emergency records are treated as signed artifacts governed by the
  same vocabulary.

## Cross-surface bindings (shared vocabulary)

Surfaces must reuse these key-scope ids and the per-scope outage posture:

- **Exact-build identity & packaging:** key scope `artifact_signing_and_release_keys`
  is the signing authority behind claim-bearing release artifacts.
- **Advisory/emergency:** advisory and emergency bundles cite signing authority
  using the same scope ids; loss of signing authority is a typed block.
- **Mirror/offline verification:** `mirror_and_sovereign_import_roots` is the
  vocabulary used by mirror identity and offline trust-root verification.
- **Secret store / device session:** `device_session_secret_material` is the
  protection boundary for managed-session secrets at rest.
- **Admin/audit:** rotation/revoke/import outcomes must be attributable and
  export-safe using the admin-audit export contract and the audit surfaces
  referenced on each scope row.
- **Assurance claims:** key-ownership (`os_store`, `vendor_managed`,
  `customer_managed`, `offline_trust_root`) and outages point to explicit scope
  rows and bounded action-family effects.

## Example coverage

Worked cases live under:
[`/fixtures/security/key_hierarchy_cases/`](../../fixtures/security/key_hierarchy_cases/).
They focus on the states reviewers most often need to reason about:

- managed-data key outage blocks only protected managed data;
- customer-managed rotation and boundary recheck;
- mirror/offline trust-root mismatch blocks import but not local work; and
- signing authority loss halts publication without silently downgrading trust.
