# Learning-state portability: export and reset

Your learning progress is yours, and Aureline lets you **carry it out** or **clear
it** without losing track of where it came from, who can see it, or which pack you
learned it against. A *learning-state export bundle* ports one slice of
learnability state — a tour, an exercise, or a learning session — to a portable
profile, a support bundle, or a local download. A *learning-state reset plan*
clears a bounded slice of that state and tells you exactly what it touches and
what it leaves alone.

The canonical machine source is checked in at
[`fixtures/help/m5/learning-state-export-and-reset/m5_learning_state_export_and_reset.json`](../../fixtures/help/m5/learning-state-export-and-reset/m5_learning_state_export_and_reset.json)
and validated against
[`schemas/help/learning-session-export.schema.json`](../../schemas/help/learning-session-export.schema.json).
Settings, Help/About, diagnostics, support export, and docs/migration surfaces
ingest that manifest rather than rephrasing export/reset, privacy, or continuity
state by hand.

## Exporting carries your progress out — safely

An export bundle takes a slice of learnability state out of the device while
keeping it honest:

- **Provenance survives.** The bundle records the source state it carries
  (`source_state_refs`) and keeps `provenance_preserved` true, so the trail back to
  where the progress came from is never lost.
- **Everything sensitive is redacted.** Raw payloads, credential bodies, and
  absolute paths are redacted, and an export **never widens who can read your
  state** — it stays user-owned and local-first.
- **Nothing happens silently.** Exports are user-initiated, and each bundle says
  whether it is safe to drop into a support export.

## Localized? The source language is always one step away

When a learning artifact is presented in a localized language, the export keeps a
**source-language escape**: a command-backed way to step back to the
source-language original. A bundle that is localized (for example, `en-US` shown
as `fr-FR`) **must** carry that escape, and localization only changes display copy
— it never disturbs provenance. Both the schema and the validator reject a
localized export that strands you without the escape.

## Cached or mirrored? The continuity stays visible

Each bundle discloses the pack it was learned against — where it came from
(`source_class`) and how current it is (`freshness`):

- `live_authoritative` — the installed, current revision.
- `mirror_synced_disclosed` — served from a mirror, disclosed (still Stable).
- `cached_disclosed`, `local_only_disclosed`, `stale_disclosed` — honest but older,
  marked narrowed (Beta) with a named reason.

A non-live pack **must** disclose its continuity. A cached pack that tried to pass
itself off as live would be a masquerade, and both the schema and the validator
reject it.

## Resetting clears what you ask — and nothing else

A reset plan is explicit about its blast radius:

- **It declares its target scope.** `target_state_kinds` lists exactly which
  learnability classes it clears — tour, exercise, learning-session, glossary,
  learning-mode profile, or contextual-hint state.
- **It protects unrelated state.** Every reset preserves your installed docs
  packs, your bookmarks, and your user-authored notes (and the seed also preserves
  model packs, checkpoints, and template packs). A reset **never silently deletes**
  state outside the reviewed learnability scope.
- **It is reversible.** Every reset offers a command-backed restore with a
  disclosed window — reset is never a one-way door.

## Guardrails

- **No hidden mutating tutorial path.** No export or reset introduces a
  tutorial-only mutating shortcut, bypasses the standard preview/approval model
  when it touches real workspace state, changes an authority boundary, or drifts
  the command graph. Explain stays separate from do.
- **Your state stays yours.** Export never widens data sharing, and both export
  and reset keep the state user-owned and local-first.
- **Nothing is silent.** Exports and resets are user-initiated.

## How it is verified

```sh
cargo test -p aureline-learning learning_state_export_and_reset
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_state_export_and_reset -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_state_export_and_reset -- summary
```

`derive_export_bundle_verdict` folds each bundle's redaction, ownership,
provenance, source-language-escape, cached-pack continuity, and mutation-fence
evidence into the strictest verdict; `derive_reset_plan_verdict` folds each plan's
target scope, protected set, reversibility, and mutation-fence evidence; and
`validate_m5_learning_state_export_and_reset` re-derives and checks both — so a
hand-edited fixture that disagrees with its own evidence fails validation.
