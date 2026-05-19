# Deployment-profile claim qualification

This doc names what every marketed beta deployment row MUST produce
before it can ship. It binds the deployment-profile beta contract
(`docs/ops/m3/deployment_profile_and_continuity_beta.md`) to the
checked-in continuity corpus
(`fixtures/deployment/m3/profile_truth/` and
`fixtures/deployment/m3/control_plane_vs_data_plane/`), the
release-evidence excerpt
(`artifacts/release/m3/deployment_profile_conformance_report.md`), and
the residual-dependency matrix
(`artifacts/release/m3/residual_dependency_matrix.json`).

The rules below are normative for the M3 beta-exit deployment-truth
lane.

## Required evidence per marketed row

Every claimed beta deployment row (`individual_local`, `self_hosted`,
`enterprise_online` in either online-live or mirror-only mode,
`air_gapped`, `managed_cloud`) MUST publish all of the following before
it appears in About, the deployment summary card, support exports, or
the release-evidence excerpt:

1. **A profile-truth case in the corpus.** One
   [`DeploymentProfilePage`](../../../crates/aureline-shell/src/deployment_profile/mod.rs)
   record per surface lens (desktop, CLI/headless, companion handoff
   where the profile permits it, support export) checked in under
   `fixtures/deployment/m3/profile_truth/cases/`. The page MUST pass
   `DeploymentProfilePage::audit()` with an empty defect set.
2. **A residual-dependency matrix row.** Every hosted or public
   dependency claimed by the row MUST resolve against a
   `residual_dependency_row_record` and a row in
   [`artifacts/release/m3/residual_dependency_matrix.json`](../../../artifacts/release/m3/residual_dependency_matrix.json),
   which mirrors the governance ledger at
   [`artifacts/governance/residual_dependencies.yaml`](../../../artifacts/governance/residual_dependencies.yaml).
   `required` rows for `ai_provider`, `browser_handoff`,
   `companion_notification_channel`, and
   `hosted_control_plane_reachability` MUST set
   `vendor_or_public_dependence = true`.
3. **Outage-separation proof.** The row MUST be exercised by every
   applicable drill in
   `fixtures/deployment/m3/control_plane_vs_data_plane/drills/`. At
   minimum:
   - `self_hosted`, `enterprise_online`, and `managed_cloud` rows MUST
     pass `control_plane_unavailable`, `stale_policy_cache`, and (when
     remote attach applies) `data_plane_blocked_pending_reconnect`.
   - `air_gapped` rows MUST pass `mirror_only_fallback` and assert
     `offline_air_gapped` mirror state with at least one
     mirror/offline artifact row.
   - Hybrid managed rows MUST pass `org_switch_boundary_recheck`,
     `region_mismatch_boundary_recheck`, and (when seat-bound)
     `seat_loss_continue_local`.
   - Mirror-only routing MUST list the
     `implied_offline_parity_when_mirror_only` guardrail.
4. **Continuity assertion.** The row's outage drills MUST prove that
   local edit, save, search, Git, tasks, docs inspection, export, and
   diagnostics remain `available_local_safe` distinct from any managed
   or control-plane loss; the page's
   [`SafestNextAction`](../../../crates/aureline-shell/src/deployment_profile/mod.rs)
   MUST stay inside the closed `safest_next_action_class` vocabulary
   and MUST be `continue_local` (or `await_resolution`) whenever the
   data plane remains local-safe and the control plane is healthy or
   not-applicable.
5. **Restart, reconnect, and export survival.** The page MUST carry an
   inspect-only `open_details_action` so About, diagnostics, support
   exports, admin audits, release-evidence excerpts, status-bar cells,
   companion surfaces, and CLI text formatters all reach the same
   posture without minting parallel paths. Residual-dependency rows,
   mirror freshness, and the continuity fallback class MUST survive
   restart, reconnect, and support-bundle export.

## Cut-vs-downgrade rules

If a row cannot produce the evidence above, do **not** paper it over
with "best effort" or "service degraded" wording. Either:

- **Cut the claim.** Remove the row from the marketed list until the
  evidence packet exists. Update About, the deployment summary card,
  support exports, the help pack, and the release-evidence excerpt in
  the same change.
