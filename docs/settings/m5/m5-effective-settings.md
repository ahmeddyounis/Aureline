# M5 Effective-Settings Parity

**Doc ref:** `docs/settings/m5/m5-effective-settings.md`  
**Contract ref:** `settings:m5_effective_settings:v1`  
**Schema version:** 1

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
shadowed), what restart posture applies, whether the value validates, and
whether a lifecycle-sensitive dependency narrows the behavior.

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
- `policy_lock` — whether an admin policy locks the row, and the policy ref.
- `lifecycle_dependency` — an optional visible marker when a missing capability
  or a Labs/Preview dependency narrows the row, with a recovery hint.

## Scope-explicit, checkpointed writes

A row whose `high_impact_class` is set — `trust_boundary`, `ai_network_egress`,
`extension_behavior`, `remote_exposure`, or `destructive_automation` — must carry
a `write_preview`. The preview is scope-explicit (it names the target scope), it
declares the effect once applied (`becomes_winning_value`, `shadowed_by_policy`,
or `denied_by_lock`), it requires confirmation, and it carries a rollback
checkpoint. A policy-locked row can never preview a write that would silently
win; the gate refuses to build such a record.

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

The settings UI, CLI inspect, help links, policy explainers, and support bundles
all consume this record and narrow with it. They must not invent private reads
or copy-only shadow settings.

## Guardrails

The record carries typed states and opaque refs only — no secrets, raw provider
payloads, credential bodies, or workspace contents. New M5 features must keep
non-stable and policy-gated behavior visible instead of hiding it behind generic
toggles or opaque precedence chains.
