# Artifact verification row, trust-root rotation state, and artifact-compromise history contract

This document is the narrative companion to the per-artifact
verification row, the trust-root continuity / rotation state record,
and the artifact-compromise / advisory history linkage Aureline freezes
for every release, install, update, and About surface that says
EXACTLY what was verified for one artifact, under which trust root,
and what compromise or advisory history applies. It pins which
verification states exist, which source / origin classes are
admissible, which mirrored / side-loaded / offline postures stay
distinct, how trust-root continuity and rotation state are surfaced
without minting a second rotation language, and how compromise /
advisory history is reachable from one artifact row instead of being
reassembled by hand across docs.

Companion artifacts:

- [`/schemas/release/artifact_verification_row.schema.json`](../../schemas/release/artifact_verification_row.schema.json)
  — boundary schema for one `artifact_verification_row_record`
  projecting artifact identity, source-or-origin envelope, verification
  envelope (signature, attestation, SBOM, checksum, trust-root pointer,
  mirror-or-offline class, advisory-or-compromise class, verifier-class,
  verification time, log ref, evidence refs), advisory-history
  envelope, the per-surface projection set, and the per-export
  redaction posture.
- [`/schemas/release/trust_root_rotation_state.schema.json`](../../schemas/release/trust_root_rotation_state.schema.json)
  — boundary schema for one `trust_root_rotation_state_record`
  projecting the trust-root family identity, the current root, the
  supersedes / revokes linkage, the rollout epoch, the overlap posture,
  the continuity statement, the stale-or-rotating state, and the
  per-surface projection set.
- [`/fixtures/release/artifact_verification_cases/`](../../fixtures/release/artifact_verification_cases)
  — seed `artifact_verification_row_record` and
  `trust_root_rotation_state_record` fixtures for the five required
  acceptance cases (official verified artifact, mirrored verified
  artifact, side-loaded publisher-asserted artifact, rotating trust
  root, advisory-affected artifact).

Cross-linked artifacts already in the repository:

- [`/docs/release/supply_chain_trust_framework_matrix.md`](./supply_chain_trust_framework_matrix.md),
  [`/artifacts/release/trust_framework_rows.yaml`](../../artifacts/release/trust_framework_rows.yaml),
  and
  [`/schemas/release/provenance_stack_exception.schema.json`](../../schemas/release/provenance_stack_exception.schema.json)
  — supply-chain trust-framework matrix and provenance-stack
  deviation policy. Every `trust_framework_row_ref` on a
  `current_root` resolves here; every `linked_provenance_stack_exception_ref`
  on a rotation linkage resolves into a deviation record.
- [`/schemas/governance/provenance_badge.schema.json`](../../schemas/governance/provenance_badge.schema.json)
  and
  [`/docs/governance/provenance_badge_contract.md`](../governance/provenance_badge_contract.md)
  — provenance badge, license row, notice row, and supply-chain status
  contract. The verification row's `signature_state_class`,
  `attestation_state_class`, `sbom_availability_class`,
  `checksum_state_class`, `advisory_or_compromise_class`, `surface_class`,
  and `redaction_class` vocabularies are aligned with the provenance-badge
  schema so a release / install / About surface can compose the badge
  and the verification row without minting a second verification
  language.
