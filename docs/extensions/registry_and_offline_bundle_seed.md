# Extension registry, mirror, and offline-bundle contract seed

This document is the narrative companion to the registry-manifest
boundary schema at
[`/schemas/extensions/registry_manifest.schema.json`](../../schemas/extensions/registry_manifest.schema.json)
and the machine-readable promotion / trust / anti-abuse register at
[`/artifacts/extensions/channel_promotion_rows.yaml`](../../artifacts/extensions/channel_promotion_rows.yaml).
It names the reserved registry-source classes, the channel lanes,
the mirror-continuity state, the offline-bundle export and restore
shape, the local-archive and quarantined-local-copy posture, and
the shared trust / compatibility / anti-abuse vocabulary. The
schema is authoritative when the narrative and the schema disagree;
this document MUST be updated in the same change that lands any
schema bump.

The seed is deliberately narrow. It does **not** land a registry
implementation, a mirror service, a marketplace UX, or an offline-
bundle exporter / importer. Its job is to freeze the distribution
vocabulary early enough that self-hosted, air-gapped, enterprise-
mirror, offline-bundle, and side-load workflows remain first-class
when later lanes build against it.

## What this seed freezes

1. A **six-class registry-source vocabulary** — `public_registry`,
   `approved_mirror`, `private_registry`, `offline_bundle`,
   `local_archive`, `quarantined_local_copy` — and its projection
   onto the ADR-0012 parent vocabulary.
2. A **three-lane channel vocabulary** — `quarantine`, `approved`,
   `production` — plus the rule that promotion across lanes
   preserves artifact identity verbatim.
3. A **content-addressed manifest shape** binding every row to a
   `(digest_algorithm, digest_hex)` pair. Mirror, offline-bundle,
   and local-archive rows quote the content-address verbatim; a row
   that re-digests or re-signs during transport is denied with
   `artifact_identity_mutated_on_repackage`.
4. A **shared trust-inheritance matrix** with five inheritance
   rules (`inherits_origin_tier`,
   `capped_at_organisational_on_private_registry`,
   `capped_at_community_on_approved_mirror`,
   `capped_at_unverified_on_local_archive`,
   `quarantined_cannot_inherit`). Inheritance is capped at the
   transport; silent promotion is forbidden.
5. A **mirror-continuity state machine** with nine states. The
   `narrowing_attempted_widening` terminal state captures the
   ADR-0012 invariant that a mirror MAY narrow (deny-list,
   allow-list, publisher-trust floor, version floor, emergency
   disable, signed-continuity required) but MUST NOT widen.
6. An **offline-bundle export/import contract** covering
   dependency metadata, per-artifact publisher-continuity
   snapshots, lockfile parity, revocation-snapshot age, and a
   deterministic restore preview.
7. A **local-archive and quarantined-local-copy posture** that
   preserves provenance without admitting a verified-publisher
   badge.
8. A **shared anti-abuse signal vocabulary** raised on every row
   kind (registry, mirror, channel promotion, offline bundle, local
   archive). A surface that hides a raised signal is denied with
   `review_disclosure_incomplete`.
9. A **typed denial-reason vocabulary** that binds every registry /
   mirror / offline-bundle failure to a repair affordance. Silent
   fallback to a generic `install blocked` chip is forbidden.

## Record kinds

| Record kind                          | Purpose                                                                                                          | Schema ref                                                                                                     |
|--------------------------------------|------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| `registry_manifest_row`              | Per-artifact content-addressed registry manifest row. Carries digest, signature class, publisher-continuity ref, registry-source class, channel, approval state, compatibility notes, mirror provenance, and anti-abuse signals. | `#/$defs/registry_manifest_row`                                                                                |
| `mirror_continuity_row`              | Mirror provenance and continuity state; quotes origin digest and origin signature verbatim.                      | `#/$defs/mirror_continuity_row`                                                                                |
| `channel_promotion_row`              | One promotion step (`quarantine → approved` or `approved → production`) pinned to `preserves_artifact_identity = true`. | `#/$defs/channel_promotion_row`                                                                                |
| `offline_bundle_manifest_row`        | Sealed offline-bundle export with per-artifact digests, signatures, publisher-continuity snapshots, lockfile parity, revocation-snapshot age, and deterministic restore preview. | `#/$defs/offline_bundle_manifest_row`                                                                          |
| `local_archive_import_row`           | Side-loaded artifact row; pins `trust_badge_inheritance_rule` to `capped_at_unverified_on_local_archive` or `quarantined_cannot_inherit`. | `#/$defs/local_archive_import_row`                                                                             |

