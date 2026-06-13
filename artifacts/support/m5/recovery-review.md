# M5 Recovery Review Evidence

This artifact freezes the support/export contract for bounded M5 host recovery.

## Covered recovery classes

- crash-loop review for repeated notebook-kernel failure with exact-build and
  no-hidden-rerun recovery paths
- scoped reset or reattach review for remote-agent identity, policy, and auth
  drift
- quarantine or rollback review for quarantined hosts and restart-budget abuse
- bounded continuity rows showing that preview, provider-run, connector, and
  adapter failures preserve surrounding layout and checkpoints

## Guaranteed export truth

- crash-loop review keeps visible crash id, build id, reopen mode, safe mode,
  open without restore, logs, export, and targeted disable paths
- scoped reset review keeps previous versus current host identity, preserved
  state, lost state, replay risk, rerun requirement, and drift signals
- quarantine review keeps trigger, scope, evidence, rollback candidate, and
  support/export follow-up path
- continuity rows keep the same fault-domain vocabulary used by the topology
  inspector, runtime badges, and crash-store viewer
- the packet remains metadata-only and does not embed raw dumps, payloads,
  secrets, or workspace contents

## Checked-in references

- Schema: `schemas/support/recovery_review.schema.json`
- Doc: `docs/support/recovery-review.md`
- Fixture: `fixtures/support/m5/recovery_review/packet.json`
- Topology source: `crates/aureline-runtime/src/topology_inspector/mod.rs`
- Crash source: `crates/aureline-support/src/crash_store/mod.rs`
- Packet source: `crates/aureline-support/src/recovery_review/mod.rs`
