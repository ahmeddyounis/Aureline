# M5 install/config/auth certification — reviewer artifact

Human-readable companion to
`artifacts/install/m5/m5-install-config-auth-certification.json`. The full
contract and gate semantics live in
`docs/install/m5/m5-install-config-auth-certification.md`; the typed model lives
in the `aureline-install` crate (`m5_install_config_auth_certification`).

This artifact freezes one certification row per claimed M5 profile and
publishes, for each, only the support label its install-topology,
configuration-portability, sync/device, and auth-recovery evidence actually
backs. Aging, stale, missing, or below-verified source evidence narrows or
withholds the row automatically before release, Help/About, docs/help, admin,
diagnostics, CLI, or support-export surfaces can publish it.

## Certification roll-up (as of 2026-06-12)

| Profile | Topology | Config portability | Sync/device | Auth/recovery | Declared | Published | Downgrade |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `desktop_stable` | verified | verified | verified | verified | verified | **verified** | none |
| `desktop_preview` | bounded | verified | bounded | verified | verified | **bounded** | refresh_evidence |
| `portable` | verified | retest_pending | verified | verified | verified | **retest_pending** | refresh_evidence |
| `managed_fleet` | bounded | verified | bounded | verified | verified | **bounded** | requalify_source |
| `mirror_offline` | retest_pending | withheld | withheld | retest_pending | verified | **withheld** | withhold_claim |

One profile admits at full trust (`desktop_stable`), proving the gate is not a
blanket downgrade. Four rows narrow below what they declared, and the
`mirror_offline` claim is withheld entirely because portability and sync
evidence are missing while topology and auth evidence are stale.

## What the certification proves

- **Every claimed profile is explicit.** Stable desktop, preview side-by-side,
  portable, managed fleet, and mirror/air-gap are certified separately rather
  than inheriting a single connected default.
- **Settings and sync stay scope-honest.** Effective-settings parity, portable
  export/import, sync scope bundles, device participation, and local-only
  fallback are represented as first-class domain evidence, not implied from an
  install badge.
- **Managed and offline claims do not overreach.** `managed_fleet` is bounded by
  governance and org-scoped sync qualification; `mirror_offline` is withheld
  until offline portability, sync, topology, and recovery evidence are current.
- **Auth recovery keeps phishing resistance and accessibility visible.**
  System-browser sign-in, passkey, step-up, and recovery depth are explicit
  domain evidence and are drilled independently from install or sync state.
- **All named consumers bind to one packet.** Release center, About, docs/help,
  admin docs, support export, diagnostics, and CLI each preserve the same
  labels and downgrade paths through `CertificationConsumerBinding`.

## Troubleshooting drills

The checked-in packet carries nine detected drills:

- `drill-install-topology`
- `drill-side-by-side`
- `drill-portable`
- `drill-mirror-offline`
- `drill-settings-portability`
- `drill-sync-device`
- `drill-passkey-recovery`
- `drill-accessibility`
- `drill-downgrade`

Together they prove the object catches install-topology, coexistence, portable,
mirror/offline, settings-portability, sync/device, passkey/recovery,
accessibility, and stale-evidence downgrade scenarios before publication.
