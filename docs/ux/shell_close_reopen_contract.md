# Shell Close, Reopen, Focus-Return, and Slot-Memory Contract

This document freezes the shell-level behavior for closing, hiding,
collapsing, replacing, and reopening shell surfaces. It covers the
activity rail, left sidebar, main workspace, right inspector, bottom
panel, status bar, transient overlays, collapsed panes, and placeholder
surfaces so later shell code can preserve layout intent without
pretending an unavailable capability is still usable.

The machine-readable schema lives at:

- [`/schemas/ux/shell_slot_memory.schema.json`](../../schemas/ux/shell_slot_memory.schema.json)

The worked fixtures live under:

- [`/fixtures/ux/shell_slot_cases/`](../../fixtures/ux/shell_slot_cases/)

This contract composes with and does not replace:

- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  for shell-zone identity, adaptive collapse, and restore phases.
- [`/docs/ux/rail_sidebar_contract.md`](./rail_sidebar_contract.md)
  for rail section ids, sidebar hide/show commands, and
  reopen-last-section behavior.
- [`/docs/ux/splitter_contract.md`](./splitter_contract.md)
  for resizable-pane collapse, collapse barriers, and proportional
  restore.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  for pane-tree identity, placeholder payloads, and no-rerun rules.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for focus-return vocabulary and consequence-bearing surface closure.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md)
  for dialog and sheet origin, dismissal, and durable handoff rules.

Where this contract disagrees with the source PRD, technical design,
UI/UX spec, shell-zone contract, rail/sidebar contract, splitter
contract, layout-serialization contract, or interaction-safety contract,
the source wins and this document plus its schema and fixtures update in
the same change. Where a downstream surface mints private close,
reopen, or focus-return rules that conflict with this document, this
contract wins and the surface is non-conforming.

## Who Reads This Contract

- Shell authors implementing hide/show, panel tabs, inspectors,
  placeholder cards, layout restore, and adaptive collapse.
- Feature owners deciding what state may be remembered when their
  surface closes.
- Extension authors contributing views that must reopen as honest
  placeholders when the extension is missing, disabled, quarantined, or
  untrusted.
- Accessibility, QA, parity, and support tooling that need inspectable
  focus-return and recovery routes.

## 1. Scope

This contract freezes:

- close, hide, collapse, dismiss, replace, and remove semantics for
  every shell zone and shell-hosted surface class;
- command-backed reopen paths for rail sections, sidebar surfaces,
  bottom-panel tabs, inspector panels, collapsed panes, and placeholder
  surfaces;
- remembered-last-surface rules, including the safe-restore gates and
  the conditions that force a truthful placeholder or fallback;
- focus-return rules after close, dismiss, apply, cancel, collapse,
  reopen, provider replacement, and placeholder recovery;
- the fixture shape used to test that close/reopen never strands focus
  or silently restores an untrusted or unavailable surface.

This contract does not implement the eventual Rust shell crate, command
descriptors, provider logic, animation, or OS window-manager behavior.
It freezes the behavioral boundary those implementations must satisfy.

## 2. Frozen Vocabulary

This document reuses these upstream values without redefining them:

- shell zones from the shell-zone contract:
  `title_context_bar`, `activity_rail`, `left_sidebar`,
  `main_workspace`, `right_inspector`, `bottom_panel`, `status_bar`,
  and `transient_overlay`;
- section ids and reopen-last-section terms from the rail/sidebar
  contract;
- collapse outcomes and barriers from the splitter contract;
- restore phases and dependency classes from layout serialization;
- focus-return target ordering from the interaction-safety contract;
- command ids, route ids, slot keys, disabled reasons, client scope,
  trust posture, and lifecycle vocabulary from the command and
  navigation contracts.

This contract adds a shell-slot-local vocabulary, frozen at schema
version 1. Adding a value is additive-minor and bumps
`shell_slot_memory_schema_version`; repurposing a value is breaking and
requires a governance decision row.

### 2.1 `shell_slot_kind`

- `title_context_bar` - required identity chrome; not user-closable.
- `activity_rail` - required top-level route map; not user-closable.
- `rail_section` - a rail entry and its owning sidebar section.
- `left_sidebar` - the sidebar zone and active section body.
- `sidebar_surface` - a concrete sidebar view, tree, collection, or
  status body.
