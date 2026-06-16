# M5 auth-callback and deep-link review

Generated from the seeded report in
[`crate::m5_callback_and_deep_link_review`](../../crates/aureline-auth/src/m5_callback_and_deep_link_review/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- report-md > \
  artifacts/platform/m5-auth-callback-and-deep-link.md
```

- Report id: `auth:m5_callback_and_deep_link_review:report:v1`
- Source schema ref: `schemas/platform/m5-deep-link-review.schema.json`
- Claimed platforms: `macos`, `windows`, `linux`
- Registered entries: `10`
- Marketed entries: `10`
- Confirm/reject parity entries: `10`
- Blocking findings: `0`
- Narrowable marketed entries: `0`
- Status: **clean**
- Generated at: `2026-06-16T00:00:00Z`

## Cross-links

| Upstream packet | Ref |
| --------------- | --- |
| `browser_handoff_ref` | `docs/auth/system_browser_callback_packet.md` |
| `embedded_boundary_ref` | `shell:embedded_boundary:v1` |
| `provider_origin_ref` | `docs/m5/embedded-boundaries-and-auth.md` |
| `auth_recovery_ref` | `artifacts/auth/m5_auth_and_recovery.md` |
| `system_entry_ref` | `artifacts/platform/m5-system-open-and-file-association.md` |
| `entry_interstitial_ref` | `shell:entry_interstitials:v1` |

## Per-entry-kind coverage

| Entry kind | Registered entries |
| ---------- | -----------------: |
| Auth-provider callback | 3 |
| Protocol deep link | 1 |
| Review handoff link | 2 |
| Collaboration join link | 1 |
| Managed resume link | 2 |
| Remote mutation link | 1 |

## Per-authority-scope coverage

| Authority scope | Entries | Gated behind confirm/reject |
| --------------- | ------: | --------------------------: |
| Plain local open | 1 | 0 |
| Crosses boundary (read-only) | 2 | 2 |
| Workspace collaboration join | 1 | 1 |
| Widens to managed authority | 5 | 5 |
| Widens to provider mutation | 1 | 1 |

## Disposition index

| Entry | Kind | Action | Authority scope | Outcome | Confirm/reject |
| ----- | ---- | ------ | --------------- | ------- | -------------- |
| `callback:auth_provider.system_browser` | Auth-provider callback | `resume_pending_sign_in` | `widens_to_managed_authority` | `admitted` | `true` |
| `callback:case.denied` | Review handoff link | `inspect_review_item` | `crosses_boundary_read_only` | `denied_by_policy` | `true` |
| `callback:case.expired` | Auth-provider callback | `resume_pending_sign_in` | `widens_to_managed_authority` | `denied_expired` | `true` |
| `callback:case.stale` | Managed resume link | `resume_managed_action` | `widens_to_managed_authority` | `denied_stale` | `true` |
| `callback:case.wrong_origin` | Auth-provider callback | `resume_pending_sign_in` | `widens_to_managed_authority` | `denied_wrong_origin` | `true` |
| `callback:collaboration_join.presence` | Collaboration join link | `join_collaboration` | `workspace_collaboration_join` | `admitted` | `true` |
| `callback:managed_resume.companion` | Managed resume link | `resume_managed_action` | `widens_to_managed_authority` | `admitted` | `true` |
| `callback:protocol_deep_link.open_local` | Protocol deep link | `open_existing_local_context` | `plain_local_open` | `admitted` | `false` |
| `callback:remote_mutation.provider` | Remote mutation link | `open_remote_mutation` | `widens_to_provider_mutation` | `admitted` | `true` |
| `callback:review_handoff.web_return` | Review handoff link | `inspect_review_item` | `crosses_boundary_read_only` | `admitted` | `true` |

## Findings summary

| Class | Count |
| ----- | ----: |
| _(none)_ | 0 |

## Per-entry rows

### `callback:auth_provider.system_browser` (auth_provider_callback via system_default_browser_return)

- Descriptor revision: `callback:auth_provider.system_browser:rev:2026.06.01-01`
- Disclosed origin: `origin:auth_provider.system_browser:disclosed` (`system_default_browser_return`)
- Origin assurance: `strict_origin_matched`
- Target identity: `target:auth_provider.system_browser:pending_sign_in`
- Requested action: `resume_pending_sign_in`
- Authority scope: `widens_to_managed_authority` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:auth.resume_pending_sign_in`
- Active profile owner: `profile-owner:callback:auth_provider.system_browser`
- Trust checkpoint: `trust:callback:auth_provider.system_browser:profile_tenant_policy`
- Pending correlation: `correlation:callback:auth_provider.system_browser:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `admitted`
- Recovery actions: _(none required)_
- Local continuity: `local_work_intact_managed_narrowed`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: A system-browser auth callback returns to the exact pending sign-in in the originating profile behind a confirm/reject sheet and never silently widens authority past what the sign-in requested.
- Degraded-state vocabulary:
  - Return to Aureline to finish signing in
  - Confirm this sign-in or stay signed out
  - Keep working locally without signing in

Findings: none.

### `callback:case.denied` (review_handoff_link via first_party_web_return)

- Descriptor revision: `callback:case.denied:rev:2026.06.01-01`
- Disclosed origin: `origin:case.denied:disclosed` (`first_party_web_return`)
- Origin assurance: `first_party_signed_link`
- Target identity: `target:case.denied:review_item`
- Requested action: `inspect_review_item`
- Authority scope: `crosses_boundary_read_only` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:review.open_handoff`
- Active profile owner: `profile-owner:callback:case.denied`
- Trust checkpoint: `trust:callback:case.denied:profile_tenant_policy`
- Pending correlation: `correlation:callback:case.denied:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `denied_by_policy`
- Recovery actions: `show_policy_block_detail`, `return_to_review_surface`, `keep_local_work_and_dismiss`
- Local continuity: `local_intent_preserved`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: A review link blocked by managed policy degrades to a policy-block detail with a return path and keeps local work, never a silent dead-end or an unscoped retry.
- Degraded-state vocabulary:
  - This link was blocked by policy
  - See why this was blocked
  - Return to the review surface

Findings: none.

### `callback:case.expired` (auth_provider_callback via system_default_browser_return)

- Descriptor revision: `callback:case.expired:rev:2026.06.01-01`
- Disclosed origin: `origin:case.expired:disclosed` (`system_default_browser_return`)
- Origin assurance: `strict_origin_matched`
- Target identity: `target:case.expired:pending_sign_in`
- Requested action: `resume_pending_sign_in`
- Authority scope: `widens_to_managed_authority` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:auth.resume_pending_sign_in`
- Active profile owner: `profile-owner:callback:case.expired`
- Trust checkpoint: `trust:callback:case.expired:profile_tenant_policy`
- Pending correlation: `correlation:callback:case.expired:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `denied_expired`
- Recovery actions: `request_fresh_link`, `retry_in_system_browser`, `continue_local_without_callback`
- Local continuity: `local_work_intact_managed_narrowed`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: An auth callback that arrives after its expiry says so plainly and offers a fresh sign-in link, never a silent no-op that looks like nothing happened.
- Degraded-state vocabulary:
  - This sign-in link has expired
  - Request a fresh sign-in link
  - Keep working locally without signing in

Findings: none.

### `callback:case.stale` (managed_resume_link via trusted_companion_app)

- Descriptor revision: `callback:case.stale:rev:2026.06.01-01`
- Disclosed origin: `origin:case.stale:disclosed` (`trusted_companion_app`)
- Origin assurance: `deep_link_scheme_pinned`
- Target identity: `target:case.stale:managed_action`
- Requested action: `resume_managed_action`
- Authority scope: `widens_to_managed_authority` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:managed.resume_action`
- Active profile owner: `profile-owner:callback:case.stale`
- Trust checkpoint: `trust:callback:case.stale:profile_tenant_policy`
- Pending correlation: `correlation:callback:case.stale:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `denied_stale`
- Recovery actions: `return_to_pending_sign_in`, `retry_in_system_browser`, `continue_local_without_callback`
- Local continuity: `local_work_intact_managed_narrowed`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: A managed-resume link whose pending session was superseded surfaces the stale state explicitly and offers a fresh resume, never silently replaying an outdated action.
- Degraded-state vocabulary:
  - This session was replaced by a newer one
  - Resume from the current session
  - Keep working locally without resuming

Findings: none.

### `callback:case.wrong_origin` (auth_provider_callback via external_provider)

- Descriptor revision: `callback:case.wrong_origin:rev:2026.06.01-01`
- Disclosed origin: `origin:case.wrong_origin:disclosed` (`external_provider`)
- Origin assurance: `origin_unverified`
- Target identity: `target:case.wrong_origin:pending_sign_in`
- Requested action: `resume_pending_sign_in`
- Authority scope: `widens_to_managed_authority` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:auth.resume_pending_sign_in`
- Active profile owner: `profile-owner:callback:case.wrong_origin`
- Trust checkpoint: `trust:callback:case.wrong_origin:profile_tenant_policy`
- Pending correlation: `correlation:callback:case.wrong_origin:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `denied_wrong_origin`
- Recovery actions: `show_origin_mismatch_detail`, `retry_in_system_browser`, `continue_local_without_callback`
- Local continuity: `local_work_intact_managed_narrowed`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: An auth callback whose origin does not match the pending handoff is named as an origin mismatch with a detail view, never an arbitrary auth failure, and is denied before any authority widens.
- Degraded-state vocabulary:
  - This sign-in came from an origin we could not verify
  - See why this origin was rejected
  - Retry the sign-in in your browser

Findings: none.

### `callback:collaboration_join.presence` (collaboration_join_link via collaboration_service)

- Descriptor revision: `callback:collaboration_join.presence:rev:2026.06.01-01`
- Disclosed origin: `origin:collaboration_join.presence:disclosed` (`collaboration_service`)
- Origin assurance: `first_party_signed_link`
- Target identity: `target:collaboration_join.presence:session`
- Requested action: `join_collaboration`
- Authority scope: `workspace_collaboration_join` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:collab.join_session`
- Active profile owner: `profile-owner:callback:collaboration_join.presence`
- Trust checkpoint: `trust:callback:collaboration_join.presence:profile_tenant_policy`
- Pending correlation: `correlation:callback:collaboration_join.presence:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `admitted`
- Recovery actions: _(none required)_
- Local continuity: `local_work_intact_managed_narrowed`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: A collaboration join link joins shared presence only after an explicit confirm/reject sheet discloses who is hosting and what is shared, and never auto-joins on open.
- Degraded-state vocabulary:
  - Join this collaboration session
  - This session is no longer active
  - Keep working locally without joining

Findings: none.

### `callback:managed_resume.companion` (managed_resume_link via trusted_companion_app)

- Descriptor revision: `callback:managed_resume.companion:rev:2026.06.01-01`
- Disclosed origin: `origin:managed_resume.companion:disclosed` (`trusted_companion_app`)
- Origin assurance: `deep_link_scheme_pinned`
- Target identity: `target:managed_resume.companion:managed_action`
- Requested action: `resume_managed_action`
- Authority scope: `widens_to_managed_authority` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:managed.resume_action`
- Active profile owner: `profile-owner:callback:managed_resume.companion`
- Trust checkpoint: `trust:callback:managed_resume.companion:profile_tenant_policy`
- Pending correlation: `correlation:callback:managed_resume.companion:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `admitted`
- Recovery actions: _(none required)_
- Local continuity: `local_work_intact_managed_narrowed`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: A managed-resume link from a trusted companion resumes a managed action only behind a confirm/reject sheet that names the action and its scope, and never widens authority silently.
- Degraded-state vocabulary:
  - Resume this managed action
  - This managed action is no longer available
  - Keep working locally without resuming

Findings: none.

### `callback:protocol_deep_link.open_local` (protocol_deep_link via registered_protocol_handler)

- Descriptor revision: `callback:protocol_deep_link.open_local:rev:2026.06.01-01`
- Disclosed origin: `origin:protocol_deep_link.open_local:disclosed` (`registered_protocol_handler`)
- Origin assurance: `deep_link_scheme_pinned`
- Target identity: `target:protocol_deep_link.open_local:local_context`
- Requested action: `open_existing_local_context`
- Authority scope: `plain_local_open` (widens authority: `false`)
- Confirm/reject required: `false` (reuses in-product path: `true`)
- Canonical command: `cmd:workspace.open.target`
- Active profile owner: `profile-owner:callback:protocol_deep_link.open_local`
- Trust checkpoint: `trust:callback:protocol_deep_link.open_local:profile_tenant_policy`
- Pending correlation: `correlation:callback:protocol_deep_link.open_local:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `admitted`
- Recovery actions: _(none required)_
- Local continuity: `local_intent_preserved`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: A protocol deep link that resolves to an already-trusted local context opens directly with no authority widening and therefore no confirm/reject sheet.
- Degraded-state vocabulary:
  - Open this item
  - This item is no longer available
  - Choose a different item

Findings: none.

### `callback:remote_mutation.provider` (remote_mutation_link via external_provider)

- Descriptor revision: `callback:remote_mutation.provider:rev:2026.06.01-01`
- Disclosed origin: `origin:remote_mutation.provider:disclosed` (`external_provider`)
- Origin assurance: `strict_origin_matched`
- Target identity: `target:remote_mutation.provider:remote_resource`
- Requested action: `open_remote_mutation`
- Authority scope: `widens_to_provider_mutation` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:provider.open_remote_mutation`
- Active profile owner: `profile-owner:callback:remote_mutation.provider`
- Trust checkpoint: `trust:callback:remote_mutation.provider:profile_tenant_policy`
- Pending correlation: `correlation:callback:remote_mutation.provider:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `admitted`
- Recovery actions: _(none required)_
- Local continuity: `local_work_intact_managed_narrowed`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: A provider link that would mutate remote state always shows a confirm/reject sheet disclosing the exact mutation before any write, and never auto-commits from an open.
- Degraded-state vocabulary:
  - Confirm this remote change before it is applied
  - This remote target is unreachable
  - Keep working locally without the remote change

Findings: none.

### `callback:review_handoff.web_return` (review_handoff_link via first_party_web_return)

- Descriptor revision: `callback:review_handoff.web_return:rev:2026.06.01-01`
- Disclosed origin: `origin:review_handoff.web_return:disclosed` (`first_party_web_return`)
- Origin assurance: `first_party_signed_link`
- Target identity: `target:review_handoff.web_return:review_item`
- Requested action: `inspect_review_item`
- Authority scope: `crosses_boundary_read_only` (widens authority: `true`)
- Confirm/reject required: `true` (reuses in-product path: `true`)
- Canonical command: `cmd:review.open_handoff`
- Active profile owner: `profile-owner:callback:review_handoff.web_return`
- Trust checkpoint: `trust:callback:review_handoff.web_return:profile_tenant_policy`
- Pending correlation: `correlation:callback:review_handoff.web_return:state_nonce_pkce`
- Expiry: `2026-06-16T00:10:00Z`
- Outcome: `admitted`
- Recovery actions: _(none required)_
- Local continuity: `local_intent_preserved`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:callback_review:narrow_on_stale_evidence`
- Redaction-safe: `yes`
- Marketed: `yes`
- Continuity note: A review handoff link opens the review surface inspect-only behind a confirm/reject sheet because it crosses a boundary, and is never coerced into a mutating action.
- Degraded-state vocabulary:
  - Review this item without making changes
  - This review link points to an item you cannot access
  - This review link has expired

Findings: none.

## Verification

```sh
cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- validate
cargo test -p aureline-auth --test m5_callback_and_deep_link_review_fixtures
python3 tools/ci/m5/callback_and_deep_link_check.py
```
