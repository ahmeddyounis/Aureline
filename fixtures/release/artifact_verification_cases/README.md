# Artifact verification, trust-root rotation, and advisory-history acceptance cases

These fixtures are the seed cases the artifact-verification contract
at
[`docs/release/artifact_verification_contract.md`](../../../docs/release/artifact_verification_contract.md)
defines. Each case pairs:

- one `artifact_verification_row_record` validated against
  [`schemas/release/artifact_verification_row.schema.json`](../../../schemas/release/artifact_verification_row.schema.json);
- where the row's `trust_root_pointer_class` is one of the rotation
  classes, one companion `trust_root_rotation_state_record` validated
  against
  [`schemas/release/trust_root_rotation_state.schema.json`](../../../schemas/release/trust_root_rotation_state.schema.json).

Every case:

- names one stable `verification_row_id` (and where present, one
  stable `rotation_state_id`);
- binds exactly one `verification_summary_class`, one
  `signature_state_class`, one `attestation_state_class`, one
  `sbom_availability_class`, one `checksum_state_class`, one
  `trust_root_pointer_class`, one `mirror_or_offline_class`, one
  `advisory_or_compromise_class`, and one `verifier_class`;
- declares the four floor surfaces (`release_publication`,
  `update_center`, `install_review_sheet`, `about_provenance_panel`)
  with `verification_summary_class`, `source_or_origin_class`, and
  `verified_at` listed as required fields on every projection.

## Case list

The five required acceptance cases:

- `official_verified_release_payload_row.yaml` +
  `release_signing_root_current_row.yaml` —
  official Aureline desktop-payload release signed under the current
  release-signing root; release-pipeline verifier produced a local
  verification log; positive `verified_locally` posture across
  signature, attestation, SBOM, and checksum.
- `mirrored_verified_release_payload_row.yaml` —
  enterprise-mirror copy of the same desktop payload retrieved through
  an enterprise mirror that preserves origin signatures;
  mirror-continuity verifier validated origin continuity;
  `mirrored_origin_verified_through_continuity` summary class quoting
  the same `release_signing_root_current_row.yaml` rotation-state record.
- `side_loaded_publisher_asserted_extension_row.yaml` —
  side-loaded extension package admitted from a workspace archive
  carrying a publisher signature; no local verification has occurred;
  `publisher_asserted` summary class with the
  `publisher_asserted_no_local_verification` verifier.
- `rotating_trust_root_dual_signed_overlap_row.yaml` +
  `release_signing_root_dual_signed_overlap.yaml` —
  release payload validated during the active dual-signed overlap
  window of the release-signing root rotation;
  `rotating_trust_root_pending_review` summary class quoting the
  rotation-state record on the `rollout_active_overlap_period` epoch
  with `dual_signed_overlap` overlap posture.
- `advisory_affected_extension_package_row.yaml` —
  extension package affected by an active advisory on the artifact
  family; `advisory_affected_review_required` summary class with a
  non-empty `advisory_refs` array.

Every case cites companion exact-build identities, channel rows,
artifact-family map entries, release-evidence packets, About cards,
release-candidate cards, support bundles, advisory records,
trust-root rotation-state records, mirror-continuity records, and
transparency-log records by stable ref so release, install, update
center, About, support, and mirror / offline surfaces can pivot in
O(1) from one verification row to the canonical proof chain.
