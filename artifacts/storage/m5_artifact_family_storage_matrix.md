# M5 artifact-family storage matrix (human-readable)

Companion to [`m5_artifact_family_storage_matrix.yaml`](./m5_artifact_family_storage_matrix.yaml),
the boundary schema at
[`/schemas/storage/m5_artifact_family_storage_matrix.schema.json`](../../schemas/storage/m5_artifact_family_storage_matrix.schema.json),
and the contract at
[`/docs/m5/freeze-the-m5-storage-class-pin-source-clear-data-and-low-disk-ordering-matrix-for-new-artifact-families.md`](../../docs/m5/freeze-the-m5-storage-class-pin-source-clear-data-and-low-disk-ordering-matrix-for-new-artifact-families.md).

Every value in the storage-class, authority, rebuild-cost, GC-policy, clear-
protection, low-disk-ladder, and pin-source columns re-exports verbatim from
[`/artifacts/runtime/storage_classes.yaml`](../runtime/storage_classes.yaml).
The default-retention and clear-data-action columns are the only sets this
matrix introduces.

## Disposable / rebuildable derived caches

| Family | Storage class | Authority | Rebuild cost | Default retention | Clear protection | Low-disk step | Allowed clear-data actions |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Generated previews | interactive_hot_cache | disposable_derived_cache | low_rebuild_cost | evict_on_session_end | generic_clear_always_allowed | trim_interactive_hot_cache | generic_clear_in_bulk |
| Notebook outputs | artifact_cache | disposable_derived_cache | high_rebuild_cost | evict_under_pressure_if_unpinned | generic_clear_with_pin_exclusions | trim_artifact_cache_unpinned | generic_clear_excluding_pins, class_selective_clear |

## Imported / signed artifact packs

| Family | Storage class | Authority | Rebuild cost | Default retention | Clear protection | Low-disk step | Allowed clear-data actions |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Docs packs | artifact_cache | admin_or_control_artifact | medium_rebuild_cost | retain_until_version_replace | generic_clear_with_pin_exclusions | trim_artifact_cache_unpinned | generic_clear_excluding_pins, class_selective_clear |
| Model packs | artifact_cache | admin_or_control_artifact | high_rebuild_cost | retain_until_version_replace | generic_clear_with_pin_exclusions | trim_artifact_cache_unpinned | generic_clear_excluding_pins, class_selective_clear |
| Template / archetype packs | artifact_cache | admin_or_control_artifact | medium_rebuild_cost | retain_until_version_replace | generic_clear_with_pin_exclusions | trim_artifact_cache_unpinned | generic_clear_excluding_pins, class_selective_clear |
| Extension downloads | artifact_cache | admin_or_control_artifact | medium_rebuild_cost | retain_until_version_replace | generic_clear_with_pin_exclusions | trim_artifact_cache_unpinned | generic_clear_excluding_pins, class_selective_clear |

## Prebuild / environment layers

| Family | Storage class | Authority | Rebuild cost | Default retention | Clear protection | Low-disk step | Allowed clear-data actions |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Prebuild layers | prebuild_environment_cache | disposable_derived_cache | high_rebuild_cost | evict_under_pressure_if_unpinned | generic_clear_with_pin_exclusions | trim_prebuild_environment_unpinned | generic_clear_excluding_pins, class_selective_clear |

## Durable, policy-bounded evidence (protected continuity)

| Family | Storage class | Authority | Rebuild cost | Default retention | Clear protection | Low-disk step | Allowed clear-data actions |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Profiler traces | evidence_support_cache | admin_or_control_artifact | high_rebuild_cost | retain_for_policy_window | protected_requires_class_specific_review | expire_unpinned_evidence_past_retention | class_specific_review_required, export_before_delete_offered |
| Replay bundles | evidence_support_cache | admin_or_control_artifact | high_rebuild_cost | retain_for_policy_window | protected_requires_class_specific_review | expire_unpinned_evidence_past_retention | class_specific_review_required, export_before_delete_offered |
| Support artifacts | evidence_support_cache | admin_or_control_artifact | high_rebuild_cost | retain_for_policy_window | protected_requires_class_specific_review | expire_unpinned_evidence_past_retention | class_specific_review_required, export_before_delete_offered |
| Review / incident evidence | evidence_support_cache | admin_or_control_artifact | authoritative_no_rebuild | retain_for_policy_window | protected_requires_class_specific_review | expire_unpinned_evidence_past_retention | class_specific_review_required, export_before_delete_offered |

## User-owned recovery state (protected continuity)

| Family | Storage class | Authority | Rebuild cost | Default retention | Clear protection | Low-disk step | Allowed clear-data actions |
| --- | --- | --- | --- | --- | --- | --- | --- |
| User-owned recovery state | user_owned_recovery_state | user_owned_recovery_state | authoritative_no_rebuild | retain_until_explicit_user_reset | protected_never_generic_clear | user_owned_recovery_state_only_under_explicit_review | explicit_per_item_review_required, export_before_delete_offered |

## Low-disk eviction order

Trimmed early → late: generated previews → notebook outputs, docs / model /
template packs, extension downloads → prebuild layers → unpinned evidence past
retention (profiler traces, replay bundles, support artifacts, review/incident
evidence) → user-owned recovery state, only under explicit review. Protected
classes are never trimmed by a generic clear and never silently disposed by an
offboarding/reset.
