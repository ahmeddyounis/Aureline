# Storage-mode, retention-mode, and raw-secret-exclusion vocabulary

This document is the normative companion to the machine-readable
storage, retention, and raw-secret-exclusion register. It binds
credentials, support bundles, sync payloads, profile exports, issue
packages, incident bundles, AI evidence packets, and other retained
artifacts to **one** explainable vocabulary. Every surface that shows a
user, admin, or reviewer where data lives and how long it lasts MUST
cite a stable id from this vocabulary. Vague phrasing such as "saved
securely," "stored safely," "kept private," "handled securely," or
"encrypted at rest" is explicitly non-conforming.

Companion artifacts:

- [`/artifacts/governance/storage_modes.yaml`](../../artifacts/governance/storage_modes.yaml)
  — machine-readable register. Tooling reads this file; the narrative
  below describes the same rows.
- [`/fixtures/governance/export_redaction_examples/`](../../fixtures/governance/export_redaction_examples/)
  — worked examples showing how each consumer declares all three axes
  on a concrete row.
- [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — the `artifact_storage_mode` and `evidence_embedding_state` enums
  on the support-bundle contract are isomorphic to the artifact-axis
  of this register; the register pins the crosswalk explicitly.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — the ADR that freezes trust-store classes, projection modes, and
  redaction defaults. Credential-storage-mode rows inherit those
  classes verbatim.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — the per-record-class retention posture this register composes over.
  The register does not replace per-class retention; it names the
  user-visible retention mode that each record-class row projects onto.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — the deployment-profile retention_class_vocabulary that bounds the
  tenant-level retention windows.

**One vocabulary, three orthogonal axes.** This vocabulary is
explicitly factored into three axes that MUST NOT be collapsed into
one lifecycle string. A row that names only one axis is incomplete:

1. **`credential_storage_mode`** — where a secret or credential
   actually materialises. Six values: `system_credential_store`,
   `enterprise_secret_store`, `session_only`, `handle_only`,
   `delegated`, `not_configured`.
2. **`artifact_storage_mode`** — where an exported or retained artifact
   body actually lives. Six values: `local_only`, `uploaded_copy`,
   `mirrored_copy`, `embedded_evidence`, `by_reference_evidence`,
   `intentionally_excluded`.
3. **`retention_mode`** — how long the record or export lasts, and
   whether raw secrets are excluded. Six values: `expiring`, `pinned`,
   `policy_held`, `exportable`, `audit_only`, `raw_secret_excluded`.

Every retained-artifact row carries all three axes. Silent defaults
are validation failures.

## Why this vocabulary exists

Previous draft strings described where data lived with phrases like
"saved securely," "kept on your device," "synced to the cloud," or
"handled by the system." Those strings failed three reviews in a row:

- They did not explain which store actually held the material, so
  locked-store and step-up-required states had nowhere to render.
- They did not distinguish a local-only retained copy from an uploaded
  copy of the same logical class, so support bundles and incident
  bundles looked identical in a preview even when their bytes lived
  in entirely different places.
- They did not say whether raw secrets were excluded, so export
  manifests silently widened collection past the ADR-0007 redaction
  defaults without tripping a reviewer.

This register fixes those failures by forcing every row to name the
three axes. A reader or reviewer looking at any retained-artifact
manifest can answer three questions without reading prose:

1. Where does the credential, if any, actually live?
2. Where does the artifact body actually live?
3. How long does the record last, and are raw secrets excluded?

## The credential-storage-mode axis

Every credential reference names exactly one credential-storage-mode
value. The frozen set and the ADR-0007 trust-store class each row
inherits are below.

| Mode                        | Required label             | Backing store (ADR-0007)                          | Raw material in exports | Typical projection modes                                                                 |
|-----------------------------|----------------------------|---------------------------------------------------|-------------------------|------------------------------------------------------------------------------------------|
| `system_credential_store`   | `System credential store`  | `os_keychain`                                     | Never                   | `alias_only`, `broker_callback`, `request_header_signer`, `inspect_metadata`, `reveal_on_demand` |
| `enterprise_secret_store`   | `Enterprise secret store`  | `enterprise_vault_adapter`, `hsm_or_kms_backed`   | Never                   | `alias_only`, `broker_callback`, `sign_only`, `decrypt_only`, `token_exchange`, `inspect_metadata` |
| `session_only`              | `Session only`             | `session_memory_cache`                            | Never                   | `broker_callback`, `request_header_signer`, `ephemeral_fd`, `env_var_isolated_child`, `token_exchange`, `inspect_metadata` |
| `handle_only`               | `Handle only`              | _n/a — consumer holds a handle or alias_           | Never                   | `alias_only`, `inspect_metadata`                                                         |
| `delegated`                 | `Delegated`                | `platform_agent`, `managed_policy_injector`       | Never                   | `sign_only`, `decrypt_only`, `policy_materialised`, `token_exchange`, `inspect_metadata` |
| `not_configured`            | `Not configured`           | _n/a — no credential registered_                  | Never                   | _none_                                                                                   |

`handle_only` is the default on every profile, recipe, sync payload,
support bundle, and mutation-journal entry: the consumer holds a
broker-minted handle or a user-stable alias, not raw material. The
handle itself is not persisted in workspace files, sync exports, or
support bundles — profiles carry aliases, not handles (ADR-0007).

`not_configured` is a distinct truth state, not an error or a silent
empty. Surfaces render the required label verbatim and name the
capability that would register one; they do not fall through to a
generic "unknown" or hide the absence.

## The artifact-storage-mode axis

Every exported or retained artifact row names exactly one
artifact-storage-mode value. The frozen set is below, together with
the isomorphic `support_bundle.schema.json` pair. Every value here
projects one-for-one onto that schema's enums so a support-bundle row
and a profile-export row describing the same logical disposition never
drift.

| Mode                       | Required label               | Bytes leave the device? | Pairs with `support_bundle.artifact_storage_mode` | Pairs with `support_bundle.evidence_embedding_state` |
|----------------------------|------------------------------|-------------------------|----------------------------------------------------|-------------------------------------------------------|
| `local_only`               | `On this device`             | No                      | `local_only_copy_retained`                         | `by_reference`                                        |
| `uploaded_copy`            | `Uploaded to support`        | Yes                     | `optional_upload_candidate`                        | `by_reference`                                        |
| `mirrored_copy`            | `Mirrored to managed store`  | Yes                     | `managed_reference_only`                           | `by_reference`                                        |
| `embedded_evidence`        | `Embedded in this export`    | Yes                     | `embedded_export_copy`                             | `embedded`                                            |
| `by_reference_evidence`    | `Referenced, not embedded`   | No                      | `managed_reference_only`                           | `by_reference`                                        |
| `intentionally_excluded`   | `Intentionally excluded`     | No                      | `intentionally_excluded`                           | `omitted`                                             |

`local_only` and `uploaded_copy` are *not* synonyms for a logical
artifact class. A crash-diagnostic payload may exist as a `local_only`
copy on the user's device, an `uploaded_copy` attached to a support
case, a `mirrored_copy` in a managed incident store, and an
`intentionally_excluded` entry on a third-party export packet — all at
the same time, as distinct rows with distinct retention timers.
Preview surfaces render each row separately.

`intentionally_excluded` is always visible. The support-bundle
manifest carries an `omission_reason_class` from the support-bundle
schema (`redacted_by_profile`, `prohibited_secret_or_token`,
`prohibited_full_shell_history`, `retained_local_only_pending_review`,
`awaiting_explicit_opt_in`, `policy_denied`, `user_declined`,
`source_unavailable_or_expired`, `not_collected_on_this_platform`,
`not_applicable`) so the absence is as auditable as the inclusion.

## The retention-mode axis

Every retained-artifact row names exactly one retention-mode value.
Retention modes compose over the per-record-class retention posture in
`record_class_registry.yaml`; they do not replace it.

| Mode                   | Required label                     | Default trigger                                                                                              | Allowed artifact-storage modes                                                          |
|------------------------|------------------------------------|--------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------|
| `expiring`             | `Expires on {trigger}`             | packet expiry, case close, session end, version replace, user reset, billing-period close                    | `local_only`, `uploaded_copy`, `mirrored_copy`, `embedded_evidence`, `by_reference_evidence` |
| `pinned`               | `Pinned`                           | user or admin pin past the default expiry                                                                    | `local_only`, `uploaded_copy`, `mirrored_copy`, `embedded_evidence`, `by_reference_evidence` |
| `policy_held`          | `On legal / policy hold`           | hold class from the record-class registry (`administrative_legal`, `support_investigation`, `retention_minimum`, `export_pending`) | `local_only`, `uploaded_copy`, `mirrored_copy`, `embedded_evidence`, `by_reference_evidence` |
| `exportable`           | `Exportable`                       | declared part of the active export packet scope                                                              | `uploaded_copy`, `mirrored_copy`, `embedded_evidence`, `by_reference_evidence`          |
| `audit_only`           | `Audit only`                       | audit / compliance retention floor                                                                           | `mirrored_copy`, `by_reference_evidence`                                                |
| `raw_secret_excluded`  | `Raw secrets excluded`             | secret-redaction default                                                                                     | _all_                                                                                    |

`raw_secret_excluded` is not a storage mode — it is a retention
marker. Every exportable row whose subject matter references any
credential class from ADR-0007 MUST assert `raw_secret_excluded` on
the row and on the governance-and-export-controls section of the
support-bundle manifest. Handles, aliases, consumer ids, class names,
failure codes, signed proofs, and attestations MAY appear; raw bytes,
raw headers, raw env-var values, raw private keys, raw notarization
credentials, and raw release-publish tokens MUST NOT.

`audit_only` requires a managed source of record (`mirrored_copy` or
`by_reference_evidence`). Embedding an audit-only body inside a user-
facing export is forbidden; previews quote the audit reference plus
redacted metadata.

`exportable` is only valid on rows whose bytes actually leave the
device. A `local_only` row is not exportable by definition; the
storage mode MUST be promoted first.

## The consumer-mapping table

The register names a default triple — `(credential_storage_mode,
artifact_storage_mode, retention_mode)` — for every retained-artifact
consumer family. Consumers MAY narrow a default (stricter retention,
fewer uploads, more exclusions). They MAY NOT widen past it without a
named decision row under
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

| Consumer                      | Default credential mode | Default artifact mode        | Default retention mode | `raw_secret_excluded` required? | Schema refs                                                                                                             |
|-------------------------------|-------------------------|------------------------------|------------------------|---------------------------------|-------------------------------------------------------------------------------------------------------------------------|
| `support_bundle`              | `handle_only`           | `local_only`                 | `expiring`             | Yes                             | `schemas/support/support_bundle.schema.json`                                                                            |
| `sync_payload`                | `handle_only`           | `mirrored_copy`              | `expiring`             | Yes                             | `schemas/settings/settings_sync_payload.schema.json`                                                                    |
| `profile_export`              | `handle_only`           | `embedded_evidence`          | `exportable`           | Yes                             | `schemas/profile/profile_export.schema.json`                                                                            |
| `issue_package`               | `handle_only`           | `uploaded_copy`              | `policy_held`          | Yes                             | `schemas/support/object_handoff_packet.schema.json`                                                                     |
| `incident_bundle`             | `handle_only`           | `mirrored_copy`              | `policy_held`          | Yes                             | `schemas/security/incident_workspace_packet.schema.json`                                                                |
| `ai_evidence_packet`          | `handle_only`           | `by_reference_evidence`      | `expiring`             | Yes                             | `schemas/ai/evidence_packet.schema.json`, `schemas/ai/evidence_replay_packet.schema.json`, `schemas/ai/audit_storage_manifest.schema.json` |
| `retained_evidence_generic`   | `handle_only`           | `local_only`                 | `expiring`             | Yes                             | `schemas/governance/evidence_packet_header.schema.json`                                                                  |

Two points on this table:

- **Support bundles default to `local_only` + `expiring`.** Local
  review is the baseline. Escalation to a managed support case is a
  promotion event: the row moves to `uploaded_copy` (bytes copied to
  the support case) or `mirrored_copy` (bytes copied to the tenant
  archive) and retention typically promotes to `pinned` or
  `policy_held`. The promotion is always consent-recorded; there is
  no silent upload.
- **Profile exports default to `embedded_evidence` + `exportable`.**
  Profiles are user-initiated portable snapshots; the bytes live
  inside the export file. Credentials project as `handle_only`
  aliases regardless of the artifact mode.

## Combination rules

Cross-axis combination rules are validation-relevant. Tooling that
emits an export manifest refuses to emit a row that violates a
forbidden combination.

- **`rule:raw_secrets_never_embedded`.** A row whose
  `artifact_storage_mode` is `embedded_evidence` and whose subject
  matter names any credential class MUST assert `raw_secret_excluded`.
  Embedding raw credential bytes is forbidden under every credential
  storage mode.
- **`rule:local_only_is_never_exportable`.** A `local_only` row cannot
  carry the `exportable` retention mode. Promote the storage mode
  first.
- **`rule:audit_only_requires_managed_source`.** `audit_only`
  retention requires a managed source of record (`mirrored_copy` or
  `by_reference_evidence`). Local-only, uploaded, or embedded shapes
  do not satisfy the single-source-of-record requirement.
- **`rule:session_only_never_persists`.** `session_only` credential
  material MUST NOT be paired with retention modes that imply
  persistence (`pinned`, `policy_held`, `audit_only`, `exportable`).
  The only valid retention modes are `expiring` and
  `raw_secret_excluded`.
- **`rule:not_configured_is_visible`.** `not_configured` rows MUST
  render the required label `Not configured` and cite the capability
  that would register one. Silent empty states and "saved securely"
  substitutes are non-conforming.
- **`rule:intentionally_excluded_is_visible`.** An
  `intentionally_excluded` row MUST carry an `omission_reason_class`
  drawn from the support-bundle schema's `omission_reason_class` enum.
  Silent exclusions are non-conforming.

## Forbidden user-facing phrases

These strings MUST NOT appear in any user-facing surface, export
manifest, or docs / help page that cites this register:

- `saved securely`
- `stored safely`
- `kept private`
- `handled securely`
- `secured automatically`
- `encrypted at rest` _(not a storage mode; use a
  `credential_storage_mode_id` plus a `retention_mode_id`)_

The register file itself names these strings once so drift scans and
CI checks can find any surface that accidentally reintroduces them.

## Working examples

The fixtures under
[`/fixtures/governance/export_redaction_examples/`](../../fixtures/governance/export_redaction_examples/)
show the vocabulary in practice. The set covers every consumer family
named above and deliberately pairs two rows of the same logical class
(`support_bundle_local_only_retained` vs.
`support_bundle_uploaded_copy_opt_in`) to make the local-only vs.
uploaded-copy distinction obvious. Each fixture explains where data
lives, how long it lasts, and whether raw secrets are excluded — with
no prose that would need to be rewritten for a different surface.

## Out of scope

- Implementing a secret store, credential broker, or enterprise
  retention backend. This register is the vocabulary surface; the
  implementation surface is separately governed by ADR-0007, the
  record-class registry, and the managed-service seed.
- Implementing an upload service, mirror transport, or incident-
  archive backend. The register names the user-visible states; the
  service-side contracts are tracked under the managed-service seed
  and the incident-workspace contract.
- The per-record-class retention timer values. Those live in
  `artifacts/governance/record_class_registry.yaml` and
  `artifacts/service/retention_rows.yaml`. This register names the
  user-visible mode each row projects onto.
- Final component styling or copy-length for the user-facing labels.
  Each row names a required label; presentation refinements are
  separately governed by the UX design-system style guide.
