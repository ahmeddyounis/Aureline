# Disabled-reason grammar, alternate-route, and why-unavailable microcopy contract

This document freezes the rendered explanation grammar for commands and
command-like actions that are blocked, degraded, hidden with reason,
preview-only, read-only, or otherwise unavailable. It sits above the
command descriptor and diagnostics records: those records decide the
typed reason; this contract decides how that reason is spoken across
palette rows, menus, command bars, banners, status items, diagnostics,
docs/help, support exports, and accessibility narration.

Machine-readable companions:

- [`/artifacts/ux/disabled_reason_classes.yaml`](../../artifacts/ux/disabled_reason_classes.yaml)
  defines the canonical reason classes, source-code mappings,
  controlled terms, required fields, placeholder rules, and surface
  projections.
- [`/fixtures/commands/why_unavailable_cases/`](../../fixtures/commands/why_unavailable_cases/)
  contains worked examples for policy, dependency, degraded index,
  wrong target, read-only, host unavailable, unsupported mode,
  exhausted budget, preview-only, and stale cached-context cases.

This contract does not replace `disabled_reason_code` in the command
descriptor or `reason_code` in command diagnostics. It groups those
machine codes into user-facing reason classes and pins the grammar,
alternate-route requirements, and translation-safe placeholder rules
that every rendering surface consumes.

## Purpose

Aureline surfaces must not invent local "disabled" strings. A single
blocked command should answer the same questions wherever the user sees
it:

1. What action is unavailable?
2. What current cause blocks or narrows it?
3. What is the safest next step?
4. Is there a concrete alternate route?
5. Is the explanation live, cached, or stale?
6. Which docs/help anchor carries deeper detail?

If any surface cannot answer those questions from structured records, it
must render an inspectable degraded explanation rather than a generic
label such as "Unavailable", "Not allowed", "Try again", "See
settings", or "Check docs".

## Scope

Frozen here:

- the canonical reason-class layer used by command palette rows, menus,
  toolbars, status surfaces, banners, diagnostics, docs/help, support
  export, and accessibility narration;
- the required explanation fields every why-unavailable record carries;
- compact, short, detail, and docs/help grammar slots;
- concrete alternate-route rules and the null-alternate posture;
- live, cached, and stale explanation-freshness language; and
- translation-safe handling for counts, missing target classes, scopes,
  narrowed result sets, policy owners, hosts, budgets, and dependency
  labels.

Out of scope:

- implementation of repair flows;
- localization of every final string; and
- new command router behavior or new disabled-reason machine codes.

## Relationship to command records

The source chain is:

| Layer | Owns | This contract consumes |
| --- | --- | --- |
| Command descriptor | `command_id`, label refs, preview/approval posture, `disabled_reason_code`, docs/help anchor | action identity and machine reason |
| Command registry | user label, summary, origin/target badges, discoverability records | action label and projection source refs |
| Command diagnostics | remediation ref, authority, target/origin badge, policy source, parity surfaces | cause owner, next step, diagnostics parity |
| Palette/menu/status rows | compact placement and surface affordance | surface-specific slot, never new prose meaning |
| Localization catalog | message ids, placeholder kinds, fallback state | translatable templates and fallback disclosure |

The reason class is derived from the machine reason and context. A
surface may not choose a different reason class for the same
`command_id`, `command_revision_ref`, policy epoch, trust state,
issuing surface, UI slot, and execution context tuple.

## Canonical reason classes

The class ids below are the user-facing vocabulary. The artifact maps
each class back to command descriptor codes, diagnostics rows, state
tokens, and fixture cases.

| Reason class | Controlled chip | Use when | Default next-step posture |
| --- | --- | --- | --- |
| `policy_blocked` | `Policy blocked` | Admin, organization, trust, publisher, kill-switch, or managed policy blocks the action. | Open policy detail or continue through an explicitly allowed local/read-only route. |
| `dependency_missing` | `Dependency missing` | A provider, credential, runtime, language pack, tool, or capability is absent or unlinked. | Install, connect, select, or authenticate the named dependency. |
| `degraded_index` | `Index degraded` | Semantic, project, graph, search, diagnostic, or provider index cannot support the ideal action yet. | Use the lower-fidelity search or current-file route, or open index status. |
| `wrong_target` | `Wrong target` | Focus, selection, target class, argument, client scope, or surface cannot host the action. | Move focus, select the required target class, or open the supported surface. |
| `read_only` | `Read-only` | Inspection, copy, or export is allowed, but mutation is blocked by snapshot, restore, trust, or authority state. | Open live source, request write authority, or keep inspecting/copying. |
| `host_unavailable` | `Host unavailable` | A local, remote, container, provider, browser, or managed host cannot currently run the action. | Reconnect, reauthenticate, or continue locally/cached where safe. |
| `unsupported_current_mode` | `Unsupported here` | The current mode, client, lifecycle, channel, platform, or command version does not support the action. | Switch to a supported mode/client or use the supported command route. |
| `quota_or_budget_exhausted` | `Budget exhausted` | A quota, spend cap, rate limit, activation budget, or managed allowance is exhausted. | Reduce scope, wait for refresh, open budget detail, or use an offline/manual route. |
| `preview_only` | `Preview required` | The action cannot apply until preview/review is opened, or only preview/export remains safe. | Open the preview/review route; do not imply apply is available. |
| `stale_context` | `Context changed` | A cached explanation, basis snapshot, provider result, policy epoch, or target changed before apply. | Refresh, re-preview, or re-run against the current context. |

