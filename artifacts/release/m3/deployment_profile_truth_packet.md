# M3 deployment profile truth packet

This packet is the release-evidence excerpt for the M3 beta-exit
deployment-truth lane. It binds the profile summary card, the
control-plane / data-plane status strip, residual-dependency rows,
mirror/offline artifact rows, mode-change and disconnect-review sheets,
and the local-core continuity packet to one inspectable record family so
the marketed local-only, managed, self-hosted, sovereign, mirrored, and
air-gapped beta rows can be reviewed without forcing reviewers to splice
generic "service degraded" prose with separate help content.

Reviewer-facing entrypoints:

- Contract: `docs/ops/m3/deployment_profile_and_continuity_beta.md`
- Profile summary schema: `schemas/deployment/profile_summary.schema.json`
- Plane-status strip schema: `schemas/deployment/plane_status.schema.json`
- Residual-dependency row schema:
  `schemas/deployment/residual_dependency.schema.json`
- Upstream record family schema (vocabulary source of truth):
  `schemas/deployment/deployment_summary_card.schema.json`
- Local-core continuity packet schema (vocabulary source of truth):
  `schemas/deployment/local_core_continuity_packet.schema.json`
- Mode-change and disconnect-review schema (vocabulary source of truth):
  `schemas/deployment/mode_change_review.schema.json`
- Governance ledgers:
  `artifacts/governance/deployment_profiles.yaml` and
  `artifacts/governance/residual_dependencies.yaml`
- Shell projection and audit: `crates/aureline-shell/src/deployment_profile/`
- Companion UX contracts: `docs/ux/deployment_summary_contract.md`,
  `docs/ux/deployment_transition_contract.md`,
  `docs/ux/control_data_plane_status_contract.md`

## What this packet asserts

The packet asserts that, for the M3 beta-exit:

- Local-only, managed-cloud, self-hosted, sovereign, mirrored, and
  air-gapped beta rows can be rendered through one inspectable
  `profile_summary_record` plus one `plane_status_strip_record` plus
  typed `residual_dependency_row_record` and
  `mirror_offline_artifact_row_record` rows.
- The five frozen `deployment_profile` values
  (`individual_local`, `self_hosted`, `enterprise_online`,
  `air_gapped`, `managed_cloud`) and the product-facing label vocabulary
  re-export from the governance ledger byte-for-byte; this lane mints no
  new profile.
- The control-plane and data-plane status strip distinguishes identity /
  policy / catalog / relay impairment from workspace / runtime / attach /
  stream impairment with one closed `safest_next_action_class`
  vocabulary; `Service degraded` is no longer a generic catch-all where
  local editing remains safe.
- Self-hosted, sovereign, mirrored, and air-gapped claims cannot
  overstate isolation: required vendor dependencies are explicit in
  product, docs, and support-safe exports, and the
  prohibited-implied-claim guardrails are enforced by the shell's audit
  pass.
- Local-only, mirrored, and air-gapped rows preserve the local-core
  baseline (`local_editing`, `local_save`, `local_search`, `local_git`,
  `local_tasks`, `local_docs_inspect`, `local_export`,
  `local_diagnostics`) and never imply a hidden public-cloud
  prerequisite.

## What this packet does not assert

- Final user-facing copy or microcopy. The contract pins the closed
  vocabulary; the design-system style guide and shell-interaction-safety
  contract own the strings.
- Managed-service backend, signed-policy distribution pipeline, mirror
  server, offline-bundle build pipeline, model registry, extension
  marketplace, docs-pack publisher, or update-channel implementation.
- Telemetry wire format, opaque-ref minting, or diagnostics-bundle
  envelope.
- Post-stable commercial control-plane breadth, marketplace commerce,
  or generalised SaaS billing UX. Continuity truth is in scope; revenue
  surfaces are not.

## Record families

| Family                              | Schema                                                                | Source of truth                                                              |
| ----------------------------------- | --------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `deployment_profile_summary_record` | `schemas/deployment/profile_summary.schema.json`                      | Shell `deployment_profile::ProfileSummary`                                   |
| `plane_status_strip_record`         | `schemas/deployment/plane_status.schema.json`                         | Shell `deployment_profile::PlaneStatusStrip`                                 |
| `residual_dependency_row_record`    | `schemas/deployment/residual_dependency.schema.json`                  | Shell `deployment_profile::ResidualDependencyRow`                            |
| `mirror_offline_artifact_row_record`| `schemas/deployment/deployment_summary_card.schema.json` (`oneOf`)    | Shell `deployment_profile::MirrorOfflineArtifactRow`                         |
| `mode_change_review_record`         | `schemas/deployment/mode_change_review.schema.json` (`oneOf`)         | UX contract `docs/ux/deployment_transition_contract.md`                      |
| `disconnect_review_record`          | `schemas/deployment/mode_change_review.schema.json` (`oneOf`)         | UX contract `docs/ux/deployment_transition_contract.md`                      |
| `local_core_continuity_packet_record` | `schemas/deployment/local_core_continuity_packet.schema.json`      | Deployment governance `artifacts/deployment/locality_matrix.yaml`            |

## Worked fixture cases bound to this packet

