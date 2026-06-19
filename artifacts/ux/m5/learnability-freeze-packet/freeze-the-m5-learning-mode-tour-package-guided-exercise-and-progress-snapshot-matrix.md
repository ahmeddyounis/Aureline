# Frozen M5 learnability lane — vocabulary, matrix, and explain-versus-do evidence

Reviewer-facing evidence packet for the frozen M5 learnability lane. One
controlled vocabulary and one family-by-term matrix freeze how every claimed M5
feature family routes its learning-mode, guided-tour, guided-exercise,
educational-AI, and progress surfaces. Later implementation rows reuse these
frozen terms and lane refs instead of inventing feature-local coachmarks or
parallel onboarding state. A row that cannot prove the posture is narrowed below
Stable with a named reason rather than inheriting an adjacent green row.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/help/m5-learnability-lane.schema.json`](../../../../schemas/help/m5-learnability-lane.schema.json)
- Fixture: [`/fixtures/help/m5/learnability-regression/m5_learnability_lane_freeze.json`](../../../../fixtures/help/m5/learnability-regression/m5_learnability_lane_freeze.json)
- Public doc: [`/docs/m5/learning-mode-and-guided-exercises.md`](../../../../docs/m5/learning-mode-and-guided-exercises.md)
- Reuses: [`/schemas/learning/guided-learning-contracts.schema.json`](../../../../schemas/learning/guided-learning-contracts.schema.json), [`/schemas/learning/m5-feature-family-learning-rails.schema.json`](../../../../schemas/learning/m5-feature-family-learning-rails.schema.json)
- Typed source: `aureline_learning::freeze_m5_learnability_lane`
- Headless emitter: `aureline_learning_m5_learnability_freeze`
- Test: `cargo test -p aureline-learning freeze_m5_learnability_lane`

## The frozen controlled vocabulary

These nine terms are the only vocabulary M5 feature families may use to name a
learnability surface. Each entry pins an explain-versus-do posture, a
mutation-path class, and a data-ownership class; none may change an authority or
trust boundary.

| Term | Explain/do | Mutation path | Ownership |
|---|---|---|---|
| `learning_mode` | read_only | read_only_no_mutation | user_owned_local_first |
| `tour_package` | apply_requires_approval | preview_approval_required | user_owned_local_first |
| `guided_exercise` | apply_requires_approval | preview_approval_required | user_owned_local_first |
| `glossary_pack` | read_only | read_only_no_mutation | user_owned_local_first |
| `contextual_why_now_card` | read_only | read_only_no_mutation | user_owned_local_first |
| `educational_ai` | apply_requires_approval | preview_approval_required | user_owned_local_first |
| `practice_sandbox_indicator` | read_only | read_only_no_mutation | user_owned_local_first |
| `learning_digest` | read_only | read_only_no_mutation | user_owned_local_first |
| `progress_snapshot` | read_only | read_only_no_mutation | user_owned_local_first |

## The lane matrix

The matrix is the full Cartesian product of nine claimed M5 families against the
nine frozen terms — eighty-one lane rows. Each per-family term (glossary,
tour, exercise, why-now card, practice/sandbox indicator, progress snapshot)
routes through that family's learning bundle; each cross-cutting term
(`learning_mode`, `educational_ai`, `learning_digest`) routes through one shared
canonical lane across every family, proving no family forks its own onboarding
state.

| Family | Rows | Verdict | Narrowing reason |
|---|---|---|---|
| `notebook` | 9 | **qualified_stable** | — |
| `request_workspace` | 9 | **qualified_stable** | — |
| `database_workspace` | 9 | **qualified_stable** | — |
| `profiler_trace` | 9 | **qualified_stable** | — |
| `docs_browser` | 9 | **qualified_stable** | — |
| `preview` | 9 | **narrowed_beta** | `tour_package` and `guided_exercise` learning packs not yet mirror-synced |
| `template_scaffold` | 9 | **qualified_stable** | — |
| `companion` | 9 | **qualified_stable** | — |
| `sync_offboarding` | 9 | **qualified_stable** | — |

**Overall freeze verdict: narrowed_beta** — the two `preview` pack-backed rows
each propagate to the overall verdict; every other row qualifies Stable
individually. The narrowing is honest about a missing mirror-sync proof, not a
broken experience.

## What this packet proves

1. **One vocabulary, frozen.** All nine terms are present exactly once, each with
   a fixed definition, explain-versus-do posture, mutation-path class, and
   `user_owned_local_first` ownership. `authority_boundary_change_allowed` is
   false on every entry — no learnability term may widen authority or trust.

2. **One lane per surface, no feature-local forks.** Every claimed family has a
   row for every term, and each row names a single `canonical_lane_ref`. The
   three cross-cutting terms resolve to exactly one shared canonical lane across
   all nine families; validation fails if any family forks its own.

3. **No hidden coachmarks, no private mutation paths.** Every row sets
   `command_backed: true`, `hidden_feature_local_coachmark: false`, and
   `private_mutation_path: false`. No row may carry `hidden_direct_mutation`.

4. **Explain stays separate from do.** Tours, exercises, and educational AI sit at
   `apply_requires_approval` with `preview_approval_required`; everything else is
   `read_only`. The frozen `educational_ai_boundary` asserts
   `explain_and_do_separate`, `do_requires_same_preview_approval`, and
   `can_mutate_live_state_directly: false`, with a practice/sandbox indicator
   present so rehearsal is never confused with live work.

5. **User-owned, local-first, support-export safe.** Every row is
   `user_owned_local_first` and proves support-export parity:
   `inspectable_in_support_export`, `matches_in_product_state`, and
   `carries_no_credential_bodies` are all true.

6. **Offline and mirror parity, explicitly labeled.** Every row's
   `mirror_parity.silent_dead_link_on_stale` is false and
   `explicit_freshness_disclosed` is true. The `preview` family is honest about
   not yet being mirror-synced (`available_on_mirror: false`,
   `local_only_disclosed`) and is narrowed rather than dead-linking.

## Failure / recovery drill

The validator (`validate_m5_learnability_lane`) and its tests exercise the
narrowing paths: a missing vocabulary term, a forked cross-cutting lane, a hidden
coachmark, a private mutation path, telemetry-grade ownership, and an educational
AI that can mutate live state all produce typed errors. Run:

```sh
cargo test -p aureline-learning freeze_m5_learnability_lane
cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_freeze -- validate
```

## Regenerating the fixture

The fixture is emitted from the typed source so it cannot drift:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_freeze \
  -- emit-fixture fixtures/help/m5/learnability-regression/m5_learnability_lane_freeze.json
```
