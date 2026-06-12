# M5 install-topology, configuration-portability, sync-device, and auth-recovery governance matrix

This document describes the canonical packet that freezes the **M5 install, configuration,
sync, and identity governance matrix** — the single qualification report that graduates the M5
desktop-stable, desktop-preview, portable-install, managed-fleet, marketplace-companion,
CLI/headless, and sync-device install/config/auth lanes. It aggregates the stable-line install,
settings, sync, and identity packets into one governance gate that automatically narrows or
withholds the published label of any lane whose binary is unverified, whose install topology is
unsupported, whose portable-state package is stale, whose device is blocked from sync, whose
passkey is missing, or whose recovery is policy-limited. It is the user-facing companion to the
governed artifact at `artifacts/install/m5/m5-install-and-portability-governance.json` and the
typed model in the `aureline-install` crate (`m5_install_and_portability_governance`).

This packet answers the install/config/identity-governance question for the M5 expansion as a
whole: **does a desktop, portable, managed, marketplace, companion, CLI, or sync install reach a
supportable, local-first install without silently widening install or auth language over an
unverified, unsupported, stale, sync-blocked, or recovery-limited state — or is it automatically
downgraded to a bounded or retest-pending label, or refused, before publication?**

## What this packet covers

The packet carries one governance row for every claimed M5 install/config/auth lane, each pinned
to one distinct install mode so system, user, portable, managed, and marketplace installs never
blur together, and each pinned to the canonical install-truth packet it draws its evidence from:

1. **`desktop_stable`** (mode `system`) — stable-channel desktop install, the first-party local
   baseline.
2. **`desktop_preview`** (mode `system`) — preview-channel desktop install running side-by-side
   with stable.
3. **`portable_install`** (mode `portable`) — portable install carrying its own durable state
   root.
4. **`managed_fleet`** (mode `managed`) — organization-managed, policy-controlled fleet install.
5. **`marketplace_companion`** (mode `marketplace`) — marketplace or companion surface paired to
   a host install.
6. **`cli_headless`** (mode `user`) — headless command-line install.
7. **`sync_device`** (mode `system`) — sync and device-registry participation for a paired
   install.

The preview, portable, managed, marketplace/companion, and sync lanes are **trust-sensitive**:
they can silently widen install or auth language and must narrow safely rather than inherit a
broader stable claim.

Each row answers, for its lane:

- **Who owns it?** An `owner` accountable for the lane's evidence and conformance.
- **How is it installed?** An `install_mode`, a `channel_ring` (stable, preview, nightly, or
  managed channel and ring), a `state_root_class` (isolated, shared, portable, managed, or
  ephemeral), a `portable_export_class` (native, portable package, export archive, imported, or
  not portable), and an `effective_setting_scope` (default, user, workspace, managed, or synced).
- **Is the binary verified?** An `install_verification` of `signed_verified`, `platform_trusted`,
  `self_signed`, or `unverified`.
- **Is the install topology supported?** An `install_topology_support` of `supported`,
  `side_by_side_bounded`, `experimental`, or `unsupported`.
- **Is the portable/export state fresh?** A `portable_state_freshness` of `current`, `aging`,
  `stale`, or `missing`.
- **Does the device participate in sync?** A `sync_device_state` of `active`, `degraded`,
  `offline`, or `blocked`.
- **What recovery can it reach?** An `auth_recovery_posture` of `passkey_verified`,
  `system_browser_fallback`, `local_only_continuity`, or `recovery_blocked`.
- **Does local durable state stay authoritative?** A `local_continuity` of `authoritative`,
  `local_only_fallback`, or `policy_restricted`, plus `install_root_namespace` and
  `state_root_namespace` so the install and state roots stay inspectable.

## The governance gate

The gate lowers each lane's declared assurance to the **weakest ceiling** implied by its observed
states. The ceilings:

