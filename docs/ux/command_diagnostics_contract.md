# Command-diagnostics view, disabled-reason projection, and protected-entry target/origin badge contract

This document freezes the command-diagnostics projection every
command palette, global application menu, context menu, toolbar
button, keybinding editor, keybinding help overlay, CLI help
output, AI-tool surface, automation-recipe response, docs
reference page, About / service-health panel, and dedicated
command-diagnostics panel reads before rendering one of:

- a typed disabled / hidden reason plus repair hook,
- a remediation hint with a named grant scope and a typed
  actionability class,
- a target / origin badge for a protected entry point (terminal,
  tasks, debug, remote attach, browser handoff, provider-bearing
  action, managed workspace control, credential-broker step-up,
  policy authoring / waiver),
- the current keybinding-help pivot (the narration a shortcut
  help or accessibility layer reads),
- the current docs / help pivot,
- the route / exposure context the command was evaluated under
  (issuing surface, UI slot class, palette visibility, context
  filter).

The machine-readable boundary is
[`/schemas/commands/diagnostic_projection.schema.json`](../../schemas/commands/diagnostic_projection.schema.json);
worked rows (local-only, policy-blocked, wrong-target, provider-
limited, stale-context, missing-capability, reapproval-required,
protected-entry terminal paste, protected-entry remote attach,
protected-entry browser handoff, and a parity set projecting one
descriptor into palette + menu + keybinding help + CLI help + AI
tool for the same invocation) live in
[`/fixtures/commands/diagnostic_rows/`](../../fixtures/commands/diagnostic_rows/).

