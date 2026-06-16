# Auth-callback and deep-link review

Aureline's local-first promise has to survive the moment a flow *returns from
outside the product*. The system browser hands back an auth callback, a protocol
handler delivers a deep link, a review or collaboration link reopens the app, or
a companion resumes a managed action — and each of those is a moment where a
wrong origin, an expired link, or a silently widened authority could ride in
unreviewed. If that return is implicit, a callback quietly becomes a trust or
scope escalation path: a sign-in return silently joins a collaboration, a deep
link silently opens a remote mutation, or a wrong-origin return looks like an
arbitrary auth failure rather than the spoof-suspect it is.

This document describes the typed review layer that makes every callback and
deep-link return explicit and reviewable. Every return is projected as one typed
entry that discloses *who asked*, *what scope they requested*, *why the user is
asked to confirm or reject*, and *how local intent survives a failure* — and
routes its confirm action through the *same* in-product command the equivalent
in-product action runs.

This layer extends the existing browser-handoff, embedded-boundary,
provider-origin, and auth-recovery rows
(`docs/auth/system_browser_callback_packet.md`,
`docs/m5/embedded-boundaries-and-auth.md`,
`artifacts/auth/m5_auth_and_recovery.md`): those packets own the outbound
handoff, the embedded boundary chrome, and the recovery vocabulary; this layer
governs what happens once a return arrives. It sits beside the system-entry
intake matrix (`artifacts/platform/m5-system-open-and-file-association.md`),
which governs OS-initiated *opens*; this layer governs callback and deep-link
*returns*.

## Canonical objects

| Object | Path |
| ------ | ---- |
| Typed consumer | `crates/aureline-auth/src/m5_callback_and_deep_link_review/mod.rs` |
| Headless inspector | `crates/aureline-auth/src/bin/aureline_auth_m5_callback_and_deep_link_review.rs` |
| Boundary schema | `schemas/platform/m5-deep-link-review.schema.json` |
| Report fixture | `fixtures/platform/m5-callback-and-deep-link/report.json` |
| Support-export fixture | `fixtures/platform/m5-callback-and-deep-link/support_export.json` |
| Compact fixture | `fixtures/platform/m5-callback-and-deep-link/compact.txt` |
| Case-export fixtures | `fixtures/platform/m5-callback-and-deep-link/cases/*.json` |
| Published report | `artifacts/platform/m5-auth-callback-and-deep-link.md` |
| CI gate | `tools/ci/m5/callback_and_deep_link_check.py` |

The headless inspector is the only mint-from-truth path. The report fixture and
the published report at `artifacts/platform/m5-auth-callback-and-deep-link.md`
are asserted bit-for-bit equal to the seeded report by
`crates/aureline-auth/tests/m5_callback_and_deep_link_review_fixtures.rs`.

## Track invariant

A callback or deep-link return never bypasses trust, profile, tenant, or policy
evaluation; the origin is disclosed and spoof-resistantly verified before any
authority widens; anything wider than a plain local open is gated behind an
explicit confirm/reject sheet; and a denied, wrong-origin, expired, or stale
return preserves local intent through a truthful placeholder or recovery sheet
rather than an empty shell.

## Entry kinds

Every return is one of the six required entry kinds. The review layer routes all
of them through a single typed path rather than a per-affordance shortcut.

- `auth_provider_callback` — a browser auth callback returning to a pending
  sign-in.
- `protocol_deep_link` — a protocol / deep-link scheme open of an existing local
  context.
- `review_handoff_link` — a review or work-item deep link routed to the review
  surface.
- `collaboration_join_link` — a collaboration join link that joins presence in a
  workspace.
- `managed_resume_link` — a managed-action resume link routed through a trusted
  companion.
- `remote_mutation_link` — a provider link that would open a remote mutation.

## Who asked: origin disclosure and spoof resistance

Each entry discloses *who asked* on two axes:

- `source_class` — the source that delivered the return
  (`system_default_browser_return`, `registered_protocol_handler`,
  `first_party_web_return`, `trusted_companion_app`, `external_provider`,
  `collaboration_service`, `unknown_untrusted`).
- `origin_assurance` — how the origin was verified
  (`strict_origin_matched`, `loopback_port_pinned`, `deep_link_scheme_pinned`,
  `device_code_poll_matched`, `first_party_signed_link`, `origin_unverified`).

The `disclosed_origin_ref` is an export-safe ref, never a raw URL. An admitted
return whose `origin_assurance` is `origin_unverified` is an
`origin_verification_bypassed` blocker, so spoof-resistance can never be skipped
for convenience.

## What scope they requested

Each entry names the requested action (`requested_action`) and the authority it
would reach (`authority_scope`), plus the workspace and tenant scope refs and
whether the authority is broader than a plain local open (`widens_authority`):

