# Educational AI and practice spaces

Aureline's educational AI teaches you about your workspace **without pretending
to be all-knowing or able to act on its own**. When a panel or a contextual
*why-now* card tells you something about your repository, it shows you exactly
where that came from — the file, symbol, doc, example, or command it read — and
keeps a one-click way to open that source or its docs. When you want to practice,
a **practice/sandbox indicator** tells you plainly whether you are in a throwaway
sandbox, a local-only scratch space, or working against your live repository — so
you always know whether a mistake is safe.

The canonical machine source is checked in at
[`fixtures/help/m5/educational-ai-and-practice/m5_educational_ai_and_practice.json`](../../fixtures/help/m5/educational-ai-and-practice/m5_educational_ai_and_practice.json)
and validated against
[`schemas/help/m5-educational-ai-and-practice.schema.json`](../../schemas/help/m5-educational-ai-and-practice.schema.json).
Help/About, settings, diagnostics, support export, and docs/migration surfaces
ingest that manifest rather than rephrasing educational-AI or practice posture by
hand.

## Educational answers are cited, not omniscient

An educational-AI panel or why-now card that claims something about your
repository **cites it**:

- It names the **files, symbols, docs, examples, or commands** it drew from.
- It keeps the **open-source** and **open-docs** actions one step away — one
  keystroke or one click, never buried in a menu.
- It attaches a **scope label** so you know whether a claim comes from your
  **live repository state**, a **simulated example**, or **local-only** state.

An answer never sounds like it knows everything, and it never claims it can act
on your workspace directly. If it offers to *do* something, that "do" is a
separate, explicit step that rides the same preview/approval path as ordinary
work — educational AI never takes a shortcut around it.

## Explain and do stay separate

Every panel declares its explain-versus-do posture:

- **Read-only** — it only explains; there is no apply verb.
- **Fully separated** — explain and apply are present but distinct.
- **Apply requires approval** — any apply is gated behind the standard preview
  and approval fence before anything changes.

A panel that blurred explain and do, presented itself as omniscient, or claimed
it could act without approval would narrow below Stable with a named reason and
fail validation.

## Practice spaces are visibly distinct from live state

A practice/sandbox indicator always tells you, up front:

- **Target scope** — what the practice surface can touch.
- **Reset / discard behavior** — whether work is `discard_on_exit`, cleared by an
  `explicit_reset_action`, or `persists_until_cleared`.
- **A persistence note** — a plain-language line about what survives.
- **Surface state** — whether it is **local-only**, **simulated**, or running
  against **live repository state**.

A simulated or local-only practice space is a safe, low-risk sandbox and ships
Stable. A practice surface that runs against live repository state is a real,
higher-risk space, so it is honestly marked narrowed (Beta) with a named reason,
and any change it makes still rides the standard preview/approval path. A practice
surface that was *not* distinct from your live workspace — or that mutated live
state without going through approval — would narrow below Stable, and both the
validator and the schema reject it.

## Overlays respect your attention and accessibility

Educational panels and practice indicators are overlays, and they stay polite:

- They **respect quiet-hours** and **reduced-motion**.
- They are **keyboard reachable** and **screen-reader labeled** — never a
  pointer-only path.
- They stay **scoped to your client**, never broadcast globally.
- They **never spam an attention surface** with toasts or badges.

If an overlay cannot surface on an offline or mirrored profile, it says so with a
disclosed cached/mirror-stale freshness (narrowed to Beta) rather than turning
into a dead link — a true dead link narrows below Stable.

## Guardrails

- **Experts are never trapped.** No panel forces a tutorial you cannot dismiss.
- **Your learning is yours.** Educational surfaces never widen who can read your
  progress to the repository or collaborators.
- **No hidden authority.** Educational AI never mutates live state without going
  through the same preview/approval model as ordinary work.

## Where to see it

Educational-AI and practice posture is inspectable in **Help/About**,
**settings**, **diagnostics**, and **support export** — never trapped in a
transient overlay.

## How it is verified

```sh
cargo test -p aureline-learning educational_ai
cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_and_practice -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_and_practice -- summary
```

`derive_panel_verdict` folds each panel's citation, open-action, omniscience,
explain-versus-do, expert-trap, overlay, and offline evidence into the strictest
verdict; `derive_practice_indicator_verdict` folds each indicator's scope,
persistence, distinctness, live-mutation fence, reversibility, overlay, and
offline evidence; and `validate_m5_educational_ai_and_practice` re-derives and
checks both — so a hand-edited fixture that disagrees with its own evidence fails
validation.
