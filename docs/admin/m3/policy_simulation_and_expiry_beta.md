# Policy Simulation And Expiry Beta Contract

This contract describes the beta policy-governance row set used by the
desktop shell, headless inspector, support exports, and offline/file-based
admin paths. It complements the broader admin policy explainability and
governed-record documents without introducing a hosted-console dependency.

## Companion Artifacts

- `/crates/aureline-policy/src/simulation/` owns the Rust types,
  seeded page, remembered-decision drift check, support export wrapper,
  and typed validator defects.
- `/schemas/policy/policy_simulation.schema.json` freezes the pre-apply
  simulation and action-time policy-state export boundary.
- `/schemas/policy/exception_and_memory_state.schema.json` freezes
  exception, waiver, and remembered-decision rows.
- `/docs/verification/policy_simulation_packet.md` remains the
  verification seed for diff severity, chronology bars, dashboard joins,
  and drift scenarios.

## Required Behavior

Admins can preview policy-bundle and settings-lock changes before apply.
Each preview names affected personas, action families, command ids,
degraded modes, protected-path changes, overlapping exceptions or
waivers, remembered-decision refs, and fields that must survive admin or
support export.

Exceptions and waivers are durable authority records, not vague bypass
states. Every row carries owner, scope, evidence refs, expiry, renewal
path, revocation path, status, dashboard bucket, and audit lineage. Raw
justification text is excluded from default support/export packets.

Remembered decisions are reusable only inside the exact binding they were
minted for: actor, object, action family, environment, policy epoch,
target version, authority epoch, and time horizon. Drift in any binding
forces explicit invalidation, narrowing, expiry, or reapproval instead of
silently broadening across enterprise rows.

Support and admin exports preserve the policy state that applied at
action time alongside the policy state current at export time. This keeps
historical truth available when a support packet is assembled after a
policy epoch changed.

## Headless Inspector

The shell exposes a JSON-only inspector:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_policy_simulation_beta -- page
cargo run -q -p aureline-shell --bin aureline_shell_policy_simulation_beta -- simulations
cargo run -q -p aureline-shell --bin aureline_shell_policy_simulation_beta -- exceptions
cargo run -q -p aureline-shell --bin aureline_shell_policy_simulation_beta -- remembered-decisions
cargo run -q -p aureline-shell --bin aureline_shell_policy_simulation_beta -- action-time-policy
cargo run -q -p aureline-shell --bin aureline_shell_policy_simulation_beta -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_policy_simulation_beta -- validate
```

The drill subcommands intentionally produce typed defects for expiry
omission, unexplained remembered-decision drift, and current-only support
export posture.

## Validation Rules

- A conforming page includes both policy-bundle and settings-lock
  simulations.
- Each simulation has at least one affected surface with persona, action,
  commands, degraded mode, and protected-path change.
- Exceptions and waivers fail validation when owner, scope, evidence,
  expiry, renewal path, revocation path, or audit lineage is missing.
- Remembered decisions fail validation when actor, object, environment,
  time horizon, or drift reasons are missing.
- Support exports fail validation when action-time policy state is absent
  or when current policy state would overwrite historical action truth.

## Non-Goals

This beta contract does not implement a full policy language, signature
verification, hosted admin console, or retention engine. It provides the
typed contract those systems consume so product, CLI, support, and
offline paths explain the same state with the same fields.
