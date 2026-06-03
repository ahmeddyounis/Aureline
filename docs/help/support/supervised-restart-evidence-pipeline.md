# Supervised-Restart Evidence Pipeline

## Overview

The supervised-restart evidence pipeline captures restart lineage, fault-domain
identity, host-lane identity, strike budget, quarantine state, and reattach /
no-rerun policy in one exportable evidence packet. It makes local, remote,
extension, debug, and notebook restarts visible to Support Center, Diagnostics
Center, status surfaces, and support bundles with exact-build correlation.

## When this pipeline is produced

- After any host-lane restart, crash-loop entry, quarantine, or reattach event.
- Before a support bundle is exported so the bundle carries restart truth.
- During shiproom proof review so stable rows show restart, quarantine, and
  reattach posture consistently.

## What the packet contains

### Restart lineage entries

One entry per host lane records:
- **Fault domain** — `local`, `remote`, `extension`, `debug`, or `notebook`.
- **Trigger** — `host_crash`, `user_initiated_restart`, `supervisor_restart`,
  `reattach_attempt`, `quarantine_entered`, or `policy_disable`.
- **Strike count and budget** — current strikes and the window budget.
- **Budget state** — `within_budget`, `budget_warning`, `budget_exhausted`,
  `quarantined`, `no_automatic_restart`, `reattach_review_required`, or
  `disabled`.
- **Exact-build identifier** — the build id of the running binary when the
  event occurred.

### Host-lane identity records

Each lane records its stable identity:
- Host family label and fault-domain id.
- Boundary badges (local, isolated, extension-owned, kernel-stateful,
  execution-facing, remote-boundary, managed-boundary, partial-truth).
- Whether the lane can perform mutating work.
- Whether the lane is externally routed.
- Current health token and restart budget reference.
- Affected capabilities, preserved checkpoints, and partial-truth result refs.

### Supervised restart review decisions

Per-lane decisions that keep reattach honest:
- `current` — the lane is live and current.
- `auto_reattached_stale_refresh` — non-mutating lane reattached safely but
  stale results need refresh.
- `review_required` — explicit review is needed before claiming current.
- `reapproval_required` — the user must reapprove before the lane resumes
  mutating or externally routed work.
- `rerun_required` — the captured action must be explicitly rerun.
- `blocked_manual_repair` — manual repair blocks reattach.

A lane that requires explicit review can never be accepted as current.

### No-rerun policy records

The policy that prevents silent rerun after crash, restart, or reattach:
- `safe_rehydrate` — the lane may rehydrate without explicit review (used for
  non-mutating lanes such as language analysis).
- `explicit_rerun_required` — the lane requires an explicit rerun confirmation.
- `reapproval_required` — the lane requires reapproval before any rerun
  (applied to mutating or externally routed lanes).
- `blocked_until_repair` — the lane is blocked from rerun until manual repair
  (applied to quarantined or crash-loop lanes).

Mutating lanes (debug, notebook) and externally routed lanes (remote) always
carry a policy that forbids silent rerun.

### Fault-domain restart summaries

Per-domain summaries that dashboards and support reviewers read at a glance:
- Restart entry count and lane count.
- Quarantined lane count.
- Lanes requiring explicit review.
- Mutating and externally routed lane counts.
- Whether any lane in the domain blocks a healthy claim.

## How to read the packet

The packet is JSON and can be opened in Diagnostics Center, included in a
support bundle, or read from the command line.

```json
{
  "record_kind": "supervised_restart_evidence_packet",
  "schema_version": 1,
  "packet_id": "supervised-restart:example",
  "workspace_id": "workspace:example",
  "generated_at": "2026-06-02T23:25:40Z",
  "build_id": "aureline-build:example:m4",
  "lineage_entries": [...],
  "host_lane_identities": [...],
  "review_decisions": [...],
  "no_rerun_policies": [...],
  "domain_summaries": [...]
}
```

The `export_safe` field is always `true`; the packet never carries raw
secret-bearing material, paths, or live authority handles.

## Related documentation

- [Crash-loop recovery center](../m3/crash_loop_recovery_beta.md)
- [Fault-domain views and host lanes](../m3/host_lane_and_reattach_beta.md)
- [Runtime replay packs](../m3/runtime_replay_packets.md)
- [Recovery ladder](../m3/recovery_ladder_alpha.md)
