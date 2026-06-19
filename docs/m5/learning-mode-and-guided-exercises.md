# Learning mode, guided exercises, and educational AI

Aureline's depth features ship learnability surfaces that you can learn and
revisit in-product — without mandatory setup videos and without leaving Aureline.
This page is the human-readable companion to the frozen learnability lane, the
single source of truth that keeps every feature family using the same vocabulary
and the same canonical lane instead of inventing its own onboarding tricks.

The canonical machine source is checked in at
[`fixtures/help/m5/learnability-regression/m5_learnability_lane_freeze.json`](../../fixtures/help/m5/learnability-regression/m5_learnability_lane_freeze.json)
and validated against
[`schemas/help/m5-learnability-lane.schema.json`](../../schemas/help/m5-learnability-lane.schema.json).
Downstream Help/About, release-center, support-export, and docs/migration
surfaces ingest that freeze packet rather than rephrasing learnability state by
hand.

## The frozen vocabulary

Every learnability surface across the depth families is named with one of nine
frozen terms:

- **Learning mode** — an opt-in, user-owned profile that tunes tip intensity,
  jargon level, and explanation posture. It never changes authority, trust,
  ownership, or mutation-approval semantics, and it never blocks first useful
  work.
- **Tour package** — a versioned, command-backed walkthrough. Every step runs the
  same command, opens the same preview, and uses the same approval prompt as
  ordinary work.
- **Guided exercise** — a hands-on practice rail. Its Apply steps are reversible
  and ride the standard preview/approval/rollback fence.
- **Glossary pack** — the terms a flow uses, each citing an authoritative command
  or docs anchor.
- **Contextual why-now card** — an in-place, read-only explanation of why a
  surface matters right now, linking back to the authoritative command and docs.
- **Educational AI** — assistance that explains freely but keeps *do* separate:
  any change goes through the same preview/approval path as ordinary work.
- **Practice / sandbox indicator** — an explicit marker so you never confuse a
  rehearsal surface with live work.
- **Learning digest** — a user-owned summary of your progress and resume points.
- **Progress snapshot** — a user-owned, local-first record of your progress and
  dismissals that survives a restart.

## One canonical lane per surface

Each claimed feature family — notebooks, request and database workspaces,
profiler and trace flows, docs/browser depth, preview surfaces, template and
scaffold planners, the companion/incident surface, and sync/offboarding — maps
every one of those terms onto a single canonical lane. The cross-cutting terms
(learning mode, educational AI, learning digest) share one lane across all
families, so no family forks its own onboarding state.

Two guarantees hold for every row in the matrix:

- **No hidden coachmarks.** Every surface is command-backed; there is no
  tutorial-only shortcut and no surface that hides behind a feature-local
  coachmark.
- **No private mutation paths.** Nothing in a learning surface mutates your
  workspace through a private path. If a step can change state, it rides the same
  preview and approval fence as ordinary work.

## Explain stays separate from do

Educational AI and guided practice keep *explain* and *do* as separate verbs.
The AI can explain a flow as much as you like, but it never mutates live state
directly: any change is prepared as a preview and applied only through the
standard approval path, with the same rollback semantics as ordinary work. A
practice/sandbox indicator marks rehearsal surfaces so you always know whether
you are practicing or working for real.

## Your progress is yours

Learning progress, dismissals, and resume points are user-owned and stored
locally by default. They are not visible to the repository, are never read at
telemetry grade, survive a restart, and are safe to include in a support bundle —
the support export shows the same state you see in-product and never carries
credential bodies.

## Offline and mirrored profiles

Learnability surfaces stay available on local-only, air-gapped, and mirrored
profiles. Instead of silently dead-linking when content is cached or the origin
is unreachable, each surface shows an explicit freshness label.

## Current status

Most lane rows qualify Stable. The preview family's tour and exercise packs are
in Beta while their learning content finishes mirror sync; the flows are fully
usable, and the Beta label reflects the missing parity proof rather than a broken
experience.

## See also

- Per-family learning rails: [`docs/help/m5/m5-feature-family-learning-rails.md`](../help/m5/m5-feature-family-learning-rails.md)
- Evidence packet: [`artifacts/ux/m5/learnability-freeze-packet/freeze-the-m5-learning-mode-tour-package-guided-exercise-and-progress-snapshot-matrix.md`](../../artifacts/ux/m5/learnability-freeze-packet/freeze-the-m5-learning-mode-tour-package-guided-exercise-and-progress-snapshot-matrix.md)
