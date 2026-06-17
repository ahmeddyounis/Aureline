# Store-lock and external-root recovery

Aureline's local-first, native-desktop promise has to survive the everyday
moment an *OS-backed store locks* or an *external root disappears*: the OS
credential store is locked or unreachable, the trust / certificate store drifts
out from under a remembered decision, a removable volume is ejected, a network
share disconnects, or an external root simply goes missing. These are real,
recoverable failure modes — not support-only edge cases. Native-desktop
credibility depends on degrading to a clear, recoverable state rather than a
silent disappearance or a generic downstream error.

This document describes the typed recovery layer that makes every such moment an
explicit, reviewable state. Each incident is projected as one typed recovery
state that names the unavailable resource, says what is paused and what remains
local-only, retains a truthful placeholder, offers precise repair guidance, and
binds every running session, queued job, and remembered decision to an explicit
resume — never a silent one — so a store unlock or a root return never widens or
replays work on its own.

The layer reuses the credential-state / secret-broker, trust-store, and
filesystem-identity objects rather than maintaining a parallel notion of store,
trust, or path, and rides alongside the deferred-intent and auth-recovery
packets. It hardens the OS-store and disappearing-root failure surfaces those
rows still leave too implicit.

## Canonical objects

| Object | Path |
| ------ | ---- |
| Typed consumer | `crates/aureline-auth/src/m5_store_lock_and_external_root_recovery/mod.rs` |
| Headless inspector | `crates/aureline-auth/src/bin/aureline_auth_m5_store_lock_and_external_root_recovery.rs` |
| Boundary schema | `schemas/platform/m5-store-lock-and-missing-root.schema.json` |
| Report fixture | `fixtures/platform/m5-store-lock-and-missing-root/report.json` |
| Support-export fixture | `fixtures/platform/m5-store-lock-and-missing-root/support_export.json` |
| Compact fixture | `fixtures/platform/m5-store-lock-and-missing-root/compact.txt` |
| Case-export fixtures | `fixtures/platform/m5-store-lock-and-missing-root/cases/*.json` |
| Published report | `artifacts/platform/m5-store-lock-and-external-root-recovery.md` |
| CI gate | `tools/ci/m5/store_lock_and_external_root_check.py` |

The headless inspector is the only mint-from-truth path. The report fixture and
the published report at
`artifacts/platform/m5-store-lock-and-external-root-recovery.md` are asserted
bit-for-bit equal to the seeded report by
`crates/aureline-auth/tests/m5_store_lock_and_external_root_recovery_fixtures.rs`.

## Incident kinds

Every incident is one of seven support-distinguishable kinds, all projected
through a single typed path:

- `credential_store_locked` — the OS credential store is locked and needs
  unlocking.
- `credential_store_unavailable` — the OS credential-store backend is
  unreachable or absent.
- `trust_store_drift` — the trust / certificate store drifted from a remembered
  decision.
- `removable_volume_missing` — a removable volume was ejected or removed.
- `network_share_missing` — a network share disconnected.
- `external_root_missing` — an external root went missing.
- `root_returned` — a previously missing root has returned and awaits explicit
  resume.

Support packets can tell a store lock from a trust-store drift, a missing root,
and a returned root apart with these kinds plus the per-incident case exports,
without manual log forensics.

## Resource, degradation, and what is paused

Each state names the unavailable resource with `resource_class`
(`credential_store`, `trust_store`, `removable_volume`, `network_share`,
`external_root`) and the typed degradation with `degraded_state_class`
(`store_locked`, `store_unavailable`, `trust_store_drifted`, `root_missing`,
`root_returned`).

A degraded state declares what stopped and what kept working:

- `paused_capabilities` — what is paused while the resource is unavailable
  (provider authentication, a signing/publishing operation, certificate
  validation, external-root file access, or managed sync). An active degradation
  that discloses nothing paused is a `missing_paused_disclosure` blocker.
- `local_only_capabilities` — what remains local-only (local editing, local
  history, local export, offline core tooling, browsing the cached context of
  the missing root). Every state MUST disclose at least one.

`local_continuity_preserved` MUST be `true`: local user-owned work stays intact
and visibly recoverable through the incident, otherwise it is a
`local_work_not_preserved` blocker.

## Truthful placeholders and unsaved local state

Every state retains a truthful placeholder so an incident never degrades to
nothing:

- `last_seen_identity_ref` — the last-seen identity of the store or root the
  placeholder names (an export-safe ref, never a raw path or secret body). A
  missing placeholder is a `silent_disappearance` blocker.
- `unsaved_local_state_posture` — `preserved_in_place`, `none_pending`, or
  `preserved_pending_recovery`, so the user can see their unsaved work is kept.

