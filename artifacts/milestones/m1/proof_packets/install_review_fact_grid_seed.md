# Proof packet: M1 install-review fact grid seed

Purpose: anchor proof captures for the M1 bounded prototype that
exposes **install-review facts** — publisher identity, declared
permissions, lifecycle, origin/source, activation budget, rollback
posture, and the typed install decision — explicitly on one certified
extension-bearing install-like wedge before the action proceeds. A
buggy caller that strips publisher identity, drops a declared-
permission rationale, leaves origin source as `unknown_source_class`,
or proposes admit alongside a widening attempt is rejected with typed
invariants the chrome MUST surface verbatim.

Reviewer landing page:
[`docs/ux/m1_install_review_fact_grid_seed.md`](../../../../docs/ux/m1_install_review_fact_grid_seed.md).

## Canonical sources

- Crate (wedge): `crates/aureline-shell/`
  - `src/install_review_fact_grid/mod.rs` —
    `InstallReviewFactGridWedge`, `InstallReviewFactGridRecord`, the
    `InstallReviewPublisherFacts` / `InstallReviewOriginFacts` /
    `InstallReviewLifecycleFacts` row shapes, the new
    `ActivationBudgetClass` and `RollbackPostureClass` closed
    vocabularies, the canonical
    `InstallReviewFactGridClaimLimit` set, the typed
    `InstallReviewFactGridInvariantViolation` rejection vocabulary,
    and the deterministic `render_plaintext()` block.
  - `src/install_review_fact_grid/tests.rs` — unit + fixture tests
    covering the verified-publisher protected walk, the unverified-
    publisher review-only protected walk, the named failure drill,
    and the adjacent drills on widening attempts, missing rollback
    posture, activation-budget inconsistency, step-up admission,
    missing diff lists, deterministic plaintext rendering, and serde
    round-trip.
- Crate (upstream manifest baseline): `crates/aureline-extensions/`
  - `src/manifest_baseline/mod.rs` —
    `ExtensionManifestBaselineRecord`,
    `EffectivePermissionBaselineRecord`,
    `ManifestInstallDecisionRecord`, and every closed enum the wedge
    re-uses (publisher trust tier, publisher lifecycle state,
    extension lifecycle state, manifest origin source, host contract
    family, permission scope class, effective permission diff class,
    manifest scope completeness, install decision class, install
    decision reason class). The wedge does not fork these
    vocabularies.
- Crate (shared degraded-state vocabulary): `crates/aureline-shell/`
  - `src/state_cards/degraded_state.rs` — `DegradedStateToken`
    (`Limited` / `PolicyBlocked` / `Warming` / ...). The wedge maps
    the chrome chip through this vocabulary instead of forking a
    local one.
- Fixtures: `fixtures/install/m1_fact_grid_cases/`
  - `protected_walk_verified_publisher_admit.json` — verified
    publisher, complete manifest, attributed public-registry origin,
    two declared permissions each carrying a rationale, no widening,
    lazy-on-demand activation, clean uninstall rollback, admit
    decision.
  - `protected_walk_unverified_publisher_review_only.json` —
    unverified publisher, complete manifest, honest review-only
    decision, `Limited` chrome chip,
    `uninstall_with_user_state_retained` rollback posture.
  - `failure_drill_incomplete_facts.json` — named failure drill: a
    buggy admit attempt with stripped publisher identity,
    `unknown_source_class` origin, missing rationale on a declared
    permission, and `incomplete_permission_rationale_missing`
    manifest scope completeness. The wedge surfaces every typed
    missing-facts invariant.
- Reviewer doc: `docs/ux/m1_install_review_fact_grid_seed.md`

## Upstream contracts the wedge projects against (without forking)

- `crates/aureline-extensions/src/manifest_baseline/mod.rs` — the
  authoritative manifest-baseline, effective-permission, and install
  decision records plus every closed publisher / origin / lifecycle /
  permission-scope / install-decision vocabulary the wedge consumes.
- `docs/extensions/m1_permission_and_publisher_baseline.md` — the
  upstream reviewer-facing landing page for the manifest baseline.
- `schemas/extensions/m1_extension_manifest.schema.json` — the frozen
  cross-tool boundary schema the manifest-baseline crate is kept in
  lock-step with.
- `crates/aureline-shell/src/state_cards/degraded_state.rs` — the
  shared `DegradedStateToken` chrome chips.

## Protected walks

