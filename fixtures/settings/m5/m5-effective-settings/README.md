# M5 effective-settings fixtures

Fixture corpus for the `m5_effective_settings` record. These fixtures pin the
seeded M5 setting rows, their winning value and shadow chain, restart posture,
validation state, lifecycle-dependency markers, and scope-explicit write
previews so a change to the typed model, the fail-closed gate, or the records is
caught against frozen evidence.

- Typed model: `crates/aureline-settings/src/m5_effective_settings/`
- Schema: `schemas/settings/m5/m5-effective-settings.schema.json`
- Companion doc: `docs/settings/m5/m5-effective-settings.md`
- Emitter: `aureline_settings_m5_effective_settings`

## Files

- `corpus_manifest.json` — indexes the scenarios and what each proves (claim
  class and trust ceiling).
- `fully-active-baseline.json` — every M5 family resolves to a fully-active
  value with an explicit winning scope, shadow chain, restart posture, and a
  checkpointed write preview for each high-impact row.
- `policy-locked-drill.json` — an admin policy locks the companion setting; the
  winning value comes from the policy ceiling and the write preview is denied by
  the lock rather than silently winning.
- `missing-capability-drill.json` — the profiler setting is narrowed by a
  missing capability, surfaced as a visible lifecycle-dependency marker.
- `labs-preview-dependent-drill.json` — the notebook setting is narrowed because
  it depends on a Labs/Preview feature lifecycle.
- `stale-schema-drill.json` — the bundle setting is read from a stale schema, so
  its winning value is withheld until migrated instead of being trusted.

## What the corpus proves

- **Every M5 family is at resolver parity.** Notebooks, data/API, profiler,
  bundle, sync, and companion each share one effective-settings record with a
  stable setting id, winning value, shadow chain, restart posture, and
  validation state.
- **Scope is explicit.** Every row names the scope that won and the candidates
  it shadowed; no row lists its own winning scope in the shadow chain.
- **High-impact writes are reversible.** Every trust, AI/network-egress,
  extension-behavior, remote-exposure, or destructive-automation row carries a
  scope-explicit write preview with a rollback checkpoint and a confirmation
  requirement.
- **Non-stable behavior stays visible.** Policy locks, missing capabilities,
  Labs/Preview dependencies, and stale schemas narrow the record through visible
  markers and a published trust ceiling, never an opaque toggle.
- **Surfaces agree.** Settings UI, CLI inspect, help links, policy explainers,
  and support bundles all consume the same record.

The fixtures carry typed states and opaque refs only — no secrets, raw provider
payloads, or workspace contents.
