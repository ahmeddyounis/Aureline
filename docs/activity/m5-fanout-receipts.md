# Fanout receipts

This document describes the working engine that mints **one durable, privacy-safe
delivery-truth receipt per cross-client destination** — the native OS notification and the
browser and mobile companions — whenever an attention object is fanned out. It turns
out-of-window delivery into a **governed receipt model** rather than a best-effort side
effect of desktop notifications or companion push.

Where the [attention-routing matrix](./m5-attention-routing.md) *names and freezes the
object model* (including the `FanoutReceipt` object and the `fanout_stale` /
`fanout_undelivered` states) and the [envelope-routing contract](./m5-envelope-routing.md)
*routes a fresh envelope to its surfaces*, this lane records **what actually happened to
each cross-client copy**: which destination received which notification, in what state,
with which explicit stale/undelivered reason, and under which privacy-safe summary posture.
A failed fanout is *visible truth*, never silently counted as delivered.

The track invariant this lane protects: **attention is routed, typed, privacy-aware, and
reopen-safe.** OS and companion fanout cannot be trusted until it is modeled as a durable,
privacy-aware receipt with explicit delivery and reopen semantics.

If this document, the companion schema, and the worked fixture disagree, the normative
sources in `.t2/docs/` win and this document plus its companions update in the same change.

## One receipt per destination

[`mint_receipt`](../../crates/aureline-activity/src/m5_fanout_receipts/mod.rs) is a pure
function of a
[`FanoutSource`](../../crates/aureline-activity/src/m5_fanout_receipts/mod.rs) — the
delivery-relevant projection of a notification envelope or durable activity object — and a
[`FanoutAttempt`](../../crates/aureline-activity/src/m5_fanout_receipts/mod.rs) — what the
transport reported for one destination. It returns a
[`FanoutReceipt`](../../crates/aureline-activity/src/m5_fanout_receipts/mod.rs).
[`mint_dispatch`](../../crates/aureline-activity/src/m5_fanout_receipts/mod.rs) mints one
receipt per governed destination for a named condition, so the same `(source, condition)`
yields the same [`FanoutDispatch`](../../crates/aureline-activity/src/m5_fanout_receipts/mod.rs)
byte-for-byte in support export and CLI / headless diagnostics.

The three governed destinations are the out-of-window mirror surfaces. The in-app activity
center is the **authoritative durable record**, not a fanout copy; the dock/taskbar badge
and operator dashboard are governed by their own lanes.

| Destination | Client scope | Authoritative |
| --- | --- | --- |
| OS native notification | `os_primary_endpoint` | no (out-of-window mirror) |
| Browser companion | `browser_companion_session` | no (out-of-window mirror) |
| Mobile companion | `mobile_companion_device` | no (out-of-window mirror) |

## Delivery states — no silent success

Every receipt carries one delivery state, mapped to a matrix state, so a copy's truth is
named in the frozen vocabulary. Only **delivered** counts as a successful fanout; every
other state is a labeled gap or a policy suppression and is never folded into the delivered
count.

| Delivery state | Matrix state | Meaning |
| --- | --- | --- |
| `delivered` | `shown` | The transport acknowledged a current mirror copy |
| `stale` | `fanout_stale` | A delivered copy now lags the authoritative object |
| `undelivered` | `fanout_undelivered` | The copy failed or was never delivered, labeled as such |
| `suppressed` | `suppressed` | Withheld by policy (not a transport failure) |
| `unknown` | `unknown_requires_review` | The transport state could not be determined |

Each non-delivered receipt names a **stale/undelivered reason**
(`superseded_by_newer_state`, `client_unreachable`, `delivery_timed_out`,
`managed_endpoint_blocked`, `transport_indeterminate`), so a delivery gap can always explain
itself. A suppressed copy carries a `suppression_reason` instead, kept distinct from a
transport failure.

## Privacy-safe summaries by default

Every **rendered** copy (delivered, stale, or unknown) uses a privacy-safe summary posture
that never renders the full payload, and the matrix redaction class derived from it ties the
posture back to the frozen vocabulary.

| Summary posture | Redaction | Applies when |
| --- | --- | --- |
| `clear_summary` | `summary_only` | A summary-safe source |
| `redacted_summary` | `redacted_payload` | A workspace- or security-sensitive source |
| `lock_screen_safe` | `count_only` | An above-summary-safe copy on a locked screen |
| `open_app_only` | `count_only` | A managed-sensitive source |
| `no_summary` | `count_only` | The copy was undelivered or suppressed (nothing rendered) |

A delivered or stale copy applies a redaction at least as strong as the source's **privacy
floor** (`summary_safe → summary_only`, `workspace_sensitive` / `security_critical →
redacted_payload`, `managed_sensitive → count_only`), so a fanout copy never widens privacy.
On a locked screen, every above-summary-safe copy is reduced to a count-only
lock-screen-safe affordance — sensitive content is never rendered in the clear.

## Managed endpoints never receive the payload