## Registry-source classes

Every `registry_manifest_row` carries exactly one
`registry_source_class`. The six-class vocabulary is a narrowing
of the ADR-0012 parent (`public_registry`, `private_registry`,
`mirror`, `offline_bundle`, `vendored_local`). Rows rendered against
the ADR-0012 schema project `approved_mirror` onto `mirror`, and
`local_archive` / `quarantined_local_copy` onto `vendored_local`
with an explicit quarantine marker.

| Source class                 | ADR-0012 projection | Default trust inheritance                           | Admits channel promotion | Admits offline export |
|------------------------------|---------------------|-----------------------------------------------------|--------------------------|-----------------------|
| `public_registry`            | `public_registry`   | `inherits_origin_tier`                              | yes                      | yes                   |
| `approved_mirror`            | `mirror`            | `capped_at_community_on_approved_mirror`            | yes                      | yes                   |
| `private_registry`           | `private_registry`  | `capped_at_organisational_on_private_registry`      | yes                      | yes                   |
| `offline_bundle`             | `offline_bundle`    | `capped_at_community_on_approved_mirror`            | no                       | yes                   |
| `local_archive`              | `vendored_local`    | `capped_at_unverified_on_local_archive`             | no                       | no                    |
| `quarantined_local_copy`     | `vendored_local` + explicit marker | `quarantined_cannot_inherit`        | no                       | no                    |

## Content-addressed identity

Every row binds the artifact through a `content_address` pair:

```
{
  "digest_algorithm": "sha256",
  "digest_hex": "<lowercase hex digest of the artifact bytes>",
  "digest_size_bytes": 1234567
}
```

The pair is the artifact's identity. A public_registry row, an
approved_mirror row, a private_registry row, an
offline_bundle_artifact_entry, and a local_archive_import_row that
refer to the same artifact MUST carry the same pair verbatim. A
channel_promotion_row carries `preserves_artifact_identity = true`
as a pinned const: the schema rejects the row otherwise.

Digests and signatures travel separately: the artifact digest
travels inside `content_address`; the signature or attestation
travels through `signature_ref` (and `attestation_bundle_ref` when
`signature_class ∈ {attestation_bundle, dual_signed_publisher_and_attestation}`).
Raw signing-key material, raw attestation-bundle bytes, and raw
artifact bytes never cross this boundary.

## Channel lanes

Three lanes, promotion monotone:

```
quarantine ──▶ approved ──▶ production
```

- **`quarantine`** is the default landing lane for new
  publications from public_registry, approved_mirror, and
  private_registry. Install is gated on step-up approval, and the
  install-review surface renders the full declared-vs-effective
  permission diff plus the full compatibility-notes entry set.
- **`approved`** is reserved for artifacts that cleared review but
  are not yet general availability. Promotion from `quarantine`
  requires at least one `approvers_ref` entry and at least one
  `required_evidence_refs` entry.
- **`production`** is general availability. An artifact reaches
  production only through a `channel_promotion_row` whose
  `promotion_outcome = promoted` and `preserves_artifact_identity =
  true`.

Reverse moves are not expressed as promotion rows. Revocation,
emergency disable, and publisher quarantine move
`approval_state_class` to `revoked`, `emergency_disabled`, or
`blocked_by_policy` rather than producing a
`channel_promotion_row` with a downgraded channel. This preserves
the invariant that every `channel_promotion_row` advances the lane
order.

## Trust-inheritance matrix

Trust inheritance is capped at the transport. `inherits_origin_tier`
is admitted only on rows whose transport re-verifies the origin
signature with a non-stale revocation snapshot and whose narrowings
do not attempt widening.

