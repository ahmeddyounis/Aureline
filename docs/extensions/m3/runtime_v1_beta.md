# Extension runtime v1 beta admission contract

This page is the reviewer-facing entrypoint for the extension runtime
v1 beta. It is the first beta contract Aureline publishes for the
extension platform: it binds capability-bounded Wasm extensions and
separately supervised external host processes into one versioned
admission record under a single policy and lifecycle model.

The contract is governed. The canonical Rust source of truth lives in
[`crates/aureline-extensions/src/runtime/`](../../../crates/aureline-extensions/src/runtime/);
the cross-tool boundary schema is
[`schemas/extensions/runtime_contract.schema.json`](../../../schemas/extensions/runtime_contract.schema.json);
the checked-in fixtures live under
[`fixtures/extensions/runtime_v1_beta_cases/`](../../../fixtures/extensions/runtime_v1_beta_cases/).

## What the contract covers

One `RuntimeV1BetaContractRecord` binds the truth that every later
surface (install / review, permission inspector, support export,
partner packet template, CLI / headless review) needs to render a
single, inspectable answer to "is this extension admitted, narrowed,
awaiting review, refused, or quarantined?":

- the manifest baseline ref and its install decision class + reason
  class consumed from `crates/aureline-extensions/src/manifest_baseline/`;
- the host contract family declared by the manifest plus a closed
  `host_placement_class` and `host_supervision_class` so a "runs
  somewhere" badge cannot quietly substitute the actual placement;
- the host-negotiation packet ref plus declared, negotiated, and
  narrowed capability-world ref sets so review surfaces can quote one
  source of truth for "what worlds did this row actually get?";
- the effective-permission summary ref, a flag for whether the
  declared-vs-effective diff was recorded, and the widening-attempted
  blocked count so missing diff reports fail closed;
- the runtime-budget summary ref plus flags for an active quarantine
  / disable and an active crash-loop trip, so the runtime never
  silently downgrades a quarantine into a generic "stopped working"
  chip;
- the lifecycle state class, restart posture class, restart attempt
  count, and degraded-state class — mirrored from
  [`artifacts/extensions/extension_lifecycle_states.yaml`](../../../artifacts/extensions/extension_lifecycle_states.yaml)
  and
  [`artifacts/extensions/quarantine_rules.yaml`](../../../artifacts/extensions/quarantine_rules.yaml);
- the SDK release-bundle ref, the marketplace metadata ref, and a
  closed `sdk_alignment_class` so runtime truth cannot drift past the
  published SDK contract and marketplace metadata.

The record always emits `RedactionClass::MetadataSafeDefault`. Raw
manifest bytes, raw signing-key material, raw policy bodies, raw
paths, raw tokens, raw helper-binary invocation bodies, raw
bridge-shim payloads, and raw runtime payload bytes MUST NOT appear
anywhere in the record set; every field is an opaque ref or a closed
vocabulary value.

## Why both Wasm and external hosts read one record

The acceptance gate for this beta is that the runtime "exposes stable
beta contracts for Wasm and external-host extensions with explicit
capability negotiation and lifecycle states." Earlier alphas had two
parallel paths: a Wasm capability-world path with its own activation
budget, and an external-host path with its own restart / crash-loop
posture. The beta collapses those into one record:

- `host_placement_class` resolves Wasm component-model / core-module
  hosts into `wasm_in_process_isolated_world` or
  `wasm_isolated_subprocess`; resolves external host processes into
  `external_host_supervised_process`; resolves short-lived helper
  binaries into `helper_binary_short_lived`; resolves remote-side
  components into `remote_side_component_attached`; and resolves
  compatibility bridges into `compatibility_bridge_translated`.
- `host_supervision_class` is the orthogonal axis: the in-process
  capability sandbox, the supervised subprocess, the helper kill
  switch, the remote-agent attached envelope, or the compatibility
  bridge's translated supervision.
- Both axes are validated against the manifest's host contract
  family. A placement / family or supervision / placement mismatch
  refuses admission with `host_placement_unsupported`, so an external
  host process cannot pretend to be an in-process capability sandbox
  and a Wasm component cannot pretend to be supervised by a remote
  agent.

That means one runtime contract record covers every host shape the
M3 wedge claims, and one support export panel quotes the same closed
tokens whether the row is a Wasm component or an external LSP host.

## Admission flow

[`evaluate_runtime_v1_beta_contract`](../../../crates/aureline-extensions/src/runtime/mod.rs)
is deterministic and fails closed. In strict precedence order:

1. If the manifest install decision class is `denied`, the runtime
   refuses with one of `publisher_identity_opaque`,
   `effective_permission_widening_attempted`, or
   `manifest_install_denied`.
2. If the declared-vs-effective permission diff is missing, the
   runtime refuses with `permission_diff_missing`.
