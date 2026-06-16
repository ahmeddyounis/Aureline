# Recent-item, dock/taskbar, and jump-list reopen fidelity

Recent items and system quick actions are launch-bearing desktop truth. The OS
shows the user a list of objects in an "Open Recent" menu, a macOS dock recent
list, a Windows taskbar recent section, or a jump-list entry, and a single click
is expected to land on the *exact* object the user last worked on. If that
expectation is implicit, a reopen quietly betrays it: the item moved and opens an
empty shell, a root unmounted, a side-by-side install now owns the registration,
a provider-linked object's authority expired, or the captured literal now
resolves to a different object than the one registered — and the surface reopens
the wrong thing as if nothing had changed.

This document describes the typed reopen-fidelity layer that keeps each
system-level reopen honest. Every object registered in a `recent_item`, `dock`,
`taskbar`, or `jump_list` surface is projected as one typed reopen target that
preserves the exact object identity, the originating channel/build owner, the
target freshness, and the resolved availability, and binds its certainty to the
*same* restore vocabulary the in-product shell already uses — so external
re-entry never looks more certain than internal restore.

This layer rides on top of the native-desktop matrix
(`docs/m5/native-desktop-integration-and-reopen.md`), which governs which reopen
surfaces a platform exposes, and the system-entry intake
(`docs/m5/system-open-and-file-association.md`), which governs what happens once
a surface delivers a target. This layer governs the *fidelity of the object the
surface offers to reopen*.

## Canonical objects

| Object | Path |
| ------ | ---- |
| Typed consumer | `crates/aureline-shell/src/m5_recent_items_and_reopen/mod.rs` |
| Headless inspector | `crates/aureline-shell/src/bin/aureline_shell_m5_reopen_target.rs` |
| Boundary schema | `schemas/platform/m5-reopen-target.schema.json` |
| Report fixture | `fixtures/platform/m5-reopen-targets/report.json` |
| Support-export fixture | `fixtures/platform/m5-reopen-targets/support_export.json` |
| Compact fixture | `fixtures/platform/m5-reopen-targets/compact.txt` |
| Case-export fixtures | `fixtures/platform/m5-reopen-targets/cases/*.json` |
| Published report | `artifacts/platform/m5-recent-item-and-reopen.md` |
| CI gate | `tools/ci/m5/reopen_target_check.py` |

The headless inspector is the only mint-from-truth path. The report fixture and
the published report at `artifacts/platform/m5-recent-item-and-reopen.md` are
asserted bit-for-bit equal to the seeded report by
`crates/aureline-shell/tests/m5_reopen_target_fixtures.rs`.

## Track invariant

A reopen from a system-level surface lands on the exact object or a clearly
labeled placeholder with recovery actions. Object identity and channel/build
ownership stay inspectable; the reopen result reuses the shell's restore and
placeholder vocabulary so external re-entry is never more certain than internal
restore; and dock/taskbar/jump-list shortcuts encode no hidden mutation — a
privileged action returns through a reviewed in-product surface rather than
firing from the shortcut.

## Reopen surfaces

Every reopen target is registered on one of the four required reopen surfaces.
Platforms do not expose the same surfaces, so each target proves its claim per
platform:

- `recent_item` — an in-app or OS "Open Recent" recent-item menu (macOS, Windows,
  Linux).
- `dock` — a macOS dock recent-documents menu (macOS only).
- `taskbar` — a Windows taskbar recent section (Windows only).
- `jump_list` — a Windows jump-list tasks or pinned entry (Windows only).

## Exact object identity

Each reopen target preserves two identities so a moved, renamed, or
re-registered object can never masquerade as the thing the user expected:

- `literal_target_ref` — an export-safe captured ref for the literal the OS
  shortcut holds. It is never a raw path or secret body.
- `canonical_object_ref` — the canonical object identity the literal was
  registered against, classified into the shared `target_kind`.

When the literal now resolves to a different object than the one registered, the
target records the `conflicting_object_ref` so the wrong-target incident is
concrete and exportable rather than an anecdotally reproduced bug.

## Restore-vocabulary binding

The reopen result reuses the same vocabulary the in-product Start Center
recent-work rows use, so external re-entry never overclaims certainty:

- `restore_availability` (`exact`, `compatible`, `layout_only`, `evidence_only`,
  `none`),