Every diagnostic row extends a command descriptor frozen in
[`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
and an invocation-session packet frozen in the same schema. The
descriptor is the governed product object; an invocation-session
packet is the governed invocation envelope; this row is the
cross-surface projection those two records publish into every
surface that renders a "why is this unavailable" / "where would
this land" / "what would this touch" disclosure. If this document
and a neighbouring ADR disagree, the ADR wins and this document
MUST be updated in the same change.

## Why freeze this now

Without a single projection shape, every surface would mint its
own dialect for:

- "Why is this command unavailable?" Palette would say
  "Unavailable", menu would say "Disabled", keybinding help would
  say "No shortcut does this here", CLI help would say "Error:
  no applicable context", AI-tool surface would say
  `{"error":"not_available"}`, diagnostics panel would say
  "Restricted", and docs would say "Not on this plan" — none of
  them mechanically comparable.
- "Where would this land?" Terminal-paste, tasks, debug, remote
  attach, browser handoff, and provider-bearing actions would
  each mint their own "target" chip. One surface might show the
  raw hostname; another the raw account handle; another the raw
  project URL. Support export, redaction-safe docs screenshots,
  and accessibility narrations would drift.
- "Who is acting?" Each surface would decide whether a delegated
  user token, an installation grant, a service identity, an
  admin policy, an automation recipe, or a collaborator's remote
  session is the "origin" — and name it inconsistently.
- "What would fix it?" Palette might offer one remediation;
  keybinding help another; diagnostics panel a third. An admin-
  only remediation could be exposed as a user-actionable button
  on the wrong surface.
- "What does the keybinding / help pivot look like?" Keybinding
  help and the palette would each re-derive shortcut narration
  and docs anchors, leading to tooltip-only disclosure drift.

This contract is the missing piece that lets, at freeze time and
before any runtime:

- every surface render the same typed disabled / hidden reason
  plus repair hook for the same `command_id` /
  `command_revision_ref` + policy context + issuing surface,
- every protected entry render the same typed target / origin
  badge with the same export-safe labels, the same required
  trust-state floor, and the same grant-scope floor,
- every remediation render the same typed grant-scope requirement,
  the same actionability class, and the same closed next-step
  deep-link target kind,
- every keybinding-help / shortcut-help / docs-help pivot read
  the same opaque refs the descriptor already publishes, without
  minting a parallel help anchor or a parallel shortcut
  narration,
- every diagnostic row stay identical across shell, palette, and
  keybinding surfaces for the same (command_id, command_revision_ref,
  policy_epoch, trust_state, issuing_surface, ui_slot_class,
  execution_context_id) tuple — so parity audits are mechanical
  rather than human.

This document does not ship a diagnostics UI. It freezes the
row shape and the badge shape so a diagnostics panel, when it
lands, reads one projection rather than re-deriving disabled
reasons and target / origin identity from registry state.

## Scope

Frozen at this revision:

- One `command_diagnostic_row_record` shape projecting a command
  descriptor + invocation-session packet (or a would-be
  invocation-session packet for surfaces that render disabled
  commands without dispatching them) into a surface-renderable
  row with typed disabled reason, typed remediation,
  keybinding-help pivot, docs-help pivot, route / exposure
  context, authority class, capability scope class, preview
  requirement, approval posture, and policy source.
- One `protected_entry_badge_record` shape naming the protected-
  entry class (terminal, tasks, debug, remote attach, browser
  handoff, provider-bearing action, managed workspace control,
  credential-broker step-up, policy authoring / waiver), the
  opaque target identity and target kind class, the target trust
  class, the origin actor class, the origin authority class, the
  origin client scope, the target route class, the required
  trust-state floor, the required grant-scope floor, and opaque
  refs to the export-safe target / origin labels and the badge
  narration.
- One `remediation_projection_record` shape naming the typed
  disabled reason, the repair-hook ref, the required grant
  scope, the actionability class, the optional next-step deep-
  link target kind, a dismissible flag, and an export-safe hint
  label.
- Projection rules binding descriptor fields, invocation-session
  packet fields, policy-context fields, and invocation-issuing-
  surface fields to diagnostic-row fields so the row is
  mechanically producible from registry + packet data alone.
- Parity rules listing the surface classes that MUST render the
  same row for the same projection key.
- Protected-entry selection rules binding `capability_scope_class`,
  `preview_class`, and `issuing_surface` to a required
  `protected_entry_badge_ref`.
- Export-safe wording classes the row / badge declare so a
  support-bundle screenshot, an accessibility narration, or a
  docs-safe recording uses the same redaction posture.

Out of scope until a superseding decision row opens:

- The live diagnostics panel UI (the dedicated surface that reads
  many rows at once), its filtering / search / sort / drill-down,
  and its empty / busy / error states. This contract pins the
  per-row shape; the panel renders the rows.
- Runtime-generated parity-diff automation that diffs emitted
  rows across shell / palette / keybinding / CLI / AI-tool
  surfaces; the seed parity corpus lives under
  [`/docs/commands/command_parity_diff.md`](../commands/command_parity_diff.md).
- The repair / drill-down interaction bodies (approval requests,
  grant prompts, step-up authentication flows); this contract
  names the `repair_hook_ref` the body resolves against.
- Final docs-pack localisation of labels; labels are opaque refs
  resolved by the docs / localisation pipeline.

## Canonical ownership

Each command descriptor has exactly one canonical owner; the
registry that owns the descriptor (workspace, editor, source-
control, search, settings, AI, extension, support, managed-
workspace — see
[`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md))
also owns the diagnostic-row projection for that descriptor.
Surfaces that render a diagnostic row for a command they do not
own MUST quote the owner's `command_diagnostic_row_record`
rather than mint a copy.

A protected-entry badge has one canonical owner per
`protected_entry_class`:

| Protected-entry class                  | Canonical owner                           |
|----------------------------------------|-------------------------------------------|
| `terminal_entry`                       | `terminal_surface_registry`               |
| `tasks_entry`                          | `tasks_surface_registry`                  |
| `debug_entry`                          | `debug_surface_registry`                  |
| `remote_attach_entry`                  | `remote_agent_surface_registry`           |
| `browser_handoff_entry`                | `browser_handoff_surface_registry`        |
| `provider_bearing_action_entry`        | `provider_surface_registry`               |
| `managed_workspace_control_entry`      | `managed_workspace_command_registry`      |
| `credential_broker_step_up_entry`      | `credential_broker_surface_registry`      |
| `policy_authoring_or_waiver_entry`     | `settings_command_registry`               |