- `main_workspace` - dominant task surface and editor group host.
- `right_inspector` - inspector zone.
- `inspector_panel` - a concrete contextual inspector occupant.
- `bottom_panel` - bottom-panel zone and tab strip.
- `bottom_panel_tab` - a terminal, problems, output, debug, test, log,
  or activity tab inside the bottom panel.
- `status_bar` - persistent state route; not user-closable.
- `transient_overlay` - palette, dialog, sheet, quick picker, or menu.
- `collapsed_pane` - a previously visible pane collapsed by user action
  or adaptive layout.
- `placeholder_surface` - a host-owned unavailable or missing-surface
  card preserving a slot.

### 2.2 `close_action_class`

- `not_closable_required_chrome`
- `hide_zone`
- `close_surface`
- `close_tab`
- `collapse_pane`
- `dismiss_overlay`
- `replace_with_placeholder`
- `remove_placeholder`
- `reset_to_default`

### 2.3 `reopen_strategy`

- `not_applicable_required_chrome`
- `exact_last_surface`
- `direct_surface`
- `same_slot_placeholder`
- `sheet_fallback`
- `first_admitted_primary`
- `main_workspace_default`
- `evidence_only_no_rerun`
- `clear_intent_then_default`

### 2.4 `memory_restore_posture`

- `restore_exact`
- `restore_with_revalidation`
- `restore_as_placeholder`
- `restore_as_evidence_only`
- `fallback_with_explanation`
- `discard_user_close_intent`
- `not_remembered`

### 2.5 `unsafe_restore_reason`

- `missing_extension`
- `extension_disabled_or_quarantined`
- `provider_unavailable`
- `policy_blocked`
- `trust_revoked_or_untrusted`
- `authority_expired_or_revoked`
- `remote_unavailable`
- `schema_unsupported`
- `target_missing_or_moved`
- `non_reentrant_live_surface`
- `stale_snapshot_for_mutation`
- `raw_secret_or_unreviewed_input`
- `client_scope_not_admitted`

## 3. Normative Invariants

1. **Close is not data loss by default.** Hiding a sidebar, closing an
   inspector, closing a bottom-panel tab, or collapsing a pane records
   layout intent and focus-return state. It does not delete workspace
   authority, dirty buffers, durable jobs, review evidence, or provider
   state.
2. **Required chrome stays reachable.** The title/context bar, activity
   rail, main workspace, status bar, and transient-overlay route system
   cannot be closed into absence. They may compress or overflow only
   through the shell-zone contract.
3. **Reopen is command-backed.** Every closeable or collapsible slot has
   at least one command-palette or menu route. A visible affordance may
   disappear under adaptive pressure only if the command route remains.
4. **Remembered state is conditional.** The shell may restore the last
   surface only after policy, trust, lifecycle, client scope,
   dependency, and authority checks still admit it.
5. **Unavailable means placeholder, not pretend-ready.** A missing,
   disabled, quarantined, untrusted, provider-unavailable, or
   unsupported surface reopens as an honest placeholder in the same slot
   when layout intent still matters.
6. **No live effects replay.** Reopening terminal, debug, notebook,
   preview, remote, or provider-backed live surfaces may restore title,
   provenance, tab order, transcript, and recovery actions. It must not
   rerun commands, replay input, silently reacquire authority, or widen
   trust.
7. **Focus never strands.** After close, dismiss, apply, replacement, or
   reopen, focus lands on the exact invoker when valid, then the
   narrowest still-valid owner, then an adjacent visible pane, then the
   main workspace. Focus never lands off-screen, on collapsed chrome, on
   a removed node, or on a placeholderless void.
8. **User close intent is respected.** A surface the user explicitly
   removed must not silently reappear on unrelated restore. Reopen
   commands may recover it; automatic restore may only preserve the
   intent and route to a default or placeholder with explanation.

## 4. Close And Reopen Matrix

