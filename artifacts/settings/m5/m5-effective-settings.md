# M5 Effective-Settings And Policy Explainability

**Artifact ref:** `artifacts/settings/m5/m5-effective-settings.md`  
**Contract ref:** `settings:m5_effective_settings:v2`  
**Schema version:** 2  
**As of:** 2026-06-12

## Purpose

This artifact certifies that M5-added settings families answer the same four
questions the stable resolver answers for every row: which value is active,
which scope won (with the shadow chain that lost), which
`source/effective/live` projection is currently active, what restart posture
applies, and whether a lifecycle-sensitive dependency narrows the behavior.
High-impact M5 settings carry scope-explicit, checkpointed write previews so a
change to trust, AI/network egress, extension behavior, remote exposure, or a
destructive-automation default is reversible and never hidden behind an opaque
toggle. Locked and constrained rows explain their governing bundle or scope,
review/expiry posture, and local-safe continuation rather than returning
copy-only denial text.

## Certification Scope

The canonical record binds:

1. Six M5 settings families — `notebooks`, `data_api`, `profiler`, `bundle`,
   `sync`, and `companion` — each with at least one effective-settings row.
2. Per-row truth: a stable `setting_id`, the winning value plus winning scope,
   the shadow chain of losing candidates, the `restart_posture`, the
   `validation_state` (`valid`, `coerced_to_default`, `out_of_range`,
   `schema_stale`), the policy-lock/constraint state, and the active
   `source/effective/live` review sheet.
3. Lifecycle-sensitive dependency markers (`missing_capability`,
   `labs_or_preview_dependent`) that stay visible with a recovery hint.
4. Scope-explicit write previews with a target scope, current and proposed
   values, the write effect (`becomes_winning_value`, `shadowed_by_policy`,
   `denied_by_lock`), the restart posture after write, a confirmation
   requirement, a rollback checkpoint, and a constraint explanation when the
   write is shadowed or denied.
5. Admin-distribution audit rows proving where each family’s governing bundle
   came from, when it last applied and validated, whether the row is being
   viewed as source/effective/live truth, and what local-safe continuation
   remains under stale or expired posture.
6. Surface-parity rows proving the settings UI, CLI inspect, help links,
   Help/About, policy explainers, admin-distribution audit, and support bundles
   render the same record.
7. A fail-closed trust gate: a record cannot be built that records a high-impact
   row without a checkpointed write preview, omits a constrained/locked write
   explanation, advertises a policy-locked write that would silently win, omits
   a required audit row, lists a winning scope inside its own shadow chain, or
   hides a lifecycle-dependency marker.

## Canonical Paths

- Typed model: `crates/aureline-settings/src/m5_effective_settings/`
- Headless emitter: `crates/aureline-settings/src/bin/aureline_settings_m5_effective_settings.rs`
- Schema: `schemas/settings/m5/m5-effective-settings.schema.json`
- Fixtures: `fixtures/settings/m5/m5-effective-settings/`
- Companion doc: `docs/settings/m5/m5-effective-settings.md`
- Emitter: `aureline_settings_m5_effective_settings`

## Corpus Outcomes

| Scenario | Claim | Trust ceiling |
| --- | --- | --- |
| `fully_active_baseline` | `fully_active` | `active` |
| `policy_locked_drill` | `narrowed_active` | `narrowed` |
| `policy_constrained_drill` | `narrowed_active` | `narrowed` |
| `missing_capability_drill` | `narrowed_active` | `narrowed` |
| `labs_preview_dependent_drill` | `narrowed_active` | `narrowed` |
| `stale_schema_drill` | `narrowed_active` | `withheld` |

## Guardrails

The record carries typed states and opaque refs only. No secrets, raw provider
payloads, credential bodies, or workspace contents are serialized into
effective-settings records. New M5 features must not hide non-stable or
policy-gated behavior behind generic toggles or opaque precedence chains.