When a destination is a non-compliant managed endpoint, the receipt is recorded as
**undelivered** with the `managed_endpoint_blocked` reason and renders **no summary** — the
payload never crosses the device boundary, and the attention stays on its durable in-product
record.

## Reopen parity and no preview/approval bypass

Every receipt copies the source's authoritative reopen route — the same `reopen_target` and
the same exact opaque `reopen_anchor_ref` — and sets `reopen_is_exact`, so an external alert
**never lands on an ambiguous generic shell** when an exact reopen path exists. Every receipt
also copies the source's preview/approval posture: a receipt whose source
`routes_through_preview_approval` has `inline_action_allowed = false`, so an OS or companion
alert **hands off to the in-product preview/approval flow** instead of executing the gated
action inline.

## The durable record survives

Every dispatch and receipt keeps `durable_record_present = true`. Even an all-undelivered
fanout (a blocked managed endpoint) or an all-suppressed fanout (a policy mute) never drops
the authoritative in-product object — the attention is always reopenable on its durable
record.

## Worked delivery — OS notification under each condition

The OS notification delivery state for each source under each transport condition. The
companion destinations follow the same engine (the mobile copy is stale under `mobile_stale`;
the browser copy is undelivered under `companion_undelivered`).

| Source \\ Condition | all_delivered | mobile_stale | companion_undelivered | os_timed_out | locked_screen | managed_blocked | policy_withheld | transport_unknown |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `task.completed` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |
| `ai.awaiting_approval` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |
| `route.policy_warning` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |
| `security.credential_revoked` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |

A delivered managed-sensitive source (`route.policy_warning`) renders `open_app_only`; a
delivered sensitive source renders `redacted_summary`; under `locked_screen` every
above-summary-safe source renders `lock_screen_safe`; and every `undelivered` / `suppressed`
copy renders `no_summary`.

## The honesty rules, enforced

The canonical [`fanout_receipts_bundle`](../../crates/aureline-activity/src/m5_fanout_receipts/mod.rs)
computes each invariant's `holds` flag from the built destinations, sources, and dispatches,
so an inconsistent edit flips an invariant and fails the freeze gate:

- `fanout.receipt_per_destination` — every dispatch mints exactly one receipt per governed
  destination.
- `fanout.binds_source_and_canonical_event` — every receipt ties back to its source
  notification and canonical event.
- `fanout.failures_labeled_never_counted_delivered` — the delivered count equals the number
  of delivered receipts; failures are labeled, never counted as delivered.
- `fanout.stale_undelivered_have_reason` — every stale/undelivered/unknown receipt carries an
  explicit reason and a reviewable note.
- `fanout.privacy_safe_summary_default` — every delivered or stale copy uses a privacy-safe
  posture and never widens privacy below the source floor.
- `fanout.lock_screen_safe` — above-summary-safe copies are count-only on a locked screen.
- `fanout.managed_endpoint_blocks_payload` — a non-compliant managed endpoint is undelivered
  with no summary.
- `fanout.reopen_parity` — every receipt reopens the source's exact authoritative object,
  never a generic shell.
- `fanout.no_preview_approval_bypass` — an approval-gated alert never acts inline.
- `fanout.durable_record_present` — the durable record survives any fanout outcome.
- `fanout.suppressed_separate_from_failure` — suppression-by-policy stays distinct from a
  transport failure.
- `fanout.every_state_exercised` / `fanout.every_posture_exercised` — the corpus exercises
  every delivery state and every summary posture.
- `fanout.dispatches_reproducible` — every dispatch recomputes from its source and condition.
- `fanout.matrix_bound` — every privacy class, scope, redaction class, reopen target,
  severity, dedupe scheme, and resulting state is one the attention-routing matrix defines.
- `fanout.support_export_safe` — every ref is a repo-relative object ref or opaque
  `aureline://` handle, never raw text.

## Companion artifacts

- [`/schemas/activity/m5-fanout-receipts.schema.json`](../../schemas/activity/m5-fanout-receipts.schema.json)
  — boundary schema for `m5_fanout_receipts_bundle`.
- [`/fixtures/activity/m5-fanout-receipts/canonical_bundle.json`](../../fixtures/activity/m5-fanout-receipts/canonical_bundle.json)
  — the published canonical bundle; the freeze gate asserts the in-code builder equals it
  byte-for-byte.
- [`/artifacts/activity/m5-fanout-receipts.md`](../../artifacts/activity/m5-fanout-receipts.md)
  — the human-readable companion (destination, source, condition, and dispatch tables).
- `crates/aureline-activity/src/m5_fanout_receipts/` — the receipt vocabulary, the source
  and condition corpus, the minting engine, the invariants, and the canonical builder.
- `crates/aureline-activity/tests/m5_fanout_receipts.rs` — the freeze gate.
- `cargo run -p aureline-activity --example dump_m5_fanout_receipts` — the headless emitter
  that regenerates the fixture (`-- --lines` for the human-readable projection).
