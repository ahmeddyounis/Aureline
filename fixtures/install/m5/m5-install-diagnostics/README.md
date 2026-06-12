# Fixtures: M5 install-and-update diagnostics

This directory contains fixture metadata and troubleshooting-drill fixtures for the
`m5_install_update_diagnostics` packet.

The canonical full corpus is checked in at:

`artifacts/install/m5/m5-install-diagnostics.json`

## Coverage

- `desktop_app`, `companion`, `marketplace_helper`, `local_model_runtime`, and `portable_export`
  are the only claimed artifact families, and each carries exactly one diagnostics row — no
  artifact inherits a verified label from an adjacent one. These are the artifact families M5 adds
  beyond the primary app binary.
- Each row records its install mode, channel-and-ring, updater owner, classified artifact /
  mutable-state / policy roots, last verification state and freshness, and rollback target, so
  install and update topology stays inspectable per artifact rather than reduced to a version
  string.
- Each row is pinned to the canonical governance lane it draws verification truth from
  (`governs_lane`), and `governs_assurance` is validated against
  `artifacts/install/m5/m5-install-and-portability-governance.json`, so an artifact never publishes
  support beyond the lane the governance gate already narrowed.
- Install mode covers `system`, `user`, `portable`, and `marketplace`; verification state covers
  `signed_verified`, `platform_trusted`, and `self_signed`; verification freshness covers `current`
  and `stale`; rollback target covers `available`, `available_bounded`, `expired`, and `missing`;
  root sensitivity exercises `public_path`, `user_scoped`, `machine_protected`, and
  `secret_bearing`. The `unverified` verification state and `never_verified`/`aging` freshness
  states are in the closed vocabulary.
- Secrecy boundaries are exercised: the `companion` session credential store (`secret_bearing`) and
  the `marketplace_helper` policy manifest root (`machine_protected`) are classified and redacted,
  so the packet names where a secret store is without dumping its contents.
- The gate is exercised in every direction: the signed, current `desktop_app` publishes
  **verified** at full trust; the platform-trusted `companion`, the self-signed `marketplace_helper`,
  and the signed-but-stale `local_model_runtime` are narrowed to **retest_pending**; and the
  rollback-less `portable_export` is **withheld**. `companion`, `local_model_runtime`, and
  `portable_export` are the automatic-downgrade cases — declared stronger, dropped below it rather
  than left widening support — while `desktop_app` proves the gate is not a blanket downgrade. Each
  row's `published_support`, `narrow_reasons`, and `recovery_path` equal the recomputed gate
  decision.

## Troubleshooting drills

Each drill fixture replays one support incident and proves the diagnostics object detects it:

- `drill_root_mismatch.json` — an artifact resolved under an unexpected root.
- `drill_stale_verification.json` — an artifact's last verification went stale.
- `drill_missing_rollback_target.json` — an artifact has no valid rollback target.
- `drill_wrong_root_support.json` — support inspected the wrong state root for an artifact.

Every required consumer surface (desktop, CLI, About, support export) binds to the packet and
narrows with it, so a narrowed artifact cannot read as supported by inertia downstream.