| Slot kind | Close path | State recorded | Reopen path | Placeholder rule |
|---|---|---|---|---|
| `title_context_bar` | not user-closable | none beyond window topology | required chrome restored by shell skeleton | not applicable |
| `activity_rail` | not user-closable | active section and rail focus target | required route map restored by shell skeleton | rail entries may show disabled reasons but the rail itself stays |
| `rail_section` | hide active sidebar section or jump away | section id, active view ref, badge/count state, focus target | `cmd:shell.section.open.*` or `cmd:shell.sidebar.reopen_last` | if admitted but body unavailable, open a section placeholder |
| `left_sidebar` | `cmd:shell.sidebar.hide` or adaptive sheet dismiss | last section, view ref, width/sheet posture, collection state | `cmd:shell.sidebar.reopen_last`, direct section command, rail entry | placeholder same section when body unavailable; fallback by rank when not admitted |
| `sidebar_surface` | section-local close or view switch | restorable query/scope/filter/scroll refs, not raw payloads | direct section/subsection command or saved-view route | provider failures render a status/placeholder body in the sidebar |
| `main_workspace` | close tab, split, editor group, or dedicated surface | pane id, tab order, dirty/recovery state, selection and scroll when safe | reopen closed editor, route from history, or restore pane skeleton | missing dependency keeps pane slot with placeholder and reopen path |
| `right_inspector` | toggle, close, sheet dismiss, or adaptive collapse | target ref, inspector kind, dock/sheet posture, focus target | `cmd:shell.inspector.open_last`, target-owned inspector link | missing or invalid target opens a placeholder or returns to owner |
| `inspector_panel` | panel close, target replacement, or sheet dismiss | target ref, panel kind, disclosure state, source route | target-owned command, inspector toggle, or command palette route | missing extension/provider/trust becomes placeholder in the inspector slot |
| `bottom_panel` | toggle or adaptive collapse | active tab id, height/sheet posture, barrier summary | `cmd:shell.bottom_panel.open_last_tab` or direct tab command | durable barriers prevent silent collapse; unavailable tabs show placeholders |
| `bottom_panel_tab` | tab close or switch | tab id, provenance, transcript/artifact refs, selected row | direct tab command, task/status route, history row | live tabs reopen evidence-only unless safe live rebind is proven |
| `status_bar` | not user-closable | owning routes and overflow summary | required state route restored by shell skeleton | status items can point to placeholders but are not placeholders themselves |
| `transient_overlay` | `Esc`, close button, cancel, apply, or route completion | invoker, owner window, target object, return target, durable handoff | original command, history/job row, or object-owned route | if invoker vanished, return to ancestor or announced placeholder |
| `collapsed_pane` | collapse command, splitter action, or adaptive fallback | pane id, role, proportional intent, barrier summary, focus target | visible stub, zone toggle, or command route | no placeholderless collapse; unavailable dependency uses a placeholder |
| `placeholder_surface` | close placeholder or run recovery action | original role, dependency reason, recovery route, user close intent | recovery command, install/reconnect/reauth, or remove slot | never render the missing capability as live until dependency checks pass |

## 5. Remembered-Last-Surface Rules

A remembered surface may restore as `restore_exact` only when all of
these gates pass:

- the saved slot id, surface class, and owning command are still known
  to the current schema;
- the window/workspace authority ref still matches or has an explicit
  compatible migration;
- policy, trust, lifecycle, client scope, and profile still admit the
  surface;
- the dependency provider, extension, remote, or runtime is present and
  not quarantined, revoked, or expired;
- the remembered state contains only restorable refs, hashes, labels,
  and safe summaries, not raw credentials, raw provider payloads,
  unreviewed destructive intent, or unsent form secrets;
- restoring the surface does not rerun live effects, reacquire broader
  authority, widen network or provider scope, or imply stale data is
  current;
- the focus-return target or a narrower valid owner still exists.

If the surface is admitted but cannot hydrate, reopen the same slot as
`restore_as_placeholder` or `restore_as_evidence_only`. If the surface
is not admitted at all, use `fallback_with_explanation`: open the first
valid owning surface and disclose why the prior target was not restored.

## 6. Placeholder Requirements

Every placeholder surface records:

- original shell slot, pane id or section id, and surface class;
- unavailable reason from the controlled vocabulary;
- last-known provenance or producer ref when safe;
- whether the prior user action was close, collapse, restore, reopen,
  provider replacement, or migration;
- safe recovery routes, including at least one keyboard route;
- focus-return target and announcement posture;
- an explicit guardrail that the missing capability is not live.

