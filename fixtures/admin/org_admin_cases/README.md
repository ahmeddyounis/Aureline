# Organization administration, seat lifecycle, and fleet governance fixtures

These fixtures anchor the organization-administration contract frozen
in
[`/docs/admin/org_admin_seat_and_fleet_contract.md`](../../../docs/admin/org_admin_seat_and_fleet_contract.md)
and validated by
[`/schemas/admin/seat_lifecycle_row.schema.json`](../../../schemas/admin/seat_lifecycle_row.schema.json)
plus
[`/schemas/admin/fleet_status_row.schema.json`](../../../schemas/admin/fleet_status_row.schema.json).

They show standards-based and file-based administration as peers and
keep local-artifact continuity, deprovisioning-vs-deletion separation,
group-to-policy targeting preview, and stale-or-offline fleet posture
explicit.

**Scope rules**

- Every fixture is one boundary record validated by the cited schema.
- Local edit, save, undo or redo, and user-owned export remain
  explicit on every record.
- Records carry opaque refs and reviewable summaries; raw directory
  payloads, raw bundle bytes, raw signing material, raw user
  identifiers, raw email or display names, raw group display names,
  raw device hostnames, raw IP addresses, and raw mirror hostnames
  never appear.

**Index**

| Fixture | Record kind | What it proves |
|---|---|---|
| [`organization_overview_self_hosted.yaml`](./organization_overview_self_hosted.yaml) | `organization_overview_record` | Self-hosted overview can be assembled without assuming a vendor-only control plane is reachable; standards-based and file-based paths appear as peers. |
| [`file_based_policy_distribution.yaml`](./file_based_policy_distribution.yaml) | `directory_provider_card_record` | A signed file-based bundle path is a first-class administration path with the same freshness, validation, rollback, and audit vocabulary as a standards-based path. |
| [`scim_drift_targeting_preview.yaml`](./scim_drift_targeting_preview.yaml) | `group_policy_targeting_sheet_record` | SCIM drift surfaces in a previewable targeting sheet with commit gates that block until signer continuity is reconciled. |
| [`seat_transfer_preserves_local_artifacts.yaml`](./seat_transfer_preserves_local_artifacts.yaml) | `seat_lifecycle_row_record` | Seat transfer pauses managed capabilities on the source seat and resumes them on the target seat after reauth while local artifacts remain available. |
| [`deprovisioning_preserves_local_artifacts.yaml`](./deprovisioning_preserves_local_artifacts.yaml) | `seat_lifecycle_row_record` | Deprovisioning is visibly distinct from local artifact deletion; offboarding export does not require a live managed seat. |
| [`fleet_ring_with_stale_and_offline_devices.yaml`](./fleet_ring_with_stale_and_offline_devices.yaml) | `fleet_ring_dashboard_record` | Stale within grace, stale past grace, offline last-known-good, and offline unverified are preserved as distinct first-class state. |