The registries listed above are projection targets; this document
does not land any of them. It pins the shape they MUST publish.

## Record fields

The full field set lives in
[`/schemas/commands/diagnostic_projection.schema.json`](../../schemas/commands/diagnostic_projection.schema.json).
The notable fields are:

### `command_diagnostic_row_record`

- **Projection key.** `diagnostic_row_id` is the stable id of
  this row; `command_id_ref` and `command_revision_ref` pin the
  descriptor the row projects from; `invocation_session_id_ref`
  pins the invocation-session packet the row projects from, or
  is null when the row is produced for a surface that renders
  disabled commands without dispatching them (the palette listing
  a disabled command, the keybinding editor surfacing an
  unavailable command). `rendering_surface_class` names the
  surface the row is being projected for; `issuing_surface_class`
  (re-exported from the command descriptor) names the surface
  that would mint the invocation-session packet if the user
  triggered the row.
- **Enablement and reason.** `enablement_decision_class` is
  re-exported from the command descriptor without modification
  (`enabled`, `disabled_with_reason`, `hidden_with_reason`).
  `disabled_reason_code` is re-exported from the command
  descriptor (the closed twenty-four-value set); null when
  `enablement_decision_class = enabled`. `disabled_reason_label_ref`
  is the opaque pin to the export-safe short label a surface
  renders next to the chip; `long_explanation_label_ref` pins
  the long-form narration a screen reader / accessibility layer
  reads. Labels are resolved by the docs / localisation pipeline;
  raw strings never appear here.
- **Remediation projection.** `remediation_projection_ref` is
  null when the reason is non-remediable (e.g. `command_retired`
  with no replacement); non-null otherwise, pointing at a
  `remediation_projection_record` that carries the typed
  repair-hook ref, the required grant scope, the actionability
  class, and optional next-step deep-link target kind.
- **Authority, scope, preview, approval.** `authority_class`,
  `capability_scope_class`, `preview_class`, and
  `approval_posture_class` are re-exported from the command
  descriptor without modification so every surface tells the same
  story about who is acting, what will change, whether a preview
  is required, and whether an approval is required.
- **Route and exposure context.** `route_and_exposure_context`
  pins the `ui_slot_class` and the `palette_visibility_class`
  the row is being rendered under, plus an optional
  `contextual_filter_class_ref` (re-exported from the descriptor's
  `ui_slot_hint`) and an optional `client_scope` (re-exported
  from the capability-lifecycle vocabulary). A row whose
  `client_scope` excludes the rendering surface MUST carry
  `disabled_reason_code = client_scope_excludes_surface`.
- **Keybinding-help pivot.** `keybinding_help_pivot` carries
  the `shortcut_narration_hint_ref` (re-exported opaque ref)
  the keybinding-help layer and the accessibility layer read,
  plus an optional `keybinding_resolver_row_ref` pinning the
  keybinding-resolver row (if any) whose precedence stack won
  for this command in this context. The row MUST NOT mint a
  parallel shortcut narration; minting a parallel narration is
  non-conforming.
- **Docs-help pivot.** `docs_help_pivot` carries the
  `docs_help_anchor_ref` (re-exported opaque ref) every palette
  tooltip, menu help, keybinding-help expansion, CLI `--help`
  output, AI-tool description, diagnostics panel, and docs
  reference page reads for this row. The row MUST NOT mint a
  parallel help anchor; minting a parallel anchor is
  non-conforming.
- **Policy source.** `policy_source` pins the `policy_epoch`,
  the `trust_state`, the optional `execution_context_id`, and
  the optional `active_policy_bundle_ref` the diagnostics
  evaluation resolved under. Support exports and parity audits
  read this block to decide whether two rows agreed under the
  same policy context.
