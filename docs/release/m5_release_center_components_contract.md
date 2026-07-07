# M5 Release-Center Component Matrix Contract

Status: Frozen (M05-860, batch B101).

This contract freezes the reusable **release-center and publication components** so
candidate, target, provenance, and promotion-history surfaces stop drifting on
blocker, evidence, auth-source, and rollback language. It is the shared component
layer on top of the already-claimed artifact graph, promotion pipeline, and mirror
transport — it does **not** re-architect any of them.

- Canonical Rust module:
  `crates/aureline-release/src/freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix/`
- Boundary schema: `schemas/ui/m5-release-center-components.schema.json`
- Support export (single source of truth, `include_str!`-embedded):
  `artifacts/release/m5-release-center-component-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-release-center-component-proof/matrix.csv`
- Markdown report: `artifacts/components/m5-release-center-components.md`
- Design matrix: `artifacts/design/m5-release-center-component-matrix.md`
- Narrowed fixtures: `fixtures/ui/m5-release-center-components/`
- Headless emitter (only mint-from-truth path):
  `cargo run -p aureline-release --bin aureline_release_freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix -- <support-export|report|csv|fixture-*|validate>`

## Component families (6)

| Family | Truth it must project |
| --- | --- |
| `release_candidate_card` | Candidate scope + blocker state with freshness. |
| `version_bump_row` | Proposed bump class + compatibility impact. |
| `publish_target_row` | Target visibility + mutability + auth source + dry-run availability. |
| `artifact_provenance_bundle_card` | Signature + attestation + SBOM status over an immutable digest lineage. |
| `promotion_timeline_step` | Rollout ring + promotion stage state. |
| `rollback_revocation_row` | Rollback blast radius + revocation scope. |

Each family declares the **full** set of its family-specific vocabulary and no other
family's vocabulary — the `family_specific_vocabularies_are_declared_only_where_applicable`
and `every_vocabulary_token_is_declared_by_some_component` tests enforce this, so a
later implementation row cannot silently invent a parallel naming system.

## Locked controlled vocabularies

- **Candidate scope** (6): single_family, multi_family, full_train, hotfix, backport_line, preview_channel.
- **Blocker state** (6): no_blockers, soft_blockers_only, hard_blocker_open, blocker_waived, blocker_resolved_pending_reverify, blocker_state_unknown.
- **Version bump class** (6): major, minor, patch, prerelease, build_metadata_only, republish_no_version_change.
- **Compatibility impact** (5): backward_compatible, breaking_change, forward_incompatible, runtime_behavior_only, schema_migration_required.
- **Target visibility** (5): public_listed, public_unlisted, private_tenant, internal_only, mirror_replicated.
- **Target mutability** (5): immutable_once_published, mutable_tag_repointable, overwrite_allowed, retraction_allowed, append_only.
- **Target auth source** (6): ci_federated_identity, maintainer_key, org_managed_identity, hardware_token_signer, delegated_bot_identity, unauthenticated_mirror.
- **Dry-run availability** (4): dry_run_supported, dry_run_partial, dry_run_unavailable, dry_run_required_before_publish.
- **Signature status** (5): signed_verified, signed_unverified_key, unsigned, signature_broken, signature_pending.
- **Attestation status** (5): attested_verified, attested_unverified, no_attestation, attestation_expired, attestation_pending.
- **SBOM status** (5): sbom_complete, sbom_partial, sbom_missing, sbom_stale, sbom_generating.
- **Digest lineage state** (5): immutable_digest_pinned, digest_lineage_continuous, digest_lineage_broken, digest_unverified, rebuild_digest_matched.
- **Rollout ring** (6): canary_ring, pilot_ring, early_access_ring, broad_ring, general_availability, held_not_promoted.
- **Promotion stage state** (5): stage_pending, stage_in_progress, stage_promoted, stage_blocked, stage_rolled_back.
- **Rollback blast radius** (5): single_artifact, family_scoped, train_scoped, cross_train_scoped, fleet_wide.
- **Revocation scope** (5): no_revocation, tag_repoint_only, artifact_revoked, signing_key_revoked, trust_root_rotated.

## Shared vocabulary (every component)

- **Publication surface families** (8): release_center, update_center, registry_publication, mirror_publication, enterprise_evaluation, support_desk, docs_help, admin_review.
- **Deployment lines** (5): local_oss, self_hosted, managed, air_gapped, mirror_offline — every component keeps the same truth across each line.
- **Consumer surfaces** (10): release_center_ui, help_about, service_health, docs_portal, admin_console, evaluation_pack, mirror_console, support_export, cli_inspect, product_ui.
- **Accessibility routes** (6): keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable — every component declares these so no release truth is hover-only, pointer-only, or visually encoded alone.
- **Required labels** (6): identity, state, keyboard_route (**mandatory on every component**), plus evidence_freshness, auth_source, rollback_vocabulary — the three optional labels close the acceptance-criteria ambiguity about evidence freshness, target auth source, and rollback vocabulary.
- **Downgrade triggers** (12): candidate_scope_unstated, blocker_freshness_hidden, version_bump_impact_unstated, target_auth_source_masked, target_mutability_hidden, dry_run_availability_unstated, signature_or_attestation_overclaimed, sbom_completeness_overstated, digest_lineage_broken_hidden, rollout_ring_unstated, rollback_blast_radius_understated, proof_stale.

## Hard invariants (must all be `false` per row)

- `masks_target_auth_source_or_mutability`
- `conflates_signed_and_unsigned_provenance`
- `invents_private_release_status_grammar`
- `overstates_rollback_reversibility_or_drops_evidence_freshness`

Any `true` raises `component_invariant_violated` and blocks publication.

## Qualification and auto-narrowing

Each row carries a qualification class (`stable`/`beta`/`preview`/`experimental`/
`unavailable`/`held`). A `stable` row must cite at least one proof packet ref; a stale
proof packet or any of the 12 downgrade triggers narrows the component below its claim
while every component family stays visible in the matrix. The two checked narrowed
fixtures demonstrate this: `promotion_timeline_step` held at Beta and
`rollback_revocation_row` narrowed to Preview, both still present and validating.

## Export safety

The packet is metadata-only (`redaction_class_token = metadata_only_export_safe`).
The validator rejects any export containing `api_key`, `password`, `secret`,
`bearer `, or `://`, so raw URLs, signing keys, tokens, and credentials never cross
the boundary.

## Consumers

Every claimed M5 release/publication surface points at this one contract instead of
rewording release truth locally: release-center, update-center, registry, mirror,
enterprise-evaluation, support, docs, and admin surfaces all read the same candidate,
bump, target, provenance, promotion, and rollback vocabulary. Source contracts this
matrix binds against: `docs/release/release_center_object_model_contract.md`,
`docs/release/update_and_rollback_contract.md`, and
`docs/release/artifact_verification_contract.md`.
