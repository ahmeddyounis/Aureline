# M1 install-review fact grid seed

This page is the reviewer-facing landing page for the bounded prototype
that exposes **install-review facts** — publisher identity, permissions,
lifecycle, origin, and rollback posture — explicitly on one certified
install-like wedge before the action proceeds. The wedge lives at
[`crates/aureline-shell/src/install_review_fact_grid/`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs)
and is exercised by the unit and fixture tests in
[`crates/aureline-shell/src/install_review_fact_grid/tests.rs`](../../crates/aureline-shell/src/install_review_fact_grid/tests.rs).

The wedge is bounded:

- It covers **one** install-like action path — the extension-bearing
  prototype path. It does not stand up a marketplace, an install
  pipeline, an update channel, or a publisher-services product.
- It does not invent install / review vocabulary. Every closed enum
  used for publisher identity, origin source, lifecycle, declared
  permissions, the declared-vs-effective diff, the install decision,
  and the install-decision reason mirrors the upstream
  [`aureline_extensions::manifest_baseline`](../../crates/aureline-extensions/src/manifest_baseline/mod.rs)
  vocabulary verbatim. The wedge owns only two **new** closed
  vocabularies that the manifest baseline does not own — the
  [`ActivationBudgetClass`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs)
  and [`RollbackPostureClass`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs)
  rows — because the install / review surface MUST surface activation
  cost and rollback truth before commit.
- It does not silently admit a row whose facts are incomplete. A
  buggy caller that strips publisher identity, drops a declared-
  permission rationale, leaves origin source as
  `unknown_source_class`, or proposes admit alongside a widening
  attempt lands a typed
  [`InstallReviewFactGridInvariantViolation`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs)
  rather than rendering a clean Install button.

## What the wedge owns

