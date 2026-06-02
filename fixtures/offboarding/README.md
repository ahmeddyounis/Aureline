# Offboarding and usage-export fixtures

This directory contains versioned offboarding packets and usage-export records
produced by the `aureline_policy::finalize_open_vs_paid_boundary_and_offboarding`
lane.

## Machine-generated fixtures

- `offboarding_packets.json` — All offboarding packets for managed and
  enterprise-governed capabilities from the seeded stable page.

## Offboarding state vocabulary

The offboarding packets use these closed-vocabulary outcome states:

- `local_only` — Data remains on-device only; no managed copy exists.
- `managed_copy` — A managed copy exists; the local copy remains.
- `queued` — Export or deletion is queued and not yet completed.
- `partial` — Only a partial export or partial deletion is available.
- `blocked_by_hold` — Action is blocked by an administrative or legal hold.
- `policy_retained` — Record is retained for policy, compliance, or audit reasons.
- `outside_platform_scope` — Record is outside the platform scope and not managed.
- `completed` — Offboarding action for this record is fully completed.

## Grace-window states

- `active` — Grace window is active; exports and local continuity are preserved.
- `expired` — Grace window has expired; managed capabilities are paused.
- `export_only` — Only export routes remain available; managed features are paused.
- `degraded` — Local core is preserved but managed features are degraded.

## Schema

- Usage-export packet schema: `schemas/policy/usage-export.schema.json`
- Offboarding packet schema: defined within `schemas/policy/usage-export.schema.json`
