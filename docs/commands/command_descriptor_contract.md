# Command-descriptor contract

This document freezes the command-descriptor contract every palette,
application and context menu, keybinding / shortcut-help layer, CLI
help output, AI-tool surface, automation-recipe surface, and
invocation-session packet reads before a command is surfaced,
enabled, disabled with a typed reason, previewed, approved,
executed, or replayed. It freezes commands as a governed product
object before shell, CLI, recipes, or AI surfaces appear so those
surfaces project against one descriptor shape, one invocation-session
envelope, one preview-class taxonomy, and one disabled-reason
vocabulary instead of inventing per-surface copies.

The machine-readable boundary is
[`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json);
worked examples (palette / menu / AI-tool first-party commands, a
disabled-with-reason case, a preview-required destructive case, an
approval-required publish case, and an invocation-session packet
projecting a result-evidence success) live in
[`/fixtures/commands/command_descriptor_examples/`](../../fixtures/commands/command_descriptor_examples/).
The seeded canonical registry companion for alias lifecycle,
discoverability projections, current-shortcut display, disabled-state
explainers, diagnostics, and machine-facing names lives in
[`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json),
[`/fixtures/commands/seed_commands/`](../../fixtures/commands/seed_commands/),
and
[`/artifacts/commands/command_registry_seed.yaml`](../../artifacts/commands/command_registry_seed.yaml).
The governed palette projection for result rows, selected-row action
footers, alternate invocation rows, and cross-surface row rendering now
lives in
[`/docs/commands/palette_row_contract.md`](./palette_row_contract.md),
[`/schemas/commands/palette_result.schema.json`](../../schemas/commands/palette_result.schema.json),
[`/schemas/commands/palette_action_footer.schema.json`](../../schemas/commands/palette_action_footer.schema.json),
and
[`/fixtures/commands/palette_rows/`](../../fixtures/commands/palette_rows/).

The eventual command-registry / invocation-session crates' Rust
types are the schema of record. This document and the JSON Schema
export are the cross-tool boundary every non-owning surface reads;
if this document and a neighbouring ADR disagree, the ADR wins and
this document MUST be updated in the same change.

## Why freeze this now

Without a single descriptor shape, the palette, the application
menu, each context menu, the keybinding help layer, the CLI help
output, each AI-tool surface, and each automation recipe would
mint its own command dialect, its own argument schema, its own
"is this enabled?" rule, its own preview posture, its own approval
step, its own audit envelope, and its own parity story between
"you asked `foo.bar`" and "we did `foo.bar`". By the time a live
command router lands, five parallel command registries would have
diverged on naming, on accessibility labels, on what counts as
"destructive", and on what a successful invocation is allowed to
produce.

This contract is the missing piece that lets, at freeze time and
before any runtime:

- the palette render a command with its canonical label, its
  accessibility role, its docs / help anchor, and its typed
  disabled-reason + repair-hook without re-deriving the mapping
  per surface,
- the menu and context-menu surfaces surface exactly the commands
  the descriptor declared UI-slot hints for, with the owning
  surface narrowing but never silently widening,
- the keybinding / shortcut-help layer narrate a consistent
  shortcut description on the primary surface instead of
  tooltip-only disclosure,
- the CLI surface resolve canonical verbs, typed arguments,
  argument provenance, and human-legible `--help` output from the
  same descriptor the palette reads,
- the AI-tool surface expose a closed set of AI-callable commands
  with explicit approval postures; irreversible / high-blast
  commands are denied at the descriptor level rather than gated
  per-site,
- the invocation-session packet quote one enablement decision, one
  preview / approval posture, one execution intent, one outcome,
  and one evidence-ref set so replay, audit, parity-audit, and
  rollback tooling read one envelope.

## Scope

Frozen at this revision:

- One `command_descriptor_record` shape with a closed set of
  issuing surfaces, authority classes, capability scope classes,
  argument kinds, argument provenance values, UI slot classes,
  palette-visibility classes, AI-tool surfacing classes, preview
  classes, approval postures, enablement decision classes,
  disabled-reason codes, execution-intent classes, outcome
  classes, result-contract classes, and evidence-ref classes.
- One `invocation_session_packet_record` shape so palette / menu /
  keybinding / CLI / AI-tool / automation-recipe surfaces emit the
  same envelope when they dispatch a command, record the
  argument-provenance map, capture the context snapshot, record the
  enablement decision, record the preview / approval posture,
  record the execution intent, record the outcome, and enumerate
  created artifacts and evidence refs.