- One canonical
  [`InstallReviewFactGridRecord`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs)
  data shape carrying, in stable order:
  - `record_kind`, `schema_version`, `prototype_label_token` (always
    `m1_prototype_install_review_fact_grid`);
  - the **publisher row** (identity ref, display label, trust tier
    class, publisher lifecycle state, signing key ref);
  - the **origin row** (manifest origin source class, origin source
    label);
  - the **lifecycle row** (extension lifecycle state class, host
    contract family class, manifest scope completeness class);
  - the **declared permissions** list (each carrying scope class,
    scope target, optional scope constraint, and a non-empty
    `rationale_label`);
  - the **declared-vs-effective diff** list (each row carrying the
    diff class — `unchanged`, `narrowed`, `denied`, `step_up_required`,
    or `widening_attempted_blocked` — and a narrowing reason label);
  - `widening_attempted_blocked_count` — the count of permission
    scopes the effective-permission summary refused to pass through;
  - the **activation budget row** (`eager_within_workspace_only`,
    `lazy_on_demand_only`, `lazy_on_event_subscription`,
    `restricted_step_up_required`, `denied_by_policy_pack`, or
    `not_applicable_install_denied`);
  - the **rollback posture row** (`clean_uninstall_and_state_purge`,
    `uninstall_with_user_state_retained`,
    `quarantine_only_pending_publisher_review`,
    `uninstall_blocked_pending_admin_review`, or
    `not_yet_admitted_no_rollback_needed`);
  - the **decision row** (install decision class, install decision
    reason class, decision summary);
  - an optional [`DegradedStateToken`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
    chrome chip;
  - the canonical claim-limit set
    (`single_bounded_wedge_only`, `no_marketplace_breadth`,
    `no_publisher_services`, `no_compatibility_policy_automation`)
    the chrome quotes verbatim under every card;
  - a typed
    [`InstallReviewFactGridInvariantViolation`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs)
    list so the chrome cannot quietly admit an extension whose facts
    are incomplete or whose decision contradicts the underlying
    truth.
- A deterministic
  [`InstallReviewFactGridRecord::render_plaintext()`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs)
  block downstream support exports and proof captures quote verbatim.

## Facts the wedge surfaces (one row per fact)

| Fact | Vocabulary source | Why it is rendered before commit |
|---|---|---|
| Publisher identity | `publisher_trust_tier_class`, `publisher_lifecycle_state_class` (from `aureline_extensions::manifest_baseline`) | The user must know whether they trust the publisher before granting any permission. |
| Origin / source | `manifest_origin_source_class` (`public_registry`, `private_registry`, `mirror`, `offline_bundle`, `vendored_local`, `unknown_source_class`) | The user must see whether the manifest came from a registry, an offline bundle, or a vendored local copy. |
| Lifecycle / compatibility | `extension_lifecycle_state_class`, `host_contract_family_class`, `manifest_scope_completeness_class` | Lifecycle (preview, deprecated, retired) and host-contract family govern compatibility and stability; incomplete manifest scope MUST block admit. |
| Declared permissions | `permission_scope_class` (13 closed members), each with a non-empty `rationale_label` | The user must see what the extension wants and why before granting it. |
| Declared-vs-effective diff | `effective_permission_diff_class` (`unchanged`, `narrowed`, `denied`, `step_up_required`, `widening_attempted_blocked`) | The chrome MUST surface the difference between declared and effective so the user is not surprised by a widened scope or a policy-pack narrowing. |
| Activation budget | `activation_budget_class` (eager / lazy / event / restricted / denied / N/A) | The user MUST know what activation cost an admitted extension will pay. |
| Rollback posture | `rollback_posture_class` (clean / user-state-retained / quarantine-only / admin-blocked / not-applicable) | The user MUST know whether they can remove the extension and what removing it actually does. |
| Install decision | `install_decision_class` (`admit`, `admit_with_step_up`, `review_only`, `denied`) + `install_decision_reason_class` (13 closed members) | The decision and the typed reason MUST be visible verbatim so the chrome cannot reword a denial as a generic warning. |

## Protected walks

The fixtures in
[`fixtures/install/m1_fact_grid_cases/`](../../fixtures/install/m1_fact_grid_cases/)
drive the protected walks through the tests in
[`crates/aureline-shell/src/install_review_fact_grid/tests.rs`](../../crates/aureline-shell/src/install_review_fact_grid/tests.rs):

1. **Verified publisher, complete manifest, immediate admit** —
   [`protected_walk_verified_publisher_admit.json`](../../fixtures/install/m1_fact_grid_cases/protected_walk_verified_publisher_admit.json).
   A verified-publisher row with a complete manifest, an attributed
   public-registry origin, two declared permissions each carrying a
   `rationale_label`, no widening attempts, lazy-on-demand activation
   budget, and a clean-uninstall rollback posture. The card renders an
   admit decision with no honesty markers and no invariant violations.
   Exercised by
   [`protected_walk_verified_publisher_renders_clean_admit_card`](../../crates/aureline-shell/src/install_review_fact_grid/tests.rs)
   and
   [`fixture_protected_walk_verified_publisher_admit_replays_into_the_wedge`](../../crates/aureline-shell/src/install_review_fact_grid/tests.rs).
2. **Unverified publisher, complete manifest, review-only** —
   [`protected_walk_unverified_publisher_review_only.json`](../../fixtures/install/m1_fact_grid_cases/protected_walk_unverified_publisher_review_only.json).
   An unverified-publisher row with the same fact-grid shape. The card
   surfaces an honest `review_only` decision with the typed
   `review_only_unverified_publisher` reason, carries the chrome
   `Limited` chip, and pins an `uninstall_with_user_state_retained`
   rollback posture so the user knows what removing the extension does.
   No invariant violations fire — review-only is an honest, distinct
   decision class.
   Exercised by
   [`protected_walk_unverified_publisher_renders_review_only_with_limited_chip`](../../crates/aureline-shell/src/install_review_fact_grid/tests.rs)
   and
   [`fixture_protected_walk_unverified_publisher_review_only_replays_into_the_wedge`](../../crates/aureline-shell/src/install_review_fact_grid/tests.rs).

## Failure drill — incomplete facts are refused, not simplified

[`failure_drill_incomplete_facts.json`](../../fixtures/install/m1_fact_grid_cases/failure_drill_incomplete_facts.json)
exercises the named failure drill from the spec ("review an install
with incomplete or risky facts and confirm the fact grid exposes the
missing authority/scope instead of simplifying it away"). A buggy
caller proposes admitting a row whose:

- `publisher_identity_ref` is empty and `publisher_trust_tier_class`
  is `anonymous_publisher_class`;
- `manifest_origin_source_class` is `unknown_source_class`;
- first declared permission's `rationale_label` is empty;
- `manifest_scope_completeness_class` is
  `incomplete_permission_rationale_missing` but `install_decision_class`
  is `admit`.

The drill confirms that the wedge:

- surfaces the typed
  `InstallReviewFactGridInvariantViolation::PublisherIdentityMissing`
  invariant against the offending extension identity;
- surfaces the typed
  `InstallReviewFactGridInvariantViolation::OriginSourceMissing`
  invariant against the offending extension identity;
- surfaces the typed
  `InstallReviewFactGridInvariantViolation::DeclaredPermissionRationaleMissing`
  invariant naming the offending scope class and target;
- surfaces the typed
  `InstallReviewFactGridInvariantViolation::ManifestValidationFindingsPresent`
  invariant carrying the count of upstream
  `validate_manifest_baseline_record` findings the chrome MUST
  re-surface;
- lights `has_invariant_violations` so the chrome MUST refuse to render
  a clean Install button;
- renders a summary line ending `INVARIANTS BLOCKED`.

Exercised by
[`failure_drill_incomplete_facts_refuses_to_render_clean_card`](../../crates/aureline-shell/src/install_review_fact_grid/tests.rs)
and
[`fixture_failure_drill_incomplete_facts_surfaces_typed_invariants`](../../crates/aureline-shell/src/install_review_fact_grid/tests.rs).

## Adjacent drills

- `widening_attempt_with_admit_decision_surfaces_typed_invariant` —
  the effective-permission summary records a non-zero
  `widening_attempted_blocked_count` but the install decision is
  `admit`. The wedge surfaces the typed
  `widening_attempted_without_denied_decision` invariant rather than
  letting the buggy caller commit.
- `widening_attempt_with_denied_decision_does_not_surface_widening_invariant`
  — the same widening shape paired with `denied` is honest; the
  invariant does not fire.
- `admit_with_not_yet_admitted_rollback_posture_surfaces_typed_invariant`
  — an admit decision paired with
  `not_yet_admitted_no_rollback_needed` lands the typed
  `admit_without_rollback_posture` invariant.
- `activation_budget_inconsistent_with_denied_decision_surfaces_invariant`
  — a denied decision paired with `eager_within_workspace_only` lands
  the typed `activation_budget_inconsistent_with_decision` invariant.
- `step_up_decision_requires_restricted_step_up_activation_budget` —
  an `admit_with_step_up` decision must be paired with
  `restricted_step_up_required` activation budget; any other pairing
  lands the same typed invariant.
- `missing_effective_permission_diff_surfaces_typed_invariant` — the
  effective-permission summary's diff list is empty even though the
  manifest declares at least one permission. The wedge surfaces the
  typed `effective_permission_diff_missing` invariant.

## Shared contracts the wedge projects against

The seed reuses these existing truth sources without forking:

- [`crates/aureline-extensions/src/manifest_baseline/mod.rs`](../../crates/aureline-extensions/src/manifest_baseline/mod.rs)
  — `ExtensionManifestBaselineRecord`,
  `EffectivePermissionBaselineRecord`,
  `ManifestInstallDecisionRecord`, and every closed enum used for
  publisher identity, origin source, lifecycle, permissions, diff
  classes, and install decisions. The wedge imports these types
  directly; it does not mint surface-local synonyms.
- [`docs/extensions/m1_permission_and_publisher_baseline.md`](../extensions/m1_permission_and_publisher_baseline.md)
  — the upstream reviewer-facing entry point for the manifest
  baseline, listing the same closed vocabularies verbatim.
- [`schemas/extensions/m1_extension_manifest.schema.json`](../../schemas/extensions/m1_extension_manifest.schema.json)
  — the frozen cross-tool boundary schema the manifest-baseline crate
  is kept in lock-step with.
- [`crates/aureline-shell/src/state_cards/degraded_state.rs`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
  — the shared `DegradedStateToken` chrome chips
  (`Warming` / `Cached` / `Limited` / `PolicyBlocked` / `Offline` / ...).

## Out of scope (deliberately)

- Marketplace breadth, install pipeline, update channels. The wedge
  renders one row's facts before commit; it does not own discovery,
  ranking, deployment, or rollback execution.
- Publisher services (verification, lifecycle transitions, key
  management). The wedge quotes the publisher trust tier and lifecycle
  state verbatim; it does not own how those values are set.
- Broad compatibility-policy automation. The wedge quotes the
  host-contract family verbatim; it does not own the compatibility
  rules engine.
- Provider-backed or remote-authoritative install flows; share /
  export / publish boundary moves. The wedge stays local to the one
  bounded prototype path.

## Validation command

```
cargo test -p aureline-shell --lib install_review_fact_grid
```