- `trust_state` (`trusted`, `restricted`, `pending_evaluation`),
- `portability_class` (`local_only`, `synced`, `imported`, `provider_linked`,
  `stale`), and
- `recovery_actions` (for example `locate_missing_target`, `open_without_restore`,
  `reauth`, `reconnect`, `open_read_only_cached_view`, `remove_from_recents`).

A degraded availability — or a target whose snapshot freshness is `stale` — that
still claims an exact restore is a `stale_certainty_overclaim` blocker.

## Availability and recovery

The resolved availability of the object at reopen time is one of `exact_object`,
`moved_target`, `missing_root`, `changed_channel`, `stale_provider_linked`, or
`wrong_target_detected`. Any value other than `exact_object` MUST carry a
labeled placeholder (`placeholder_label_ref`) and at least one recovery action,
and each unavailable class stays a distinct failure:

- a `wrong_target_detected` reopen with no recovery is a `wrong_target_reopen`
  blocker — its own failure class, never folded into the unavailable-path class;
  and
- a `moved_target`, `missing_root`, `changed_channel`, or `stale_provider_linked`
  target with no recovery is an `unavailable_target_silent_loss` blocker.

## No hidden mutation

Each target declares an `action_class`:

- `reopen_object` — reopen the exact object in place (summary-safe);
- `reveal_object` — reveal the object in the OS file manager (summary-safe); and
- `privileged_or_mutating` — an action that would mutate provider/workspace state
  or widen authority.

A `privileged_or_mutating` shortcut MUST return through a reviewed in-product
surface (`reviewed_return_surface_ref`) instead of firing directly; one that
stays summary-only or names no reviewed surface is a `silent_mutating_action`
blocker. Dock/taskbar/jump-list shortcuts therefore never encode a hidden
approval or a mutating action.

## Channel/build ownership

Each target names an `originating_channel_build_owner_ref` and an
`ownership_kind` (`channel_scoped_owner`, `shared_default_arbitrated`,
`managed_fleet_owned`, `portable_non_registering`). When a side-by-side or
portable install could plausibly own the registration
(`side_by_side_or_portable_plausible`), the channel/build owner stays visible; a
missing owner is a `hidden_channel_ownership` blocker.

## Incident case exports

The five required incident classes are published as standalone case-export
packets under `fixtures/platform/m5-reopen-targets/cases/`, so support can
reproduce each from typed diagnostics instead of a screenshot:

- `moved_target.json` — a recent-item reopen whose file moved, recovered with a
  locate action.
- `missing_root.json` — a jump-list reopen whose workspace root is unmounted,
  recovered with locate and open-without-restore.
- `changed_channel.json` — a dock reopen whose registration is now owned by a
  side-by-side or portable channel.
- `stale_provider_linked.json` — a taskbar reopen of a provider-linked cloud
  workspace whose authority went stale, routed through the reviewed auth-recovery
  surface to reauthorize.
- `wrong_target.json` — a recent-item reopen whose captured literal now resolves
  to a different object, shown as a wrong-target placeholder naming the
  conflicting object.

## Other invariants

- Every target names an `active_profile_owner_ref` and a `trust_checkpoint_ref`;
  a missing trust checkpoint is a `trust_evaluation_bypassed` blocker.
- Every target reuses a `canonical_command_ref` — the same command the in-product
  path runs — so the reopen path can never grant more authority than the
  in-product path, and a `restore_provenance_ref` that binds it to the shell's
  restore-provenance contract.
- Stale evidence on a marketed target is a `stale_evidence_on_marketed_target`
  blocker so release tooling can narrow the surface instead of shipping it as
  implicitly stable.
- The report cross-links the native-desktop matrix, the system-entry intake, the
  install-topology packet, the restore-provenance contract, the Start Center
  recent-work surface, and the entry interstitials so identity and ownership
  cannot drift independently.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- validate
cargo test -p aureline-shell --test m5_reopen_target_fixtures
python3 tools/ci/m5/reopen_target_check.py
```

Regenerate the fixtures and the published report from the seed:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- report \
  > fixtures/platform/m5-reopen-targets/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- support-export \
  > fixtures/platform/m5-reopen-targets/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- compact \
  > fixtures/platform/m5-reopen-targets/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- report-md \
  > artifacts/platform/m5-recent-item-and-reopen.md
```
