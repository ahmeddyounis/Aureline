# Registry, mirror, channel-promotion, offline-bundle, and local-archive example fixtures

These fixtures anchor the vocabulary reserved in
[`/docs/extensions/registry_and_offline_bundle_seed.md`](../../../docs/extensions/registry_and_offline_bundle_seed.md)
and validated by the seed schema at
[`/schemas/extensions/registry_manifest.schema.json`](../../../schemas/extensions/registry_manifest.schema.json)
and the channel-promotion / trust-inheritance / anti-abuse register at
[`/artifacts/extensions/channel_promotion_rows.yaml`](../../../artifacts/extensions/channel_promotion_rows.yaml).

The registry-manifest seed is at `Status: Proposed`. These fixtures
exercise the reserved field sets, enumerated vocabularies, and
schema `allOf` gates so the later registry-mirror, install-review,
permission-inspector, offline-bundle, and support-export lanes can
be built against one contract rather than invent registry-shaped
fields ad hoc.

**Scope rules**

- Each fixture validates against
  `schemas/extensions/registry_manifest.schema.json` as one of
  `registry_manifest_row`,
  `mirror_continuity_row`,
  `channel_promotion_row`,
  `offline_bundle_manifest_row`, or
  `local_archive_import_row`.
- A fixture MUST exercise at least one frozen
  `registry_source_class`, `channel_class`, `approval_state_class`,
  `signature_class`, `trust_claim_source`,
  `trust_badge_inheritance_rule`, `compatibility_claim_class`,
  `revocation_snapshot_age_class`, `lockfile_parity_state`,
  `mirror_continuity_state_class`, `offline_bundle_restore_step_class`,
  `anti_abuse_signal_class`, `registry_audit_event_id`, or
  `registry_denial_reason` and MUST name the seed section that
  motivates it.
- Raw artifact bytes, raw signing-key material, raw attestation-
  bundle bytes, raw mirror-cache bodies, raw offline-bundle payload
  bytes, and raw local-archive bytes MUST NOT appear; refs stand in
  for every field that would otherwise carry raw material.
- Ids, refs, aliases, and monotonic timestamps are opaque; they
  are chosen to read well rather than to reflect any real
  deployment.

**Index**

| Fixture                                                                              | Record kind                    | Key classes exercised                                                                                                                                                       | Seed section                                                  |
|--------------------------------------------------------------------------------------|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------|
| [`public_registry_production_row.json`](./public_registry_production_row.json)       | `registry_manifest_row`        | `public_registry` / `production` / `promoted_to_production` / `publisher_signature` / `origin_public_registry` / `inherits_origin_tier` / `fresh`                           | Registry-source classes; Trust-inheritance matrix             |
| [`approved_mirror_verified_cached.json`](./approved_mirror_verified_cached.json)     | `mirror_continuity_row`        | `approved_mirror` / `verified_cached` / `deny_list_applied` + `publisher_trust_floor_applied` / `warm_cached`                                                                | Mirror-continuity state                                       |
| [`channel_promotion_quarantine_to_approved.json`](./channel_promotion_quarantine_to_approved.json) | `channel_promotion_row` | `quarantine → approved` / `promoted` / `preserves_artifact_identity = true`                                                                                                  | Channel lanes; Channel-promotion rules                        |
| [`offline_bundle_deterministic_restore.json`](./offline_bundle_deterministic_restore.json) | `offline_bundle_manifest_row` | `offline_bundle` / `capped_at_community_on_approved_mirror` / `exact_match` / `warm_cached` / `install_artifact` + `prompt_user_for_trust_decision`                          | Offline-bundle export and import                              |
| [`offline_bundle_blocked_revocation_stale.json`](./offline_bundle_blocked_revocation_stale.json) | `offline_bundle_manifest_row` | `offline_bundle` / `drift_detected_blocking` / `stale` / `block_on_lockfile_drift` + `block_on_revocation_snapshot_stale`                                                    | Offline-bundle export and import; Denial reasons              |
| [`local_archive_unverified_sideload.json`](./local_archive_unverified_sideload.json) | `local_archive_import_row`     | `local_archive` / `capped_at_unverified_on_local_archive` / `unsigned_requires_out_of_band_verification` / `removable_media`                                                 | Local archive and quarantined local copy; Trust-inheritance matrix |
| [`quarantined_local_copy_install_denied.json`](./quarantined_local_copy_install_denied.json) | `local_archive_import_row` | `quarantined_local_copy` / `quarantined_cannot_inherit` / `typosquatting_candidate`                                                                                          | Local archive and quarantined local copy; Denial reasons      |