| Source class               | Admitted inheritance rules                                                                        | Default                                            |
|----------------------------|---------------------------------------------------------------------------------------------------|----------------------------------------------------|
| `public_registry`          | `inherits_origin_tier`                                                                            | `inherits_origin_tier`                             |
| `approved_mirror`          | `inherits_origin_tier`, `capped_at_community_on_approved_mirror`, `quarantined_cannot_inherit`    | `capped_at_community_on_approved_mirror`           |
| `private_registry`         | `inherits_origin_tier`, `capped_at_organisational_on_private_registry`, `quarantined_cannot_inherit` | `capped_at_organisational_on_private_registry`  |
| `offline_bundle`           | `inherits_origin_tier`, `capped_at_community_on_approved_mirror`, `quarantined_cannot_inherit`    | `capped_at_community_on_approved_mirror`           |
| `local_archive`            | `capped_at_unverified_on_local_archive`, `quarantined_cannot_inherit`                             | `capped_at_unverified_on_local_archive`            |
| `quarantined_local_copy`   | `quarantined_cannot_inherit`                                                                      | `quarantined_cannot_inherit`                       |

The schema `allOf` gates enforce the admitted set per source class;
a row that declares an inheritance rule outside its admitted set is
non-conforming.

## Mirror-continuity state

A mirror quotes the origin digest and origin signature verbatim and
records its continuity state in a `mirror_continuity_row`.

| State                                    | Meaning                                                                                           |
|------------------------------------------|---------------------------------------------------------------------------------------------------|
| `verified_live`                          | Mirror re-verified the origin signature against a fresh revocation snapshot.                       |
| `verified_cached`                        | Mirror served from cache against a warm-cached revocation snapshot.                                |
| `stale_cached`                           | Mirror served from cache against a degraded-cached or older revocation snapshot.                   |
| `broken_digest_mismatch`                 | Cached digest diverges from origin digest.                                                         |
| `broken_signature_mismatch`              | Cached signature does not verify.                                                                  |
| `broken_publisher_revoked`               | Origin publisher's signing key is revoked.                                                         |
| `broken_publisher_quarantined`           | Origin publisher is quarantined.                                                                   |
| `broken_attestation_missing`             | Attestation bundle expected but absent.                                                            |
| `narrowing_attempted_widening`           | Terminal rejection. Mirror tried to relax origin trust / signature posture; denied with `mirror_narrowing_attempted_widening`. |

A mirror MAY apply ADR-0012 narrowings (`deny_list_applied`,
`allow_list_applied`, `publisher_trust_floor_applied`,
`version_floor_applied`, `emergency_disable_applied`,
`signed_continuity_required_applied`) in `narrowings_applied`.
Widening is denied.

## Offline-bundle export and import

An offline bundle is a sealed, content-addressed envelope carrying:

- `bundle_content_address` — the outer envelope digest.
- `artifacts` — per-artifact entries with their own
  `content_address`, `signature_class`, `signature_ref`,
  `publisher_continuity_snapshot_ref`, `compatibility_notes`,
  `revocation_snapshot_ref`, `origin_registry_source_class`
  (restricted to `public_registry` / `approved_mirror` /
  `private_registry`), and `dependency_closure_refs`.
- `dependency_closure_complete` — true when every
  `dependency_closure_refs` target is resolvable in the bundle.
  A false value MUST be mirrored by a `block_on_missing_dependency`
  step in `deterministic_restore_preview`.
- `lockfile_parity_state` — one of `exact_match`,
  `additive_rebuild_allowed`, `drift_detected_blocking`, or
  `not_applicable_first_resolve`. `drift_detected_blocking` denies
  restore with `offline_bundle_restore_refused_lockfile_drift`
  and MUST be mirrored by a `block_on_lockfile_drift` preview step.
