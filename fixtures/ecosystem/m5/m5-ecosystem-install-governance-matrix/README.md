# Fixtures: M5 ecosystem install-governance matrix

This directory contains fixture metadata for the
`m5_ecosystem_install_governance_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json`

## Coverage

- `first_party_framework_pack`, `docs_pack`, `local_model_pack`,
  `signed_recipe_pack`, `template_artifact`, `bridge_backed_package`,
  `side_loaded_package`, and `mirrored_registry_variant` are the only claimed
  artifact families, and each carries exactly one row — no family inherits trust
  from an adjacent one.
- Each family carries its own provenance, permission-manifest, activation-budget,
  compatibility/downgrade, rollback, and support-export ref.
- Published support class covers `fully_supported`, `best_effort_supported`,
  `community_supported`, and `unsupported`, and the promotion decision covers
  `promote`, `narrow_to_best_effort`, `narrow_to_community`, and `fail_promotion`.
- Runtime origin covers `first_party_signed`, `partner_signed`, `community_signed`,
  `bridge_runtime`, `local_model_runtime`, and `unsigned_side_loaded`; compatibility
  covers `compatible`, `degraded_compatible`, `compatibility_bridge_required`, and
  `unsupported_on_target`; permission state covers `unchanged`, `reduced`,
  `additive_disclosed`, `expanded_unreviewed`, and `not_applicable`; activation band
  covers `healthy_under_budget`, `approaching_ceiling`, `over_budget`,
  `budget_unknown`, and `not_applicable`; lifecycle covers `available`, `installed`,
  `update_available`, `disabled`, `quarantined`, `rolled_back`, and `retired`;
  evidence freshness covers `current`, `stale`, `expired`, and `unknown`; and
  rollback posture covers `reversible_verified`, `reversible_unverified`,
  `compensating_only`, `irreversible`, and `not_applicable`.
- The seven canonical narrowing reasons — `provenance_unverified`, `evidence_stale`,
  `permission_expansion_unreviewed`, `activation_budget_exceeded`,
  `compatibility_unsupported`, `rollback_incomplete`, and `quarantined` — are each
  exercised by at least one family.
- The promotion gate is exercised in both directions: the clean
  `first_party_framework_pack` promotes to full support, while stale, over-budget,
  permission-expanded, rollback-incomplete, unsupported, and quarantined families
  narrow automatically, and the unsigned `side_loaded_package` and the mirrored
  `mirrored_registry_variant` fail promotion. Each row's `published_support_class`,
  `promotion_decision`, and `narrowing_reasons` equal the recomputed gate decision,
  so release and registry tooling can prove underqualified families narrow before
  publication.
