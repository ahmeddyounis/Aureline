# Feature-availability explanation row, support-scope badge, and current-readiness contract

This document is the **cross-surface availability contract** for Aureline.
It freezes one feature-availability row, one support-scope badge group,
and one current-readiness object so that entry points, help, compatibility
reports, empty states, command results, search rows, marketplace entries,
issue templates, migration guides, and in-product affordances all answer
the same question — *can the user use this here, and if not, why?* — with
the same vocabulary.

The contract is normative. Where it disagrees with the UI / UX Spec
sections it quotes, the source spec wins and this document MUST be
updated in the same change. Where it disagrees with a downstream
surface's private "available" / "unavailable" model, this document wins
and the surface is non-conforming.

It exists so that:

- A "Run tests" entry, a docs help pane, a search result row, a
  marketplace card, an empty state, and a command-result message all
  read **one** availability row instead of recomputing field names.
- The user can distinguish `not configured`, `not yet indexed`,
  `disabled by policy`, `not certified here`, and `not supported`
  without reading the fine print.
- Marketing-tier "supports X" copy cannot land without naming the
  archetype row, the supported environment modes, the toolchain /
  runtime dependency, and the certified-archetype state behind the
  claim.

## Companion artifacts

- [`/schemas/ux/feature_availability_row.schema.json`](../../schemas/ux/feature_availability_row.schema.json)
  — boundary schema for `feature_availability_row_record`,
  `support_scope_badge_record`, and the embedded
  `current_readiness_object`.
- [`/artifacts/ux/support_scope_vocabulary.yaml`](../../artifacts/ux/support_scope_vocabulary.yaml)
  — machine-readable vocabulary catalogue: workflow capability
  classes, environment-mode classes, dependency-marker classes,
  availability states, support classes, why-unavailable handoff
  classes, and the consumer-surface map.
- [`/fixtures/ux/feature_availability_cases/`](../../fixtures/ux/feature_availability_cases/)
  — worked rows for the four acceptance scenarios named in the
  spec (ready locally, partial in a sparse workset, blocked by
  policy, unsupported in a managed / remote mode).

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen upstream;
it consumes existing seeds by name and by value:

- [`/docs/governance/capability_axis_matrix.md`](../governance/capability_axis_matrix.md)
  and [`/artifacts/governance/capability_badge_axes.yaml`](../../artifacts/governance/capability_badge_axes.yaml)
  — seven-axis capability badge matrix. The support-scope badge in
  this contract carries the same `lifecycle_state`, `support_class`,
  `release_channel`, `client_scope`, `freshness_class`,
  `certified_archetype_state`, and `dependency_markers` axes; it
  re-exports them, it does not invent parallel ones.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  and [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — `lifecycle_state`, `support_class`, `release_channel`,
  `client_scope`, `freshness_class`, `marker_kind`,
  `effect_on_parent`, and `reason_code` enums.
- [`/docs/compat/reference_workspace_program_seed.md`](../compat/reference_workspace_program_seed.md)
  and [`/artifacts/compat/archetype_rubric.yaml`](../../artifacts/compat/archetype_rubric.yaml),
  [`/artifacts/compat/reference_workspace_rows.yaml`](../../artifacts/compat/reference_workspace_rows.yaml)
  — certified-archetype rubric, support-class taxonomy, and
  reference-workspace rows. "Supports X" claims resolve to a row id
  in this seed.
- [`/docs/product/known_limits_contract.md`](../product/known_limits_contract.md)
  and [`/schemas/product/known_limit_note.schema.json`](../../schemas/product/known_limit_note.schema.json)
  — known-limit notes the row cites when an availability state
  carries a published exclusion or downgrade trigger.
- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md)
  — toolchain / SDK / interpreter discovery vocabulary the
  `Missing toolchain` state cites when naming the missing
  dependency.
