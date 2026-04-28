# Update manifest, rollback, and helper negotiation contract

This document freezes the update and rollback object family Aureline
uses to connect build identities, channel manifests, signatures,
platform trust receipts, helper and remote-agent negotiation, migration
journals, backups, and repair paths. It is a contract layer, not an
updater implementation.

The goal is simple: release, support, enterprise validation, and
headless publication tooling must be able to reconstruct what update
was offered, why it was trusted, what helper versions were admitted,
what state was migrated, and what rollback or repair path remains
available without reading installer code or reverse-engineering local
side effects.

Companion artifacts:

- [`/schemas/release/update_manifest.schema.json`](../../schemas/release/update_manifest.schema.json)
  - machine-readable update-manifest object family.
- [`/schemas/release/helper_version_negotiation.schema.json`](../../schemas/release/helper_version_negotiation.schema.json)
  - helper, sidecar, and remote-agent version-negotiation packet
  schema used before upgrade, attach, side-by-side reuse, rollback,
  repair, or mirror import.
- [`/fixtures/release/upgrade_downgrade_cases/`](../../fixtures/release/upgrade_downgrade_cases/)
  - worked upgrade, blocked downgrade, helper-skew, mirror-fed update,
  and exact-build reconstruction cases.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  - build identity model every update artifact resolves through.
- [`/docs/release/release_artifact_graph.md`](./release_artifact_graph.md)
  - coordinated release-family graph and rollback atom.
- [`/docs/release/channel_and_branch_contract.md`](./channel_and_branch_contract.md),
  [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml),
  and
  [`/artifacts/release/artifact_family_versioning.yaml`](../../artifacts/release/artifact_family_versioning.yaml)
  - channel, downgrade posture, last-known-good repair path, and
  per-family versioning model.
- [`/docs/compat/upgrade_order_contract.md`](../compat/upgrade_order_contract.md),
  [`/schemas/compat/mixed_version_envelope.schema.json`](../../schemas/compat/mixed_version_envelope.schema.json),
  and
  [`/artifacts/compat/skew_windows.yaml`](../../artifacts/compat/skew_windows.yaml)
  - shared mixed-version, skew-window, upgrade-order, and rollback-order
  vocabulary.
- [`/docs/release/supply_chain_trust_framework_matrix.md`](./supply_chain_trust_framework_matrix.md)
  and
  [`/artifacts/release/trust_framework_rows.yaml`](../../artifacts/release/trust_framework_rows.yaml)
  - TUF-style metadata, signing, provenance, mirror continuity, and
  trust-root rotation expectations.
- [`/docs/state/migration_and_restore_playbook.md`](../state/migration_and_restore_playbook.md),
  [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md),
  and
  [`/docs/support/reconstruction_drill.md`](../support/reconstruction_drill.md)
  - migration journal, restore-provenance, repair transaction, and
  support reconstruction rules.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` sections on update, rollback, packaging,
  enterprise update controls, signing, update verification, and
  supply-chain compromise.
- `.t2/docs/Aureline_Technical_Design_Document.md` deployment,
  release artifact graph, mixed-version compatibility, release-center,
  and rollback matrices.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` release
  candidate, promotion, artifact provenance, signature, rollback,
  yank, and revoke templates.

If this document disagrees with those sources, those sources win and
this document plus its companion schemas and fixtures update in the
same change.

## Scope

Frozen here:

- The `update_manifest_record` shape for update, rollback, revoke,
  yank, repair, reconstruction, mirror import, and managed-fleet
  publication.
- The per-artifact entries that bind channel, source, artifact family,
  exact-build identity, build id, digest, signature, platform trust or
  notarization refs, provenance refs, support refs, and rollback-atom
  membership.
- The staged-rollout fields that preserve ring, cohort, pause, expiry,
  rollback-stop, and repair state.
- The downgrade eligibility fields that decide whether a downgrade is
  automatic, review-gated, blocked, manually reviewed, or unsupported.
- The helper and remote-agent negotiation packet fields used before
  upgrade, attach, side-by-side reuse, rollback repair, and mirror
  import.
