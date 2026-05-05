# Core-control family contract

This contract freezes the shared state, size, density, source, and
async-feedback grammar for Aureline's reusable controls. Buttons,
split buttons, checkboxes, radios, toggles, text inputs, selects,
comboboxes, segmented controls, and form-level submit / review actions
consume this contract before a family-specific packet adds extra
anatomy.

The contract is normative. Where this document disagrees with the
source UI / UX spec, design-system style guide, component-state
taxonomy, form-validation contract, field-row contract, or
disabled-reason grammar it cites, that upstream source wins and this
document plus its schema and fixtures update in the same change. Where
this document disagrees with a downstream control packet, this document
wins and the downstream packet is non-conforming.

## Companion artifacts

- [`/schemas/ux/control_state.schema.json`](../../schemas/ux/control_state.schema.json)
  - boundary schema for one `control_state_record`. It keeps control
  family, size, density, focus, disabled, busy, locked, destructive,
  value-source, live-vs-staged, keyboard, and support-handoff semantics
  explicit.
- [`/fixtures/ux/control_family_cases/`](../../fixtures/ux/control_family_cases/)
  - worked records covering busy submit, policy lock, imported value,
  live toggle, staged apply, and destructive action controls.

This contract composes with, and does not replace:

- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  for frozen component states, density rules, token families, and
  non-color state conveyance.
- [`/docs/design/component_state_taxonomy.md`](../design/component_state_taxonomy.md)
  for the shared `default`, `disabled`, `read_only`, `loading`,
  `pending`, `locked`, and `degraded` meanings controls cite.
- [`/docs/ux/forms_validation_contract.md`](./forms_validation_contract.md)
  for staged review, async probe, mutation-blocking, stale validation,
  and apply timing.
- [`/docs/ux/input_and_combobox_contract.md`](./input_and_combobox_contract.md)
  for shared input anatomy, inline-validation timing, clear/reveal
  semantics, and search/combobox option-list state honesty.
- [`/docs/ux/field_row_and_value_source_contract.md`](./field_row_and_value_source_contract.md)
  for value-source precedence, source labels, effective-value
  inspection, and exact-row links.
