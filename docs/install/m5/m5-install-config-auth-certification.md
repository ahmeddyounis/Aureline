# M5 install/config/auth certification

The M5 install/config/auth certification packet is the single inspectable object
that decides whether each claimed M5 desktop, managed, and mirror/offline
profile may keep its published install, configuration-portability, sync/device,
and auth-recovery claim. Release center, Help/About, docs/help, admin docs,
diagnostics, CLI, and support exports are expected to ingest this packet instead
of cloning status text.

- Typed model: `aureline-install` crate, module `m5_install_config_auth_certification`
  ([`M5InstallConfigAuthCertification`]).
- Canonical packet: `artifacts/install/m5/m5-install-config-auth-certification.json`
- Schema: `schemas/install/m5-install-config-auth-certification.schema.json`
- Reviewer artifact: `artifacts/install/m5/m5-install-config-auth-certification.md`
- Fixtures: `fixtures/install/m5/m5-install-config-auth-certification/`

## What the packet records

One `CertificationRow` covers one claimed M5 profile:

| Profile | Covers |
| --- | --- |
| `desktop_stable` | Stable-channel desktop install |
| `desktop_preview` | Preview install running side-by-side with stable |
| `portable` | Portable install with its own durable state root |
| `managed_fleet` | Policy-controlled managed install |
| `mirror_offline` | Mirror or air-gapped install provisioned from offline media |

Each row carries exactly four `DomainQualification` entries:

- `install_topology` for side-by-side, portable, mirror/offline, and update-root
  truth;
- `config_portability` for effective-settings scope and portable/export truth;
- `sync_device` for sync scope bundles, device participation, and local-only
  fallback honesty;
- `auth_recovery` for system-browser sign-in, passkey, step-up, and recovery
  depth.

Every domain qualification binds to one canonical source packet through a closed
`SourcePacket` vocabulary and a required `source_packet_ref`, so the
certification packet can aggregate earlier B19 install/settings/identity packets
 without copying or widening their claims.

## The certification gate

The published support for a profile is computed, never asserted by hand:

```text
domain_published_support = min(declared_support, evidence_freshness ceiling)
row_published_support =
    min(row declared_support, weakest domain_published_support)
```

The gate withholds a row entirely if any required domain is missing. This is the
non-inheriting rule the packet exists to enforce: a claimed M5 row is either
qualified with current proof or automatically narrowed before publication.

The freshness ceiling is:

- `current` -> `verified`
- `aging` -> `bounded`
- `stale` -> `retest_pending`
- `missing` -> `withheld`

The packet records the exact `narrow_reasons` and `downgrade_path` that follow
from those inputs, so stale install, portability, sync, or auth evidence cannot
stay green by inertia.

## Roll-up in the checked-in packet

As of `2026-06-12`, the packet publishes:

- `desktop_stable` as `verified`
- `desktop_preview` as `bounded`
- `portable` as `retest_pending`
- `managed_fleet` as `bounded`
- `mirror_offline` as `withheld`

That roll-up exercises every gate direction: one row stays fully verified, two
rows narrow to bounded, one narrows to retest-pending, and one is withheld.

## Source-packet aggregation

The packet aggregates seven canonical sources:

- `install_governance`
- `coexistence_fleet_rollout`
- `install_diagnostics`
- `effective_settings`
- `portable_state_and_restore`
- `sync_and_device_review`
- `auth_and_recovery`

This means the same packet can be rendered by release, help/about, docs/help,
admin, diagnostics, CLI, and support surfaces without each surface re-deriving a
different certification story.

## Troubleshooting drills

Nine `CertificationDrill` entries prove the packet detects each required
scenario:

- `install_topology`
- `side_by_side`
- `portable`
- `mirror_offline`
- `settings_portability`
- `sync_device`
- `passkey_recovery`
- `accessibility`
- `downgrade`

Each drill targets a real certification row and proves stale, missing, or
underqualified evidence narrows or withholds the claim before publication.

## Consumer surfaces

Every required consumer surface binds to the packet through a
`CertificationConsumerBinding`:

- `release_center`
- `about`
- `docs_help`
- `admin_docs`
- `support_export`
- `diagnostics`
- `cli`

Each binding must ingest the packet, preserve published support and downgrade
paths, narrow automatically on downgrade, and exclude raw private material.

## Boundary safety

The packet is metadata-only. It carries typed states, opaque refs, caveats, and
review-safe notes. It does not carry credential bodies, raw provider payloads,
workspace contents, or machine-unique secret material.

[`M5InstallConfigAuthCertification`]: ../../../crates/aureline-install/src/m5_install_config_auth_certification/mod.rs
