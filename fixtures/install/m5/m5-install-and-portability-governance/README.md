# Fixtures: M5 install, configuration, sync, and auth-recovery governance matrix

This directory contains fixture metadata for the `m5_install_and_portability_governance_matrix`
packet.

The canonical full corpus is checked in at:

`artifacts/install/m5/m5-install-and-portability-governance.json`

## Coverage

- `desktop_stable`, `desktop_preview`, `portable_install`, `managed_fleet`,
  `marketplace_companion`, `cli_headless`, and `sync_device` are the only claimed lanes, and each
  carries exactly one row — no lane inherits a verified label from an adjacent one.
- Each lane is pinned to one distinct install mode (`system`, `user`, `portable`, `managed`,
  `marketplace`), validated against `InstallConfigLane::install_mode`, so system, user, portable,
  managed, and marketplace installs never blur together; every mode is exercised.
- Each row binds to the canonical install-truth packet it governs via `packet_ref` (validated
  against `InstallConfigLane::source_packet`), so the governance matrix aggregates the landed
  stable-line install, settings, sync, and identity packets rather than a parallel spreadsheet,
  and each row carries its own conformance, evidence, governance-receipt, release-evidence,
  Help/About, support-export, and diagnostics refs.
- Install verification covers `signed_verified`, `platform_trusted` (in the closed vocabulary),
  `self_signed`, and `unverified`; install topology covers `supported`, `side_by_side_bounded`,
  `experimental`, and `unsupported`; portable-state freshness covers `current`, `aging`, `stale`,
  and `missing`; sync-device state covers `active`, `degraded`, `offline`, and `blocked`; and
  auth-recovery posture covers `passkey_verified`, `system_browser_fallback`,
  `local_only_continuity`, and `recovery_blocked`. The channel/ring, state-root, portable/export,
  and effective-setting vocabularies are likewise closed and pinned.
- Local continuity covers `authoritative`, `local_only_fallback`, and `policy_restricted`, so a
  lane whose local durable state stays authoritative is kept distinct from one that degraded to
  local-only because sync or auth degraded, and from one whose local-safe work a policy restricts.
  The `managed_fleet` lane is `policy_restricted`; `marketplace_companion` and `sync_device` are
  `local_only_fallback`; the rest are `authoritative`.
- Published label covers `verified`, `bounded`, `retest_pending`, and `withheld`, and the
  admission outcome covers `admit_full`, `admit_bounded`, `admit_retest`, and `refuse`.
- The six downgrade reasons — `unverified_install`, `unsupported_install_topology`,
  `stale_portable_state`, `blocked_sync_apply`, `missing_passkey`, and `policy_limited_recovery` —
  are each exercised by at least one lane.
- The governance gate is exercised in every direction: the clean `desktop_stable` and
  `cli_headless` installs admit at full trust (verified); the side-by-side `desktop_preview`
  install narrows to a bounded label; the stale `portable_install` and the self-signed, offline,
  passkey-less `marketplace_companion` narrow to retest-pending; and the policy-blocked
  `managed_fleet` recovery and the sync-blocked `sync_device` lane are refused entirely.
  `desktop_preview`, `portable_install`, and `marketplace_companion` are the automatic-downgrade
  cases — declared verified, dropped below it rather than left widening install or auth language —
  while `desktop_stable` and `cli_headless` prove the gate is not a blanket downgrade. The
  trust-sensitive `desktop_preview`, `portable_install`, `managed_fleet`,
  `marketplace_companion`, and `sync_device` lanes narrow safely instead of inheriting a broader
  stable claim. The `sync_device` lane proves the local-first invariant: the sync-apply claim is
  withheld, but local durable state stays authoritative so local-only work continues. Each lane's
  `published_assurance`, `admission_outcome`, and `downgrade_reasons` equal the recomputed gate
  decision, and every required consumer surface (release center, Help/About, support export,
  diagnostics, CLI, admin docs) binds to the packet and narrows with it, so a narrowed lane
  cannot stay stable by inertia.