- [`/docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  for unavailable-action reason classes and why-unavailable copy.
- [`/docs/ux/component_contract_template.md`](./component_contract_template.md)
  for reusable component packet structure and extension guidance.

Normative source sections projected here include the core-control,
density, button, input, shared-state, component-review, structured-input,
live-versus-staged, command-graph, and terminology sections in
`.t2/docs/`.

## Scope

This contract applies to every first-party or must-match reusable
control that can appear in settings, onboarding, install / update
review, package review, admin policy, migration / import, request /
runtime, command, or provider-backed surfaces.

Low-risk local controls may render without a full exported
`control_state_record` while they are local-only, immediately
reversible, non-secret, and not source-layered. The moment a control
needs source provenance, async validation, policy / managed lock,
preview, staged review, destructive confirmation, provider-backed work,
remote work, or support / admin handoff, it emits a record or appears
inside a domain packet that embeds an equivalent record.

## Control Families

Every conforming control declares exactly one family.

| Family | Use | Required distinctions |
| --- | --- | --- |
| `button` | One direct action. | Variant, command ref, actionability, busy / pending state, destructive posture. |
| `icon_button` | Compact direct action. | Accessible name, tooltip / help route, keyboard equivalent, 28 px minimum hit target. |
| `split_button` | Default action plus alternate variants. | A clear default action, menu route, and default-action review posture. |
| `checkbox` | Independent boolean or batch inclusion. | Checked state, label, staged-vs-live timing, disabled / locked reason. |
| `radio` | One choice from a mutually exclusive set. | Group label, selected option, roving keyboard path, source label when inherited. |
| `toggle` | Binary state that can apply immediately or stage. | Live-vs-staged timing is explicit; policy locks render as locked, not disabled. |
| `text_input` | Free-form or structured text. | Persistent label, helper text, clear / reveal affordance where relevant, inline validation. |
| `select` | Choice from a bounded set. | Current option, disabled option reasons, source label, keyboard open / choose behavior. |
| `combobox` | Searchable or fuzzy choice. | Text input plus option list semantics, async search / validation state, clear behavior. |
| `segmented_control` | Small finite mode switch. | Option labels, selected mode, arrow-key model, not large-workflow navigation. |
| `form_submit_action` | Commits a form or staged record. | Validation rollup, busy submit state, mutation-blocking tier, support evidence. |
| `form_review_action` | Opens preview, review, diff, policy, or effective-value detail. | Review target, source / scope summary, no misleading apply language. |

Family-specific packets may add anatomy, variants, or domain options,
but they must not redefine the state meanings frozen here.

## Size And Density

Controls use one size ladder:

| Size | Visual height | Default use |
| --- | ---: | --- |
| `small` | 24 px | Dense toolbars, compact rows, table actions. |
| `medium` | 28 px | Default command bars, field rows, review actions. |
| `large` | 32 px | Dialog primary actions, onboarding, accessibility-forward flows. |

Every interactive target keeps at least a 28 px hit target. A visually
small control may use padding, invisible hit slop, or row-level target
expansion to meet the target without changing its visual height.

Controls also declare `density_class` as `compact`, `standard`, or
`comfortable`. Density affects spacing, padding, row rhythm, and chrome
density only. It must not change command semantics, focus visibility,
source explanation availability, validation behavior, or whether review
is required.

Busy and pending controls preserve layout width. A submit button may
show a spinner, progress, or "Submitting" style label, but the label
meaning and button footprint stay stable while work is pending.

## Shared State Grammar

Every control maps local state to the shared component taxonomy. The
core distinctions are load-bearing:

- `disabled` means no valid action exists in the current context. It is
  not used for policy, trust, permission, managed authority, source
  authority, or missing capability.
- `locked` means a policy, managed authority, permission, trust,
  source, ownership, or capability constraint prevents editing or
  action. Locked controls show source and reason, plus an inspect route
  when one exists.
- `read_only` means content remains inspectable, copyable, navigable,
  or exportable, but cannot be edited or written.
- `loading` means context is not ready and was not triggered by a just
  submitted user action.
- `pending` means a user action was submitted, staged, queued, or is
  awaiting validation / commit.
- `destructive` marks action consequence, not merely color. It requires
  object / scope naming and a review or recovery path when available.

Controls declare `actionability_class` separately from visual state:

| Class | Meaning |
| --- | --- |
| `enabled` | Activation is currently allowed. |
| `disabled_no_valid_action` | No valid action exists in this context; show a concise reason when useful. |
| `read_only_inspectable` | Mutating action is unavailable, but inspect / copy / open source remains. |
| `locked_policy_or_managed` | Policy or managed authority pins, narrows, or blocks the action. |
| `locked_permission_or_trust` | Permission, trust, ownership, or capability blocks the action. |
| `busy_temporarily_blocked` | The same action is already submitted or running. |
| `pending_validation_blocked` | Required validation or probe has not resolved. |
| `preview_required` | Preview / dry run must be opened before apply. |
| `review_required` | Review, approval, or staged sheet is required before commit. |
| `destructive_confirmation_required` | A destructive action requires confirmation or review before commit. |

## Value Source Semantics

Value-bearing controls declare the value source even when the source is
only a compact label:

| Source state | Meaning | Required cue |
| --- | --- | --- |
| `default_value` | Built-in, schema, adapter, or release default. | Source label or effective-value inspector where source matters. |
| `detected_value` | Inferred from workspace, runtime, provider, environment, or probe. | Detected source label and revalidate / inspect path when stale. |
| `imported_value` | Migration, profile import, sync pull, package metadata, or external snapshot. | Imported source label and reset / review path where editable. |
| `user_value` | Explicit user choice or edit. | Dirty / applied / staged state when mutation is delayed. |
| `policy_value` | Policy, managed admin, entitlement, kill switch, or signed bundle. | Lock state, policy source, explanation action, no enabled mutating primary action. |
| `staged_value` | Edited draft not yet committed. | Dirty state, staged value, apply / revert controls, validation state. |
| `live_value` | Current applied value. | Live-change note and undo / revert path when immediate. |

`default`, `detected`, and `imported` are not synonyms. An imported
value may become the current profile layer; it still carries imported
provenance until reset, overwritten, or explicitly adopted by a
higher-precedence source.

## Labels, Help, Validation, And Source Labels

Controls use stable content slots:

- Action labels are verb-first where practical: `Apply settings`,
  `Open preview`, `Delete cache`.
- Field labels are noun phrases with persistent visible labels.
  Placeholder-only labels are non-conforming.
- Boolean labels state the thing being toggled or included, not only
  the resulting value.
- Helper text names constraints, source, scope, or consequence. It is
  not the only place a critical blocker appears.
- Inline validation is attached to the control or group, is reachable
  by keyboard, and can move focus to the exact blocked value.
- Source labels use the source vocabulary above and remain available to
  assistive technology, support export, and search / help projections.
- Clear controls remove editable values; reset controls restore the
  next winning source. These are different commands.
- Reveal controls for secrets or sensitive values require explicit
  friction and never expose raw secret material in exported records.

## Async And Busy Feedback

Async controls disclose both work state and boundary crossing. A record
declares:

- `async_state_class`: not async, loading context, validating,
  submitted busy, queued, running locally, running remotely, running
  through a provider, waiting for review, completed, or failed;
- whether activation is blocked while work runs;
- whether pending validation is visible;
- progress form: none, spinner, labeled spinner, progress bar, or
  status text;
- cancel / retry affordance where domain rules allow; and
- boundary crossing: local only, local workspace, remote host,
  provider-backed, managed admin, or external network.

Loading and pending are distinct. A select whose options are still
loading uses `loading`. A submit button that the user just activated
uses `pending` / busy feedback. A provider-backed action must name the
provider or boundary class before or during work; it must not reuse the
same generic spinner text as a local-only action.

## Live, Staged, Preview, And Optimistic Updates

Controls declare one apply timing:

| Timing | Allowed use | Optimistic update rule |
| --- | --- | --- |
| `immediate_live_apply` | Low-risk local controls with no broad, secret, policy, remote, provider, or destructive side effect. | May update optimistically when reversible and local validation passed. |
| `staged_apply_required` | Settings, connection profiles, request environments, or forms where edits are reviewed before commit. | May update the local draft, not the live target. |
| `preview_first_apply_required` | Package, repair, migration, publish, route, database, provider, or broad workspace mutations. | Must open preview / dry run before commit; no optimistic target mutation. |
| `dry_run_then_apply` | Infra, package, repair, request replay, or runtime actions where plan effects must be validated. | Dry run stays non-mutating and attributable. |
| `policy_locked_no_apply` | Policy or managed authority forbids mutation. | No optimistic update; explanation action remains. |
| `observe_only_no_apply` | Support, audit, export, read-only, or history context. | No mutating action rendered. |

Optimistic updates are allowed only when all of these are true:

1. The mutation is local-only or local-workspace scoped.
2. The prior state has a clear undo, revert, or compensating route.
3. No external, provider, remote, managed-admin, policy, secret-bearing,
   broad workspace, destructive, or publish / install boundary is
   crossed.
4. Required local validation has passed.
5. The command graph admits the same behavior from keyboard, palette,
   automation, and UI invocation.

Preview or review is required when any of these are true:

- the action is destructive or externally visible;
- the target is remote, provider-backed, managed, policy-owned, or
  secret-bearing;
- the action installs, updates, publishes, rolls back, exports,
  deletes, repairs, migrates, or changes route / trust / permission
  state;
- async validation is pending, stale beyond grace, skipped-required, or
  policy-blocked; or
- the action has broad workspace impact or cannot be exactly undone.

## Family-Specific Extension Rules

Family packets may extend this contract with local anatomy and richer
variants. They must keep these meanings intact:

- Button packets may add primary / secondary / quiet / destructive
  variants, but `pending`, `disabled`, `locked`, and `destructive`
  retain the meanings above.
- Split-button packets must identify the default action. If the default
  action requires preview or is blocked, the primary face exposes that
  posture; alternates do not silently become the default.
- Input packets may add masks, autocomplete, typeahead, clear, reset,
  or reveal behavior, but imported / detected / default source labels,
  validation, and secret reveal friction remain required.
- Toggle, checkbox, radio, and segmented-control packets may add option
  layout rules, but live-vs-staged timing and selected-vs-current
  semantics stay explicit.
- Form action packets may add validation summaries or domain previews,
  but busy submit, pending validation, policy lock, destructive review,
  and support-handoff semantics stay inherited from this contract.

## Keyboard And Accessibility

Every control record names:

- focus model: normal tab stop, roving tabindex, delegated input focus,
  or not-focusable with explanation;
- focus-visible treatment, which is always required for keyboard and
  assistive-technology focus;
- activation keys and navigation keys;
- Enter / Space behavior, including whether Enter submits, opens,
  chooses, inserts a newline, or does nothing;
- Escape behavior for menus, comboboxes, reveal controls, pending work,
  and sheets;
- accessible name and optional description;
- whether state, busy, validation, and source changes are announced;
  and
- focus return after a review sheet, menu, validation jump, or
  cancellation.

Hover-only disclosure is insufficient for disabled, locked, busy,
pending-validation, source, imported, destructive, or review-required
meaning.

## Surface Mapping

The same record vocabulary is used across launch-critical surfaces:

| Surface | Required mapping |
| --- | --- |
| Settings | Policy / managed values render as locked, imported profile values keep imported source labels, broad writes stage before apply. |
| Onboarding | Default, detected, imported, and user values remain distinct; optional account or provider actions name boundary crossing. |
| Install / update review | Install, update, rollback, remove, and publish controls use preview / review timing and destructive posture where applicable. |
| Admin surfaces | Policy locks show source, owner, freshness, and explanation; disabled means ordinary context unavailability only. |
| Provider-backed forms | Remote / provider async work names boundary, pending validation, stale results, and review route. |
| Command surfaces | Buttons, menus, palette entries, keybindings, automation, and CLI projections resolve to the same command and disabled-reason semantics. |

## Conformance

A control family conforms when:

- disabled, locked, read-only, loading, pending, destructive, staged,
  imported, detected, and default states retain the meanings above;
- opacity or color alone never carries state;
- busy submit and pending validation are distinct from initial loading;
- policy / managed locks expose source and explanation;
- source labels survive compact density, keyboard navigation,
  accessibility announcements, support export, and search / help
  projections;
- immediate live apply is limited to low-risk, reversible local
  controls;
- preview / review sheets are required for destructive, external,
  remote, provider, policy, secret, broad workspace, install, repair,
  migration, publish, route, and trust / permission changes; and
- family-specific packets extend this contract without redefining the
  reserved state semantics.

The fixture corpus is the executable review checklist for this
revision. Any new enum value or behavior branch added to the schema
must add or update a fixture in the same change.