- **Downgrade the claim.** Move the row to a narrower profile that the
  evidence supports (for example, narrow a claimed `self_hosted` to
  `enterprise_online` when the customer cannot operate the entire
  control plane, or narrow a claimed `air_gapped` to `online_mirror_only`
  when egress lanes remain reachable). Surface the narrowing on the
  profile-summary card via an explicit
  `prohibited_implied_claim_class` entry.

A claim that cannot be cut or honestly narrowed is a release blocker.

## Prohibited implied claims

The following implied claims are non-conforming and the shell's
`DeploymentProfilePage::audit()` MUST hold them out:

- `implied_air_gapped_when_egress_allowed` — when a row claims
  air-gapped behavior while an egress lane is reachable.
- `implied_sovereign_when_vendor_managed` — when a row claims
  sovereignty while vendor-managed keys, vendor-managed control plane,
  or vendor retention defaults remain in scope.
- `implied_self_hosted_when_managed_cloud` — when a managed-cloud row
  reads as self-hosted.
- `implied_no_residual_dependency_when_required_present` — when a row
  asserts independence while any required residual dependency is
  present.
- `implied_offline_parity_when_mirror_only` — when a mirror-only row
  reads as if it has offline parity with the live control plane.
- `implied_managed_independence_when_local_dependent` — when a
  managed-cloud row reads as if it operates independently of the
  customer's local install.
- `implied_always_fresh_when_bounded_or_unbounded_stale` — when a row
  reads as if its caches and mirrors are always fresh.

Every managed-cloud row MUST list
`implied_self_hosted_when_managed_cloud` and
`implied_managed_independence_when_local_dependent`; every air-gapped
row MUST list `implied_air_gapped_when_egress_allowed`; every
mirror-only row MUST list `implied_offline_parity_when_mirror_only`.
Self-hosted rows MUST NOT silently carry `vendor_managed` keys.

## Vocabulary reuse

Product, docs, support, and the release-evidence excerpt MUST quote the
same closed vocabulary for the tested scenarios:

- `deployment_profile_class`, `product_facing_label_class`,
  `tenant_org_scope_class`, `region_scope_class`, `retention_class`,
  `key_mode_class` — re-exported from
  [`artifacts/governance/deployment_profiles.yaml`](../../../artifacts/governance/deployment_profiles.yaml).
- `dependency_class`, `posture_class`, `absence_impact_class`,
  `continuity_fallback_class` — re-exported from
  [`artifacts/governance/residual_dependencies.yaml`](../../../artifacts/governance/residual_dependencies.yaml).
- `control_plane_service_state_class`, `data_plane_capability_state_class`,
  `mirror_offline_state_class`, `safest_next_action_class`,
  `prohibited_implied_claim_class` — re-exported from
  [`schemas/deployment/deployment_summary_card.schema.json`](../../../schemas/deployment/deployment_summary_card.schema.json).

Surfaces that mint parallel wording are non-conforming and the corpus
replay test (see Verification below) will fail when the vocabulary
diverges.

## Verification

Run the corpus replay test:

```bash
cargo test -p aureline-shell --test deployment_profile_corpus_fixtures
```

The test loads every fixture under `fixtures/deployment/m3/profile_truth/`
and `fixtures/deployment/m3/control_plane_vs_data_plane/`, asserts
every page passes `audit()`, and asserts the rendered conformance
report and residual-dependency matrix match the seeded packet
byte-for-byte.

## Refresh trigger

Refresh this doc, the contract, the corpus, the matrix, and the
conformance report in the same change when any of these change:

- The shell module at `crates/aureline-shell/src/deployment_profile/`.
- The governance ledger at
  `artifacts/governance/residual_dependencies.yaml` or
  `artifacts/governance/deployment_profiles.yaml`.
- The schemas under `schemas/deployment/`.
- Any UX contract under `docs/ux/deployment_*` or
  `docs/ux/control_data_plane_status_contract.md`.
- The marketed deployment-row list (a row is added, narrowed, downgraded,
  or cut).