- Rules for preview-required, approval-required,
  disabled-with-reason, and result-evidence states so surfaces
  cannot render "this command is available" without a typed
  backing decision.
- Descriptor projection rules for palette / menu / keybinding-help
  / CLI-help / AI-tool surfaces so one descriptor projects
  consistently without field churn or label drift.

Out of scope until a superseding decision row opens:

- The live command router, the command registry runtime, and the
  runtime-generated parity-diff automation between palette / menu
  / CLI / AI-tool registrations.
- The seed report format and synthetic parity corpus now live in
  [`/docs/commands/command_parity_diff.md`](./command_parity_diff.md)
  and
  [`/artifacts/commands/command_parity_seed.yaml`](../../artifacts/commands/command_parity_seed.yaml);
  what remains out of scope here is runtime-emitted surface
  capture and diffing against shipped registries.
- The full palette UI (scoring, ranking, typeahead).
- Recipe / macro / batch-command authoring surfaces (one step
  further out; they will mint `invocation_session_packet_record`
  sequences, not their own envelope).
- Keybinding resolution, conflict-detection, and remapping UI;
  the frozen cross-surface resolver and import-bridge contract
  now lives in
  [`/docs/ux/keybinding_resolver_contract.md`](../ux/keybinding_resolver_contract.md).
- The preview / apply / revert packet bodies, the approval ticket
  bodies, and the evidence packet bodies (these are frozen by
  their own lanes; this document names the refs a descriptor /
  invocation-session packet quotes into).

## Canonical ownership

Each command descriptor has exactly one canonical owner; surfaces
that render a command they do not own MUST quote the owner's
`command_descriptor_record` and `invocation_session_packet_record`
rather than mint a copy.

| Command family                              | Canonical owner                    |
|---------------------------------------------|------------------------------------|
| Workspace / open / clone / import / restore | `workspace_command_registry`       |
| Editor / buffer / selection / refactor      | `editor_command_registry`          |
| Source control / review / PR                | `source_control_command_registry`  |
| Search / find / symbol-jump                 | `search_command_registry`          |
| Settings / policy / waiver authoring        | `settings_command_registry`        |
| AI apply / AI session / AI tool             | `ai_command_registry`              |
| Extension / install / update                | `extension_command_registry`       |
| Support / export / diagnostic               | `support_command_registry`         |
| Managed workspace control                   | `managed_workspace_command_registry` |

The registries listed above are projection targets; this document
does not land any of them. It pins the shape they MUST publish.

## Record fields

The full field set lives in
[`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json).
The notable fields are:

- **Identity.** `command_id` is the stable id the registry uses
  across revisions; `command_revision_ref` is the opaque pin an
  `invocation_session_packet_record.command_revision_ref` cites
  so a replay proves it saw the same descriptor shape.
- **Canonical verb.** `canonical_verb` is the dotted snake_case
  verb CLI / AI-tool / automation-recipe surfaces resolve against
  (`workspace.open_folder`, `git.commit`, `ai.apply_patch`).
  Stable across descriptor revisions; rename is a new descriptor.
- **Labels and accessibility.** `primary_label_ref` is the opaque
  pin to the user-visible label; `accessibility_label_path` is
  the per-surface accessibility payload (primary, short, long
  description, keyboard-shortcut narration, accessibility role).
  Surfaces MUST project these refs verbatim; minting a parallel
  label per surface is non-conforming.
- **Shortcut narration.** `shortcut_narration_hint` names the
  narration a keybinding-help layer reads when a shortcut is
  bound and when one is not bound, plus a `chord_class_hint` for
  grouping.
- **Docs / help anchor.** `docs_help_anchor_ref` pins the
  docs-pack anchor every palette tooltip, menu help, keybinding-
  help expansion, CLI `--help` output, and AI-tool description
  reads. Re-exports the anchor-kind vocabulary from
  [`/docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md).
- **Aliases.** `aliases` is the closed set of alias shapes
  (`legacy_command_id`, `alternate_palette_phrasing`,
  `alternate_cli_verb`, `ai_tool_handle`, `keybinding_target`)
  so rename / bridge / surface-specific phrasing is a typed
  declaration rather than a free-form string.
