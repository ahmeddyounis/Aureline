# Proof packet: M1 typed permission prompts seed

Purpose: anchor proof captures for the M1 bounded prototype that
mints **typed permission prompts** on one certified ecosystem-bearing
install-review wedge. The prompt answers, before any approve action is
offered: *who is asking*, *who owns the decision*, *what boundary is
being crossed*, *why it is needed*, *what changes if allowed*, *what
still works if denied*, and *how long the grant lasts*. A buggy
caller that strips actor identity, drops the deny path, omits the
persistence label, or attempts to render an approve action on top of a
blocked install-review card is rejected with typed invariants the
chrome MUST surface verbatim.

Reviewer landing page:
[`docs/ux/m1_typed_permission_prompt.md`](../../../../docs/ux/m1_typed_permission_prompt.md).

## Canonical sources

- Crate (wedge): `crates/aureline-shell/`
  - `src/permission_prompts/mod.rs` —
    `PermissionPromptWedge`, `TypedPermissionPromptRecord`, the
    `PermissionPromptRequester` / `PermissionPromptAuthorityOwner` /
    `PermissionPromptScope` / `PermissionPromptConsequence` /
    `PermissionPromptDenialBranch` / `PermissionPromptQuestions`
    block shapes, the new closed `RequesterClass` /
    `AuthorityIssuerClass` / `ScopeFilterClass` / `GrantScopeClass` /
    `DegradedCapabilityClass` / `DecisionActionRole` /
    `PermissionPromptDecisionState` vocabularies, the canonical
    `TypedPermissionPromptClaimLimit` set, the typed
    `TypedPermissionPromptInvariantViolation` rejection vocabulary,
    and the deterministic `render_plaintext()` block.
  - `src/permission_prompts/tests.rs` — 15 unit and fixture tests
    covering the verified-publisher approve protected walk, the
    unverified-publisher review-only deny protected walk, the named
    failure drill, and the adjacent drills on approving against a
    denied install decision, suppressing the deny action, approving
    while blocked, grant-persistence-label inconsistency, the
    policy-service issuer adding the request-admin-review action,
    deterministic plaintext rendering, and serde round-trip.
- Crate (upstream install-review fact grid):
  `crates/aureline-shell/src/install_review_fact_grid/mod.rs` — the
  `InstallReviewFactGridRecord` the typed permission prompt is bound
  to. The wedge does not fork the upstream card's vocabularies.
- Crate (upstream manifest baseline):
  `crates/aureline-extensions/src/manifest_baseline/mod.rs` —
  `ExtensionManifestBaselineRecord`,
  `EffectivePermissionBaselineRecord`,
  `ManifestInstallDecisionRecord`, and every closed enum the wedge
  re-uses (publisher trust tier, publisher lifecycle state, extension
  lifecycle state, manifest origin source, host contract family,
  permission scope class, effective permission diff class, manifest
  scope completeness, install decision class, install decision
  reason class).
- Crate (shared degraded-state vocabulary):
  `crates/aureline-shell/src/state_cards/degraded_state.rs` —
  `DegradedStateToken` (`Limited` / `PolicyBlocked` / `Warming` /
  ...). The wedge maps the chrome chip through this vocabulary
  instead of forking a local one.
- Fixtures: `fixtures/permissions/publish_or_ecosystem_cases/`
  - `protected_walk_verified_publisher_approve.json` — verified
    publisher (Acme Labs / prose-helper) install-review row plus a
    typed permission prompt that names actor, issuer, scope,
    consequence, deny path, and a workspace-scope persistence label;
    user approves; record renders clean approved decision state.
  - `protected_walk_unverified_publisher_deny.json` — unverified
    publisher (Indie Author / quick-notes) review-only row plus a
    typed permission prompt that suppresses approve and offers deny
    only; user denies; record renders clean denied decision state
    with the `Limited` chrome chip.
  - `failure_drill_incomplete_authority_facts.json` — named failure
    drill: a buggy caller mints a prompt against a blocked
    install-review card, strips requester identity / issuer source /
    scope target / persistence label / deny-path label, leaves two
    prompt questions unanswered, and force-offers approve. The
    wedge surfaces every typed missing-facts invariant plus
    `approve_offered_with_blocked_install_review`.
- Reviewer doc: `docs/ux/m1_typed_permission_prompt.md`

## Upstream contracts the wedge projects against (without forking)

- `crates/aureline-extensions/src/manifest_baseline/mod.rs` — the
  authoritative manifest-baseline, effective-permission, and install
  decision records plus every closed publisher / origin / lifecycle /
  permission-scope / install-decision vocabulary the wedge consumes.
- `crates/aureline-shell/src/install_review_fact_grid/mod.rs` — the
  authoritative install-review fact grid the typed prompt is bound to.
- `docs/ux/trust_prompt_contract.md` — the upstream trust / policy /
  permission prompt contract. The wedge's closed vocabularies are
  deliberate subsets of the upstream contract; growing them is
  additive.
- `docs/ux/m1_install_review_fact_grid_seed.md` — the upstream M1
  reviewer-facing landing page for the install-review fact grid.
- `crates/aureline-shell/src/state_cards/degraded_state.rs` — the
  shared `DegradedStateToken` chrome chips.

