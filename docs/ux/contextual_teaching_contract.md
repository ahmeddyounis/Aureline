# Contextual Teaching Surface Contract

This document freezes the contract for Aureline teaching surfaces that
appear next to the user's current work: contextual tip cards, migration
bridge cards, why-unavailable explainers, and source-language fallback
surfaces. The goal is practical help that stays close to the blocked or
unfamiliar action, cites the source of its claim, and never creates a
private path around trust, policy, permission disclosure, or write-scope
review.

The machine-readable schema lives at:

- [`/schemas/ux/teaching_surface.schema.json`](../../schemas/ux/teaching_surface.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/migration_bridge_cards/`](../../fixtures/ux/migration_bridge_cards/)

This contract specializes the broader learnability model for the
small, in-context surfaces that users see while working. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, design-system style
guide, localization contract, learnability contract, command contract,
or migration-center object model, those sources win and this document,
its schema, and fixtures update in the same change.

## Companion Contracts

This contract does not re-mint upstream truth. It consumes these
contracts by reference:

- [`/docs/ux/learnability_contract.md`](./learnability_contract.md) and
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  - guided surface identity, citation authority, dismissal, reset,
  learning profile, and presentation adjunct rules.
- [`/docs/ux/learning_and_presentation_contract.md`](./learning_and_presentation_contract.md)
  - presentation-mode and learning-digest behavior used when tips are
  deferred.
- [`/docs/ux/localization_and_locale_pack_contract.md`](./localization_and_locale_pack_contract.md)
  and
  [`/schemas/ux/locale_fallback_state.schema.json`](../../schemas/ux/locale_fallback_state.schema.json)
  - locale fallback, source-language escape hatches, message IDs, and
  citation-preserving translation rules.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md),
  [`/docs/ux/command_diagnostics_contract.md`](./command_diagnostics_contract.md),
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  - canonical command IDs, disabled reasons, command docs pivots, and
  protected-entry review posture.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md),
  [`/schemas/migration/migration_session.schema.json`](../../schemas/migration/migration_session.schema.json),
  and
  [`/schemas/migration/importer_outcome.schema.json`](../../schemas/migration/importer_outcome.schema.json)
  - migration session, importer outcome, restore checkpoint, and bridge
  requirement truth.
- [`/docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md)
  and
  [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  - docs/help anchors, source references, freshness, and reuse-surface
  rules.
- [`/docs/ux/transient_surface_contract.md`](./transient_surface_contract.md)
  - anchored tooltip, hovercard, popover, peek, and promotion behavior.
- [`/docs/verification/source_fidelity_and_undo_packet.md`](../verification/source_fidelity_and_undo_packet.md)
  and
  [`/docs/verification/migration_and_profile_packet.md`](../verification/migration_and_profile_packet.md)
  - write-scope review, undo, and migration restore evidence.

## Scope

The schema freezes three record kinds:

| Record kind | Purpose |
|---|---|
| `teaching_surface_record` | One rendered or deferred teaching surface. |
| `teaching_surface_rule_record` | One rule row per surface kind. |
| `teaching_surface_manifest_record` | One aggregate row proving the invariant set for a fixture pack, test corpus, or release lane. |

The surface kinds are closed:

| Surface kind | Job |
|---|---|
| `contextual_tip_card` | Explain a nearby control, shortcut, or workflow briefly. |
| `migration_bridge_card` | Explain how an imported behavior maps into Aureline. |
| `why_unavailable_explainer` | Explain why an action is disabled, hidden, degraded, or unsafe here. |
| `source_language_fallback_surface` | Keep untranslated docs/help/training surfaces linkable to canonical source-language truth. |

This contract does not author final onboarding copy, localization copy,
or full tutorial content. It freezes the fields every such surface must
carry before it renders.

## Required Surface Fields

Every `teaching_surface_record` carries:

- **Identity** - `surface_id`, `surface_kind`, `rule_ref`, schema
  version, locale, target build, and deployment profiles.
- **Trigger** - the event or state that made the surface relevant,
  such as `blocked_action`, `unfamiliar_action`,
  `imported_behavior_used`, `docs_locale_missing`, or
  `source_language_requested`.
- **Placement** - one of the allowed local placements:
  `inline_near_anchor`, `row_adjacent`, `status_item_detail`,
  `inspector_adjacent`, `popover_attached`, or
  `learning_digest_only`. Full-screen takeover is not a valid value.
- **Owning boundary** - the owner of the rule or state being explained:
  command registry, workspace trust, admin policy, runtime capability,
  execution context, docs locale pack, migration importer, keybinding
  resolver, extension bridge, write-scope review, source file or
  symbol, or input/accessibility mode.
- **Concrete source references** - at least one governed reference:
  command ID, docs/help anchor, file ref, symbol ref, keybinding
  resolver ref, migration session ref, importer outcome ref, policy ref,
  schema ref, locale-pack ref, extension-bridge ref, source-language
  message ID, or support packet ref.
- **Command refs and docs pivots** - command refs are present when the
  surface teaches or explains a command; docs/help pivots are required
  for deeper explanation.
- **Dismiss and snooze rules** - dismissal class, snooze class,
  quiet-hours behavior, presentation-mode behavior, and the explicit
  reversal path.
- **Actions** - typed actions such as `open_docs`, `open_source`,
  `prepare_preview`, `open_diff`, `request_trust`, `open_policy`,
  `undo_import`, `view_mapping_details`, or `open_local_fallback`.
  Each action declares whether it is read-only, preview-before-write,
  trust-prompt-gated, policy-review-gated, write-scope-review-gated,
  or not actionable.
- **Trust/policy/write-scope guard** - const-true fields proving that
  the surface preserves trust prompts, policy review, permission
  disclosure, write-scope preview, and separation between explanation
  and action.
- **Freshness and fallback** - freshness, active/deferred state,
  locale fallback disclosure, policy context, redaction class, and
  source-language payload when applicable.

## Migration Bridge Cards

Migration bridge cards are for imported or bridged behavior, not for
generic marketing claims. A card must name:

- `imported_behavior_domain` in the closed set:
  `keymaps`, `settings`, `snippets`, `task_configs`,
  `extension_bridges`;
- `imported_behavior_state` in the closed set:
  `native`, `bridge`, `shimmed`, `partial`, `unsupported`;
- source product, source behavior ref, target behavior ref when one
  exists, mapping basis, parity notes, unsupported edge refs, bridge
  refs when relevant, docs/help refs, and rollback checkpoint ref when
  imported state changed.

State meanings:

| State | Meaning |
|---|---|
| `native` | The behavior maps to first-party Aureline behavior without compatibility narrowing. |
| `bridge` | The behavior runs through a declared compatibility layer or adapter. |
| `shimmed` | Aureline emulates a subset of the old behavior and must disclose limitations. |
| `partial` | Only some behavior carried over; omitted parts stay visible in the card and report. |
| `unsupported` | No truthful mapping exists; the card offers docs, review, or rollback, not false parity. |

Unsupported and partial outcomes are end states. They may not collapse
into a quiet `skipped` or `best effort` message.

## Why-Unavailable Explainers

A why-unavailable explainer answers "why can't I do this here?" without
making the user infer authority state from a greyed-out button. It must
name:

- the blocked action;
- availability state (`available`, `degraded`, `unavailable`,
  `hidden_by_policy`);
- typed reason (`command_disabled`, `policy_blocked`,
  `trust_required`, `runtime_missing`, `context_not_supported`,
  `docs_or_locale_missing`, `extension_bridge_missing`,
  `write_scope_review_required`, `source_unavailable`,
  `not_supported`);
- owning boundary;
- next safe action;
- docs/help pivot.

The next safe action must preserve the active boundary. For example,
opening a local preview, opening policy details, or preparing a diff is
valid; silently enabling trust, widening policy, or applying a mutation
is not.

## Source-Language Fallback

A source-language fallback surface renders when requested localized
content is missing, stale, disabled, or policy-blocked. It must preserve:

- requested locale, effective locale, and source locale;
- canonical message ID or docs/help anchor;
- fallback reason;
- source-language open action;
- stable identity preservation;
- locale fallback disclosure.

The fallback is not a new copy source. It is a linkable projection of
the canonical source-language entry, so support, docs, and training can
trace what the user saw back to a stable message ID or docs anchor.

## Dismissal, Snooze, Quiet Hours, and Reversal

Dismissal and snooze state must be reversible and scoped:

| Field | Required behavior |
|---|---|
| `dismissal_class` | `per_session`, `per_profile`, `per_device`, `policy_locked`, or `not_dismissible`. |
| `snooze_class` | `none`, `until_next_session`, `duration_bounded`, `until_context_changes`, `quiet_hours_digest_only`, or `presentation_mode_digest_only`. |
| `quiet_hours_behavior` | Non-critical surfaces defer to digest or suppress; blocking surfaces may render if needed. |
| `presentation_mode_behavior` | Non-critical surfaces hide or digest; presenter-anchored or blocking explanations may render. |
| `reversal_path_kind` | Help/Learning Digest, command palette reset, settings learning state, migration-center restore, source-language toggle, policy admin only, or none. |

If a user dismisses or snoozes a teaching surface, Help and the command
palette must still offer a recovery path unless policy owns the state.
Quiet-hours and presentation-mode deferral must collect non-critical
teaching in a durable digest rather than losing it.

## Trust, Policy, and Write-Scope Review

Teaching surfaces are explanatory. They do not gain authority by being
helpful. A conforming record must keep these invariants true:

- trust prompts still gate trust escalation;
- policy review still gates policy-owned actions;
- permission disclosure still names extension, runtime, provider, or
  account permissions;
- write-scope preview still gates mutations;
- `Explain` and `Do` stay separate controls;
- mutating actions resolve to canonical command IDs and review packets;
- migration rollback remains a migration-center or restore-record
  action, not an inline hidden mutation.

## Denial Reasons

When a publisher cannot satisfy this contract, it denies or defers with
one of these typed reasons instead of rendering vague prose:

- `teaching_surface_without_concrete_source_ref`
- `teaching_surface_without_deeper_doc_pivot`
- `teaching_surface_not_near_trigger_anchor`
- `teaching_surface_dismissal_without_reversal`
- `teaching_surface_quiet_hours_bypass`
- `teaching_surface_presentation_mode_bypass`
- `teaching_surface_missing_migration_state`
- `teaching_surface_unsupported_import_hidden`
- `teaching_surface_source_language_identity_lost`
- `teaching_surface_locale_fallback_not_disclosed`
- `teaching_surface_trust_policy_or_write_scope_bypassed`
- `teaching_surface_explain_and_do_collapsed`
- `teaching_surface_schema_version_lagging`

## Acceptance Mapping

| Acceptance clause | Resolved by |
|---|---|
| Help appears nearest to the blocked or unfamiliar action instead of hijacking the whole screen. | Placement vocabulary and rule rows; full-screen takeover is not a valid placement. |
| Teaching surfaces preserve trust, policy, and write-scope review. | Trust/policy/write-scope guard const-true fields and action review posture. |
| Source-language fallback remains linkable to canonical docs/help and preserves stable IDs or citations. | Source-language payload, source refs, docs pivots, locale fallback disclosure, and stable identity invariant. |
| Migration bridge cards honestly describe imported behavior. | Domain/state vocabulary, bridge payload, unsupported edge refs, docs/help refs, rollback checkpoint refs. |

## Worked Examples

Fixtures under
[`/fixtures/ux/migration_bridge_cards/`](../../fixtures/ux/migration_bridge_cards/)
cover:

1. a contextual tip anchored near a failed symbol-search action;
2. keymap, settings, snippet, task-config, and extension-bridge
   migration bridge cards spanning native, bridge, shimmed, partial, and
   unsupported states;
3. a why-unavailable explainer that preserves policy and write-scope
   review; and
4. a source-language fallback surface that preserves a canonical docs
   anchor and source-language message ID.

## Out of Scope

- Final onboarding content or localized wording.
- Runtime implementation of importers, bridges, or docs pack loading.
- Visual card styling beyond the placement and action contract.
- Analytics beyond the refs and record fields frozen here.