3. If the effective-permission summary recorded a widening attempt,
   the runtime refuses with `effective_permission_widening_attempted`.
4. If the host placement / supervision is unknown or not reserved
   for the declared host contract family, the runtime refuses with
   `host_placement_unsupported`.
5. If the lifecycle state class is `publisher_blocked`, the runtime
   refuses with `publisher_identity_opaque`. `removed` and
   `discovered` refuse with `lifecycle_terminal_state`.
6. If the runtime-budget evidence reports an active crash-loop trip,
   the runtime quarantines with `crash_loop_quarantine_active`. If it
   reports an active runtime-budget quarantine or disable, OR the
   lifecycle state is `quarantined`, the runtime quarantines with
   `runtime_budget_quarantine_active`.
7. If declared worlds is empty, or negotiated worlds is empty, or
   negotiated worlds widens beyond declared, the runtime refuses with
   `capability_world_unavailable_on_host` (or
   `host_placement_unsupported` for widening).
8. If declared worlds were narrowed but no narrowing reasons were
   recorded on the negotiation packet, the runtime refuses with
   `permission_diff_missing` — the negotiation packet must record one
   typed reason per dropped world.
9. If `sdk_alignment_class` is not `aligned`, the runtime refuses
   with `sdk_or_marketplace_metadata_out_of_date`.
10. If declared worlds were narrowed and the lifecycle state is
    `pending_activation`, the runtime returns `awaiting_user_review`
    with reason `awaiting_user_world_acknowledgement`.
11. If declared worlds were narrowed otherwise, the runtime returns
    `admitted_narrowed` with reason `admitted_with_world_narrowing`.
12. Otherwise the runtime returns `admitted` with reason
    `admitted_after_capability_negotiation`.

## First consumer: support / partner export

[`project_runtime_v1_beta_support_export`](../../../crates/aureline-extensions/src/runtime/mod.rs)
is the first reading consumer. It emits a
`RuntimeV1BetaSupportExportRecord` that quotes the same closed tokens
visible to install / review and to the permission inspector, plus
scalar counts of declared / negotiated / narrowed capability worlds.
It never embeds raw manifests, raw signatures, raw policy bodies, raw
paths, raw tokens, or raw runtime payload bytes; the `blocks_activation`
flag is set whenever the contract is `refused` or `quarantined`.

The support export is what the partner packet template, the
support-export bundle, and any later CLI / headless review consumer
read. They join through `contract_ref` and quote
`admission_decision_class` / `admission_reason_class` verbatim
instead of inventing a local "stopped working" string.

## Fixtures

The checked-in fixtures replay every reserved decision class through
the support export bundle:

- [`wasm_in_process_admitted.json`](../../../fixtures/extensions/runtime_v1_beta_cases/wasm_in_process_admitted.json)
  — a capability-bounded Wasm component-model row, nominal lifecycle,
  every declared world admitted, SDK aligned.
- [`wasm_subprocess_admitted_narrowed.json`](../../../fixtures/extensions/runtime_v1_beta_cases/wasm_subprocess_admitted_narrowed.json)
  — a supervised Wasm subprocess where two declared worlds were
  narrowed by restricted-mode trust state; one_warm_restart_under_budget
  posture; recovered lifecycle.
- [`external_host_quarantined.json`](../../../fixtures/extensions/runtime_v1_beta_cases/external_host_quarantined.json)
  — a separately supervised external host process that tripped a
  crash-loop quarantine; runtime quarantines the row.
- [`anonymous_publisher_refused.json`](../../../fixtures/extensions/runtime_v1_beta_cases/anonymous_publisher_refused.json)
  — a row whose manifest install decision is denied with
  `publisher_anonymous`; the runtime refuses with
  `publisher_identity_opaque` as the spec's "no beta widening on
  opaque publisher identity" guardrail requires.

## Guardrails

The beta admission contract refuses to widen on:

- opaque publisher identity (manifest decision denied with
  `publisher_anonymous`, `publisher_identity_required`,
  `publisher_quarantined`, or `publisher_lifecycle_retired`);
- a missing declared-vs-effective diff report;
- a host placement / supervision class that is not reserved for the
  declared host contract family;
- a capability negotiation that admitted zero worlds or widened
  beyond declared worlds; and
- SDK / marketplace metadata that is not aligned with the runtime
  contract version.

These map directly to the spec's "no beta widening on opaque publisher
identity, missing diff reports, or unbounded host authority" rule.

## How to verify

```
cargo test -p aureline-extensions runtime::
```

The runtime tests replay every fixture above end-to-end through
`evaluate_runtime_v1_beta_contract`,
`validate_runtime_v1_beta_contract`, and
`project_runtime_v1_beta_support_export`, and exercise every refuse /
quarantine guardrail.
