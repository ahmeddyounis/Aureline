# M5 sync-and-device review fixtures

Fixture corpus for the `m5_sync_and_device_review` record. These fixtures pin the
seeded M5 scope bundles, their schema version, revision sets, redaction mode,
capability dependencies, field-aware conflicts, device-action audit trail, and
degraded-state drills so a change to the typed model, the fail-closed gate, or
the records is caught against frozen evidence.

- Typed model: `crates/aureline-settings/src/m5_sync_and_device_review/`
- Schema: `schemas/settings/m5/m5-sync-and-device-review.schema.json`
- Companion doc: `docs/settings/m5/m5-sync-and-device-review.md`
- Emitter: `aureline_settings_m5_sync_and_device_review`

## Files

- `corpus_manifest.json` — indexes the scenarios and what each proves (claim
  class and trust ceiling).
- `fully-synced-baseline.json` — every M5 family resolves to a clean-synced scope
  bundle with explicit revision sets, the full device-action catalog, and every
  degraded-state drill.
- `same-key-divergent-drill.json` — local and remote disagree on a key; the
  divergence is held field-aware and local stays authoritative.
- `policy-locked-drill.json` — an admin policy locks a key locally; the remote
  value is blocked rather than silently winning.
- `missing-capability-drill.json` — the remote value needs a capability that is
  absent on this device, so the apply is held.
- `machine-only-drill.json` — a machine-only field that never reconciles across
  devices and stays excluded from sync.
- `delete-versus-modify-drill.json` — one side deleted a key while the other
  modified it; the conflict is held for review.
- `stale-remote-drill.json` — the remote revision predates the local common
  ancestor, so the stale payload is rejected.
- `trust-widening-blocked-drill.json` — remote fields that would widen trust,
  extension permissions, AI egress, or a managed entitlement are each blocked and
  require explicit review.

## What the corpus proves

- **Sync is field-aware, not last-writer-wins.** Every scope bundle is a typed
  packet with a schema version, capability dependencies, redaction mode, source
  device/profile, and local/remote revision sets; divergences are classed
  per-field (same-key divergent, policy-locked, missing-capability, machine-only,
  delete-versus-modify, stale-remote) with a disposition that never auto-applies
  a remote value.
- **Local durable state stays authoritative.** Every bundle marks local
  authoritative, and when transport, encryption, or policy degrades the row is
  held local-authoritative and visibly labeled rather than overwritten.
- **Trust never widens silently.** A remote field that would raise a trust
  boundary, grant extension permissions, open AI egress, or widen a managed
  entitlement is blocked and requires explicit review; it can never be recorded
  as applied.
- **Device actions are reversible and audited.** Pause, resume, revoke, forget,
  and rotate each carry a durable audit reference and the explicit statement that
  local durable state remains intact.
- **Degraded states are drilled honestly.** Offline, stale-remote, blocked-apply,
  E2EE-unavailable, and local-only fallback each keep local authoritative,
  labeled, and recoverable.
- **Surfaces agree.** Desktop settings, CLI inspect, docs/help, and support
  exports all consume the same record.

The fixtures carry typed states and opaque refs only — no secrets, raw provider
payloads, hostnames, serials, or workspace contents.
