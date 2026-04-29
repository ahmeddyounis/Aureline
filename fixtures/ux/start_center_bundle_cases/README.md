# Start Center bundle card and bundle detail page fixtures

Seed corpus for the contract frozen in
[`/docs/ux/start_center_bundle_surfaces.md`](../../../docs/ux/start_center_bundle_surfaces.md)
and the schemas at
[`/schemas/ux/bundle_card.schema.json`](../../../schemas/ux/bundle_card.schema.json)
and
[`/schemas/ux/bundle_detail_page.schema.json`](../../../schemas/ux/bundle_detail_page.schema.json).

Each file is a single JSON document validating against either
`start_center_bundle_card_record` or
`start_center_bundle_detail_page_record`.

Every fixture:

- Re-exports the underlying workflow-bundle manifest fields
  verbatim (per the contract's "no new vocabulary downstream"
  invariant). The manifests these fixtures shadow live under
  [`/fixtures/workflow/bundles/`](../../workflow/bundles/).
- Pins the badge set, freshness overlay, packaging chip, and
  card / detail-page action set this contract owns.
- Carries no raw absolute paths, raw URLs, raw credential
  material, or raw secrets. Every id is an opaque ref; every
  timestamp is a monotonic placeholder.
- Names the contract sections it exercises under
  `__fixture__.contract_sections`.

## Cases

| Fixture | Record kind | Scenario axis | Contract anchor |
| --- | --- | --- | --- |
| [`bundle_card_certified_launch_typescript.json`](./bundle_card_certified_launch_typescript.json) | `start_center_bundle_card_record` | Certified launch bundle on Start Center secondary entry; `badge.certified` + `badge.live_or_mirror`; `freshness.fresh_current`; no successor. | §4, §4.3 rule 1, §10.1 |
| [`bundle_card_org_approved_mirror_only.json`](./bundle_card_org_approved_mirror_only.json) | `start_center_bundle_card_record` | Org-approved bundle on the gallery; `badge.managed_approved` + `badge.mirror_only`; `local_offline_availability_class = available_via_mirror`. | §4.2 rule 3, §4.3 rule 2, §10.2 |
| [`bundle_card_community_unreviewed.json`](./bundle_card_community_unreviewed.json) | `start_center_bundle_card_record` | Community bundle never silently certified; `badge.community` + `unreviewed` chip; install gated by `signature_review_required`. | §4.3 rule 3, §3.3 rule 2, §10.3 |
| [`bundle_card_imported_pending_review.json`](./bundle_card_imported_pending_review.json) | `start_center_bundle_card_record` | Imported-user bundle pending review; `badge.imported` + `badge.retest_pending`; `freshness.age_unknown`. | §4.3 rule 4, §4.2 rule 7, §10.4 |
| [`bundle_card_local_draft_workspace_switcher.json`](./bundle_card_local_draft_workspace_switcher.json) | `start_center_bundle_card_record` | Local-draft bundle on the workspace-switcher; `badge.local_draft`; cannot promote without explicit publish flow. | §4.3 rule 5, §10.5 |
| [`bundle_card_retest_pending_certified.json`](./bundle_card_retest_pending_certified.json) | `start_center_bundle_card_record` | Certified bundle whose evidence is aging; `badge.retest_pending` + `badge.signed_offline_bundle`; `freshness.aging_within_window`. | §4.3 rule 1, §9 rules 4 and 5, §10.6 |
| [`bundle_card_deprecated_with_successor.json`](./bundle_card_deprecated_with_successor.json) | `start_center_bundle_card_record` | Deprecated launch bundle on update / what's-changed surface with first-party successor; `badge.deprecated`. | §9 rules 3 and 6, §10.7 |
| [`bundle_card_status_unknown_offline.json`](./bundle_card_status_unknown_offline.json) | `start_center_bundle_card_record` | Bundle whose status could not be determined; `badge.status_unknown`; typed `disabled_reason_code = network_unreachable` on export-facing summary. | §4.3 rule 7, §6 rule 3, §10.8 |
| [`bundle_detail_page_certified_launch_typescript.json`](./bundle_detail_page_certified_launch_typescript.json) | `start_center_bundle_detail_page_record` | Detail page rendering all nineteen sections; typed `Review bundle` / `View certification sheet` / `Open without this bundle` / `Remove` actions. | §5, §5.3, §7.2, §8, §10.9 |

## Schema references

- Bundle card schema:
  [`/schemas/ux/bundle_card.schema.json`](../../../schemas/ux/bundle_card.schema.json).
- Bundle detail-page schema:
  [`/schemas/ux/bundle_detail_page.schema.json`](../../../schemas/ux/bundle_detail_page.schema.json).
- Workflow-bundle manifest (source of truth for every axis
  these fixtures re-export):
  [`/schemas/workflow/bundle_manifest.schema.json`](../../../schemas/workflow/bundle_manifest.schema.json).
- Start Center surface schema (composing zones, account
  posture, freshness / absence vocabulary):
  [`/schemas/ux/start_center_surface.schema.json`](../../../schemas/ux/start_center_surface.schema.json).

## Companion corpora

- Underlying workflow-bundle manifest fixtures these cards
  shadow:
  [`/fixtures/workflow/bundles/`](../../workflow/bundles/).
- Start Center surface fixtures whose secondary-entry zone
  these bundle rows render in:
  [`/fixtures/ux/start_center_rows/`](../start_center_rows/).
