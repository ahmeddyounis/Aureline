# Workflow-bundle manifest fixtures

Seed corpus for the contract frozen in
[`/docs/workflow/workflow_bundle_object_model.md`](../../../docs/workflow/workflow_bundle_object_model.md)
and the schema at
[`/schemas/workflow/bundle_manifest.schema.json`](../../../schemas/workflow/bundle_manifest.schema.json).

Each file is a single YAML `workflow_bundle_manifest_record`. The
fixtures exercise the closed `bundle_class`, `bundle_source_class`,
`bundle_status_class`, signer-source, packaging-posture, channel-
relation, dependency-marker, component-inventory, and lifecycle
vocabularies plus the §7 class-linkage table.

Every fixture:

- names a stable `bundle_id` plus integer `bundle_revision`;
- declares one each of `bundle_class`, `bundle_source_class`,
  `bundle_status_class`, `bundle_signer_source_class`,
  `mirror_or_offline_packaging_posture`,
  `bundle_channel_relation_class`, `signer_continuity_class`,
  and `signature_class`;
- emits the full `component_inventory` with every slot present,
  including empty arrays where the bundle declares no components
  of that kind, so reviewers see the absence;
- carries the `lifecycle` block (state class, evidence age,
  retest-needed posture, successor recommendation,
  certification-sheet refs, removal surface kind, rollback
  surface kind, freshness window class) so bundle truth survives
  install;
- carries no raw signing keys, raw certificate material, raw
  repository URLs, raw absolute paths, raw secrets, or raw
  user-authored content.

## Cases

| Fixture | `bundle_class` | `bundle_source_class` | `bundle_status_class` | Notable invariants exercised |
| --- | --- | --- | --- | --- |
| [`launch_bundle_typescript_web_app.yaml`](./launch_bundle_typescript_web_app.yaml) | `launch_bundle` | `certified` | `certified_current` | All seven scoreboard families covered; `core_signing_root`; full inventory; `live_or_mirror`. |
| [`org_approved_bundle_internal_frontend_baseline.yaml`](./org_approved_bundle_internal_frontend_baseline.yaml) | `org_approved_bundle` | `managed_approved` | `managed_approved_current` | Org signer; `mirror_only`; org-policy removal / rollback surfaces; migration mapping. |
| [`imported_user_bundle_pending_review.yaml`](./imported_user_bundle_pending_review.yaml) | `imported_user_bundle` | `imported` | `imported_pending_review` | `unsigned_user_review_required`; ingress review pending; empty `certification_targets` slot present, not absent. |
| [`design_partner_bundle_field_pilot.yaml`](./design_partner_bundle_field_pilot.yaml) | `design_partner_bundle` | `managed_approved` | `community_reviewed` | Pilot block with `quarterly` review; tour and glossary packs; `branch_or_fork_revision`. |
| [`local_draft_bundle_user_authored.yaml`](./local_draft_bundle_user_authored.yaml) | `local_draft_bundle` | `local_draft` | `local_draft` | `local_user_trust_only`; `not_applicable_local_draft` channel relation; `not_applicable_local_draft` removal surface. |
| [`launch_bundle_signed_offline_air_gap.yaml`](./launch_bundle_signed_offline_air_gap.yaml) | `launch_bundle` | `certified` | `certified_retest_pending` | `signed_offline_bundle` posture; `signed_offline_bundle_root` signer; `aging_within_window`; `retest_recommended`. |
| [`launch_bundle_deprecated_with_successor.yaml`](./launch_bundle_deprecated_with_successor.yaml) | `launch_bundle` | `certified` | `deprecated_or_archived` | `cross_channel_demotion`; `soft_deprecated_with_successor`; `successor_recommended_first_party_native` with decision row; rollback surface live. |
| [`launch_bundle_status_unknown_offline.yaml`](./launch_bundle_status_unknown_offline.yaml) | `launch_bundle` | `certified` | `status_unknown` | `packaging_posture_unknown`; `age_unknown`; `retest_blocked` with `network_unreachable`; demonstrates that absent status renders verbatim. |

The case-index manifest at
[`./manifest.yaml`](./manifest.yaml) maps each file to the
contract sections it exercises.
