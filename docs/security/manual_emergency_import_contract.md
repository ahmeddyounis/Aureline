# Manual emergency-import receipt, signer-continuity chain, and offline metadata audit contract

This document freezes the pre-implementation contract for **manual and
mirror-imported emergency metadata** so offline and air-gapped
deployments can prove *how* emergency state arrived (and under which
trust posture it was accepted) without relying on operator memory or
volatile local logs.

The contract is built around two export-safe artifacts:

- `manual_import_receipt_record` — the reviewable receipt minted when an
  operator imports signed emergency metadata into a mirror, offline
  bundle, or air-gapped target.
- `metadata_chain_entry_record` — the chain-of-custody row that links an
  imported emergency-metadata change to the target scope it affected.

Companion artifacts:

- [`/docs/security/emergency_distribution_policy.md`](./emergency_distribution_policy.md)
  — normative distribution policy (admissible paths, freshness labels,
  supersedence/expiry rules).
- [`/schemas/security/manual_import_receipt.schema.json`](../../schemas/security/manual_import_receipt.schema.json)
  — machine boundary for `manual_import_receipt_record`.
- [`/schemas/security/metadata_chain_entry.schema.json`](../../schemas/security/metadata_chain_entry.schema.json)
  — machine boundary for `metadata_chain_entry_record`.
- [`/fixtures/security/manual_import_cases/`](../../fixtures/security/manual_import_cases/)
  — worked receipt + chain-entry fixtures (mirror import, removable-media
  offline import, reimport/supersedence, and stale imported metadata).

Normative source alignment:

- `.t2/docs/Aureline_PRD.md` §10.15 and §10.18 — incident response,
  offline/manual recovery, signed bundles, exportability, and auditability.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6.1 and
  §22.8 — signed bundle lifecycle, emergency disable bundles, mirror and
  offline distribution expectations.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.13 — advisory
  and emergency metadata as signed, inspectable objects with explicit
  freshness and continuity on mirror/offline paths.

If this document disagrees with `.t2/docs/` sources, the `.t2/docs/`
source wins and this document, schemas, and fixtures update together.

## Scope

Frozen at this revision:

- the receipt contract for manual or mirror imports of:
  - advisory metadata (`advisory_record`),
  - emergency actions (`emergency_action_record`, including channel-freeze metadata),
  - revocations (`revocation_record`), and
  - emergency disable bundles (`emergency_disable_bundle_record`);
- the signer-continuity and trust-root evidence a receipt MUST preserve
  to be reviewable offline;
- the chain-of-custody entry contract (`metadata_chain_entry_record`)
  used by admin exports, support packets, and shiproom evidence bundles
  to reconstruct when emergency state arrived and what scope it changed;
- export-safety and redaction rules for receipts and chain entries.

Out of scope:

- running mirror infrastructure, air-gap transport plumbing, or offline
  bundle distribution tooling;
- embedding raw bundle bytes, raw signatures, raw trust-root payloads,
  raw absolute paths, raw device serial numbers, or raw operator
  identifiers in exported records.

## Manual import receipt record

A `manual_import_receipt_record` is the durable, export-safe receipt
minted when emergency metadata is imported into a target that is not
served by the live authoritative origin.

It MUST record:

- **Imported record kind** (`imported_record_kind`) and **identity**
  (`receipt_id`, `receipt_state`).
- **Scope facts that survive export**:
  - `artifact_family_refs[]` (what artifact families this import governs),
  - `subject_refs[]` (typed affected subject refs),
  - `target_scope` (which mirror / offline bundle / trust-root scope was touched).
- **Operator attribution** (`operator`) under stable opaque refs.
- **Import path** (`import_path_class`) and **source artifact** facts
  (`source_artifact.*`) including digests (`source_artifact_digests[]`)
  rather than raw bytes.
- **Verification envelope** (`verification.*`) including detached-signature
  posture, verification outcome, tool/policy refs, and timing.
- **Signer-continuity posture as observed on the receiving target**
  (`observed_signer_continuity_state`) plus **trust-root rotation state**
  (`trust_root_rotation_state_refs[]`) when trust-root posture gated the
  verification outcome.
- **Receiving-target posture**:
  `freshness_class`, `validation_state`, `applied_state`,
  `supersedence_state`, and `expiry`.
- **Follow-up obligations** (`follow_up_obligations[]`) that keep offline
  response actionable (for example: propagate to sibling offline bundles,
  export a support packet, re-verify at expiry).
- **Signer-continuity chain stub** (`metadata_chain`) rooted at
  `authoritative_origin_ref` with typed chain links describing upstream
  and downstream hops.

Non-negotiable rules:

1. **Export-safe only.** Receipts MUST NOT embed raw bundle bytes, raw
   signature bytes, raw trust-root payloads, raw absolute paths, raw
   device identifiers, or raw operator identifiers.
2. **No surface-invented truth.** Surfaces MUST NOT invent receipt-local
   freshness, signer-continuity, verification, supersedence, or expiry
   terms. They MUST project from the receipt fields as-is.
3. **Trust-root posture is inspectable.** If the receipt targets a
   `trust_root_scope`, it MUST name at least one
   `trust_root_rotation_state_ref` so offline reviewers can inspect what
   trust-root state gated acceptance.
4. **Supersedence is durable.** A superseded receipt remains exportable
   and reviewable; supersedence is represented by relationship links and
   state fields, not deletion.

## Metadata chain entry record

A `metadata_chain_entry_record` is the per-change chain-of-custody row
used to join:

- the authoritative origin record,
- the receipt that proved an import on a specific target, and
- the target scope and posture the import changed.

The record exists so support, admins, and shiproom reviewers can
reconstruct *when* emergency state arrived and *what it changed* without
re-loading an entire local database or relying on non-portable logs.

It MUST:

- name one propagation family (`propagation_kind_class`);
- reference the authoritative origin (`authoritative_origin_ref`);
- reference the proving receipt (`manual_import_receipt_ref`);
- mirror the export-safe scope facts (`artifact_family_refs[]`,
  `subject_refs[]`, `target_scope`);
- carry the operator/import/verification envelopes (by value) so the
  chain entry remains reviewable even when the receipt is not co-packaged;
- carry receipt posture (`receipt_state`, `applied_state`,
  `supersedence_state`, `expiry`, `freshness_class`, `validation_state`);
- carry `trust_root_rotation_state_refs[]` when trust-root posture gated
  the import; and
- provide timestamps (`imported_at`, `applied_at`, `minted_at`) suitable
  for offline review.

## Offline audit and export rules

When a deployment is mirrored, offline, or air-gapped, **auditability is
a product feature**:

- Every manual/mirror/offline import MUST mint a receipt.
- Every receipt that changes emergency posture on a target SHOULD mint at
  least one chain entry describing the specific propagation kind and
  target scope it affected.
- Support packets and admin exports that claim emergency metadata
  coverage MUST include the receipt refs and chain ids necessary to join
  back to the authoritative origin without live network access.
- Stale, expired, or unverified imported metadata MUST remain visible
  and exportable with explicit `freshness_class`, `validation_state`,
  and follow-up obligations; “silent ignore” is non-conforming.