- `revocation_snapshot_age_class` and `revocation_snapshot_ref` —
  one of `fresh`, `warm_cached`, `degraded_cached`, `stale`, or
  `unverified_no_snapshot`. A `stale` or `unverified_no_snapshot`
  bundle MUST carry a `block_on_revocation_snapshot_stale` preview
  step; restore against it denies with
  `offline_bundle_restore_refused_revocation_snapshot_stale` unless
  the operator records an out-of-band attestation.
- `deterministic_restore_preview` — an ordered list of
  preview-step rows. The importer renders this list verbatim;
  reordering or hiding steps denies with
  `review_disclosure_incomplete`.

### Restore-preview step vocabulary

| Step class                                  | Renders                                                                                     |
|---------------------------------------------|---------------------------------------------------------------------------------------------|
| `install_artifact`                          | Artifact will install; no block, no prompt.                                                 |
| `reuse_already_present_artifact`            | Artifact is already installed at the same content-address; restore reuses it.                |
| `prompt_user_for_trust_decision`            | Restore will prompt the user (e.g. approved-mirror → local-archive demotion).                |
| `prompt_admin_for_policy_exception`         | Restore will prompt the admin (policy-pack narrowing would otherwise deny).                  |
| `block_on_missing_dependency`               | Restore blocks; bundle is not dependency-closed.                                             |
| `block_on_revocation_snapshot_stale`        | Restore blocks; revocation snapshot is stale / missing.                                      |
| `block_on_lockfile_drift`                   | Restore blocks; lockfile parity is `drift_detected_blocking`.                                |
| `block_on_artifact_identity_mutated`        | Restore blocks; the bundle's content-address does not match the expected artifact identity.  |

Every `block_on_*` and `prompt_*` step carries a
`repair_affordance_label`.

### Trust posture on restore

The bundle's `trust_claim_source` is pinned to `offline_bundle`.
`trust_badge_inheritance_rule` defaults to
`capped_at_community_on_approved_mirror`; it may declare
`inherits_origin_tier` only when every artifact entry's
`revocation_snapshot_age_class` is `fresh` or `warm_cached` and
the importer re-verifies origin signatures on restore. A bundle
whose posture does not meet both preconditions renders the capped
tier regardless of what the origin publisher row claims.

## Local archive and quarantined local copy

A `local_archive_import_row` records a side-loaded artifact. The
schema pins:

- `trust_claim_source ∈ {local_archive, quarantined_local_copy}`.
- `trust_badge_inheritance_rule ∈ {capped_at_unverified_on_local_archive, quarantined_cannot_inherit}`.
- `imported_from_path_class` — one of
  `workspace_relative_path`, `user_home_relative_path`,
  `removable_media`, `network_mount_class`, `process_stdin_stream`,
  `not_applicable`. The raw filesystem path never crosses the
  boundary.
- `user_attestation_label` — optional human-legible operator
  attestation.

A `quarantined_local_copy` row does not admit install; the install-
review surface renders `quarantined_local_copy_install_denied`.
The row exists so support export, claim manifests, and mutation-
journal entries can preserve provenance without admitting
execution.

## Parity hooks

Public registry, approved mirror, private registry, and offline-
bundle rows share one trust, compatibility, and anti-abuse
vocabulary:

- **Trust.** `trust_claim_source`, `trust_badge_inheritance_rule`,
  and `publisher_continuity_ref` are first-class fields on every
  row. Surfaces read these fields rather than invent registry-
  specific badges.
- **Compatibility.** Every row carries a non-empty
  `compatibility_notes` array whose members quote one of five
  `compatibility_claim_class` values and (optionally)
  `declared_host_contract_family_refs` (ADR-0012) and
  `declared_capability_world_refs` (ADR-0019). A surface that hides
  compatibility notes is denied with
  `review_disclosure_incomplete`.
- **Anti-abuse.** Every row carries an `anti_abuse_signals` array
  whose members come from the eight-value vocabulary. Signals are
  raised, never consumed: a raised signal never promotes or demotes
  the publisher-continuity row on its own, but it is rendered on
  install-review, permission-inspector, support-export, and claim-
  manifest surfaces verbatim.