Placeholder close removes only the placeholder occupant when the user
chooses that action. It does not delete durable evidence, dirty buffers,
support packets, history rows, or provider records owned elsewhere.

## 7. Focus-Return Rules

Every close, dismiss, apply, replacement, and reopen operation evaluates
focus in this order:

1. exact invoking control, if still visible, enabled, and in the same
   window;
2. invoking row, card, tab, tree item, or panel header;
3. current collection owner, sidebar section, inspector target, or
   bottom-panel tab list;
4. logically adjacent visible pane after split merge or collapse;
5. placeholder card occupying the same slot, when the original surface
   was replaced;
6. main workspace focus target;
7. command palette route only when no visible surface can safely own
   focus.

The shell must announce when it cannot return to the exact invoker and
why. It must never return focus to a hidden pane, collapsed panel body,
off-screen window, removed list row, stale modal body, disabled control,
or document body fallback.

## 8. Command-Backed Recovery Routes

The following command families are reserved by this contract. Concrete
descriptors must exist before any launch-bearing UI ships:

| Purpose | Command family |
|---|---|
| Hide/toggle/reopen sidebar | `cmd:shell.sidebar.hide`, `cmd:shell.sidebar.toggle`, `cmd:shell.sidebar.reopen_last` |
| Open a rail/sidebar section | `cmd:shell.section.open.*` |
| Open/toggle inspector | `cmd:shell.inspector.toggle`, `cmd:shell.inspector.open_last`, `cmd:shell.inspector.open_for_target` |
| Open/toggle bottom panel | `cmd:shell.bottom_panel.toggle`, `cmd:shell.bottom_panel.open_last_tab`, `cmd:shell.bottom_panel.open_tab.*` |
| Collapse/reopen pane | `cmd:shell.pane.collapse`, `cmd:shell.pane.reopen_collapsed` |
| Recover placeholder | `cmd:shell.placeholder.recover`, `cmd:shell.placeholder.close` |
| Reopen closed workspace surface | `cmd:shell.workspace_surface.reopen_closed` |
| Open restore details | `cmd:shell.restore.details` |

Visible routes may be rail entries, tab headers, split stubs, status
items, placeholder buttons, inspector links, or menu items. At least one
keyboard-first route must exist for every recoverable closed or
collapsed slot.

## 9. Fixture Coverage Contract

The fixture corpus must include:

- a catalog row that lists all shell slots and their close/reopen
  posture;
- a sidebar or rail case where remembered state restores safely;
- a sidebar, inspector, or extension-owned case where remembered state
  reopens as a placeholder;
- a bottom-panel case that proves live surfaces reopen as evidence-only
  or rebind only after validation;
- a collapsed-pane case with both visible and command-backed recovery;
- a focus-return case where the exact invoker vanished and focus moves
  to the narrowest safe owner.

The schema and fixtures are review artifacts, not runtime telemetry.
They must not contain raw file paths, raw query strings, raw provider
payloads, raw credentials, or product planning identifiers.

## 10. Reviewer Checklist

1. Does every closeable surface have a visible route or command-backed
   reopen path?
2. Does the remembered-state decision prove policy, trust, lifecycle,
   client scope, dependency, and authority checks before exact restore?
3. Does an unavailable or missing surface reopen as an honest
   placeholder in the same slot when layout intent matters?
4. Does every placeholder expose recovery and removal routes without
   pretending the original capability is live?
5. Does focus return to the exact invoker or narrowest still-valid owner
   and avoid off-screen, hidden, disabled, or removed targets?
6. Does the bottom-panel or live-surface path avoid automatic command
   rerun, input replay, or hidden authority reacquisition?
7. Does explicit user close/remove intent prevent silent reappearance on
   unrelated restore?

## 11. Source Anchors

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.1.1 — canonical desktop shell zones.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.1.11 — workspace-window, split-layout, and session-restore architecture.
- `.t2/docs/Aureline_Technical_Design_Document.md` Appendix AF — restore defaults and missing dependency posture.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix CF — remembered-state inspector and missing-surface placeholder templates.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix EL — focus-return matrix.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix EP — shell zone close/reopen paths and responsive fallback ladder.
