# Native-desktop integration and reopen

Aureline's local-first, native-desktop promise reaches more surfaces and
handoff objects in M5, but the system-entry and OS-affordance contract is easy
to leave implicit. System open/save/reveal flows, file associations, protocol
handlers, browser auth callbacks, recent-item and dock/taskbar/jump-list reopen
paths, OS notifications, badge/progress indicators, removable-volume and
network-share disappearance, and credential-store lock states each have
materially different behavior that a single "OS integration supported" claim
hides.

This document describes the one authoritative, machine-readable matrix that
names every claimed system-entry, handler, reopen, notification, and
external-path surface and binds it to the controls every native-desktop entry
must honor. It replaces scattered installer notes, UI copy, and support tribal
knowledge: Help/About, install/update UI, diagnostics, support exports, release
notes, and partner evaluations ingest the matrix instead of maintaining
parallel summaries.

## Canonical objects

| Object | Path |
| ------ | ---- |
| Typed consumer | `crates/aureline-shell/src/m5_native_desktop/mod.rs` |
| Headless inspector | `crates/aureline-shell/src/bin/aureline_shell_m5_native_desktop.rs` |
| Boundary schema | `schemas/platform/m5-native-desktop-matrix.schema.json` |
| Report fixture | `fixtures/platform/m5_os_entry_and_reopen/report.json` |
| Support-export fixture | `fixtures/platform/m5_os_entry_and_reopen/support_export.json` |
| Compact fixture | `fixtures/platform/m5_os_entry_and_reopen/compact.txt` |
| Published matrix | `artifacts/platform/m5-native-desktop-matrix.md` |
| Shiproom review packet | `artifacts/shiproom/m5-native-desktop-review-packet/native_desktop_review_packet.md` |
| CI gate | `tools/ci/m5/native_desktop_check.py` |

The headless inspector is the only mint-from-truth path. The report fixture and
the published matrix at `artifacts/platform/m5-native-desktop-matrix.md` are
asserted bit-for-bit equal to the seeded matrix by
`crates/aureline-shell/tests/m5_native_desktop_fixtures.rs`.

## Track invariant

OS-level entry and reopen never bypass trust, profile, tenant, or policy
evaluation; channel/build ownership is inspectable so no handler can be silently
taken over; notification, badge, and progress signals derive from durable
objects rather than transient polls; and missing roots, locked stores, or
topology drift preserve user context through truthful placeholders and recovery
actions.

## Surface kinds

Every registered surface is one of the required system-entry/reopen surface
kinds. The matrix never collapses materially different behaviors into one
generic row.

- `system_open` — system open / save / reveal flows.
- `file_association` — file-type associations.
- `protocol_handler` — protocol / deep-link scheme handlers.
- `auth_callback` — browser auth callbacks returning to the app.
- `recent_item` — recent-item lists.
- `dock_taskbar_jumplist` — dock, taskbar, and jump-list reopen entries.
- `os_notification` — OS notifications.
- `badge_progress` — badge and progress indicators.
- `removable_path` — removable-volume or network-share paths that can disappear.
- `store_lock_state` — credential-store lock states.

## Controls

Every surface declares a binding for each of the seven canonical controls. A
control is `satisfied` (with captured evidence), `not_applicable` or
`explicitly_narrowed` (with a documented reason), or `failed` (a blocker).

- `trust_policy_evaluation` — OS-level entry routes through trust / profile /
  tenant / policy evaluation rather than bypassing it.
- `channel_build_ownership` — the channel/build that owns the OS registration is
  inspectable; no handler can be silently taken over.
- `wrong_target_recovery` — a reopen hits the exact target, and a wrong target
  offers a recovery path rather than dead-ending.
- `unavailable_path_recovery` — a missing root or unavailable path preserves
  context through a truthful placeholder and a recovery action.
- `policy_block_recovery` — a policy-blocked entry degrades truthfully with a
  recovery action.
- `signal_durability` — a notification / badge / progress signal derives from a
  durable object rather than a transient poll.
- `notification_privacy` — notification, badge, and progress content is
  privacy-safe and carries no credential body or secret on shared surfaces.

Each satisfied control carries an evidence pack. The three recovery controls
(`wrong_target_recovery`, `unavailable_path_recovery`, `policy_block_recovery`)
also carry a recovery-path ref, and `signal_durability` carries the durable
object the signal derives from. The two signal controls (`signal_durability`,
`notification_privacy`) apply to the notification-class surfaces
(`os_notification`, `badge_progress`) and are honestly marked
`not_applicable` on the entry/reopen surfaces.

## Distinct failure classes

Wrong-target reopen, hidden handler takeover, and privacy-unsafe notifications
remain distinct failure classes. A failed control emits its own class so a green
card can never mask a materially different failure:

| Control | Failure class |
| ------- | ------------- |
| `trust_policy_evaluation` | `trust_evaluation_bypassed` |
| `channel_build_ownership` | `hidden_handler_takeover` |
| `wrong_target_recovery` | `wrong_target_no_recovery` |
| `unavailable_path_recovery` | `unavailable_path_silent_loss` |
| `policy_block_recovery` | `policy_block_unsafe` |
| `signal_durability` | `transient_poll_signal` |
| `notification_privacy` | `privacy_unsafe_notification` |

## Owner, platform, channel, and recovery per row

Every claimed row names:

- a **channel/build owner** (`channel_build_owner_ref` + `ownership_kind`), so a
  side-by-side install, managed fleet, or portable build cannot drift its
  registration independently;
- a **platform scope** (`claimed_platforms`, one or more of `macos`, `windows`,
  `linux`);
- a **trust/policy checkpoint** (`trust_checkpoint_ref`) the entry routes
  through;
- **recovery behavior** for the wrong-target, unavailable-path, and
  policy-blocked cases; and
- the exact **degraded-state vocabulary** user-visible surfaces must use when
  the entry is degraded.

## Evidence freshness and downgrade rules

Every row carries an `evidence_freshness` flag and a `downgrade_rule_ref`. A
marketed row whose evidence goes `stale` is a blocker and appears in
`narrowable_marketed_entries`, so release tooling narrows the row on the
affected surfaces instead of shipping it as implicitly stable. Any row that
might appear in Help/About, install/update UI, release notes, support exports,
or partner evaluations carries the same freshness and downgrade discipline.

## Cross-links

The matrix cross-links the upstream packets so channel or handler ownership
cannot drift independently:

- install topology — `artifacts/install/m5/m5-install-and-portability-governance.md`
- embedded boundary — `artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md`
- activity center — `artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md`
- auth recovery — `artifacts/auth/m5_auth_and_recovery.md`
- channel ownership — `artifacts/release/channel_ownership_audit.yaml`
- protocol-handler ownership — `artifacts/platform/protocol_handler_ownership_matrix.yaml`
- file-association ownership — `artifacts/platform/file_association_ownership_matrix.yaml`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- validate
cargo test -p aureline-shell --test m5_native_desktop_fixtures
python3 tools/ci/m5/native_desktop_check.py
```

To regenerate the fixtures and the published matrix after a contract change:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop --"
$BIN report        > fixtures/platform/m5_os_entry_and_reopen/report.json
$BIN support-export > fixtures/platform/m5_os_entry_and_reopen/support_export.json
$BIN compact       > fixtures/platform/m5_os_entry_and_reopen/compact.txt
$BIN report-md     > artifacts/platform/m5-native-desktop-matrix.md
```