- Reserved release-center fields for publish target, promotion stage,
  requested action, and break-glass reconciliation so UI and headless
  publication flows read one object family.

Out of scope:

- Building a hosted update service, installer, package-manager backend,
  signing service, transparency-log integration, notarization pipeline,
  or fleet controller.
- Defining signing key custody or raw signing material. Records carry
  refs, states, receipts, and evidence ids only.
- Defining platform-specific installer scripts. A conforming installer
  consumes these records; the record shape does not depend on installer
  implementation details.

## Invariants

1. **Manifests are sufficient reconstruction input.** Given an update
   manifest, exact-build identity records, signature or digest refs,
   helper negotiation packets, and linked support evidence, release or
   support tooling can reconstruct the update, rollback, or repair
   state without reading installer code.
2. **Exact-build identity is primary.** Version strings are labels.
   Every release-bearing artifact entry carries an
   `exact_build_identity_ref` and a `build_id`. When the two disagree,
   the manifest is non-conforming until release evidence resolves the
   mismatch.
3. **Trust is referenced, not embedded.** Hashes, signature refs,
   platform receipts, notarization tickets, transparency-log entries,
   provenance refs, and trust-root refs are stable references. Raw
   binaries, raw signing material, raw receipts, and raw logs do not
   appear in the manifest.
4. **Rollback targets a coordinated release family.** A rollback target
   is invalid when it restores only a binary while leaving paired docs,
   support sidecars, schema exports, provenance, revocation metadata,
   or known-limit truth on a contradictory build.
5. **Downgrade is explicit.** Automatic downgrade is permitted only
   when state compatibility, backup, migration journal, helper
   negotiation, and rollback target checks all pass. Any unsafe or
   unknown case becomes `manual_review_required`, `blocked`, or
   `unsupported`; it never falls into generic failure.
6. **Helper negotiation happens before mutation.** A helper, local
   sidecar, or remote agent is checked before upgrade, attach,
   side-by-side reuse, rollback, repair, or mirror import can mutate
   runtime state.
7. **Mirrors preserve identity.** A mirror or offline bundle may narrow
   trust, freshness, or installability. It may not re-anchor an
   artifact to a new build identity or silently omit revocation,
   rollback, expiry, or signature state.
8. **Break-glass is still structured.** Emergency publication, mirror
   repair, rollback, yank, or revoke actions use the same action and
   reconciliation fields as ordinary publication, with explicit
   approval, reason, target, and follow-up refs.

## Update Manifest Model

An `update_manifest_record` is the shared object consumed by release
center, headless publication, update center, installer, support bundle,
enterprise validation, and mirror/offline import tooling.

Required field groups:

| Field group | Purpose |
|---|---|
| `channel` | Names channel, source class, feed or mirror ref, freshness, trust-root ref, and mirror lineage where applicable. |
| `requested_action_class` | One of publish, promote, rollback, revoke, yank, repair, or reconstruct. |
| `publication` | Carries publish target, promotion stage, release-center event ref, headless command ref, and break-glass reconciliation fields. |
| `artifacts` | One entry per released or support-bearing artifact family. Each entry binds family, exact-build identity, build id, digest, signature, platform trust or notarization refs, provenance refs, support refs, and rollback-atom membership. |
| `staged_rollout` | Captures rollout state, ring, percentage basis points, cohort policy, pause or rollback-stop ref, expiry, and notes. |
| `downgrade_policy` | Captures eligibility state, source and target build refs, schema epochs, backup and migration-journal requirements, manual-review or blocked reasons, preserved state roots, and repair ref. |
| `rollback_repair_links` | Required rollback, repair, migration, backup, support, and manual-review refs. |
| `helper_negotiation_refs` | Packet refs proving helper, sidecar, or remote-agent compatibility was checked before mutation. |
| `support_reconstruction` | Stable refs used by support and enterprise validation to reconstruct the manifest state. |

Artifact entries MUST include:

- `artifact_family_class` or a stable `artifact_family_ref`;
- `exact_build_identity_ref`;
- `build_id`;
- `digest.algorithm`, `digest.value`, and `digest.content_address_ref`;
- `signature.signature_state` and the refs needed to verify or explain
  the state;
- `platform_trust.platform_trust_class`, including notarization or
  platform trust refs when applicable;
- provenance refs for SBOM, attestation, source, or reproducibility
  where the family requires them; and
- support refs for symbols, source maps, crash symbols, support
  packets, or release-evidence packets where applicable.

## Trust and Platform Receipts

The manifest treats trust as a layered result:

1. **Content identity:** digest algorithm, digest value, and
   content-addressed artifact ref.
2. **Signature state:** signature present or absent, verifier state,
   signer identity ref, trust-root ref, revocation ref, and
   transparency-log ref where available.
3. **Platform trust:** platform-native code signing, notarization,
   package-manager signature, enterprise mirror receipt, or manual
   import receipt.
4. **Provenance:** exact-build identity, SBOM, attestation, source, and
   reproducibility refs.
5. **Metadata freshness:** expiry, staged-rollout pause, revocation,
   supersedence, and mirror lineage.

An update may not move past `verified` when any required layer is
missing, stale, revoked, or unverifiable. Mirrors and manual import
paths carry the same trust refs plus their mirror or import receipt;
they do not replace the origin identity.

## Downgrade Eligibility

Downgrade eligibility is a manifest field, not installer-local logic.
The closed states are:

| State | Meaning |
|---|---|
| `auto_eligible` | All rollback, helper, state, backup, migration, and trust checks passed. The downgrade may apply without manual review under the current policy. |
| `eligible_with_review` | Checks passed but policy or enterprise posture requires user or admin confirmation before apply. |
| `manual_review_required` | The target may be safe, but evidence is incomplete or state semantics changed enough that a reviewer must choose backup, repair, export, or abort. |
| `blocked` | The downgrade would violate state, trust, forced-minimum, helper-skew, mirror-lineage, revocation, or rollback-atom rules. |
| `unsupported` | The target is outside the published downgrade window or cannot preserve required state. |

Automatic downgrade requires all of the following:

- target build belongs to an admissible rollback atom;
- target artifact family entries resolve to exact-build identities;
- signatures, digests, and platform trust refs verify for every
  required artifact;
- helper negotiation packet returns an allow or explicitly degraded
  decision that still supports the rollback path;
- state schema is additive-compatible or the target can read the
  current state;
- backup snapshot exists before any user-authored state migration;
- migration journal exists when schema epoch, durable state format, or
  helper capability set changed;
- repair transaction exists before any destructive or compensating
  support path is offered; and
- enterprise policy does not force a higher minimum version for
  security or compliance.

Manual review is required when:

- the target build cannot prove it reads the current state schema but a
  repair or export path may preserve user-authored state;
- a migration journal is missing, incomplete, or produced by a build
  whose exact-build identity is not in the manifest;
- backup exists but does not cover every affected durable state root;
- helper or remote-agent negotiation narrows to review-only or file-only
  mode and the rollback plan expects mutation capability;
- mirror lineage is incomplete but an enterprise operator can supply a
  signed manual-import receipt;
- an admin pinned floor conflicts with user-authored state migration;
  or
- support evidence can reconstruct the build but not the state
  transition.

Blocked downgrade is required when:

- target cannot verify digest, signature, trust root, platform receipt,
  or revocation freshness;
- target rollback atom omits a required docs, schema, provenance,
  sidecar, release-evidence, or support artifact;
- target cannot read, repair, or export newer user-authored state;
- forced minimum version or active advisory forbids the target;
- helper or remote-agent skew is outside the published window and no
  review-only or file-only posture satisfies the operation; or
- the requested action would delete user-authored configuration as the
  primary recovery path.

## Helper Version Negotiation

`helper_version_negotiation_packet_record` is emitted before any
runtime helper, local sidecar, or remote agent participates in an
update-sensitive operation.

Required preflight surfaces:

