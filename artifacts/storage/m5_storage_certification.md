# M5 Storage Certification Review

This review packet certifies storage-class truth, low-disk / managed-quota
behavior, class-selective clear-data previews, and pin/retention integrity for
every claimed M5 heavy-artifact family on every claimed M5 profile, using one
shared certification index.

## Evidence

| Evidence | Path |
| --- | --- |
| Rust packet | `crates/aureline-support/src/m5_storage_certification/mod.rs` |
| Boundary schema | `schemas/storage/m5_storage_certification.schema.json` |
| Reviewer doc | `docs/storage/m5_storage_certification_contract.md` |
| Canonical fixture | `fixtures/storage/m5_storage_certification/packet.json` |
| Storage-class truth | `fixtures/storage/m5_artifact_family_storage_matrix/support_export.golden.json` |
| Clear-data review | `fixtures/storage/m5_clear_data_review/support_export.golden.json` |
| Low-disk pressure | `fixtures/storage/m5_storage_pressure/support_export.golden.json` |
| Pin/retention audit | `fixtures/storage/m5_pin_retention/support_export.golden.json` |
| Corruption repair | `fixtures/storage/m5_cache_repair/support_export.golden.json` |
| Export-before-delete | `fixtures/storage/m5_offboarding_continuity/support_export.golden.json` |

## Review Findings

| Area | Result |
| --- | --- |
| Canonical certification index | Every claimed family/profile row binds storage-class-truth, clear-data, low-disk, pin/retention, corruption-repair, and export-before-delete proof in one checked packet. |
| Storage-class consistency | Each row's storage class, authority, and protection posture is cross-checked against the storage-governance matrix; the certification cannot invent a storage truth the matrix does not publish. |
| Pressure-source truth | Only `managed_cloud` adds a managed quota ceiling; protected families on `managed_cloud` stay `managed_quota_protected_excluded`, so quota can never silently delete user-owned or evidence-grade state. |
| Downgrade automation | Stale storage-class, clear-data, pressure, pin/retention, corruption-repair, export-before-delete, or consumer-binding proof can no longer keep a broad claim green. |
| Shared consumer contract | Help/About, service health, support export, and release manifest bindings all point to the same packet id and preserve the same row fields verbatim. |
| Export safety | The certification remains metadata-only and by-reference; raw artifact payloads, raw caches, raw logs, and secrets stay outside this boundary. |

## Current posture

- `desktop_local_first`, `hybrid_remote_attach`, `self_hosted_sovereign`, and
  `air_gapped_mirror_only` carry local-disk-only pressure for every family;
  storage governance is local-first and applies identically on each.
- `managed_cloud` adds a managed quota ceiling for disposable and rebuildable
  families (`disk_and_managed_quota`) while keeping protected families excluded
  from quota-driven deletion (`managed_quota_protected_excluded`).
- Degraded fixtures prove that a stale pin/retention audit gates every protected
  family to `protected_review_gated_only`, and that stale/blurred storage-class
  truth blocks authoritative families as `blocked_unverified` and narrows
  disposable, pressure-evicted families to `limited_class_scoped`.

## Follow-ups

- If live Help/About, service-health, or release-manifest renderers are added
  for this lane, they should ingest `packet.json` directly rather than copying
  its labels into a new local model.
- If future M5 heavy-artifact families are added to the storage-governance
  matrix, they should extend this certification row set in the same change.
