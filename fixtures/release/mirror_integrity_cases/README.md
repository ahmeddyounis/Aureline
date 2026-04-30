# Mirror-integrity, revocation-propagation, and offline-verification acceptance cases

These fixtures are the seed cases the mirror-integrity, revocation-
propagation, and offline-verification packet contract at
[`docs/release/mirror_integrity_and_offline_verification_contract.md`](../../../docs/release/mirror_integrity_and_offline_verification_contract.md)
defines. Each case validates against one of the three boundary
schemas:

- [`schemas/release/mirror_integrity_packet.schema.json`](../../../schemas/release/mirror_integrity_packet.schema.json)
- [`schemas/release/revocation_propagation_record.schema.json`](../../../schemas/release/revocation_propagation_record.schema.json)
- [`schemas/release/offline_verification_packet.schema.json`](../../../schemas/release/offline_verification_packet.schema.json)

Every case:

- names one stable `packet_id` or `record_id`;
- binds exactly one closed-vocabulary value per axis (artifact subject
  / origin source / mirror class / artifact-identity relationship /
  digest continuity / signer continuity / customer-mirror identity /
  integrity outcome on the mirror-integrity packet; propagation kind /
  status / age / overlap / last-seen state / last-imported via on the
  revocation-propagation record; packet kind / validity window /
  packet disclosure summary / refresh path on the offline-verification
  packet);
- declares the four floor surfaces (`mirror_offline_review`,
  `support_bundle`, `install_review_sheet`,
  `about_provenance_panel`) on mirror-integrity packets and
  offline-verification packets, and (`update_center`,
  `install_review_sheet`, `mirror_offline_review`, `support_bundle`)
  on revocation-propagation records;
- excludes raw signature bytes, raw artifact bytes, raw mirror cache
  bodies, raw URLs, raw hostnames, raw absolute paths, raw key
  material, raw private mirror endpoints, raw tokens, raw certificate
  material, raw advisory payloads, raw rotation policy bodies, raw
  transparency-log payloads, and raw bundle bytes from every export
  envelope.

## Case list

The five required acceptance cases:

- `approved_mirror_same_artifact_release_payload.yaml` —
  Aureline desktop stable 2.3.0 release payload retrieved through
  the Acme Corp enterprise mirror with origin signature reused;
  customer-mirror identity declared and pinned; integrity outcome
  `integrity_ok_same_artifact`.
- `approved_mirror_repackaged_blocked_extension_package.yaml` —
  community pack mirror has repackaged the partner_lint_pack 2.1.0
  extension under its own signing key; integrity outcome
  `blocked_repackaged_identity` with the
  `mirror_resigned_repackaged_blocked` resign kind.
- `stale_revocation_metadata_propagation.yaml` —
  revocation against legacy_lint_pack 1.0.4 propagated onto the
  customer offline bundle on 2026-04-18 and now past the configured
  10-day grace window; propagation status
  `propagated_to_offline_bundle` with `propagation_age_class =
  stale_past_grace`; reconcile actions name the
  `refresh_via_approved_mirror` and `re_verify_against_current_trust_root`
  actions.
- `offline_bundle_with_verification_packet.yaml` —
  Acme Corp sovereign install snapshot bundling seven artifact
  families (release payload, update metadata, policy bundle, docs
  pack, extension packs, model pack, debug-symbol sidecars) plus
  three emergency-metadata coverage rows (advisory, revocation,
  trust-root rotation) imported via manual file import; packet
  disclosure summary `mixed_local_and_imported` with refresh path
  `manual_file_import_refresh`.
- `customer_mirror_identity_mismatch_release_payload.yaml` —
  Aureline desktop stable 2.3.0 release payload retrieved through a
  mirror operating under the wrong customer-mirror identity claim;
  artifact bytes match the origin but customer-mirror identity is
  `mismatched_against_pin`; integrity outcome
  `blocked_customer_mirror_identity_mismatch`.

Every case cites companion artifact-family map entries, channel rows,
exact-build identities, release-evidence packets, About cards,
verification rows, mirror-promotion rows, manual-import receipts,
trust-root rotation state records, mirror-continuity records,
support bundles, and (where applicable) offline-continuity cards by
stable ref so release, install, update center, About, support, and
mirror / offline surfaces can pivot in O(1) from one mirror-integrity
packet, propagation record, or offline-verification packet to the
canonical proof chain.