- **Protected-entry badge.** `protected_entry_badge_ref` is
  non-null when the descriptor's `capability_scope_class` or
  `preview_class` or the issuing surface's protected-entry
  selection rule requires a badge (see the protected-entry
  selection rules below); null otherwise.
- **Parity set.** `parity_surface_set` lists the closed set of
  rendering-surface classes that MUST render the same row for
  the same projection key. A surface rendering this row MUST
  verify its own `rendering_surface_class` is in this set; if
  not, the row is non-conforming.
- **Export-safe wording class.** `export_safe_wording_class`
  is one of `safe_for_public_docs`, `safe_for_support_export`,
  `operator_only_restricted`, `managed_only_restricted`. A
  surface rendering the row in a public docs screenshot or a
  support bundle MUST narrow labels to the declared class; a
  `managed_only_restricted` row rendered in a public docs
  screenshot is non-conforming and MUST deny render.
- **Redaction class.** `redaction_class` is re-exported from
  ADR-0011 without modification and is re-validated before bytes
  reach any persistent or exportable sink.
- **Minted at and policy context.** Standard fields re-exported
  from the command descriptor.

### `protected_entry_badge_record`

- **Identity.** `badge_id` is the stable id; `command_id_ref`
  and `command_revision_ref` pin the descriptor the badge
  attaches to. A badge MAY be produced for a surface-level entry
  that is not a descriptor-level command (a terminal keybinding
  chord that enters a paste review state, a remote-attach entry
  point rendered in the rail, a browser-handoff entry point
  rendered in a provider tile); in that case `command_id_ref` is
  null and `surface_entry_id_ref` pins the surface entry.
- **Protected-entry class.** `protected_entry_class` is one of
  `terminal_entry`, `tasks_entry`, `debug_entry`,
  `remote_attach_entry`, `browser_handoff_entry`,
  `provider_bearing_action_entry`,
  `managed_workspace_control_entry`,
  `credential_broker_step_up_entry`,
  `policy_authoring_or_waiver_entry`.
- **Target.** `target_identity_ref` is the opaque pin to the
  target (local shell handle, remote agent session, debug
  adapter instance, browser handoff target, provider account,
  provider project, managed workspace tenant, secret-broker
  scope, policy bundle). Raw hostnames, raw URLs, raw account
  handles, raw credential material never appear here.
  `target_kind_class` is one of
  `local_device_shell`,
  `remote_agent_shell`,
  `containerized_process`,
  `task_runner_session`,
  `debug_adapter_session`,
  `remote_host_attach_session`,
  `browser_handoff_external_site`,
  `provider_account_mutation`,
  `provider_project_mutation`,
  `managed_workspace_control_plane`,
  `credential_broker_scope`,
  `policy_bundle_target`.
  `target_trust_class` is one of `local_trusted`,
  `local_restricted`, `remote_agent_trusted`,
  `remote_agent_restricted`, `external_provider_trusted`,
  `external_provider_restricted`, `managed_control_plane_trusted`,
  `managed_control_plane_restricted`, `unknown`.
- **Origin.** `origin_actor_class` is re-exported from ADR-0010
  (`human_account`, `installation_or_app_grant`,
  `delegated_user_token`, `project_scoped_grant`,
  `policy_injected_service_identity`). `origin_authority_class`
  is re-exported from the command-descriptor authority-class
  vocabulary without modification. `origin_client_scope` is
  re-exported from ADR-0011 without modification.
  `origin_identity_ref` is the opaque pin to the origin identity
  the broker holds; raw credential material never appears here.
- **Route.** `target_route_class` is one of
  `editor_keybinding_route`,
  `palette_invocation_route`,
  `application_menu_route`,
  `context_menu_route`,
  `toolbar_route`,
  `cli_invocation_route`,
  `ai_tool_invocation_route`,
  `automation_recipe_route`,
  `extension_invocation_route`,
  `external_deep_link_route`,
  `restored_session_reconnect_route`. Names how the user / AI /
  extension / recipe / deep-link arrived at the protected entry;
  reopening an expired session through a restored-session route
  reopens the approval prompt even when the descriptor would
  normally allow apply-direct.
