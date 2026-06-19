# Guided exercises that work the way the rest of Aureline does

Aureline's depth features ship hands-on practice as **guided-exercise rails**: a
short, ordered set of steps that teach a flow by having you do it — not by
trapping you in a separate tutorial mode. A rail walks you from reading the
concept, to practising safely in a sandbox, to applying the real change through
the exact command, preview, and approval path you would use any other day.

This page is the human-readable companion to the canonical rail manifest checked
in at
[`fixtures/help/m5/guided-exercise-rails/m5_guided_exercise_rails.json`](../../../fixtures/help/m5/guided-exercise-rails/m5_guided_exercise_rails.json).
Help/About, docs/migration, and support-export surfaces ingest that manifest
rather than cloning the status text below.

## What a rail gives you

- **A current step with success criteria.** Each step points at the stable file
  or surface it works on and lists the checks that mark it done — so you always
  know what "finished" means.
- **Hints, a reveal, reset, and skip.** Every step exposes the same four
  controls. They are keyboard reachable, show up in the action log like any other
  command, and survive a restart — they are never trapped inside a modal popup.
- **Resumable, private progress.** Where you are in a rail is stored locally and
  owned by you. Close the app and come back; you pick up where you left off. Your
  progress is never shared with the repository or your collaborators.
- **An explicit sandbox/reversible preference.** Where a sandbox is possible, a
  rail practises in it first. Where it isn't, the rail keeps effects reversible
  and asks before it touches real workspace state.

## Explain and do stay separate

Every step declares what it is:

| Step kind | What it may touch |
|---|---|
| `explain` | Opens docs or previews a diff. Read-only — no mutation. |
| `prepare_practice` | Practises in a sandbox/scratch space. Local-only, reversible. |
| `apply_with_approval` | Applies a real change through the standard command, preview, and approval path. |

An explain or prepare-practice step **never** silently escalates into a real
change. Only an `apply_with_approval` step touches the real workspace, and it
does so through the same command id, preview sheet, approval path, and
trust/policy check Aureline uses outside learning mode. A practice step is never
a back door to a mutation that skips the fence.

## Every step is clearly labelled

A rail says what each step will touch, so you are never handed a wider blast
radius than the label implied:

| Mutation target | Meaning |
|---|---|
| `no_mutation` | The step reads, explains, or previews. Nothing changes. |
| `sandboxed_local_reversible` | The step runs in a sandbox/scratch space. Local-only and reversible. |
| `workspace_reversible_approved` | The step changes real workspace state, reversibly, after you approve. |
| `workspace_irreversible_approved` | The step changes real workspace state irreversibly. Honest, but flagged Beta. |

## Controls you can always reach

Hint, reveal, reset, and skip are ordinary, inspectable commands — not modal
buttons that disappear when you click away:

- **Hint** reveals a progressive nudge without giving away the answer.
- **Reveal** shows the full solution when you want it.
- **Reset** returns the step to its starting state.
- **Skip** moves you past a step you do not want to do.

None of these mutate your workspace; all of them are keyboard reachable and
restart-safe. You are never trapped, and an expert can skip ahead at any time.

## Offline and cached rails stay honest

A rail records its freshness state, and a cached or mirrored copy is always
visibly distinct from current live help:

| Freshness | Meaning |
|---|---|
| `live_authoritative` | The installed, current authoritative revision. |
| `mirror_synced_disclosed` | Served from a mirror, disclosed as such. |
| `cached_disclosed` | A cached revision, freshness disclosed. |
| `local_only_disclosed` | Available locally only; not yet mirror-synced. |
| `stale_disclosed` | Known stale; disclosed rather than hidden. |

A non-live rail is never presented as current live knowledge — its freshness is
disclosed, and it is labelled Beta until it is live or mirror-synced.

## Current status

The notebook, request, database, profiler/trace, docs/browser, template/scaffold,
and sync/offboarding families ship Stable, live (or mirror-synced) rails. The
preview family's rail is local-only while mirror sync finishes, and the companion
family's rail is served from a cached revision; both are in Beta and clearly
disclosed. In every case the practice is fully usable — the Beta label reflects
the missing freshness/parity proof, not a broken experience. The docs/browser
rail is read-only end to end, proving a rail need not teach an apply flow to
qualify.

## See also

- Release evidence packet: [`artifacts/ux/m5/guided-exercise-proof/ship-guided-exercise-rails.md`](../../../artifacts/ux/m5/guided-exercise-proof/ship-guided-exercise-rails.md)
- Schema: [`schemas/help/m5-guided-exercise-rails.schema.json`](../../../schemas/help/m5-guided-exercise-rails.schema.json)
- Glossary packs and guided tours: [`docs/help/m5/tour-and-glossary-packages.md`](./tour-and-glossary-packages.md)
- Feature-family learning rails: [`docs/help/m5/m5-feature-family-learning-rails.md`](./m5-feature-family-learning-rails.md)