- `plain_local_open` — an exact, local, already-trusted open. The fast path; no
  confirm/reject.
- `crosses_boundary_read_only` — crosses a network, review, or tenant boundary to
  inspect a remote target (still read-only).
- `workspace_collaboration_join` — joins workspace collaboration / presence.
- `widens_to_managed_authority` — widens to managed authority (a managed sign-in
  or managed-action resume).
- `widens_to_provider_mutation` — would trigger a mutating provider-side flow.

The requested action and the authority scope must agree; a widening action that
claims a `plain_local_open` scope breaks confirm/reject parity.

## Why confirm or reject

Every authority class other than `plain_local_open` requires an explicit
confirm/reject sheet (`confirm_reject_sheet_ref`) before it commits. An
auto-admit that widens authority without one is a `silent_authority_widen`
blocker; one that opens a remote mutation without one is a distinct
`silent_remote_mutation` blocker. The two never collapse into a single finding,
so "sign-in return → join collaboration" and "deep link → remote mutation" stay
separate failure classes. The confirm action always routes to a
`canonical_command_ref` — the same command the in-product action runs — so a
callback can never grant more authority than the in-product path.

## Outcome and recovery

The disposition of the return at review time is one of `admitted`,
`denied_wrong_origin`, `denied_expired`, `denied_stale`, or `denied_by_policy`.
Any denied outcome MUST offer at least one recovery action, and each denial class
stays a distinct failure so it never looks like an arbitrary auth failure or a
silent no-op:

- a `denied_wrong_origin` with no recovery is a
  `wrong_origin_looks_like_auth_failure` blocker;
- a `denied_expired` with no recovery is an `expired_silent_no_op` blocker;
- a `denied_stale` with no recovery is a `stale_state_unsurfaced` blocker; and
- a `denied_by_policy` with no recovery is a `policy_denial_dead_end` blocker.

Recovery actions are bounded (`retry_in_system_browser`,
`continue_local_without_callback`, `return_to_pending_sign_in`,
`show_origin_mismatch_detail`, `return_to_review_surface`, `request_fresh_link`,
`show_policy_block_detail`, `keep_local_work_and_dismiss`).

## Local continuity

Each entry records how local work and intent survive the return
(`local_continuity`): `local_intent_preserved`,
`local_work_intact_managed_narrowed`, or `local_continuity_at_risk`. A return
that puts local intent at risk is a `local_continuity_lost` blocker — a failed
return lands on a truthful placeholder or recovery sheet, never an empty shell
that discards the original intent.

## Incident case exports

The four required incident classes are published as standalone case-export
packets under `fixtures/platform/m5-callback-and-deep-link/cases/`, so support
can reproduce each from typed diagnostics instead of a screenshot:

- `wrong_origin.json` — an auth callback from an unverified external origin,
  named as an origin mismatch with `show_origin_mismatch_detail`.
- `expired.json` — an auth callback that arrived after its expiry, recovered with
  `request_fresh_link`.
- `stale.json` — a managed-resume link whose pending session was superseded,
  recovered with `return_to_pending_sign_in`.
- `denied.json` — a review handoff link blocked by managed policy, degraded to a
  policy-block detail with a return path.

## Other invariants

- Every entry names a `disclosed_origin_ref`, a `target_identity_ref`, a
  `pending_correlation_ref` (an alias for the state / nonce / PKCE correlation,
  never a raw token), an `expiry_at`, an `active_profile_owner_ref`, and a
  `trust_checkpoint_ref`; a missing trust checkpoint is a
  `trust_evaluation_bypassed` blocker.
- Every entry carries only redaction-safe refs; an entry with
  `redaction_safe = false` is a `raw_target_leak` blocker, so packets never ship
  a raw URL or provider token to an end-user surface.
- Stale evidence on a marketed entry is a blocker so release tooling can narrow
  the surface instead of shipping it as implicitly stable.
- The report cross-links the browser-handoff, embedded-boundary, provider-origin,
  auth-recovery, system-entry, and entry-interstitial packets so trust
  vocabulary cannot drift independently.

## Verification

```sh
cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- validate
cargo test -p aureline-auth --test m5_callback_and_deep_link_review_fixtures
python3 tools/ci/m5/callback_and_deep_link_check.py
```

Regenerate the fixtures and the published report from the seed:

```sh
cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- report \
  > fixtures/platform/m5-callback-and-deep-link/report.json
cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- support-export \
  > fixtures/platform/m5-callback-and-deep-link/support_export.json
cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- compact \
  > fixtures/platform/m5-callback-and-deep-link/compact.txt
cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- report-md \
  > artifacts/platform/m5-auth-callback-and-deep-link.md
```