- **Trust / grant floors.** `required_trust_state_floor` is one
  of `trusted`, `restricted` (re-exported from ADR-0001).
  `required_grant_scope_floor` is re-exported from the
  shell-interaction-safety permission-grant-scope vocabulary
  (`once`, `session`, `workspace`, `profile`, `policy_managed`).
  Badges whose floors are not met MUST pair with a
  `command_diagnostic_row_record` whose `disabled_reason_code`
  is one of `workspace_trust_restricted`,
  `required_credential_missing`, `managed_only_channel_required`,
  `policy_blocked_in_context`, or `capability_disabled_by_policy`.
- **Export-safe labels.** `export_safe_target_label_ref` is an
  opaque pin to a target label safe to render in the declared
  `export_safe_wording_class`; `export_safe_origin_label_ref`
  is the matching origin label. `badge_narration_ref` is the
  opaque pin to the accessibility narration a screen reader
  reads when the badge is focused. Labels are resolved by the
  docs / localisation pipeline; raw labels never appear here.
- **Parity.** `applicable_surface_classes` enumerates the
  closed set of rendering surfaces that MUST render the same
  badge for the same (command_id, command_revision_ref,
  target_identity_ref, origin_identity_ref) tuple. Silent
  divergence is non-conforming.
- **Standard fields.** `policy_context`, `redaction_class`,
  `minted_at`.

### `remediation_projection_record`

- **Identity.** `remediation_id` is the stable id.
- **Reason.** `reason_code` is re-exported from the command
  descriptor's `disabled_reason_code` vocabulary without
  modification.
- **Repair.** `repair_hook_ref` is re-exported from ADR-0011
  without modification.
- **Grant scope.** `required_grant_scope` is re-exported from
  the shell-interaction-safety permission-grant-scope
  vocabulary (`once`, `session`, `workspace`, `profile`,
  `policy_managed`).
- **Actionability.** `actionability_class` is one of
  `actionable_by_user`,
  `actionable_by_workspace_admin`,
  `actionable_by_managed_fleet_admin`,
  `actionable_by_provider_owner`,
  `actionable_by_support`,
  `not_actionable_wait_for_lifecycle`,
  `not_actionable_deterministic`.
  A surface that exposes a user-facing "Fix this" button MUST
  verify the row's `actionability_class` is `actionable_by_user`
  or `actionable_by_workspace_admin` (when the viewer holds that
  role); otherwise the surface MUST render the remediation as
  read-only guidance.
- **Deep link.** `follow_up_deep_link_target_kind` is null or
  re-exported from the shareability `deep_link_target_kind`
  vocabulary (`reveal_command_in_palette`,
  `reveal_command_in_application_menu`,
  `reveal_command_in_context_menu`,
  `reveal_command_in_keybinding_editor`,
  `open_command_docs_help_anchor`,
  `prefill_command_invocation_preview`,
  `open_command_about_service_health_panel`). A non-null
  follow-up MUST still re-run the trust, policy, permission,
  preview, approval, credential-broker, managed-channel,
  execution-context, client-scope, publisher, kill-switch, and
  freshness gates before any consequence (see the shareability
  contract).
- **Dismissibility.** `dismissible_by_user` is false for
  remediations whose reason is `capability_disabled_by_policy`,
  `policy_blocked_in_context`, `managed_only_channel_required`,
  `command_retired`, `publisher_not_permitted`, or
  `kill_switch_tripped`; true otherwise.
- **Hint label.** `export_safe_hint_label_ref` is the opaque
  pin to the short hint label a surface may render next to the
  chip.

## Projection rules

A diagnostic row is mechanically producible from:

1. The `command_descriptor_record` for (command_id,
   command_revision_ref).
2. An `invocation_session_packet_record` for the invocation
   (if the row is produced after a dispatch attempt), or a
   synthetic "would-be invocation" computed by the registry for
   a surface rendering a disabled listing (palette / keybinding
   editor / diagnostics panel).
