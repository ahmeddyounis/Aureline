# M5 mutation-and-handoff review matrix

This document describes the canonical packet that freezes the **M5 mutation-and-handoff
review matrix** — one reviewed preview/apply/handoff row per M5 mutation path that can
widen authority or side effects — and that automatically narrows, flags, or withholds the
commit of any path whose actor authority is only inherited, whose approval was bypassed,
whose rollback class is unknown, whose route is unbounded, or whose handoff would sever
authority. It is the user-facing companion to the governed artifact at
`artifacts/execution/m5/m5-mutation-and-handoff-review.json` and the typed model in the
`aureline-execution` crate (`m5_mutation_and_handoff_review`).

The companion **host-boundary matrix** (`docs/execution/m5/m5-host-boundary.md`) answers
*where* work ran; the **build-and-host governance matrix**
(`docs/execution/m5/m5-build-and-host-governance.md`) answers *who owns its truth*; the
**target-discovery matrix** (`docs/execution/m5/m5-target-discovery.md`) answers *how a
target was discovered*. This packet narrows to a fourth question and answers it for every
mutation path: **who is acting, was it approved, how is it recovered, and what does it do
before it commits?** New M5 mutation, preview, browser-runtime, remote, and live-resource
surfaces resolve their review story through this packet, so a request-workspace mutation,
a browser-runtime action, or a companion handoff reuses one reviewed sheet instead of
inventing a hidden per-feature prompt.

## What this packet covers

The packet carries one row for every claimed M5 mutation path:

1. **`request_workspace_mutation`** — a request runtime mutating workspace state.
2. **`browser_runtime_action`** — a browser-runtime action.
3. **`preview_route_action`** — a preview-route action.
4. **`live_resource_operation`** — a live-resource operation.
5. **`remote_mutation`** — a remote mutation.
6. **`companion_handoff`** — a browser/companion handoff.
7. **`tunnel_route_action`** — a time-bound route or tunnel action.
8. **`provider_console_handoff`** — an explicit vendor-console handoff.

Each row answers, for its path:

- **Who is acting?** An `actor_authority` of `verified`, `delegated`, `inherited`, or
  `unestablished`. Authority that is only **inherited** from a trusted local shell is
  never enough to commit on its own.
- **Was it approved?** An `approval_state` of `approved`, `not_required`, `required`, or
  `bypassed`. A **bypassed** approval blocks the commit.
- **How is it recovered?** A `rollback_class` of `reversible`, `compensable`,
  `provider_managed`, or `unknown`. An **unknown** rollback blocks the commit; a
  **provider-managed** rollback can only be previewed natively.
- **What route does it open?** A `route_effect` of `none`, `time_bound`, `persistent`, or
  `unbounded`. An **unbounded** route caps the path at preview-only; a **time-bound**
  route must carry an expiry ref.
- **Does the handoff preserve authority?** A `handoff_continuity` of `not_handoff`,
  `preserved`, `renegotiated`, or `severed`. A **severed** handoff blocks the commit, and
  a **renegotiated** handoff is always flagged for review.
- **What does the user see before commit?** An `expected_duration` of `instant`, `short`,
  `extended`, or `open_ended`, and a `fallback_path` of `reviewed_retry`,
  `open_in_provider`, `vendor_console`, or `no_fallback`.
- **What is backing it?** An `actor_ref`, a `target_context_ref`, a `review_sheet_ref` for
  the reviewed sheet the user saw, a `mutation_receipt_ref` for the machine-readable
  receipt, an `execution_ref` joining the row to the in-product execution, and a
  `support_export_ref` binding the row into desktop, CLI, support exports, and release
  evidence. A granted or pending approval carries an `approval_ticket_ref`; a compensable
  or provider-managed rollback carries a `rollback_plan_ref`; a time-bound route carries a
  `route_expiry_ref`.

## The commit gate

The `published_readiness` a path may publish is the **weakest ceiling** implied by its
observed states, computed as the minimum of the path's declared readiness and the ceilings
of its actor-authority, approval, rollback, route, and handoff states. Ordered low-to-high,
the readinesses are `blocked` < `preview_only` < `approval_required` < `reviewed_apply`.

Each input caps the published readiness:

- **Actor authority** caps at `reviewed_apply` for `verified`, `approval_required` for
  `delegated`, `preview_only` for `inherited`, and `blocked` for `unestablished`.
- **Approval** caps at `reviewed_apply` for `approved` and `not_required`,
  `approval_required` for `required`, and `blocked` for `bypassed`.
- **Rollback** caps at `reviewed_apply` for `reversible`, `approval_required` for
  `compensable`, `preview_only` for `provider_managed`, and `blocked` for `unknown`.
