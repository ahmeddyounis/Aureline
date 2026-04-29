# Import-review, handoff-artifact inspection, and post-entry handoff-card fixtures

This corpus is the worked-example projection of
[`/docs/ux/import_handoff_review_contract.md`](../../../docs/ux/import_handoff_review_contract.md)
and the boundary schemas:

- [`/schemas/ux/import_review.schema.json`](../../../schemas/ux/import_review.schema.json)
  — discriminates between `import_review_record` and
  `handoff_artifact_inspection_record` via the top-level `oneOf`
  on `record_kind`.
- [`/schemas/ux/post_entry_handoff_card.schema.json`](../../../schemas/ux/post_entry_handoff_card.schema.json)
  — single-record boundary schema for
  `post_entry_handoff_card_record`.

Each fixture is one record. `import_review_record` and
`handoff_artifact_inspection_record` fixtures validate against
`/schemas/ux/import_review.schema.json`; the
`post_entry_handoff_card_record` fixtures validate against
`/schemas/ux/post_entry_handoff_card.schema.json`.

## Pairings

| Import-review fixture | Paired handoff-artifact inspection fixture | Paired post-entry handoff-card fixture |
|---|---|---|
| `import_review_portable_state_package_inspect_only.yaml` | `handoff_artifact_inspection_portable_state_inspect.yaml` | (re-uses `post_entry_handoff_card_user_paused_for_later.yaml` semantics) |
| `import_review_portable_state_package_extract_then_review.yaml` | `handoff_artifact_inspection_portable_state_inspect.yaml` (semantically identical inspection profile) | `post_entry_handoff_card_user_paused_for_later.yaml` |
| `import_review_handoff_packet_compare_before_restore.yaml` | `handoff_artifact_inspection_handoff_packet.yaml` | `post_entry_handoff_card_import_failed_pending_recovery.yaml` |
| `import_review_support_bundle_replay_redacted.yaml` | `handoff_artifact_inspection_support_bundle.yaml` | (re-uses paused-for-later semantics; replay is inspection-only) |
| `import_review_competitor_config_partial_mapping.yaml` | `handoff_artifact_inspection_competitor_config.yaml` | `post_entry_handoff_card_import_blocked_by_unsupported_items.yaml` |
| `import_review_archive_bundle_unscoped_inspection.yaml` | `handoff_artifact_inspection_archive_bundle.yaml` | (re-uses paused-for-later semantics; archive is inspection-only) |

## Coverage matrix

Disclosure axes exercised: artifact classes
`portable_state_package`, `handoff_packet`, `support_bundle_replay`,
`competitor_config_root`, `archive_bundle_unscoped`; schema-version
classes `schema_version_match`, `schema_version_compatible_minor`,
`schema_version_unknown`; producer-continuity classes
`same_producer_compatible_build`, `producer_explicitly_third_party`,
`producer_unknown`; inspect-or-write classes
`inspect_only_no_write`, `write_to_labelled_inspection_staging`,
`write_to_labelled_extraction_staging`; extraction-target classes
`ephemeral_inspection_only`, `labelled_inspection_staging`,
`labelled_extraction_staging`; lossy-mapping classes
`no_lossy_mapping`, `lossy_with_review`,
`schema_migrated_compatible`, `competitor_mapping_partial`,
`manual_review_required`; cleanup-posture classes
`cleanup_on_cancel`, `retain_for_review`,
`retain_until_durable_promotion`, `rollback_checkpoint_retained`;
machine-local-exclusion classes `secret_material_excluded`,
`live_handle_excluded`, `machine_unique_handle_excluded`,
`credential_store_only_excluded`, `absolute_path_excluded`,
`display_affinity_excluded`, `no_exclusions`.

Handoff-inspection classes exercised:
`portable_state_package_inspection`,
`issue_or_support_packet_inspection`,
`archive_bundle_inspection`, `bundle_like_handoff_inspection`,
`competitor_config_inspection`. The remaining class
(`template_or_prebuild_inspection`) is reserved for the
template / prebuild start-from-snapshot fixtures.