| State (best → worst) | `verified` | `bounded` | `retest_pending` | `withheld` |
| --- | --- | --- | --- | --- |
| `install_verification` | signed_verified | platform_trusted | self_signed | unverified |
| `install_topology_support` | supported | side_by_side_bounded | experimental | unsupported |
| `portable_state_freshness` | current | aging | stale | missing |
| `sync_device_state` | active | degraded | offline | blocked |
| `auth_recovery_posture` | passkey_verified | system_browser_fallback | local_only_continuity | recovery_blocked |

The **published assurance** is the minimum of the declared floor and every ceiling above, so a
desktop, marketplace, companion, managed, preview, portable, or sync lane **never silently widens
install or auth language** — a side-by-side topology caps the lane at `bounded`, a stale portable
package or an offline device caps it at `retest_pending`, and a blocked device or a
policy-blocked recovery caps it at `withheld`, regardless of how strong the other states are.

The **admission outcome** mirrors the published label one-to-one: `admit_full` for `verified`,
`admit_bounded` for `bounded`, `admit_retest` for `retest_pending`, and `refuse` for `withheld`.
The recorded published label, admission outcome, and downgrade reasons must equal the recomputed
gate decision, so a narrowed lane cannot stay stable by inertia.

### Downgrade reasons and recovery

The six headline downgrade reasons are recomputed from the observed states, not asserted by hand:

- `unverified_install` — the binary is not signed and verified.
- `unsupported_install_topology` — the topology is side-by-side bounded, experimental, or
  unsupported.
- `stale_portable_state` — the portable/export state is aging, stale, or missing.
- `blocked_sync_apply` — the device is degraded, offline, or blocked from sync.
- `missing_passkey` — no verified passkey is enrolled and recovery fell back to a weaker path.
- `policy_limited_recovery` — account recovery is limited by organization policy.

A narrowed or refused lane must offer a real recovery path (`verify_install_signature`,
`switch_supported_topology`, `refresh_portable_state`, `restore_sync_apply`, `enroll_passkey`,
`request_recovery_policy`, or `withhold_claim`), list at least one caveat, and name what is stale
or narrowing. A `verified` lane must be genuinely whole-trust — signed, supported, current,
sync-active, passkey-ready, with authoritative local continuity and nothing narrowing — so a lane
never widens install or auth language over an unverified or degraded install.

## Local-first continuity

Local durable state stays authoritative when sync or auth degrades. A lane whose sync or auth
degraded must report a `local_only_fallback` (local-only durable state that still works) or a
`policy_restricted` continuity (local-safe work restricted by policy), never claim its local
state is still fully `authoritative`. The `sync_device` lane is the worked example: even when the
device is blocked from sync and no passkey is enrolled, local durable state remains authoritative
and local-only work continues — only the sync-apply claim is withheld.

<a id="release-evidence"></a><a id="help-about"></a><a id="support-export"></a><a id="diagnostics"></a><a id="cli"></a><a id="admin-docs"></a>

## Consumer surfaces

Release center, Help/About, support export, diagnostics, CLI, and admin docs each bind to this
one packet via an `InstallConsumerBinding`, ingest it, preserve its labels and recovery paths,
and narrow with it, so a row narrowed here cannot read as stable downstream. The
`export_projection` is the redaction-safe index those surfaces render instead of restating each
lane's posture by hand, and the `support_export` wrapper preserves the exact matrix for support
and evidence packets. Both carry typed states and opaque refs only — no credential bodies, raw
provider payloads, or workspace contents.

## Conformance anchors

<a id="desktop-stable"></a><a id="desktop-preview"></a><a id="portable-install"></a><a id="managed-fleet"></a><a id="marketplace-companion"></a><a id="cli-headless"></a><a id="sync-device"></a>

Each lane's row references this document for its conformance, release-evidence, Help/About,
support-export, and diagnostics anchors. The typed model and its validation gate live in
`crates/aureline-install/src/m5_install_and_portability_governance/`, the JSON Schema at
`schemas/install/m5-install-and-portability-governance.schema.json`, and the fixture corpus at
`fixtures/install/m5/m5-install-and-portability-governance/`.