These are not synonyms. For example, `host_unavailable` means an
execution owner cannot answer; `dependency_missing` means a required
dependency has not been connected or installed; `degraded_index` means a
lower-fidelity route still exists because the richer index is partial.

## Required explanation fields

Every why-unavailable record carries the following fields, even if a
compact surface renders only a subset:

| Field | Required content | Rule |
| --- | --- | --- |
| `reason_class_id` | One class from the canonical set. | Must be derived from typed command/diagnostic/state data. |
| `action_label` | Canonical action label plus label ref. | The rendered label may localize, but the command id and label ref do not change. |
| `current_cause` | Plain cause, owner/source, affected scope, and machine reason refs. | The cause must say what is true now, not what might be wrong. |
| `safe_next_step` | One bounded action or read-only guidance. | Must preserve work and revalidate policy/trust/preview gates. |
| `alternate_route` | Concrete command, surface, handoff, or null with reason. | Generic "see settings" or "check docs" does not satisfy this field. |
| `explanation_freshness` | `live_evaluated`, `cached_snapshot`, or `stale_snapshot`. | Cached/stale explanations must disclose age or invalidation cause. |
| `docs_help_anchor_ref` | Existing docs/help anchor. | Help may deepen detail; it is not a substitute for the next step. |
| `surface_projection` | Surfaces allowed to render the explanation. | Each surface renders the same class, cause, next step, and alternate route. |
| `message_template_refs` | Compact, short, detail, status/banner, diagnostics, and docs template ids. | Runtime rendering resolves message ids, not ad hoc strings. |
| `placeholder_values` | Typed placeholders and privacy/redaction class. | Raw paths, raw hosts, account handles, secrets, prompts, and private URLs stay out of source copy. |

## Grammar slots

### Compact chip

Use two to four words from the controlled chip vocabulary:

- `Policy blocked`
- `Dependency missing`
- `Index degraded`
- `Wrong target`
- `Read-only`
- `Host unavailable`
- `Unsupported here`
- `Budget exhausted`
- `Preview required`
- `Context changed`

Do not use `Disabled`, `Unavailable`, `Error`, `Not allowed`, or
surface-local synonyms when a class is known.

### Short line

Use this sentence shape:

```text
{action_label} {availability_verb} because {current_cause}.
```

The availability verb is chosen by class:

| Class | Verb phrase |
| --- | --- |
| `policy_blocked` | `is blocked` |
| `dependency_missing` | `requires {dependency_label}` |
| `degraded_index` | `is limited` |
| `wrong_target` | `needs {target_class}` |
| `read_only` | `is read-only here` |
| `host_unavailable` | `cannot reach {host_class}` |
| `unsupported_current_mode` | `is not supported here` |
| `quota_or_budget_exhausted` | `is paused` |
| `preview_only` | `needs preview first` |
| `stale_context` | `needs refresh` |

### Detail line

Use this order:

```text
{action_label}: {current_cause}. {safe_next_step}. {alternate_route_or_null}. {freshness_if_not_live}
```

For example:

```text
Rename Symbol: the semantic index is rebuilding for this workspace. Use text search now or open index status. Semantic rename will re-enable after the index is current.
```

### Docs/help paragraph

Docs/help may add background and examples, but it must preserve the same
reason class, current cause, safe next step, alternate route, and
freshness claim as the live surface. Help content must not imply a
repair path the current policy, host, mode, budget, or target cannot
actually offer.

## Alternate-route rules

An alternate route is valid only when all of these are true:

1. It names a concrete command id, surface, handoff, or read-only route.
2. It does not widen authority, scope, target, network egress, or write
   posture compared with the blocked action.
3. It says what capability is lost or narrowed.
4. It revalidates trust, policy, permissions, preview, approval,
   credential, host, and freshness gates at activation.
5. It has a truthful null state when no alternate exists.

Allowed examples:

- `Use text search in the current workset`
- `Open cached read-only source`
- `Open local preview only`
- `Export metadata only`
- `Open command preview`
- `Reconnect remote host`
- `Switch to desktop command surface`

Forbidden examples:

- `See settings`
- `Check docs`
- `Try again`
- `Contact admin` without policy source and inspect path
- `Open anyway`
- `Use fallback` without naming what the fallback does

## Surface projection