Provenance-label classes exercised: `first_party_signed`,
`competitor_origin_declared`, `support_bundle_redacted`,
`anonymous_origin`. The remaining classes
(`first_party_unsigned`, `third_party_signed`,
`third_party_unsigned`, `support_bundle_unredacted`,
`provenance_missing`) are reserved for downstream surfaces.

Redaction-posture classes exercised:
`default_redaction_applied`, `support_redaction_applied`,
`competitor_redaction_applied`, `redaction_unavailable`. The
remaining classes (`redaction_skipped_by_user`,
`redaction_skipped_by_policy`, `redaction_partial`) are reserved
for downstream surfaces.

Handoff-card status classes exercised: `user_paused_for_later`,
`import_failed_pending_recovery`,
`import_blocked_by_unsupported_items`. The remaining classes
(`review_pending`, `compare_pending`, `import_admitted_partial`,
`import_admitted_full`, `import_rolled_back`,
`import_blocked_by_policy`, `import_blocked_by_authority`,
`import_discarded_no_change`) are reserved for downstream
surfaces.

Safest-next-action classes exercised: `set_up_later`,
`roll_back_import`, `review_unsupported_items`. The remaining
classes (`review_migration_report`, `compare_before_restore`,
`review_trust_and_open`, `promote_to_durable_destination`,
`keep_imported_state`, `open_minimal`, `inspect_only`,
`return_to_start_center`, `request_admin_help`,
`discard_inspection`, `no_action_required`) are reserved for
downstream surfaces.

## Pre-import / post-entry invariants

Every `import_review_record` fixture asserts:

- `trust_unchanged_until_admit = true`.
- `no_durable_write_before_review = true`.
- `no_state_rehydration_before_review = true`.
- `no_portability_claim_before_review = true`.
- When `inspect_or_write_class` is one of the labelled-staging
  variants, `non_durable_label_visible = true` and
  `durable_promotion_requires_review = true`.
- When `cleanup_posture_class = rollback_checkpoint_retained`,
  `rollback_checkpoint_ref` is set.

Every `handoff_artifact_inspection_record` fixture asserts:

- The inspection class binds to the artifact class per §4.2.
- When `handoff_inspection_class =
  issue_or_support_packet_inspection`, `support_redaction_policy_ref`
  is set and the redaction posture is one of the support-bundle
  classes.
- When `handoff_inspection_class = competitor_config_inspection`,
  `competitor_mapping_ref` is set.
- `raw_body_inspectable` and `redacted_summary_available` agree
  with the parent import-review's `raw_actions_offered[]`.

Every `post_entry_handoff_card_record` fixture asserts:

- `trust_unchanged_until_admit = true`.
- `no_durable_write_at_handoff = true`.
- `no_state_rehydration_at_handoff = true`.
- `no_portability_claim_at_handoff = true`.
- `later_support_or_export_retrieval_supported = true`.
- `preserved_provenance_disclosure.exact_artifact_identity_preserved
  = true`.
- The preserved provenance and exclusion lists agree with the
  upstream inspection record and import review.

A fixture that emits any of these as the opposite boolean is
non-conforming and the surface MUST not commit.

## Adding a fixture

1. Pick the smallest scenario that exercises a single new value
   (a new artifact class, schema-version class, lossy-mapping
   class, machine-local exclusion, handoff-inspection class,
   provenance class, redaction posture, handoff-card status,
   or safest-next-action).
2. Write the fixture as a YAML document with the `__fixture__`
   prelude naming the scenario and the contract sections
   asserted.
3. Add the fixture to the pairing table and the coverage matrix
   above.
4. Validate the fixture against
   `/schemas/ux/import_review.schema.json` or
   `/schemas/ux/post_entry_handoff_card.schema.json`, depending
   on the record kind.
