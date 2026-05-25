# Learnability, glossary, and contextual docs/help — release evidence

Reviewer-facing evidence packet for the lane that promotes the learnability
layer for switching users on claimed stable rows: one canonical disclosure
record per imported source ecosystem (the switching cohort) that binds an opt-in
why-now card, glossary chips, and contextual docs/help to a public claim
ceiling, an automatic narrow-below-Stable verdict, lifecycle-marked guided
affordances, a user-owned local-first privacy posture, recovery and route parity
across the switching row / docs-help browser / command palette / menus,
accessibility across normal / high-contrast / zoomed layouts, and rows that stay
available without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance/`](../../../fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance/)
- Schema: [`/schemas/ux/promote-learnability-glossary-and-contextual-docs-help-guidance.schema.json`](../../../schemas/ux/promote-learnability-glossary-and-contextual-docs-help-guidance.schema.json)
- Companion doc: [`/docs/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance.md`](../../../docs/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance.md)
- Typed source: `aureline_shell::learnability_glossary_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_learnability_glossary_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/learnability_glossary_stable_fixtures.rs`

## The claimed-stable matrix

| Cohort | Ecosystem | Claim | Why-now grounded | Glossary chips | Guided tour |
| --- | --- | --- | --- | --- | --- |
| `vs_code_code_oss.json` | VS Code / Code-OSS | **stable** | yes | 5 | beta |
| `jetbrains_family.json` | JetBrains IDEs | beta (narrowed) | no | 5 | beta |
| `vim_neovim.json` | Vim / Neovim | beta (narrowed) | no | 5 | beta |
| `emacs.json` | Emacs | beta (narrowed) | no | 5 | beta |

The matrix spans the four incumbent ecosystems and deliberately spans both sides
of the cutline: the VS Code cohort is the wizard's live source and qualifies
Stable, while the other three project from the same scoreboard, share the same
stable glossary anchors / cited docs / focus return / non-blocking posture, but
carry a why-now card grounded in docs only and are narrowed to `beta` with
`why_now_card_not_grounded_in_truth`.

## What this packet proves

1. **Stable only when grounded in command/file/symbol truth.** Each record's
   `glossary_chips[*].anchor` is a stable anchor, and `why_now_card.cited_target`
   is a command/file/symbol anchor for the Stable cohort. The builder narrows any
   cohort whose chips lack a stable anchor or whose why-now card is grounded in
   docs only.

2. **Opt-in and non-blocking.** Each record's `posture` has `opt_in: true`,
   `blocks_first_useful_work: false`, and the `why_now_card` is `dismissible`
   with `blocks_first_useful_work: false`. No row forces a tutorial funnel before
   first useful work.

3. **Exact focus return is preserved.** `posture.preserves_exact_focus_return` is
   true with a canonical `focus_return_anchor_ref`, and a
   `dismiss_and_return_focus` recovery route is always present.

4. **Contextual docs reachable in place.** `contextual_docs.help_node_refs`
   cites real docs/help nodes and `opens_in_place` is true.

5. **Guided affordances are lifecycle-marked, never stable by adjacency.** Each
   record's `guided_affordances` carries the learning-mode tour marked `beta`
   with a `support_boundary` and its marker visible in product, docs/help, and
   support export. `honesty_marker_present` is true on every row.

6. **Learning state is user-owned and local-first.** Each record's `privacy` has
   `dismissals_user_owned`, `resume_entries_user_owned`, and
   `learning_digest_user_owned` true; `repo_visible` and `telemetry_grade` are
   false. The builder rejects any record that leaks.

7. **No row over-claims.** Each pillar of `claim_ceiling` is bound to the real
   evidence; `stable_qualification.claim_class` is derived, and narrowed rows
   carry a named `narrowing_reason` and sit below the cutline.

8. **One model across surfaces, keyboard-first, every layout.** `surfaces`
   binds the switching-row / docs-help / command-palette projections to one
   identity and the same recovery ids; `routes` reaches the same row from all
   four surfaces, keyboard-first; `accessibility` holds narration, action
   labels, and affordances across `normal`, `high_contrast`, and `zoomed`.

9. **No account, no managed services.** `available_without_account` and
   `available_without_managed_services` are true on every row.

## How to reproduce

```sh
# Print the stable corpus index.
cargo run -q -p aureline-shell \
  --bin aureline_shell_learnability_glossary_stable -- index

# Per-cohort plaintext truth block (support-export shape).
cargo run -q -p aureline-shell \
  --bin aureline_shell_learnability_glossary_stable -- plaintext

# Re-emit the on-disk fixtures.
cargo run -q -p aureline-shell \
  --bin aureline_shell_learnability_glossary_stable -- emit-fixtures \
  fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance

# Replay + invariant gate.
cargo test -p aureline-shell --test learnability_glossary_stable_fixtures
```

## Verdict

The lane is replacement-grade on the VS Code switching cohort and honestly
narrowed to `beta` on the three cohorts that lack live guided coverage. The
checked-in records, schema, fixtures, headless emitter, and replay gate are the
canonical truth source; the switching row, docs/help browser, command palette,
diagnostics, support export, Help/About, and docs ingest these records rather
than cloning status text.
