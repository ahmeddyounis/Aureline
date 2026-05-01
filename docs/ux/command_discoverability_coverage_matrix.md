# Command Discoverability Coverage Matrix, Guided-Entry Parity Audit, and Onboarding Hint-Source Ledger

This document freezes the cross-surface contract that pins onboarding,
the command palette, menus and context menus, keybinding help and the
current-shortcut display, primary toolbar buttons and inline
affordances, in-product help search, docs / help anchors, contextual
tips, migration bridge cards, why-unavailable explainers, and
guided-surface tour or learning-mode prompts to **one** discoverability
truth instead of parallel hint systems. Every launch-critical action
must be reachable through more than one discoverable route, every
route must derive its label, scope, badge, shortcut hint, and
explainer text from the same canonical command record, and any
deliberate one-route exception must be declared with a typed reason
rather than left implicit.

The companion artifacts are:

- [`/artifacts/ux/discoverability_coverage_rows.yaml`](../../artifacts/ux/discoverability_coverage_rows.yaml)
  &mdash; one machine-readable row per launch-critical command mapping
  it to every discoverability surface the registry projects to, plus a
  closed exception lane for deliberate single-route actions.
- [`/artifacts/ux/hint_source_ledger.yaml`](../../artifacts/ux/hint_source_ledger.yaml)
  &mdash; the single ledger every onboarding card, contextual tip,
  guided tour step, glossary card, migration bridge card, learning-mode
  card, current-shortcut display, and why-unavailable explainer reads
  to obtain canonical command IDs, docs anchors, glossary packs,
  keymap-bridge refs, current-shortcut rows, and disabled-reason
  explainers.
- [`/fixtures/ux/guided_entry_parity_cases/`](../../fixtures/ux/guided_entry_parity_cases/)
  &mdash; worked parity audit cases for Start Center, clone / import /
  restore review, missing-target recovery, trust-stage admission,
  workspace-admission, and imported-keymap or migration-bridge
  scenarios.

This contract is normative for the projection, route exposure, and
hint-sourcing posture of any launch-critical guided or learnability
surface. Where it disagrees with the PRD, TAD, TDD, UI/UX spec, the
learnability contract, the contextual teaching contract, the palette
row contract, the keybinding resolver contract, the migration center
object model, or the command registry seed, those sources win and this
document plus its companion artifacts and fixtures update in the same
change. Where a downstream onboarding, palette, menu, help, or
migration surface mints a parallel command id, alias, shortcut row, or
explainer string outside this contract, this contract wins and the
surface is non-conforming.

## Companion contracts this contract rides on

This contract does not re-mint vocabulary already frozen upstream; it
consumes it by reference:

- [`/artifacts/commands/command_registry_seed.yaml`](../../artifacts/commands/command_registry_seed.yaml)
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  &mdash; canonical `command_id`, `alias_records`, `current_keybinding_refs`,
  `disabled_reason_records`, `origin_badge`, `target_badges`,
  `discoverability_record`, `preferred_surface_exposures`,
  `machine_name_records`, and reserved projection refs (`palette_row`,
  `help_search`, `onboarding_hint`, `migration_bridge_card`,
  `current_shortcut_display`, `why_unavailable_explainer`,
  `key_sequence_discoverability`).
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  and the `ui_slot_class` taxonomy
  ([`/schemas/commands/ui_slot_taxonomy.schema.json`](../../schemas/commands/ui_slot_taxonomy.schema.json))
  &mdash; closed `ui_slot_class` set used to bind menu / palette /
  toolbar / keybinding help exposures. Coverage rows resolve through
  these classes; nothing in this contract redefines them.
- [`/docs/commands/palette_row_and_modifier_contract.md`](../commands/palette_row_and_modifier_contract.md)
  and
  [`/schemas/commands/palette_row.schema.json`](../../schemas/commands/palette_row.schema.json)
  &mdash; palette row primary label, scope detail, origin badge,
  winning shortcut hint, reason chip, automation labels, lifecycle
  cue, and modifier-action vocabulary.
