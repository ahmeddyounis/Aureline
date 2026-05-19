# Deployment profile and continuity beta contract

This contract makes deployment-profile truth inspectable for users,
administrators, support, and release reviewers. It binds the profile
summary card, residual-dependency rows, control-plane/data-plane status
strip, mirror/offline artifact rows, mode-change and disconnect-review
sheets, and local-core continuity packet to the same checked-in record
family so claimed local-only, managed, self-hosted, sovereign, mirrored,
and air-gapped beta rows can be reviewed end to end without forcing a
reviewer to splice generic "service degraded" copy with separate help
prose.

The contract is normative for the M3 beta deployment-truth lane. Where
it disagrees with the upstream design contracts in
[`docs/ux/deployment_summary_contract.md`](../../ux/deployment_summary_contract.md),
[`docs/ux/deployment_transition_contract.md`](../../ux/deployment_transition_contract.md),
and
[`docs/ux/control_data_plane_status_contract.md`](../../ux/control_data_plane_status_contract.md),
those documents win and this one plus its schemas, fixtures, and shell
module update in the same change.

## Source artifacts

- Profile summary record schema:
  [`schemas/deployment/profile_summary.schema.json`](../../../schemas/deployment/profile_summary.schema.json)
- Plane-status strip record schema:
  [`schemas/deployment/plane_status.schema.json`](../../../schemas/deployment/plane_status.schema.json)
- Residual-dependency row record schema:
  [`schemas/deployment/residual_dependency.schema.json`](../../../schemas/deployment/residual_dependency.schema.json)
- Upstream record family schema (frozen vocabulary source of truth):
  [`schemas/deployment/deployment_summary_card.schema.json`](../../../schemas/deployment/deployment_summary_card.schema.json)
- Local-core continuity packet schema (frozen vocabulary source of truth):
  [`schemas/deployment/local_core_continuity_packet.schema.json`](../../../schemas/deployment/local_core_continuity_packet.schema.json)
- Mode-change and disconnect-review schema (frozen vocabulary source of
  truth):
  [`schemas/deployment/mode_change_review.schema.json`](../../../schemas/deployment/mode_change_review.schema.json)
- Deployment-profile governance ledger:
  [`artifacts/governance/deployment_profiles.yaml`](../../../artifacts/governance/deployment_profiles.yaml)
- Residual-dependency governance ledger:
  [`artifacts/governance/residual_dependencies.yaml`](../../../artifacts/governance/residual_dependencies.yaml)
- Worked summary card cases:
  [`fixtures/deployment/deployment_summary_cases/`](../../../fixtures/deployment/deployment_summary_cases/)
- Worked continuity cases:
  [`fixtures/deployment/continuity_cases/`](../../../fixtures/deployment/continuity_cases/)
- Worked mode-change and disconnect-review cases:
  [`fixtures/deployment/mode_change_cases/`](../../../fixtures/deployment/mode_change_cases/)
- Shell projection and audit module:
  [`crates/aureline-shell/src/deployment_profile/`](../../../crates/aureline-shell/src/deployment_profile/)

## Required user, admin, support, and reviewer truth

Every claimed beta deployment row MUST expose, through the
`profile_summary_record`, `plane_status_strip_record`,
`residual_dependency_row_record`, and `mirror_offline_artifact_row_record`
shapes:

- Deployment profile (`individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, or `managed_cloud`) and the
  product-facing label class.
- Tenant/org scope, region scope, retention class, and key mode. The
  `self_hosted`, `enterprise_online`, and `managed_cloud` profiles MUST
  carry actionable values on all four axes; `not_applicable` on any axis
  is a contract violation.
- Mirror/offline state and a freshness summary (last managed sync,
  cache-age label, or staleness rationale). Mirror-only or air-gapped
  state MUST emit at least one mirror/offline artifact row.
- Control-plane worst surviving state and data-plane worst surviving
  state, **separately**. Generic "service degraded" copy that collapses
  control-plane impairment into workspace failure is a contract
  violation when local-safe data-plane capabilities remain.
- The safest bounded next action, drawn from the closed
  `safest_next_action_class` vocabulary (`continue_local`,
  `retry_policy_sync`, `switch_mirror`, `export_packet`,
  `reconnect_managed_session`, `recheck_boundary`, `await_resolution`,
  `open_outage_notice`). When local-safe data-plane work remains and the
  control plane is healthy or not-applicable, the safest next action MUST
  be `continue_local` (or `await_resolution`).
- Residual hosted or public dependencies: control-plane reachability,
  AI provider, browser handoff, companion notification channel,
  package registry, remote mirror, remote agent, symbolication service,
  policy bundle, docs pack, or sign-in. Each row names the
  `posture_class`, the `unreachable_impact_class`, the
  `continuity_fallback_class`, and a back-pointer into the
  residual-dependency ledger.
- Mirror/offline artifact rows for the five frozen artifact families
  (`updates`, `extensions`, `docs_pack`, `policy_bundle`, `models`) with
  signer state, opaque signer fingerprint, digest state, opaque digest
  ref, mirror freshness class, offline-cache posture, mirror source
  class, and inspect-only `verify_action` and `open_manifest_action`
  entries.
- The prohibited-implied-claim guardrails the surface MUST honour:
  - mirror-only state MUST list
    `implied_offline_parity_when_mirror_only`;
  - managed-cloud profiles MUST list
    `implied_self_hosted_when_managed_cloud` and
    `implied_managed_independence_when_local_dependent`;
  - air-gapped profiles MUST NOT imply `air_gapped` while egress is
    allowed, MUST NOT route through `companion_surface`, and MUST list
    `implied_air_gapped_when_egress_allowed` when an egress lane is
    reachable.
- An inspect-only open-details action so About, diagnostics, support,
  admin-audit, status-bar, and CLI surfaces all reach the same details
  route without minting a parallel UX path.

## Cross-surface invariants the shell module enforces

The shell projection at
[`crates/aureline-shell/src/deployment_profile/mod.rs`](../../../crates/aureline-shell/src/deployment_profile/mod.rs)
mints one
`DeploymentProfilePage` per render. The page composes one
`ProfileSummary`, one `PlaneStatusStrip`, and any number of
`ResidualDependencyRow` and `MirrorOfflineArtifactRow` records. The
page's `audit()` method returns one or more
`DeploymentProfileDefect` values whenever any of these invariants are
violated:

- `NotApplicableTenancyOrRegionOrKeyOnManagedProfile` — a
  `self_hosted` / `enterprise_online` / `managed_cloud` profile left
  any of the tenant / region / key axes at `not_applicable`.
- `SelfHostedClaimedVendorManagedKeys` — a `self_hosted` profile carried
  `vendor_managed` keys.
- `AirGappedMissingOfflineAirGappedState` — an `air_gapped` profile did
  not declare `offline_air_gapped` mirror state.
- `AirGappedRoutedThroughCompanionSurface` — an `air_gapped` profile
  routed through a `companion_surface` consumer.
- `MirrorOrAirGappedMissingArtifactRow` — mirror-only or air-gapped
  state emitted no mirror/offline artifact row.
- `ManagedCloudMissingGuardrails` — a managed-cloud profile did not
  list both
  `implied_self_hosted_when_managed_cloud` and
  `implied_managed_independence_when_local_dependent`.
- `MirrorOnlyMissingOfflineParityGuardrail` — mirror-only state did not
  list `implied_offline_parity_when_mirror_only`.
- `GenericServiceDegradedWhereLocalSafeRemains` — the safest next
  action was not `continue_local` or `await_resolution` while the data
  plane was `available_local_safe` and the control plane was healthy or
  not-applicable.
- `RequiredVendorBoundDependencyMissingVendorDependenceFlag` — a
  `required` residual-dependency row for `ai_provider`,
  `browser_handoff`, `companion_notification_channel`, or
  `hosted_control_plane_reachability` did not set
  `vendor_or_public_dependence = true`.
- `DigestMismatchVerifyActionNotBlocked` — a mirror/offline artifact row
  reported `digest_mismatch` but its `verify_action` did not set
  `revalidation_on_open = blocked_until_fresh`.
- `SignedArtifactMissingSignerFingerprint` — a signed artifact row did
  not carry an opaque signer fingerprint.
- `ExportSafeArtifactRowWidenedRedaction` / `ExportSafeProfileSummaryWidenedRedaction`
  — a record claimed `export_safe = true` but carried
  `internal_support_restricted` or `signing_evidence_only` redaction.
- `PlaneStatusStripRefMismatch`,
  `ResidualDependencyRowProfileSummaryRefMismatch`, and
  `MirrorOfflineArtifactRowProfileSummaryRefMismatch` — the page's
  plane-status strip ref or any child row's `profile_summary_ref` did
  not match the page's `profile_summary.summary_id`.

The same module exposes `DeploymentProfilePage::project_support_export`
which returns a `DeploymentProfileSupportExport` packet that drops any
non-export-safe profile summary, plane-status strip, residual-dependency
row, or mirror/offline artifact row. The support-bundle assembler MUST
use this projection rather than serialising the page wholesale.

## Mode-change and disconnect-review reuse

Browser/companion sign-out, org-switch, seat-loss, mirror fallback,
offline transition, service degradation, reconnect-required, and
deployment-profile-narrow flows reuse the existing
`mode_change_review_record` and `disconnect_review_record` shapes (see
[`schemas/deployment/mode_change_review.schema.json`](../../../schemas/deployment/mode_change_review.schema.json)
and the worked cases under
[`fixtures/deployment/mode_change_cases/`](../../../fixtures/deployment/mode_change_cases/)).
The deployment-profile page links these records through
`linked_outage_notice_refs` and `linked_continuity_packet_ref` so the
same posture object travels through every surface; surfaces MUST NOT
restate the from/to posture in drift-prone prose.

## Acceptance evidence

The release-evidence packet at
[`artifacts/release/m3/deployment_profile_truth_packet.md`](../../../artifacts/release/m3/deployment_profile_truth_packet.md)
binds the schemas, fixtures, and the shell module's `audit()` outcomes to
the M3 claim manifest. Reviewers consume that packet to verify that the
M3 beta-exit deployment-truth claim is supported.

## Verification

Run:

```bash
cargo test -p aureline-shell --lib deployment_profile
```

This exercises the baseline `individual_local` page, the managed-cloud
baseline with required vendor dependencies and guardrails, the
self-hosted-with-vendor-managed-keys defect, the air-gapped
missing-state / missing-artifact-row / companion-surface defects, the
mirror-only missing-guardrail defect, the relay-outage local-safe
continuity case, the digest-mismatch verify-blocked defect, the
support-export redaction filter, and the serde round-trip.
