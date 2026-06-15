# Commercial-boundary cards — human-readable rendering

Human-readable rendering of the canonical commercial-boundary-card set. This row
is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at
`artifacts/service/m5-commercial-boundary-cards.json`.

## Per-card boundary

| Card | Boundary class | Service family | Declared claim | Effective claim | Residual deps |
| --- | --- | --- | --- | --- | --- |
| commercial_boundary.local_open_core | local_open_source | (none) | local_safe_only | local_safe_only | 0 |
| commercial_boundary.ai_gateway | managed_paid_optional | ai_gateway_family | managed_full | managed_full | 2 |
| commercial_boundary.settings_sync | managed_paid_optional | sync_family | managed_full | managed_full | 2 |
| commercial_boundary.companion_relay | managed_paid_optional | collaboration_relay_family | managed_full | managed_full | 2 |
| commercial_boundary.registry_mirror | managed_paid_optional | registry_or_mirror_metadata_family | managed_full | managed_full | 2 |
| commercial_boundary.support_ingest | managed_paid_optional | telemetry_or_support_ingest_family | managed_full | managed_full | 2 |
| commercial_boundary.managed_workspace | managed_paid_optional | remote_workspace_control_plane_family | managed_full | managed_full | 2 |

The effective claim is the declared claim capped by the card's evidence status.
With current evidence no card is narrowed; a stale status narrows every managed
card to `managed_narrowed`, and a missing or downgraded status drops them to
`local_safe_only`. The local open core never narrows. Managed cards are
cross-checked against the control-plane lane for their service family.

## Open-versus-paid statement

| Card | Open-versus-paid |
| --- | --- |
| local_open_core | Open and local: the editor core, search, Git, and local automation are open-source and run with no managed dependency and no payment. |
| ai_gateway | Paid and optional: managed-broker inference is metered; direct, BYOK, and local AI routes are the open alternative. |
| settings_sync | Paid and optional: managed sync replicates settings across devices; local settings and files are authoritative either way. |
| companion_relay | Paid and optional: the managed relay carries live sessions; local notes, patches, and offline packets are the open alternative. |
| registry_mirror | Paid and optional: the managed registry/mirror serve discovery metadata; a customer mirror or offline bundle is the open alternative. |
| support_ingest | Paid and optional: managed ingest uploads bundles; local support-bundle export is the open alternative and always available. |
| managed_workspace | Paid and optional: the remote workspace runs on the managed control plane; local checkout, tasks, and Git are the open alternative. |

## Residual dependency disclosure — no boundary overstated

| Card | Residual dependency classes | Remains vendor-hosted | Eliminated under self-host |
| --- | --- | --- | --- |
| ai_gateway | ai_provider, hosted_control_plane_reachability | yes | yes (BYOK / local) |
| settings_sync | hosted_control_plane_reachability, sign_in | yes | reachability yes; sign-in no |
| companion_relay | hosted_control_plane_reachability, companion_notification_channel | yes | reachability yes; channel no |
| registry_mirror | package_registry, remote_mirror | yes | yes (signed mirror / offline bundle) |
| support_ingest | hosted_control_plane_reachability, policy_bundle | yes | reachability no; policy bundle yes |
| managed_workspace | remote_agent, hosted_control_plane_reachability | yes | yes (self-hosted control plane) |

The local open core declares **no** residual vendor dependency.

## Deployment-profile qualifiers

| Card | Holds in | Not offered in |
| --- | --- | --- |
| local_open_core | individual_local, self_hosted, enterprise_online, air_gapped, managed_cloud | (none) |
| ai_gateway | enterprise_online, managed_cloud | air_gapped |
| settings_sync | self_hosted, enterprise_online, managed_cloud | individual_local, air_gapped |
| companion_relay | self_hosted, enterprise_online, managed_cloud | individual_local, air_gapped |
| registry_mirror | self_hosted, enterprise_online, air_gapped, managed_cloud | (none) |
| support_ingest | self_hosted, enterprise_online, managed_cloud | air_gapped |
| managed_workspace | self_hosted, enterprise_online, managed_cloud | individual_local, air_gapped |

## Procurement/support evidence — one object model

| Card | Packet kinds | Export guarantee |
| --- | --- | --- |
| local_open_core | open_source_license_manifest, deployment_profile_truth_packet | parity_with_csv_and_json |
| ai_gateway | usage_and_forecast_export, chargeback_export, residual_dependency_disclosure, entitlement_summary | parity_with_csv_and_json |
| settings_sync | usage_and_forecast_export, chargeback_export, residual_dependency_disclosure | parity_with_json_only |
| companion_relay | usage_and_forecast_export, chargeback_export, residual_dependency_disclosure, support_bundle | parity_with_csv_and_json |
| registry_mirror | usage_and_forecast_export, chargeback_export, residual_dependency_disclosure | parity_with_csv_and_json |
| support_ingest | usage_and_forecast_export, residual_dependency_disclosure, support_bundle | parity_with_csv_and_json |
| managed_workspace | usage_and_forecast_export, chargeback_export, residual_dependency_disclosure | parity_with_json_only |

The procurement packet and the support/admin packet bind the same evidence object
for the same cards, so a buyer and a support engineer read one vocabulary.

## Action priority — commercial prompts never outrank truth

| Action | Rank | Present on |
| --- | --- | --- |
| export_evidence | 1 | all seven cards |
| continue_local | 2 | all seven cards |
| view_procurement_packet | 3 | all seven cards |
| view_residual_dependencies | 4 | all seven cards |
| view_deployment_profile_truth | 5 | all seven cards |
| learn_about_paid | 6 | the six managed cards only |

No `learn_about_paid` action ranks above `export_evidence`,
`view_procurement_packet`, or `continue_local`. The local open core carries no
upsell.

## Surface bindings

| Surface | Binds cards |
| --- | --- |
| help_about | all seven cards |
| release_center | all seven cards |
| diagnostics | all seven cards |
| procurement_packet | all seven cards |
| support_admin_packet | all seven cards |
| claim_public_truth_automation | all seven cards |

## Summary

- 7 commercial-boundary cards: 1 local-open-core card + 6 managed-lane cards.
- Every card keeps a non-empty local-safe baseline; the local core is never
  blocked when a managed lane's evidence is stale or missing.
- The open core makes only the local-safe claim with no residual dependency; every
  managed card declares the full managed claim and discloses its residual
  vendor-hosted dependencies.
- No open boundary is overstated: residual dependencies and per-profile
  qualifiers are explicit, and lanes unavailable air-gapped or individual-local
  say so.
- Procurement and support reuse one evidence object at the same export guarantee.
- Export, procurement, and local continuation always outrank any learn-about-paid
  prompt.
- Boundary cards show no bare numbers; figures live on the metering surfaces.
- 6 surfaces, each projecting the effective claim, never a stronger one.
