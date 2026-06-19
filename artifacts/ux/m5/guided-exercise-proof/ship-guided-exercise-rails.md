# Guided-exercise rails — release evidence

Reviewer-facing evidence packet for the M5 guided-exercise-rail lane. Every M5
feature family ships a **command-backed** guided-exercise rail whose steps carry
success criteria, point at stable target files/surfaces, expose inspectable
hint/reveal/reset/skip controls, keep explain and do separate, and route every
apply through the same command id, preview sheet, approval path, and trust/policy
check Aureline uses outside learning mode. A rail that cannot prove that posture
is explicitly narrowed below Stable with a named reason rather than inheriting an
adjacent green row.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/help/m5-guided-exercise-rails.schema.json`](../../../../schemas/help/m5-guided-exercise-rails.schema.json)
- Fixture: [`/fixtures/help/m5/guided-exercise-rails/m5_guided_exercise_rails.json`](../../../../fixtures/help/m5/guided-exercise-rails/m5_guided_exercise_rails.json)
- Public doc: [`/docs/help/m5/guided-exercise-rails.md`](../../../../docs/help/m5/guided-exercise-rails.md)
- Aligns with: [`/schemas/help/m5-tour-and-glossary-packages.schema.json`](../../../../schemas/help/m5-tour-and-glossary-packages.schema.json) (shared stable-target taxonomy) and [`/schemas/learning/guided-learning-contracts.schema.json`](../../../../schemas/learning/guided-learning-contracts.schema.json) (shared posture vocabulary)
- Typed source: `aureline_learning::guided_exercise_rails`
- Headless emitter: `aureline_learning_m5_guided_exercise_rails`
- Test: `cargo test -p aureline-learning guided_exercise`

## The rail matrix

| Family | Verdict | Freshness | Shape | Narrowing reason |
|---|---|---|---|---|
| `notebook` | **qualified_stable** | live_authoritative | explain → sandbox → apply | — |
| `request_workspace` | **qualified_stable** | live_authoritative | explain → apply | — |
| `database_workspace` | **qualified_stable** | live_authoritative | explain → sandbox → apply | — |
| `profiler_trace` | **qualified_stable** | live_authoritative | explain → apply | — |
| `docs_browser` | **qualified_stable** | mirror_synced_disclosed | explain → explain (read-only) | — |
| `preview` | **narrowed_beta** | local_only_disclosed | explain → apply | not yet mirror-synced |
| `template_scaffold` | **qualified_stable** | live_authoritative | explain → sandbox → apply | — |
| `companion` | **narrowed_beta** | cached_disclosed | explain → apply | served from a cached (not live) revision |
| `sync_offboarding` | **qualified_stable** | live_authoritative | explain → apply | — |

**Overall manifest verdict: narrowed_beta** — the `preview` mirror-parity gap and
the `companion` cached revision each propagate to the overall verdict; all other
families ship Stable individually.

## What this packet proves

1. **A current step, success criteria, and target file/surface.** Every step
   carries at least one `stable_targets` ref (`command_id`, `file_object_id`,
   `symbol_object_id`, `docs_node_id`, `graph_node_id`, or `surface_object_id`)
   and at least one `success_criteria` entry with a deterministic `check_ref`. A
   step with an empty `stable_targets` is reported as `coordinate_only` and a step
   with no criterion is rejected — a rail can never depend on pixel positions or
   leave "done" undefined.

2. **Explain and do stay separate.** Each step declares a `step_kind`. An
   `explain` or `prepare_practice` step that sets a `mutation_target` touching
   real workspace state is reported as an educational-step escalation and fails
   validation (and the schema's `if/then` forbids it outright). Only an
   `apply_with_approval` step may mutate the real workspace.

3. **Apply is command-backed through the standard model.** Every
   `apply_with_approval` step's `command_backing` names a command id, preview
   sheet, approval path, and trust/policy check and sets
   `uses_standard_command_model: true`. An apply step missing any of those — a
   tutorial-only shortcut that would create hidden authority — narrows below
   Stable and fails validation. The schema enforces the same via `if/then`.

4. **Reversibility preferred, blast radius labelled.** Each step's
   `mutation_target` labels whether it is `no_mutation`, `sandboxed_local_reversible`,
   `workspace_reversible_approved`, or `workspace_irreversible_approved`. An
   irreversible real-workspace mutation is honest but narrows below Stable.
   `SandboxPreference` records that effects are reversible by default and that any
   real-workspace mutation requires explicit opt-in.

5. **Hint/reveal/reset/skip are inspectable, keyboard reachable, restart-safe,
   and non-mutating.** Every step exposes all four controls. Each is checked for
   `inspectable: true`, a `keyboard_shortcut_ref`, `restart_safe: true`, and
   `mutates_workspace: false`. A control that is trapped in a modal, not keyboard
   reachable, not restart-safe, or that mutates workspace state fails validation.

6. **Progress is user-owned and resumable.** Each rail's `progress` is
   `survives_restart`, `resumable`, `user_owned_local`, and not `shared_with_repo`;
   a fresh rail resolves to its first step and the index can never point past the
   last step. Progress shared with the repository narrows below Stable.

7. **Freshness disclosed, localization preserves identity.** Each rail's
   `freshness_state` agrees with its `mirror_parity.freshness_label`, every
   non-live state sets `explicit_freshness_disclosed: true`, and each rail carries
   `fr-FR` and `ja-JP` locale overlays that localize labels without touching target
   refs or citations. The export/reopen round-trip test confirms the
   target/citation fingerprints survive serialization unchanged.

## How the verdict is derived

`derive_exercise_rail_verdict` folds each rail's freshness, mirror-parity,
sandbox, privacy, progress, accessibility, locale-overlay, citation,
explain-versus-do, command-backing, mutation, and control evidence into the
strictest verdict. The manifest's `overall_verdict` is the narrowest across all
rails. Stored verdicts are re-derived and checked by
`validate_m5_guided_exercise_rails`, so a hand-edited fixture that disagrees with
its own evidence fails validation.

## How to reproduce

```sh
cargo test -p aureline-learning guided_exercise
cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_rails -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_rails -- summary
```