- **Typed arguments.** `typed_arguments` is an ordered set of
  `typed_argument_slot` entries. Each slot pins a stable
  snake_case name, an `argument_kind`, a required flag, and
  optional default provenance, enum refs, numeric bounds,
  trust-state-triggered policy pinning, and a narration label.
  Order is stable across revisions; removing / reordering a slot
  is breaking.
- **Capability class.** `capability_scope_class` is one of
  `inert_metadata_only`, `reversible_local_read`,
  `reversible_local_mutation`, `recoverable_durable_mutation`,
  `externally_visible_mutation`, `irreversible_high_blast_mutation`,
  `credential_or_secret_bearing`, `managed_workspace_control`,
  `policy_authoring_or_waiver`. Silent promotion at invocation
  time is forbidden; an invocation that drifts beyond its declared
  scope denies with `scope_class_drifted_from_descriptor`.
- **Discoverability metadata.** `palette_visibility` is one of
  `always_visible`, `visible_when_enabled`,
  `visible_when_focused_context_matches`, `developer_only`,
  `policy_restricted`, `hidden_palette_callable_only`.
  `ai_tool_surfacing_class` is one of `not_ai_callable`,
  `ai_callable_read_only`, `ai_callable_reversible_mutation`,
  `ai_callable_recoverable_durable_mutation`,
  `ai_callable_externally_visible_mutation_requires_approval`,
  `ai_callable_irreversible_high_blast_denied`.
- **UI slot hints.** `ui_slot_hints` is a closed set of
  `ui_slot_class` entries (palette, global application menu,
  context menus, toolbars, status bar, keybinding help, CLI
  help, AI-tool surface). This is the coarse discoverability
  vocabulary, not the final shell slot catalog. Exact slot-family
  and slot-key ownership is translated by
  [`/docs/commands/command_graph_and_ui_slots_seed.md`](./command_graph_and_ui_slots_seed.md)
  and
  [`/schemas/commands/ui_slot_taxonomy.schema.json`](../../schemas/commands/ui_slot_taxonomy.schema.json).
  A descriptor requests surfacing; the owning surface MAY narrow
  (policy / lifecycle / client-scope) but MAY NOT silently widen
  to a slot the descriptor did not declare.
- **Lifecycle metadata.** `lifecycle_state`, `support_class`,
  `release_channel`, `declared_freshness_class`, and
  `client_scopes` are re-exported from
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  without modification (ADR 0011). A generic "available" chip is
  forbidden.
- **Preview class.** `preview_class` is the high-risk preview
  taxonomy below; surfaces that elide the preview for a non-
  `no_preview_required` class deny apply with
  `preview_required_not_shown`.
- **Approval posture.** `approval_posture_class` is one of
  `no_approval_required`, `explicit_confirmation_required`,
  `step_up_authentication_required`, `admin_policy_approval_required`,
  `second_party_review_required`, `managed_only_approval_required`.
  `ai_callable_externally_visible_mutation_requires_approval`
  commands MUST declare `second_party_review_required`,
  `admin_policy_approval_required`, or
  `managed_only_approval_required`; `explicit_confirmation_required`
  alone is not an AI-callable approval path for externally visible
  mutations.
- **Result contract.** `result_contract` pins the
  `result_contract_class`, the optional artifact-kind ref, the
  optional typed-value-shape ref, and the closed set of
  `evidence_ref_class` values an invocation-session packet MUST
  carry on success.
- **Policy context and redaction.** `policy_context`
  (`policy_epoch`, `trust_state`, `execution_context_id`) and
  `redaction_class` are re-exported from ADR 0001 / ADR 0007 /
  ADR 0008 / ADR 0009 / ADR 0011 without modification.

## High-risk preview-class taxonomy

`preview_class` is the closed taxonomy a command descriptor
declares so the preview / apply / revert lane knows whether the
descriptor may be invoked with
`execution_intent_class = apply_direct_trusted_path` or whether
apply MUST ride a preview path. The admissible values are:

| Preview class                                    | Meaning                                                                                                                                   |
|--------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| `no_preview_required`                            | Inert / metadata / reversible-read commands may apply directly.                                                                            |
| `inline_summary_preview`                         | One-row summary inline on the invocation surface; admissible for reversible local mutations.                                               |
| `structured_diff_preview`                        | A structured diff rendered before apply.                                                                                                   |
| `batch_scope_preview`                            | Batch-scope review contract re-exported from `docs/ux/shell_interaction_safety_contract.md`; select-all MUST say whether it means visible / loaded / all-matching rows. |
| `destructive_bulk_mutation_preview`              | Destructive batch mutation; preview MUST enumerate protected-path / policy-blocked members before commit.                                  |
| `broad_workspace_scope_preview`                  | Workspace-wide or cross-repo scope; preview MUST declare the effective scope class.                                                        |
| `irreversible_publish_preview`                   | External publish to a remote system with no local rollback.                                                                                |
| `externally_mutating_preview`                    | External system mutation that may or may not be reversible externally.                                                                     |
| `credential_or_secret_access_preview`            | Touching broker / secret store; preview MUST pair with `step_up_authentication_required` approval posture.                                 |
| `policy_authoring_or_waiver_preview`             | Authoring / editing / waiving admin policy; preview MUST pair with `admin_policy_approval_required` approval posture.                      |
| `managed_workspace_control_preview`              | Managed workspace control-plane action; pairs with `managed_only_approval_required`.                                                       |
| `remote_attach_preview`                          | Attaching to a remote agent / host; preview MUST surface target identity and scope.                                                        |
| `install_or_update_preview`                      | Installing / updating an extension / bundle / provider; preview MUST surface publisher and lifecycle posture.                              |
| `collaboration_invite_preview`                   | Inviting a remote collaborator; preview MUST surface permission grant scope.                                                               |
| `browser_handoff_preview`                        | Handing off to a browser (ADR 0010); preview quotes the `browser_handoff_packet` envelope.                                                 |
| `rich_active_content_preview`                    | Rendering active / scriptable / notebook-widget content; preview sandboxes.                                                                |
| `bidi_or_invisible_formatting_reveal_preview`    | Revealing bidi / zero-width / invisible formatting before apply.                                                                           |
| `confusable_identifier_reveal_preview`           | Revealing confusable identifiers in a diff / symbol surface before apply.                                                                  |

A descriptor that declares a non-`no_preview_required` preview
class MUST NOT be invocable with
`execution_intent_class = apply_direct_trusted_path`. Apply direct
is admissible only for `inert_metadata_only`,
`reversible_local_read`, and `reversible_local_mutation` scopes
paired with `no_preview_required`.

## Disabled-reason vocabulary

`disabled_reason_code` is the closed vocabulary an enablement
decision / invocation-session outcome cites when a command is
disabled, hidden, or denied. Every reason pairs with a
`repair_hook_ref` (re-exported from ADR 0011); silent disable with
no typed reason or no repair hook is non-conforming.

The admissible values are:

- `workspace_trust_restricted` — ADR 0001 trust state narrows
  this command in the current workspace.
- `capability_lifecycle_retired` — the descriptor's
  `lifecycle_state` is `retired` on this surface.
- `capability_disabled_by_policy` — ADR 0008 admin policy has
  disabled the capability on this surface.
- `kill_switch_tripped` — an ADR 0011 kill switch has tripped on
  a dependency.
- `client_scope_excludes_surface` — the descriptor's
  `client_scopes` excludes the rendering surface.
- `freshness_floor_unmet` — the descriptor's
  `declared_freshness_class` is below the floor ADR 0011 requires
  for this surface.
- `required_provider_unlinked` — a required provider (ADR 0010)
  is unlinked.
- `required_credential_missing` — a required credential handle
  (ADR 0007) is missing.
- `required_argument_unresolved` — a required typed argument
  cannot resolve; `argument_provenance_map` on the
  invocation-session packet shows the gap.
- `execution_context_unavailable` — no ADR 0009 execution-context
  can host the invocation.
- `managed_only_channel_required` — the descriptor requires
  `managed_only_channel` and the running build is on a different
  channel.
- `dependency_state_below_command_ceiling` — an ADR 0011
  dependency narrows the effective lifecycle state below the
  command's declared ceiling.
- `command_deprecated_within_window` — the command is in the
  deprecation window; surfaces MAY render with a migration hint.
- `command_retired` — the command is retired.
- `command_version_unknown` — the registry does not recognise the
  descriptor's `command_descriptor_schema_version`.
- `preview_denial_no_safe_preview` — the preview path cannot
  render a safe preview (e.g. the target body could not be
  sanitised); apply is denied.
- `approval_denial_no_approval_path` — no approval path is
  available (no second-party reviewer, no admin approver, no
  managed-workspace approver).
- `publisher_not_permitted` — an extension-provided descriptor
  whose publisher is outside the permitted set (ADR 0012).
- `policy_blocked_in_context` — admin policy is blocking this
  command in the current context.
