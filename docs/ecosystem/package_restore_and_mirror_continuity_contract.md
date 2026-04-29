# Package restore, mirror-promotion, and offline-continuity contract

This document freezes the package-level trust surface shared by package
restore previews, mirror promotion, offline bundles, local archive imports,
support exports, admin exports, and reproducibility capsules.

Machine-readable companions:

- [`/schemas/ecosystem/package_restore_preview.schema.json`](../../schemas/ecosystem/package_restore_preview.schema.json)
  defines `package_restore_preview`.
- [`/schemas/ecosystem/mirror_promotion_row.schema.json`](../../schemas/ecosystem/mirror_promotion_row.schema.json)
  defines `mirror_promotion_row`.
- [`/schemas/ecosystem/offline_continuity_card.schema.json`](../../schemas/ecosystem/offline_continuity_card.schema.json)
  defines `offline_continuity_card`.
- [`/fixtures/ecosystem/package_restore_cases/`](../../fixtures/ecosystem/package_restore_cases/)
  contains worked public-registry, approved-mirror, offline-bundle,
  local-archive, and quarantined-installed-copy cases.

Related contracts remain authoritative for their own layers:

- [`/docs/package/package_action_contract.md`](../package/package_action_contract.md)
  owns package action and review packet vocabularies.
- [`/docs/execution/package_manager_and_lockfile_safety_contract.md`](../execution/package_manager_and_lockfile_safety_contract.md)
  owns package-change plans and registry-source records.
- [`/docs/extensions/registry_and_offline_bundle_seed.md`](../extensions/registry_and_offline_bundle_seed.md)
  owns extension registry, mirror, offline-bundle, and local-archive
  vocabulary that this package contract mirrors at package-manager level.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  owns support-bundle export semantics.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  owns exact-build and reproducibility-pack identity propagation.

This contract does not implement registry sync, package download,
lockfile resolution, marketplace ranking, or offline bundle sealing. It
freezes the records those systems must emit before they can install,
update, restore, promote, import, or export package sets.

## Why This Exists

Package restore flows can produce identical final package sets through
very different trust paths: a live public registry, an approved mirror, a
sealed offline bundle, a local archive, or a previously installed copy.
Those paths are not interchangeable. A deterministic package set is useful
only if reviewers can still see where each artifact came from, whether it
is byte-identical to the original artifact, and which revocation, yank,
advisory, and publisher-continuity evidence was available at decision
time.

The core rule is:

> Same package set does not mean same source class.

Restore, support, admin, and reproducibility exports MUST preserve source
class truth even when `package_id`, `package_version`, and artifact digest
match exactly.

## Record Roles

| Record | Purpose |
| --- | --- |
| `package_restore_preview` | One row per package candidate before install, update, import, reuse, or restore. It carries package id, version, artifact digest, source class, host range, permission digest, lock state, revocation/yank status, artifact identity, source lineage, restore decision, and export linkage. |
| `mirror_promotion_row` | One row per mirror promotion decision. It compares original source and mirror source, says whether the mirror is serving the same artifact or a repackaged identity, and carries signer continuity plus freshness. |
| `offline_continuity_card` | One card per offline package set or bundle. It explains snapshot age, included revocation/advisory data, included publisher-continuity metadata, source-class preservation, and refresh/import path. |

Each record carries refs and class labels, not raw package bodies, raw
lockfile bodies, raw registry URLs, raw hostnames, raw absolute paths,
raw artifact bytes, raw mirror cache bodies, tokens, certificates, or
signing-key material.

## Source Classes

The package restore source-class vocabulary is intentionally small:

| Source class | Meaning | Restore posture |
| --- | --- | --- |
| `public_registry` | Live or cached source is the public registry for the package ecosystem. | May install when artifact identity, lock state, and revocation/yank checks pass. |
| `approved_mirror` | Source is an approved mirror with a `mirror_promotion_row`. | May install only when promotion proves `same_artifact` or a governed policy explicitly accepts the mirror path. |
| `private_registry` | Source is an internal or managed registry. | May install when policy, auth, artifact identity, and revocation/yank checks pass. |
| `offline_bundle` | Source is a sealed offline package set with an `offline_continuity_card`. | May install only within the bundle's freshness and included-trust-data posture. |
| `local_archive` | Source is a side-loaded local archive or vendored artifact. | Requires local-archive import evidence and cannot inherit a public-registry trust badge silently. |
| `installed_copy` | Source is an already installed package copy being reused or audited. | May be reused only when digest, lock state, and revocation/yank posture permit. |
| `quarantined_installed_copy` | Source is an installed copy retained for evidence after quarantine or revocation. | Must block restore; may appear in support/admin exports as provenance. |

Package-action schemas use package-manager-specific classes such as
`public_default_registry`, `customer_operated_mirror`, or
`offline_bundle_registry`. Restore previews normalize those into the
source classes above while retaining refs back to the original
registry-source record.

## Restore Preview

Every pre-apply package restore emits a `package_restore_preview`.

Required truth:

| Field family | Required facts |
| --- | --- |
| Package identity | `package_id`, `package_version`, optional requested requirement, and package-manager family. |
| Artifact identity | `artifact_digest` plus artifact-identity class: origin artifact, mirrored same artifact, offline-bundle same artifact, local-archive attested artifact, installed-copy reuse, repackaged identity blocked, or unknown identity blocked. |
| Source class | Current and original source class, source refs, and `source_class_preserved = true`. |
| Host range | Compatible host range class, host-family refs, platform refs, and evidence ref. |
| Permission digest | Digest of the declared permission set or explicit empty-set digest for package ecosystems that have no package permission model. |
| Lock state | Lockfile ref, entry ref, lockfile digest, resolver identity, locked source class, and whether the lock matches the candidate artifact. |
| Revocation/yank status | Active, yanked, revoked, publisher quarantined, artifact quarantined, or unknown requiring refresh, plus evidence source. |
| Restore action | Allowed, reuse allowed, user/admin prompt, or blocked reason class. |
| Reproducibility linkage | Deterministic package-set digest plus support/admin export refs with `source_class_truth_preserved = true`. |

Rules:

- Approved-mirror previews MUST cite a `mirror_promotion_row`.
- Offline-bundle previews MUST cite an `offline_continuity_card`.
- Local-archive previews MUST cite a local archive import ref.
- Quarantined installed copies MUST block restore and cite the installed
  copy ref.
- Repackaged or unknown identity MUST block. The preview may offer repair
  guidance, but it cannot render as an unchanged public-registry artifact.
- Revoked, publisher-quarantined, or artifact-quarantined rows MUST block
  restore. A yanked row may block or prompt depending on policy, but the
  yank state remains visible.

## Mirror Promotion

A `mirror_promotion_row` exists so approved mirrors can narrow policy,
availability, or channel lanes without changing artifact identity.

Required truth:

| Field family | Required facts |
| --- | --- |
| Original source | Source class, source ref, artifact digest, signature ref, publisher-continuity ref, and revocation/yank status. |
| Mirror source | Mirror ref, mirror source class, channel, artifact digest, signature ref, and policy owner ref. |
| Artifact identity | `same_artifact`, `repackaged_identity`, `digest_mismatch`, `signature_mismatch`, or `unknown_blocked`. |
| Signer continuity | Origin signer verified, cached verified, stale requiring review, signer changed blocked, signer revoked blocked, or continuity unknown blocked. |
| Freshness | Mirror snapshot age, revocation snapshot age, advisory snapshot age, computed time, and evidence refs. |
| Outcome | Promoted to approved/production or blocked by identity, signer, freshness, or policy. |

Rules:

- A row may promote only when `identity_relationship = same_artifact` and
  `same_artifact = true`.