| Surface | Required rendering |
| --- | --- |
| Command palette row | Compact chip, short line on selection, next-step action in footer, docs/help anchor, why-unavailable detail. |
| Menu/context menu/toolbar | Compact disabled reason or why-unavailable affordance; focusable disabled row when discoverability matters. |
| Status item/status strip | Reason class, affected scope, freshness, and inspect path; no hue-only status. |
| Banner | Cause, preserved capability, safe next step, concrete alternate route or null reason. |
| Command diagnostics | Full record: class, machine reason codes, owner/source, remediation, target/origin badge refs, freshness, docs/help anchor. |
| Docs/help | Same class and next-step guidance as the live record, with citations and source-language fallback support. |
| Support export | Stable ids, class, machine reason refs, redacted placeholders, freshness, and route refs; no rendered prose scraping. |
| Accessibility narration | Action label, reason class, cause, next step, alternate route, and focus path. |

## Translation-safe microcopy rules

1. **Use stable message ids.** Runtime strings resolve through message
   catalog ids. Business logic, command routing, policy evaluation,
   analytics, and support tooling never parse localized prose.
2. **Use typed placeholders.** Placeholders declare kind and redaction:
   count, action label, scope label, target class, dependency class,
   policy owner, host class, budget name, relative time, docs anchor,
   command id token, or freeform string.
3. **Use plural/select templates for counts.** Counts are not string
   concatenated. Use forms such as:

   ```text
   {blocked_count, plural, one {# blocked} other {# blocked}}
   ```

4. **Name count semantics.** `matching`, `selected`, `included`,
   `excluded`, `blocked`, `hidden by policy`, `hidden by filter`,
   `current workset`, and `workspace` are distinct terms.
5. **Do not hide scope in adjectives.** Say `current file`,
   `selected files`, `current workset`, `whole workspace`,
   `policy-hidden services`, or `cached snapshot`.
6. **Target classes are nouns.** Use `Select a symbol`, `Select a
   branch`, `Open an editor tab`, or `Choose a runtime`, not "pick
   something".
7. **Policy owners are placeholders.** Use `{policy_owner}` or
   `{policy_source}`; do not bake organization names, tenants, URLs, or
   bundle ids into source text.
8. **Freshness is explicit.** Live explanations may omit freshness in
   compact slots. Cached or stale explanations must say `from cached
   context`, `last checked {relative_time}`, or `refresh required`.
9. **No variable inside a word.** Placeholders stand alone so languages
   can reorder or inflect around them.
10. **Machine tokens stay locale-neutral.** Command ids, JSON keys,
    schema enum values, and policy ids are never translated.

## Count and scope phrasebook

| Meaning | Source-language form |
| --- | --- |
| Blocked subset | `{blocked_count, plural, one {# blocked} other {# blocked}} by {cause_owner}` |
| Hidden by policy | `{hidden_count, plural, one {# hidden by policy} other {# hidden by policy}}` |
| Narrowed scope | `{scope_label} only` |
| Missing target | `Select {target_class} to continue` |
| Dependency missing | `Connect {dependency_label}` or `Install {dependency_label}` |
| Cached explanation | `Last checked {relative_time}` |
| Stale context | `Refresh current context` |
| No alternate | `No alternate route is available under {cause_owner}` |

## Acceptance checklist

A why-unavailable explanation is conforming when:

1. It resolves to exactly one canonical `reason_class_id`.
2. It carries the required fields listed above.
3. Palette, menu, status, banner, diagnostics, docs/help, support, and
   accessibility surfaces render the same reason class and next-step
   semantics for the same projection key.
4. Any alternate route is concrete, bounded, and truthful about lost
   capability.
5. Null alternate routes say why no route exists.
6. Quantities, scopes, target classes, dependencies, owners, freshness,
   and machine tokens use typed placeholders.
7. Cached or stale explanations disclose freshness.
8. The docs/help anchor deepens the explanation without replacing the
   immediate safe next step.

## Source anchors

- [`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  for command identity, disabled-reason codes, preview/approval posture,
  and invocation-session outcomes.
- [`docs/ux/command_diagnostics_contract.md`](./command_diagnostics_contract.md)
  for diagnostics projection, remediation, target/origin badges, and
  parity surfaces.
- [`docs/commands/palette_row_contract.md`](../commands/palette_row_contract.md)
  for palette row and action-footer projection.
- [`docs/ux/menu_command_bar_contract.md`](./menu_command_bar_contract.md)
  for menu, context-menu, toolbar, and command-bar disabled disclosure.
- [`docs/ux/state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md)
  and [`docs/ux/degraded_mode_pattern.md`](./degraded_mode_pattern.md)
  for state placement, preserved capability, recovery, and degraded
  state language.
- [`docs/ux/localization_and_locale_pack_contract.md`](./localization_and_locale_pack_contract.md)
  for message ids, placeholders, locale fallback, and source-language
  escape hatches.
