# System-open and file-association intake

Aureline's local-first promise has to survive the moment an open *starts
outside the product*. The OS hands the app a file, a folder, a workspace
manifest, a review or work-item deep link, a patch bundle, or a browser auth
return, and the product has to decide what that target really is and what mode
will result *before* it commits to anything broader than a plain local read. If
that decision is implicit, an OS handoff quietly becomes a trust or scope
escalation path: an `open file` silently becomes an `open workspace`, a
`review handoff` silently becomes a mutating provider flow, or a wrong target
opens as if it were the thing the user expected.

This document describes the typed intake layer that makes that decision explicit
and reviewable. Every OS-initiated open is projected as one typed intake that
preserves what the OS handed Aureline, what Aureline thinks the target really
is, and what mode will result — and resolves it through the *same* reviewed
project-entry path the in-product Open/Clone/Import/Restore/Resume flows use.

The intake layer rides on top of the native-desktop matrix
(`docs/m5/native-desktop-integration-and-reopen.md`): that matrix governs which
OS affordance owns a handler and how a reopen finds its target; this layer
governs what happens once a surface delivers a target.

## Canonical objects

| Object | Path |
| ------ | ---- |
| Typed consumer | `crates/aureline-shell/src/m5_system_entry/mod.rs` |
| Headless inspector | `crates/aureline-shell/src/bin/aureline_shell_m5_system_entry.rs` |
| Boundary schema | `schemas/platform/m5-system-entry.schema.json` |
| Report fixture | `fixtures/platform/m5-system-entry/report.json` |
| Support-export fixture | `fixtures/platform/m5-system-entry/support_export.json` |
| Compact fixture | `fixtures/platform/m5-system-entry/compact.txt` |
| Case-export fixtures | `fixtures/platform/m5-system-entry/cases/*.json` |
| Published report | `artifacts/platform/m5-system-open-and-file-association.md` |
| CI gate | `tools/ci/m5/system_entry_check.py` |

The headless inspector is the only mint-from-truth path. The report fixture and
the published report at `artifacts/platform/m5-system-open-and-file-association.md`
are asserted bit-for-bit equal to the seeded report by
`crates/aureline-shell/tests/m5_system_entry_fixtures.rs`.

## Track invariant

OS-level entry never bypasses trust, profile, tenant, or policy evaluation;
channel/build ownership is inspectable; and the literal target, the canonical
target, and the resulting mode are all visible before the product commits to a
broader open path. Anything wider than a plain local read is gated behind an
explicit interstitial, and an unavailable, wrong, or policy-blocked target
preserves context through truthful placeholders and recovery actions.

## Intake kinds

Every OS-initiated open is one of the six required intake kinds. The intake
layer routes all of them through a single typed path rather than a per-affordance
shortcut.

- `file` — a single file handed over for a plain local open.
- `folder` — a folder handed over for a local open or add-root.
- `workspace` — a workspace manifest handed over for a multi-root open.
- `review_link` — a review or work-item deep link routed to the review surface.
- `patch_bundle` — a patch or portable-state bundle routed to the import flow.
- `provider_return` — a browser auth callback returning to a pending sign-in.

The OS affordance that delivered the intake (`system_open`, `file_association`,
`protocol_handler`, `auth_callback`, `recent_item`, `dock_taskbar_jumplist`) is
tracked separately, so a wrong-association incident on a file-association open is
a distinct diagnostic from the same kind arriving through a recent-item reopen.

## Literal vs canonical target

Each intake preserves two target identities so the user can see what the OS
handed over against what Aureline detected:

- `literal_target_ref` — an export-safe captured ref for the literal target the
  OS handed over, plus a `literal_format` shape hint
  (`windows_drive_path`, `windows_unc_path`, `posix_path`, `file_uri`,
  `deep_link_uri`, `provider_callback`, `unknown`). It is never a raw path or
  secret body; user-visible surfaces render the literal locally.
- `canonical_target_ref` — the canonical identity Aureline detected the literal
  to be, classified into the shared `detected_target_kind`.

A wrong association, a moved target, or a normalization variant can never
masquerade as the thing the user expected, because the two identities are
recorded side by side.

## Resulting-mode parity

