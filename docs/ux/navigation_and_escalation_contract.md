# Navigation hierarchy, escalation model, and progressive-disclosure contract

This document freezes one navigational model — **route hierarchy,
inline-first escalation ladder, and progressive-disclosure rules** —
that every shell, command, support, and onboarding surface inherits
instead of minting surface-local conventions.

It is the narrative companion to one machine-readable artifact:

- [`/artifacts/ux/navigation_hierarchy.yaml`](../../artifacts/ux/navigation_hierarchy.yaml)
  — canonical route rows, escalation-tier rows, progressive-disclosure
  rows, and worked-example rows for search, review, fixes, destructive
  actions, credentialed flows, and high-risk approvals.

If this document and the source UI/UX spec disagree, the spec wins
and this contract plus the companion YAML MUST update in the same
change. If this document and a downstream surface's private
navigation, escalation, or disclosure story disagree, this contract
wins and the surface is non-conforming.

This contract rides alongside — it does **not** re-mint — the
vocabularies already frozen in:

- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  — shell zones, slot families, adaptive classes, and identity-cue
  fallback rules. This contract names **how a route enters a zone**;
  the zone-and-density contract names **how a zone behaves and
  collapses** once entered.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — `responsive_fallback_mode`, `required_visible_field_class`,
  `focus_return_state`, the `chrome_hid_required_field` denial,
  authority class, consequence class, preview / apply / revert
  posture, representation-labeled copy / export, and the protected-
  surface inventory. Escalation tiers in this contract bind to those
  classes and never re-mint them.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  — command identity, `ui_slot_class`, `palette_visibility_class`,
  `preview_class`, `approval_posture_class`, `disabled_reason_code`,
  `issuing_surface`, and the invocation-session packet shape every
  route resolves through.
- [`/docs/commands/command_graph_and_ui_slots_seed.md`](../commands/command_graph_and_ui_slots_seed.md)
  — slot families and stable slot keys, direct-projection vs
  mirror / handoff projection, and the descriptor-to-slot mapping
  rules. This contract cites those slot keys verbatim and never
  invents a rail-, sidebar-, or status-local route alias.
- [`/artifacts/commands/command_registry_seed.yaml`](../../artifacts/commands/command_registry_seed.yaml)
  and
  [`/fixtures/commands/seed_commands/`](../../fixtures/commands/seed_commands/)
  — the canonical command registry. Every navigation example and
  worked escalation row resolves a `command_id` from this registry
  rather than minting surface-local verbs.
- [`/docs/ux/keybinding_resolver_contract.md`](./keybinding_resolver_contract.md)
  — keybinding-precedence, conflict-review, disabled-command, and
  leader-overlay packets. Keyboard routes in this contract resolve
  through that resolver; this document does not re-mint precedence
  layers.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  — interruption tier, quiet-hours posture, and durable-attention
  semantics. Inline-first / panel-second / modal-last interacts with
  but never replaces the activity / attention vocabulary.
