# Supervised-Restart Evidence Pipeline Report

This packet reviews the supervised-restart evidence pipeline: the unified,
exportable envelope that captures restart lineage, fault-domain identity,
host-lane identity, strike budget, quarantine state, and reattach / no-rerun
policy across local, remote, extension, debug, and notebook fault domains.

The pipeline keeps recovery honest under real fault-domain churn by replacing
inferred restart behavior with typed, versioned, exportable evidence. Support
Center, Diagnostics Center, status surfaces, support bundles, shiproom proof,
and known-limits publication all ingest the same packet.

## Evidence

| Evidence | Path |
| --- | --- |
| Boundary schema | `schemas/support/supervised-restart-evidence-pipeline.schema.json` |
| Rust evaluator | `crates/aureline-support/src/supervised_restart_evidence_pipeline/mod.rs` |
| Canonical fixtures | `artifacts/support/fault_domain_packets/host_lane_fault_domain_packet.json` |
| Reviewer doc | `docs/help/support/supervised-restart-evidence-pipeline.md` |
| Tests | `crates/aureline-support/tests/supervised_restart_evidence_pipeline.rs` |

## Review Findings

| Area | Result |
| --- | --- |
| Five fault domains covered | The packet surfaces local, remote, extension, debug, and notebook domains explicitly; no domain is folded into generic reconnect copy. |
| Exact-build correlation | Every lineage entry carries the exact `build_id` so support exports and shiproom packets correlate restarts with the running binary. |
| Host-lane identity | Each lane records its family, fault-domain ownership, boundary badges, health token, restart budget ref, and capability tokens so identity survives reattach. |
| Restart lineage | One chronological entry per lane captures the trigger (crash, supervisor restart, reattach, quarantine, policy disable), strike count, budget state, and quarantine ref. |
| Reattach review | Reattach decisions (current, auto-reattached-stale-refresh, review-required, reapproval-required, rerun-required, blocked-manual-repair) are explicit per lane. |
| No-rerun policy | Mutating or externally routed lanes carry a policy that forbids silent rerun (`reapproval_required`, `explicit_rerun_required`, or `blocked_until_repair`). Non-mutating lanes may rehydrate safely (`safe_rehydrate`). |
| Degraded state over optimism | Restart storms and quarantine are never hidden under generic reconnect copy; the packet surfaces explicit degraded state and partial-truth refs. |
| Metadata-only | The packet is export-safe: no raw paths, credentials, command lines, or live authority handles embed; `export_safe = true` is pinned. |

## Support Export Posture

`SupervisedRestartEvidencePacket` compiles a metadata-safe envelope with:

- the exact-build identifier, workspace id, and generation timestamp;
- one restart lineage entry per host lane (domain, trigger, strike count,
  budget state, quarantine status, build id);
- one host-lane identity record per lane (family, fault domain, boundary badges,
  mutating capability, external routing, health, capabilities, checkpoints,
  partial-truth refs);
- one supervised restart review decision per lane (decision class, current
  claim acceptance, explicit review requirement, preserved and lost state refs);
- one no-rerun policy record per lane (policy class, silent-rerun prohibition,
  reason);
- one fault-domain restart summary per domain (restart entries, lane counts,
  quarantined lanes, review-required lanes, mutating lanes, externally routed
  lanes, healthy-claim blocks).

`render_plaintext` renders a support-safe, screen-reader-legible view of the
same truth. The envelope carries opaque ids and closed-vocabulary tokens only.

## Chaos Drill Coverage

The protected corpus exercises the following postures:

- **Local shell service** — healthy, no hidden restart budget, safe rehydrate;
- **Language analysis host** — reconnecting, within budget, stale results,
  non-mutating safe rehydrate with refresh;
- **Extension sandbox host** — quarantined, budget exhausted, blocked until
  repair, no silent rerun;
- **Debug/task adapter host** — disconnected, reattach review required,
  reapproval required before rerun;
- **Notebook kernel** — crash loop, quarantined, blocked until repair,
  partial-truth output refs preserved;
- **Remote workspace agent** — disconnected, identity drift, reapproval required,
  lost forwarded-port and live-terminal-control state;
- **Managed service lane** — degraded, circuit-breaker budget, reapproval
  required, partial-truth AI tool action.

## Follow-Ups

- Wire `SupervisedRestartEvidencePacket` into the shell status surface so
  host-lane restart badges and fault-domain cards render the same truth.
- Bridge the packet into the support-bundle export writer so support bundles
  automatically include supervised-restart lineage.
- Extend the pipeline with persistent remote-session transport drills once
  long-running remote reattach semantics land.
