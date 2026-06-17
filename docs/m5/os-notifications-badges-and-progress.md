# OS notifications, badges, and progress parity

Aureline's in-product activity center already treats attention as durable truth:
every badge, progress indicator, and reopen action is derived from a durable job
object, never from a transient toast. This document describes how that promise is
carried out to the **OS surfaces** — lock-screen and notification-center text,
the dock/taskbar badge, the dock/taskbar progress affordance, and the companion
mirror — for the M5 durable job families.

The contract is a typed truth packet, not prose: the
[`aureline-shell` `m5_os_notifications_and_badges`](../../crates/aureline-shell/src/m5_os_notifications_and_badges/mod.rs)
module mints one canonical
[`M5OsAttentionReport`](../../crates/aureline-shell/src/m5_os_notifications_and_badges/mod.rs)
that the live shell, support exports, Help/About, and release surfaces read
verbatim. The headless inspector
(`aureline_shell_m5_os_notifications`) is the only mint-from-truth path.

## What the OS surfaces promise

Every registered OS attention surface derives from one durable job object,
reusing the canonical durable job family, durable job state class, and durable
badge count class vocabularies — it never synthesizes a desktop-only state. Each
surface certifies the five OS-attention parity guarantees:

- **`privacy_safe_summary`** — the lock-screen and notification-center copy is a
  bounded summary that names the source object, the client scope, and one safe
  reopen action. It never carries code, secrets, AI prompt text, or high-risk
  mutation detail. The OS notification packet carries stable class enums and
  durable refs only — never a protected payload body.
- **`badge_durable_class`** — the dock/taskbar badge count derives from a durable
  count class (pending review/approval, failed runs, provider-auth attention,
  managed advisories, durable-running work, …), so the badge stays correct after
  retries and partial delivery instead of reflecting raw event fanout.
- **`progress_named_job_class`** — the dock/taskbar progress affordance maps to a
  named durable job class and its envelope progress. It is **never** a generic
  activity spinner; surfaces with no progress (approval or advisory states)
  narrow this guarantee honestly rather than painting a meaningless bar.
- **`suppression_parity`** — quiet-hours, do-not-disturb, and admin suppression
  apply identically across the in-app, OS, and companion surfaces, with a visible
  suppression audit. The OS surface can never be louder or quieter than the
  in-product surface, and a suppressed alert always preserves its durable object
  and reopen target.
- **`exact_reopen_parity`** — the OS action lands on the exact durable object or
  a truthful placeholder that names the source and freshness of the missing
  target. It resolves through an in-product surface and never a privileged OS
  shortcut.

A red result on any guarantee — a lock-screen leak, a protected payload body, a
raw-event badge counter, a generic progress spinner, a diverging suppression
decision, a missing suppression audit, or a lost reopen target — is a blocking
finding. A surface that paints an OS affordance from a synthesized desktop-only
state, or a marketed guarantee claimed with no evidence, is also a blocker, and
stale evidence on a marketed guarantee narrows the row instead of shipping it as
implicitly stable.

## The typed OS notification envelope

Each surface carries a typed
[`M5OsNotificationEnvelope`](../../crates/aureline-shell/src/m5_os_notifications_and_badges/mod.rs) —
the privacy-safe truth packet the OS surfaces read. It binds the durable job
family and state class, the durable job id and canonical event id, the privacy
class and client scope, the explicit source-object and safe-reopen-action label
refs, the lock-screen and payload disclosure classes, the durable badge count
class, the named progress basis, the suppression decision and parity, and the
exact-target reopen linkage. The envelope carries no credential bodies or raw
provider payloads.

## Source of truth and verification

The checked-in fixtures and the published audit are the canonical truth source
for OS notifications, badges, progress, and reopen across the M5 durable job
families. Regenerate and verify them with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- report > \
  fixtures/ux/m5_os_notifications_and_badges/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- support-export > \
  fixtures/ux/m5_os_notifications_and_badges/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- compact > \
  fixtures/ux/m5_os_notifications_and_badges/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_os_notifications -- report-md > \
  artifacts/ux/m5/os-notification-and-reopen.md
cargo test -p aureline-shell --test m5_os_notifications_and_badges_fixtures
python3 tools/ci/m5/os_notifications_and_badges_check.py --repo-root .
```

## Canonical artifacts

- Typed consumer:
  [`crates/aureline-shell/src/m5_os_notifications_and_badges/`](../../crates/aureline-shell/src/m5_os_notifications_and_badges/mod.rs)
- Boundary schema:
  [`schemas/ux/m5-os-notification-envelope.schema.json`](../../schemas/ux/m5-os-notification-envelope.schema.json)
- Fixtures: `fixtures/ux/m5_os_notifications_and_badges/report.json`,
  `support_export.json`, `compact.txt`
- Published audit:
  [`artifacts/ux/m5/os-notification-and-reopen.md`](../../artifacts/ux/m5/os-notification-and-reopen.md)
- CI gate: `tools/ci/m5/os_notifications_and_badges_check.py`

This lane builds on the in-product notification and badge truth in
[notification privacy and badges](notification-privacy-and-badges.md) and the
durable progress and reopen identity in
[durable progress and reopen](durable-progress-and-reopen.md); the OS surfaces
reuse those durable objects rather than maintaining a parallel summary.
