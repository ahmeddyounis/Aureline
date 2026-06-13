# Approval-ticket issuance, deny reasons, and local-first verification

This document is the canonical contract for the M5 **approval-ticket ledger**:
the export-safe set of short-lived authority tickets actually minted for
mutating or privileged M5 actions, the deny-reason packets emitted when a ticket
cannot be honored, and the local-first verification descriptor that lets an
allowed local action be checked offline. Where the runtime-authority matrix
states *what posture* each executing surface requires and the capability-envelope
packet states the concrete authority issued for one execution, this ledger is
the **verb side** of that contract: the tickets, the denials, and the offline
proof. Desktop, command, policy, CLI/headless, diagnostics, support-export,
help/About, and release surfaces consume one ticket object instead of cloning
per-surface approval prose.

- Implementation: `crates/aureline-policy/src/implement_approval_ticket_issuance_deny_reason_packets_replay_nonce_or_expiry_enforcement_and_local_first_verification_f/`
- Boundary schema: `schemas/execution-auth/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.schema.json`
- Support export (truth source): `artifacts/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/support_export.json`
- Markdown summary: `artifacts/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f.md`
- Narrowed fixtures: `fixtures/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/`
- Producer / validator: `cargo run -p aureline-policy --example dump_m5_approval_ticket_ledger`

## Track invariant

No ambient privilege. No AI tool, recipe, extension, browser-routed, or remote
helper self-issues authority: every ticket carries an externally issued lineage
and is flagged `self_issued_by_executor: false` (`self_issued_authority_forbidden`
otherwise). Actor, action class, target identity, policy epoch, sandbox and
capability-envelope hash, expiry, and replay protection stay inspectable and
export-safe. Raw secret material, credential bodies, and live ticket signatures
stay outside the support boundary. If a ticket cannot be honored it **narrows or
fails closed with a named deny reason** instead of silently widening, and an
allowed local action verifies offline without depending on live control-plane
reachability.

## The seven binding dimensions

Every `M5ApprovalTicket` binds the dimensions an enforcer must check before
letting a mutating action run. A ticket that omits one fails validation:

| Dimension | Field | Failure token |
| --- | --- | --- |
| Actor | `actor` (`M5TicketActor`) | `ticket_incomplete` / `self_issued_authority_forbidden` |
| Action class | `action_class` (`M5TicketActionClass`) | `action_class_capability_unbound` |
| Target identity | `target` (`M5TicketTarget`) | `off_device_target_unverified` |
| Policy epoch | `binding.policy_epoch` | `binding_hash_missing` |
| Sandbox / capability hash | `binding.sandbox_profile`, `binding.capability_envelope_hash` | `binding_hash_missing` / `sandbox_profile_widens` |
| Expiry | `validity` (`M5TicketValidity`) | `validity_incomplete` |
| Replay protection | `replay_protection` (`M5ReplayProtection`) | `replay_protection_missing` |

Each action class pins exactly one capability class that must appear in
`binding.bound_capability_classes` (`action_class_capability_unbound` otherwise),
and the bound capabilities are always a subset of the matrix surface row
(`capability_widens_beyond_matrix` otherwise):

| Action class | Required capability | Representative surface |
| --- | --- | --- |
| `workspace_mutation` | `write_workspace` | `scaffold_hook` |
| `process_execution` | `process_spawn` | `notebook_kernel` |
| `network_send` | `network_egress` | `request_api_send` |
| `database_write` | `database_write` | `database_action` |
| `secret_projection` | `secret_handle_projection` | `ai_tool` |
| `remote_mutation` | `remote_mutation` | `remote_mutation` |
| `browser_routed_action` | `browser_navigation` | `browser_routed_action` |

The canonical ledger issues at least one currently-valid ticket for every action
class; a packet missing one fails validation (`required_action_class_missing`).

## Deny-reason packets

A denied or expired ticket does not collapse into a generic permission error: it
carries an `M5TicketDenyReason` that names the failed binding dimension and a
concrete recovery action. The deny dimension determines the verification state
the ticket must carry (`deny_dimension_state_mismatch` otherwise):

| Deny dimension | Verification state | Narrows to |
| --- | --- | --- |
| `expiry_elapsed` | `denied_expired` | `require_fresh_ticket` |
| `replay_nonce_consumed`, `replay_window_exceeded` | `denied_replay_detected` | `narrow_to_read_only` |
| `policy_epoch_superseded` | `denied_epoch_superseded` | `narrow_to_sanitized_preview` |
| `capability_hash_mismatch`, `sandbox_profile_mismatch`, `target_identity_mismatch`, `actor_binding_mismatch` | `denied_binding_mismatch` | `fail_closed_block` |
| `ticket_revoked` | `denied_revoked` | — |

A denied ticket without a deny reason fails (`deny_reason_missing`); a deny
reason without an explanation or recovery action fails
(`deny_explanation_missing` / `deny_recovery_missing`); a valid ticket carrying a
deny reason or downgrade trigger fails (`deny_reason_on_valid_ticket`).

## Replay-nonce-or-expiry enforcement

Every ticket carries an `M5ReplayProtection` block with a one-time `nonce`, a
`monotonic_counter`, and a non-zero `replay_window_seconds`. A consumed nonce
(`nonce_consumed: true`) is a replay attempt and denies the ticket with
`replay_nonce_consumed`. Expiry is enforced through `validity.expires_at` and a
non-zero `ttl_seconds`; an elapsed expiry denies with `expiry_elapsed`. The
nonce is an opaque export-safe token, never raw secret material.

## Local-first verification

Each ticket carries an `M5LocalFirstVerification` descriptor proving an allowed
local action verifies without depending on live control-plane reachability:

- `method` — `local_signature_chain` and `cached_policy_bundle` are
  offline-capable; `remote_broker_attestation` is used only off-device.
- `verifiable_offline` — true for an allowed local action.
- `requires_live_control_plane` — must be false for an allowed on-device action
  (`local_first_requires_control_plane` otherwise).
- `audit_lineage_preserved` — always true; offline verification never skips
  audit lineage (`audit_lineage_dropped` otherwise).
- `authority_widened_offline` — always false; offline verification never widens
  authority (`local_first_widens_authority` otherwise).

Off-device tickets (`remote_mutation`, `browser_routed_action`) preserve the
identical ticket shape and may verify through a remote broker attestation, but
still carry a verified target identity and full audit lineage.

## Consumers

`M5ApprovalTicketConsumerProjection` records that desktop, command palette,
policy inspector, CLI/headless, support export, diagnostics, help/About, and
release evidence consume this ledger directly. Downstream surfaces should read
the ticket objects rather than re-deriving approval or deny prose.

## Producing and validating

```sh
# Canonical support export (truth source)
cargo run -p aureline-policy --example dump_m5_approval_ticket_ledger

# Deterministic Markdown summary
cargo run -p aureline-policy --example dump_m5_approval_ticket_ledger -- markdown

# Narrowed fixtures
cargo run -p aureline-policy --example dump_m5_approval_ticket_ledger -- fixture all-valid
cargo run -p aureline-policy --example dump_m5_approval_ticket_ledger -- fixture expiry-replay
cargo run -p aureline-policy --example dump_m5_approval_ticket_ledger -- fixture epoch-binding

# Validate any packet
cargo run -p aureline-policy --example dump_m5_approval_ticket_ledger -- validate <packet.json>
```

The checked-in support export is the canonical truth source; a test asserts it
deserializes back to the frozen in-code packet unchanged. Regenerate it with the
dumper whenever the packet shape changes, alongside the schema and fixtures.