1. **Verified publisher, complete manifest, immediate admit.** Open
   the fact grid against a verified-publisher row with a complete
   manifest, an attributed public-registry origin, two declared
   permissions each carrying a `rationale_label`, no widening
   attempts, a lazy-on-demand activation budget, and a
   `clean_uninstall_and_state_purge` rollback posture. The card
   renders with `install_decision_class = admit`,
   `install_decision_reason_class = admitted_no_violation`, no honesty
   markers, and no invariant violations.
2. **Unverified publisher, complete manifest, review-only.** Open the
   fact grid against an unverified-publisher row with the same
   fact-grid shape. The card surfaces an honest
   `install_decision_class = review_only` decision with
   `install_decision_reason_class = review_only_unverified_publisher`,
   carries the chrome `Limited` chip, and pins an
   `uninstall_with_user_state_retained` rollback posture. No invariant
   violations fire; review-only is an honest, distinct decision class.

Evidence:

- `crates/aureline-shell/src/install_review_fact_grid/tests.rs::protected_walk_verified_publisher_renders_clean_admit_card`
- `crates/aureline-shell/src/install_review_fact_grid/tests.rs::protected_walk_unverified_publisher_renders_review_only_with_limited_chip`
- `crates/aureline-shell/src/install_review_fact_grid/tests.rs::fixture_protected_walk_verified_publisher_admit_replays_into_the_wedge`
- `crates/aureline-shell/src/install_review_fact_grid/tests.rs::fixture_protected_walk_unverified_publisher_review_only_replays_into_the_wedge`
- Fixtures:
  `fixtures/install/m1_fact_grid_cases/protected_walk_verified_publisher_admit.json`,
  `fixtures/install/m1_fact_grid_cases/protected_walk_unverified_publisher_review_only.json`

## Failure drill — incomplete or risky facts are refused, not simplified

A buggy install / review caller proposes admitting a row whose
publisher identity has been stripped, whose origin source is
`unknown_source_class`, and whose first declared permission's
`rationale_label` is empty. The wedge MUST surface every typed
missing-facts invariant rather than collapse them into a generic
warning. The card lights `has_invariant_violations` and the summary
line ends `INVARIANTS BLOCKED`.

Evidence:

- `crates/aureline-shell/src/install_review_fact_grid/tests.rs::failure_drill_incomplete_facts_refuses_to_render_clean_card`
- `crates/aureline-shell/src/install_review_fact_grid/tests.rs::fixture_failure_drill_incomplete_facts_surfaces_typed_invariants`
- Fixture:
  `fixtures/install/m1_fact_grid_cases/failure_drill_incomplete_facts.json`

## Adjacent drills — facts stay distinct; chrome cannot widen scope after preview

- `widening_attempt_with_admit_decision_surfaces_typed_invariant` —
  a non-zero `widening_attempted_blocked_count` paired with `admit`
  lands the typed `widening_attempted_without_denied_decision`
  invariant. The same widening shape paired with `denied` is honest;
  the invariant does not fire.
- `admit_with_not_yet_admitted_rollback_posture_surfaces_typed_invariant`
  — admit paired with `not_yet_admitted_no_rollback_needed` lands the
  typed `admit_without_rollback_posture` invariant.
- `activation_budget_inconsistent_with_denied_decision_surfaces_invariant`
  — denied paired with `eager_within_workspace_only` lands the typed
  `activation_budget_inconsistent_with_decision` invariant.
- `step_up_decision_requires_restricted_step_up_activation_budget` —
  `admit_with_step_up` paired with any activation budget other than
  `restricted_step_up_required` lands the same invariant.
- `missing_effective_permission_diff_surfaces_typed_invariant` — an
  empty `declared_vs_effective_diff` list against a manifest that
  declares at least one permission lands the typed
  `effective_permission_diff_missing` invariant so the chrome cannot
  silently hide what changes between declared and effective.

## Validation command

```
cargo test -p aureline-shell --lib install_review_fact_grid
```

## Evidence storage

- Crate sources:
  `crates/aureline-shell/src/install_review_fact_grid/`,
  `crates/aureline-shell/src/state_cards/degraded_state.rs`,
  `crates/aureline-extensions/src/manifest_baseline/mod.rs`
- Reviewer doc: `docs/ux/m1_install_review_fact_grid_seed.md`
- Fixtures: `fixtures/install/m1_fact_grid_cases/`
