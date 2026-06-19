# Learning-mode profiles — release evidence

Reviewer-facing evidence packet for the M5 learning-mode-profile lane. A
learning-mode profile is the **dial** over Aureline's learnability surfaces: an
opt-in, user-owned preset that tunes tip intensity, jargon level, educational-AI
explanation posture, and a mutation guardrail, and that records dismissals,
bookmarks, lifecycle controls, and a change history. A profile never changes
authority, ownership, trust, or the command graph; it never traps an expert; and
its educational AI keeps *explain* and *do* separate. A profile that cannot prove
that posture is explicitly narrowed below Stable with a named reason rather than
inheriting an adjacent green row.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/help/m5-learning-mode-profiles.schema.json`](../../../../schemas/help/m5-learning-mode-profiles.schema.json)
- Fixture: [`/fixtures/help/m5/learning-mode-profiles/m5_learning_mode_profiles.json`](../../../../fixtures/help/m5/learning-mode-profiles/m5_learning_mode_profiles.json)
- Public doc: [`/docs/m5/learning-mode-profiles.md`](../../../../docs/m5/learning-mode-profiles.md)
- Aligns with: [`/schemas/learning/m5-feature-family-learning-rails.schema.json`](../../../../schemas/learning/m5-feature-family-learning-rails.schema.json) (shared surface-family taxonomy) and [`/schemas/learning/guided-learning-contracts.schema.json`](../../../../schemas/learning/guided-learning-contracts.schema.json) (shared posture/verdict vocabulary)
- Typed source: `aureline_learning::learning_mode_profiles`
- Headless emitter: `aureline_learning_m5_learning_mode_profiles`
- Test: `cargo test -p aureline-learning learning_mode_profiles`

## The profile matrix

| Profile | Scope | Verdict | Preset | Tips | Jargon | AI posture | Sync | Narrowing reason |
|---|---|---|---|---|---|---|---|---|
| `expert_minimal_user_local` | user_local | **qualified_stable** | expert_minimal | silent_inline_only | expert | explain_then_prepare_preview | local_only | — |
| `balanced_default_user_local` | user_local | **qualified_stable** | balanced_default | gentle_hint | intermediate | explain_then_prepare_preview | local_only | — |
| `guided_learner_user_local` | user_local | **qualified_stable** | guided_learner | prompted_acknowledge | beginner | preview_only_after_explicit_do | local_only | — |
| `balanced_workspace_opt_in` | workspace_opt_in | **narrowed_beta** | balanced_default | gentle_hint | intermediate | explain_then_prepare_preview | portable_profile_synced | portable-profile sync may lag across machines (disclosed) |

**Overall manifest verdict: narrowed_beta** — the workspace-opt-in profile's
disclosed portable-profile sync propagates to the overall verdict; all three
user-local profiles ship Stable individually.

## What this packet proves

1. **Learning mode turns on, off, resets, and narrows — without touching the
   command graph.** Every profile exposes the full control set (enable, disable,
   pause, snooze, resume, reset, narrow); the four required controls (enable,
   disable, reset, narrow) are checked for presence. Each control is
   command-backed (`command_id_ref`), keyboard reachable (`keyboard_shortcut_ref`),
   reversible, inspectable, never a silent write, and never mutates the workspace.
   `authority_boundary_change_allowed` is false and `command_graph_unchanged` is
   true on every profile, so the dial never moves authority or the command graph.

2. **Per-user and per-workspace scope stay explicit and never leak into the
   repo.** Each `scope_binding` is either `user_local` or `workspace_opt_in`; a
   workspace profile must set `opt_in_explicit: true`. `repo_committed` and
   `shared_with_collaborators` are false on every profile, and the schema's
   `if/then` forbids a workspace scope without an explicit opt-in. Onboarding
   preferences cannot silently become repo state or follow a collaborator.

3. **Educational-AI posture and mutation guardrails are inspectable and fenced.**
   Each profile names an `ai_explanation_posture` and a `mutation_guardrail`. Any
   do-capable posture (`explain_then_prepare_preview`,
   `preview_only_after_explicit_do`) MUST set
   `educational_ai_uses_standard_preview_approval: true` and pick a guardrail that
   still fences the do — the schema enforces both via `if/then`, and the validator
   reports an unfenced do as a hard violation. There is deliberately no "unfenced"
   guardrail value, so a profile can never structurally permit a direct live-state
   write.

4. **Experts are never trapped.** `blocking_onboarding_allowed` is false on every
   profile. Learner presets keep `explain_before_act_default: true`; only
   `expert_minimal` may turn forced pre-explanation off — and even then it keeps
   the mutation fence and the standard preview/approval model.

5. **Progress is user-owned, reversible, and private.** `dismissals` are
   reversible, user-owned, and scope-following; `bookmarks` are user-owned;
   `change_history` events are user-initiated and inspectable in support export.
   No profile shares progress with the repo or with collaborators.

6. **State is inspectable, not hidden in overlays.** Every profile's `exposure`
   is visible in settings, Help/About, diagnostics, and support export, and
   `hidden_in_transient_overlay_only` is false. Settings/Help/diagnostics/support
   read this manifest instead of rephrasing learning-mode state by hand.

7. **Sync is honest.** Local-only state is live-authoritative. A
   `portable_profile_synced` profile MUST set `sync_disclosed: true`; an
   undisclosed synced profile is a masquerade that the validator narrows to
   Preview and the schema rejects outright. A disclosed sync is an honest,
   user-chosen deviation that narrows to Beta with a named reason.

## How the verdict is derived

`derive_learning_mode_profile_verdict` folds each profile's authority, ownership,
command-graph, blocking-onboarding, explain-before-act, educational-AI-fence,
scope, sync-disclosure, exposure, dismissal/bookmark, control, and change-event
evidence into the strictest verdict. Hard safety violations narrow to
`narrowed_preview`; a disclosed portable-profile sync narrows to `narrowed_beta`.
The manifest's `overall_verdict` is the narrowest across all profiles. Stored
verdicts are re-derived and checked by `validate_m5_learning_mode_profiles`, so a
hand-edited fixture that disagrees with its own evidence fails validation.

## How to reproduce

```sh
cargo test -p aureline-learning learning_mode_profiles
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_profiles -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_profiles -- summary
```