- [`/docs/ux/learnability_contract.md`](./learnability_contract.md)
  and
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  &mdash; guided-surface authority class
  (`command_id_anchored`, `keymap_bridge_anchored`,
  `derived_with_upstream_anchors`, `glossary_pack_anchored`,
  `tour_step_anchored`), dismissal / reset / export classes,
  freshness, suppression cause, and presentation adjunct rules.
- [`/docs/ux/contextual_teaching_contract.md`](./contextual_teaching_contract.md)
  and
  [`/schemas/ux/teaching_surface.schema.json`](../../schemas/ux/teaching_surface.schema.json)
  &mdash; contextual tip, migration bridge card, why-unavailable
  explainer, and source-language fallback record shape, including
  required source refs, deeper-doc pivots, and trust / policy /
  write-scope guard.
- [`/docs/ux/keybinding_resolver_contract.md`](./keybinding_resolver_contract.md)
  and
  [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
  &mdash; legacy and imported keymap bridge records. Imported-keymap
  parity cases resolve through `keymap_bridge:*` ids rather than
  re-authoring shortcut copy.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  &mdash; importer outcome and migration session truth. Migration
  bridge card parity cases resolve through importer-outcome refs
  rather than reauthoring imported behavior states.
- [`/docs/ux/start_center_contract.md`](./start_center_contract.md)
  and
  [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json)
  &mdash; Start Center zones, primary action wiring, recent-work and
  restore-card row vocabulary. Start-Center parity cases mirror the
  zones declared there.
- [`/docs/ux/no_account_local_entry_contract.md`](./no_account_local_entry_contract.md)
  and the first-useful-work qualification corpus
  ([`/artifacts/ux/first_useful_work_corpus/`](../../artifacts/ux/first_useful_work_corpus/))
  &mdash; entry-route taxonomy, deployment profile vocabulary, and
  `fuw_row:*` ids. Parity cases that exercise an entry route reuse
  the upstream `fuw_row:*` rather than minting a parallel scenario.

## Who reads this contract

- **Onboarding and learnability authors** &mdash; to find the
  canonical anchor for each launch-critical action before drafting an
  onboarding card, tour step, contextual tip, or learning-mode prompt.
- **Palette, menu, toolbar, and keybinding authors** &mdash; to verify
  every launch-critical action is exposed across the routes the user
  is likely to discover it through, with one source of truth for
  primary label, scope, origin badge, and current shortcut.
- **Help search and docs authors** &mdash; to project canonical
  command rows into help search, docs anchors, and contextual
  explainers without minting alternative IDs or alias drift.
- **Migration and keymap-bridge authors** &mdash; to attach imported
  keymap, settings, snippet, task config, and extension bridge
  surfaces to the same canonical command record the rest of the
  shell renders.
- **Reviewers and release governance** &mdash; to read off, in one
  artifact, whether a workflow is discoverable only through one route
  and whether that is an explicit, typed exception.
- **Support and field readiness** &mdash; to project the same
  canonical command, alias, current shortcut, and disabled-reason
  row into support exports without rebuilding the discoverability
  story per channel.

## Why this exists

Without one discoverability truth, the same action ends up with
slightly different names in the palette, menus, onboarding, contextual
tips, help search, migration bridges, keybinding help, and CLI help.
Aliases drift, shortcut copy goes stale, and "hidden" actions get
taught only by an onboarding card or a how-to that bypasses the
canonical command. This contract enforces three invariants:

1. **One canonical anchor per action.** Every discoverable route to a
   launch-critical action resolves through a stable
   `command_id`, `alias_record`, `keymap_bridge_ref`, or migration
   `importer_outcome_ref` on the registry, not free-text labels or
   parallel verb sets.
2. **Multi-route by default, exception by declaration.** Every
   launch-critical action must reach at least two discoverable
   surfaces from the closed `surface_route_class` set below, and any
   deliberate single-route or hidden exposure declares a typed
   `exception_class` plus owner notes.
3. **No hidden mutation paths inside teaching surfaces.** Onboarding
   cards, contextual tips, guided tour steps, migration bridge cards,
   and why-unavailable explainers may only invoke an action by
   projecting the same `command_id` (or alias) the rest of the shell
   uses; they may not smuggle a one-off mutation route.

## Closed vocabularies

The coverage rows, hint-source ledger, and parity audit fixtures
resolve through the following closed sets. Where a vocabulary is
already frozen upstream the table cites the upstream source verbatim
rather than re-minting it.

### `surface_route_class`

| Value | Job | Upstream source |
|---|---|---|
| `command_palette` | Universal text-driven discoverability and invocation. | `ui_slot_class: command_palette` |
| `global_application_menu` | Title-bar / menu-bar navigation. | `ui_slot_class: global_application_menu` |
| `context_menu` | Right-click / accessible-context surfaces (explorer, editor, recent work). | `ui_slot_class: explorer_context_menu` and peers |
| `primary_toolbar` | Persistent shell toolbar buttons. | `ui_slot_class: primary_toolbar` |
| `inline_affordance` | Row-local buttons, inline hints, status strip controls. | shell zone family |
| `current_shortcut_display` | Shortcut shown next to a row, button, or menu item. | `discoverability_record.projection_refs.current_shortcut_display_ref` |
| `keybinding_help` | Keybinding help / cheatsheet surface. | `ui_slot_class: keybinding_help` |
| `key_sequence_help` | Multi-step chord / leader sequence discoverability. | `discoverability_record.projection_refs.key_sequence_discoverability_ref` |
| `help_search` | In-product help / docs search results. | `discoverability_record.projection_refs.help_search_ref` |
| `docs_help_anchor` | Linked docs / help page anchor. | `descriptor.docs_help_anchor_ref` |
| `onboarding_card` | First-run onboarding card. | `surface_kind: onboarding_card` |
| `guided_tour_step` | Sequential guided-tour step. | `surface_kind: guided_tour_step` |
| `contextual_tip_card` | In-context tip card. | `surface_kind: contextual_tip_card` |
| `migration_bridge_card` | Imported-behavior bridge card. | `surface_kind: migration_bridge_card` |
| `why_unavailable_explainer` | Disabled / blocked / hidden explainer. | `surface_kind: why_unavailable_explainer` |
| `glossary_card` | Glossary card / definition surface. | `surface_kind: glossary_card` |
| `cli_help` | CLI / automation help projection. | `ui_slot_class: cli_help` |

### `route_role_class`

How the action participates in that route. Mirrors
`preferred_surface_exposures.preference_class` on the registry plus an
explicit `not_applicable` cell for typed exceptions.

| Value | Meaning |
|---|---|
| `entry` | Primary route the user invokes the command from. |
| `mirror` | Non-primary route that re-renders the canonical row, label, badge, and shortcut hint. |
| `contextual` | Route only renders when its scope filter matches (selection, archetype, deployment profile). |
| `explain_only` | Route explains the command without invoking it (Labs, deprecated, why-unavailable). |
| `not_applicable` | Route is deliberately omitted under a declared `exception_class`. |

### `source_anchor_kind`

How a coverage row or ledger entry resolves to canonical truth. Every
row carries at least one anchor in this set.

| Value | Source |
|---|---|
| `canonical_command_id` | `descriptor.command_id` on the registry. |
| `alias_record` | `alias_records[*].alias_id`. |
| `docs_help_anchor` | `descriptor.docs_help_anchor_ref` or `docs_anchor:*` ref. |
| `glossary_pack_anchor` | `glossary_pack_ref` plus `glossary_term_anchor` from the citation contract. |
| `guided_tour_step_anchor` | `tour_step_anchor` from the learnability contract. |
| `keybinding_resolver_ref` | `keymap_bridge:*` from the keybinding resolver. |
| `migration_bridge_ref` | `importer_outcome_ref` or `bridge_ref` from the migration center model. |
| `why_unavailable_explainer_ref` | `disabled_reason_records[*].explanation_ref`. |
| `current_shortcut_display_ref` | Projection ref for the row's current shortcut. |
| `learning_mode_profile_ref` | `learning_mode_profile_ref` from the learnability contract. |

### `exception_class`

Declared posture for a deliberate single-route or restricted exposure.
A row whose route count is below the floor below MUST declare one of
these with a typed `exception_owner_role` and `exception_review_ref`.

| Value | Meaning |
|---|---|
| `none` | No exception; row meets the multi-route floor. |
| `deliberately_palette_only` | Discoverable from the palette only; no menu, toolbar, or keybinding mirror. |
| `deliberately_menu_only` | Reachable through menus / context menus only. |
| `deliberately_keybinding_only` | Bound shortcut only; no menu, toolbar, palette mirror. |
| `deliberately_inline_only` | Row-local affordance only (e.g. recent-work row Locate button). |
| `deliberately_unbound_keybinding` | Keybinding intentionally unassigned in the platform default. |
| `deliberately_disabled_until_capability_enabled` | Hidden or disabled until a capability flag, policy, or trust stage flips. |
| `deliberately_help_only` | Surface explains rather than invokes (deprecated alias, replaced command). |
| `deliberately_migration_only` | Reachable only through a migration bridge until import completes. |
| `deliberately_guided_only` | Reachable through guided-tour or onboarding flow during a bounded run. |
| `deliberately_diagnostics_only` | Labs / diagnostics; explicit gate beyond palette explain-only. |

### `parity_finding_class`

Used by the parity audit fixtures. A row carries one finding class.

| Value | Meaning |
|---|---|
| `parity_complete` | All required routes resolve through canonical anchors and the floor is met. |
| `parity_complete_with_disclosed_exception` | Single-route or restricted exposure declared with a typed `exception_class`. |
| `parity_partial_known_gap` | Route gap acknowledged with a tracked follow-up; not a violation while a typed waiver is open. |
| `parity_violation_orphan_action` | A guided / help / migration surface invokes or teaches an action without a canonical anchor. |
| `parity_violation_alias_drift` | Two surfaces use different ids / labels for the same action without an alias record. |
| `parity_violation_anchor_missing` | A route claims to project a registry projection ref that does not exist. |
| `parity_violation_route_route_drift` | The same canonical command renders different scope / badge / shortcut copy across routes. |

## Coverage matrix &mdash; required floor

For every command in the launch-critical scope listed in
`/artifacts/ux/discoverability_coverage_rows.yaml`, a coverage row
declares the role of every `surface_route_class` value above. The
floor is:

1. **Multi-route invocation.** At least two of
   `command_palette`, `global_application_menu`, `context_menu`,
   `primary_toolbar`, `inline_affordance`, or
   `current_shortcut_display` carry a non-`not_applicable`,
   non-`explain_only` `route_role_class`.
2. **Searchability.** `help_search` MUST be `mirror` or `contextual`,
   and `docs_help_anchor` MUST resolve to a valid
   `descriptor.docs_help_anchor_ref` for any command that ships an
   onboarding card or contextual tip.
3. **Onboarding coverage.** `onboarding_card`, `guided_tour_step`,
   or `contextual_tip_card` MUST be `mirror` or `explain_only` for any
   command exercised by the first-useful-work corpus.
4. **Migration parity.** Commands with `keymap_bridge:*` records or
   `importer_outcome:*` records MUST carry a `migration_bridge_card`
   row and a `current_shortcut_display` row; otherwise the bridge is
   classified `parity_violation_orphan_action`.
5. **Disabled posture.** Commands with at least one
   `disabled_reason_records[*]` entry MUST carry a
   `why_unavailable_explainer` row resolved through the same
   `explanation_ref`.
6. **Automation / CLI parity.** `cli_help` MUST be `mirror` or
   `not_applicable`; if `not_applicable`, the row's `automation_labels`
   MUST contain `ui_only` or the registry's `machine_name_records`
   MUST flag the verb as `eligible_for_new_automation = false`.

A row that fails the floor without a typed `exception_class` is
non-conforming and blocks the discoverability review gate.

## Hint-source ledger

`/artifacts/ux/hint_source_ledger.yaml` records, per launch-critical
command, the canonical references onboarding, learnability, and
migration surfaces are allowed to read:

- `canonical_command_id` (the only allowed identity)
- `alias_record_refs` (eligible-for-new-bindings vs. deprecated)
- `docs_help_anchor_refs` (overview, onboarding, troubleshooting,
  migration)
- `glossary_pack_anchor_refs` (terms used by tour or learning-mode
  cards)
- `guided_tour_step_anchor_refs` (tour step ids the command
  participates in)
- `keymap_bridge_refs` (legacy / imported keymap bridges that resolve
  to this command)
- `current_shortcut_display_refs` (per-platform shortcut rows)
- `why_unavailable_explainer_refs` (per-disabled-reason explainers)
- `learning_mode_profile_refs` (profiles that gate or surface this
  command differently)
- `migration_bridge_refs` (importer-outcome refs that produce or
  shim this command)

The ledger is read-only: every entry MUST resolve through an upstream
record. New onboarding hints, contextual tips, or migration bridge
cards may only attach to a ledger row; they may not invent a parallel
anchor.

## Guided-entry parity audit

`/fixtures/ux/guided_entry_parity_cases/*.yaml` carries one fixture
per audit case. Each case names:

- the canonical `command_id` and any `alias_record_refs` exercised
- the `case_category` (`start_center_first_run`,
  `clone_review`, `import_profile`, `restore_from_checkpoint`,
  `missing_target_recovery`, `trust_stage`, `workspace_admission`,
  `imported_keymap`, `migration_bridge`)
- the `fuw_row:*` ids (where applicable) the case mirrors
- the `surface_route_class` rows under audit and their
  `route_role_class` values
- the `parity_finding_class` outcome
- the `source_anchor_kind` values resolved
- any declared `exception_class` and `exception_review_ref`

Parity cases are reusable: docs/help, migration, keymap bridges, and
learnability evidence cite the case ids without rewriting command
names or shortcut facts.

## Acceptance mapping

This contract maps to the M00-527 acceptance criteria as follows:

- **No launch-critical onboarding or help step points to a one-off
  hidden action that lacks a canonical command or source anchor.**
  Enforced by the multi-route floor and the hint-source ledger; any
  guided surface that mints an action without an entry in
  `hint_source_ledger.yaml` is `parity_violation_orphan_action`.
- **Reviewers can tell when a workflow is discoverable only through
  one route and whether that is an explicit exception.** Enforced
  by `exception_class` and the parity finding vocabulary; rows that
  fall below the floor without a typed exception are non-conforming.
- **Guided-entry parity cases can be reused by docs/help, migration,
  keymap bridges, and learnability evidence without rewriting command
  names or shortcut facts.** Enforced by the parity case fixture
  shape: every case resolves through canonical refs only.
- **Help search, contextual tips, and migration bridges can project
  canonical command truth without inventing alternative IDs or
  drift-prone aliases.** Enforced by the closed `source_anchor_kind`
  set and the rule that aliases must resolve to a registry
  `alias_record`.

## Out of scope

This contract does not author final onboarding copy, full tour
content, localized strings, the M1 onboarding experience, or final
copywriting polish. It freezes the structural truth every such
artifact must resolve through.