- [`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md)
  — shell-zone ids, focus ownership, and command-entry routing.

## Who reads this document

- **Shell, palette, and command surface authors** deciding whether a
  new action belongs inline next to its target, in a panel attached
  to the active surface, or in a modal / sheet, and which route or
  routes should advertise it.
- **Review, AI, install / attach, collaboration, and support-export
  surface authors** deciding when a flow may stay inline, when it
  must escalate to a panel or sheet, and when it must escalate to a
  modal with consequence block, preview, or approval.
- **Onboarding, docs / help, and migration surface authors** deciding
  which navigation routes a learning hint may quote, when an
  affordance must hand off to a canonical route, and how progressive
  disclosure can teach without front-loading every detail.
- **Support and parity-audit tooling** reading the route rows,
  escalation rows, disclosure rows, and worked-example rows
  mechanically — every row resolves to ids in the frozen
  vocabularies above.

## 1. Scope

- Freeze one canonical **navigation hierarchy** spanning command
  palette, quick open, sidebar zones, breadcrumbs, tabs / editor
  groups, and status-bar or inspector links — naming each route's
  scope, primary job, command-graph attach point, and what it must
  not be the only path for.
- Freeze the **inline-first, panel-second, modal-last escalation
  ladder** with named tiers, the consequence / preview / approval
  classes that justify each tier, and worked examples for search,
  review, fixes, destructive actions, credentialed flows, and high-
  risk approvals.
- Freeze **progressive-disclosure** rules and command-backed
  inspection paths so complex surfaces can reveal metadata,
  evidence, or logs without front-loading every detail; freeze the
  closed set of disclosure depths (`summary`, `detail`, `evidence`,
  `inspection`) and the rule that every depth resolves to an
  inspectable command rather than a surface-local mode.
- Freeze the **shared command-graph attach rules** every route
  inherits: a route is a different view into one command graph, not
  a parallel registry; a route may narrow visibility but may never
  silently widen authority, scope, or approval posture.

## 2. Out of scope

- Final menu structure, palette router implementation, leader-
  overlay copy, and onboarding tour scripting.
- Per-OS menu, jump-list, dock, taskbar, share-sheet, or system-
  notification routing rules. Those live in the platform-adapter and
  attention-activity contracts.
- The eventual command-router, palette, and onboarding crates' Rust
  types. This contract freezes the vocabulary and boundary every
  route renderer must plug into.
- Surface-specific copy. This contract names the route, the
  escalation tier, and the disclosure depth; product writers own the
  user-facing strings within those rules.

## 3. Frozen vocabulary (re-exported)

This contract mints no new shell-zone ids, slot families, adaptive
classes, issuing surfaces, preview classes, approval postures,
authority classes, consequence classes, responsive-fallback modes,
required-visible-field classes, or attention tiers. Every row
resolves to ids in the frozen vocabularies above. New rows are
additive-minor; repurposing an existing id is breaking and requires
a new decision row in `artifacts/governance/decision_index.yaml`.

This contract introduces three small navigation-local vocabularies.
They are scoped to navigation routes, escalation tiers, and
progressive-disclosure depth and never substitute for any frozen
upstream vocabulary.

- **`navigation_route_id`** — one of `route.command_palette`,
  `route.quick_open`, `route.activity_rail`, `route.sidebar`,
  `route.breadcrumbs`, `route.editor_tabs`, `route.editor_group`,
  `route.status_bar_link`, `route.inspector_link`,
  `route.global_menu`, `route.context_menu`, `route.notification_or_banner`,
  `route.deep_link_or_protocol`, `route.cli_invocation`,
  `route.ai_tool_invocation`. The route id names *which view* into
  the command graph; the slot key (from
  `command_graph_and_ui_slots_seed.md`) names *which slot* within
  that route.
- **`escalation_tier`** — one of
  `tier.inline_in_target_surface`, `tier.contextual_inline_overlay`,
  `tier.panel_attached_to_surface`, `tier.sheet_attached_to_window`,
  `tier.dialog_modal`, `tier.full_surface_takeover`. Tiers ascend
  monotonically; a flow MUST advance one tier at a time and MUST cite
  the consequence / preview / approval class that justifies any
  advance past `tier.contextual_inline_overlay`.
- **`disclosure_depth`** — one of `depth.summary`, `depth.detail`,
  `depth.evidence`, `depth.inspection`. Depth rises only on user
  request and MUST resolve to an inspectable command path; a depth
  that exists only as a surface-local mode with no command-graph
  attach is non-conforming.

## 4. Truthfulness posture (normative)

Every rule below is normative. A new shell, command, support, or
onboarding surface that violates any of them is non-conforming
regardless of how the violation is painted.

1. **One command graph, many routes.** Every route resolves to a
   `command_id` from the frozen registry. A route MAY narrow
   visibility (palette filter, context predicate, scope, lifecycle,
   client scope) but MUST NOT widen authority, scope, approval
   posture, or capability class beyond the descriptor. A surface
   that mints a launch-bearing affordance with no `command_id`
   attach is non-conforming.
2. **At least two routes for every frequent destination.** Every
   destination that users reach more than rarely MUST be reachable
   from at least two independent routes, **one of which is keyboard
   first** (palette, quick open, keybinding, or menu key). A
   destination reachable only by hover, drag, or pointer is non-
   conforming.
3. **Routes narrow from global to local scope.** Command palette is
   universal; quick open is known-target lookup; sidebar is durable
   structural exploration; breadcrumbs are local hierarchy; tabs and
   editor groups are the active working set; status-bar and
   inspector links are operational or contextual state. A route MUST
   NOT be the **only** path for a destination outside its declared
   scope.
4. **Inline first, panel second, modal last.** A flow starts inline
   with the target. It MAY escalate to a panel (sidebar tab, bottom-
   panel tab, right-inspector pane) or sheet (window-attached
   detail) when the user needs sustained scan space without losing
   workspace context. It MAY escalate to a modal dialog or full-
   surface takeover only when at least one of the criteria in §6
   holds. Skipping a tier is non-conforming.
5. **Escalation cites the class that justified it.** A flow that
   reaches `tier.dialog_modal`, `tier.sheet_attached_to_window`, or
   `tier.full_surface_takeover` MUST attach the
   `consequence_class_label`, `preview_class`, `approval_posture_class`,
   or attention-tier value that justified the escalation. An
   unattributed modal or sheet is non-conforming.
6. **De-escalation is explicit.** A flow that advanced to a panel,
   sheet, modal, or takeover returns focus through the escalation
   ladder it climbed (per the focus-return rules in
   `shell_interaction_safety_contract.md`). A close that lands in
   a random shell location is non-conforming.
7. **Progressive disclosure narrows what is visible, never what is
   true.** Hidden detail MUST be reachable through an inspectable
   command (`copy command id`, `open inspector`, `open evidence`,
   `view raw`, `open diagnostics`, `open route source`) and MUST NOT
   change a flow's consequence, preview, or approval posture. A
   summary that hides a destructive consequence is non-conforming.
8. **Routes that change because of policy, lifecycle, or trust
   explain the change.** A route that disappears because of policy
   block, capability lifecycle, client scope, freshness floor, or
   trust posture MUST surface the typed `disabled_reason_code` and
   repair hook from the command registry. Silent disappearance is
   non-conforming.
9. **Onboarding, docs / help, migration, and companion surfaces are
   handoff routes.** They MAY quote a `command_id`, label, docs
   anchor, and current shortcut and they MAY deep-link to a
   canonical route, but they MUST NOT mint a new direct execution
   path. A learning hint that runs a command without entering a
   canonical route is non-conforming.

## 5. Canonical navigation hierarchy

This is the frozen route catalog. Every row names:

- **Route id** — stable navigation route id (§3).
- **Scope class** — global, lookup, structural, local, working-set,
  contextual, mirror, or external.
- **Primary job** — what the route exists to do.
- **Attach to** — the slot family / slot keys
  (`command_graph_and_ui_slots_seed.md`) the route projects through,
  and the typical issuing surface from the command-descriptor
  contract.
- **Keyboard first?** — whether the route is keyboard first by
  default.
- **Must not be the only path for** — destinations or actions that
  require a second route per §4 rule 3.
- **Default escalation tier** — where a flow on this route starts.
- **May escalate to** — admissible higher tiers, with the rules in
  §6 governing when.

| Route id | Scope class | Primary job | Attach to | Keyboard first? | Must not be the only path for | Default escalation tier | May escalate to |
|---|---|---|---|---|---|---|---|
| `route.command_palette` | global | universal known-or-unknown action entry | `command_palette.result_row`, `command_palette.inline_action`; `issuing_surface = command_palette` | yes | persistent browsing, dense structural exploration, multi-step authoring | `tier.contextual_inline_overlay` | `tier.panel_attached_to_surface`, `tier.sheet_attached_to_window`, `tier.dialog_modal` |
| `route.quick_open` | lookup | fastest path to known files, symbols, recent places, settings, and commands | `command_palette.result_row` (palette-hosted) with quick-open mode predicate; `issuing_surface = command_palette` | yes | settings explanation, batch review, authority-sensitive setup | `tier.contextual_inline_overlay` | `tier.panel_attached_to_surface` |
| `route.activity_rail` | global | durable mode switching across top-level sections | `rail.primary_route_entry`, `rail.handoff_entry`; mirror / handoff family — focuses an existing slot or seeds palette | yes | direct command invocation; minting a new command path | `tier.inline_in_target_surface` | `tier.panel_attached_to_surface` (focuses sidebar / panel) |
| `route.sidebar` | structural | durable exploration of trees, lists, queues, and structural views | `sidebar.section_action`, `sidebar.selection_action`; `issuing_surface = toolbar_button` for actions | yes (focus + arrow keys) | one-off destructive or credentialed actions; primary palette / search entry | `tier.inline_in_target_surface` | `tier.contextual_inline_overlay`, `tier.panel_attached_to_surface`, `tier.sheet_attached_to_window` |
| `route.breadcrumbs` | local | local file / folder / symbol ancestry close to the editor | `editor_chrome.breadcrumb_action`; `issuing_surface = toolbar_button` | yes (focus + activate) | broad workspace search; structural exploration outside the active editor | `tier.inline_in_target_surface` | `tier.contextual_inline_overlay` (overflow / segment menu) |
| `route.editor_tabs` | working-set | rapid switching among already-open editors / diffs / notebooks | per-editor-group tab strip in `main_workspace`; `issuing_surface = toolbar_button` | yes | discovery of unopened resources; primary command invocation | `tier.inline_in_target_surface` | `tier.contextual_inline_overlay` (tab overflow / tab context menu) |
| `route.editor_group` | working-set | split, compare, and diff layout within `main_workspace` | `main_workspace` (split + tab routing) | yes (group nav shortcuts) | minting a non-editor surface family; replacing sidebar / panel routes | `tier.inline_in_target_surface` | `tier.contextual_inline_overlay`, `tier.panel_attached_to_surface` |
| `route.status_bar_link` | contextual | jump from a persistent state cue to the surface that owns it | `status_bar.primary_status_item`, `status_bar.quick_action`; `issuing_surface = toolbar_button` | yes (status nav shortcut) | hidden primary navigation; multi-step authoring; modal escalation without a panel intermediate | `tier.inline_in_target_surface` | `tier.contextual_inline_overlay`, `tier.panel_attached_to_surface`, `tier.sheet_attached_to_window` |
| `route.inspector_link` | contextual | open contextual explanation, evidence, or detail attached to the active workspace target | `inspector.header_action`, `inspector.footer_action`; `issuing_surface = toolbar_button` | yes (focus + activate) | dominant task surface ownership; primary navigation route | `tier.inline_in_target_surface` | `tier.contextual_inline_overlay`, `tier.panel_attached_to_surface`, `tier.sheet_attached_to_window` |
| `route.global_menu` | global | menubar / global-application menu entry for canonical commands | `global_menu.command_item`; `issuing_surface = global_application_menu` | yes (menu key + accelerators) | hidden-only entry for high-frequency core commands; replacing palette as the only entry | `tier.contextual_inline_overlay` | `tier.dialog_modal`, `tier.sheet_attached_to_window`, `tier.full_surface_takeover` |
| `route.context_menu` | local | right-click / `Shift+F10` actions scoped to the current target | `context_menu.editor_item`, `context_menu.explorer_item`, `context_menu.review_item`, `context_menu.source_control_item`, `context_menu.terminal_item`, `context_menu.search_item`; `issuing_surface = context_menu` | yes (`Shift+F10` / platform menu key) | global authority-sensitive flows; the only visible path for a destructive action | `tier.contextual_inline_overlay` | `tier.panel_attached_to_surface`, `tier.sheet_attached_to_window`, `tier.dialog_modal` |
| `route.notification_or_banner` | mirror | acknowledgement and escalation cue for activity / attention items | activity-event envelope rows; quotes commands minted elsewhere; mirror only — does not own a new `issuing_surface` | yes (activity center keyboard route) | primary entry for any command (per attention taxonomy); replacing inspectable activity rows | `tier.inline_in_target_surface` | `tier.panel_attached_to_surface`, `tier.sheet_attached_to_window` |
| `route.deep_link_or_protocol` | external | OS / browser / collaborator-initiated entry into a known target | mirror / handoff onto an existing route after trust + scope review; never widens authority | n/a | minting a fresh execution path; bypassing trust, profile, or scope review | `tier.contextual_inline_overlay` (interstitial review) | `tier.dialog_modal`, `tier.sheet_attached_to_window`, `tier.full_surface_takeover` |
| `route.cli_invocation` | external | command-line / headless entry resolving the same command graph | `cli_help.command_row`; `issuing_surface = cli_surface` | yes | implicit modal escalation (CLI prompts ride the same approval / preview rules as UI flows) | `tier.inline_in_target_surface` | `tier.dialog_modal` (interactive prompt) |
| `route.ai_tool_invocation` | external | AI-callable command entry under the descriptor's AI-surfacing class | `ai_tool_surface.tool_entry`; `issuing_surface = ai_tool_surface` | n/a | irreversible / high-blast paths denied at descriptor level; replacing user-initiated review | `tier.contextual_inline_overlay` (preview / approval card) | `tier.sheet_attached_to_window`, `tier.dialog_modal` |

Rules (frozen):

1. **Routes are different views into one command graph.** A route
   row MUST resolve every projected affordance through a
   `command_id`, an admissible `ui_slot_class`, and an existing
   `issuing_surface`. Adding a new route id is additive-minor; the
   row MUST also declare its slot family, attach point, and
   escalation tier.
2. **Two-route minimum is route-class aware.** The two independent
   routes for a frequent destination MUST come from different scope
   classes — for example, `route.command_palette` (global) plus
   `route.context_menu` (local), or `route.global_menu` (global)
   plus `route.activity_rail` (mirror that focuses a structural
   route). Listing the same route twice satisfies discoverability
   only if the second listing is keyboard first.
3. **Route narrowing is policy-aware, not opinion-aware.** A route
   MAY hide a command because of `palette_visibility_class`,
   `disabled_reason_code`, lifecycle state, client scope, or policy
   block. Hiding for product or surface preference is non-conforming
   when the descriptor admits the slot.
4. **Mirror / handoff routes never widen authority.** Activity rail,
   notifications, deep links, onboarding tips, and companion handoff
   surfaces MAY focus or seed a canonical route, but the actual
   `invocation_session_packet_record` MUST resolve through the
   canonical `issuing_surface`. Activity rail seeding palette is
   conforming; activity rail dispatching a destructive action
   directly is not.
5. **Status-bar and inspector routes resolve to the surface that
   owns the state.** A status item or inspector link MUST jump to
   the narrowest useful surface that can explain or repair the
   state. A status link that opens a generic settings page when a
   per-state inspector exists is non-conforming.

## 6. Inline-first, panel-second, modal-last escalation model

A flow advances through escalation tiers monotonically. The default
tier for every route is named in §5. A flow MAY advance one tier
when at least one criterion in the table below holds. A flow MUST
NOT skip a tier; if a flow cannot stay at its current tier and the
next tier cannot host it, the surface MUST deny rather than jump
two tiers silently.

| Tier | Where it renders | Admissible criteria for entry | Required attribution |
|---|---|---|---|
| `tier.inline_in_target_surface` | inside the active row, cell, breadcrumb, tab, status item, or editor selection | low-stakes default action that does not need scan space, preview, approval, or shared focus | none beyond standard component contract |
| `tier.contextual_inline_overlay` | popover, hovercard with keyboard equivalent, palette result preview, breadcrumb / tab overflow menu, segment context menu | the user needs richer choice or supporting metadata that does not justify a panel; rendering preserves workspace context and does not steal global focus | resolve to a `command_id`; record `focus_return_state` on dismiss |
| `tier.panel_attached_to_surface` | sidebar tab, bottom-panel tab, right-inspector pane, or activity-center entry | the flow needs sustained scan space, multi-row review, longitudinal state, or concurrent inspection of evidence; does not gate the rest of the workspace | quote slot family + slot key from `command_graph_and_ui_slots_seed.md`; record `focus_return_state` on dismiss |
| `tier.sheet_attached_to_window` | window-attached sheet that inerts the invoking pane but not the workspace | the flow benefits from keeping workspace identity visible while presenting batch / structured / preview content; user can still see the target underneath | cite `preview_class` and / or `responsive_fallback_mode = narrow_width_sheet`; preserve owning-window dialog rules from `shell_zone_and_density_contract.md` §10 |
| `tier.dialog_modal` | window-modal dialog that inerts the workspace until dismissed | a binary or short structured decision gates the workflow; user authority, trust posture, credential, or destructive scope MUST be reviewed before continuing | cite `consequence_class_label`, `preview_class`, and `approval_posture_class`; satisfy the `required_visible_field_class` set at commit per `shell_interaction_safety_contract.md` |
| `tier.full_surface_takeover` | dedicated surface in `main_workspace` (review, diff, evidence, recovery, governance) | the flow needs more room than a sheet for diff, evidence, recovery, or governance review; preserves origin breadcrumb and back-to-source path | cite the originating `command_id`, `preview_class`, and `result_contract_class`; surface MUST expose a back-to-source route |

Rules (frozen):

1. **Default to inline.** A new affordance starts at
   `tier.inline_in_target_surface`. A surface MAY default to
   `tier.contextual_inline_overlay` only when the action requires
   multi-row choice, scope clarification, or supporting metadata
   that cannot fit on the row.
2. **Modal requires at least one of the following to be true.**
   (a) user authority, trust posture, credential reveal, or step-up
   authentication MUST be obtained before proceeding;
   (b) the action crosses a security, identity, remote-mutation,
   publish, deletion, or policy-authoring boundary
   (`capability_scope_class` ∈ {`externally_visible_mutation`,
   `irreversible_high_blast_mutation`,
   `credential_or_secret_bearing`,
   `managed_workspace_control`, `policy_authoring_or_waiver`});
   (c) the user cannot safely continue the workflow without an
   explicit decision and no truthful inline path preserves context
   and recovery;
   (d) the descriptor's `preview_class` or `approval_posture_class`
   denies a non-modal apply.
   A modal that satisfies none of (a)–(d) is non-conforming.
3. **Sheet is the workspace-preserving step before modal.** Use
   `tier.sheet_attached_to_window` when the user benefits from
   seeing the target underneath. A flow that jumps from inline /
   overlay straight to modal MUST justify why a sheet was not
   sufficient.
4. **Full-surface takeover requires diff, evidence, recovery, or
   governance scope.** Use `tier.full_surface_takeover` for
   structured diff / merge, evidence packets, recovery (Project
   Doctor, safe mode, restore from checkpoint), and governance
   review (policy diff, ownership, release center). A takeover
   without one of those scopes is non-conforming.
5. **Escalation is monotonic.** A flow MAY return to a lower tier
   only by completing or cancelling the higher tier. A flow MUST NOT
   bounce between tiers as the user types or hovers.
6. **Every tier above inline returns focus.** Dismiss MUST emit a
   `focus_return_state` per `shell_interaction_safety_contract.md`;
   focus returns to the invoking control or the narrowest still-valid
   working surface.

### 6.1 Escalation worked examples

The full machine-readable example rows live in
[`/artifacts/ux/navigation_hierarchy.yaml#example_rows`](../../artifacts/ux/navigation_hierarchy.yaml).
The summary below is the human view; rows MUST cite the same
`command_id`s and `preview_class` / `approval_posture_class` /
`consequence_class_label` values.

- **Search (`route.command_palette` + `route.quick_open`).**
  Default `tier.inline_in_target_surface` (typing in the palette /
  quick-open input). Result rows render at
  `tier.contextual_inline_overlay`. Optional preview pane stays
  inside the overlay. Open destination drops back to
  `tier.inline_in_target_surface`. No modal unless a destination
  needs trust / preview / approval — in which case the destination's
  own escalation rules apply, not the search route's.
- **Review of an inline diagnostic or quick-fix
  (`route.inspector_link` / `route.editor_tabs`).** Default
  `tier.inline_in_target_surface` (squiggle, gutter action). Quick-
  fix list renders at `tier.contextual_inline_overlay`. Multi-file
  apply or batch fix escalates to `tier.panel_attached_to_surface`
  (review panel) and, for `preview_class ∈ {`structured_diff_preview`,
  `batch_scope_preview`, `broad_workspace_scope_preview`}`, to
  `tier.sheet_attached_to_window` for the apply preview. Modal only
  if `approval_posture_class = explicit_confirmation_required` and
  the apply cannot be staged inside the sheet.
- **Apply a fix or rename (`route.context_menu` /
  `route.command_palette`).** Inline single-file refactor stays at
  `tier.contextual_inline_overlay` (rename inline). Multi-file
  refactor escalates to `tier.panel_attached_to_surface` (rename
  preview panel) and `tier.sheet_attached_to_window` for the
  preview / apply / revert review. `tier.dialog_modal` reserved for
  the explicit-confirmation step on `recoverable_durable_mutation`
  with non-trivial scope.
- **Destructive action (`route.context_menu`,
  `route.global_menu`).** Inline destructive single-row action
  (`Delete file`) escalates to `tier.dialog_modal` only when the
  consequence class requires it (`recoverable_durable_mutation` with
  bulk scope, `externally_visible_mutation`,
  `irreversible_high_blast_mutation`). Bulk destructive review
  enters `tier.sheet_attached_to_window` (batch scope review per
  `interaction_safety` `batch_scope_review`) before the modal
  confirm. The modal MUST cite the `consequence_class_label`,
  `preview_class`, and `approval_posture_class`.
- **Credentialed flow (`route.command_palette`,
  `route.status_bar_link`, `route.deep_link_or_protocol`).**
  Default `tier.contextual_inline_overlay` (credential picker,
  account chip). Reveal / step-up enters
  `tier.sheet_attached_to_window` so the workspace remains visible.
  `tier.dialog_modal` is reserved for `step_up_authentication_required`
  and `credential_or_secret_access_preview`. Browser handoff rides
  `route.deep_link_or_protocol` and the `browser_handoff_packet`
  per ADR 0010; raw URL launches are forbidden.
- **High-risk approval (`route.command_palette`,
  `route.global_menu`, `route.notification_or_banner`).**
  Default `tier.panel_attached_to_surface` (review packet, evidence
  panel). Apply preview escalates to
  `tier.sheet_attached_to_window` (preview / apply / revert with
  representation labels). `tier.dialog_modal` is required when
  `approval_posture_class ∈ {`admin_policy_approval_required`,
  `second_party_review_required`, `managed_only_approval_required`}`.
  `tier.full_surface_takeover` is admissible when the review
  surface is a structured diff, evidence packet, or governance
  review that exceeds sheet capacity.

## 7. Progressive-disclosure rules and command-backed inspection paths

Progressive disclosure narrows what is **visible** so users can scan
quickly; it never narrows what is **true** about consequence,
preview, or approval posture. Every depth resolves to an inspectable
command path so a reviewer or support engineer can pivot from
summary to evidence without changing surfaces.

| Disclosure depth | What it shows | Required command-backed inspection path |
|---|---|---|
| `depth.summary` | one-line claim, primary identity, severity / freshness chip, one or two primary actions | `cmd:command_palette.open` to find the canonical command; `Open details` action quoting the canonical `command_id` |
| `depth.detail` | structured fields, scope statement, blocked / hidden counts, target / actor identity, supporting badges | `Copy command id`, `Open inspector`, route to the command's `docs_help_anchor_ref`, route to the surface that owns the state |
| `depth.evidence` | diff, log excerpt, citation anchor, evidence packet, raw / rendered toggle, representation label | `Open evidence`, `Open citation`, `Copy raw`, `Copy rendered` per `shell_interaction_safety_contract.md` representation rules; routes never re-mint copy / export labels |
| `depth.inspection` | full mutation journal, invocation-session packet, route source, policy diff, ownership card, support handoff | `Open invocation session`, `Open route source`, `Open policy rule`, `Open support handoff`; every action MUST resolve to a registered command |

Rules (frozen):

1. **Depth rises only on user request.** A surface MAY default to
   `depth.summary` or `depth.detail`. Auto-revealing
   `depth.evidence` or `depth.inspection` because focus moved, a
   provider warmed, or a hover dwelled is non-conforming.
2. **Depth never changes consequence.** The consequence,
   preview, and approval posture visible at `depth.summary` MUST
   remain truthful at every higher depth. A summary that hides a
   destructive scope or a `policy_blocked` state is non-conforming
   even if the detail row carries it.
3. **Every depth resolves to a command.** A depth MUST expose at
   least one route into the command graph (palette, inspector link,
   `Open …` action) so the user can pivot to the canonical surface.
   A `depth.evidence` view that has no command-backed path to the
   underlying journal, packet, or citation is non-conforming.
4. **Disclosure within a modal preserves the consequence block.**
   Progressive disclosure inside a `tier.dialog_modal` is admissible
   only when the default state remains honest about side effects,
   prerequisites, and blocked reasons (per UI/UX spec §10
   "dialogs and sheets" rules). A collapsed warning beneath an
   expand affordance inside a modal is non-conforming.
5. **Depth is keyboard-complete.** Every disclosure depth MUST be
   reachable by keyboard. Hover-only or pointer-only depth controls
   are non-conforming.

## 8. Onboarding, docs / help, and migration handoff rules

Onboarding tips, in-context teaching, docs / help destinations,
migration bridges, and companion-surface affordances are
**handoff routes** under §5 rule 4. They MAY:

- quote a `command_id`, primary label, accessibility label, docs
  anchor, current shortcut, and disabled-state reason from the
  command registry;
- deep-link to `route.command_palette`, `route.global_menu`,
  `route.context_menu`, `route.sidebar`, `route.status_bar_link`,
  `route.inspector_link`, `route.cli_invocation`, or
  `route.deep_link_or_protocol`;
- present a short-lived migration diff for high-frequency commands
  per the keybinding-resolver contract.

They MUST NOT:

- mint a new `issuing_surface` value;
- run a command without entering a canonical route (palette,
  context menu, keybinding, CLI, AI tool);
- substitute their own labels, docs anchors, or shortcut narration
  for the values published by the command registry;
- bypass trust, preview, approval, or focus-return rules even when
  the action looks low-stakes.

A learning surface that violates these rules denies with the
`disabled_reason_code` from the command registry (most commonly
`issuing_surface_unresolved` or `command_version_unknown`).

## 9. Relationship to adjacent contracts

- **Shell-zone and density contract** is the authoritative source
  for shell-zone ids, slot families, adaptive classes, and identity-
  cue fallback. This contract names **how a route enters a zone**;
  the zone-and-density contract names **how a zone behaves and
  collapses** once entered.
- **Shell-interaction-safety contract** is the authoritative source
  for `responsive_fallback_mode`, `required_visible_field_class`,
  `focus_return_state`, the `chrome_hid_required_field` denial,
  `consequence_class_label`, and the preview / apply / revert
  envelope. This contract attaches escalation tiers to those classes
  but does not re-mint them.
- **Command-descriptor contract** is the authoritative source for
  `command_id`, `ui_slot_class`, `palette_visibility_class`,
  `preview_class`, `approval_posture_class`, `disabled_reason_code`,
  `issuing_surface`, and the invocation-session packet shape. Every
  navigation row resolves a command-id and slot-class through that
  contract.
- **Command-graph and UI-slot seed** is the authoritative source for
  slot families, stable slot keys, direct-projection vs mirror /
  handoff projection, and descriptor-to-slot mapping. This contract
  cites those slot keys verbatim and never invents a rail-, sidebar-,
  or status-local route alias.
- **Command registry seed** is the canonical source of `command_id`s
  every navigation row resolves through. Worked examples in the
  companion YAML quote registry-seeded commands rather than minting
  new ones.
- **Keybinding resolver contract** is the authoritative source for
  precedence, conflict-review, disabled-command, and leader-overlay
  packets. Keyboard routes in this contract resolve through that
  resolver; this contract does not re-mint precedence layers.
- **Attention-activity taxonomy** is the authoritative source for
  interruption tier, quiet-hours posture, and durable-attention
  semantics. `route.notification_or_banner` mirrors that taxonomy.

## 10. Schema-of-record posture

The eventual command-router, palette, and onboarding crates' Rust
types are the schema of record. The companion YAML at
[`/artifacts/ux/navigation_hierarchy.yaml`](../../artifacts/ux/navigation_hierarchy.yaml)
is the cross-tool boundary every non-owning surface reads. Adding a
new route id, escalation tier, disclosure depth, or worked example
is additive-minor and bumps `navigation_hierarchy_schema_version`.
Repurposing an existing value is breaking and requires a new
decision row in `artifacts/governance/decision_index.yaml`.

## 11. Non-goals at this milestone

Out of scope until a superseding decision row opens:

- the live command router, palette, quick-open, leader overlay, and
  onboarding tour engines;
- final palette / menu / status-bar visual specification (the UX
  design-system style guide owns those);
- per-OS menu, jump-list, dock, taskbar, share-sheet, or system-
  notification routing rules;
- the eventual command-router, palette, and onboarding crates' Rust
  types.

These lines move only by opening a new decision row, not by editing
this contract.

## 12. Reuse guarantee

This contract is reusable by shell, command, support, and
onboarding surfaces without redefining navigation, escalation, or
disclosure semantics. A new surface MUST:

1. cite at least one `navigation_route_id` from §5 and resolve every
   launch-bearing affordance through a `command_id` from the frozen
   registry;
2. declare its default `escalation_tier` and the highest tier it
   may reach, citing the consequence / preview / approval class for
   any tier above `tier.contextual_inline_overlay`;
3. declare which `disclosure_depth` it presents by default and the
   command-backed inspection path for every higher depth;
4. honor the focus-return rules from
   `shell_interaction_safety_contract.md` on every dismiss above
   `tier.inline_in_target_surface`;
5. refuse to widen a command into a route, slot, or issuing surface
   the published taxonomy does not already allow.

## 13. Source anchors

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §4.1 — inline-first /
  panel-second / modal-last principle.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.2 — shell rules
  including the inline-first / panel-second / modal-last default.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §7.1 — navigation
  hierarchy table.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §7.2 — one command
  graph rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §7.3 — command object
  model.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §7.4 — command palette
  rules and metadata contract.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §7.5 — quick open,
  search, and symbol discovery.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §7.6 — breadcrumbs and
  structural context.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §9.14 — attention
  routing and inline / status / modal escalation rule.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §10 (dialogs / sheets,
  banners, toasts, status-bar items, breadcrumbs, menus, command
  bars) — escalation surfaces and progressive-disclosure rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix W.4 —
  interruption escalation checklist.

## 14. Linked artifacts

- Canonical navigation-hierarchy artifact:
  `artifacts/ux/navigation_hierarchy.yaml`.
- Companion shell-zone metrics: `artifacts/ux/shell_metrics.yaml`.
- Command-graph and UI-slot seed:
  `docs/commands/command_graph_and_ui_slots_seed.md` and
  `schemas/commands/ui_slot_taxonomy.schema.json`.
- Command-descriptor contract and registry:
  `docs/commands/command_descriptor_contract.md`,
  `schemas/commands/command_descriptor.schema.json`,
  `schemas/commands/command_registry_entry.schema.json`,
  `artifacts/commands/command_registry_seed.yaml`.
- Shell interaction-safety contract:
  `docs/ux/shell_interaction_safety_contract.md`,
  `schemas/ux/interaction_safety.schema.json`,
  `fixtures/ux/interaction_safety_cases/`.
- Shell-zone, density, and window-restore contract:
  `docs/ux/shell_zone_and_density_contract.md`.
- Keybinding resolver contract:
  `docs/ux/keybinding_resolver_contract.md`,
  `schemas/commands/keybinding_resolver.schema.json`.
- Attention / activity taxonomy:
  `docs/ux/attention_activity_taxonomy.md`,
  `schemas/ux/activity_event_envelope.schema.json`.
- ADR 0016 shell-windowing-input-accessibility-boundary:
  `docs/adr/0016-shell-windowing-input-accessibility-boundary.md`.