- `upgrade_preflight` before a helper or agent is replaced;
- `attach_preflight` before desktop or CLI attach;
- `side_by_side_reuse_preflight` before reusing a helper across channel
  or install roots;
- `rollback_repair_preflight` before helper rollback or repair; and
- `mirror_import_preflight` before importing a helper or agent from a
  mirror or offline bundle.

The packet records:

- client, helper, sidecar, or agent participant versions;
- exact-build identity and artifact digest refs for compiled
  participants;
- contract version, schema epoch, and min/max supported versions;
- required, offered, negotiated, and dropped capabilities;
- mixed-version envelope, skew-window, version-skew-register, and
  compatibility-row refs;
- selected contract version;
- decision (`allow`, `allow_degraded_review_only`, `allow_file_only`,
  `block`, `manual_review`, `replace_helper_before_continue`, or
  `reattach_required`);
- explicit unsupported-skew behavior; and
- repair, rollback, migration, backup, and support refs.

Capability negotiation uses intersection. A helper that advertises a
capability the client cannot verify is narrowed or blocked; it is not
treated as a successful full-capability attach.

Unsupported skew behavior MUST be explicit:

| Unsupported behavior | Result |
|---|---|
| `refuse_upgrade` | Upgrade stops before replacing helper bytes. |
| `refuse_attach` | Attach stops before opening a mutating session. |
| `refuse_side_by_side_reuse` | Existing helper cannot be reused across the requested channel or install root. |
| `degrade_review_only` | Read, search, or review-only paths may continue; mutating paths are unavailable. |
| `require_helper_replace` | A verified helper replacement must occur before continuing. |
| `require_manual_repair` | The operation stops for a repair transaction or operator review. |
| `require_probe` | Unknown combination records a probe-required state and does not widen claims. |

## Release-Center Action Fields

The update manifest reserves one object family for UI and headless
release actions. The same fields are used for publish, promote,
rollback, revoke, yank, repair, and reconstruction:

- `requested_action_class`;
- `publication.publish_target_class`;
- `publication.promotion_stage_class`;
- `publication.release_center_event_ref`;
- `publication.headless_command_ref`;
- `publication.approval_refs`;
- `publication.break_glass_reconciliation`.

Break-glass reconciliation carries:

- whether break-glass is active;
- reason ref;
- approval ref;
- reconciliation target ref;
- follow-up packet ref;
- reconciliation due time; and
- a note explaining what must be reconciled.

Break-glass never deletes the ordinary timeline. It adds exception
metadata to the same action object and later reconciles back to a
normal manifest state.

## Reconstruction From Support Evidence

Support, enterprise validation, and incident review reconstruct update
state from the following ordered set:

1. `update_manifest_record`;
2. exact-build identity records for every artifact entry;
3. digest and signature verification refs;
4. platform trust or notarization refs;
5. provenance refs for SBOM, attestation, source, and reproducibility;
6. helper negotiation packets;
7. migration journal and backup refs where state changed;
8. repair transaction refs where rollback was blocked or compensating;
9. ring, rollout, promotion, and break-glass refs; and
10. support packet refs.

If any required ref is missing, the reconstructed state is narrowed.
It may become `manual_review_required`, `blocked`, or `unsupported`,
but it may not silently claim the update or rollback succeeded.

## Fixture Coverage

The fixture corpus under
[`/fixtures/release/upgrade_downgrade_cases/`](../../fixtures/release/upgrade_downgrade_cases/)
covers:

- safe upgrade with verified signatures, platform trust, backup,
  migration journal, and helper negotiation;
- blocked downgrade where newer user-authored state cannot be read by
  the target and the migration journal is missing;
- helper skew that narrows attach to review-only rather than failing
  ambiguously;
- mirror-fed update with origin digest, signature, revocation, and
  manual-import receipt preservation; and
- exact-build reconstruction using manifest, support evidence,
  signature refs, and helper negotiation packets.

These fixtures are structural seeds. They do not claim a live updater,
fleet controller, signing service, or helper runtime exists yet.