- Repackaging, digest mismatch, signature mismatch, or unknown identity
  blocks promotion.
- Signer changes, signer revocation, or unknown signer continuity block
  promotion.
- Stale or absent revocation freshness blocks promotion until the mirror
  refreshes or a governed exception is recorded outside this row.
- Mirror signatures may wrap origin signatures, but wrapping must not hide
  the origin signature or change the artifact digest.

## Offline Continuity

An `offline_continuity_card` is the reviewable summary for an offline
package set. It must be readable in the IDE, CLI, support export, and
admin export without unpacking raw bundle bytes.

Required truth:

| Field family | Required facts |
| --- | --- |
| Bundle source | Bundle ref, bundle digest, and original source classes represented inside the bundle. |
| Snapshot age | Export time, evaluation time, age class, and policy-window ref. |
| Included trust data | Revocation data class, advisory data class, refs to included snapshots, artifact-digest manifest ref, and package count. |
| Publisher continuity | Per-artifact publisher continuity, refs-only requiring refresh, or not included and blocking restore. |
| Source preservation | Preserved source classes, restore-preview refs, and `source_class_truth_preserved = true`. |
| Refresh/import path | Direct registry refresh, approved mirror refresh, offline-bundle reimport, local-archive reimport with attestation, admin attestation required, or blocked with no refresh path. |
| Reproducibility linkage | Deterministic package-set digest plus support/admin export refs. |

Rules:

- Offline bundles MUST NOT claim live freshness. Snapshot age is rendered
  as `fresh`, `warm_cached`, `degraded_cached`, `stale`, or `unverified`.
- Missing revocation data, missing advisory data, or missing publisher
  continuity metadata is a card field, not hidden prose.
- An unverified snapshot blocks restore unless the operator records a
  separate governed attestation and the restore preview points at it.
- The card preserves the original source class of every package member.
  A bundle that contains a public-registry package and a mirror package
  exports both classes, even when both artifacts are byte-identical to a
  live package set.

## Reproducibility Export Linkage

`reproducibility_export_linkage` appears on all three record families.
It is the bridge to support bundles, admin exports, offboarding packages,
and reproducibility capsules.

The linkage always carries:

- `package_set_ref` for the deterministic set being inspected or shared;
- `deterministic_package_set_digest`, a digest over the ordered package
  set metadata, not raw package bytes;
- `source_class_truth_preserved = true`;
- `export_scope`;
- support and admin export refs.

Support and admin tools MUST use this linkage instead of reconstructing
source truth from lockfile text, package-manager logs, or UI copy. Two
exports with the same package-set digest but different source classes are
different trust records.

## Consumer Expectations

- Package UI and CLI restore previews read `package_restore_preview`
  before writing manifests, lockfiles, caches, or package state.
- Registry-source and package-change-plan records link to restore previews
  rather than duplicating source-class language.
- Mirror services emit `mirror_promotion_row` before serving a promoted
  package as an approved mirror artifact.
- Offline importers emit `offline_continuity_card` before rendering any
  bundle as restorable.
- Support bundles and admin exports preserve all three record families by
  ref and never flatten them into "from cache", "from mirror", or
  "offline" prose.
- Reproducibility capsules preserve deterministic package-set digest and
  source class together.

## Fixture Coverage

Worked cases live under
[`/fixtures/ecosystem/package_restore_cases/`](../../fixtures/ecosystem/package_restore_cases/):

- `public_registry_restore.yaml`
- `approved_mirror_restore.yaml`
- `approved_mirror_promotion_same_artifact.yaml`
- `approved_mirror_promotion_repackaged_blocked.yaml`
- `offline_bundle_restore.yaml`
- `offline_continuity_card.yaml`
- `local_archive_restore.yaml`
- `quarantined_installed_copy_restore.yaml`

Removing one of these coverage classes is a breaking change to the
pre-implementation package restore corpus.
