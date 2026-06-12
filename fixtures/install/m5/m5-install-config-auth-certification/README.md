# Fixtures: M5 install/config/auth certification

This directory contains fixture metadata and troubleshooting-drill fixtures for
the `m5_install_config_auth_certification` packet.

The canonical full corpus is checked in at:

`artifacts/install/m5/m5-install-config-auth-certification.json`

## Coverage

- `desktop_stable`, `desktop_preview`, `portable`, `managed_fleet`, and
  `mirror_offline` are the only claimed profiles, and each carries exactly one
  certification row. No profile inherits a greener label from a nearby row.
- Every row carries exactly four domain qualifications:
  `install_topology`, `config_portability`, `sync_device`, and
  `auth_recovery`.
- The packet aggregates all seven named source packets, so install governance,
  coexistence, diagnostics, settings parity, portable restore, sync/device, and
  auth/recovery truth stay tied together in one certification gate.
- The gate is exercised in every direction: one row publishes **verified**, two
  rows publish **bounded**, one publishes **retest_pending**, and one is
  **withheld**. Four rows are automatic downgrades below what they declared.
- Evidence freshness exercises `current`, `aging`, `stale`, and `missing`.
  Narrow reasons exercise `stale_evidence`, `missing_evidence`, and
  `source_unqualified`. The `incomplete_domain_coverage` downgrade path is
  covered by validation tests in the crate.
- All required consumer surfaces bind to the packet and narrow with it:
  `release_center`, `about`, `docs_help`, `admin_docs`, `support_export`,
  `diagnostics`, and `cli`.

## Troubleshooting drills

Each checked-in drill fixture replays one certification scenario and must match
the embedded packet exactly:

- `drill-install-topology.json`
- `drill-side-by-side.json`
- `drill-portable.json`
- `drill-mirror-offline.json`
- `drill-settings-portability.json`
- `drill-sync-device.json`
- `drill-passkey-recovery.json`
- `drill-accessibility.json`
- `drill-downgrade.json`
