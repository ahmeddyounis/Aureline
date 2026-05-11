# M1 typed permission prompt seed

This page is the reviewer-facing landing page for the bounded prototype
that mints **typed permission prompts** on one certified
ecosystem-bearing install-review path. The wedge lives at
[`crates/aureline-shell/src/permission_prompts/`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
and is exercised by the unit and fixture tests in
[`crates/aureline-shell/src/permission_prompts/tests.rs`](../../crates/aureline-shell/src/permission_prompts/tests.rs).

The wedge is bounded:

- It runs on **one** ecosystem-bearing path — the certified extension
  install-review wedge owned by
  [`crates/aureline-shell/src/install_review_fact_grid/`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs).
  Org policy packs, admin approval consoles, publish/promote/rollback
  prompts, AI-tool prompts, and remote-attach prompts stay out of
  scope.
- It does not invent install / authority / permission vocabulary. The
  declared-permission rows, the install-decision class, and the
  install-decision reason class are projected verbatim from the
  upstream
  [`aureline_extensions::manifest_baseline`](../../crates/aureline-extensions/src/manifest_baseline/mod.rs).
  The wedge owns five new closed vocabularies the upstream contract
  does not own — requester class, authority issuer class, scope filter
  class, grant scope (persistence) class, and degraded-capability
  class — because the prompt itself must answer "who is asking", "who
  owns the decision", "what boundary", "how long does the grant
  last", and "what still works if I decline".
- It refuses to collapse to a generic "Allow?" button when authority
  facts are incomplete. Stripped requester identity, missing issuer
  source, missing scope target, missing persistence label, missing
  deny path, an unanswered prompt question, or an approve action on
  top of a blocked install-review card each surface a typed
  [`TypedPermissionPromptInvariantViolation`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  the chrome MUST surface verbatim before any approve action is
  offered.

## What the wedge owns

- One canonical
  [`TypedPermissionPromptRecord`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  data shape carrying, in stable order:
  - `record_kind`, `schema_version`, `prototype_label_token` (always
    `m1_prototype_typed_permission_prompt`);
  - the **install-review card ref** the prompt is bound to, plus the
    upstream `install_decision_class` token and the upstream
    invariant-violation count so the chrome cannot hide the fact
    that the prompt is sitting on a blocked card;
  - the **requester block** (requester class, requester ref, display
    label, request origin) — answers *who is asking*;
  - the **authority owner block** (issuer class, issuer source ref,
    issuer source label) — answers *who owns the decision*;
  - the **scope block** (scope filter class, scope target label,
    grant scope class, grant persistence label, and one row per
    requested permission carrying the upstream permission-scope
    vocabulary and a non-empty rationale) — answers *what boundary*
    and *how long does the grant last*;
  - the **consequence block** (install decision class, install
    decision reason class, consequence summary) — answers *what
    changes if allowed*;
  - the **denial branch block** (degraded-capability class, deny-path
    label, preserved-work refs) — answers *what still works if
    denied*;
  - the **prompt questions block** carrying the explicit
    `who_is_asking`, `what_boundary`, `why_needed`,
    `what_changes_if_allowed`, `what_works_if_denied`, and
    `grant_persistence_statement` strings the chrome quotes verbatim;
  - the **decision actions list** (closed action roles:
    `primary_approve`, `primary_deny`, `safer_alternative`, `details`,
    `request_admin_review`);
  - the **decision state** (`pending`, `approved`, `denied`,
    `cancelled`, `blocked_by_policy`);
  - an optional [`DegradedStateToken`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
    chrome chip;
  - the canonical claim-limit set
    (`single_bounded_wedge_only`,
    `no_org_policy_pack_productization`,
    `no_admin_approval_console`,
    `no_multi_lane_permission_system`)
    the chrome quotes verbatim under every card;
  - a typed
    [`TypedPermissionPromptInvariantViolation`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
    list so the chrome cannot offer a generic approval when authority
    facts are missing.
- A deterministic
  [`TypedPermissionPromptRecord::render_plaintext()`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  block downstream support exports and proof captures quote verbatim.

## Questions every prompt answers (verbatim)

| Question | Field on the record | Refuses to render when… |
|---|---|---|
| Who is asking? | `requester.requester_display_label` plus `prompt_questions.who_is_asking` | requester ref or display label is empty |
| Who owns the decision? | `authority_owner.issuer_source_ref` / `issuer_source_label` | issuer source ref or label is empty |
| What boundary is being crossed? | `scope.scope_filter_token`, `scope.scope_target_label`, `prompt_questions.what_boundary` | scope target label is empty |
| Why is it needed? | upstream `declared_permissions[*].rationale_label` plus `prompt_questions.why_needed` | rationale is empty (caught by upstream `manifest_baseline.declared_permission_rationale_required`) |
| What changes if allowed? | `consequence.consequence_summary`, `consequence.install_decision_class_token`, `prompt_questions.what_changes_if_allowed` | decision summary is empty |
| What still works if denied? | `denial_branch.degraded_capability_token`, `denial_branch.deny_path_label`, `prompt_questions.what_works_if_denied` | deny path label is empty |
| How long does the grant last? | `scope.grant_scope_token`, `scope.grant_persistence_label`, `prompt_questions.grant_persistence_statement` | persistence label is empty or disagrees with the grant scope token |

If any of those questions cannot be answered, the wedge surfaces the
typed `prompt_question_unanswered` / `requester_identity_missing` /
`authority_owner_missing` / `scope_missing` / `grant_persistence_missing` /
`deny_path_missing` / `consequence_missing` invariant rather than
rendering a generic "Allow?" button.

## Closed vocabularies (the chrome quotes verbatim)

- [`RequesterClass`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  — `extension`, `extension_publisher_flow`, `user_initiated_install`.
- [`AuthorityIssuerClass`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  — `shell`, `policy_service`. A `policy_service` issuer
  automatically adds the `request_admin_review` action and forbids
  local approval from widening beyond the policy.
- [`ScopeFilterClass`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  — `current_root`, `named_workset`, `full_workspace`,
  `policy_limited_view`.
- [`GrantScopeClass`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  — `once`, `session`, `workspace`, `profile`, `policy_managed`. The
  chrome's persistence label MUST mention the grant-scope token; if
  it doesn't, the typed `grant_persistence_inconsistent` invariant
  fires.
- [`DegradedCapabilityClass`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  — `no_degrade_available`, `local_only_continues`,
  `read_only_inspection_continues`, `preview_only_continues`,
  `metadata_only_export`, `install_disabled_capability_removed`.
- [`DecisionActionRole`](../../crates/aureline-shell/src/permission_prompts/mod.rs)
  — `primary_approve`, `primary_deny`, `safer_alternative`,
  `details`, `request_admin_review`.

## Protected walks

The fixtures in
[`fixtures/permissions/publish_or_ecosystem_cases/`](../../fixtures/permissions/publish_or_ecosystem_cases/)
drive the protected walks through the tests in
[`crates/aureline-shell/src/permission_prompts/tests.rs`](../../crates/aureline-shell/src/permission_prompts/tests.rs):

1. **Verified publisher, complete manifest, user approves** —
   [`protected_walk_verified_publisher_approve.json`](../../fixtures/permissions/publish_or_ecosystem_cases/protected_walk_verified_publisher_approve.json).
   The shell renders a typed permission prompt on top of a clean
   install-review fact-grid card. The prompt names Acme Labs as the
   requester, names the shell as the authority owner, asks for
   workspace-scope `current_root` access to `workspace:/docs/**` and
   the user's AI provider, persists the grant at `workspace` scope,
   and explains that local editing and manifest inspection continue
   if denied. The user approves; the record transitions to a clean
   approved state with no invariants and a `decision=admit —
   approved` summary line.
   Exercised by
   [`protected_walk_verified_admit_pending_card_offers_approve_and_deny`](../../crates/aureline-shell/src/permission_prompts/tests.rs),
   [`protected_walk_verified_admit_then_approve_renders_clean_approved_card`](../../crates/aureline-shell/src/permission_prompts/tests.rs),
   and
   [`fixture_protected_walk_verified_publisher_approve_replays_into_the_wedge`](../../crates/aureline-shell/src/permission_prompts/tests.rs).
2. **Unverified publisher, review-only, user denies** —
   [`protected_walk_unverified_publisher_deny.json`](../../fixtures/permissions/publish_or_ecosystem_cases/protected_walk_unverified_publisher_deny.json).
   The install-review card is `review_only` with a `Limited` chrome
   chip. The permission prompt suppresses the approve action
   (auto-admit is forbidden for review-only rows), offers deny and
   details only, and explains that the manifest remains visible for
   review. The user denies; the record transitions to a clean denied
   state with no invariants.
   Exercised by
   [`protected_walk_verified_admit_then_deny_renders_clean_denied_card`](../../crates/aureline-shell/src/permission_prompts/tests.rs),
   [`protected_walk_unverified_review_only_suppresses_approve`](../../crates/aureline-shell/src/permission_prompts/tests.rs),
   and
   [`fixture_protected_walk_unverified_publisher_deny_replays_into_the_wedge`](../../crates/aureline-shell/src/permission_prompts/tests.rs).

## Failure drill — incomplete authority facts are refused, not simplified

[`failure_drill_incomplete_authority_facts.json`](../../fixtures/permissions/publish_or_ecosystem_cases/failure_drill_incomplete_authority_facts.json)
exercises the named failure drill from the spec ("request a typed
permission with incomplete authority facts and confirm the prompt
refuses to collapse to generic approval copy"). A buggy caller
proposes a typed permission prompt where:

- the requester ref and display label are empty;
- the issuer source ref and label are empty;
- the scope target label is empty;
- the grant persistence label is empty;
- the deny-path label is empty;
- two of the six required prompt questions are unanswered;
- the underlying install-review card itself carries blocking
  invariants (stripped publisher identity, unknown origin, missing
  rationale on a declared permission);
- the prompt force-offers an approve action anyway.

The wedge surfaces every typed invariant rather than collapsing to a
generic "Allow?":

- `requester_identity_missing`;
- `authority_owner_missing`;
- `scope_missing`;
- `grant_persistence_missing`;
- `deny_path_missing`;
- `prompt_question_unanswered` (one per unanswered question);
- `approve_offered_with_blocked_install_review` carrying the upstream
  violation count.

`has_invariant_violations` lights, the chrome MUST refuse to render a
clean approve button, and the summary line ends `INVARIANTS BLOCKED`.

Exercised by
[`failure_drill_incomplete_authority_facts_refuses_to_collapse_to_generic_copy`](../../crates/aureline-shell/src/permission_prompts/tests.rs)
and
[`fixture_failure_drill_incomplete_authority_facts_surfaces_typed_invariants`](../../crates/aureline-shell/src/permission_prompts/tests.rs).

## Adjacent drills

- `approve_offered_against_denied_decision_surfaces_typed_invariant` —
  the upstream fact grid records a `denied` install decision but the
  prompt force-offers approve. The typed
  `approve_offered_against_denied_decision` invariant fires.
- `suppressed_deny_action_surfaces_typed_invariant` — the prompt
  drops the deny action. The typed `no_deny_action_path` invariant
  fires; a typed prompt MUST always offer a deny path.
- `approved_while_blocked_surfaces_typed_invariant` — the caller
  marks the prompt approved while requester identity is missing. The
  typed `approved_while_blocked` invariant fires alongside the
  requester invariant; the chrome cannot record an approval against a
  blocked prompt.
- `grant_persistence_label_inconsistency_surfaces_typed_invariant` —
  the grant-scope token is `workspace` but the persistence label says
  "Session". The typed `grant_persistence_inconsistent` invariant
  fires; the chrome would otherwise mislead the user about how long
  the grant lasts.
- `policy_service_issuer_adds_request_admin_review_action` — when the
  issuer is `policy_service`, the typed `request_admin_review` action
  is added so the user can forward the request to the admin path.

## Shared contracts the wedge projects against

The seed reuses these existing truth sources without forking:

- [`crates/aureline-extensions/src/manifest_baseline/mod.rs`](../../crates/aureline-extensions/src/manifest_baseline/mod.rs)
  — `ExtensionManifestBaselineRecord`,
  `EffectivePermissionBaselineRecord`,
  `ManifestInstallDecisionRecord`, plus every closed publisher /
  origin / lifecycle / permission-scope / install-decision
  vocabulary. The wedge does not fork these.
- [`crates/aureline-shell/src/install_review_fact_grid/mod.rs`](../../crates/aureline-shell/src/install_review_fact_grid/mod.rs)
  — the `InstallReviewFactGridRecord` the typed permission prompt is
  bound to; the upstream card's invariants and decision class feed
  the prompt's own invariants directly.
- [`crates/aureline-shell/src/state_cards/degraded_state.rs`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
  — the shared `DegradedStateToken` chrome chips
  (`Limited`, `PolicyBlocked`, `Warming`, ...). The wedge maps the
  chrome chip through this vocabulary instead of forking one.
- [`docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  — the upstream trust / policy / permission prompt contract this
  wedge projects against. The closed vocabularies here are deliberate
  subsets of the upstream contract; growing them is additive.
- [`docs/ux/m1_install_review_fact_grid_seed.md`](./m1_install_review_fact_grid_seed.md)
  — the upstream M1 reviewer-facing landing page for the
  install-review fact grid.

## What stays out of scope (deliberately)

- Org policy packs, remembered-decision stores, and trust-root
  productization. The wedge does not own a persistent grant store; it
  only projects what the user did on this prompt.
- Admin approval consoles, multi-lane managed-policy flows. A
  `policy_service` issuer adds the `request_admin_review` action and
  forbids local approval from widening; it does not stand up an admin
  surface.
- Publish, promote, rollback, merge, AI-tool, and remote-attach
  permission prompts. Each of those is a separate prompt family in
  the upstream
  [`docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  and is not part of this M1 wedge.
- Provider-backed or remote-authoritative grant minting. The wedge
  runs on local install-review truth only.

## Validation command

```
cargo test -p aureline-shell --lib permission_prompts
```
