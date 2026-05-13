# Notification Taxonomy Alpha

This alpha note documents the first shell-owned runtime path for
notification taxonomy, badge reconciliation, quiet-hours routing,
privacy-safe external payloads, and suppression audit.

The normative contracts remain:

- `docs/ux/attention_activity_taxonomy.md`
- `docs/ux/notification_envelope_contract.md`
- `docs/ux/os_notification_and_quiet_hours_contract.md`
- `docs/ux/notification_action_grammar.md`
- `artifacts/ux/quiet_hours_policy_matrix.yaml`

The alpha implementation is in `crates/aureline-shell/src/notifications/`.
It keeps four responsibilities separate:

- `envelope.rs` defines the typed envelope shared by toasts, banners,
  status items, durable rows, OS notifications, lock-screen summaries,
  and companion fanout.
- `router.rs` dedupes repeated events by the declared scheme and emits
  one receipt per requested surface, including held, suppressed, and
  deduped outcomes.
- `external.rs` projects OS, lock-screen, and companion payloads with
  privacy-safe summary copy, at most one safe primary action, exact
  reopen, and no shortcut bypass around review or approval flows.
- `actions.rs` and `audit.rs` keep notification verbs and suppression
  explanations inspectable after routing.

## Alpha Behavior

Meaningful work must have durable truth. The router can mirror a durable
event onto a toast or OS notification, but the durable row, activity
center entry, digest group, review context, or canonical object remains
the authoritative reopen target.

Quiet-hours and do-not-disturb behavior narrows interruption without
deleting the event. The posture records active modes and reasons on the
envelope, the router holds or suppresses attention-grabbing surfaces,
and durable surfaces continue to carry the canonical event.

Badge counts reconcile from deduped durable items and action state, not
from raw delivery attempts. Dismiss closes transient chrome only;
acknowledge clears the badge without mutating the source object; snooze
requires a resume condition; mute records the class or source being
muted; resolve requires a source-object mutation; suppress records a
system-side hold.

External payloads are summary surfaces. They expose only privacy-safe
copy, one safe primary action when that action is an in-product open
path, and exact reopen. Destructive or privileged actions are not
completed from OS, lock-screen, dock/taskbar, system-tray, or companion
surfaces.

## Proof Paths

- `fixtures/ux/notification_envelope_cases/` covers the envelope shape
  and fanout receipts.
- `fixtures/ux/notification_routing_cases/` covers toast, banner,
  status item, durable row, dedupe, and reopen routing.
- `fixtures/ux/quiet_hours_cases/` covers quiet-hours, DND,
  privacy-safe badge projection, external payload projection, action
  semantics, exact reopen, and suppression audit.
- `artifacts/notifications/notification_suppression_audit_alpha.yaml`
  is the exported metadata-only audit report for the alpha desktop
  notification and badge case.

## Verification

Run these checks when changing this lane:

```sh
cargo test -p aureline-shell --test notification_routing_tests
cargo test -p aureline-shell --test quiet_hours_protected_walk
cargo test -p aureline-shell --test notification_taxonomy_alpha_tests
```

Use the blessing environment variables named in the fixture README only
when the contract output intentionally changes.
