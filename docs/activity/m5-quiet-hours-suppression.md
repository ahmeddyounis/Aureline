# Quiet-hours and suppression routing

This document describes the working engine that applies **one coherent suppression
policy** — quiet-hours, do-not-disturb, presentation/follow, lock-screen privacy, admin
suppression, and managed-endpoint posture — across the in-app activity center, OS
notification, and companion attention surfaces, and explains for every surface whether
an event was **shown, downgraded, or withheld**.

Where the [attention-routing matrix](./m5-attention-routing.md) *names and freezes the
object model* and the [envelope-routing contract](./m5-envelope-routing.md) *routes a
fresh envelope to its surfaces against a routing context*, this lane makes quiet-hours
and suppression a **first-class routing policy** rather than a surface-local
preference. The same [`SuppressionPolicy`](../../crates/aureline-activity/src/m5_quiet_hours_suppression/mod.rs)
governs every surface, so a person never sees one surface honor quiet-hours while
another ignores it, and no surface invents its own mute logic.

The track invariant this lane protects: **attention is routed, typed, privacy-aware,
and reopen-safe.** Suppression never drops the durable record; suppression state stays
separate from audit history and never implies the underlying job or incident
disappeared; high-value events stay accountable even when blocked; and a security
advisory is never silenced.

If this document, the companion schema, and the worked fixture disagree, the normative
sources in `.t2/docs/` win and this document plus its companions update in the same
change.

## One policy, evaluated per surface

[`evaluate_suppression`](../../crates/aureline-activity/src/m5_quiet_hours_suppression/mod.rs)
is a pure function of an
[`AttentionSignal`](../../crates/aureline-activity/src/m5_quiet_hours_suppression/mod.rs) —
the suppression-relevant projection of a notification envelope or durable activity
object — and a `SuppressionPolicy`. It returns a
[`SuppressionDecision`](../../crates/aureline-activity/src/m5_quiet_hours_suppression/mod.rs)
with one `SurfaceSuppressionOutcome` per governed surface. The same `(signal, policy)`
yields the same decision byte-for-byte in support export and CLI / headless
diagnostics.

The four governed surfaces are the durable in-app activity center plus the
out-of-window OS notification and browser/mobile companions. The dock/taskbar badge and
operator dashboard are governed by their own lanes and are out of scope here.

| Surface | Privacy ceiling | Default redaction | Durable | Suppression role |
| --- | --- | --- | --- | --- |
| In-app activity center | managed_sensitive | metadata_safe_default | yes | Always shows the durable record |
| OS native notification | summary_safe | summary_only | no | Governed out-of-window mirror |
| Browser companion | workspace_sensitive | redacted_payload | no | Governed out-of-window mirror |
| Mobile companion | summary_safe | summary_only | no | Governed out-of-window mirror |

## The three dispositions

Every surface reports one of three dispositions and a stable suppression-source token,
so it can always explain itself:

- **Shown** — delivered at the surface's normal treatment; no suppression input
  applied.
- **Downgraded** — delivered with a raised redaction because a suppression input
  applied (lock-screen privacy, admin restriction, or a named/security escape from an
  interruption posture).
- **Withheld** — not delivered to this surface now; the durable in-product record is
  unaffected and the event returns when the policy ends.

The in-app activity center is always **Shown** with the durable record, independent of
any suppression — that is how a muted or quiet event never loses its in-product home,
and how a security advisory is never silenced.

## Suppression sources and precedence

For an out-of-window surface, the engine evaluates the policy in a fixed precedence and
names the one source that produced the disposition:

1. **Managed-endpoint policy** — a non-compliant managed endpoint may not receive
   out-of-window payloads at all; the event is withheld.
2. **Admin suppression (locked)** — admin policy locks cross-client companion fanout;
   companions are withheld.
3. **Interruption postures** — quiet-hours, then do-not-disturb, then
   presentation/follow. A security advisory or a named high-importance event escapes
   here with a redacted summary; everything else is withheld.
4. **Lock-screen privacy** — sensitive content (above summary-safe) is downgraded to a
   count-only affordance.
5. **Admin suppression (restricted)** — cross-client fanout is downgraded to a raised
   redaction.
6. Otherwise the event is **shown** at the surface's normal treatment.

Lock-screen redaction is also folded into an escape, so an event that escapes
quiet-hours on a locked screen still shows only a count-only affordance.

## High-importance escapes only when named

A high-importance security, trust, approval, or route warning escapes the interruption
postures **only when it explicitly names its scope and consequence**. Raw severity is
never enough. Under quiet-hours, the OS surface resolves as:

| Signal | Severity | Named consequence | OS under quiet-hours |
| --- | --- | --- | --- |
| `task.completed` | minor_success | — | withheld |
| `support.export_ready` | informational | — | withheld |
| `collab.review_requested` | handoff_actionable | none | withheld |
| `ai.awaiting_approval` | handoff_actionable | approval_required | downgraded (escapes) |
| `route.policy_warning` | handoff_actionable | route_warning | downgraded (escapes) |
| `trust.provider_changed` | handoff_actionable | trust_change | downgraded (escapes) |
| `security.credential_revoked` | security_advisory | security_advisory | downgraded (escapes) |

`collab.review_requested` is the contrast: it is high-importance but names no
consequence, so it is withheld out-of-window and kept in-product — while
`ai.awaiting_approval`, which names its scope and consequence, escapes with a redacted
summary. A security advisory always escapes.

## Suppression stays separate from audit history

Every out-of-window surface that withheld or downgraded an event records a
`SuppressionLedgerEntry`. The ledger is the durable, inspectable record of *why a
surface suppressed an event*; it is explicitly **separate from the underlying object's
audit history** (`separate_from_audit_history` is always true) and never implies the
underlying object disappeared (`implies_underlying_disappeared` is always false). It
carries the named consequence, the reopen route to the same authoritative object, and a
short reviewable note — never the message body — so suppression choices are inspectable
and exportable without leaking sensitive text.

A withheld or downgraded **high-importance** event additionally sets
`audit_trail_required`, so a blocked high-value event is always accountable.

## The honesty rules, enforced

The canonical [`quiet_hours_suppression_bundle`](../../crates/aureline-activity/src/m5_quiet_hours_suppression/mod.rs)
computes each invariant's `holds` flag from the built surfaces, policies, signals, and
decisions, so an inconsistent edit flips an invariant and fails the freeze gate:

- `suppression.parity_one_policy_all_surfaces` — every decision evaluates the same four
  governed surfaces against one policy.
- `suppression.in_app_durable_record_always` — the in-app activity center always shows
  the durable record.
- `suppression.explains_every_surface` — every outcome carries a stable source token
  and a reason.
- `suppression.three_dispositions_exercised` / `suppression.every_source_exercised` —
  the corpus exercises all three dispositions and every suppression source.
- `suppression.security_never_silenced` — a security advisory always shows in-app and
  escapes out-of-window; it is never silenced on every surface.
- `suppression.high_importance_escapes_only_when_named` /
  `suppression.escape_names_scope_and_consequence` — a high-importance event escapes the
  interruption postures exactly when it names its scope and consequence.
- `suppression.withheld_keeps_durable_record_and_reopen` — a withheld or downgraded
  out-of-window event keeps the durable record and a ledger reopen route.
- `suppression.separate_from_audit_history` — every ledger entry is separate from audit
  history and never implies the underlying object disappeared.
- `suppression.audit_trail_for_blocked_high_importance` — every blocked or downgraded
  high-importance event requires an audit trail and a ledger entry.
- `suppression.downgrade_never_widens_privacy` — a downgrade only ever raises redaction.
- `suppression.state_is_matrix_suppression_state` — every non-shown outcome maps to a
  matrix suppression state (suppressed or quiet-hours-deferred).
- `suppression.decisions_reproducible` — every decision recomputes from its signal and
  policy.
- `suppression.matrix_bound` — every privacy class, scope, redaction class, reopen
  target, severity, and suppression state is one the attention-routing matrix defines.
- `suppression.support_export_safe` — every ref is a repo-relative object ref or opaque
  `aureline://` handle, never raw text.

## Companion artifacts

- [`/schemas/activity/m5-quiet-hours-suppression.schema.json`](../../schemas/activity/m5-quiet-hours-suppression.schema.json)
  — boundary schema for `m5_quiet_hours_suppression_bundle`.
- [`/fixtures/activity/m5-quiet-hours-suppression/canonical_bundle.json`](../../fixtures/activity/m5-quiet-hours-suppression/canonical_bundle.json)
  — the published canonical bundle; the freeze gate asserts the in-code builder equals
  it byte-for-byte.
- [`/artifacts/activity/m5-quiet-hours-suppression.md`](../../artifacts/activity/m5-quiet-hours-suppression.md)
  — the human-readable companion (surface, policy, signal, and decision tables).
- `crates/aureline-activity/src/m5_quiet_hours_suppression/` — the suppression
  vocabulary, the policy and signal corpus, the suppression engine, the invariants, and
  the canonical builder.
- `crates/aureline-activity/tests/m5_quiet_hours_suppression.rs` — the freeze gate.
- `cargo run -p aureline-activity --example dump_m5_quiet_hours_suppression` — the
  headless emitter that regenerates the fixture (`-- --lines` for the human-readable
  projection).