The intended verb and resulting mode are expressed in the shared project-entry
vocabulary the in-product flows already use — `intended_entry_verb` (`open`,
`clone`, `import`, `add_root`, `restore`, `resume`, `start_from_snapshot`) and
`intended_resulting_mode` (for example `single_file`, `folder`,
`workspace_with_roots`, `inspect_only`, `extract_then_review`,
`resume_live_session`). The OS path never invents its own vocabulary.

Each intake declares a `parity_class`:

- `entry_flow_resolved` — the intake resolves through the canonical
  `aureline_workspace::resolve_entry_flow` resolver, and its intended verb and
  mode MUST equal the resolver's output. A divergence is a `verb_coercion`
  blocker, so an OS open can never coerce one verb into another.
- `routed_to_review_surface` — a review-link opens the review surface
  inspect-only and names that reviewed surface (it is never coerced into a
  mutating provider action).
- `routed_to_auth_recovery` — a provider return routes to the auth-recovery
  surface and names it.

## Scope discipline

Each intake declares the authority its auto-open would reach as a `scope_class`:

- `plain_local_read` — an exact, local, already-trusted read. The fast path; no
  interstitial.
- `widens_to_workspace_scope` — promotes a single-file or folder open into
  workspace / multi-root scope.
- `crosses_boundary` — crosses a network, review, or tenant boundary to inspect
  a remote target (still read-only).
- `widens_to_provider_mutation` — would trigger a mutating provider-side flow.
- `requires_trust_decision` — targets an untrusted root and requires an explicit
  trust decision.

Every class other than `plain_local_read` requires an explicit interstitial. An
auto-open that widens to workspace scope without one is a `silent_scope_widen`
blocker; one that widens to a mutating provider flow without one is a distinct
`silent_provider_mutation` blocker. The two never collapse into a single
finding, so "open file → open workspace" and "review handoff → mutating provider
flow" stay separate failure classes.

## Availability and recovery

The canonical target's availability at intake time is one of `exact_available`,
`wrong_association`, `moved_target`, `mixed_root`, `blocked_by_policy`, or
`missing_or_unmounted`. Any value other than `exact_available` MUST offer at
least one recovery action, and each unavailable class stays a distinct failure:

- a `wrong_association` or `moved_target` with no recovery is a
  `wrong_target_no_recovery` blocker;
- a `mixed_root` or `missing_or_unmounted` target with no recovery is an
  `unavailable_path_silent_loss` blocker; and
- a `blocked_by_policy` target with no recovery is a `policy_block_unsafe`
  blocker.

## Incident case exports

The four required incident classes are published as standalone case-export
packets under `fixtures/platform/m5-system-entry/cases/`, so support can
reproduce each from typed diagnostics instead of a screenshot:

- `wrong_association.json` — a file delivered through an association owned by
  another channel, recovered with `open_with_correct_handler`.
- `moved_target.json` — a recent-item reopen whose folder moved, recovered with
  a target picker.
- `mixed_root.json` — a workspace whose roots span mismatched roots, recovered
  with `select_intended_root`.
- `policy_blocked.json` — a review deep link blocked by managed policy, degraded
  to a policy-block detail with a return path.

## Other invariants

- Every intake names an `active_profile_owner_ref`, a `channel_build_owner_ref`,
  and a `trust_checkpoint_ref`; a missing trust checkpoint is a
  `trust_evaluation_bypassed` blocker and a missing channel owner is a
  `hidden_channel_ownership` blocker.
- Every intake reuses a `canonical_command_ref` — the same command the
  in-product path runs — so the OS path can never grant more authority than the
  in-product path.
- Stale evidence on a marketed intake is a blocker so release tooling can narrow
  the surface instead of shipping it as implicitly stable.
- The report cross-links the native-desktop matrix, the install-topology packet,
  the project-entry contract, the entry interstitials, the handoff-review
  surface, and the auth-recovery packet so ownership and routing cannot drift
  independently.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- validate
cargo test -p aureline-shell --test m5_system_entry_fixtures
python3 tools/ci/m5/system_entry_check.py
```

Regenerate the fixtures and the published report from the seed:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- report \
  > fixtures/platform/m5-system-entry/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- support-export \
  > fixtures/platform/m5-system-entry/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- compact \
  > fixtures/platform/m5-system-entry/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- report-md \
  > artifacts/platform/m5-system-open-and-file-association.md
```
