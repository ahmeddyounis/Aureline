# M5 Storage Certification

## Overview

This packet is the canonical certification index for M5 heavy-artifact storage
truth. It does not introduce new runtime behavior; it binds the already-landed
storage-governance matrix and the clear-data review, low-disk / managed-quota
pressure, pin/retention, cache-repair, and offboarding/continuity lanes into one
shared decision surface.

The machine-readable truth source is:

- `fixtures/storage/m5_storage_certification/packet.json`

Downstream consumers must ingest that same index rather than re-deriving storage
maturity from local copy:

- Help/About
- service health
- support export
- release manifest / publication truth

## Profiles and families covered

The certification covers these claimed M5 profiles:

- `desktop_local_first`
- `hybrid_remote_attach`
- `managed_cloud`
- `self_hosted_sovereign`
- `air_gapped_mirror_only`

It covers every heavy-artifact family the M5 depth lanes add, plus the
user-owned recovery state they touch:

- `generated_preview`, `notebook_output`
- `docs_pack`, `model_pack`, `template_pack`, `extension_download`
- `prebuild_layer`
- `profiler_trace`, `replay_bundle`, `support_artifact`,
  `review_incident_evidence`
- `user_owned_recovery_state`

Each family/profile row names one published state:

- `qualified`
- `limited_class_scoped`
- `protected_review_gated_only`
- `blocked_unverified`

## What each row proves

Every certification row carries:

- the governing `storage_class_id`, `authority_class`, and
  `protected_continuity` flag — quoted from the storage-governance matrix so the
  certification never invents a storage truth the matrix does not also publish
- the `pressure_source_posture` for the family on that profile
- the storage-class-truth proof ref
- the class-selective clear-data review proof ref
- the low-disk / managed-quota pressure proof ref
- the pin/retention audit proof ref
- the corruption-repair drill proof ref
- the export-before-delete / offboarding-continuity proof ref
- the published state plus any active `stale_proof_tokens`
- the active downgrade-rule ids that explain the narrowed state

A support, release, or Help/About surface can therefore answer exactly what is
qualified for a heavy-artifact family on a given profile without inventing local
wording.

## Pressure-source posture

Only the `managed_cloud` profile adds a managed quota ceiling on top of local
disk pressure. The posture is computed per row:

- `local_disk_only` — every non-managed profile; only local disk pressure can
  trim the family, in the frozen eviction order.
- `disk_and_managed_quota` — a disposable or rebuildable family on
  `managed_cloud`; both local disk pressure and a managed quota ceiling apply.
- `managed_quota_protected_excluded` — a protected family on `managed_cloud`;
  managed quota may never auto-delete it, so only explicit, reviewed removal can
  free it. This is the guard that keeps managed quota from silently deleting
  user-owned or evidence-grade state.

## Required downgrade behavior

The packet freezes downgrade automation for these cases:

- `storage_class_truth_stale` — blurs cache versus authoritative state; blocks
  the broad claim.
- `clear_data_review_stale` — narrows the clear-data claim so no generic clear
  is advertised that could reach protected or user-owned state.
- `low_disk_pressure_proof_stale` — hides pressure behavior; narrows the
  eviction claim.
- `pin_retention_evidence_stale` — gates every protected family behind an
  explicit review.
- `corruption_repair_drill_stale` — narrows the targeted-repair claim.
- `export_before_delete_validation_stale` — gates protected families so no
  offboarding or reset removes them without an exported copy.
- `consumer_binding_missing` — blocks the broad claim until every consumer
  ingests the index by reference again.

No stale storage-class, clear-data, pressure, pin/retention, corruption-repair,
or export-before-delete proof may keep a broader M5 claim green.

## Shared-surface binding

The certification includes one `surface_binding` row for:

- `help_about`
- `service_health`
- `support_export`
- `release_manifest`

Every binding must preserve these fields verbatim:

- `certification_row_id`
- `family_id`
- `profile`
- `published_state`
- `stale_proof_tokens`
- `downgrade_rule_ids`

If one consumer stops ingesting the certification by reference, the broad claim
blocks until parity is restored.

## Export safety

The certification is metadata-only and by reference. It carries no raw artifact
payloads, raw caches, raw logs, or live credentials; it cites the same
metadata-safe golden support-export projections its sibling lanes already check
in.