## Protected walks

1. **Verified publisher, complete manifest, user approves.** The shell
   opens an install-review fact-grid card for a verified publisher
   (Acme Labs / prose-helper). The typed permission prompt names the
   extension as the requester, names the shell as the authority
   owner, asks for workspace-scope `current_root` access to
   `workspace:/docs/**` and the user's AI provider, and explains that
   local editing and manifest inspection continue if denied. The
   record carries the prototype-label chip, the canonical
   claim-limit set, the upstream `install_decision_class = admit`,
   and a `decision_state = pending` initial state with approve / deny
   / details actions all offered. The user approves; the record
   transitions to a clean `decision_state = approved` state with no
   invariants.
2. **Unverified publisher, review-only, user denies.** The shell
   opens an install-review card whose `install_decision_class` is
   `review_only` with a `Limited` chrome chip. The typed permission
   prompt suppresses the approve action (auto-admit is forbidden for
   review-only rows), offers deny and details, and explains that the
   manifest remains visible for review. The user denies; the record
   transitions to a clean `decision_state = denied` state with no
   invariants.

Evidence:

- `crates/aureline-shell/src/permission_prompts/tests.rs::protected_walk_verified_admit_pending_card_offers_approve_and_deny`
- `crates/aureline-shell/src/permission_prompts/tests.rs::protected_walk_verified_admit_then_approve_renders_clean_approved_card`
- `crates/aureline-shell/src/permission_prompts/tests.rs::protected_walk_verified_admit_then_deny_renders_clean_denied_card`
- `crates/aureline-shell/src/permission_prompts/tests.rs::protected_walk_unverified_review_only_suppresses_approve`
- `crates/aureline-shell/src/permission_prompts/tests.rs::fixture_protected_walk_verified_publisher_approve_replays_into_the_wedge`
- `crates/aureline-shell/src/permission_prompts/tests.rs::fixture_protected_walk_unverified_publisher_deny_replays_into_the_wedge`
- Fixtures:
  `fixtures/permissions/publish_or_ecosystem_cases/protected_walk_verified_publisher_approve.json`,
  `fixtures/permissions/publish_or_ecosystem_cases/protected_walk_unverified_publisher_deny.json`

## Failure drill — incomplete authority facts are refused, not simplified

A buggy caller proposes a typed permission prompt where the requester
identity is stripped, the issuer source is empty, the scope target
label is empty, the persistence label is empty, the deny-path label
is empty, two of the six required prompt questions are unanswered,
the underlying install-review card carries blocking invariants, and
the prompt force-offers an approve action anyway. The wedge MUST
surface every typed invariant rather than collapse to a generic
"Allow?":

- `requester_identity_missing`
- `authority_owner_missing`
- `scope_missing`
- `grant_persistence_missing`
- `deny_path_missing`
- `prompt_question_unanswered`
- `approve_offered_with_blocked_install_review`

The card lights `has_invariant_violations` and the summary line ends
`INVARIANTS BLOCKED`.

Evidence:

- `crates/aureline-shell/src/permission_prompts/tests.rs::failure_drill_incomplete_authority_facts_refuses_to_collapse_to_generic_copy`
- `crates/aureline-shell/src/permission_prompts/tests.rs::fixture_failure_drill_incomplete_authority_facts_surfaces_typed_invariants`
- Fixture:
  `fixtures/permissions/publish_or_ecosystem_cases/failure_drill_incomplete_authority_facts.json`

## Adjacent drills — facts stay distinct; chrome cannot widen scope after preview

- `approve_offered_against_denied_decision_surfaces_typed_invariant` —
  the upstream fact-grid decision is `denied` but the prompt
  force-offers approve. The typed
  `approve_offered_against_denied_decision` invariant fires.
- `suppressed_deny_action_surfaces_typed_invariant` — the prompt
  drops the deny action. The typed `no_deny_action_path` invariant
  fires; a typed prompt MUST always offer a deny path.
- `approved_while_blocked_surfaces_typed_invariant` — the caller
  marks the prompt approved while requester identity is missing. The
  typed `approved_while_blocked` invariant fires alongside the
  requester invariant.
- `grant_persistence_label_inconsistency_surfaces_typed_invariant` —
  the grant-scope token is `workspace` but the persistence label
  rendered to the user says "Session". The typed
  `grant_persistence_inconsistent` invariant fires.
- `policy_service_issuer_adds_request_admin_review_action` — when
  the issuer is `policy_service`, the typed `request_admin_review`
  action is added so the user can forward the request to the admin
  path without the chrome inventing a fake local approve action.

## Validation command

```
cargo test -p aureline-shell --lib permission_prompts
```

## Evidence storage

- Crate sources:
  `crates/aureline-shell/src/permission_prompts/`,
  `crates/aureline-shell/src/install_review_fact_grid/`,
  `crates/aureline-shell/src/state_cards/degraded_state.rs`,
  `crates/aureline-extensions/src/manifest_baseline/mod.rs`
- Reviewer doc: `docs/ux/m1_typed_permission_prompt.md`
- Fixtures: `fixtures/permissions/publish_or_ecosystem_cases/`