3. The rendering surface's (rendering_surface_class,
   ui_slot_class, palette_visibility_class,
   contextual_filter_class_ref) tuple.
4. The policy context at render time (policy_epoch, trust_state,
   execution_context_id, active_policy_bundle_ref).
5. The keybinding-resolver row (if any) currently bound for this
   command in this context.

The projection MUST:

- Re-export `authority_class`, `capability_scope_class`,
  `preview_class`, `approval_posture_class`,
  `ai_tool_surfacing_class`, `palette_visibility_class`,
  `ui_slot_class`, `lifecycle_state`, `support_class`,
  `release_channel`, `declared_freshness_class`,
  `client_scope`, `redaction_class`, `disabled_reason_code`,
  and `repair_hook_ref` verbatim from the descriptor / invocation
  packet — never mint a parallel value.
- Re-export `docs_help_anchor_ref` and
  `shortcut_narration_hint` verbatim — never mint a parallel
  help anchor or parallel shortcut narration.
- Re-compute `disabled_reason_code` deterministically when the
  invocation-session packet's enablement decision is
  `disabled_with_reason` / `hidden_with_reason`; when the
  enablement decision is `enabled` but the descriptor's
  `capability_scope_class` excludes the rendering surface's
  `client_scope`, the row MUST deny with
  `disabled_reason_code = client_scope_excludes_surface`.
- Carry a non-null `remediation_projection_ref` whenever the
  reason is in the remediable set (every
  `disabled_reason_code` value except `command_retired`,
  `command_version_unknown`, and `capability_lifecycle_retired`
  — those three carry null remediation and a typed
  "not_actionable_wait_for_lifecycle" posture rendered as
  read-only guidance).
- Carry a non-null `protected_entry_badge_ref` whenever one or
  more of the protected-entry selection rules below apply.

## Protected-entry selection rules

A `protected_entry_badge_ref` MUST be set on the row when any of
the following hold:

- The descriptor's `capability_scope_class` is one of
  `credential_or_secret_bearing`, `managed_workspace_control`,
  `policy_authoring_or_waiver`,
  `externally_visible_mutation`,
  `irreversible_high_blast_mutation`.
- The descriptor's `preview_class` is one of
  `remote_attach_preview`,
  `browser_handoff_preview`,
  `credential_or_secret_access_preview`,
  `policy_authoring_or_waiver_preview`,
  `managed_workspace_control_preview`,
  `install_or_update_preview`,
  `collaboration_invite_preview`,
  `irreversible_publish_preview`,
  `externally_mutating_preview`.
- The descriptor's `ui_slot_hints` include
  `terminal_context_menu` and the invocation is an apply (not a
  read-only metadata inspection).
- The invocation's `issuing_surface` is one of
  `remote_agent_surface`, `extension_invocation_surface`,
  `automation_recipe_surface`, `ai_tool_surface` and the
  descriptor's `capability_scope_class` is
  `reversible_local_mutation` or above.
- The invocation's route is a `restored_session_reconnect_route`
  or an `external_deep_link_route` regardless of other fields
  (reconnects and deep links reopen origin and target disclosure).

When none of the rules above hold, `protected_entry_badge_ref` is
null and the row renders without a badge.

## Parity rules

For the same (command_id, command_revision_ref, policy_epoch,
trust_state, issuing_surface, ui_slot_class, execution_context_id)
tuple, the following surfaces MUST render rows whose
`disabled_reason_code`, `remediation_projection_ref.reason_code`,
`authority_class`, `capability_scope_class`, `preview_class`,
`approval_posture_class`, `docs_help_anchor_ref`, and
`shortcut_narration_hint_ref` fields are field-for-field
identical:

- `command_palette`
- `global_application_menu`
- `editor_context_menu` / `explorer_context_menu` /
  `review_context_menu` / `source_control_context_menu` /
  `terminal_context_menu` / `search_context_menu`
