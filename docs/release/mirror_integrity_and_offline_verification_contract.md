# Mirror-integrity, revocation-propagation, and offline-verification packet contract

This document is the narrative companion to the per-artifact
mirror-integrity packet, the per-emergency-metadata
revocation-propagation record, and the per-bundle offline-verification
packet Aureline freezes for every release, install, update, About,
support, and mirror / offline review surface that needs to keep
mirrored and offline installs trustworthy when the public internet is
absent. It pins how integrity, revocation, and verification evidence
travel through approved mirrors, customer-managed mirrors, offline
bundles, and air-gapped relays; how repackaged or stale artifacts are
prevented from masquerading as unchanged origin artifacts; how
revocations / advisories / disable bundles / channel-freeze /
kill-switch / trust-root rotation metadata propagate onto downstream
targets; and how an offline or sovereign install can prove what was
verified locally, what was imported, what is stale, and what is
policy-pinned WITHOUT requiring a live vendor call.

Companion artifacts:

- [`/schemas/release/mirror_integrity_packet.schema.json`](../../schemas/release/mirror_integrity_packet.schema.json)
  — boundary schema for one `mirror_integrity_packet_record`
  projecting one artifact subject, one origin envelope, one mirror
  envelope, one artifact-identity-relationship class, one
  digest-continuity class, one signer-continuity class, one freshness
  envelope, one customer-mirror-identity envelope, one
  integrity-outcome class, the linkage envelope, the per-surface
  projection set, and the per-export redaction posture.
- [`/schemas/release/revocation_propagation_record.schema.json`](../../schemas/release/revocation_propagation_record.schema.json)
  — boundary schema for one `revocation_propagation_record`
  projecting one propagation kind (revocation / advisory / disable
  bundle / channel freeze / kill switch / trust-root rotation / yank),
  one subject envelope, one authoritative origin envelope, one
  propagation-status class, one propagation-age class, one
  overlap-window envelope, one last-imported-state envelope, the
  reconcile / refresh actions a target MUST take, the linkage
  envelope, the per-surface projection set, and the per-export
  redaction posture.
- [`/schemas/release/offline_verification_packet.schema.json`](../../schemas/release/offline_verification_packet.schema.json)
  — boundary schema for one `offline_verification_packet_record`
  projecting one packet kind (offline install bundle, sovereign install
  snapshot, mirror offline export, air-gapped evidence pack), one
  bundle source envelope, the per-artifact-family coverage rows for
  release payloads / update metadata / extensions / policy bundles /
  docs packs / models / symbols / support packets, the
  per-emergency-metadata coverage rows naming the propagation records
  and manual-import receipts that prove WHEN and HOW each emergency
  metadata kind arrived, the trust-root pointer block, the freshness
  envelope, the disclosure envelope, the linkage envelope, the
  per-surface projection set, and the per-export redaction posture.
- [`/fixtures/release/mirror_integrity_cases/`](../../fixtures/release/mirror_integrity_cases)
  — seed `mirror_integrity_packet_record`,
  `revocation_propagation_record`, and
  `offline_verification_packet_record` fixtures for the five required
  acceptance cases (approved mirror same artifact, approved mirror
  repackaged artifact, stale revocation metadata, offline bundle with
  verification packet, customer mirror identity mismatch).

Cross-linked artifacts already in the repository:

- [`/docs/release/artifact_verification_contract.md`](./artifact_verification_contract.md)
  and
  [`/schemas/release/artifact_verification_row.schema.json`](../../schemas/release/artifact_verification_row.schema.json)
  — per-artifact verification row, trust-root rotation state, and
  advisory-history contract. This contract layers the mirror-integrity
  packet ON TOP of the verification row: every `integrity_ok_*` mirror
  outcome MUST cite the verification row that records the local /
  mirror-continuity / publisher-asserted verification posture. The
  `artifact_subject_class`, `surface_class`, and `redaction_class`
  vocabularies are aligned across both contracts so a release / install
  / About / support / mirror surface can compose them without minting
  a second mirror-integrity language.
- [`/docs/release/supply_chain_trust_framework_matrix.md`](./supply_chain_trust_framework_matrix.md)
  and
  [`/artifacts/release/trust_framework_rows.yaml`](../../artifacts/release/trust_framework_rows.yaml)
  — supply-chain trust-framework matrix. Every mirror-integrity
  packet, propagation record, and offline-verification packet binds
  back here through `artifact_family_ref` and (where present)
  `trust_framework_row_ref` so the per-artifact trust expectations are
  the same across the live, mirrored, and offline paths.
