# Command Palette Query-Session, History, And Alternate Invocation Contract

This document freezes the command palette query session as a governed
launcher object. A palette session is not just a fuzzy-list widget: it owns
the current query text posture, active provider set, ranking phase, held
modifier intent, local-first history policy, redaction posture, and the
selected-row action footer links that decide what `Enter`, split/open-alt,
copy, recipe, and inspect actions mean.

Machine-readable companions:

- [`/schemas/commands/palette_query_session.schema.json`](../../schemas/commands/palette_query_session.schema.json)
  defines `palette_query_session_record`,
  `palette_query_history_entry_record`, and
  `palette_recent_query_set_record`.
- [`/schemas/commands/palette_action_footer.schema.json`](../../schemas/commands/palette_action_footer.schema.json)
  defines selected-row footer actions and alternate invocation rows.
- [`/schemas/commands/palette_result.schema.json`](../../schemas/commands/palette_result.schema.json)
  defines command result-row projections that cite this session family.
- [`/fixtures/commands/palette_query_cases/`](../../fixtures/commands/palette_query_cases/)
  contains worked cases for lexical-to-semantic streaming, bounded local
  history, clear-history behavior, and privacy-safe support export.

This contract composes with
[`palette_row_contract.md`](./palette_row_contract.md),
[`palette_row_and_modifier_contract.md`](./palette_row_and_modifier_contract.md),
[`command_descriptor_contract.md`](./command_descriptor_contract.md), and
[`shareability_and_automation_contract.md`](./shareability_and_automation_contract.md).
The command descriptor remains the canonical command object. The query
session is the canonical launcher state object.

## Scope

Frozen here:

- one typed query session that carries current text, provider set, ranking
  state, held modifier intent, history policy, local-first retention class,
  and redaction posture;
- lightweight history entries and recent-query sets with bounded retention,
  clear-history rules, and privacy-safe export behavior;
- action-footer links for default run/open, split/open-alt, open alternate
  target, copy command ID, copy CLI/headless form, add to recipe, and inspect
  why not automatable;
- transition rules that let recent and lexical results appear immediately and
  richer semantic/provider results stream later without losing current text,
  selected row, held modifier intent, history policy, or redaction posture.

Out of scope:

- final fuzzy ranking quality, scoring weights, provider performance, and
  visual styling;
- live command dispatch, approval ticket bodies, and preview/apply runtime;
- new command IDs, capability classes, disabled-reason vocabulary, or
  automation-safety vocabulary.

## Session Ownership

The palette opens one `palette_query_session_record` for a user-visible
launcher intent. Result rows, footer actions, docs/help projections, CLI help
rows, support captures, and automation builders may cite the session, but they
do not own its query text or privacy state.

Examples:

| Flow | Session rule |
| --- | --- |
| User types a command name, then holds `Alt` before semantic rows arrive | Same session; held modifier intent is preserved across provider frames. |
| Recent history returns before command-registry fuzzy results | Same session; recent and lexical groups are truth-labeled and may be replaced or appended only by declared transition rules. |
| Semantic command results arrive after the selected lexical row is focused | Same session; selected row remains stable unless the session declares selection can move. |
| User copies a CLI form from the footer | Same session; copy action does not dispatch and raw query text is not exported with the copied form. |
| User clears palette history | Same session may transition to `cleared_history`; recent-query set and history entries apply their declared deletion effects. |

Closing the palette does not create permission to retain raw query text.
Retention is admitted only by the session's history policy and local-first
retention block.

## Query Session Fields

Every session record carries these field groups:

| Field group | Required contents |
| --- | --- |
| Session identity | `palette_session_id`, schema version, session state, opened/updated/closed timestamps, projection surfaces. |
| Current text | text state, material class, storage class, query text ref, optional local-only raw text, query hash ref, locale, input mode, and secret-scan state. |
| Provider set | command registry, keybinding resolver, recent history, lexical index, semantic index, docs/help, settings, file/symbol, extension, or migration provider state with result phase and truth label. |
| Ranking state | ranking mode, active ranking stages, stable-ordering policy, reason export posture, ranker epoch, and visible/hidden counts. |
| Held modifier intent | default run, split/open-alt, alternate target, copy ID, copy CLI/headless form, add to recipe, inspect, or detail intent plus the footer action ref it resolves to. |
| History policy | no history, session-only, local profile, local device, or managed governed recent history, with max entries, max age, sensitive-literal handling, raw export flag, sync flag, and clear-history rules. |
| Local-first retention | storage class, bounded retention class, max entries, max age, workspace-untrust clearing, and low-disk eviction ordering. |
| Redaction posture | local/private, metadata-safe, support-redacted, policy-review, or export-denied posture with raw query export flag and support export material ref. |
| Action footer projection | selected footer ref plus default run, alternate invocation, copy, add-to-recipe, and inspect-why-not-automatable links. |
| Lightweight history | history-entry refs, recent-query set ref, and history policy ref. |
| Transition rules | from/to states, triggers, rank update policy, selection update policy, provider classes appended, and preservation constants. |

The palette may render materialized labels and raw local text for the active
user session, but any exportable or cross-surface object cites opaque refs,
hash refs, command refs, or redacted text refs.

## Query Text And Redaction

Raw query text is local by default.

- `raw_local_only` text may be present only with local-first retention,
  `raw_query_export_allowed: false`, and a support export posture that denies
  raw text.
- Exportable, support, CLI, docs, or managed projections use `redacted_text`,
  `hashed_terms_only`, `command_refs_only`, `classification_only`, or no query
  text.
- Secret detection state remains explicit. A session that cannot scan due to
  policy must not silently mark text safe.
- Redacted text is review copy, not proof that the original text contained no
  secrets.
- Salted query hashes are local comparison aids. The salt is local and is not
  reused across tenants, workspaces, or support packets.

A session that cannot prove its redaction posture denies export rather than
dropping fields and pretending the packet is complete.

## Provider And Ranking State

Provider state is explicit so fast and slow result sources can coexist without
misleading the user.

Required behavior:

- Recent and lexical providers may return first.
- Semantic or extension-backed providers may stream later.
- Each provider declares whether it is ready, streaming, partial, stale,
  blocked, unavailable, or complete.
- Ranking state declares whether later results can append groups, replace a
  provider group, rerank only unselected rows, freeze after commit, or leave
  ranking unchanged.
- Selected row stability is declared. If a user has a selected row, streaming
  semantic results do not steal selection unless the session explicitly
  allows it before selection exists.
- Ranking reasons exported outside the product are reason classes or redacted
  refs, not private numeric weights.

This lets the palette show immediate recent/lexical results while still
admitting richer semantic results later with no hidden state transition.

## Held Modifier Intent

The session owns held modifier intent before invocation. A footer may render
the action, but the intent itself remains session state so streaming providers
cannot reinterpret a held key.

Required intents:

| Intent | Footer/action meaning |
| --- | --- |
| `default_run` | Default run/open path for the selected row. |
| `split_or_open_alt` | Placement delta such as split pane or open alt. |
| `open_alternate_target` | Declared target change where the descriptor and enablement decision allow it. |
| `copy_command_id` | Copy canonical command ID only. No dispatch. |
| `copy_cli_headless_form` | Copy documented CLI/headless skeleton. No pre-approved token. |
| `add_to_recipe` | Insert typed recipe step preserving command identity and argument shape. No dispatch. |
| `inspect_why_not_automatable` | Open structured automation blocker explanation. No dispatch. |
| `detail_preview` | Open detail/preview/why-unavailable inspection without dispatch. |

Holding a modifier previews an alternate action, but it never widens command
authority and never skips preview, approval, trust, policy, or permission
revalidation.

## Lightweight History

Palette history is deliberately small. A `palette_query_history_entry_record`
may record that a query was committed, a command was invoked, a result was
selected, a query was repeated, or history was cleared. It does not become a
transcript.

Each entry records:

