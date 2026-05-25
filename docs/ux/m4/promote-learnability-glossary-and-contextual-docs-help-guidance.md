# Learnability, glossary, and contextual docs/help on switching rows — contract

This is the reviewer-facing companion for the stable lane that makes the
learnability layer replacement-grade for switching users: one governed
disclosure record per imported source ecosystem (the switching cohort) that
binds an opt-in **why-now card**, **glossary chips**, and **contextual
docs/help** to a public claim ceiling and an automatic narrow-below-Stable
verdict — and that surfaces the lifecycle marker of any guided tour / learning /
teaching affordance instead of implying stable coverage by adjacency.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance/`](../../../fixtures/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance/)
- Schema:
  [`/schemas/ux/promote-learnability-glossary-and-contextual-docs-help-guidance.schema.json`](../../../schemas/ux/promote-learnability-glossary-and-contextual-docs-help-guidance.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance.md`](../../../artifacts/ux/m4/promote-learnability-glossary-and-contextual-docs-help-guidance.md)
- Typed source: `aureline_shell::learnability_glossary_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_learnability_glossary_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/learnability_glossary_stable_fixtures.rs`

## Why one disclosure record per switching cohort

A user switching from an incumbent editor asks, the moment a flow is in front of
them: *why does this matter now, what is this thing called here, and where do I
read more?* When the switching row, the docs/help browser, the command palette,
and the menu each answer with their own bespoke copy they drift — a glossary
chip points at a moved coordinate instead of a stable command, a why-now card
claims to be grounded in product truth when it is not, a tutorial funnel blocks
first useful work, or a Beta guided tour sits next to Stable cards and inherits
"Stable" by adjacency.

`aureline_shell::learnability_glossary_stable` mints one
`learnability_glossary_disclosure_record` per imported source ecosystem. Each
record is a genuine projection of the **live** shell code, not a parallel model:

- The switching cohort, incumbent terms, and docs/help node refs come from
  `aureline_shell::migration_corpus::seeded_migration_scoreboard`.
- Which cohort currently has live guided coverage comes from the live migration
  wizard (`seeded_migration_wizard_page`), exactly as the migration center
  decides which flow is a live apply session.
- The stable command anchors, the guided-affordance lifecycle marker, and the
  privacy posture come from
  `aureline_shell::learning_mode::{seeded_learning_mode_beta_manifest,
  seeded_learning_mode_beta_surface_projection}`.
- The anchor type is `aureline_shell::learning_mode::LearningTargetRef` reused
  verbatim, so the "cites command/file/symbol truth" bar is the same
  `is_stable_anchor` the learning surface already enforces.

## The pillars the record binds

1. **The why-now card (`why_now_card`).** An inline, `dismissible` card that does
   not block first useful work. It is *grounded* only when its `cited_target` is
   a stable command/file/symbol anchor (`is_command_file_symbol_anchor`). A
   cohort whose card cites docs only — until guided coverage lands — is narrowed
   with `why_now_card_not_grounded_in_truth`.

2. **The glossary chips (`glossary_chips`).** Each chip maps an `incumbent_term`
   to an `aureline_term` and cites a stable `anchor`. The cohort is narrowed with
   `glossary_anchors_not_stable` if any chip lacks a stable anchor.

3. **The contextual docs/help (`contextual_docs`).** `help_node_refs` are the
   docs/help nodes reachable in place (`opens_in_place`). A cohort that cites no
   docs node is narrowed with `contextual_docs_uncited`.

4. **The posture (`posture`).** `opt_in` is true, `blocks_first_useful_work` is
   false, and `preserves_exact_focus_return` is true with a canonical
   `focus_return_anchor_ref`. A layer that blocks first useful work is narrowed
   with `blocks_first_useful_work`; one that loses focus return is narrowed with
   `focus_return_not_preserved`.

These four pillars are the v8 stable bar: a learnability layer is stable **only
when** it cites command/file/symbol truth, preserves exact focus return, and
never forces a tutorial funnel before first useful work.

## Guided affordances are lifecycle-marked, never stable by adjacency

Every cohort carries the learning-mode guided affordance in
`guided_affordances`, marked `beta` (the learning module's real lifecycle), with
a `support_boundary` sentence and its marker visible in product, docs/help, and
the support export. The builder refuses to mint a record whose affordance hides
its marker on any surface (`GuidedAffordanceMarkerHidden`) or omits the support
boundary. Because a below-stable affordance is present, `honesty_marker_present`
is true on every row — the inline cards, chips, and contextual docs can be
Stable while the optional tour is honestly Beta.

## Learning state stays user-owned and local-first

`privacy` is projected from the learning manifest's progress posture:
`dismissals_user_owned`, `resume_entries_user_owned`, and
`learning_digest_user_owned` are true, and `repo_visible` / `telemetry_grade`
are false. The builder rejects any record that is repo-visible
(`LearningStateRepoVisible`), telemetry-grade (`LearningStateTelemetryGrade`),
or not user-owned (`LearningStateNotUserOwned`). Dismissals, resume entries, and
the learning digest never become repo-visible or telemetry-grade by accident.

## The public claim ceiling and the derived verdict

`claim_ceiling` is the set of assertions a row may publish. The builder refuses
to mint a record whose ceiling exceeds its evidence on any pillar
(`OverclaimsGlossaryAnchors`, `OverclaimsWhyNowGrounded`,
`OverclaimsContextualDocs`, `OverclaimsFocusReturn`, `OverclaimsNonBlocking`).
`stable_qualification` is *derived* from the evidence: a row that holds all four
pillars is `stable`; any failing pillar pushes it to `beta` and records a named
`narrowing_reason`. A narrowed row never inherits an adjacent green row.

## One model across surfaces, keyboard-first, in every layout

`surfaces` binds the switching-row, docs/help-row, and command-palette
projections to one identity and the same recovery action ids; `reopen_surfaces`
keeps docs/help, command palette, and support export. `routes` reaches the same
row from the switching row, docs/help browser, command palette, and a menu
command — every route keyboard-reachable, activating the same row, with a
canonical `aureline://` ref. `recovery_routes` always exposes Reopen-why-now,
Open-glossary, Open-contextual-docs, Dismiss-and-return-focus, and
Export-learning-support, plus Resume-guided-tour when an affordance is present.
`accessibility` keeps the row narration (which discloses the ecosystem), the
action labels in rendered order, and the recovery affordances reachable in
`normal`, `high_contrast`, and `zoomed` layouts.

## No account, no managed services

`available_without_account` and `available_without_managed_services` are true on
every row; the builder rejects any row that would bury the learnability layer
behind identity or managed services.

## The matrix

| Cohort | Ecosystem | Claim | Why-now grounded | Glossary chips | Guided tour |
| --- | --- | --- | --- | --- | --- |
| `vs_code_code_oss.json` | VS Code / Code-OSS | **stable** | yes | 5 | beta |
| `jetbrains_family.json` | JetBrains IDEs | beta (narrowed) | no | 5 | beta |
| `vim_neovim.json` | Vim / Neovim | beta (narrowed) | no | 5 | beta |
| `emacs.json` | Emacs | beta (narrowed) | no | 5 | beta |

The VS Code cohort is the wizard's live source, so its why-now card cites a live
command anchor and the cohort qualifies Stable; the other three share the same
stable glossary anchors, cited contextual docs, focus return, and non-blocking
posture but their why-now card cites docs only until guided coverage lands, so
they are narrowed to `beta` with `why_now_card_not_grounded_in_truth` instead of
inheriting the VS Code green row.
