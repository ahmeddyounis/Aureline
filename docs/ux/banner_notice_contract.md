# Banner and inline-notice contract: scope, persistence, dismissal, and not-a-toast escalation

This document freezes Aureline’s **context notice** contract: the rules
that decide whether a long-lived condition is rendered as an inline
notice, a banner, a review sheet, or a durable activity state. The goal
is to prevent degraded, trust, quota, update, storage-safety, and policy
conditions from being demoted into transient toasts or otherwise
disappearing while they still materially affect work.

The contract is normative. Where this document disagrees with the UI /
UX Spec or with upstream contracts it composes with, the source spec
wins and this document plus the schema and fixtures MUST change in the
same patch. Where a downstream surface represents a durable condition
only via a transient or easily dismissed cue, this contract wins and the
surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/context_notice.schema.json`](../../schemas/ux/context_notice.schema.json)
  defines the `context_notice_record` and the `context_notice_case_record`
  used by shell, renderer, accessibility, and support/export validation.
- [`/fixtures/ux/banner_cases/`](../../fixtures/ux/banner_cases/)
  contains worked examples for restricted workspaces, policy blocks,
  quota warnings, degraded remote state, update-required notices, and
  low-disk and corruption-recovery paths.

## Upstream contracts

This contract composes with existing owners and does not replace them:

- [`toast_contract.md`](./toast_contract.md) owns toast acknowledgement,
  undo posture, durable rediscovery, and the rule that a toast must never
  be the only durable record of a condition that still matters after
  dismissal.
- [`notification_contract.md`](./notification_contract.md) owns
  notification event lineage, exact-reopen posture, and the high-level
  prohibition that toast-only delivery cannot carry durable outage,
  policy denial, connectivity loss, or long-running work.
- [`overlay_layer_contract.md`](./overlay_layer_contract.md) owns banner
  versus toast overlay placement, z-order, and focus behavior.
- [`dialog_sheet_contract.md`](./dialog_sheet_contract.md) owns when a
  condition must escalate into a sheet, dialog, or dedicated review
  surface instead of remaining as a banner or inline notice.
- [`durable_work_contract.md`](./durable_work_contract.md) owns the
  durable activity-row contract when a condition must remain visible in
  the activity center (or equivalent durable lane) beyond the current
  surface.
- [`status_strip_family_contract.md`](./status_strip_family_contract.md)
  owns top-of-surface persistent state vocabulary and “what still works”
  disclosure for readiness banners and status strips.
- [`degraded_mode_pattern.md`](./degraded_mode_pattern.md) owns the
  lifecycle-status card template and safe recovery disclosure when a
  surface is partially usable.

If this document conflicts with any upstream owner above, the upstream
owner wins and this document plus the schema and fixtures must change in
the same patch.

## Scope

Frozen at this revision:

- five notice classes: `info`, `warning`, `error`, `restricted`, and a
  rare long-lived `success_long_lived` class;
- four scope kinds: `pane`, `file`, `workspace`, `account`;
- one placement ladder that maps a condition to `inline_notice`,
  `banner`, `review_sheet`, or `durable_activity_state`;
- one banner-stacking prohibition: at most one banner per scope boundary
  when a summary banner can truthfully explain multiple active
  conditions;
- dismissal and persistence rules, including the requirement that active
  conditions remain visible (in some form) after dismissal; and
- reviewer-verifiable fields (`what_changed`, `what_still_works`, and a
  next inspect/repair path) so durable conditions are never represented
  only by a transient or ambiguous cue.

Out of scope:

- pixel layout, animation, and iconography details (owned by the design
  system);
- localized copy (the contract freezes structure and required semantics,
  not final phrasing); and
- implementation code paths.

## Definitions

### Context notice

A context notice is a **stateful disclosure**: it names a condition that
continues to affect the user’s current work (capability, trust/policy,
quota, availability, data safety, or recovery posture). A context notice
is not an event log and is not a toast.

When the condition remains true, the notice (or a durable indicator that
represents it) remains visible at the narrowest truthful scope.

### Inline notice

An inline notice is rendered **inside the affected surface**, anchored
to the closest UI locus of attention (field row, editor region, panel
section, command footer). It is the default when the condition affects a
single control, row, or narrow region and does not need cross-surface
awareness.

### Banner

A banner is a **sticky top-of-surface** notice (pane/window/workspace
local per the overlay contract). It is used when the condition affects
the whole pane (or a broad workflow slice) and needs to remain visible
while the user continues working in that surface.

### Review sheet

A review sheet is a sheet/dialog/dedicated review surface used when the
user must **inspect structured details** (policy deltas, capability
groups, evidence, recovery ladders, update/rollback choices, quota
breakdown) before a safe next step is possible. The review surface owns
the detailed explanation; banners and inline notices link to it.

### Durable activity state

A durable activity state is a durable row or durable lane state (for
example an activity-center row) that persists beyond the current
surface’s lifetime. If the condition can outlive the current surface,
must survive dismissal, or needs later return-to after interruption
suppression, it MUST have a durable representation.

### Not a toast

Toasts are for acknowledgement or for mirroring an already-durable
representation. A toast MUST NOT be the only representation of a
condition that continues to affect work. If a toast exists, the
condition still needs an inline notice, banner, durable activity row, or
another durable indicator that remains visible after toast dismissal.

## Notice classes (what they mean)

Notice classes are not “styling themes”. They are meaning-bearing
contracts that dictate scope, persistence, and next-action posture.

| Notice class | Use when | Required posture | Smallest meaningful scope (default) |
| --- | --- | --- | --- |
| `info` | Something changed but the workflow remains fully usable; the user benefits from knowing the reason and where to inspect. | Optional next action; dismissible; must keep a details route available. | `pane` or `file` |
| `warning` | The workflow remains usable but materially degraded: partial results, limited scope, pending quota, stale caches, reduced watch fidelity. | Must state what still works; must offer a safe inspect/repair action; dismissible only if a durable indicator remains. | `pane` (or `workspace` when cross-pane) |
| `error` | The workflow slice is not usable without recovery (but the rest of the product may still work): corruption recovery, hard failure to save, update cannot proceed, provider unavailable with no safe fallback. | Must include a recovery route; may be non-dismissible when the user is blocked; must preserve “continue local-only” when safe. | `pane` or `workspace` |
| `restricted` | The user is blocked or narrowed by trust/policy/entitlement/revocation rules: restricted workspace, policy block, quarantine, forced-disable. | Must say what is blocked and what still works; must route to policy/trust details; dismissal never removes the restriction. | `workspace` or `account` |
| `success_long_lived` | A rare, stateful success that materially affects future work and remains relevant until the user changes scope or acknowledges it: e.g. recovery completed but follow-up verification remains. | Must avoid “celebration”; must say what changed and what to verify; dismissible with durable indicator until acknowledged. | `file` or `pane` |

Rules:

- `restricted` is not a synonym for `error`. It is a **policy/trust**
  state and must preserve the policy owner and inspect path.
- `success_long_lived` exists to prevent hiding stateful “recovered but
  verify” posture behind a transient toast.

## Placement ladder (inline vs banner vs review sheet vs durable state)

Selection is determined by the narrowest truthful scope plus the
expected persistence of the condition.

### 1) Choose the narrowest truthful scope first

Scope kinds are ordered from narrowest to broadest:

1. `file` (one file/buffer/document)
2. `pane` (one panel/editor group/surface slice)
3. `workspace` (current workspace/workset scope)
4. `account` (entitlement/quota/policy that spans workspaces)

A notice MUST NOT be promoted to a broader scope merely for visibility.
If the condition affects only one file or pane, the notice lives there.
If the condition affects multiple panes, the workspace-level strip or
one workspace-level banner MAY summarize it while each pane still shows
its narrow inline or pane-local banner.

### 2) Map the condition to a surface kind

Use this selection matrix:

| Condition characteristic | Minimum surface kind | Escalate when |
| --- | --- | --- |
| Affects one control, one row, or one editor region | `inline_notice` | It impacts the whole pane, blocks navigation, or persists across multiple panes. |
| Affects the whole pane or a broad workflow slice | `banner` | The user must inspect structured detail before proceeding (policy delta, capability group, recovery ladder, update plan). |
| Blocks or gates a consequential action and needs structured inspection | `review_sheet` | The review cannot fit with the primary action visible, or it must survive navigation/restart (dedicated review). |
| Must remain visible beyond the initiating surface’s lifetime, or must survive interruption suppression | `durable_activity_state` | Any long-running work, repeated failures, deferred retries, or policy holds that can be revisited later. |

Notes:

- A banner is **not** a substitute for a durable activity row when the
  condition can outlive the surface or needs later return-to semantics.
- Review sheets are not optional “learn more” surfaces; they are
  required when safe progress depends on structured review.

## Banner stacking and coalescing

At most one banner per scope boundary (pane or workspace) should be
visible when a summary banner can truthfully explain all active
conditions.

Rules:

1. **No banner stacks by default.** If multiple conditions are active at
   the same scope boundary, they MUST coalesce into one summary banner.
2. **Highest-severity wins the headline.** The summary banner’s
   `notice_class` is the highest-severity class among its members, in
   this order:
   `restricted` > `error` > `warning` > `info` > `success_long_lived`.
3. **Details remain inspectable.** The summary banner MUST include a
   details route that reveals the member list and each member’s next
   action (typically via a review sheet or inspector panel).
4. **Do not hide narrow scopes.** A workspace-level summary banner does
   not replace required file- or pane-local inline notices. It may link
   to them; it must not erase them.

## Dismissal, persistence, and “still visible after dismissal”

Dismissal is a UI affordance, not a state transition. A notice is
**resolved** only when the underlying condition changes or when an
explicit acknowledgement rule says resolution is allowed.

### Dismissal policies

- `not_dismissible`: used when the user is currently blocked and the
  banner/inline notice is the only truthful explanation in the current
  view.
- `dismissible_until_resolved`: dismissal may hide the current surface
  instance but MUST leave a durable indicator (status strip cue,
  activity row, inspector row, or equivalent) while the condition
  remains active.
- `dismissible_until_scope_exit`: dismissal may hide the notice for the
  current pane/file/workspace session; returning to the scope re-shows
  it if the condition is still active.
- `dismissible_acknowledges_success`: only valid for
  `success_long_lived` when acknowledgement is the intended end state.

### Persistence rules

1. **If it still affects work, it must remain visible.** After
   dismissal, an active condition MUST still be represented by a durable
   indicator in the same scope (or broader, if the broader indicator is
   the only place the condition is true).
2. **Dismissal cannot clear `restricted`.** Policy/trust restrictions
   remain visible until the policy/trust state changes.
3. **Dismissal cannot erase evidence.** Details routes remain reachable
   after dismissal through the durable indicator.
4. **Do not demote durable conditions into toast-only mirrors.** A toast
   may mirror a notice, but it cannot replace the durable or persistent
   representation.

## Fixture coverage

Fixtures under [`/fixtures/ux/banner_cases/`](../../fixtures/ux/banner_cases/)
MUST:

- validate as `context_notice_case_record`;
- include `what_changed` and `what_still_works` disclosure in the
  expected notices;
- show a details route (command-backed) for inspect/repair; and
- encode dismissal policy such that active conditions remain visible
  after dismissal via a durable indicator.

## Acceptance checklist

A reviewer can accept a contextual notice implementation when all of
these are true:

1. The notice resolves to the narrowest truthful scope (`file`/`pane`/
   `workspace`/`account`) and is not promoted purely for visibility.
2. The notice uses the correct notice class (`info`/`warning`/`error`/
   `restricted`/`success_long_lived`) and does not collapse restrictions
   into generic error state.
3. `what_changed` and `what_still_works` are both present and truthful.
4. The notice exposes a command-backed inspect/repair route that is
   keyboard reachable.
5. Banners do not stack when a summary banner can explain the set;
   member details remain inspectable.
6. Dismissing the notice does not remove an active condition from
   visibility; a durable indicator remains until resolved (or until
   acknowledged for `success_long_lived`).
7. No long-lived condition is represented only as a toast.