- `authority_class_unresolved` — the invocation could not resolve
  its authority class.
- `issuing_surface_unresolved` — the invocation could not resolve
  its issuing surface.
- `scope_class_drifted_from_descriptor` — the invocation tried to
  apply a scope class above the descriptor's declared ceiling.
- `preview_required_not_shown` — the descriptor declared a
  non-`no_preview_required` preview class and the preview was not
  shown before apply.
- `basis_snapshot_drifted` — the context-snapshot basis changed
  between preview and apply; re-preview is required.

Adding a reason is additive-minor and bumps
`command_descriptor_schema_version`. Repurposing an existing
reason is breaking and requires a new decision row.

## Invocation-session packet

Every consequence-bearing command invocation emits exactly one
`invocation_session_packet_record`. The packet is the envelope
every palette / menu / keybinding / CLI / AI-tool / automation-
recipe / scripted-invocation site writes; replay, audit, parity,
and rollback tooling read one shape.

The packet carries:

- **Identity.** `invocation_session_id`, `command_id`, and
  `command_revision_ref` — retries / rollbacks / follow-up apply
  phases ride the same session id; a replay against a different
  descriptor revision reopens the preview / approval posture.
- **Issuing surface and authority.** `issuing_surface` is one of
  `command_palette`, `global_application_menu`, `context_menu`,
  `toolbar_button`, `keybinding_chord`, `cli_surface`,
  `ai_tool_surface`, `automation_recipe_surface`,
  `scripted_invocation_surface`, `remote_agent_surface`,
  `extension_invocation_surface`. `authority_class` is re-exported
  from `docs/ux/shell_interaction_safety_contract.md` without
  modification.
- **Argument provenance map.** Per-argument rows pinning the
  `argument_provenance` value (`user_typed`,
  `user_selected_from_palette_suggestion`, `default_from_descriptor`,
  `inferred_from_focused_context`, `inferred_from_selection`,
  `supplied_by_keybinding_args`, `supplied_by_cli_argv`,
  `ai_proposed_requires_review`, `automation_recipe_supplied`,
  `extension_supplied`, `policy_pinned_cannot_edit`) and the
  resolved value ref. Every typed-arguments slot on the descriptor
  MUST have exactly one entry (or the enablement decision is
  `disabled_with_reason` / `required_argument_unresolved`).
- **Context snapshot.** `focused_entity_ref`, `selection_ref`,
  `workspace_trust_state`, `execution_context_id`,
  `scope_filter_class_ref`, and `basis_snapshot_ref`. If the
  basis drifts before apply, the packet MUST deny with
  `basis_snapshot_drifted`.
- **Enablement decision.** `decision_class` is one of `enabled`,
  `disabled_with_reason`, `hidden_with_reason`; non-enabled
  decisions MUST carry a `disabled_reason_code` and a
  `repair_hook_ref`.
- **Preview / approval posture.** `preview_posture.preview_shown`
  MUST be true when `preview_class_declared` is not
  `no_preview_required`; `approval_posture.approval_state` is
  one of `not_required`, `approval_pending`, `approval_granted`,
  `approval_denied`, `approval_expired`. `not_required` is
  admissible only when `approval_posture_class_declared =
  no_approval_required`.
- **Execution intent.** `execution_intent` is one of
  `query_only_no_mutation`, `propose_preview_only`,
  `apply_after_preview`, `apply_with_approval`,
  `apply_direct_trusted_path`, `rollback_or_revert`,
  `simulate_or_dry_run`, `schedule_for_background_execution`,
  `cancel_pending_invocation`. `apply_direct_trusted_path` is
  admissible only when the descriptor permits it.
- **Outcome.** `outcome_class` is one of `succeeded`,
  `succeeded_with_warnings`, `denied_by_enablement`,
  `denied_by_preview`, `denied_by_approval`, `cancelled_by_user`,
  `failed_with_typed_error`, `rolled_back_after_apply`,
  `partially_applied_and_halted`, `scheduled_pending_background`.
  Denied outcomes MUST carry a `disabled_reason_code`;
  `partially_applied_and_halted` MUST enumerate applied and
  unapplied artifact refs.
- **Created artifacts and evidence refs.** `created_artifact_refs`
  enumerates artifacts the invocation created / modified / deleted
  against the `result_contract_class` vocabulary. `evidence_refs`
  MUST cover every `evidence_ref_class_required` entry from the
  descriptor's `result_contract`; any gap is non-conforming.