- the source session ref;
- query material posture: raw-local-only, redacted, hashed, command-ref-only,
  classification-only, or not recorded;
- selected command ref, if any;
- provider scope classes;
- ranking state ref;
- held modifier intent and footer action class;
- local-first retention policy;
- privacy-safe export behavior;
- created/last-used timestamps and use count.

Recent-query sets are bounded by `max_entries` and `max_age_days`. They carry
an ordering class and the same history policy, local-first retention, export
behavior, and clear-history rules as the entries they expose.

## Clear-History Rules

Clear-history actions are typed:

| Rule | Effect |
| --- | --- |
| `clear_current_session_only` | Clears volatile current-session material. |
| `clear_palette_recent_queries` | Clears palette recent query entries. |
| `clear_command_recents` | Clears command-specific recent invocations used by the palette. |
| `clear_profile_command_history` | Clears profile-local command history admitted by policy. |
| `admin_policy_purge` | Clears managed/governed history according to the policy epoch. |
| `erase_on_workspace_untrust` | Clears history when workspace trust is removed. |

Clear-history effects distinguish raw text deletion, hash deletion, command
recent deletion, aggregate tombstoning, and audit-minimum preservation. The
palette must never route a generic cache clear into local history deletion
without the user or policy selecting the relevant history class.

## Footer And Alternate Invocation Fields

The session links to one selected-row action footer. The footer schema owns the
full action records, while the session pins which actions are active for the
current query and selected row:

- `default_run_action`;
- `alternate_invocation_actions` for split/open-alt and alternate targets;
- `copy_actions` for canonical command ID and CLI/headless forms;
- `add_to_recipe_action`;
- `inspect_why_not_automatable_action`.

The action footer must preserve one action vocabulary across keyboard help,
menus, docs/help, CLI help, automation recipes, and support export. A footer
action that copies or inspects never dispatches. A footer action that changes
placement or target does not widen command authority and still revalidates
trust, policy, permission prompts, preview, and approval.

## Transition Rules

The session transition table is part of the record. The following
preservation constants are required on every transition:

- current text is preserved;
- held modifier intent is preserved;
- history policy is preserved;
- redaction posture is preserved.

This is especially important when a semantic provider streams after recent or
lexical results. A valid transition can append a semantic group, replace a
provider group, or rerank unselected rows, but it cannot silently drop the
held modifier or relax privacy/export posture.

Conforming examples:

| Transition | Allowed behavior |
| --- | --- |
| `filtering_lexical` -> `lexical_recent_ready` | Recent and lexical rows appear; first row may be selected if none was selected. |
| `lexical_recent_ready` -> `semantic_streaming` | Semantic rows append or replace their provider group while selected row and held modifier stay stable. |
| `semantic_streaming` -> `results_ready` | Provider states become complete; ranking freezes if the user committed. |
| `results_ready` -> `committed` | Dispatch occurs through command invocation contract; copy/recipe/inspect actions remain no-dispatch. |
| Any active state -> `cleared_history` | Clear-history effects apply; redaction posture remains explicit in any support capture. |

## Export And Support Rules

Support and export packets must:

- cite `palette_session_id`, `history_entry_id`, `recent_query_set_id`, result
  row refs, action footer refs, and command refs rather than copying raw
  query text;
- include bounded retention values and clear-history rules;
- include no raw query text unless an explicit reviewed policy allows it;
- preserve support export class, redaction posture, and secret-scan state;
- preserve that captured rows are a session snapshot, not necessarily current
  live command truth; and
- fail closed when redaction or retention cannot be proven.

## Fixture Acceptance

A fixture is conforming when a reviewer can determine:

- which single session owns current text, providers, ranking state, modifier
  intent, history policy, retention, and redaction posture;
- how recent and lexical results can appear before semantic results without
  losing selected row or held modifier state;
- which history entries and recent-query sets are retained, for how long, and
  how they are cleared;
- what support export may include and what raw material stays local-only; and
- whether footer verbs mean the same thing as keyboard, menus, docs/help, CLI
  help, and automation surfaces.
