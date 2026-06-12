# M5 install, configuration, sync, and auth-recovery governance report — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/install/m5/m5-install-and-portability-governance.json`. The full contract and gate
semantics live in `docs/install/m5/m5-install-and-portability-governance.md`; the typed model
lives in the `aureline-install` crate (`m5_install_and_portability_governance`).

This artifact freezes the canonical M5 install/config/auth matrix by aggregating the stable-line
install, settings, sync, and identity lanes and publishing, for each lane, **only the assurance
label its evidence actually backs**. Unverified, unsupported, stale, sync-blocked, or
recovery-limited lanes are automatically narrowed to a bounded or retest-pending label, or
refused, before publication.

## Governance roll-up (as of 2026-06-11)

| Lane | Mode | Declared | Published label | Admission | Recovery |
| --- | --- | --- | --- | --- | --- |
| `desktop_stable` | system | verified | **verified** | admit_full | none |
| `desktop_preview` | system | verified | **bounded** | admit_bounded | switch_supported_topology |
| `portable_install` | portable | verified | **retest_pending** | admit_retest | refresh_portable_state |
| `managed_fleet` | managed | verified | **withheld** | refuse | request_recovery_policy |
| `marketplace_companion` | marketplace | verified | **retest_pending** | admit_retest | enroll_passkey |
| `cli_headless` | user | verified | **verified** | admit_full | none |
| `sync_device` | system | verified | **withheld** | refuse | restore_sync_apply |

Two lanes admit at full trust (`desktop_stable`, `cli_headless`), proving the gate is not a
blanket downgrade; one narrows to bounded, two to retest-pending, and two are refused. The
published label of every lane equals the gate's recomputed ceiling and never widens install or
auth language past the weakest observed state.

## What the gate proves

- **Install modes stay distinct.** System, user, portable, managed, and marketplace installs are
  pinned to their lanes and never blur together.
- **Install and auth language never silently widens.** The side-by-side `desktop_preview` install
  is bounded to its slice; the stale `portable_install` is held at retest-pending; the
  self-signed, offline, passkey-less `marketplace_companion` is held at retest-pending with three
  downgrade reasons; the policy-blocked `managed_fleet` recovery and the sync-blocked
  `sync_device` lane are refused.
- **Local-first continuity stays explicit.** `managed_fleet` is `policy_restricted`, while
  `marketplace_companion` and `sync_device` fall back to `local_only_fallback` — local durable
  state stays authoritative and local-only work continues even when sync or recovery is degraded.
- **Every required surface ingests one packet.** Release center, Help/About, support export,
  diagnostics, CLI, and admin docs each bind to this packet and narrow with it.

## Consumer surfaces

Release center, Help/About, support export, diagnostics, CLI, and admin docs each bind to this
one packet, ingest it, preserve its labels and recovery paths, and narrow with it, so a row
narrowed here cannot stay stable downstream. The export projection and support-export wrapper
carry typed states and opaque refs only — no credential bodies, raw provider payloads, or
workspace contents.
