# Learning-mode profiles

Learning mode in Aureline is a **dial, not a mode switch that takes the wheel**.
A learning-mode profile is an opt-in, user-owned preset that tunes how much
guidance you see — without ever changing who owns your data, what a command can
do, or which trust boundary applies. Turn it up while you learn a new surface,
turn it down once it is muscle memory, and reset it whenever you like. Nothing
about it is hidden, and nothing about it is permanent.

The canonical machine source is checked in at
[`fixtures/help/m5/learning-mode-profiles/m5_learning_mode_profiles.json`](../../fixtures/help/m5/learning-mode-profiles/m5_learning_mode_profiles.json)
and validated against
[`schemas/help/m5-learning-mode-profiles.schema.json`](../../schemas/help/m5-learning-mode-profiles.schema.json).
Settings, Help/About, diagnostics, and support export ingest that manifest rather
than rephrasing learning-mode state by hand.

## What a profile tunes

A profile turns exactly four knobs, and none of them changes authority,
ownership, or the command graph:

- **Tip intensity** — `silent_inline_only` (the expert default; tips never
  interrupt), `gentle_hint`, or `prompted_acknowledge`. Every level is
  dismissable and inline-friendly, so even the most insistent profile cannot
  block your first useful work.
- **Jargon level** — `beginner`, `intermediate`, `advanced`, or `expert`. This
  decides how much specialist vocabulary is defined inline versus assumed.
- **Educational-AI explanation posture** — `explain_only`,
  `explain_then_prepare_preview`, or `preview_only_after_explicit_do`. Explain and
  do stay separate at every posture: the AI explains freely, but any change it
  prepares is a preview that still rides the standard preview/approval/rollback
  fence.
- **Mutation guardrail** — `explain_only_no_mutation`, `preview_required`,
  `approval_required`, or `blocked_until_trust`. Every value fences mutation;
  there is deliberately no "unfenced" option, so a profile can never let
  educational AI or a guided surface write to live state directly.

## Presets are a starting point, not a cage

Profiles are derived from a named preset, then every axis stays independently
tunable:

- **Expert (minimal guidance)** — the quietest profile: inline-only tips, expert
  jargon, and no forced pre-explanation. It never traps you in a tutorial and it
  still fences every mutation.
- **Balanced (default)** — gentle hints, intermediate jargon, and explain before
  act.
- **Guided learner** — the most guidance: prompted hints, beginner jargon,
  explain before act, and an AI that prepares a "do" only after you explicitly
  ask for one.

Learner-facing presets keep *explain before act* on. Only the expert preset may
turn forced pre-explanation off — and even then it keeps the mutation fence and
the standard preview/approval model.

## You own your progress

Dismissals, bookmarks, and the full change history are **yours**: user-owned,
local-first, reversible, and inspectable.

- **Dismissals** can always be undone and follow the profile's scope rather than
  leaking wider.
- **Bookmarks** are user-owned and can travel in your portable-profile export.
- **Change history** records every enable, disable, pause, snooze, resume, reset,
  narrow, and axis change. Every entry is user-initiated — there are no silent
  changes — and every entry is visible in support export.

None of this is ever shared with the repository or with collaborators.

## Scope is explicit

A profile is scoped one of two ways, and the scope is never ambiguous:

- **User-local** — stored in your portable profile and the same across every
  workspace.
- **Workspace opt-in** — layered over your user profile for one workspace only.
  It must be **explicitly** opted into, is never committed to the repository, and
  is never shared with collaborators.

Onboarding preferences cannot silently become repo state or follow a
collaborator into a shared checkout.

## Turning it on, off, paused, reset, or narrowed

Every profile exposes a full set of command-backed controls — enable, disable,
pause, snooze, resume, reset, and narrow. Each control is:

- **Command-backed** — it runs the same command graph as everything else.
- **Keyboard reachable** — every control has a shortcut.
- **Reversible** — no control is a one-way door.
- **Inspectable** — every control shows up in the action log.
- **Non-mutating** — a control only touches your local onboarding state; it never
  writes to or mutates the workspace, and never writes silently.

So you can always turn learning mode on, off, reset it to defaults, or just
narrow it (quieter tips) without disabling it — and without breaking the normal
command graph or changing data-ownership semantics.

## Sync is your choice, and it is disclosed

By default a profile is **local-only** and live-authoritative. You may opt into
**portable-profile sync** so your dial follows you across machines — but synced
state can lag behind another machine, so a synced profile must disclose its sync
and is marked as narrowed (Beta) with a named reason. Policy can also pin a
profile local-only. A synced profile that did *not* disclose its sync would be a
masquerade, and the validator and schema both reject it.

## Where to see it

Learning-mode state is never trapped in a transient overlay. Every profile's
state, change history, and reset path are visible in:

- **Settings** — the live dial and its history.
- **Help/About** — the canonical learnability state.
- **Diagnostics** — the educational-AI posture and mutation guardrail.
- **Support export** — the full, inspectable record.

## How it is verified

```sh
cargo test -p aureline-learning learning_mode_profiles
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_profiles -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_profiles -- summary
```

`derive_learning_mode_profile_verdict` folds each profile's safety, scope,
ownership, exposure, control, and sync evidence into the strictest verdict, and
`validate_m5_learning_mode_profiles` re-derives and checks it — so a hand-edited
profile that disagrees with its own evidence fails validation.