- [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  and
  [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  — exact-build identity model. Every verification row binds
  `artifact_identity.exact_build_identity_ref` here so the row can be
  diffed against sibling verification rows of the same exact build.
- [`/schemas/release/release_candidate_card.schema.json`](../../schemas/release/release_candidate_card.schema.json)
  and
  [`/docs/release/release_status_surface_contract.md`](./release_status_surface_contract.md)
  — release-candidate card, version-bump row, promotion timeline,
  and rollback / revocation panel. The release-candidate card quotes
  one or more verification rows by ref through its
  `artifact_provenance_links`; this contract resolves the row state
  the card depends on.
- [`/schemas/about/about_card.schema.json`](../../schemas/about/about_card.schema.json),
  [`/schemas/about/reproducibility_packet.schema.json`](../../schemas/about/reproducibility_packet.schema.json),
  and
  [`/docs/about/about_provenance_and_boundary_contract.md`](../about/about_provenance_and_boundary_contract.md)
  — About card, reproducibility packet, and openness boundary
  contract. About surfaces quote verification rows by ref so build
  identity, signature / attestation state, SBOM availability,
  trust-root pointer, mirror-or-offline class, and advisory linkage
  render without scattered docs lookup.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  — artifact-family vocabulary the verification row groups against
  through `artifact_identity.artifact_family_ref`.
- [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
  — channel-and-branch matrix. Every verification row's
  `artifact_identity.channel_row_ref` resolves here.
- [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  — decision index. Every rotation record cites a stable
  `rotation_decision_row_ref` here so the rotation is reviewable as
  an architecture / security decision.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` sections on code signing, release
  engineering, supply-chain hygiene, mirrored / offline distribution,
  emergency response, and About / public-truth surfaces.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §25.7 and
  §25.8 (signing, verification, trust-root rotation, mirror continuity,
  emergency supersedence).
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.2.1 and
  Appendix N (provenance / freshness / client-scope, verification chain,
  advisory linkage).

If this document disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## 1. Why publish this now

The supply-chain trust-framework matrix froze the artifact family /
trust pattern crosswalk. The provenance-badge contract froze the
compact badge plus detail row vocabulary. The release-candidate card,
update-manifest, and rollback / revocation panel contracts froze the
release-status surface. The About card and reproducibility-packet
contracts froze the canonical product self-description.

What was still implicit was the **per-artifact verification row** —
the one row a release publication, an update center, an install review
sheet, an About panel, a support export, and a mirror / offline review
each quote when they need to state EXACTLY what was verified for ONE
artifact, under which trust root, and what compromise or advisory
history applies. Left implicit, every surface would re-invent its own
verification copy and silently collapse seven distinguishable states
into one bare chip:

1. **Verified.** The release-pipeline / update-client / install-review
   verifier validated the signature, attestation, SBOM, and checksum
   against the current trust root and recorded a verification log
   entry.
2. **Publisher-asserted.** The artifact carries a publisher claim but
   no local verification has occurred. The verification row MUST say
   so explicitly rather than rendering as verified.
3. **Mirrored.** The artifact came through an enterprise or community
   mirror that preserves origin signatures. The row MUST say so
   explicitly and disclose whether origin continuity is intact or
   broken; collapsing "mirrored with continuity intact" with "verified
   locally against the live origin" is non-conforming.
4. **Missing.** The artifact lacks one or more required verification
   evidence kinds (signature, attestation, SBOM, or checksum). The
   row MUST say so explicitly rather than rendering as verified.
5. **Policy-pinned.** The installable target was pinned by a policy
   bundle. The row MUST cite the policy-pin ref so the support
   reviewer can pivot to the pinning policy.
6. **Rotating-trust-root.** The verification chain anchors a trust
   root that is mid-rotation (planned, dual-signed overlap,
   cross-signed overlap, post-overlap legacy grace, revoked, or
   emergency). The row MUST cite the rotation-state record so the
   rotation transition is reviewable in O(1).
7. **Advisory-affected.** An advisory or compromise event affects the
   artifact family. The row MUST cite the advisory and (when relevant)
   the compromise / remediation packets so the support reviewer can
   reach the proof chain from one artifact row instead of reassembling
   it from docs.

This contract freezes all seven projections plus the
**trust-root continuity and rotation state** record they bind to plus
the **advisory / compromise history linkage** they cite. It also
freezes the mirrored, side-loaded, and offline verification treatment
so current state stays precise about what was verified locally versus
asserted by an upstream publisher.

This is a **pre-implementation plan**. No release publication panel,
update-center verification chip, install-review verification row,
About panel verification disclosure, or rotation chip is implemented at
this revision. Every fixture is tagged `seeded` / `proposed`; rows are
not deleted, they are superseded by an ADR / RFC recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## 2. Scope

Frozen at this revision:

- One closed `artifact_subject_class` vocabulary with eight values
  (release_payload, update_metadata, extension_package, policy_bundle,
  model_or_tool_pack, docs_pack, support_or_proof_packet,
  symbol_or_debug_sidecar) mirroring the artifact family groups in
  the supply-chain trust-framework matrix.
- One closed `source_or_origin_class` vocabulary with eight values,
  of which seven (official_release, mirrored, side_loaded,
  third_party_import, policy_pinned, local_build, offline_bundle_imported)
  are positive source classes and one (unknown_provenance) is the
  explicit refusal class.
- One closed `verification_summary_class` vocabulary with eleven
  values that the seven distinguishable acceptance states project onto
  one-to-one (verified_locally, verified_with_history,
  publisher_asserted, mirrored_origin_verified_through_continuity,
  mirrored_with_continuity_warning, missing_evidence,
  policy_pinned_verified, rotating_trust_root_pending_review,
  advisory_affected_review_required, revoked_or_yanked,
  unknown_provenance).
- One closed `signature_state_class` with seven values (signed_locally_verified,
  signed_publisher_asserted, signed_present_unverified, signature_missing,
  signature_revoked, signature_mismatch, not_applicable). The three
  refusal states (missing, revoked, mismatch) stay distinct.
- One closed `attestation_state_class` with seven values mirroring
  the provenance-badge attestation vocabulary plus a publisher_asserted
  projection.
- One closed `sbom_availability_class` with eight values that include
  a publisher_asserted projection plus the sbom_referenced_only
  projection (typical for symbol or debug sidecars that reference the
  SBOM of the paired payload).
- One closed `checksum_state_class` with six values.
- One closed `trust_root_pointer_class` with nine values. The
  detail of the rotation transition lives on the linked rotation-state
  record; the row carries the pointer plus the rotation-state ref.
- One closed `mirror_or_offline_class` with eight values that
  distinguish the four mirrored postures plus the two offline postures
  plus the side-loaded posture and the air-gapped local-evidence
  posture.
- One closed `advisory_or_compromise_class` with eight values that
  union advisory_state and artifact_compromise_state from the
  provenance-badge schema.
- One closed `verifier_class` vocabulary with eleven values.
  publisher_asserted_no_local_verification names the case where the
  row carries a publisher claim but no local verification has
  occurred; collapsing it into one of the local-verifier classes is
  non-conforming.
- One closed `surface_class` with eleven values, of which four
  (release_publication, update_center, install_review_sheet,
  about_provenance_panel) form the required floor.
- One closed `projection_mode_class` with five values.
- One closed `redaction_class` with four values.
- One closed `denial_reason_class` with nineteen values used by review
  tooling to refuse a verification row that violates the schema's
  invariants.

Frozen at this revision on the rotation-state side:

- One closed `trust_root_family_class` vocabulary with nine values
  mirroring the artifact family groups in the supply-chain trust-framework
  matrix plus the mirror_continuity_root that anchors mirror snapshots.
- One closed `root_kind_class` with seven values mirroring the trust
  patterns in the matrix.
- One closed `rollout_epoch_class` with six values (rollout_planned,
  rollout_active_overlap_period, rollout_overlap_ended,
  rollout_completed_legacy_grace, rollout_revoked, rollout_emergency).
- One closed `overlap_posture_class` with seven values that keep
  dual_signed_overlap, cross_signed_overlap, single_signed_pre_rotation,
  single_signed_post_rotation, no_overlap_emergency, and
  no_overlap_planned_legacy_grace_only DISTINCT.
- One closed `continuity_statement_class` with five values
  (signed_continuity_statement, cross_signed_transition, exception_record,
  no_continuity_emergency, not_applicable_initial_root).
- One closed `stale_or_rotating_state_class` with nine values that
  the verification row's trust_root_pointer_class projects against
  verbatim.

Out of scope until a superseding decision row opens:

- Implementing signing, attestation, or trust-root infrastructure.
  The task spec marks this out of scope.
- Operating a transparency log, a key-rotation ceremony, or an emergency
  rotation runbook. Those are governance / security council authority.
- Final marketing copy for badge labels. The contract freezes the
  machine vocabulary; the copy resolves against it.

## 3. Verification-row state vocabulary

Closed eleven-value `verification_summary_class`. The seven
distinguishable acceptance states from the task spec project onto
these eleven values one-to-one so risky surfaces cannot ship one bare
chip that hides which state applies.

| Acceptance state | Verification summary class(es) | What the row MUST carry |
|---|---|---|
| Verified | `verified_locally`, `verified_with_history` | a local verifier class plus a non-null `verified_at` plus a non-null `verification_log_ref` plus locally-verified signature / attestation / SBOM / checksum projections |
| Publisher-asserted | `publisher_asserted` | the `publisher_asserted_no_local_verification` verifier plus a null `verified_at` plus a null `verification_log_ref` plus the publisher_asserted / present_unverified projections |
| Mirrored | `mirrored_origin_verified_through_continuity`, `mirrored_with_continuity_warning` | the `mirrored` source-or-origin class plus a non-null `mirror_ref` plus the `mirror_continuity_verifier` (continuity intact) or a continuity-broken posture |
| Missing | `missing_evidence` | at least one missing / present_unverified evidence kind; locally-verified projections are forbidden |
| Policy-pinned | `policy_pinned_verified` | the `policy_pinned` source-or-origin class plus a non-null `policy_pin_ref` |
| Rotating trust root | `rotating_trust_root_pending_review` | a rotation-class `trust_root_pointer_class` plus a non-null `trust_root_rotation_state_ref` |
| Advisory-affected | `advisory_affected_review_required` | an `active_advisory_affecting_family` / `compromise_suspected` / `compromise_confirmed` advisory class plus at least one `advisory_ref` |

Two additional refusal projections are admitted:

- `revoked_or_yanked` — the artifact has been revoked or yanked. The
  row MUST cite at least one `remediation_packet_ref` (replacement
  release, channel freeze, kill switch, or successor manifest).
- `unknown_provenance` — the source-or-origin class is
  `unknown_provenance`. The verifier class MUST be
  `publisher_asserted_no_local_verification` or `not_applicable` and
  the trust-root pointer MUST be `trust_root_unknown` or
  `not_applicable`.

### 3.1 Why these eleven projections stay distinct

Collapsing them into one bare chip ("verified") loses the support /
recovery path:

- **Verified locally vs. publisher-asserted.** Locally-verified means
  the verifier validated the artifact against the current trust root.
  Publisher-asserted means the artifact carries a claim but no local
  verification has occurred. Rendering them with the same chip lets a
  side-loaded artifact masquerade as a release-pipeline-verified
  artifact.
- **Mirrored with continuity vs. mirrored with broken continuity.**
  The first means origin signatures are still validating through the
  mirror; the second means continuity is broken and the row needs
  review. Collapsing them lets a stale or compromised mirror
  masquerade as an active mirror.
- **Missing vs. unknown provenance.** Missing means evidence is
  absent for an artifact whose source is known. Unknown means the
  source itself cannot be resolved. The remediation differs (request
  evidence vs. resolve provenance).
- **Rotating trust root vs. advisory-affected.** Rotating trust root
  means the verification chain anchors a root that is mid-rotation
  (review the rotation-state record). Advisory-affected means an
  advisory or compromise event affects the artifact family (review
  the advisory). Collapsing them sends the support reviewer to the
  wrong proof chain.

## 4. Source-or-origin envelope

Closed eight-value `source_or_origin_class`. Each class binds different
required fields in the schema's `allOf` block.

| Source class | Required envelope fields | Forbidden fields |
|---|---|---|
| `official_release` | `origin_ref` non-null, `acquired_via_class` is `release_pipeline` typically | `mirror_ref`, `import_receipt_ref`, `side_load_review_ref`, `policy_pin_ref` may be null |
| `mirrored` | `origin_ref` non-null, `mirror_ref` non-null | the mirrored row MUST cite the mirror; collapsing is non-conforming |
| `side_loaded` | `side_load_review_ref` non-null, `acquired_via_class` is `local_file_picker` / `workspace_archive` / `air_gapped_media` / `offline_bundle_media` | `acquired_via_class` of `release_pipeline` is forbidden because a side-loaded artifact did not flow through the release pipeline |
| `third_party_import` | `origin_ref` non-null pointing at the third-party origin | exact_build_identity_ref MAY be null |
| `policy_pinned` | `policy_pin_ref` non-null | — |
| `local_build` | `exact_build_identity_ref` non-null pointing at the developer build | channel_row_ref MAY be null |
| `offline_bundle_imported` | `import_receipt_ref` non-null, `mirror_or_offline_class` is `offline_bundle_within_validity` / `offline_bundle_expired` / `air_gapped_local_evidence_only` | — |
| `unknown_provenance` | none | `verifier_class` MUST be `publisher_asserted_no_local_verification` or `not_applicable` |

The envelope also carries a closed ten-value `acquired_via_class`
aligned with `provenance_badge.schema.json#source.acquired_via` so a
release / install / support reviewer can pivot in O(1) into the
acquisition channel without reconstructing it from prose.

## 5. Verification envelope

Every verification row MUST carry one
`verification_summary_class`, one `signature_state_class`, one
`attestation_state_class`, one `sbom_availability_class`, one
`checksum_state_class`, one `trust_root_pointer_class`, one
`mirror_or_offline_class`, one `advisory_or_compromise_class`, one
`verifier_class`, a `verified_at` timestamp, a `verification_log_ref`,
and a `verification_evidence_refs` array that names the proof chain a
reviewer can follow from this row alone.

### 5.1 Signature, attestation, SBOM, checksum

Each state vocabulary keeps a positive-locally-verified projection,
a positive-publisher-asserted projection, an unverified projection,
and the distinct refusal states (missing, revoked, mismatch, stale,
policy_blocked) so the row never collapses three different refusal
postures into one bare label.

The `publisher_asserted` summary class forbids the locally-verified
projections on signature, attestation, SBOM, and checksum because no
local verification has occurred. The `missing_evidence` summary class
forbids the locally-verified projections because at least one evidence
kind is absent.

### 5.2 Trust-root pointer

The verification row carries a `trust_root_pointer_class` plus a
`trust_root_rotation_state_ref` pointer into the rotation-state
record. The rotation-state record carries the rotation transition
detail (see §6); the row carries the pointer so a release / update /
install / About surface can pivot in O(1) without restating the
rotation.

The schema enforces that the six rotation-class pointer values
(`trust_root_rotating_dual_signed`, `trust_root_rotating_cross_signed`,
`trust_root_rotated_with_continuity`, `trust_root_rotation_pending_review`,
`trust_root_post_rotation_legacy_grace`, `trust_root_revoked`) MUST
resolve to a non-null `trust_root_rotation_state_ref`. The three
non-rotation pointer values (`trust_root_current`, `trust_root_unknown`,
`not_applicable`) MAY have a null ref.

### 5.3 Mirror-or-offline class

Closed eight-value `mirror_or_offline_class`. Distinguishes:

- `live_origin_no_mirror` — official-release path with no mirror.
- `mirrored_with_origin_continuity` — origin signatures still validate
  through the mirror.
- `mirrored_origin_continuity_broken` — origin continuity broken;
  needs review.
- `offline_bundle_within_validity` — offline bundle import receipt
  valid.
- `offline_bundle_expired` — offline bundle import receipt expired.
- `side_loaded_publisher_asserted_only` — side-loaded artifact carrying
  publisher claim only.
- `air_gapped_local_evidence_only` — air-gapped lab posture with local
  evidence packet only.
- `not_applicable` — explicit refusal state.

Collapsing any two of these into one chip is non-conforming because
the support-review path differs.

### 5.4 Advisory-or-compromise class

Closed eight-value `advisory_or_compromise_class` unioning
`advisory_state` and `artifact_compromise_state` from the
provenance-badge schema:

| Class | Meaning | Required advisory-history envelope fields |
|---|---|---|
| `none_known` | No advisory or compromise affects the family. | empty arrays |
| `active_advisory_affecting_family` | Active advisory; review required. | non-empty `advisory_refs` |
| `fixed_with_history` | Advisory was published, the artifact was patched, history is preserved. | non-empty `advisory_refs` and non-empty `fixed_with_history_event_refs` |
| `revoked_or_yanked` | The artifact was revoked or yanked. | non-empty `advisory_refs` and non-empty `remediation_packet_refs` |
| `compromise_suspected` | A compromise is suspected. | non-empty `advisory_refs` and non-empty `compromise_event_refs` |
| `compromise_confirmed` | A compromise is confirmed. | non-empty `advisory_refs` and non-empty `compromise_event_refs` |
| `compromise_remediated` | A confirmed compromise has shipped a remediation. | non-empty `advisory_refs`, `compromise_event_refs`, `fixed_with_history_event_refs`, and `remediation_packet_refs` |
| `advisory_unknown` | Advisory linkage cannot be resolved. | empty arrays |

The schema enforces that the `advisory_affected_review_required`
summary class binds only to `active_advisory_affecting_family`,
`compromise_suspected`, or `compromise_confirmed` because
`fixed_with_history` and `compromise_remediated` are explicitly NOT
review-required and MUST project as `verified_with_history` instead.

### 5.5 Verifier class

Closed eleven-value `verifier_class` keeping the per-pipeline
verifiers DISTINCT (release_pipeline_verifier, update_client_verifier,
install_review_verifier, extension_install_verifier,
policy_bundle_verifier, model_pack_verifier, support_export_verifier,
mirror_continuity_verifier, offline_bundle_import_verifier) plus the
explicit publisher-asserted-no-local-verification class plus the
not_applicable refusal class.

## 6. Trust-root continuity and rotation state record

The rotation-state record is the per-trust-root projection a
verification row binds to when its `trust_root_pointer_class` is one
of the six rotation classes.

### 6.1 Family identity and current root

Every record carries:

- One `trust_root_family_class` (release_signing_root,
  update_metadata_root, extension_publisher_root, policy_signer_root,
  model_pack_root, support_packet_root, docs_pack_root,
  symbol_or_debug_sidecar_root, mirror_continuity_root) mirroring the
  artifact family groups in the supply-chain trust-framework matrix.
- One `current_root` block carrying a stable `root_id` (pattern
  `^trust_root:[a-z0-9]+(?:[._-][a-z0-9]+)*$`), a reviewable
  `root_label`, one `root_kind_class` (TUF-style, Sigstore-style,
  platform-native, key-pinned, policy-signer, mirror-continuity),
  a `valid_from` timestamp, a `valid_until` timestamp (null only on
  emergency or revoked roots), a `fingerprint_ref` into the fingerprint
  catalog, an `issuer_ref` into the issuer record, and a
  `trust_framework_row_ref` into `trust_framework_rows.yaml`.

### 6.2 Rollout epoch and overlap posture

Closed six-value `rollout_epoch_class`. Each epoch binds a different
overlap posture and stale-or-rotating state in the schema's `allOf`
block:

| Epoch | Overlap posture | Stale-or-rotating state | Required fields |
|---|---|---|---|
| `rollout_planned` | `single_signed_pre_rotation` | `trust_root_rotation_pending_review` | non-null `rollout_started_at`, null `rollout_overlap_ended_at`, null `rollout_completed_at`; continuity statement MUST be `signed_continuity_statement` or `cross_signed_transition` |
| `rollout_active_overlap_period` | `dual_signed_overlap` or `cross_signed_overlap` | `trust_root_rotating_dual_signed` or `trust_root_rotating_cross_signed` | non-null `overlap_starts_at` and `overlap_ends_at` |
| `rollout_overlap_ended` | `single_signed_post_rotation` | `trust_root_rotated_with_continuity` | non-null `rollout_overlap_ended_at` |
| `rollout_completed_legacy_grace` | `no_overlap_planned_legacy_grace_only` | `trust_root_post_rotation_legacy_grace` | non-null `legacy_grace_until` and `rollout_completed_at` |
| `rollout_revoked` | (any) | `trust_root_revoked` | non-empty `revokes_root_refs`, non-null `successor_root_ref` |
| `rollout_emergency` | `no_overlap_emergency` | (operator-set) | continuity statement MUST be `no_continuity_emergency` (with null `continuity_statement_ref`); non-empty `revokes_root_refs` |

### 6.3 Continuity statement

Closed five-value `continuity_statement_class`:

| Class | Required `continuity_statement_ref` | Required `linked_provenance_stack_exception_ref` | Required `supersedes_root_refs` |
|---|---|---|---|
| `signed_continuity_statement` | non-null | null | non-empty |
| `cross_signed_transition` | non-null | null | non-empty |
| `exception_record` | non-null | non-null | non-empty |
| `no_continuity_emergency` | null | null | (any) |
| `not_applicable_initial_root` | null | null | empty |

Emergency rotations forbid a non-null `continuity_statement_ref`
because the predecessor root was compromised and cannot sign a
continuity statement.

### 6.4 Stale-or-rotating state class

Closed nine-value `stale_or_rotating_state_class` aligned with the
verification row's `trust_root_pointer_class` (eight shared values
plus one rotation-only value and one row-only value). The rotation
side adds `trust_root_stale` for the case where the rotation record
exists but has not been reviewed by its review forum within the
named freshness floor; collapsing it with `trust_root_unknown` is
non-conforming because the support-review path differs. The row side
adds `not_applicable` for artifacts that have no trust-root binding
(for example, a `local_build` developer build); a rotation-state
record always has a `current_root` so `not_applicable` is not
admitted there.

## 7. Mirrored, side-loaded, and offline verification treatment

The `mirror_or_offline_class` plus the `verifier_class` plus the
source envelope freeze how mirrored, side-loaded, and offline rows
disclose what was verified locally vs. asserted by the upstream
publisher.

- **Mirrored with origin continuity intact.** The verifier is
  `mirror_continuity_verifier`; the `mirror_or_offline_class` is
  `mirrored_with_origin_continuity`; the `verification_summary_class`
  is `mirrored_origin_verified_through_continuity`. The signature /
  attestation / SBOM / checksum states project the locally-verified
  projections because the origin signatures still validate through the
  mirror.
- **Mirrored with origin continuity broken.** The
  `mirror_or_offline_class` is `mirrored_origin_continuity_broken`;
  the `verification_summary_class` is `mirrored_with_continuity_warning`.
  The advisory-or-compromise class MUST be one of
  `active_advisory_affecting_family`, `advisory_unknown`, or
  `none_known` so the warning posture is bounded.
- **Side-loaded publisher-asserted.** The source class is
  `side_loaded`; the `mirror_or_offline_class` is
  `side_loaded_publisher_asserted_only`; the
  `verification_summary_class` is `publisher_asserted`. The signature
  state projects `signed_publisher_asserted`; the attestation, SBOM,
  and checksum project their publisher-asserted / present-unverified /
  missing projections.
- **Offline bundle within validity.** The source class is
  `offline_bundle_imported`; the `mirror_or_offline_class` is
  `offline_bundle_within_validity`; the verifier is
  `offline_bundle_import_verifier`. The verification summary may be
  `verified_locally` or `verified_with_history` depending on the
  evidence pack.
- **Offline bundle expired.** The `mirror_or_offline_class` is
  `offline_bundle_expired`; the verification summary cannot be
  `verified_locally` (the validity window has ended).
- **Air-gapped local evidence only.** The source class is
  `local_build` or `offline_bundle_imported`; the
  `mirror_or_offline_class` is `air_gapped_local_evidence_only`;
  the verifier is the matching local verifier.

## 8. Required surface set

Every `artifact_verification_row_record` MUST render onto at least
these four surfaces:

1. **Release publication** (`release_publication`) — the row appears
   on the release-publication panel alongside the release-candidate
   card and the rollback / revocation panel.
2. **Update center** (`update_center`) — the row appears in the
   update-center entry for the artifact, BEFORE the user clicks
   "update".
3. **Install review sheet** (`install_review_sheet`) — the row
   appears on the install / extension-install review sheet, BEFORE
   the user confirms the install.
4. **About / provenance panel** (`about_provenance_panel`) — the row
   appears on the About panel through the About card's
   `verification_row_refs`.

Admissible secondary surfaces (not required but admitted by the
schema): `post_install_disclosure`, `marketplace_discovery`,
`package_explorer`, `support_bundle`, `public_proof_packet`,
`mirror_offline_review`, `headless_dry_run_output`. Tooling MAY add
more required surfaces in a later additive-minor revision; it MUST
NOT drop one.

Every surface projection MUST list `verification_summary_class`,
`source_or_origin_class`, and `verified_at` as required fields so the
row's positive / refusal posture and verification time always render.

## 9. Export and redaction posture

The row carries an `export_envelope` with one
`redaction_class` (`metadata_safe_default`, `public_proof_safe`,
`support_redacted`, or `internal_only`) plus four export booleans
(screenshot, support bundle, public proof packet, offline review)
plus the `raw_private_material_excluded` boolean (which MUST be true)
plus an optional `omission_reasons` array.

Raw signature bytes, raw attestation bodies, raw SBOM bodies, raw
advisory payloads, raw release prose, raw URLs, raw private mirror
endpoints, raw key material, and raw artifact bytes never cross this
boundary. The same constraint applies to the rotation-state record:
raw key material, raw signatures, raw transparency-log payloads, raw
rotation policy bodies, raw URLs, raw private trust-root endpoints,
and raw signing infrastructure identifiers never cross the rotation
boundary.

## 10. Forbidden collapses

The contract is non-conforming if any of the following occur on a
release / update / install / About / support / mirror surface:

- Rendering a `publisher_asserted` row as a `verified_locally` chip.
- Rendering a `mirrored_with_continuity_warning` row as a
  `mirrored_origin_verified_through_continuity` chip.
- Rendering a `missing_evidence` row as a `verified_with_history`
  chip.
- Dropping the `trust_root_rotation_state_ref` on a rotation-class
  row.
- Dropping the `advisory_refs` on an advisory-affected row.
- Collapsing `revoked_or_yanked` into `missing_evidence` or
  `unknown_provenance`.
- Collapsing two distinct refusal states (`signature_missing`,
  `signature_revoked`, `signature_mismatch`) into one bare label.
- Exposing raw key material, raw signatures, raw transparency-log
  payloads, raw advisory payloads, raw URLs, raw private mirror
  endpoints, or raw artifact bytes on any verification row or
  rotation record.
- Emitting role / verifier / issuer fields above the audience
  ceiling.
- Omitting one of the four required surface projections
  (`release_publication`, `update_center`, `install_review_sheet`,
  `about_provenance_panel`).

## 11. Worked fixtures

The five required acceptance cases are seeded under
[`/fixtures/release/artifact_verification_cases/`](../../fixtures/release/artifact_verification_cases):

1. **Official verified artifact** —
   `official_verified_release_payload_row.yaml` plus the linked
   `release_signing_root_current_row.yaml` rotation-state record on
   the `trust_root_current` posture.
2. **Mirrored verified artifact** —
   `mirrored_verified_release_payload_row.yaml` quoting the same
   rotation-state record under the `mirrored_origin_verified_through_continuity`
   summary class.
3. **Side-loaded publisher-asserted artifact** —
   `side_loaded_publisher_asserted_extension_row.yaml` under the
   `publisher_asserted` summary class with the
   `publisher_asserted_no_local_verification` verifier.
4. **Rotating trust root** —
   `rotating_trust_root_dual_signed_overlap_row.yaml` quoting the
   `release_signing_root_dual_signed_overlap.yaml` rotation-state
   record under the `rotating_trust_root_pending_review` summary
   class.
5. **Advisory-affected artifact** —
   `advisory_affected_extension_package_row.yaml` under the
   `advisory_affected_review_required` summary class with a non-empty
   `advisory_refs` array.

## 12. Additive-minor change discipline

Adding a new `artifact_subject_class`, `source_or_origin_class`,
`verification_summary_class`, `signature_state_class`,
`attestation_state_class`, `sbom_availability_class`,
`checksum_state_class`, `trust_root_pointer_class`,
`mirror_or_offline_class`, `advisory_or_compromise_class`,
`verifier_class`, `surface_class`, `projection_mode_class`,
`redaction_class`, or `denial_reason_class` is **additive-minor** and
bumps `artifact_verification_row_schema_version`.

Adding a new `trust_root_family_class`, `root_kind_class`,
`rollout_epoch_class`, `overlap_posture_class`,
`continuity_statement_class`, `stale_or_rotating_state_class`,
`surface_class`, `projection_mode_class`, `redaction_class`, or
`denial_reason_class` on the rotation side is additive-minor and bumps
`trust_root_rotation_state_schema_version`.

Repurposing an existing value, weakening any `allOf` invariant, or
collapsing two refusal states into one bare label is **breaking** and
requires a new decision row in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