## Result-evidence rule

An invocation-session packet with `outcome_class = succeeded` or
`succeeded_with_warnings` against a descriptor whose
`capability_scope_class` is anything but `inert_metadata_only` or
`reversible_local_read` MUST carry at least one entry in
`evidence_refs` covering every `evidence_ref_class_required`
entry the descriptor's `result_contract` declares. A bare "apply
succeeded" packet with no evidence is non-conforming and denies
on replay. This mirrors the ADR 0007 / ADR 0011 rule that every
auditable consequence MUST be addressable by a typed evidence ref.

## Preview-required, approval-required, disabled-with-reason states

The schema represents each of the four required states as a
separately addressable configuration on the same two records:

- **Preview-required.** A descriptor declares a non-
  `no_preview_required` `preview_class`; the invocation-session
  packet's `preview_posture.preview_shown` MUST be true before
  any outcome other than `denied_by_preview` /
  `cancelled_by_user`. Schema enforcement: the
  `preview_class_declared != no_preview_required` +
  `preview_shown = false` combination denies with
  `outcome_class = denied_by_preview` and
  `disabled_reason_code = preview_required_not_shown`.
- **Approval-required.** A descriptor declares
  `approval_posture_class` other than `no_approval_required`; the
  invocation-session packet's
  `approval_posture.approval_state` walks through
  `approval_pending` → `approval_granted` / `approval_denied` /
  `approval_expired`. Denied or expired states MUST resolve to
  `outcome_class = denied_by_approval` with
  `disabled_reason_code = approval_denial_no_approval_path`.
- **Disabled-with-reason.** A descriptor's enablement decision
  resolves to `disabled_with_reason` / `hidden_with_reason` with
  a typed `disabled_reason_code` and a `repair_hook_ref`. The
  invocation-session packet MAY still be minted to record the
  denial; the `outcome_class` resolves to
  `denied_by_enablement` with the same `disabled_reason_code`.
- **Result-evidence.** A descriptor's `result_contract` declares
  `evidence_ref_class_required` values; the invocation-session
  packet's `evidence_refs` MUST cover every declared class on
  `succeeded` / `succeeded_with_warnings` outcomes. Parity audits
  consume this rule mechanically.

## Descriptor projection across surfaces

One descriptor projects consistently into palette, menu,
keybinding / shortcut help, CLI help, and AI-tool surfaces:

- **Palette** reads `primary_label_ref`,
  `accessibility_label_path`, `docs_help_anchor_ref`,
  `palette_visibility`, `ui_slot_hints` (for grouping), the
  enablement decision, and the `disabled_reason_code` +
  `repair_hook_ref` when disabled.
- **Application / context menu** reads `primary_label_ref`, the
  `ui_slot_hint` whose `ui_slot_class` is
  `global_application_menu` / `*_context_menu` (including
  `menu_path_refs`), `accessibility_label_path`,
  `shortcut_narration_hint`, and the enablement decision.
- **Keybinding / shortcut help** reads `shortcut_narration_hint`,
  `accessibility_label_path.keyboard_shortcut_narration_ref`,
  `docs_help_anchor_ref`, and the `ui_slot_hint` whose
  `ui_slot_class` is `keybinding_help`.
- **CLI help** reads `canonical_verb`, `typed_arguments` (for
  `--help` output), `docs_help_anchor_ref`, `aliases` whose
  `alias_kind` is `alternate_cli_verb`, and `client_scopes` to
  decide whether the command is exposed on the CLI.
- **AI-tool surface** reads `canonical_verb` (or the
  `ai_tool_handle` alias), `typed_arguments`,
  `ai_tool_surfacing_class`, `preview_class`,
  `approval_posture_class`, `capability_scope_class`, and
  `docs_help_anchor_ref`. `ai_callable_irreversible_high_blast_denied`
  means the command is never exposed as an AI tool.

A surface that reads any other field is allowed, but it MUST NOT
mint a different version of the above fields (different labels,
different shortcut narration, different help anchor, different
argument schema). A parity audit between two surfaces' emitted
rows for the same `command_id` / `command_revision_ref` MUST
match field-for-field on these fields. Silent divergence is
non-conforming.

