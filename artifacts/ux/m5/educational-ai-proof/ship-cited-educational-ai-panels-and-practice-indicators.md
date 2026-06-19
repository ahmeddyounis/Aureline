# Cited educational-AI panels and practice indicators — release evidence

Reviewer-facing evidence packet for the M5 educational-AI and practice lane. An
*educational panel* is an educational-AI answer panel or a contextual *why-now*
card a person reads while they work; when it claims repository truth it cites the
files, symbols, docs, examples, or commands it draws from and keeps the
open-source / open-docs actions one step away, so it never sounds omniscient or
action-capable. A *practice indicator* declares a practice/sandbox surface's
target scope, reset/discard behavior, persistence note, and whether the surface
is local-only, simulated, or running against live repository state, so a low-risk
teaching space is always visibly distinct from the live workspace. Both are
educational overlays that respect quiet-hours, reduced-motion, accessibility, and
client-scope limits. A record that cannot prove that posture is explicitly
narrowed below Stable with a named reason rather than inheriting an adjacent green
row.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/help/m5-educational-ai-and-practice.schema.json`](../../../../schemas/help/m5-educational-ai-and-practice.schema.json)
- Fixture: [`/fixtures/help/m5/educational-ai-and-practice/m5_educational_ai_and_practice.json`](../../../../fixtures/help/m5/educational-ai-and-practice/m5_educational_ai_and_practice.json)
- Public doc: [`/docs/m5/educational-ai-and-practice.md`](../../../../docs/m5/educational-ai-and-practice.md)
- Aligns with: [`/schemas/learning/m5-feature-family-learning-rails.schema.json`](../../../../schemas/learning/m5-feature-family-learning-rails.schema.json) (shared surface-family taxonomy) and [`/schemas/learning/guided-learning-contracts.schema.json`](../../../../schemas/learning/guided-learning-contracts.schema.json) (shared verdict vocabulary)
- Typed source: `aureline_learning::educational_ai_and_contextual_cards`
- Headless emitter: `aureline_learning_m5_educational_ai_and_practice`
- Test: `cargo test -p aureline-learning educational_ai`

## The panel matrix

| Panel | Family | Kind | Scope | Verdict | Citations | Explain/apply | Offline | Narrowing reason |
|---|---|---|---|---|---|---|---|---|
| `notebook_explain` | notebook | educational_ai_panel | live_repo_state | **qualified_stable** | file · symbol · command | fully_separated | live_present | — |
| `request_workspace_why_now` | request_workspace | why_now_card | live_repo_state | **qualified_stable** | doc · example | read_only | live_present | — |
| `database_why_now_cached` | database_workspace | why_now_card | live_repo_state | **narrowed_beta** | doc | read_only | cached_disclosed | offline/mirror freshness disclosed |
| `docs_browser_simulated` | docs_browser | educational_ai_panel | simulated_example | **qualified_stable** | example | apply_requires_approval | live_present | — |

Every panel keeps both an open-source and an open-docs action one step away
(`steps_away: 1`, keyboard reachable).

## The practice-indicator matrix

| Indicator | Family | Surface state | Reset | Mutates live | Verdict | Narrowing reason |
|---|---|---|---|---|---|---|
| `notebook_sandbox` | notebook | simulated | discard_on_exit | no | **qualified_stable** | — |
| `request_workspace_scratch` | request_workspace | local_only | explicit_reset_action | no | **qualified_stable** | — |
| `database_live_practice` | database_workspace | live_repo_state | explicit_reset_action | yes (fenced) | **narrowed_beta** | live repo-state practice touches the real workspace (disclosed) |

**Overall manifest verdict: narrowed_beta** — the cached why-now card and the
live-repo-state practice surface each narrow themselves with a disclosed reason,
and the narrowest member propagates to the overall verdict; the four
cited/scoped/sandboxed records ship Stable individually.

## What this packet proves

1. **Cited, never omniscient or action-capable.** Every panel that claims live
   repository truth carries at least one citation (`file`, `symbol`, `doc`,
   `example`, or `command`) and keeps an open-source/open-docs action one step
   away — the schema enforces both via `if/then`, and the validator reports an
   uncited claim or a missing open action as a hard violation.
   `presents_as_omniscient` and `claims_direct_action_without_approval` are false
   on every panel.

2. **Explain stays separate from do.** Each panel's `explain_apply_class` is
   `fully_separated`, `read_only`, or `apply_requires_approval` — never
   `conflated`. An `apply_requires_approval` panel MUST set
   `mutation_routes_through_standard_preview_approval: true`; the schema enforces
   it via `if/then` and the validator reports an unfenced do as a hard violation.
   Educational AI never mutates live state outside the same preview/approval model
   as ordinary work.

3. **Practice spaces are distinct from live state.** Every practice indicator
   declares its `target_scope_refs`, `reset_behavior`, and a non-empty
   `persistence_note`, sets `distinct_from_live_workspace: true`, and is
   reversible or discardable. A `simulated` or `local_only` sandbox ships Stable;
   a `live_repo_state` practice surface is an honest, disclosed higher-risk choice
   that narrows to Beta with a named reason. A live surface that mutated outside
   the standard preview/approval model would narrow to Preview — the schema and
   validator both enforce the fence.

4. **Overlays respect attention and accessibility.** Every panel and indicator
   carries an `overlay` that sets `respects_quiet_hours`,
   `respects_reduced_motion`, `keyboard_reachable`, `screen_reader_labeled`, and
   `client_scoped_not_global` true and `spams_attention_surface` false — no
   pointer-only paths, no attention-surface spam, no global broadcast.

5. **Offline and mirror parity is honest.** `live_present` ships Stable;
   `cached_disclosed` and `mirror_stale_disclosed` narrow to Beta with a named
   reason; a `missing_on_offline` dead link would narrow to Preview. The
   round-trip test proves citations, scope labels, and practice state survive
   export and reopen unchanged.

6. **Experts are never trapped.** `traps_expert_in_tutorial` is false on every
   panel.

## How the verdict is derived

`derive_panel_verdict` folds each panel's citation, open-action, omniscience,
explain-versus-do, expert-trap, overlay, and offline evidence into the strictest
verdict. `derive_practice_indicator_verdict` folds each indicator's scope,
persistence, distinctness, live-mutation fence, reversibility, overlay, and
offline evidence. Hard safety violations narrow to `narrowed_preview`; a disclosed
cached/mirror-stale freshness or a live-repo-state practice surface narrows to
`narrowed_beta`. The manifest's `overall_verdict` is the narrowest across all
panels and indicators. Stored verdicts are re-derived and checked by
`validate_m5_educational_ai_and_practice`, so a hand-edited fixture that disagrees
with its own evidence fails validation.

## How to reproduce

```sh
cargo test -p aureline-learning educational_ai
cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_and_practice -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_and_practice -- summary
```
