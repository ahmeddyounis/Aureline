# Template gallery, prebuild / warm-start, resume-live, and open-without-starter disclosure fixtures

Seed corpus for the contract frozen in
[`/docs/ux/template_and_prebuild_contract.md`](../../../docs/ux/template_and_prebuild_contract.md)
and its machine-readable matrix at
[`/artifacts/ux/template_source_class_matrix.yaml`](../../../artifacts/ux/template_source_class_matrix.yaml).

Each file is a single JSON document exercising one of the seven
record kinds the contract freezes
(`template_and_prebuild_gallery_record`,
`template_card_record`,
`prebuild_card_record`,
`environment_starter_summary_record`,
`resume_live_workspace_card_record`,
`post_create_handoff_summary_record`,
`template_health_row_record`,
`starter_policy_notice_record`).

Every fixture:

- Resolves every axis to vocabulary re-exported from the
  entry-restore object model §1–§4, the source-acquisition seed
  §1–§3, the start-center contract §3, and the entry-restore
  truth audit §6 (`startup_state` tokens).
- Pins the `template_and_prebuild_surface_family`,
  `template_source_class`, `support_class`,
  `runtime_and_toolchain_scope`, `template_lifecycle_class`,
  `declared_freshness_class`, `signer_continuity_class`,
  `starter_setup_cost_class`, `availability_narrowing_class`,
  `bypass_path_id`, health signal / check, preflight axes,
  handoff axes, and policy-notice values this contract owns.
- Carries no raw absolute paths, raw URLs, raw credential
  material, or raw secrets. Every id is an opaque ref; every
  timestamp is a monotonic placeholder.
- Names the contract sections it exercises under
  `__fixture__.contract_sections` and the matrix rows it
  exercises under `__fixture__.matrix_rows`.

## Cases

| Fixture | Record kind | Scenario axis | Contract anchor |
| --- | --- | --- | --- |
| [`template_gallery_first_party_local_only.json`](./template_gallery_first_party_local_only.json) | `template_and_prebuild_gallery_record` wrapping one `template_card_record` | First-party template, `runtime_and_toolchain_scope = local_only`, `starter_setup_cost_class = light_local_setup`, equal-weight bypass path exposed. | §3.2 (first_party), §3.4 (local_only), §3.7 (light_local_setup), §4, §11.1 / §11.5, §13.1 |
| [`environment_starter_summary_devcontainer.json`](./environment_starter_summary_devcontainer.json) | `environment_starter_summary_record` | First-party devcontainer starter; all six §6.9 preflight axes plus `extension_install_preview`, `port_binding_preview`, `trust_grant_preview`; itemised `setup_actions_list`; bypass paths at equal weight. | §3.12, §6, §11.2 / §11.3 / §11.4, §13.2 |
| [`template_card_community_signature_review.json`](./template_card_community_signature_review.json) | `template_card_record` + companion `starter_policy_notice_record` | Community source whose signer changed; `signer_continuity_class = signer_changed_review_required`, `disabled_reason_code = signature_review_required`; bypass path `bypass.clone_repository_without_starter` still at equal weight. | §3.2 (community), §3.14 (community_source_notice), §5.1 rule 3, §13.3 |
| [`prebuild_picker_stale_warmstart.json`](./prebuild_picker_stale_warmstart.json) | `template_and_prebuild_gallery_record` (family `prebuild_picker`) wrapping `prebuild_card_record` + `template_health_row_record`s | Stale warm-start prebuild: `declared_freshness_class = mirror_stale`, health row `template_freshness → warn` with `template_health_signal_class = stale_or_invalid`; bypass path preserved; `bypass_path_still_available = true` on every health row. | §3.6, §3.10, §5.2, §9, §13.4 |
| [`template_gallery_fleet_policy_narrowed.json`](./template_gallery_fleet_policy_narrowed.json) | `template_and_prebuild_gallery_record` + `starter_policy_notice_record` | Admin-fleet machine where fleet policy excludes community and signature-review-required rows; in-place `fleet_policy_notice` names `availability_narrowing_class = policy_narrowed_fleet` with resolution hook `continue_in_restricted_mode`; surviving rows render at equal weight. | §3.8, §3.14, §10, §13.5 |
| [`template_gallery_mirror_only_offline.json`](./template_gallery_mirror_only_offline.json) | `template_and_prebuild_gallery_record` + `starter_policy_notice_record` | Offline startup; only the mirror-cached subset of the canonical gallery renders; `mirror_or_airgap_policy_notice` names `availability_narrowing_class = mirror_only_cached_subset`; all four matrix-required bypass paths render. | §3.6, §3.8 (mirror_only_cached_subset), §4, §13.6 |
| [`resume_live_managed_cloud_card.json`](./resume_live_managed_cloud_card.json) | `resume_live_workspace_card_record` | Managed-cloud workspace in state `ready` with `attach_authority_class = authority_live`; the four `alternative_lanes` — `resume_live_workspace`, `start_from_snapshot`, `clone_fresh_repository`, `open_without_starter` — render distinctly; the resume lane enqueues only `remote_attach_handshake`. | §3.5 rules 1 & 2, §7, §11.10, §13.7 |
| [`post_create_handoff_partial_setup_failure.json`](./post_create_handoff_partial_setup_failure.json) | `post_create_handoff_summary_record` | Starter with five bootstrap items: two succeed, one skipped (offline bundle), one partially applied, one failed; itemised `succeeded_actions` / `skipped_actions` / `partially_applied_actions` / `failed_actions`; `cleanup_performed`; `local_only_continuation_path_ref`. | §3.13, §8, §11.9, §13.8 |

## Schema references

- Template and prebuild disclosure contract:
  [`/docs/ux/template_and_prebuild_contract.md`](../../../docs/ux/template_and_prebuild_contract.md).
- Template-source-class matrix:
  [`/artifacts/ux/template_source_class_matrix.yaml`](../../../artifacts/ux/template_source_class_matrix.yaml).
- Entry / restore object model (source of truth for entry
  verbs, target kinds, resulting modes, restore levels,
  recovery classes, next-step decision hooks re-exported
  here):
  [`/docs/workspace/entry_restore_object_model.md`](../../../docs/workspace/entry_restore_object_model.md).
- Source-locator, checkout-plan, trust-stage, and bootstrap-
  queue seed (source of truth for locator / transport /
  freshness / signer-continuity / bootstrap-item / absence /
  repair-hook vocabularies re-exported here):
  [`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../../../docs/workspace/source_acquisition_and_bootstrap_seed.md).
- Start Center, workspace-switcher, open-flow, restore-card,
  and recent-work disclosure contract (source of truth for
  zone ordering, primary-action ids, privacy-reduction mode,
  and disclosure classes this contract composes into):
  [`/docs/ux/start_center_contract.md`](../../../docs/ux/start_center_contract.md).

## Companion corpora

- Start Center, workspace-switcher, and open-flow disclosure
  fixtures that the template gallery composes into:
  [`/fixtures/ux/start_center_rows/`](../start_center_rows/).
- Source-acquisition bootstrap-case fixtures that the setup-
  action lists resolve into:
  [`/fixtures/workspace/bootstrap_cases/`](../../workspace/bootstrap_cases/).
- Entry / restore record shapes the gallery / picker / card
  rows wrap:
  [`/fixtures/workspace/entry_restore_examples/`](../../workspace/entry_restore_examples/).