## Recovery actions and repair guidance

Each active degradation offers typed `recovery_actions` and a
`repair_guidance_ref`. The vocabulary is `unlock_store`, `repair_store`,
`retry_after_unlock`, `review_trust_change`, `re_evaluate_trust`,
`reconnect_network_share`, `remount_volume`, `locate_root`,
`open_cached_context`, `close_placeholder`, and `confirm_explicit_resume`. A
missing root in particular offers the Locate / Open cached context / Close
actions. None of these ever imply writing a secret to plaintext.

Recovery is required for an active degradation, and each store/trust/root family
stays a distinct failure:

- a `credential_store_locked` or `credential_store_unavailable` with no recovery
  is a `credential_store_lock_unrecoverable` blocker;
- a `trust_store_drift` with no recovery is the distinct
  `trust_store_drift_unrecoverable` blocker; and
- a `removable_volume_missing`, `network_share_missing`, or
  `external_root_missing` with no recovery is the distinct
  `missing_root_unrecoverable` blocker.

## No plaintext fallback

`implies_plaintext_fallback` MUST be `false`. A recovery path that implies a
plaintext-secret fallback is a `plaintext_fallback_implied` blocker. This is a
hard guardrail: a locked or unavailable secure store never degrades to a
plaintext-file credential.

## No silent resume on recovery

No running session, queued job, or remembered decision is silently widened or
re-run after a store unlocks or a root returns:

- `resume_posture` is `explicit_resume_required` or `not_applicable`, never
  silent.
- `resumes_silently_on_recovery` MUST be `false`.
- every `protected_continuations` entry carries a `continuation_class`
  (`running_session`, `queued_job`, `remembered_decision`) and a
  `resume_disposition` of `explicit_resume_required` or `held_for_review`, never
  `silent_resume`.
- a `root_returned` state MUST require explicit resume.

A silent resume posture, a silent continuation disposition, or a returned root
that does not require explicit resume is a `silent_resume_on_recovery` blocker. A
returned root never auto-rejoins a session or replays a deferred write just
because the resource came back.

## Surface parity and trust evaluation

Every state declares `surface_parity` including `desktop`, `cli_headless`, and
`support` so a store-lock or missing-root incident carries the same vocabulary in
the desktop product, the CLI / headless flows, and support export; an incomplete
parity is a `surface_parity_incomplete` blocker. Every state also names an
`active_profile_owner_ref`, a `trust_checkpoint_ref`, and a
`canonical_command_ref` — the same command the in-product path runs — so OS-store
recovery never bypasses trust / profile / policy evaluation; a missing trust
checkpoint is a `trust_evaluation_bypassed` blocker.

## Incident case exports

The four required incident families are published as standalone case-export
packets under `fixtures/platform/m5-store-lock-and-missing-root/cases/`, so
support can reproduce each from typed diagnostics instead of a screenshot:

- `credential_store_locked.json` — the OS credential store is locked; provider
  auth and managed sync are paused while local work continues, recovery is to
  unlock or repair, and held work requires explicit resume.
- `trust_store_drift.json` — the trust store drifted; certificate validation is
  paused, the remembered trust acceptance is held for review, and recovery is to
  review and re-evaluate.
- `missing_root.json` — an external root went missing; the placeholder names the
  last-seen identity, unsaved edits are preserved, and recovery offers Locate /
  Open cached context / Close.
- `root_returned.json` — a previously missing network share returned; nothing
  resumes on its own and the held write, bound session, and remembered decision
  all require explicit confirmation.

## Other invariants

- Every state cross-links the credential-store, trust-store, filesystem-identity,
  deferred-intent, auth-recovery, and Help/About surfaces so store, trust, path,
  and continuity vocabulary cannot drift independently.
- Stale evidence on a marketed state is a `stale_evidence_on_marketed_state`
  blocker so release tooling can narrow the surface instead of shipping it as
  implicitly stable.
- Every state rides the governed recovery harness
  (`registered_on_recovery_harness = true`); a state off the harness is a
  `state_not_on_harness` blocker.

## Verification

```sh
cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- validate
cargo test -p aureline-auth --test m5_store_lock_and_external_root_recovery_fixtures
python3 tools/ci/m5/store_lock_and_external_root_check.py
```

Regenerate the fixtures and the published report from the seed:

```sh
cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- report \
  > fixtures/platform/m5-store-lock-and-missing-root/report.json
cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- support-export \
  > fixtures/platform/m5-store-lock-and-missing-root/support_export.json
cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- compact \
  > fixtures/platform/m5-store-lock-and-missing-root/compact.txt
cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- report-md \
  > artifacts/platform/m5-store-lock-and-external-root-recovery.md
```