Concrete shell slot-key ownership for title/context bar, rail,
sidebar, editor chrome, bottom panel, inspector, status bar,
onboarding affordances, and companion handoff surfaces is frozen
separately in
[`/docs/commands/command_graph_and_ui_slots_seed.md`](./command_graph_and_ui_slots_seed.md).
This contract keeps `ui_slot_hints` deliberately coarse so the
command object remains stable while the shell slot taxonomy grows
without field churn.

## Linkage to neighbouring contracts

- **ADR 0011 capability lifecycle.** `lifecycle_state`,
  `support_class`, `release_channel`, `freshness_class`,
  `client_scope`, `redaction_class`, and `repair_hook_ref` are
  re-exported from
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  without modification.
- **`docs/ux/shell_interaction_safety_contract.md`.**
  `authority_class` is re-exported without modification. The
  preview path, apply / revert phase, permission prompt, and
  responsive-fallback surface the invocation-session packet cites
  via `preview_record_ref` are frozen there; this document names
  the ref, not the body.
- **ADR 0001 workspace trust.** `workspace_trust_state` and
  `trust_state` are re-exported without modification. Commands
  whose typed argument's `policy_pinned_when_trust_state_is`
  resolves for the current state force the argument provenance to
  `policy_pinned_cannot_edit`.
- **ADR 0007 secret broker.** `credential_or_secret_bearing`
  scope class and `credential_handle_ref` argument kind route
  through the broker; raw credential material never appears on
  the invocation-session packet.
- **ADR 0008 settings resolver.** Admin policy may narrow
  palette visibility, ui-slot hints, client scopes, approval
  posture, and enabled-reason codes; policy MAY NOT silently
  widen beyond the frozen rules.
- **ADR 0009 execution context.** `execution_context_id` and the
  `scope_filter_class_ref` in the context snapshot are
  re-exported without modification.
- **ADR 0010 browser handoff.** `browser_handoff_preview` preview
  class and `browser_handoff_packet_ref` evidence class route
  through the ADR 0010 envelope; raw URLs never cross the
  invocation-session packet boundary.
- **ADR 0012 extension manifest.** Extension-provided descriptors
  resolve `publisher_not_permitted` from the ADR 0012 permitted-
  publisher set; extension-initiated authority rides
  `extension_initiated`.
- **ADR 0013 docs / help truth.** `docs_help_anchor_ref` pins a
  `docs_pack_manifest_record` pack + anchor kind from
  [`/docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md);
  surfaces deny render with `derived_explanation_uncited` if they
  can't resolve the anchor under a `citation_required` posture.
- **Command shareability, deep-link, and automation-safety
  contract.** The copy-id, deep-link, invocation-ref, CLI /
  headless equivalent, argument-inspection, macro / recipe /
  headless / UI-only / approval-required cue, and why-unavailable
  metadata every share / deep-link / CLI / AI-tool / automation-
  recipe / docs / About-panel surface reads is frozen in
  [`/docs/commands/shareability_and_automation_contract.md`](./shareability_and_automation_contract.md).
  The `disabled_reason_code` vocabulary this document owns is
  re-exported there; the shareability record extends the
  descriptor without widening it.

## Schema of record

The eventual command-registry and invocation-session crates' Rust
types are the schema of record. The JSON Schema export at
[`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
is the cross-tool boundary every non-owning surface reads. Adding
a new issuing surface, authority class, capability scope class,
argument kind, argument provenance value, UI slot class,
palette-visibility class, AI-tool surfacing class, preview class,
approval posture, enablement decision class, disabled-reason code,
execution-intent class, outcome class, result-contract class, or
evidence-ref class is additive-minor and bumps
`command_descriptor_schema_version`; repurposing an existing value
is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014.

## Source anchors

- [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — lifecycle, support class, release channel, freshness, client
  scope, redaction, and repair-hook vocabularies re-exported here.
- [`docs/ux/shell_interaction_safety_contract.md`](../ux/shell_interaction_safety_contract.md)
  — authority class, preview / apply / revert phase, permission
  prompt, copy / export representation, focus return, responsive
  fallback, and high-risk preview neighbourhood the command
  preview classes plug into.
- [`docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — docs / help source-of-truth anchor-kind vocabulary the
  command `docs_help_anchor_ref` re-exports.
- [`docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md)
  — the docs-pack manifest a command's `docs_help_anchor_ref`
  resolves into.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md),
  [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md),
  [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md),
  [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — command governance, palette / menu / keybinding / CLI / AI-
  tool surface expectations, and the preview / approval / apply /
  revert posture the command descriptors plug into.
