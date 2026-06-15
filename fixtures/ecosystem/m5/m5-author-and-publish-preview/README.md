# Fixtures: M5 author-and-publish-preview matrix

This directory contains fixture metadata for the
`m5_author_and_publish_preview_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`

## Coverage

- `first_party_framework_pack`, `docs_pack`, `local_model_pack`,
  `signed_recipe_pack`, `template_artifact`, `bridge_backed_package`,
  `side_loaded_package`, and `mirrored_registry_variant` are the only claimed
  artifact families, and each carries exactly one author-side row — no family
  inherits an author-lane posture from an adjacent one.
- Each family carries its own local-workspace, sideload-review, sandbox-inspector,
  publish-preview, anti-abuse, conformance, and support-export ref.
- Runtime class covers `passive_package`, `wasm_capability_sandbox`,
  `declarative_host_rendered_view`, `external_host`, `compatibility_bridge`, and
  `remote_side_component`; host/ABI covers `no_code_execution`, `local_machine`,
  `managed_host`, `remote_target`, `external_process`, and `browser_runtime`;
  workspace state covers `source_present_built`, `source_missing`, and
  `build_failed`; signing state covers `signed_verified`, `signed_unverified`,
  `unsigned_local_dev`, `unsigned_sideload`, and `revoked_signature`; published
  trust posture covers `unsigned_local_only`, `registry_bound`,
  `verified_publisher`, and `enterprise_approved`; hot-reload posture covers
  `no_widening`, `relaunch_only`, `runtime_class_widened_pending_review`,
  `permissions_widened_pending_review`, and
  `external_executable_added_pending_review`; publish-review requirement covers
  `full_registry_policy_review`, `standard_review`, `expedited_review`, and
  `not_publishable_from_local`; conformance output covers `conformant`,
  `bridge_conformant`, `partial`, `failed`, `retest_pending`, and `not_run`; and
  anti-abuse transparency covers `disclosed_clean`,
  `publisher_loss_history_disclosed`, `undisclosed`, and `quarantined`.
- The twelve canonical finding codes — `build_failed`, `source_missing`,
  `signature_revoked`, `provenance_unverified`, `hot_reload_runtime_widened`,
  `hot_reload_permissions_widened`, `hot_reload_external_executable_added`,
  `conformance_failed`, `conformance_incomplete`, `anti_abuse_undisclosed`,
  `publisher_loss_history`, and `anti_abuse_quarantined` — are each exercised by at
  least one family, and both severities (`blocker`, `warning`) and all five finding
  domains appear.
- The publish gate is exercised in all four directions: the clean
  `first_party_framework_pack` is ready to publish; the `docs_pack` and
  `local_model_pack` publish with warnings; the `signed_recipe_pack`,
  `template_artifact`, `bridge_backed_package`, and `side_loaded_package` are
  blocked; and the `mirrored_registry_variant` is withheld as quarantined.
- The non-inheritance guardrail is proven directly: the unsigned `local_model_pack`
  declares a verified-publisher badge and the revoked `mirrored_registry_variant`
  declares an enterprise-approved badge, yet both publish as `unsigned_local_only`.
  Hot-reload widening of the runtime class, permissions, or an external executable
  each raises a blocking finding so authority can never widen without a fresh
  review, and each row's `published_trust_posture`, `publish_readiness`, and
  `findings` equal the recomputed gate decision.