- [`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  — `transport_posture` and `environment_status_strip` shapes the
  `Policy blocked` and `Not certified in this mode` states cite
  for active policy bundle, deployment profile, and identity mode.
- [`/docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  and [`/artifacts/ux/disabled_reason_classes.yaml`](../../artifacts/ux/disabled_reason_classes.yaml)
  — disabled-reason class catalogue. Per-surface chip / short /
  detail copy in this contract aligns with the disabled-reason
  grammar; the availability row is the *cross-surface* row, the
  disabled-reason class is the *per-command* row.

## Who reads this contract

- **Entry-point host (start center, project entry, command palette,
  search row, marketplace entry, run / debug / test pickers, empty
  state, restore card)** — to render one availability badge plus one
  reviewable explanation sentence per row, with one `Why unavailable`
  handoff per non-`Ready` state.
- **Help / docs pane (`docs/help/about` surfaces, contextual help
  triggers, migration guides, issue-template authors)** — to render
  the same row in help context with the same handoff classes, so an
  in-product affordance and a docs page never disagree about why a
  workflow is unavailable.
- **Compatibility-report row writer, claim-manifest publisher,
  marketplace listing publisher, AI-assist planner** — to read the
  same support-scope badge object and never widen a claim above the
  current `support_class`, `certified_archetype_state`, or
  `effective_environment_mode_set`.
- **Reviewers (release, security, accessibility, product copy)** —
  to verify mechanically that no availability row collapses two
  states into one chip, that every non-`Ready` state names a
  concrete `why_unavailable_handoff_class`, and that no claim row
  outruns the certified-archetype state it cites.

## Two questions the contract answers

Any Aureline surface naming a workflow capability MUST answer both
questions mechanically, without relying on per-surface copy:

1. **What is the current availability state?** Which one of the nine
   frozen states is in force for the active workspace, workset,
   environment mode, and identity / policy posture? Which workflow
   capability is the row about? Which environment mode (local,
   remote, devcontainer, managed, air-gapped) is active? Which
   archetype row is the claim being made under, and at what
   `support_class` and `certified_archetype_state`? Which toolchain
   / SDK / runtime / index / provider dependency is required, and
   what is its current state?

2. **If the state is not `Ready`, what is the handoff?** Which one
   of the eight frozen `why_unavailable_handoff_class` values does
   the row promote (open index status, install or locate the named
   toolchain, open configuration setup, open the policy explainer,
   open the compatibility report for the active mode, open the
   archetype status or request-support flow, open diagnostics
   repair, or open the capability-axis inspector)? Which evidence
   refs (capability lifecycle row, compatibility row,
   certified-archetype report row, archetype rubric row, reference
   workspace row, known-limit note, transport posture, environment
   status, policy bundle, entitlement snapshot, doctor finding,
   support bundle) does the handoff cite?

Generic phrases like "feature unavailable", "not supported", "needs
setup", "try again", or "see docs" are forbidden when a more precise
state and a more precise handoff are knowable. The schema enforces
typed sentences and typed vocabulary; surfaces render those values.

## The current-readiness object

`current_readiness_object` is the single inspectable object that an
availability row, a support-scope badge, and a docs help pane share
without recomputing field names. The schema declares it once and the
record types in this contract embed it by reference.

### Fields

The object carries:

- `availability_state` — one of the nine frozen states named below.
- `availability_state_label_text` — short reviewable label (`Ready`,
  `Partial index`, `Missing toolchain`, `Not configured`,
  `Policy blocked`, `Not certified in this mode`, `Not supported`,
  `Ready with caveats`, `Unknown repair required`). The label
  vocabulary is closed.
- `availability_explanation_sentence` — one short reviewable
  sentence stating the cause in product terms. Generic prose is
  forbidden.
- `workflow_capability_id` — opaque ref to the workflow capability
  row this availability is reported against (for example a
  `run_tests`, `find_references`, `apply_refactor`, or
  `extension_install` capability). Raw command lines never appear.
- `workflow_capability_class` — one of the closed workflow-capability
  classes seeded in `support_scope_vocabulary.yaml`.
- `effective_environment_mode` — one of `local_only`, `remote_attach`,
  `devcontainer_or_container`, `managed_workspace`, `air_gapped`,
  `mixed_local_plus_remote`, `notebook_kernel_local`,
  `notebook_kernel_remote`, `companion_surface`, `cli_or_headless`.
- `effective_environment_mode_set` — non-empty ordered set of the
  environment modes the row is *currently considered ready in*. An
  empty set is forbidden; rows that are not ready in any mode
  render the explicit `unsupported_in_every_observed_mode` sentinel.
- `dependency_marker_state` — one entry per required dependency
  (toolchain, SDK, language pack, runtime, index, provider,
  credential, extension, archetype-row, policy-bundle). Empty
  collapses to the `no_dependencies_required` sentinel.
- `support_scope_badge_ref` — opaque ref to the
  `support_scope_badge_record` rendered alongside the availability
  state.
- `why_unavailable_handoff` — required when `availability_state` is
  anything other than `ready`. Carries the handoff class, label,
  command ref, and evidence link refs.
- `evidence_links` — opaque refs to the records the row cites
  (capability lifecycle row, compatibility row, certified-archetype
  report row, archetype rubric row, reference-workspace row,
  known-limit note, transport posture, environment status,
  diagnostics card, doctor finding, settings record).
- `captured_at` — RFC 3339 UTC timestamp from a monotonic clock.

### Same shape, every consumer

This contract guarantees the same `current_readiness_object` is
read by:

- **Entry points** — start center / project entry rows, command
  palette entries, run / debug / test pickers, restore cards,
  AI-assist proposed actions.
- **Help / docs** — contextual help triggers, the docs help pane,
  migration guides, issue templates, marketplace listings.
- **Compatibility reports and claim manifest rows** — for the
  certified-archetype `current_state` cell.
- **Empty states** — when the empty state is *because of an
  availability state*, not because the data is genuinely empty.
- **Command results** — when a command refuses or downgrades
  because of one of the nine states.
- **Search** — search row badges and group headers when a search
  scope is narrowed by an availability state.
- **Support exports** — the row is serialised byte-for-byte under
  the declared `redaction_class` and never widens redaction
  relative to the live surface.

The `consumer_surfaces` field on `feature_availability_row_record`
enumerates which consumers are reading a given record so a reviewer
can confirm one record fans out to the expected surfaces.

## The nine availability states

The state vocabulary is **closed**. Adding a value is an additive-
minor schema bump and requires new fixture coverage. Repurposing a
value is breaking and requires a decision row.

### `ready`

The workflow can be invoked here under the current archetype,
environment mode, and policy posture without any narrowing handoff.
`why_unavailable_handoff` is null. `dependency_marker_state` reports
each dependency at `present` or `not_required`. The schema forbids
attaching a handoff to a `ready` row.

### `ready_with_caveats`

The workflow is invokable, but at least one dependency marker has
narrowed the effective lifecycle, support, channel, or client-scope
axis (per the upstream capability axis matrix). The row carries
`why_unavailable_handoff_class = open_capability_axis_inspector`
with a non-null capability lifecycle row ref. The visible label
remains `Ready with caveats`; the user is not blocked.

### `partial_index`

A required index (semantic, project graph, search, diagnostics,
provider) is warming, partial, or stale; the workflow is available
against the indexed scope but not against the remainder. The row
carries `why_unavailable_handoff_class = open_index_status_repair`
with a non-null index dependency ref naming the warming or stale
index. The schema forbids `partial_index` without an index marker.

### `missing_toolchain`

A required interpreter, SDK, runtime, or language pack is absent or
below the required version. The row carries
`why_unavailable_handoff_class = install_or_locate_toolchain` with
a non-null toolchain dependency ref. The schema forbids
`missing_toolchain` without a `toolchain_or_sdk_or_runtime` marker.

### `not_configured`

The capability shipped, the toolchain is present, but workspace,
workset, or user setup has not declared the configuration the
workflow needs (no test runner pinned, no debug target chosen, no
provider linked, no archetype declared). The row carries
`why_unavailable_handoff_class = open_configuration_setup` with a
non-null setup target ref. The schema forbids `not_configured`
without a setup-target ref.

### `policy_blocked`

The active policy bundle, organization policy, workspace trust,
extension publisher policy, or kill switch blocks the workflow in
the current scope. The row carries
`why_unavailable_handoff_class = open_policy_explainer` with a
non-null `active_policy_bundle_ref` and `policy_rule_ref`. The
schema forbids `policy_blocked` without a policy bundle ref.

### `not_certified_in_this_mode`

The workflow has a certified-archetype claim for one or more
modes, but the active environment mode is not in the certified
set. For example, a workflow certified for `local_only` and
`devcontainer_or_container` running under `managed_workspace`. The
row carries
`why_unavailable_handoff_class = open_compatibility_report_for_mode`
with a non-null certified-archetype report ref and a non-null
`active_environment_mode` value, plus the
`effective_environment_mode_set` listing the modes where it *is*
certified. The schema forbids `not_certified_in_this_mode` without
those refs.

### `not_supported`

The workflow does not exist on this client scope, archetype, or
host (for example a desktop-only workflow on the CLI, a workflow
not in the active reference-workspace row's archetype, or a
workflow whose `client_scope` set excludes the active client). The
row carries
`why_unavailable_handoff_class = open_archetype_status_or_request_support`
with a non-null reference-workspace row ref or archetype rubric
row ref. The schema forbids `not_supported` without one of those
refs.

### `unknown_repair_required`

The availability resolver could not determine state — the
capability lifecycle row failed to resolve, the dependency probe
timed out, the policy bundle is unreachable, or the archetype row
is unresolved. The row carries
`why_unavailable_handoff_class = open_diagnostics_repair` with a
non-null doctor-finding or diagnostics-card ref. The schema forbids
`unknown_repair_required` without one of those refs.

## The `Why unavailable` handoff classes

The handoff vocabulary is **closed**. Each non-`ready` state pairs
with exactly one handoff class:

| Availability state | Handoff class | Required evidence ref |
| --- | --- | --- |
| `ready` | `none_state_is_ready` | none |
| `ready_with_caveats` | `open_capability_axis_inspector` | capability lifecycle row |
| `partial_index` | `open_index_status_repair` | index dependency ref |
| `missing_toolchain` | `install_or_locate_toolchain` | toolchain / SDK / runtime ref |
| `not_configured` | `open_configuration_setup` | setup target ref |
| `policy_blocked` | `open_policy_explainer` | active policy bundle ref + policy rule ref |
| `not_certified_in_this_mode` | `open_compatibility_report_for_mode` | certified-archetype report ref + active environment mode |
| `not_supported` | `open_archetype_status_or_request_support` | reference-workspace row ref or archetype rubric row ref |
| `unknown_repair_required` | `open_diagnostics_repair` | doctor finding or diagnostics card ref |

The handoff carries:

- `handoff_class` — one of the nine values above.
- `handoff_label_text` — a short reviewable label (for example
  `Open index status`, `Install Python`, `Choose a test target`,
  `Open policy explainer`, `Open managed compatibility report`,
  `Request archetype support`, `Open diagnostics`,
  `Inspect capability axes`).
- `handoff_command_ref` — opaque command id the handoff promotes.
  `none_state_is_ready` rows leave this null.
- `handoff_evidence_refs` — opaque refs the handoff cites; the
  schema enforces the per-state required evidence ref above.

## The support-scope badge group

`support_scope_badge_record` is one structured badge group that
declares the seven capability axes for a workflow capability,
re-exporting the upstream capability axis matrix verbatim. The badge
is what marketing-tier copy and "supports X" claims must read instead
of inventing scalar tier chips.

### Required fields

Every badge declares:

- `lifecycle_state` — one of `labs`, `preview`, `beta`, `stable`,
  `lts_facing`, `deprecated`, `disabled_by_policy`, `retired`,
  `unresolved_axis_lifecycle_state`. Re-exported from
  `capability_lifecycle.schema.json#/$defs/lifecycle_state`.
- `support_class` — one of `best_effort`, `community_supported`,
  `standard_support`, `extended_support`, `operator_only_support`,
  `no_support`, `unresolved_axis_support_class`. Re-exported from
  `capability_lifecycle.schema.json#/$defs/support_class`.
- `release_channel` — one of `nightly_channel`,
  `experimental_channel`, `preview_channel`, `stable_channel`,
  `lts_channel`, `managed_only_channel`,
  `unresolved_axis_release_channel`.
- `client_scopes` — non-empty set drawn from `desktop_product`,
  `cli`, `companion_surface`, `remote_agent`, `sdk_or_api`,
  `managed_admin_surface`. Empty collapses to
  `unresolved_axis_client_scope`.
- `freshness_class` — one of `authoritative_live`, `warm_cached`,
  `degraded_cached`, `stale`, `unverified`,
  `unresolved_axis_freshness_class`.
- `certified_archetype_state` — one of `not_applicable`,
  `supported`, `best_effort`, `untested`, `degraded`, `unsupported`,
  `unresolved_axis_certified_archetype_state`.
- `dependency_markers` — list of `{ marker_kind, effect_on_parent,
  reason_code, repair_hook_ref }` entries. Empty renders the
  explicit `no_live_markers` sentinel.
- `archetype_row_ref` — opaque ref to the
  `reference_workspace_rows.yaml` row backing the support claim.
  `not_applicable` for capabilities outside the certified-archetype
  program.
- `support_window_ref` — opaque ref to the support-window row the
  claim aligns with. Required when `support_class` is anything
  other than `best_effort` or `community_supported`.
- `known_limit_refs` — opaque refs to known-limit notes that
  caveat the claim. Empty when no caveats are published.

### No collapses

Per the upstream matrix, a "supports Python" / "supports Rust" /
"supports Bazel" chip that asserts `support_class = standard_support`
without naming both the certified-archetype row and the
`effective_environment_mode_set` is non-conforming. The schema
enforces:

- A badge that renders `support_class = standard_support` or
  `extended_support` MUST cite a non-null `archetype_row_ref` and a
  non-null `support_window_ref`.
- A badge with `certified_archetype_state = supported` MUST cite a
  non-null `archetype_row_ref` and the `effective_environment_mode_set`
  on the parent availability row MUST be non-empty.
- A badge whose `certified_archetype_state` is anything other than
  `supported` or `not_applicable` MUST narrow the visible
  availability label (per the per-state rules above).

### Forbidden collapses

The schema rejects any record that:

- Uses a generic chip word from the forbidden list:
  `Available`, `Unavailable`, `Supported`, `Coming soon`,
  `Sometimes`, `Maybe`, `In progress`, `Try again later`,
  `See docs`, `Generic error`.
- Renders `availability_state = ready` together with a non-null
  `why_unavailable_handoff`.
- Renders `availability_state` other than `ready` without a
  non-null `why_unavailable_handoff`.
- Renders `partial_index` without an index marker, `missing_toolchain`
  without a toolchain marker, `not_configured` without a setup-target
  ref, `policy_blocked` without a policy bundle ref,
  `not_certified_in_this_mode` without a certified-archetype report
  ref, `not_supported` without a reference-workspace row or archetype
  rubric row ref, or `unknown_repair_required` without a diagnostics
  ref.
- Claims `support_class = standard_support` or `extended_support`
  without an `archetype_row_ref` and a `support_window_ref`.

## Explanation rules

To prevent vague language-level marketing claims, every availability
row that names a language, framework, or tool MUST carry the
following four scope fields. The schema enforces non-null on each:

- `archetype_row_ref` — opaque ref to the row in
  `artifacts/compat/reference_workspace_rows.yaml` that frames the
  claim. A capability row with no archetype binding renders the
  explicit `archetype_not_applicable` sentinel; this is *not* the
  same as a missing ref.
- `effective_environment_mode_set` — the modes in which the row is
  currently `ready` or `ready_with_caveats`. A claim row with an
  empty mode set is non-conforming.
- `toolchain_or_runtime_dependency_refs` — non-empty list of
  toolchain / SDK / runtime markers from the dependency-marker
  state. A row that asserts a language claim with no toolchain
  dependency is non-conforming.
- `support_class` — re-exported from the upstream lifecycle schema.
  A row that claims `standard_support` or higher MUST cite the
  archetype row and support-window row.

A row that fails any of these is non-conforming. The phrase
"supports Python" without a row in
`reference_workspace_rows.yaml` cannot be rendered through this
schema; the surface either lowers the claim to `community_supported`
with the matching archetype caveat, or it does not render the
support-scope badge at all.

## Cross-surface invariants

The schema enforces the following invariants mechanically:

1. `availability_state = ready` implies `why_unavailable_handoff`
   is null and the `availability_state_label_text` is exactly
   `Ready`.
2. `availability_state` other than `ready` implies a non-null
   `why_unavailable_handoff` whose `handoff_class` matches the
   per-state row above.
3. `availability_state = partial_index` requires the dependency
   marker list to contain at least one
   `marker_kind = freshness_floor_dependency` or
   `index_warming_dependency` entry.
4. `availability_state = missing_toolchain` requires the dependency
   marker list to contain at least one
   `marker_kind = toolchain_or_sdk_or_runtime_dependency` entry
   whose state is not `present`.
5. `availability_state = not_configured` requires a non-null
   `setup_target_ref` on the handoff and a non-null
   `configuration_kind` from the closed
   `configuration_kind_vocabulary` (test-runner pin, debug-target
   pin, provider link, archetype declaration, environment pin,
   credential link).
6. `availability_state = policy_blocked` requires non-null
   `active_policy_bundle_ref` and `policy_rule_ref` on the
   handoff.
7. `availability_state = not_certified_in_this_mode` requires
   non-null `certified_archetype_report_ref`, non-null
   `active_environment_mode`, and a non-empty
   `effective_environment_mode_set` listing the modes where the
   row *is* certified.
8. `availability_state = not_supported` requires either a non-null
   `reference_workspace_row_ref` or a non-null
   `archetype_rubric_row_ref`.
9. `availability_state = unknown_repair_required` requires a
   non-null `doctor_finding_ref` or `diagnostics_card_ref`.
10. `support_scope_badge.support_class` of `standard_support` or
    `extended_support` requires non-null `archetype_row_ref` and
    `support_window_ref`.
11. `support_scope_badge.certified_archetype_state = supported`
    implies the parent availability row's
    `effective_environment_mode_set` is non-empty.
12. `support_scope_badge.dependency_markers` empty MUST render the
    `no_live_markers` sentinel; the empty-list-without-sentinel
    posture is non-conforming.
13. The forbidden generic chip vocabulary
    (`Available`, `Unavailable`, `Supported`, `Coming soon`,
    `Sometimes`, `Maybe`, `In progress`, `Try again later`,
    `See docs`, `Generic error`) is rejected by the schema in
    every text field.

These are not suggestions; they are schema-enforced rules. A fixture
that fails any of them is non-conforming.

## Consumer-surface coverage

`feature_availability_row_record` carries a `consumer_surfaces`
field naming which surfaces consume the row. The closed surface
vocabulary is:

- `entry_point_start_center`
- `entry_point_project_entry`
- `entry_point_command_palette`
- `entry_point_run_or_debug_or_test_picker`
- `entry_point_search_row`
- `entry_point_marketplace_card`
- `entry_point_restore_card`
- `help_contextual_trigger`
- `help_docs_pane`
- `help_about_pane`
- `compatibility_report_row`
- `claim_manifest_row`
- `migration_guide_row`
- `issue_template_row`
- `empty_state_row`
- `command_result_message`
- `support_export_row`

A row whose `consumer_surfaces` is empty is non-conforming; every
record names at least one surface. A row that fans out across
incompatible surfaces (for example, an `entry_point_*` surface
together with `support_export_row` only) is admissible — the same
record can drive both. The `consumer_surfaces` field is a fan-out
declaration, not an XOR.

## Redaction and export posture

Every record (`feature_availability_row_record`,
`support_scope_badge_record`) carries a `redaction_class` from the
four-value standard vocabulary (`metadata_safe_default`,
`operator_only_restricted`, `internal_support_restricted`,
`signing_evidence_only`) and an `export_safe` flag. Raw absolute
paths, raw URLs, raw command lines, raw policy bodies, raw secret
values, raw OAuth tokens, raw CA bundle bytes, raw certificate
chain PEM, raw stdout / stderr, raw notebook output bytes, raw
clipboard bytes, and raw provider payloads MUST NOT appear on this
boundary. Surfaces resolve labels from opaque refs locally and
never embed raw bytes in the record.

## Adding or changing vocabulary

Adding a value to any vocabulary in this contract is
**additive-minor**:

1. Update the schema enum in
   `schemas/ux/feature_availability_row.schema.json`.
2. Update this document.
3. Update the matching catalogue row in
   `artifacts/ux/support_scope_vocabulary.yaml`.
4. Add or update a fixture under
   `fixtures/ux/feature_availability_cases/` exercising the new
   value.
5. Bump `feature_availability_row_schema_version`.

Repurposing an existing value is **breaking** and requires:

1. A new decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `product_scope_review` and the owning lane for the touched
   axis (lifecycle, support, channel, freshness, archetype,
   client scope, or dependency-marker lane).
2. Deprecation of the old value, addition of the new value
   through an additive-minor landing, and a translation pass on
   support exports and admin-audit packets across the
   deprecation window.

## Out of scope at this revision

- Final pixel-perfect chrome for availability badges, support-scope
  badge groups, or `Why unavailable` panels in the desktop product.
- The full set of per-archetype fixtures for every workflow
  capability the program will ship; this contract only seeds the
  four acceptance scenarios named in the spec.
- Localization-ready string tables; the contract carries
  reviewable English sentences and the localization layer is
  consumed separately through
  `docs/ux/localization_and_locale_pack_contract.md`.
- The compatibility-report and claim-manifest rendering pipeline
  themselves; this contract is the *availability boundary* those
  rows consume, not the row authoring pipeline.