- **Route** caps at `reviewed_apply` for `none` and `time_bound`, `approval_required` for
  `persistent`, and `preview_only` for `unbounded`.
- **Handoff** caps at `reviewed_apply` for `not_handoff` and `preserved`,
  `approval_required` for `renegotiated`, and `blocked` for `severed`.

The `review_decision` records the gate's action:

- **`apply`** — the path resolves a clean reviewed apply.
- **`require_approval`** — the commit may proceed only after an explicit approval ticket.
- **`preview_only`** — only a preview or dry-run is allowed; no commit.
- **`flag_for_review`** — a renegotiated handoff is surfaced for review before adoption.
- **`withhold`** — the mutation cannot proceed at all.

The `narrowing_reasons` are the headline release-control triggers recomputed from the
observed states: `unverified_actor`, `inherited_authority`, `approval_bypassed`,
`rollback_unknown`, `unbounded_route`, and `handoff_severed`. The stored
`published_readiness`, `review_decision`, and `narrowing_reasons` must all equal the
recomputed gate decision, so a path can neither overstate its readiness nor hide a
narrowing by hand.

## The guardrails

A **request-workspace or browser-runtime path can never inherit hidden authority** merely
because it originated from a trusted local shell, and a **browser or companion handoff is
never an approval bypass** or an excuse to drop rollback or fallback semantics. Three
mechanisms enforce this:

- An `inherited` actor authority caps the readiness at `preview_only` and raises
  `inherited_authority`, so the `remote_mutation` row — which came from a trusted shell but
  never re-established the actor's authority for the remote target — can be previewed but
  never silently committed.
- A handoff path must declare a non-`not_handoff` `handoff_continuity`. A `severed` handoff
  blocks the commit (`provider_console_handoff`), a `renegotiated` handoff is flagged for
  review (`companion_handoff`), and only a `preserved` handoff may commit
  (`browser_runtime_action`) — so a handoff is reviewed rather than used as a bypass.
- A narrowed or withheld path must offer a real `fallback_path` (not `no_fallback`), so a
  blocked mutation always surfaces an open-in-provider or vendor-console path instead of
  dropping its fallback semantics.

A handoff is not a blanket block: the `browser_runtime_action` row shows that a handoff
which **preserves** the verified actor, granted approval, and reversible rollback publishes
a clean reviewed apply.

## Per-path rows

| Path | Actor | Approval | Rollback | Route | Handoff | Published | Decision |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `request_workspace_mutation` | `verified` | `not_required` | `reversible` | `none` | `not_handoff` | `reviewed_apply` | `apply` |
| `browser_runtime_action` | `verified` | `approved` | `reversible` | `none` | `preserved` | `reviewed_apply` | `apply` |
| `live_resource_operation` | `verified` | `approved` | `compensable` | `time_bound` | `not_handoff` | `approval_required` | `require_approval` |
| `remote_mutation` | `inherited` | `approved` | `provider_managed` | `persistent` | `not_handoff` | `preview_only` | `preview_only` |
| `preview_route_action` | `verified` | `not_required` | `reversible` | `unbounded` | `not_handoff` | `preview_only` | `preview_only` |
| `companion_handoff` | `delegated` | `required` | `compensable` | `none` | `renegotiated` | `approval_required` | `flag_for_review` |
| `tunnel_route_action` | `unestablished` | `bypassed` | `unknown` | `time_bound` | `not_handoff` | `blocked` | `withhold` |
| `provider_console_handoff` | `delegated` | `required` | `provider_managed` | `none` | `severed` | `blocked` | `withhold` |

## Consuming this packet

Downstream surfaces render the packet's export projection instead of restating each
mutation's review posture by hand:

- **Desktop and CLI review sheets** show the actor, approval, duration, rollback class, and
  fallback so the user sees the full consequence before commit.
- **Request/browser/preview/remote/live-resource mutation lanes** resolve through the one
  reviewed sheet model instead of hidden per-feature prompts, and carry the same actor and
  receipt into the action they run.
- **Companion and vendor-console handoff surfaces** preserve the same authority and
  rollback semantics, flag a renegotiated handoff for review, and offer the open-in-provider
  path rather than treating the handoff as a bypass.
- **Support exports and release/audit evidence** join the per-path `execution_ref`,
  `support_export_ref`, and `mutation_receipt_ref` to the same in-product execution the user
  saw, so which reviewed mutation or handoff actually ran can be reconstructed without
  replaying the action.

The packet is metadata-only: every field is a typed state or an opaque ref, and it carries
no credential bodies, raw provider payloads, host tokens, or control-plane secrets.