- [`/docs/ecosystem/package_restore_and_mirror_continuity_contract.md`](../ecosystem/package_restore_and_mirror_continuity_contract.md)
  and
  [`/schemas/ecosystem/mirror_promotion_row.schema.json`](../../schemas/ecosystem/mirror_promotion_row.schema.json)
  — package-level mirror-promotion / restore-preview /
  offline-continuity contract. The mirror-integrity packet generalizes
  the package-level mirror-promotion row to ALL artifact families
  (release payloads, update metadata, extensions, policy bundles,
  docs packs, models, symbols, support packets) and quotes
  `mirror_promotion_row_refs` for the package-manager families where a
  package-promotion row already exists. This contract does NOT
  re-define package-restore semantics; it adds the release-level
  mirror-integrity surface that release / install / About / support
  surfaces quote.
- [`/docs/security/emergency_distribution_policy.md`](../security/emergency_distribution_policy.md),
  [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json),
  and
  [`/schemas/security/manual_import_receipt.schema.json`](../../schemas/security/manual_import_receipt.schema.json)
  — emergency distribution policy, emergency-action record, and
  manual-import receipt. The revocation-propagation record composes
  ONTO these records: every `propagated_via_manual_import` propagation
  status MUST cite a `manual_import_receipt_ref` and the
  `metadata_chain_ref` rooted at the authoritative origin. This
  contract does NOT re-define manual-import semantics; it freezes the
  release-level propagation-status / propagation-age / overlap-window /
  reconcile-action surface that release / update / install / About /
  support / mirror / offline review surfaces quote.
- [`/schemas/release/trust_root_rotation_state.schema.json`](../../schemas/release/trust_root_rotation_state.schema.json)
  — trust-root continuity and rotation state record. Every offline
  verification packet binds at least one `trust_root_rotation_state_ref`
  in its `trust_root_pointer_block` so an offline reviewer can resolve
  the rotation transition without leaving the bundle.
- [`/schemas/ecosystem/offline_continuity_card.schema.json`](../../schemas/ecosystem/offline_continuity_card.schema.json)
  — package-level offline-continuity card. The offline-verification
  packet quotes `offline_continuity_card_refs` for the
  package-manager families covered by the bundle and aligns its
  `refresh_path_class` vocabulary with the card's `import_route_class`.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  — artifact-family vocabulary the three records group against.
