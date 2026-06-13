# Add issue-use-revoke authority-lifecycle ledgers

This document is the canonical contract for the M5 **authority-lifecycle ledger**
packet: the export-safe, issue-use-revoke audit trail that threads each M5
authority grant through its whole life. The capability-envelope packet states the
authority one issued execution holds, and the approval-ticket ledger states the
short-lived ticket minted for one mutating action; this packet joins those
point-in-time grants into a single inspectable lineage per grant — when it was
**issued**, each time it was **used** and with what outcome, whether it was
**revoked**, and whether it was **invalidated** because the world drifted out
from under it. Desktop, command, policy, CLI/headless, diagnostics,
support-export, incident-review, and release surfaces consume one ledger object
instead of cloning per-surface lifecycle prose.

- Implementation: `crates/aureline-policy/src/add_issue_use_revoke_audit_ledgers_invalidation_on_target_or_trust_or_policy_or_sandbox_drift_and_support_export_safe_au/`
- Boundary schema: `schemas/execution-auth/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.schema.json`
- Support export (truth source): `artifacts/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/support_export.json`
- Markdown summary: `artifacts/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au.md`
- Fixtures: `fixtures/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/`
- Producer / validator: `cargo run -p aureline-policy --example dump_m5_authority_lifecycle_ledger`

## Track invariant

No ambient privilege. No grant confers ambient machine authority, and no AI tool,
recipe, extension, browser route, or remote helper self-issues authority: every
issue event carries an externally issued lineage flagged
`self_issued_by_executor: false` with a non-empty `decision_chain`. Secret
references are handle-only; raw secret material, credential bodies, and live
ticket signatures never cross the ledger boundary. When the target identity, the
trust anchor, the policy epoch, the network posture, or the sandbox profile
drifts away from what a grant was bound to, the grant **narrows or fails closed
on a named drift dimension** — recording the downgrade trigger, the narrowed
fallback, and a concrete recovery action — instead of silently widening or
collapsing into a generic permission error.

## Origin flows

Each ledger entry records the flow family the grant was minted inside, so
execution, repair, AI-assisted, provider-linked, and remote flows all land in one
ledger rather than per-surface spreadsheets:

| Origin flow | Meaning |
| --- | --- |
| `execution` | A direct execution flow (scaffold hook, notebook kernel, preview server, recipe). |
| `repair` | A repair or incident-response flow. |
| `ai_assisted` | An AI-assisted flow that invoked a tool on the operator's behalf. |
| `provider_linked` | A provider-linked flow (request/API send, database action, browser-routed action). |
| `remote` | A remote-execution flow brokered by another runtime. |

A packet missing any origin flow fails validation
(`required_origin_flow_missing`).

## What an entry carries

Each `M5AuthorityLedgerEntry` threads one authority grant through its lifecycle:

| Field | Meaning |
| --- | --- |
| `entry_id` | Stable id for this authority grant. |
| `surface` | The matrix executing surface the grant is bound to. |
| `action_class` | The mutating or privileged action class the grant authorizes. |
| `actor` | The actor class, an export-safe `actor_ref`, and any delegated `on_behalf_of`. |
| `target` | The export-safe target identity, whether it is off-device, and whether its identity is verified. |
| `linkage` | The cross-cutting join: `origin_flow`, optional `command_ref`, `session_ref`, `approval_ticket_ref`, `capability_envelope_ref`, and the envelope hash. |
| `issue` | The issue event: issuer class and ref, policy epoch, sandbox profile, secret scope, expiry, `decision_chain`, and `self_issued_by_executor: false`. |
| `uses` | The monotonic sequence of spend attempts, each with a timestamp, `sequence`, `outcome`, optional `narrowed_to` fallback, and a `note`. |
| `invalidation` | Present exactly when the grant is `invalidated`: the drifted dimension, the downgrade trigger, the narrowed fallback, an explanation, and a recovery action. |
| `revocation` | Present exactly when the grant is `revoked`: the revoking principal, the narrowed fallback, and a reason. |
| `lifecycle_state` | `issued`, `active`, `revoked`, `invalidated`, or `expired`. |
| `applied_downgrade_triggers` | The triggers applied to a terminated grant; empty while the grant is spendable. |

## Lifecycle coherence

The lifecycle state is kept coherent with the recorded events
(`lifecycle_state_incoherent` otherwise):

- `issued` — not yet used: `uses` is empty and there is no invalidation,
  revocation, or downgrade trigger.
- `active` — used at least once and still valid: `uses` is non-empty and there is
  no invalidation, revocation, or downgrade trigger
  (`spendable_entry_carries_termination` otherwise).
- `invalidated` — terminated by drift: carries an `invalidation`
  (`invalidation_missing` otherwise) and no revocation.
- `revoked` — terminated by an explicit revocation: carries a `revocation`
  (`revocation_missing` otherwise) and no invalidation.
- `expired` — terminated by its own time-to-live: carries neither an
  invalidation nor a revocation.

Every terminated grant carries the downgrade trigger for its termination
(`termination_trigger_missing` otherwise), and an invalidation's `trigger` must
match the trigger its drift dimension implies
(`invalidation_trigger_mismatch` otherwise). Use sequences increase strictly from
1 (`use_sequence_not_monotonic`), and a use outcome must be consistent with the
grant's recorded state: a `denied_invalidated` use requires an invalidation, a
`denied_revoked` use requires a revocation, and a `denied_expired` use requires
the `expired` state (`use_outcome_inconsistent` otherwise).

## Invalidation on drift

Invalidation fires on exactly one named drift dimension, each mapped to a
downgrade trigger and a default narrowed fallback:

| Drift dimension | Downgrade trigger | Narrows to |
| --- | --- | --- |
| `target_identity_drift` | `target_identity_unverified` | `require_fresh_ticket` |
| `trust_anchor_drift` | `ambient_privilege_detected` | `fail_closed_block` |
| `policy_epoch_drift` | `policy_epoch_superseded` | `narrow_to_sanitized_preview` |
| `network_posture_drift` | `upstream_dependency_narrowed` | `offline_local_core_only` |
| `sandbox_profile_drift` | `sandbox_profile_unavailable` | `fail_closed_block` |

A packet that never demonstrates one of these dimensions fails validation
(`required_drift_dimension_missing`). The `full_issue_use_revoke_ledger.json`
fixture exercises every drift dimension; `with_issued_grant_ledger.json` and
`with_expired_grant_ledger.json` add the `issued` and `expired` lifecycle states.

## Consumers

The packet is the single source of truth for the issue-use-revoke lifecycle. Its
`consumer_projection` block records that desktop, command/policy, CLI/headless,
support export, diagnostics, incident review, and release-evidence surfaces each
consume the same ledger entries, and that remote and browser-routed surfaces
preserve ledger semantics off-device. Downstream surfaces must ingest these
records instead of re-deriving per-surface lifecycle or invalidation prose.