Later discovery ranking, quarantine, and revocation surfaces MUST
reuse the same `content_address`, `signature_ref`,
`publisher_continuity_ref`, `compatibility_claim_class`, and
`anti_abuse_signal_class` values rather than minting parallel
vocabularies.

## Audit events reserved

Emitted on the ADR-0012 `extension_trust` stream. Raw artifact
bytes, raw signing-key material, raw attestation bytes, and raw
mirror-cache bodies MUST NOT appear on any event.

- `registry_manifest_indexed`
- `registry_manifest_digest_recomputed`
- `registry_manifest_signature_reverified`
- `mirror_continuity_verified`
- `mirror_continuity_broken`
- `mirror_narrowing_attempted_widening`
- `channel_promotion_submitted`
- `channel_promotion_approved`
- `channel_promotion_blocked_by_policy`
- `channel_promotion_revoked`
- `offline_bundle_exported`
- `offline_bundle_imported`
- `offline_bundle_restore_preview_rendered`
- `offline_bundle_restore_blocked_by_lockfile_drift`
- `offline_bundle_restore_blocked_by_revocation_stale`
- `local_archive_imported`
- `local_archive_trust_tier_capped`
- `quarantined_local_copy_recorded`
- `revocation_snapshot_refreshed`
- `revocation_snapshot_stale_detected`
- `anti_abuse_signal_raised`

## Denial reasons reserved

In addition to the ADR-0012 set:

- `registry_digest_mismatch`
- `registry_signature_mismatch`
- `registry_source_class_unsupported_on_target`
- `artifact_identity_mutated_on_repackage`
- `mirror_continuity_broken`
- `mirror_narrowing_attempted_widening`
- `channel_promotion_denied_by_policy`
- `channel_promotion_widened_trust_tier`
- `offline_bundle_restore_refused_lockfile_drift`
- `offline_bundle_restore_refused_revocation_snapshot_stale`
- `offline_bundle_restore_refused_missing_dependency`
- `local_archive_trust_tier_capped`
- `local_archive_signature_missing_denied_on_policy`
- `quarantined_local_copy_install_denied`
- `publisher_namespace_recently_quarantined`
- `review_disclosure_incomplete`

## Consumer expectations

The downstream surfaces below MUST read this seed rather than
invent registry-shaped fields:

- **Registry indexer and mirror adapter.** Index on
  `content_address`, `signature_ref`,
  `publisher_continuity_ref`, and `registry_source_class`;
  mirrors preserve the origin digest and signature verbatim.
- **Install / update review sheet and permission inspector.**
  Project `trust_claim_source`,
  `trust_badge_inheritance_rule`, `compatibility_notes`, and
  `anti_abuse_signals` on every row. A surface that hides any of
  these is denied with `review_disclosure_incomplete`.
- **Offline-bundle exporter and importer.** Emit and consume
  `offline_bundle_manifest_row` rows verbatim. The importer
  renders `deterministic_restore_preview` in the order the
  exporter recorded; reordering or hiding steps is non-conforming.
- **Support export, mutation-journal entry, save manifest, claim
  manifest.** Carry the row id, content-address, signature_ref,
  publisher-continuity ref, registry-source class, channel class,
  approval-state class, and trust-badge inheritance rule. Raw
  artifact or mirror-cache bytes forbidden.
- **SDK bindgen seed.** Binds to each record kind as a typed row;
  stability-window labels align with the
  `registry_manifest_schema_version` axis (successor ADR
  concrete).
- **Discovery ranking, quarantine, and revocation surfaces.**
  Reuse the content-address, compatibility-claim class, and
  anti-abuse signal class from this seed. Minting parallel
  vocabularies is non-conforming.

## Out of scope

- Running a registry, a mirror, or a marketplace UX. This seed
  freezes distribution truth, not implementation.
- Full freshness ceilings for revocation snapshots. The
  revocation-snapshot age classes are frozen; the quantitative
  ceilings per class land in the successor ADR.
- The export/import of the offline bundle envelope itself (format
  on disk, signing envelope, compression). The seed pins the
  row shape; the envelope format is a successor concern.
- Third-party registry integration beyond the `approved_mirror`
  and `private_registry` source classes.
