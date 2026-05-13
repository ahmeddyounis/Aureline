# Restricted-Mode Launch Wedge Alpha

This document describes the bounded alpha contract implemented in
`crates/aureline-auth/src/trust`. It covers launch-wedge workspace trust
gating: the packet emitted when a workspace opens or remains in restricted,
revoked, recovery, identity-gated, or policy-degraded posture.

Companion artifacts:

- `/schemas/trust/restricted_mode_launch_wedge.schema.json`
- `/fixtures/trust/restricted_mode_alpha/`
- `crates/aureline-auth/tests/restricted_mode_alpha.rs`

## Contract

The canonical record is `restricted_mode_launch_wedge_packet_record`.
It carries:

- workspace root ref and display scope, without raw paths;
- identity mode and binary trust state reused from the existing auth and
  workspace vocabularies;
- detailed trust state and entry transition;
- the trust source, remembered-decision scope, policy epoch, and source
  reason refs;
- one capability row per launch-wedge family;
- a receipt that keeps the trust gate visible after open.

## Required Behavior

Restricted posture keeps the local floor available:

- open and restore;
- read, edit, and save local files;
- local search and navigation;
- local Git inspection;
- policy summary read;
- redacted support export.

Restricted posture gates execution, mutation, install/update, provider,
remote, and AI apply capability. Blocked and review-needed rows must include:

- the authority class, such as `blocked_pending_trust`,
  `blocked_pending_approval`, `approval_required_per_invocation`, or
  `degraded_preview_only`;
- the source that caused the decision;
- the scope affected;
- recovery actions such as `request_trust_grant_session_only`,
  `request_approval_ticket`, `open_policy_details`, or
  `continue_restricted_no_elevation`.

## Guardrails

The packet validator rejects rows that violate the alpha guardrails:

- no silent trust widening;
- no hidden hosted, provider, network, or identity dependency;
- no plaintext secret fallback;
- no install/update mutation without review;
- no blocked row without source, scope, explanation, and recovery;
- no launch-only gate that disappears after the workspace opens.

## First Consumer

The Labs wedge inspector consumes `RestrictedModeAlphaPacket` and renders a
`restricted_mode_launch_wedge` row. That row is read-only and quotes the same
allowed-vs-blocked capability projection used by tests and fixtures.