- `primary_toolbar` / `secondary_toolbar`
- `keybinding_help` / `keybinding_editor`
- `cli_help`
- `ai_tool_surface`
- `automation_recipe_response_surface`
- `docs_reference_page_surface`
- `about_service_health_panel_surface`
- `command_diagnostics_panel_surface`
- `support_export_surface`

The `parity_surface_set` field on the row enumerates the
applicable subset (a CLI-only command's parity set will not
include context menus; a UI-only command's parity set will not
include CLI help). Silent divergence within the declared set is
non-conforming and detected mechanically by the parity-diff
corpus under
[`/docs/commands/command_parity_diff.md`](../commands/command_parity_diff.md).

## Export-safe wording classes

`export_safe_wording_class` is one of:

| Class                          | Meaning                                                                                                                                                   |
|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `safe_for_public_docs`         | Labels may be rendered in public docs screenshots, README images, and marketing material. Target / origin labels name categories, not identities.          |
| `safe_for_support_export`      | Labels may be rendered in a user-triggered support bundle. Target / origin labels name opaque ids the support responder resolves behind the bundle gate.   |
| `operator_only_restricted`     | Labels render only under an operator-only scope; public docs / support rendering MUST deny with `derived_explanation_uncited` or an equivalent gate.      |
| `managed_only_restricted`      | Labels render only under a managed-control-plane scope; any other rendering surface MUST deny.                                                             |

A badge's labels inherit the row's class; a row whose class is
narrower than the badge's `export_safe_wording_class` MUST deny
render.

## Keybinding-help and docs-help pivots

The row MUST project the descriptor's `shortcut_narration_hint`
verbatim as its `keybinding_help_pivot.shortcut_narration_hint_ref`
and, when a keybinding resolver row is currently bound for the
command in this context, MUST project that row's opaque id as
its `keybinding_help_pivot.keybinding_resolver_row_ref`.

The row MUST project the descriptor's `docs_help_anchor_ref`
verbatim as its `docs_help_pivot.docs_help_anchor_ref`. Minting
a parallel shortcut narration or parallel help anchor is
non-conforming — parity audits compare these refs for
field-for-field equality across the `parity_surface_set`.

## Linkage to neighbouring contracts

- **Command descriptor contract.** `authority_class`,
  `capability_scope_class`, `preview_class`,
  `approval_posture_class`, `ai_tool_surfacing_class`,
  `palette_visibility_class`, `ui_slot_class`,
  `enablement_decision_class`, `disabled_reason_code`,
  `result_contract_class`, and `evidence_ref_class` are
  re-exported from
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
  without modification.
- **UI slot taxonomy.** `ui_slot_class` is re-exported from
  [`/schemas/commands/ui_slot_taxonomy.schema.json`](../../schemas/commands/ui_slot_taxonomy.schema.json)
  without modification; the badge's `applicable_surface_classes`
  and the row's `parity_surface_set` quote the same slot
  taxonomy.