| Profile             | Fixture set                                                                                            |
| ------------------- | ------------------------------------------------------------------------------------------------------ |
| `individual_local`  | `fixtures/deployment/deployment_summary_cases/individual_local_baseline_card.yaml`                     |
| `managed_cloud`     | `fixtures/deployment/deployment_summary_cases/managed_cloud_baseline_card.yaml` + dep rows + relay-disconnect |
| `self_hosted`       | `fixtures/deployment/deployment_summary_cases/self_hosted_sovereign_baseline_card.yaml` + dep rows + policy-bundle artifact |
| `enterprise_online` | `fixtures/deployment/deployment_summary_cases/enterprise_online_hybrid_baseline_card.yaml` + dep rows |
| `air_gapped`        | `fixtures/deployment/deployment_summary_cases/air_gapped_mirror_only_card.yaml` + artifact rows + dep rows |

Mode-change and disconnect-review reuse:
`fixtures/deployment/mode_change_cases/` (sign-in, sign-out, org-switch,
mirror-fallback, offline-transition, service-degradation,
reconnect-required, profile-narrow).

Local-core continuity reuse: `fixtures/deployment/continuity_cases/`
(individual local baseline, managed-cloud relay disconnect,
self-hosted stale policy session, enterprise failover boundary recheck,
enterprise policy denies external export, air-gapped mirror-only docs).

## Acceptance evidence

The release is acceptable as a controlled beta-exit dry run while:

- Every claimed deployment profile resolves to one
  `profile_summary_record` whose `audit()` returns an empty defect set
  for the corresponding fixture; the shell's
  `DeploymentProfilePage::audit()` is the authoritative reference and is
  exercised by `cargo test -p aureline-shell --lib deployment_profile`.
- `Service degraded` is not rendered as a generic catch-all where local
  editing remains safe; the `GenericServiceDegradedWhereLocalSafeRemains`
  defect is held live by
  `relay_outage_keeps_local_safe_next_action` and
  `generic_service_degraded_action_is_rejected_when_local_remains_safe`
  in the test module.
- Self-hosted and sovereign claims do not silently carry vendor-managed
  keys: the `self_hosted_cannot_silently_carry_vendor_managed_keys` test
  proves the audit fires.
- Air-gapped claims declare `offline_air_gapped` mirror state, emit at
  least one mirror/offline artifact row, and never route through a
  `companion_surface` consumer: see
  `air_gapped_must_declare_offline_state_and_artifact_row_and_no_companion`.
- Managed-cloud claims carry both the
  `implied_self_hosted_when_managed_cloud` and
  `implied_managed_independence_when_local_dependent` guardrails: see
  `managed_cloud_page_requires_guardrails_and_vendor_dependencies`.
- Mirror-only state lists the
  `implied_offline_parity_when_mirror_only` guardrail and at least one
  mirror/offline artifact row: see
  `mirror_only_must_emit_artifact_rows_and_offline_parity_guardrail`.
- Mirror/offline artifact rows whose digest state is `digest_mismatch`
  block the verify action until fresh: see
  `digest_mismatch_must_block_verify_until_fresh`.
- Support-bundle exports drop any non-export-safe profile summary,
  plane-status strip, residual-dependency row, or mirror/offline
  artifact row: see `support_export_drops_non_export_safe_rows`.

## Refresh trigger

Refresh the packet when any of these change:

- `schemas/deployment/profile_summary.schema.json`,
  `schemas/deployment/plane_status.schema.json`,
  `schemas/deployment/residual_dependency.schema.json`,
  `schemas/deployment/deployment_summary_card.schema.json`,
  `schemas/deployment/local_core_continuity_packet.schema.json`,
  `schemas/deployment/mode_change_review.schema.json`
- `artifacts/governance/deployment_profiles.yaml`,
  `artifacts/governance/residual_dependencies.yaml`,
  `artifacts/deployment/locality_matrix.yaml`
- `crates/aureline-shell/src/deployment_profile/mod.rs` or its tests
- Any worked fixture under
  `fixtures/deployment/deployment_summary_cases/`,
  `fixtures/deployment/continuity_cases/`, or
  `fixtures/deployment/mode_change_cases/`
- Any UX contract under `docs/ux/deployment_*` or
  `docs/ux/control_data_plane_status_contract.md`

## Known limits and exclusions

- This packet pins the **record family** consumed by About, diagnostics,
  support packets, admin-audit exports, release-evidence excerpts,
  status-bar deployment cells, companion surfaces, and CLI text
  formatters. It does not pin per-surface visual layout, iconography,
  or animation.
- The packet does not run a managed control plane, fetch policy bundles,
  switch mirrors, or drive auth flows. It projects existing posture
  facts the shell already has into the named record family.
- The packet does not duplicate the schema validation performed by the
  upstream deployment-summary-card schema's `oneOf` branches; the
  beta-exit schemas re-export the same closed vocabulary and the
  upstream schema remains authoritative for value sets.

## Failure drill

To confirm the guardrail is live:

1. In a scratch test, narrow a managed-cloud profile by removing
   `implied_managed_independence_when_local_dependent` from
   `prohibited_implied_claim_classes`.
2. Call `DeploymentProfilePage::audit()`; it MUST return
   `ManagedCloudMissingGuardrails`.
3. Restore the guardrail and re-run; the audit MUST return an empty
   defect set.

The shell module's test suite encodes this drill plus eleven others.