- [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
  — channel-and-branch matrix. Every record's `channel_row_ref`
  resolves here.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` sections on mirrored / offline / sovereign
  / air-gapped distribution, supply-chain hygiene, emergency response,
  revocation, and About / public-truth surfaces.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6.1
  (emergency-control chain), §22.8 (mirror and offline distribution),
  §25.7 / §25.8 (signing, verification, trust-root rotation, mirror
  continuity, emergency supersedence), and §26.7 (revocation and
  continuity).
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.2.1 (provenance
  / freshness / client-scope), Appendix N (verification chain,
  advisory linkage), and §7.11.13 (advisory and emergency-action
  verification lane).

If this document disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## 1. Why publish this now

The artifact-verification contract froze the per-artifact verification
row, the trust-root rotation state, and the advisory-history linkage
release / install / About / support / mirror surfaces quote. The
package-restore contract froze the package-level mirror-promotion row
and offline-continuity card. The emergency-distribution policy froze
the manual-import receipt and metadata-chain stub. The supply-chain
trust-framework matrix froze the per-artifact-family trust pattern
crosswalk.

What was still implicit was the **release-level mirror-integrity
packet**, the **release-level revocation-propagation record**, and the
**offline-verification packet** that aggregates verification rows AND
emergency-metadata propagation onto one reviewable bundle. Left
implicit, mirrored and offline installs would lose the ability to
distinguish:

1. **Same artifact vs. repackaged artifact through a mirror.** A
   mirror that serves the same digest under the same origin signature
   is operating safely. A mirror that re-packages the artifact and
   re-signs it under its own key is producing a different artifact and
   MUST NOT render as the unchanged public artifact.
2. **Customer-mirror identity preserved vs. mismatched.** A customer
   pinning a mirror identity ensures that ONLY that mirror serves
   traffic under that identity. A mismatch (a mirror operating under
   the wrong identity) MUST be a typed refusal state, not a silent
   acceptance.
3. **Live revocation metadata vs. stale revocation metadata.** A
   target whose last revocation metadata import is past grace cannot
   render a fresh-looking install posture. The propagation record
   freezes a typed `propagation_age_class` so targets cannot mask
   staleness.
4. **Emergency metadata that arrived through a managed pull vs. a
   manual import.** An emergency action delivered through the
   authoritative origin lives at the origin manifest. An emergency
   action delivered through a manual / mirrored / offline import MUST
   carry its receipt and its metadata-chain back to the authoritative
   origin so offline review can prove WHEN and HOW it arrived.
5. **Local-evidence offline install vs. imported-only offline
   install vs. stale offline install.** An offline install that
   verified evidence locally, an offline install that carries imported
   publisher claims only, an offline install whose validity has ended,
   and an offline install whose installable targets are policy-pinned
   are four different trust postures. Collapsing them sends the
   support reviewer to the wrong recovery path.

This contract freezes all five distinctions plus the
**release-level mirror-integrity packet** that release / install /
About / mirror / offline surfaces quote, the **release-level
revocation-propagation record** that release / update / install /
mirror / offline surfaces quote, and the **offline-verification
packet** that aggregates verification rows AND propagation records
onto one reviewable bundle for offline / sovereign / air-gapped
installs.

This is a **pre-implementation plan**. No mirror-integrity surface,
revocation-propagation surface, or offline-verification packet is
implemented at this revision. Every fixture is tagged `seeded` /
`proposed`; rows are not deleted, they are superseded by an ADR / RFC
recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## 2. Scope

Frozen at this revision on the mirror-integrity packet:

- One closed `artifact_subject_class` vocabulary with eight values
  mirroring the artifact family groups in
  /docs/release/supply_chain_trust_framework_matrix.md.
- One closed `origin_source_class` vocabulary with five values
  (official_release_pipeline, public_registry, private_registry,
  managed_authoritative_origin, third_party_origin).
- One closed `mirror_class` vocabulary with five values
  (approved_mirror, customer_managed_mirror, community_mirror,
  private_registry_mirror, vendor_operated_regional_mirror).
- One closed `mirror_resign_kind_class` vocabulary with five values
  mirroring /schemas/ecosystem/mirror_promotion_row.schema.json#signature_equivalence_class
  plus a `not_applicable` refusal.
- One closed `artifact_identity_relationship_class` vocabulary with
  five values where only `same_artifact` is positive; the four other
  classes are typed refusal states that block integrity.
- One closed `digest_continuity_class` vocabulary with three values
  (digests_match, digests_differ_repackaged, digests_unverifiable).
- One closed `signer_continuity_class` vocabulary with seven values
  keeping rotation-overlap and cross-signed-transition continuity
  distinct from the three blocked refusal states.
- One closed `freshness_age_class` vocabulary with six values
  (fresh, warm_cached, degraded_cached, stale, unverified, expired).
- One closed `customer_mirror_identity_class` vocabulary with five
  values (declared_and_pinned, declared_unpinned, undeclared,
  mismatched_against_pin, not_applicable).
- One closed `integrity_outcome_class` vocabulary with ten values:
  two positive (integrity_ok_same_artifact,
  integrity_ok_resigned_with_continuity) and eight typed refusal
  classes (blocked_repackaged_identity, blocked_digest_mismatch,
  blocked_signature_mismatch, blocked_signer_continuity,
  blocked_stale_freshness, blocked_customer_mirror_identity_mismatch,
  blocked_unknown_identity, blocked_policy).
- One closed `surface_class` with eight values, of which four
  (mirror_offline_review, support_bundle, install_review_sheet,
  about_provenance_panel) form the required floor.
- One closed `denial_reason_class` with twenty values used by review
  tooling to refuse a mirror-integrity packet that violates the
  schema's invariants.

Frozen at this revision on the revocation-propagation record:

- One closed `propagation_kind_class` with seven values
  (revocation_metadata, advisory_metadata, disable_bundle_metadata,
  channel_freeze_metadata, kill_switch_metadata,
  trust_root_rotation_metadata, yank_metadata).
- One closed `propagation_status_class` with seven values
  (live_authoritative_origin_only, propagated_to_approved_mirror,
  propagated_to_offline_bundle, propagated_via_manual_import,
  propagation_pending, propagation_blocked,
  propagation_superseded_by_later_metadata).
- One closed `propagation_age_class` with six values
  (fresh, within_grace, stale_past_grace, expired, never_propagated,
  unknown).
- One closed `overlap_class` with six values mirroring
  /schemas/release/trust_root_rotation_state.schema.json#overlap_posture_class.
- One closed `last_seen_state_class` with seven values
  (active, superseded_by_later_metadata, revoked, withdrawn, expired,
  never_seen, unknown).
- One closed `last_imported_via_class` with eight values aligned with
  the import_path vocabulary in
  /schemas/security/manual_import_receipt.schema.json plus
  managed_pull_from_authoritative_origin and runtime_preload and
  never_imported.
- One closed `reconcile_action_kind_class` with ten values naming the
  typed actions a target MUST take to bring its last-imported state
  into agreement with the authoritative origin.
- One closed `deadline_semantics_class` with five values aligned with
  the deadline language in
  /schemas/security/emergency_action_record.schema.json.
- One closed `denial_reason_class` with eighteen values.

Frozen at this revision on the offline-verification packet:

- One closed `packet_kind_class` with four values
  (offline_install_bundle, sovereign_install_snapshot,
  mirror_offline_export, air_gapped_evidence_pack).
- One closed `validity_window_class` with four values
  (within_validity, past_validity, unbounded_local_evidence,
  unknown).
- One closed `coverage_disclosure_class` with nine values mirroring
  the verification-summary projections in
  /schemas/release/artifact_verification_row.schema.json.
- One closed `packet_disclosure_summary_class` with seven values
  (all_local_evidence, mixed_local_and_imported, all_imported,
  stale_past_validity_blocked, missing_emergency_metadata_blocked,
  rotating_trust_root_pending_review, policy_pinned_only).
- One closed `refresh_path_class` with eight values aligned with
  /schemas/ecosystem/offline_continuity_card.schema.json#import_route_class
  plus the manual / removable-media / cross-domain transfer routes
  from /schemas/security/manual_import_receipt.schema.json.

Out of scope until a superseding decision row opens:

- Operating mirrors, registries, or revocation feeds. The task spec
  marks this out of scope.
- Implementing mirror sync, registry publication, or transparency-log
  query infrastructure. The contract freezes the records those systems
  must emit; it does not implement them.
- Final marketing copy for mirror-integrity / propagation / offline
  badge labels. The contract freezes the machine vocabulary; the copy
  resolves against it.

## 3. Mirror-integrity packet

A `mirror_integrity_packet_record` is the per-artifact projection that
release / install / About / mirror / offline surfaces quote when they
need to state EXACTLY whether a mirror is serving the same artifact as
the authoritative origin or a repackaged copy, under which signer
continuity, against which customer-mirror identity, and at what
freshness.

### 3.1 Origin and mirror envelopes

The packet binds one origin envelope (origin source class, origin ref,
origin digest ref, origin signature ref, origin signer ref, origin
signed at) and one mirror envelope (mirror class, mirror ref, channel
class, mirror digest ref, mirror signature ref, mirror resign kind
class, mirror operator role ref, policy owner ref). The schema enforces:

- `official_release_pipeline` origins MUST carry a non-null
  `origin_signature_ref`, `origin_signer_ref`, and `origin_signed_at`.
- `approved_mirror` mirrors MUST cite at least one
  `mirror_promotion_row_ref` so the package-level promotion decision
  is reviewable from the release-level mirror-integrity packet.
- `customer_managed_mirror` mirrors MUST carry a
  `customer_mirror_identity_class` of `declared_and_pinned`,
  `declared_unpinned`, or `mismatched_against_pin`. `undeclared` and
  `not_applicable` are non-conforming for a customer-managed mirror.

### 3.2 Identity, digest, and signer continuity

Three closed vocabularies keep three different distinctions distinct:

| Distinction | Vocabulary | Notes |
|---|---|---|
| Artifact identity | `artifact_identity_relationship_class` (5 values) | Only `same_artifact` is positive; collapsing `repackaged_identity` into `same_artifact` is non-conforming |
| Digest continuity | `digest_continuity_class` (3 values) | `digests_unverifiable` MUST NOT collapse into `digests_match` because the mirror cannot prove identity |
| Signer continuity | `signer_continuity_class` (7 values) | Three blocked classes (`signer_changed_blocked`, `signer_revoked_blocked`, `continuity_unknown_blocked`) stay distinct because the support-review path differs |

The schema enforces that any of `repackaged_identity`,
`digest_mismatch`, `signature_mismatch`, or `unknown_identity` blocks
the integrity outcome out of the two `integrity_ok_*` classes. Any
blocked signer-continuity class forces
`integrity_outcome_class = blocked_signer_continuity`.

### 3.3 Customer-mirror identity

A customer-managed mirror exposes a per-deployment mirror identity.
The packet captures four distinct postures plus a refusal state:

| Class | Meaning |
|---|---|
| `declared_and_pinned` | Operator has declared the mirror identity AND pinned it in policy |
| `declared_unpinned` | Operator has declared the mirror identity but no pin in policy yet |
| `undeclared` | Mirror identity is not declared (admissible only for non-customer-managed mirrors) |
| `mismatched_against_pin` | Observed mirror identity does NOT match the pinned identity — typed refusal state |
| `not_applicable` | Vendor- or community-operated mirrors that do not expose a per-customer mirror identity |

A `mismatched_against_pin` posture forces the integrity outcome to
`blocked_customer_mirror_identity_mismatch`. A wrong mirror cannot
serve traffic under another mirror's identity.

### 3.4 Freshness envelope

The packet binds one freshness envelope with three distinct age lanes:
`mirror_snapshot_age_class`, `revocation_snapshot_age_class`, and
`advisory_snapshot_age_class`. Stale, unverified, or expired
revocation snapshots force the integrity outcome to
`blocked_stale_freshness`. A mirror cannot serve a fresh-looking
artifact when its revocation snapshot is past grace.

### 3.5 Required surface set

Every `mirror_integrity_packet_record` MUST render onto at least these
four surfaces:

1. **Mirror / offline review** (`mirror_offline_review`)
2. **Support bundle** (`support_bundle`)
3. **Install review sheet** (`install_review_sheet`) — BEFORE the
   user confirms the install
4. **About / provenance panel** (`about_provenance_panel`)

Admissible secondary surfaces: `release_publication`, `update_center`,
`public_proof_packet`, `headless_dry_run_output`. Tooling MAY add more
required surfaces in a later additive-minor revision; it MUST NOT drop
one.

Every surface projection MUST list `artifact_identity_relationship_class`,
`integrity_outcome_class`, and `computed_at` as required fields.

## 4. Revocation-propagation record

A `revocation_propagation_record` is the per-emergency-metadata
projection that release / update / install / About / mirror / offline /
support surfaces quote when they need to state EXACTLY how a
revocation, advisory, disable bundle, channel freeze, kill switch,
yank, or trust-root rotation has propagated from the authoritative
origin onto a mirrored, offline, sovereign, or air-gapped target.

### 4.1 Propagation kind, status, age

Three closed vocabularies keep three different distinctions distinct:

| Distinction | Vocabulary | Notes |
|---|---|---|
| Propagation kind | `propagation_kind_class` (7 values) | Pins exactly which kind of trust-affecting metadata the record describes |
| Propagation status | `propagation_status_class` (7 values) | Distinguishes the four positive `propagated_*` outcomes from `live_authoritative_origin_only`, `propagation_pending`, `propagation_blocked`, and `propagation_superseded_by_later_metadata` |
| Propagation age | `propagation_age_class` (6 values) | Distinguishes `fresh`, `within_grace`, `stale_past_grace`, `expired`, `never_propagated`, and `unknown` |

The schema enforces:

- `expired` propagation age forces the status out of
  `live_authoritative_origin_only` and out of the `propagated_*`
  family; expired metadata cannot render as live or propagated.
- `never_propagated` propagation age forces the status to
  `live_authoritative_origin_only`, `propagation_pending`, or
  `propagation_blocked`.
- `revoked` last-seen state forces the status out of
  `live_authoritative_origin_only`; a target that has observed a
  revocation MUST have recorded the propagation onto the target.

### 4.2 Authoritative origin envelope

Every record carries an authoritative-origin envelope (origin label,
origin ref, signer ref, signed at, valid from, valid until,
supersedes-origin refs). The `valid_until` is nullable only for
permanent revocations or trust-root rotations whose validity is
open-ended.

### 4.3 Overlap or grace window

Closed six-value `overlap_class` aligned with
/schemas/release/trust_root_rotation_state.schema.json#overlap_posture_class:

| Class | Required fields |
|---|---|
| `no_overlap` | none |
| `planned_overlap` | non-null `overlap_starts_at` and `overlap_ends_at` |
| `dual_signed_overlap` | non-null `overlap_starts_at` and `overlap_ends_at` |
| `cross_signed_overlap` | non-null `overlap_starts_at` and `overlap_ends_at` |
| `emergency_no_overlap` | none |
| `legacy_grace_only` | non-null `grace_until` |

### 4.4 Last-imported state envelope

Captures the durable lifecycle state the target last observed:
`last_seen_state_class` (active, superseded_by_later_metadata,
revoked, withdrawn, expired, never_seen, unknown), `last_seen_at`
(nullable only when `never_seen`), `last_imported_via_class`
(managed_pull / mirror_sync / manual_file_import /
offline_bundle_import / removable_media_import /
cross_domain_transfer_import / runtime_preload / never_imported), and
the `last_import_receipt_ref` into the manual-import receipt (required
when the import path is one of the manual / mirrored / offline /
cross-domain classes).

### 4.5 Reconcile actions

Every record MUST carry at least one typed reconcile action. Closed
ten-value `reconcile_action_kind_class` covering refresh, re-verify,
pin-last-known-good, quarantine, restore, post-incident-review,
import-successor, narrow-scope, and the
`no_action_required_already_current` action (admissible only when the
target is already current).

Every reconcile action carries a `deadline_semantics_class` aligned
with the emergency-action deadline vocabulary plus a nullable
`deadline_at` (required for hard deadlines / install-blocking /
update-blocking semantics).

### 4.6 Required surface set

Every `revocation_propagation_record` MUST render onto at least these
four surfaces: `update_center`, `install_review_sheet`,
`mirror_offline_review`, and `support_bundle`. Every surface
projection MUST list `propagation_kind_class`,
`propagation_status_class`, and `propagation_age_class` as required
fields.

### 4.7 Linkage to authoritative records

The schema enforces that:

- `trust_root_rotation_metadata` records cite at least one
  `trust_root_rotation_state_ref`.
- `advisory_metadata` records cite at least one `advisory_record_ref`.
- `revocation_metadata` records cite at least one
  `revocation_record_ref`.
- `disable_bundle_metadata`, `channel_freeze_metadata`, and
  `kill_switch_metadata` records cite at least one
  `emergency_action_record_ref`.
- `propagated_via_manual_import` status records cite at least one
  `manual_import_receipt_ref`.

## 5. Offline-verification packet

An `offline_verification_packet_record` is the per-bundle projection
that an offline install, a sovereign install, an air-gapped lab, an
enterprise mirror export, and a support / mirror / offline review
surface quote when they need to inspect trust, revocation, freshness,
and emergency-metadata provenance for an offline or mirrored install
WITHOUT requiring a live vendor call.

### 5.1 Packet kind and bundle source envelope

Closed four-value `packet_kind_class`:

| Class | Meaning |
|---|---|
| `offline_install_bundle` | A sealed bundle for an offline install |
| `sovereign_install_snapshot` | A snapshot frozen for a sovereign install with full-loop self-hosting |
| `mirror_offline_export` | An export from an enterprise mirror for offline review |
| `air_gapped_evidence_pack` | A per-import evidence packet a fully air-gapped target carries after a manual import |

The bundle source envelope binds the bundle ref, bundle digest ref,
exported_at, evaluated_at, validity-window class, valid_until
(nullable when validity is unbounded or unknown), and exporter role.

`air_gapped_evidence_pack` packets cannot claim a `fresh`
`bundle_age_class` — there is no live origin path against which to
test freshness; they MUST report `warm_cached`, `degraded_cached`,
`stale`, `unverified`, or `expired`.

### 5.2 Per-artifact-family coverage

The packet MUST carry at least one `artifact_family_coverage` row.
Each row pins one `subject_class` (release_payload / update_metadata /
extension_package / policy_bundle / model_or_tool_pack / docs_pack /
support_or_proof_packet / symbol_or_debug_sidecar), one
`artifact_family_ref`, one `included_artifact_count` (≥ 1), one
non-empty `verification_row_refs` array, an optional
`mirror_integrity_packet_refs` array, and one
`coverage_disclosure_class` (locally_verified, imported_publisher_asserted,
mirrored_through_continuity, stale_past_validity, policy_pinned,
rotating_trust_root_pending_review, advisory_affected_review_required,
missing_evidence, unknown_provenance).

The `coverage_disclosure_class` vocabulary mirrors the
verification-summary projections from
/schemas/release/artifact_verification_row.schema.json so an offline
packet says exactly what its evidence asserts about each family
without inventing a second disclosure language.

### 5.3 Per-emergency-metadata coverage

The packet MUST carry at least one `emergency_metadata_coverage` row
so offline review can prove WHEN and HOW emergency state arrived. Each
row pins one `propagation_kind_class`, one
`revocation_propagation_record_ref` into the propagation record, the
`manual_import_receipt_refs` (required to be non-empty when the
metadata reached the bundle through any manual / mirrored / offline /
removable-media / cross-domain transfer path), the `metadata_chain_ref`
into the chain stub on the manual-import receipt or propagation
record, the `authoritative_origin_ref`, and the `imported_at`
timestamp.

This is the rule that satisfies the spec's
"manual emergency-import receipt and metadata-chain entries for
advisories, revocations, disable bundles, and channel-freeze
metadata so offline verification can prove when and how emergency
state arrived" requirement.

### 5.4 Trust-root pointer block

The packet MUST carry a non-empty `trust_root_rotation_state_refs`
list so an offline reviewer can resolve every rotation transition
without leaving the bundle, plus a `trust_root_freshness_class` so
the reviewer can see whether the rotation evidence is fresh.

### 5.5 Disclosure envelope

Closed seven-value `packet_disclosure_summary_class`:

| Class | Meaning | Required fields |
|---|---|---|
| `all_local_evidence` | Every coverage row carries `locally_verified` evidence | none |
| `mixed_local_and_imported` | Some rows are locally verified, some are imported | none |
| `all_imported` | Every coverage row is imported / publisher-asserted only | none |
| `stale_past_validity_blocked` | Bundle's signed validity window has ended | none (forced from `past_validity` validity window) |
| `missing_emergency_metadata_blocked` | One or more required emergency-metadata kinds is missing | none |
| `rotating_trust_root_pending_review` | Trust root is mid-rotation and the bundle binds the rotation-state record under review | none |
| `policy_pinned_only` | Every installable target is policy-pinned | non-empty `policy_pin_refs` |

Closed eight-value `refresh_path_class` aligned with
/schemas/ecosystem/offline_continuity_card.schema.json#import_route_class
plus the manual / removable-media / cross-domain transfer routes from
/schemas/security/manual_import_receipt.schema.json#import_path_class.

### 5.6 Required surface set

Every `offline_verification_packet_record` MUST render onto at least
these four surfaces: `mirror_offline_review`, `install_review_sheet`,
`about_provenance_panel`, and `support_bundle`. Every surface
projection MUST list `packet_kind_class`,
`packet_disclosure_summary_class`, and `evaluated_at` as required
fields.

## 6. Disclosure rules: local vs. imported vs. stale vs. policy-pinned

Offline and mirrored verification stays precise about which trust
decisions are local, imported, stale, or policy-pinned through three
mechanisms:

1. **Per-artifact verification rows** carry one
   `verification_summary_class` and one `mirror_or_offline_class`
   (see /docs/release/artifact_verification_contract.md). The
   per-coverage-row `coverage_disclosure_class` on the
   offline-verification packet mirrors that vocabulary so an offline
   packet's coverage row says exactly what its underlying verification
   row says.
2. **Per-bundle disclosure summary** rolls the per-coverage-row
   classes up into one `packet_disclosure_summary_class` so a release
   / install / About / mirror / offline reviewer sees at a glance
   whether the bundle is fully local, mixed, fully imported, stale,
   blocked, mid-rotation, or policy-pinned.
3. **Per-emergency-metadata coverage rows** carry the
   `manual_import_receipt_refs` and `metadata_chain_ref` so the
   bundle's emergency-metadata provenance is reviewable from the
   packet alone.

A bundle that mixes local and imported evidence MUST render as
`mixed_local_and_imported`. Collapsing it into `all_local_evidence`
is non-conforming because a side-loaded artifact would masquerade as
a release-pipeline-verified artifact.

A bundle whose signed validity has ended MUST render as
`stale_past_validity_blocked`. Collapsing it into
`mixed_local_and_imported` is non-conforming because the recovery
path differs (refresh vs. continue-as-mixed).

## 7. Manual emergency-import receipts and metadata chains

Every offline-verification packet whose emergency-metadata coverage
rows include rows that arrived through a manual / mirrored / offline /
removable-media / cross-domain transfer path MUST carry the matching
`manual_import_receipt_refs` and `metadata_chain_ref`. The receipts
and chain stubs live on
/schemas/security/manual_import_receipt.schema.json; the packet does
NOT redefine receipt semantics, it cites them.

This is what makes offline-verification packets "able to explain
emergency metadata provenance as well as artifact provenance":

- For artifact provenance, the packet cites
  `artifact_verification_row_refs` and `mirror_integrity_packet_refs`.
- For emergency metadata provenance, the packet cites
  `revocation_propagation_record_refs`, `manual_import_receipt_refs`,
  and the `metadata_chain_ref` rooted at the authoritative origin.

A reviewer who opens the packet can answer four questions for every
artifact and every emergency-metadata kind:

1. **What was verified locally?** → `coverage_disclosure_class =
   locally_verified` plus the cited `verification_row_ref`.
2. **What is asserted by the upstream publisher only?** →
   `coverage_disclosure_class = imported_publisher_asserted`.
3. **What is stale?** → `bundle_age_class = stale` /
   `revocation_metadata_age_class = stale_past_grace` /
   `validity_window_class = past_validity`.
4. **What is policy-pinned?** → `coverage_disclosure_class =
   policy_pinned` plus the cited `policy_pin_ref`.

## 8. Export and redaction posture

Every record carries an `export_envelope` with one `redaction_class`
(`metadata_safe_default`, `public_proof_safe`, `support_redacted`, or
`internal_only`) plus four export booleans (screenshot, support
bundle, public proof packet, offline review) plus the
`raw_private_material_excluded` boolean (which MUST be true) plus an
optional `omission_reasons` array.

Raw signature bytes, raw artifact bytes, raw mirror cache bodies, raw
URLs, raw hostnames, raw absolute paths, raw key material, raw private
mirror endpoints, raw tokens, raw certificate material, raw advisory
payloads, raw rotation policy bodies, raw transparency-log payloads,
and raw bundle bytes never cross any of these three boundaries.

## 9. Forbidden collapses

The contract is non-conforming if any of the following occur on a
release / update / install / About / support / mirror / offline
surface:

- Rendering a `repackaged_identity` mirror-integrity packet as a
  `same_artifact` packet.
- Rendering a `digests_unverifiable` packet as a `digests_match`
  packet.
- Rendering a `mismatched_against_pin` customer-mirror identity as
  `undeclared` or `declared_unpinned`.
- Collapsing two distinct refusal signer-continuity classes
  (`signer_changed_blocked`, `signer_revoked_blocked`,
  `continuity_unknown_blocked`) into one bare label.
- Rendering an `expired` propagation-age record as `live_authoritative_origin_only`.
- Rendering a `revoked` last-seen state as `propagated` to a target
  that has not actually recorded the propagation.
- Dropping the `manual_import_receipt_ref` on a
  `propagated_via_manual_import` propagation status.
- Dropping the `trust_root_rotation_state_ref` on a
  `trust_root_rotation_metadata` propagation kind.
- Rendering a `past_validity` offline-verification packet as
  `mixed_local_and_imported`.
- Rendering an `air_gapped_evidence_pack` packet with `bundle_age_class
  = fresh`.
- Dropping the `metadata_chain_ref` on an emergency-metadata coverage
  row that arrived through a manual / mirrored / offline / removable-
  media / cross-domain transfer path.
- Exposing raw signature bytes, raw artifact bytes, raw mirror cache
  bodies, raw URLs, raw hostnames, raw absolute paths, raw key
  material, raw private mirror endpoints, raw tokens, raw certificate
  material, raw advisory payloads, raw rotation policy bodies, raw
  transparency-log payloads, or raw bundle bytes on any of the three
  records.
- Omitting one of the four required surface projections.

## 10. Worked fixtures

The five required acceptance cases are seeded under
[`/fixtures/release/mirror_integrity_cases/`](../../fixtures/release/mirror_integrity_cases):

1. **Approved mirror same artifact** —
   `approved_mirror_same_artifact_release_payload.yaml`. Aureline
   desktop stable 2.3.0 release payload retrieved through the Acme
   Corp enterprise mirror with origin signature reused; integrity
   outcome `integrity_ok_same_artifact` plus the linked
   verification row, mirror promotion row, and trust-root rotation
   state ref.
2. **Approved mirror repackaged artifact** —
   `approved_mirror_repackaged_blocked_extension_package.yaml`. A
   community-published extension repackaged by an approved mirror
   under its own signing key; integrity outcome
   `blocked_repackaged_identity` plus the typed denial reason.
3. **Stale revocation metadata** —
   `stale_revocation_metadata_propagation.yaml`. A revocation against
   a revoked extension version whose last propagation onto the
   customer offline bundle is past grace; propagation status
   `propagated_to_offline_bundle` with `propagation_age_class =
   stale_past_grace` and a `refresh_via_approved_mirror` reconcile
   action.
4. **Offline bundle with verification packet** —
   `offline_bundle_with_verification_packet.yaml`. A sovereign
   install snapshot bundling release payload, update metadata,
   policy bundle, docs pack, model pack, and symbols, plus the
   emergency-metadata coverage rows for the advisory, revocation,
   and trust-root rotation that arrived through the manual file
   import path; packet disclosure summary
   `mixed_local_and_imported` with the
   `manual_file_import_refresh` refresh path.
5. **Customer mirror identity mismatch** —
   `customer_mirror_identity_mismatch_release_payload.yaml`. An
   Aureline desktop release payload retrieved through what the
   policy pinned as the Acme Corp mirror but where the observed
   mirror identity does not match the pin; customer-mirror identity
   class `mismatched_against_pin` and integrity outcome
   `blocked_customer_mirror_identity_mismatch`.

## 11. Additive-minor change discipline

Adding a new `artifact_subject_class`, `origin_source_class`,
`mirror_class`, `mirror_resign_kind_class`,
`artifact_identity_relationship_class`, `digest_continuity_class`,
`signer_continuity_class`, `freshness_age_class`,
`customer_mirror_identity_class`, `integrity_outcome_class`,
`surface_class`, `projection_mode_class`, `redaction_class`, or
`denial_reason_class` on the mirror-integrity packet is
**additive-minor** and bumps `mirror_integrity_packet_schema_version`.

Adding a new `propagation_kind_class`, `propagation_status_class`,
`propagation_age_class`, `overlap_class`, `last_seen_state_class`,
`last_imported_via_class`, `reconcile_action_kind_class`,
`deadline_semantics_class`, `surface_class`, `projection_mode_class`,
`redaction_class`, or `denial_reason_class` on the
revocation-propagation record is additive-minor and bumps
`revocation_propagation_record_schema_version`.

Adding a new `packet_kind_class`, `validity_window_class`,
`coverage_disclosure_class`, `packet_disclosure_summary_class`,
`refresh_path_class`, `surface_class`, `projection_mode_class`,
`redaction_class`, or `denial_reason_class` on the
offline-verification packet is additive-minor and bumps
`offline_verification_packet_schema_version`.

Repurposing an existing value, weakening any `allOf` invariant, or
collapsing two refusal states into one bare label is **breaking** and
requires a new decision row in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