- **Keybinding resolver.** `keybinding_resolver_row_ref` pins
  the resolver row frozen in
  [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
  and narrated by
  [`/docs/ux/keybinding_resolver_contract.md`](./keybinding_resolver_contract.md).
- **Shareability and automation contract.** The `follow_up_deep_link_target_kind`
  vocabulary re-exports the shareability contract's
  `deep_link_target_kind` without modification; parity surfaces
  re-export shareability's `unavailability_parity_surface` values
  by intent (the set is broader here because the diagnostics row
  covers surfaces the shareability record does not: the editor
  context menu, the command-diagnostics panel, the support
  export).
- **Disabled-reason grammar.** Rendered reason classes, alternate-route
  microcopy, freshness wording, and quantitative scope phrases are
  governed by
  [`/docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  and
  [`/artifacts/ux/disabled_reason_classes.yaml`](../../artifacts/ux/disabled_reason_classes.yaml).
  Diagnostics rows carry the typed reason and refs; rendering surfaces
  consume the grammar instead of authoring prose from the machine code.
- **Shell interaction safety.** `permission_grant_scope`,
  `authority_class`, and `representation_class` are re-exported
  from
  [`/schemas/ux/interaction_safety.schema.json`](../ux/interaction_safety.schema.json)
  without modification. A badge rendered in the
  `support_export_surface` MUST respect the same
  representation-class posture the shell-interaction-safety
  contract defines.
- **ADR-0001 workspace trust.** `trust_state`, `required_trust_state_floor`,
  and `policy_context.trust_state` are re-exported without
  modification.
- **ADR-0007 secret broker.** `credential_broker_step_up_entry`
  and `credential_broker_scope` target kinds route through the
  broker; raw credential material never appears in a badge.
- **ADR-0008 settings resolver.** Admin policy may narrow the
  `remediation_projection_record.dismissible_by_user` flag to
  `false`, may narrow `actionability_class` to
  `actionable_by_workspace_admin` /
  `actionable_by_managed_fleet_admin`, and may narrow
  `export_safe_wording_class` to `managed_only_restricted`;
  policy MAY NOT silently widen.
- **ADR-0009 execution context.** `execution_context_id` is
  re-exported without modification. Protected-entry badges whose
  target route is `restored_session_reconnect_route` MUST
  re-validate the execution context at render time.
- **ADR-0010 browser handoff and connected provider.**
  `origin_actor_class` re-exports ADR-0010's
  `provider_actor_class` without modification;
  `browser_handoff_external_site` target kind routes through
  the browser-handoff envelope frozen in
  [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json).
- **ADR-0011 capability lifecycle.** `lifecycle_state`,
  `support_class`, `release_channel`, `freshness_class`,
  `client_scope`, `redaction_class`, and `repair_hook_ref` are
  re-exported without modification. A generic "available" chip
  is forbidden.
- **ADR-0012 extension manifest.** `publisher_not_permitted`
  disabled reasons resolve against the ADR-0012 permitted-
  publisher set; extension-initiated authority rides
  `extension_initiated`.
- **ADR-0013 docs / help truth.** `docs_help_anchor_ref` pins a
  `docs_pack_manifest_record` anchor; surfaces deny render with
  `derived_explanation_uncited` if they cannot resolve the
  anchor under a `citation_required` posture.

## Schema of record

The eventual command-registry / invocation-session /
diagnostics-surface crates' Rust types are the schema of record.
The JSON Schema export at
[`/schemas/commands/diagnostic_projection.schema.json`](../../schemas/commands/diagnostic_projection.schema.json)
is the cross-tool boundary every non-owning surface reads. Adding
a new protected-entry class, target-kind class, target-trust
class, target-route class, actionability class, export-safe
wording class, or rendering-surface class is additive-minor and
bumps `diagnostic_projection_schema_version`; repurposing an
existing value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR-0004 through ADR-0014.

## Source anchors

- [`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  — the command descriptor and invocation-session packet the
  diagnostic row projects from.
- [`docs/commands/shareability_and_automation_contract.md`](../commands/shareability_and_automation_contract.md)
  — the shareability record whose deep-link target-kind
  vocabulary the remediation follow-up re-exports.
- [`docs/commands/command_parity_diff.md`](../commands/command_parity_diff.md)
  — the seed parity-diff corpus that consumes the
  `parity_surface_set` field mechanically.
- [`docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  — the rendered disabled-reason, alternate-route, freshness, and
  translation-safe microcopy grammar consumed by diagnostics surfaces.
- [`docs/ux/keybinding_resolver_contract.md`](./keybinding_resolver_contract.md)
  — the keybinding resolver row whose opaque id the
  keybinding-help pivot pins.
- [`docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — the authority class, permission grant scope, and
  representation class the row / badge re-export.
- [`schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — the lifecycle, support, release, freshness, client-scope,
  redaction, and repair-hook vocabularies re-exported here.
- [`schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — the browser-handoff envelope the
  `browser_handoff_external_site` target kind routes through.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md),
  [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md),
  [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md),
  [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — the command-diagnostics, disabled-reason, and protected-
  entry target / origin posture the row and badge plug into.
