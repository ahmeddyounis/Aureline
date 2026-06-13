# M5 Exceptions, Waivers, and Remembered Decisions

This page is the human-readable companion to the canonical M5 exception/expiry
packet emitted by `aureline-policy` and mirrored at
`artifacts/governance/m5_exception_expiry.md`. A frozen example of the packet
lives at `fixtures/governance/m5_exception_expiry/canonical_packet.yaml` and its
shape is validated by `schemas/governance/m5_exception_expiry.schema.json`.

## Why this packet exists

The M5 surfaces add high-risk connector, provider, remote, and admin actions on
durable artifacts — AI evidence, provider-linked work items, companion and
incident packets, managed sync/offboarding records, and richer support/export
surfaces. When one of those actions needs to deviate from policy, the deviation
must not be a vague, permanent "bypass". It must be a narrow, time-bounded,
auditable object that visibly revalidates when the real world drifts.

This packet makes each exception, waiver, and remembered decision honest:

- **exact, not generic** — the request shows the precise variance, the pinned
  scope, the reason, the approver, and the expiry rather than generic bypass
  language;
- **always bounded** — every row carries an explicit expiry and review target;
- **always scoped** — every row pins all five authority dimensions
  (actor, object, target, policy epoch, environment);
- **always revalidated** — a remembered decision is re-reviewed, never silently
  reused, when any pinned dimension drifts or the expiry lapses; and
- **never widening** — revalidation can only confirm the unchanged, pinned
  scope; it can never broaden the original grant.

It is metadata-only and carries no credential bodies or raw provider payloads.

## Object model

Each row (`M5ExceptionExpiryRow`) binds:

- **Identity and class** — the `exception_id`, the subject class (policy
  exception, policy waiver, or remembered decision), and the governed artifact
  family and record class tokens it applies to.
- **Exact variance** — the precise scope and operation the exception grants, the
  accountable owner/approver, the reason, and the mitigation kept in force while
  it is live.
- **Bounds** — the creation, exact expiry, and review-target timestamps, plus
  the fallback behavior once it lapses.
- **Pinned scope** — an `ExceptionScopeBinding` naming the actor, object, target,
  policy epoch, and environment; no dimension may be left unbound.
- **Reapproval triggers** — the drift classes (target, policy, version,
  authority) that automatically force revalidation.
- **Approval history** — the ordered `ApprovalEvent` lineage from request to the
  current state; the latest event must match the row's `current_state`.

## Projections consumed by product, CLI/headless, and support

- **Exception request sheets** (`request_sheets`) — show the exact variance,
  scope summary, reason, approver, mitigation, and expiry.
- **Approval-history rows** (`approval_history`) — keep the full lineage and
  current state visible everywhere, including support exports.
- **Expiry banners** (`expiry_banners`) — surface `active`, `expiring_soon`, or
  `expired`, computed by comparing the exact expiry and review target against
  the packet's `as_of` instant (ISO-8601 UTC instants compare chronologically,
  so the banner needs no wall clock).
- **Remembered-decision revalidation** (`revalidate`) — takes the
  `ObservedContext` seen at reuse time and returns `still_valid` only when every
  pinned dimension matches and the decision is unexpired; otherwise it returns
  `must_re_review` with the drifted dimensions and `must_reauthorize = true`.

## Relationship to the records and simulation lanes

The records-side hold/retention packet (`aureline-records::m5_records_policy`)
references these exception ids; the support export
(`aureline-support::m5_records_policy_governance`) proves every referenced
exception resolves to a live, bounded policy exception here, surfaces the
request sheets, approval history, and expiry banners alongside the hold/retention
and pre-apply simulation truth, and re-validates every remembered decision so a
stale or drifted exception cannot keep gating a managed claim.

## Guardrails

- An exception never implies remote deletion, remote export, or remote legal
  hold for an artifact the platform only knows locally.
- A remembered decision never widens authority across actor, object, target,
  policy epoch, or environment drift.
- Delete/export honesty outranks cosmetically simple "done" copy: a drifted or
  expired decision narrows to a re-review prompt rather than silently
  proceeding.
