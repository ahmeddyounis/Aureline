# Learning digest and progress

Aureline remembers where you are in a learning flow **for you, on your device,
and only for you**. A *progress snapshot* records how far you got through one
tour, exercise, glossary walkthrough, first-run checklist, or contextual-help
sequence — its completed and dismissed steps, where to resume, and how its data
is allowed to move. A *learning digest* is the durable place those snapshots
live: it shows you resume, dismiss, snooze, reset, and export controls instead of
a banner that vanishes the moment you click away. Pause whenever you like, pick
up later, and never wonder where your progress went or who else can see it.

The canonical machine source is checked in at
[`fixtures/help/m5/learning-progress/m5_learning_progress_snapshots.json`](../../fixtures/help/m5/learning-progress/m5_learning_progress_snapshots.json)
and validated against
[`schemas/help/m5-learning-progress-snapshots.schema.json`](../../schemas/help/m5-learning-progress-snapshots.schema.json).
Settings, Help/About, diagnostics, support export, and docs/migration surfaces
ingest that manifest rather than rephrasing progress or privacy state by hand.

## What a snapshot remembers

- **Completed and dismissed steps.** Each step carries a state —
  `not_started`, `in_progress`, `completed`, `dismissed`, or `skipped`. A
  dismissed step is **always reversible**, so dismissing one hint never strands
  the rest of the flow.
- **A resume point.** A resumable flow records the step you resume at, and that
  resume point **survives a restart** — that is what lets you pause and pick up
  later without losing progress.
- **A disclosed lifecycle state.** Every snapshot says plainly whether it is
  `local_only`, `sync_eligible`, `exported`, or `reset` — and that disclosure
  survives support and export review unchanged.
- **A device/local sync policy.** By default progress is `local_only_default`
  and never leaves your device. You can opt into `device_sync_eligible_disclosed`
  so it follows you across machines, or policy can pin it with
  `sync_blocked_by_policy`.
- **Export refs.** When you export progress, the export is recorded, redacts raw
  payloads, and is always user-initiated — never silent.

## Your progress is yours

Progress is **user-owned and local-first by default**. The mere existence of a
tour or exercise never widens who can read your progress:

- It is never visible to the repository.
- It is never shared with collaborators.
- No extension or background service gets telemetry-grade read access to it.
- Sharing always requires an explicit promotion you choose.

A snapshot that broke any of these would narrow below Stable with a named reason
and fail validation.

## The digest replaces the banner

Feature-family onboarding no longer leans on a toast that disappears. Every
snapshot is surfaced by a **durable learning digest** that exposes the full
action set:

- **Resume** — jump back to the resume point.
- **Dismiss** — set the flow aside (reversibly).
- **Snooze** — quiet it for a bounded period.
- **Reset** — clear progress, with a restore available.
- **Export** — carry progress out in a redacted, user-initiated bundle.

Each action is **command-backed** (it runs the normal command graph),
**keyboard reachable**, **reversible**, **inspectable** (it shows up in the
action log), and **non-mutating** (it only touches local progress state — never
the workspace, and never silently). Every snapshot must be covered by at least
one durable digest, so no progress is ever stranded in an ephemeral-only state.

## Sync is your choice, and it is disclosed

Local-only progress is live-authoritative. If you opt into device sync, the
snapshot must disclose it — synced state can lag behind another device, so a
`device_sync_eligible_disclosed` snapshot is marked narrowed (Beta) with a named
reason. A sync-eligible snapshot that did *not* disclose its sync would be a
masquerade, and both the validator and the schema reject it.

## Guardrails

- **Experts are never trapped.** No snapshot may force blocking onboarding.
- **Authority and the command graph never move.** A snapshot never changes an
  authority boundary or drifts the command graph.
- **Educational AI keeps explain and do separate.** A flow that uses educational
  AI routes any prepared "do" through the same preview/approval model as ordinary
  work.

## Where to see it

Progress is never trapped in a transient overlay. Every digest's state, actions,
and recovery path are visible in **settings**, **Help/About**, **diagnostics**,
and **support export**.

## How it is verified

```sh
cargo test -p aureline-learning progress_snapshots
cargo run -q -p aureline-learning --bin aureline_learning_m5_progress_snapshots -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_progress_snapshots -- summary
```

`derive_snapshot_verdict` folds each snapshot's ownership, privacy, restart,
resume, educational-AI, sync, and disclosure evidence into the strictest verdict;
`derive_digest_verdict` folds in each digest's actions, durability, exposure, and
the narrowest verdict of the snapshots it covers; and
`validate_m5_learning_progress_snapshots` re-derives and checks both — so a
hand-edited fixture that disagrees with its own evidence fails validation.
