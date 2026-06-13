# M5 Effective-Settings And Policy Explainability

**Doc ref:** `docs/settings/m5/m5-effective-settings.md`  
**Contract ref:** `settings:m5_effective_settings:v2`  
**Schema version:** 2

## Overview

This document defines the shared product truth for the new M5 settings families.
The canonical record is `M5EffectiveSettingsCertification` in
`crates/aureline-settings/src/m5_effective_settings/`. The settings UI, CLI
inspect, help links, policy explainers, and support bundles consume the same
record so they answer the same questions for an M5 setting instead of cloning
shadow status text.

The record extends the stable effective-settings resolver to the families that
M5 adds — notebooks, data/API, profiler, bundle, sync, and companion — without
letting them ship private precedence rules. For each row it makes resolution
honest: which value is active, which scope won (and which candidates were
shadowed), whether the row is currently being inspected as `source`,
`effective`, or `live` truth, what restart posture applies, whether the value
validates, and whether a lifecycle-sensitive dependency narrows the behavior.

## M5 settings families

Every record covers all six M5-added families:

- `notebooks`
- `data_api`
- `profiler`
- `bundle`
- `sync`
- `companion`

## Per-row truth

Each `M5EffectiveSettingRow` carries:

- `setting_id` — stable identifier shared by every surface.
- `winning_value` — the active value and the scope that won.
- `shadow_chain` — the ordered candidates that lost, each with the scope and the
  reason it was shadowed (`lower_precedence`, `policy_narrowed`,
  `validation_rejected`). A row may never list its winning scope here.
- `restart_posture` — `no_restart`, `reload_view`, `reload_workspace`,
  `restart_extensions`, or `restart_process`. Surfaces quote it verbatim.
- `validation_state` — `valid`, `coerced_to_default`, `out_of_range`, or
  `schema_stale`. A stale-schema value is withheld until migrated.
- `policy_lock` — whether an admin policy leaves the row `unlocked`,
  `constrained`, or `locked`, plus the governing policy ref, source bundle or
  scope, bundle owner, distribution source, last-applied time, review/expiry
  timing where present, and the local-safe continuation facts that still apply.
- `effective_value_review` — the governed effective-value review sheet: selected
  keys, active `source/effective/live` projection, the available projection
  switch targets, winning layers, unresolved values, export posture, and the
  bounded actions a surface may offer.
- `lifecycle_dependency` — an optional visible marker when a missing capability
  or a Labs/Preview dependency narrows the row, with a recovery hint.

## Locked and constrained writes

A row whose `high_impact_class` is set — `trust_boundary`, `ai_network_egress`,
`extension_behavior`, `remote_exposure`, or `destructive_automation` — must carry
a `write_preview`. The preview is scope-explicit (it names the target scope), it
declares the effect once applied (`becomes_winning_value`, `shadowed_by_policy`,
or `denied_by_lock`), it requires confirmation, and it carries a rollback
checkpoint. Whenever the preview is constrained or denied, it also carries a
`WriteConstraintExplanation` that names the source bundle or scope, bundle
owner, review or expiry facts where available, the repair hint, and what still
works under the narrower local-safe posture. A policy-locked row can never
preview a write that would silently win; the gate refuses to build such a
record.

## Admin-distribution audit

Every record carries one `AdminDistributionAuditRow` per M5 family. The audit
rows preserve:

- `bundle_ref` and `bundle_owner_ref` — the active bundle identity and owner.
- `policy_scope_ref` — the scope at which the bundle applied.
- `distribution_source` — `managed_pull`, `mirror_publication`, `file_import`,
  `mdm_fleet_drop`, `air_gap_transfer`, or `last_known_good_cache`.
- `last_applied_at` and `last_validated_at` — the stable timestamps Help/About,
  CLI inspect, admin audit, and support export quote instead of cloning prose.
- `active_projection_mode` — whether the audit row is currently shown as source,
  effective, or live truth.
- `freshness_state` — `current`, `stale`, or `expired`.
- `local_safe_continuation` — the exact bounded actions that remain available
  while the bundle is stale or expired.

## Trust ceiling

The record publishes the weakest row trust as its `effective_trust_ceiling`:

- `active` — the value validates and no dependency or lock narrows it.
- `narrowed` — the value resolves but a lock, coercion, or lifecycle dependency
  narrows it.
- `withheld` — the value cannot be trusted as resolved (e.g. a stale schema).

The derived `claim_class` is `fully_active` only when every pillar holds and the
ceiling is `active`; otherwise it is `narrowed_active`, or `unsupported` when a
structural pillar fails.

## Surfaces

The settings UI, CLI inspect, help links, Help/About, policy explainers,
admin-distribution audit, and support bundles all consume this record and
narrow with it. They must not invent private reads, shadow settings, or
bundle-owner explanations of their own.

## Guardrails

The record carries typed states and opaque refs only — no secrets, raw provider
payloads, credential bodies, or workspace contents. New M5 features must keep
non-stable and policy-gated behavior visible instead of hiding it behind generic
toggles or opaque precedence chains.
