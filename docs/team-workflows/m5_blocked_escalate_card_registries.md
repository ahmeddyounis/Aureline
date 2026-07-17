# M5 blocked-escalate-card and escalation-outcome registries

Implement lane over the frozen [M5 change-intent-and-engineering-lifecycle matrix][matrix]
(`m5_change_intent_and_engineering_lifecycle_matrix`). It makes the matrix's `blocked_escalate_card`
object class operable by carrying resolved, honest projections of two registries so the work-item
detail, review detail, Git / worktree, provider-handoff, help / docs, and support / export surfaces
inherit one canonical blocked-or-escalate descriptor rather than a hand-authored parallel prose
that has to be kept consistent. It preserves momentum when work cannot progress cleanly by giving
users one explicit blocker / escalation object — instead of generic failure chrome — that stays
honest about what the provider has actually accepted, what stays local as a handoff packet, and what
remains blocked, reusing the already-landed provider-boundary work-item components, hosted-review
rows, Git worktree identity, AI evidence rows, and review-pack / local-parity truth.

## Registry-A — blocked-escalate card

One reusable, machine-readable blocked-escalate card per blocked or escalated tracked work item,
showing:

- a stable card identity that survives export packets, support bundles, and reopened work-item
  workspaces;
- the blocker class (dependency, approval, provider, policy, or unresolved-engineering cause);
- the missing dependency or approval;
- the suggested escalation path;
- the attach-evidence action;
- the local note or handoff-packet fallback (each side effect disclosed separately);
- the retained linked evidence;
- the resolution-form coverage (canonical object, accessible summary, audit record).

The blocker class, provider authority, and escalation path are always named before the blocked state
can be cleared. A card that would clear a blocked state while an engineering blocker remains
unresolved, that is a hand-copied per-item assumption instead of tracing to the shared registry, that
drops its attach-evidence / export / retry path, or that publishes an incomplete object degrades
honestly instead of implying an acceptance the provider has not given. The registry reuses the matrix
`m5-change-intent.schema.json` domain schema for the tracked item and the
`m5-blocked-escalate-card.schema.json` domain schema for the card layout.

## Registry-B — escalation-outcome

The typed outcome a blocked-or-escalate card resolves toward, keeping the blocker cause and commit
state explicit — escalated to provider, queued as local handoff packet, exported locally, blocked by
missing permission, or blocked by unresolved engineering state — so a local handoff packet never reads
as a provider-committed escalation and a target that is offline, policy-blocked, or only partially
writable stays visible and actionable instead of implying provider acceptance. The registry keeps the
dependency / approval / provider / policy / unresolved-engineering causes distinct rather than
flattening them into one generic warning. The registry reuses the matrix
`m5-blocked-escalate-card.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Users can attach evidence and export or retry from the same blocked state without losing the
   tracked-item context: every surface resolves the same card and escalation-outcome from the shared
   registry, and a card that would drop the attach-evidence / export / retry path or clear a blocked
   state while an engineering blocker remains degrades instead of reading as a clean render.
2. Blocked states distinguish dependency, approval, provider, policy, and unresolved-engineering
   causes instead of one generic warning: the blocker cause and commit state stay visible in the UI
   projection, the CSV / export, and the support packet, and a local handoff packet never reads as a
   provider-committed escalation.
3. Escalated to provider, queued as local handoff packet, exported locally, blocked by missing
   permission, and blocked by unresolved engineering state stay differentiated in both live UI and
   exported packets; no local note, handoff packet, or linked evidence is dropped when a provider
   write fails, and the binding registry keeps each escalation-outcome dimension distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/teamwork/m5-blocked-escalate-card-registries.schema.json`) documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_intent_and_engineering_lifecycle_matrix/mod.rs
