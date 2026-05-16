# Extension incident response beta

This governance note defines the minimum operating contract for extension
advisory, emergency-disable, quarantine, and revocation actions. It points
to the implementation contract in
[`docs/extensions/m3/revocation_and_emergency_disable_beta.md`](../extensions/m3/revocation_and_emergency_disable_beta.md).

## Operating Rules

- Every incident action must have a copy-safe incident ID and advisory ID.
- Forced disable, quarantine, and revocation actions must name blocked
  operations and recovery guidance before the action is considered
  actionable.
- Primary registry and mirror lanes must both expose a trust state. A mirror
  lane may be current, stale, pending import, continuity-broken, or failed by
  signature/digest mismatch, but it may not be left unknown.
- Support exports must preserve the packet's incident ID, advisory ID,
  lifecycle state, revocation state, blocked operations, and audit refs.
- Break-glass actors must be attributable through actor class, signer ref,
  policy refs, and audit event refs.

## Owner Coverage

| Area | Required coverage |
|---|---|
| Public registry emergency action | primary registry operator plus backup operator |
| Approved mirror propagation | mirror operator plus registry operator handoff |
| Security advisory publication | security responder plus release responder |
| Workspace or fleet policy action | workspace admin plus policy owner |
| Support export and incident packet review | supportability owner plus security reviewer |

## Required Rehearsal

Before widening a registry or mirror claim, exercise at least one packet in
each class:

| Class | Evidence |
|---|---|
| Emergency disable | forced-disable packet plus support export |
| Quarantine | mirror or runtime quarantine packet with explicit recovery guidance |
| Revocation | revoked artifact packet with last-known-good rollback or removal guidance |
| Mirror import lag | packet showing `pending_mirror_import` or a blocked mirror state |

The rehearsal passes only when product, CLI/headless, docs/help, support
export, and mirror import consumers can quote the same incident and
lifecycle identifiers.
