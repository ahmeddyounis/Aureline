# Durable attention: routing, job-row truth, quiet hours, privacy-safe OS alerts, and exact-target reopen — contract

This is the reviewer-facing companion for the stable lane that locks durable
attention to Aureline's truth model: one governed record per durable attention
class that binds **one-envelope routing**, a **durable activity-center job row**,
**coherent quiet-hours / admin suppression**, **privacy-safe OS alerts**,
**interruptibility** (no toast-only truth), and **deterministic exact-target
reopen** to a public claim ceiling and an automatic narrow-below-Stable verdict.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet/`](../../../fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet/)
- Schema:
  [`/schemas/ux/lock-notification-routing-durable-activity-center-truth-quiet.schema.json`](../../../schemas/ux/lock-notification-routing-durable-activity-center-truth-quiet.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet.md`](../../../artifacts/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet.md)
- Typed source: `aureline_shell::notification_attention_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_notification_attention_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/notification_attention_stable_fixtures.rs`

## Why one lock record per durable attention class

A long-running or reviewable flow — indexing, restore, install/update, download,
attach/reconnect, a task/test run, a provider sync, a policy notice, a managed
alert — must survive look-away, quiet-hours policy, sleep/resume, and
cross-client delivery without losing its identity or quietly bypassing approval.
When the toast, banner, status item, activity-center row, dock badge, native OS
notification, and companion push each reason about the alert with their own
bespoke behaviour they drift: a toast becomes the *only* record of a failure, a
badge outpaces the durable job model, a notification shortcut runs a mutating
action off the lock screen, or a reopen lands on a generic home pane instead of
the authoritative object.

This lane mints one governed `notification_attention_lock_record` per durable
attention class. It does **not** reinvent the envelope, the router, the lifecycle
grammar, the quiet-hours posture, or the badge reconciliation: each record is a
genuine projection of the live attention stack
(`aureline_shell::notifications`, `aureline_shell::attention_router`), routed
through the corpus in `aureline_shell::notification_envelope_corpus`. The corpus
routes the representative envelope for each class through the one governed router,
derives every pillar fact from the resulting route outcome, and re-checks it
against the shipping conformance lane — so a lock record can never claim more than
what ships.

## The six pillars (and the three honesty rails)

Each record binds, for one canonical attention identity:

1. **Routing** — the alert flows from one typed notification envelope through the
   one governed router into a single route outcome; every resolved surface keeps
   the same reopen target (`routing.routes_from_one_envelope`).
2. **Durable job row** — a `job_id`, `actor_subsystem`, `current_phase`,
   `label`, a canonical `durable_object_ref`, and cancel/retry/open-details
   affordances; it survives look-away, sleep/resume, and restart/restore where
   continuity is claimed (`durable_job.is_durable()`).
3. **Quiet hours / admin suppression** — applied coherently across in-app, OS,
   and companion surfaces; suppression may change fanout but preserves the
   durable object, the reopen target, and the audit trail
   (`quiet_hours.is_coherent()`).
4. **Privacy-safe OS alert** — lock-screen / notification-center copy is
   summary-first and never exposes secrets, raw code, AI prompt content, or
   high-risk action detail by default (`privacy.is_privacy_safe()`).
5. **Interruptibility** — durable work and repeated failures never degrade into
   toast-only truth; repeats coalesce by root cause instead of badge / toast
   churn (`interruptibility.holds()`).
6. **Exact-target reopen** — notifications, badges, and job rows reopen the
   authoritative object, or a truthful placeholder that names what is now
   unavailable, and never re-issue a side effect from the notification surface
   (`reopen.is_deterministic()`).

Three honesty rails keep the record from over-claiming:

- **Distinct lifecycle verbs.** Acknowledge, resolve, dismiss, snooze, and mute
  are distinct transitions on one durable object, derived from the router's own
  lifecycle grammar — not surface-local counters.
- **Truthful badge counts.** Counts are reconciled from durable item state, never
  from raw event fanout, and a badge may not outpace the durable job model.
- **A public claim ceiling + automatic narrowing.** No row asserts a pillar it
  cannot prove. A row missing a pillar, or sitting on a surface whose own
  lifecycle marker is below Stable, is narrowed below Stable with a named reason
  rather than inheriting an adjacent green row.

## The claimed-stable matrix

| Class | Subsystem | Claim | Surface marker | Durable | Cancel / Retry / Resolve |
| --- | --- | --- | --- | --- | --- |
| `indexing.json` | indexer | **stable** | stable | yes | cancel · retry · — |
| `restore.json` | vfs_save | **stable** | stable | yes | — · retry · — |
| `install_update_download.json` | install_update_attach | **stable** | stable | yes | cancel · retry · — |
| `ai_approval.json` | ai_apply | **stable** | stable | yes | — · — · resolve |
| `provider_sync.json` | provider_bearing | **stable** | stable | yes | — · retry · resolve |
| `policy_change.json` | admin_policy | **stable** | stable | yes | — · — · resolve |
| `remote_reconnect.json` | remote_agent | **stable** | stable | yes | — · retry · — |
| `managed_alert.json` | admin_policy | **stable** | stable | yes | — · — · resolve |
| `classroom_presentation_overlay.json` | collaboration | beta (narrowed) | beta | yes | — · — · resolve |

Eight launch-critical durable-job classes lock every pillar and qualify Stable.
The classroom / presentation overlay class is genuinely conformant on routing,
durability, quiet hours, privacy, badges, and reopen, but its attention surface
is still **Beta**; it is therefore narrowed below Stable with the named reason
`surface_not_yet_stable` rather than inheriting the adjacent green rows.

## Quiet hours, suppression, and privacy under live routing

The records are projected through the live channel context, so the suppression
and privacy facts are real:

- `policy_change.json` is minted under **admin suppression** with a
  `policy_forbidden_on_lock_screen` payload. The OS banner and lock-screen
  summary are held, yet the durable status item still delivers,
  `suppression_preserves_durable_object` and `suppression_preserves_reopen_target`
  stay true, the audit trail is present, and `lock_screen_safe_by_default` is true
  because the forbidden payload is never rendered.
- `provider_sync.json` reopens with `denied_requires_revalidation` — a truthful
  placeholder that names the expired session instead of opening the wrong pane.
- `restore.json` reopens with a `placeholder_announced` target because the
  restored object is still rebuilding; this counts as a deterministic, truthful
  reopen, not a failure.

## What the replay gate guards

`notification_attention_stable_fixtures.rs` re-derives every invariant from the
fixtures and the in-code corpus:

- Each fixture matches the in-code projection bit-for-bit (re-emit with the
  emitter when the model or upstream routing changes).
- No launch-critical row is toast-only; every row keeps a durable surface that
  survives look-away and sleep/resume.
- Quiet hours / admin suppression preserves the durable object, reopen target,
  and audit trail.
- OS alerts are summary-first and privacy-safe by default.
- Badges derive from durable item state and never outpace the durable model.
- Acknowledge / resolve / dismiss / snooze / mute are distinct.
- Reopen is deterministic, truthful, and side-effect free.
- No row over-claims; narrowed rows drop below the cutline with a named reason.
- The same item opens from the activity center, command palette, status bar, and
  a menu command, keyboard-first, in normal / high-contrast / zoomed layouts.
- Every row stays available without an account or managed services.

## Regenerating

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_notification_attention_stable -- emit-fixtures \
  fixtures/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet
cargo test -p aureline-shell --test notification_attention_stable_fixtures
```
