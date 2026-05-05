# Field-row and value-source contract

This document freezes the shared row contract used when a form,
settings surface, request editor, transport review, package action,
repair flow, or support projection needs to show one editable or
inspectable value. It complements the staged-review and high-risk
field-rule contracts: those contracts decide how a change is validated
and why a field is risky; this contract decides how the row itself
shows label, value, source, state, help, actions, effective value, and
object references.

The contract is normative. Where this document disagrees with the
source UI / UX, settings, policy, form-validation, or field-risk
contract it cites, the source contract wins and this document plus its
schema and fixtures update in the same change. Where this document
disagrees with a downstream surface's private row wording, this
document wins and the surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/field_row.schema.json`](../../schemas/ux/field_row.schema.json)
  - boundary schema for one `field_row_record`. It carries row anatomy,
    source-pill metadata, effective-value inspector state, object
    references, exact-row links, search highlighting, and optional
    extension slots for path basis, structured preview, and multi-value
    rules.
- [`/fixtures/ux/field_rows/`](../../fixtures/ux/field_rows/)
  - worked JSON records covering a policy-locked settings row, stale
    object references under narrowed permission, an external-vault
    source, and a preview-first multi-value row.
- Settings specialization:
  - [`/docs/settings/settings_row_contract.md`](../settings/settings_row_contract.md)
    for settings-row deep links, search landing/highlight, lock behavior,
    and reset/diff affordances.
  - [`/schemas/settings/settings_row_state.schema.json`](../../schemas/settings/settings_row_state.schema.json)
    for the settings-only specialization of `field_row_record`.
  - [`/fixtures/settings/setting_rows/`](../../fixtures/settings/setting_rows/)
    for additional settings-row examples (precedence, overrides, high-risk
    preview-first settings).
- [`/docs/ux/forms_validation_contract.md`](./forms_validation_contract.md)
  and [`/schemas/ux/staged_review_state.schema.json`](../../schemas/ux/staged_review_state.schema.json)
  - validation, staged apply, probe freshness, mutation-blocking, and
  lock explanation vocabulary. A staged-review `field_rows[]` entry
    is the compact projection of this richer row contract.
- [`/docs/ux/field_rules_contract.md`](./field_rules_contract.md)
  and [`/schemas/ux/field_rule.schema.json`](../../schemas/ux/field_rule.schema.json)
  - high-risk field families and their redaction, validation, copy,
    paste, drop, duplicate, normalization, and evaluation-context rules.
- [`/docs/settings/settings_vocabulary.md`](../settings/settings_vocabulary.md)
  and [`/schemas/settings/effective_setting.schema.json`](../../schemas/settings/effective_setting.schema.json)
  - upstream settings source, shadow-chain, lock, preview, restart, and
    control-stack vocabulary consumed by settings rows.
- [`/docs/admin/policy_explainability_contract.md`](../admin/policy_explainability_contract.md)
  - policy source, owner, freshness, lock, deep-link, and export rules
    consumed by policy or managed rows.

Normative source sections projected here include the form, source, and
reference templates in `.t2/docs/Aureline_UI_UX_Spec_Document.md`; the
settings effective-value and schema-registry rows; and the admin policy
explainability contract. This document does not introduce a settings
resolver, picker widget implementation, policy evaluator, or package /
repair / request domain model.

## Scope

The `field_row_record` applies to any row that displays a value whose
source, writability, lock state, external dependency, or reference
identity matters. Covered rows include:

- settings rows and generated settings search results;
- request environment, assertion, and runtime parameter rows;
- transport, endpoint, certificate, and proxy rows;
- package action, registry auth, and lockfile-review rows;
- repair preview rows and project-doctor repair inputs;
- data connection, provider resource, and support-export rows; and
- read-only or inspect-only projections of the same rows in CLI /
  headless output.

Low-risk local UI controls MAY render without a full record while they
are inert, local, immediately reversible, non-secret, and not source
layered. The moment a row needs an effective-value explanation, a
policy or external-source explanation, a stable object reference, a
preview-first apply path, or support/export fidelity, it emits this
record.

## Row Anatomy

Every row has the same minimum anatomy.

| Element | Required meaning |
|---|---|
| `label` | Visible row label, required/optional/computed cue, and compact description. Placeholder-only labels are not conforming. |
| `value` | Redaction-aware current and effective value projection. Raw secrets, raw policy bodies, raw provider payloads, and sensitive raw paths stay out of the record. |
| `source_pill` | Short rendered source class and label. The pill is derived from source metadata, never inferred from value text. |
| `effective_value_inspector` | One-interaction path that shows the winning value, source chain, precedence order, stale or permission warnings, and source actions. |
| `state` | Validation state, apply posture, editability, lock state, dirty state, currentness, and a reviewable state summary. |
| `help` | Inline or one-interaction help that can open source, lock, policy, or external explanation details without losing row focus. |
| `primary_action` / `secondary_actions` | Row actions with typed availability. Disabled actions name the reason through state or an explanation ref. |
| `exact_row_deep_link` | Stable route that lands on this exact row and reveals source, effective value, and lock or override explanation. |
| `search_projection` | Optional projection that lets search or help hits highlight the matching label, value, source, stable id, or lock explanation on this row. |

Surfaces may condense or hide secondary details visually, but the
backing row record remains complete. A compact list row may show only
label, value, source pill, state icon, and one action; selecting the
row or opening the inspector must reveal the rest without changing row
identity.

## Apply Posture

Every row declares exactly one `apply_posture_class`.

| Posture | Meaning | Required cue |
|---|---|---|
| `immediate_live_apply` | A safe local value applies as soon as changed. | Live-change note and undo/revert path. |
| `staged_until_apply` | The value edits a draft and commits with a later apply action. | Dirty state, staged value, apply/revert controls, and affected scope. |
| `preview_first_apply` | The value can mutate code, provider, package, route, policy, repair, or broad workspace state. | Preview/dry-run/diff route, target basis, side-effect summary, and rollback or compensation note. |
| `policy_locked_no_apply` | Policy or managed authority pins or constrains the effective value. | Lock state, policy/source owner, explanation action, and what remains adjustable. |
| `external_source_inspect_only` | The effective value comes from an external source the row can cite but cannot mutate here. | External source label, permission/freshness warning when relevant, and open/copy-id action where allowed. |
| `observe_only_no_apply` | The row is read-only support, audit, export, or history context. | Inspect-only affordance and no mutating apply action. |

A row with `policy_locked_no_apply`, `external_source_inspect_only`, or
`observe_only_no_apply` must not render an enabled mutating primary
action. It may render inspection, revalidation, copy-stable-id, or
source-opening actions.

## Value Sources

All rows use one source vocabulary:

| Source class | Use |
|---|---|
| `policy` | Signed policy, managed admin, release/channel, emergency, or entitlement source. |
| `user` | Explicit user edit, user-global setting, temporary override, or command parameter. |
| `workspace` | Workspace, folder, project, repository, or environment-layer source. |
| `profile` | Selected or imported profile layer currently applied as a profile. |
| `imported` | Migration, import, sync pull, package/import metadata, or external snapshot source. |
| `external_vault` | Secret manager, credential broker, certificate store, provider-side object, or external source resolved by reference. |
| `default` | Built-in, schema, experiment, release-channel, or adapter default. |

When a domain has no more specific precedence contract, rows use this
default high-to-low order:

1. `policy`
2. `user`
3. `workspace`
4. `profile`
5. `imported`
6. `external_vault`
7. `default`

Domain contracts may refine the order by adding ranks inside
`source_entries[]`, but they may not rename the source classes. Policy
does not silently widen behavior: it pins or narrows the effective
value. External-vault sources contribute a resolvable value or handle;
they do not expose raw secret bytes or private provider payloads.
When a higher layer selects an external handle, the inspector shows both
the selecting source and the external-resolution source. The selecting
source keeps precedence; the external source carries freshness,
permission, redaction, and open-source warnings.

The effective-value inspector must show:

- the effective value projection and the source that won;
- every contributing source in precedence order;
- whether each source is winning, shadowed, constrained, fallback, or
  external-resolution only;
- freshness and inspection state for each source;
- the action that opens the source or explains why it cannot be opened;
  and
- a warning when the current user cannot inspect a referenced source.

## Source-Pill Rendering

The source pill is compact but not decorative. It must preserve:

- source class;
- short source label;
- rendering tone or icon family;
- inspector action ref; and
- explanation action ref when a policy, lock, external source, stale
  source, or permission warning exists.

The pill may truncate visually, but the full label and source class
must remain available to assistive tech, search, support export, and
the effective-value inspector. A source pill must not display raw paths,
raw URLs, raw policy names that reveal tenant-private data, raw secret
aliases, or provider payload fragments.

## Object-Reference Picker

Object-reference fields preserve stable identity separately from the
human-friendly label. The `object_reference_picker` block is required
whenever a field value names a provider object, project, environment,
database, policy bundle, vault item, certificate, package registry,
route, runtime, notebook kernel, repair target, support packet, or
other object whose display label can drift.

The picker must show:

- current display label;
- previous display label when drift was detected;
- stable id path and stable id ref;
- reference state, including stale, missing, permission-lost, deleted,
  revoked, or provider-unreachable;
- permission note when the current user cannot inspect the object;
- stale-reference warning when the label, target, or authority changed;
- open-target action when allowed; and
- copy-stable-id action when export policy allows.

When a display label changes, the stable id path, stable id ref,
source-entry ref, permission note, and warning survive. The picker must
not replace the stable id with the new label, and search/help routes
must be able to reveal the stable id path even when the target cannot be
opened.

## Exact Rows, Search, And Help

`exact_row_deep_link` is the route contract for search, help, docs,
support, and CLI projections. Opening the link must land on one row,
focus that row, and reveal:

- visible source pill;
- effective value or redacted value projection;
- lock, override, policy, stale, or external-source explanation; and
- object-reference stable id when the row carries one.

Search hits do not invent a separate result-row model. They cite
`search_projection`, highlight the matching row fragment, and open the
same exact row projection the product surface would render in place.

Policy and external-source explanations must be inspectable in one
interaction from either the source pill, help affordance, disabled
action, or row state. Hover-only disclosure is insufficient for locked,
stale, permission-lost, or external-source rows.

## Extension Slots

The core anatomy must not be redefined when a row needs richer behavior.
The schema reserves `extension_slots` for optional details:

- `path_basis` for workspace-root, containing-file, target, runtime,
  remote, container, or user-chosen path basis disclosure;
- `structured_diff_preview` for current-vs-staged, generated,
  dry-run, rollback, or preview-first deltas;
- `multi_value_rules` for overflow counts, order significance,
  duplicate handling, hidden values, and inspect paths.

These blocks extend the row; they do not replace label, value, source,
state, help, action, or exact-row routing.

## Export And Handoff

Support bundles, issue handoff packets, CLI JSON, and admin/support
exports must preserve the machine-readable row state. They may redact
payloads, but they must not drop:

- row id and canonical field path;
- value redaction class and effective value ref;
- source class, source label, source entries, and winning source;
- apply posture, editability, lock state, and state summary;
- object stable id path and permission warning when present;
- exact-row deep link; and
- explanation and action refs that justify disabled or inspect-only
  behavior.

## Conformance

A surface conforms when:

1. Every high-impact row emits one `field_row_record` or a documented
   compact projection that points to the full record.
2. Users can distinguish editable, staged, preview-first,
   policy-locked, external-source, and inspect-only rows before acting.
3. The effective-value inspector shows the winning value, source chain,
   source precedence, stale/permission warnings, and open/explain
   actions in one interaction.
4. Object references preserve stable id path and warnings when labels
   drift or permissions narrow.
5. Search and help routes land on exactly one row with visible source,
   effective value, and lock or override explanation.
6. Path basis, structured diff/preview, and multi-value overflow or
   duplicate rules use extension slots instead of redefining core row
   anatomy.
7. Raw secrets, raw policy bodies, raw provider payloads, raw URLs with
   sensitive components, and sensitive raw paths stay out of the row
   record.

Adding a new enum value is additive-minor and requires updates to the
schema, this document, and at least one fixture. Repurposing an
existing value is breaking and requires a governance decision before a
surface consumes it.
