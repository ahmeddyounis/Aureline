# Command shareability, deep-link, and automation-safety contract

This document freezes the shareability, deep-link, invocation-ref,
CLI / headless equivalent, argument-inspection, automation-safety
cue, and why-unavailable metadata every command palette,
application and context menu, toolbar button, keybinding editor,
keybinding help layer, CLI help output, AI-tool surface,
automation-recipe response, docs reference page, and About /
service-health panel reads before exposing a copyable command id,
deep link, invocation ref, CLI equivalent, or typed unavailability
explanation. It extends the command contract beyond stable command
ids so users and tools can share, inspect, and automate commands
without minting per-surface dialects of "what does 'safe to
automate' mean?", "how do I link to this command?", or "why is
this command unavailable in this context?".

The machine-readable boundary is
[`/schemas/commands/shareability_metadata.schema.json`](../../schemas/commands/shareability_metadata.schema.json);
worked examples (core first-party command, extension command,
policy-blocked command, UI-only interactive command, approval-
gated remote / network command) live in
[`/fixtures/commands/shareability_cases/`](../../fixtures/commands/shareability_cases/).

Every shareability record extends a command descriptor frozen in
[`/docs/commands/command_descriptor_contract.md`](./command_descriptor_contract.md).
The descriptor is the governed product object; this record is the
cross-tool shareability projection the descriptor owner publishes
alongside it. If this document and a neighbouring ADR disagree,
the ADR wins and this document MUST be updated in the same change.

## Why freeze this now

Without a single shareability shape, every surface would mint its
own dialect for:

- "Copy command id" vs. "Copy canonical verb" vs. "Copy palette
  search token" — users would not know which token they are
  sharing.
- Deep links — a pasted deep link could resolve directly into a
  command dispatch that bypasses workspace trust, admin policy,
  the permission prompt, preview, approval, credential broker,
  managed channel, execution context, client scope, publisher
  admission, kill-switch, or freshness-floor gates.
- CLI and headless paths — each tool would invent its own
  "`--help`" surface, its own argv shape, and its own "this
  command is not available in headless mode" phrasing.
- Automation safety — a macro author, a recipe author, and an AI
  tool author would each guess whether a command is safe to
  automate.
- Why-unavailable reasons — the palette would say "Unavailable",
  the context menu would say "Disabled", the CLI would say
  "Error", the docs would say "Not on this plan", and the
  automation response would say `{"status": "denied"}` — none of
  them machine-checkable against each other.

This contract is the missing piece that lets, at freeze time and
before any runtime:

- every surface expose a closed set of copy affordances backed by
  one token vocabulary;
- every deep link resolve into a named surface state rather than
  dispatching a command; the resolver MUST re-run the trust,
  policy, permission, preview, approval, credential-broker,
  managed-channel, execution-context, client-scope, publisher,
  kill-switch, and freshness gates before any consequence;
- every CLI / headless / AI-tool / automation-recipe surface read
  one CLI-equivalence class, one argv shape, one headless-mode
  posture, one argument-inspection form, and one automation-
  safety cue set;
- every surface render the same typed why-unavailable reason for
  the same `command_id` / `command_revision_ref`, with the same
  repair hook, regardless of whether the user is looking at the
  palette, the menu, the toolbar, the keybinding editor, the
  keybinding help overlay, the CLI help output, the AI-tool
  descriptor, the automation-recipe response, the docs reference
  page, or the About / service-health panel.

## Scope

Frozen at this revision:

- One `shareability_metadata_record` shape with a closed set of
  copy forms, deep-link target kinds, deep-link gating
  requirements, CLI-equivalence classes, headless-mode classes,
  argument-inspection kinds, automation-safety cues,
  unavailability-parity surfaces, and why-unavailable reasons.
- A gating-contract block that pins, schema-level, the rule that
  deep links / invocation refs / CLI paths never bypass trust,
  policy, or permission gates and that preview and approval
  paths are preserved regardless of which shareability affordance
  initiated dispatch.
- Rules binding copy forms to their matching blocks
  (`copy_deep_link_ref` ↔ `deep_link` block,
  `copy_cli_invocation_skeleton` ↔ `cli_equivalent` block,
  `ui_only_interactive` ↔ absent `invocation_ref` +
  `ui_only_command_not_representable_as_invocation` reason).
- Projection rules binding one record to one vocabulary every
  command surface, docs surface, and automation response reads.

Out of scope until a superseding decision row opens:

- The live deep-link dispatcher, the live CLI router, the live
  macro / recipe runtime, and the runtime-generated parity-diff
  automation between surfaces' actual unavailability renders.
- The wire format of deep links (URI scheme, base64 encoding,
  signing). This contract names the opaque `deep_link_id`; the
  wire format is frozen by its own lane.
- The recipe / macro / batch-command authoring surfaces. They
  quote `invocation_ref_id` rather than minting their own
  command dialect.
- Full CLI `--help` rendering and AI-tool descriptor serialisation.

## Canonical ownership

Each command descriptor's shareability record has exactly one
canonical owner: the registry that owns the descriptor. Surfaces
that render a shareability projection MUST quote the owner's
`shareability_metadata_record` rather than mint a copy.

| Command family                              | Canonical owner                       |
|---------------------------------------------|---------------------------------------|
| Workspace / open / clone / import / restore | `workspace_command_registry`          |
| Editor / buffer / selection / refactor      | `editor_command_registry`             |
| Source control / review / PR                | `source_control_command_registry`     |
| Search / find / symbol-jump                 | `search_command_registry`             |
| Settings / policy / waiver authoring        | `settings_command_registry`           |
| AI apply / AI session / AI tool             | `ai_command_registry`                 |
| Extension / install / update                | `extension_command_registry`          |
| Support / export / diagnostic               | `support_command_registry`            |
| Managed workspace control                   | `managed_workspace_command_registry`  |

The registries listed above are projection targets; this document
does not land any of them. It pins the shape they MUST publish
alongside each descriptor.

## Record fields

The full field set lives in
[`/schemas/commands/shareability_metadata.schema.json`](../../schemas/commands/shareability_metadata.schema.json).
The notable fields are:

- **Identity.** `command_id`, `command_revision_ref`, and
  `canonical_verb` are re-exports of the matching descriptor
  fields. A shareability record whose identity does not resolve
  to a live descriptor is rejected by the registry.
- **Copy forms.** `copy_forms` is a non-empty closed set of
  `copy_form_slot` entries. Every record MUST publish at minimum
  `copy_canonical_command_id` and `copy_canonical_verb`;
  `copy_deep_link_ref`, `copy_cli_invocation_skeleton`,
  `copy_ai_tool_handle`, `copy_palette_search_token`,
  `copy_shortcut_chord_narration`, and `copy_docs_help_anchor`
  are optional and pair with the matching blocks
  (`deep_link` / `cli_equivalent` / `invocation_ref`).
- **Deep-link block.** `deep_link` is present when `copy_forms`
  contains `copy_deep_link_ref`. The block pins the opaque
  `deep_link_id`, the `deep_link_target_kind` the resolver
  resolves into, the non-empty closed set of gating requirements
  the resolver MUST re-run, schema-level `revalidates_*` constants
  (workspace trust / policy admission / permission prompt), the
  argument-prefill refs the preview pane may seed, and an optional
  scope anchor ref. Raw URLs / URI strings never appear.
- **Invocation-ref block.** `invocation_ref` is present when the
  command is automation-callable (`macro_safe` / `recipe_safe` /
  `headless_safe` / AI-callable). The block pins the opaque
  `invocation_ref_id`, the canonical verb, the descriptor revision
  ref the invocation ref was minted against, and the argument
  provenance template a dispatcher resolves. A dispatch whose
  descriptor revision differs from `command_revision_ref` denies
  with `invocation_ref_revision_mismatch` rather than silently
  upgrading. UI-only interactive commands omit this block.
- **CLI-equivalent block.** `cli_equivalent` is always present.
  The block pins the `cli_equivalent_presence_class`, the CLI
  verb ref (null when the command has no CLI path), the alternate
  CLI verb alias refs from the descriptor, the typed argv shape
  refs, the docs-pack anchor the CLI `--help` output cites, and
  the headless-mode block.
- **Headless-mode block.** `cli_equivalent.headless_mode` pins
  the `headless_mode_class`, the prompts the headless path elides
  (required when `headless_safe_with_declared_prompt_elision`),
  and the non-interactive pathways the headless path uses.
  `denied_in_headless_mode` forces the `ui_only_interactive`
  automation-safety cue.
- **Argument-inspection block.** `argument_inspection` is always
  present. `inspectable_arguments` MUST mirror the descriptor's
  `typed_arguments` order 1:1; each entry quotes an
  `argument_inspection_kind` so palette / keybinding editor /
  CLI `--describe` / AI-tool `--describe` / automation-recipe
  response / docs reference page surfaces explain "where does
  this value come from?" consistently.
- **Automation-safety cues.** `automation_safety_cues` is a
  non-empty closed set drawn from the frozen cue vocabulary
  (`macro_safe`, `recipe_safe`, `headless_safe`,
  `ui_only_interactive`, `approval_required`,
  `preview_required_before_apply`, `credential_or_secret_gated`,
  `never_ai_callable`, `managed_only`,
  `requires_trusted_workspace`, `policy_restricted`,
  `irreversible_high_blast_denied_for_automation`,
  `extension_invocation_only`, `remote_agent_invocation_only`).
  Every surface that renders a chip or cue MUST quote from this
  set; minting a parallel cue is non-conforming.
- **Unavailability-disclosure block.** `unavailability_disclosure`
  freezes the set of why-unavailable reasons and the set of
  parity surfaces that MUST render the same reason + repair hook
  for the same command. Parity audits consume this rule
  mechanically.
- **Gating-contract block.** `gating_contract` pins eight
  schema-level constants to `true`: deep-link bypass forbidden,
  invocation-ref bypass forbidden, CLI-path bypass forbidden,
  preview path preserved, approval path preserved, trust
  revalidation required, policy revalidation required, permission
  prompt revalidation required. The constants are the schema's
  way of freezing the rules mechanically; a `false` value is
  non-conforming.
- **Policy context and redaction.** `policy_context` and
  `redaction_class` are re-exported from ADR 0001 / ADR 0007 /
  ADR 0008 / ADR 0011 without modification.

## Copy-form vocabulary

`copy_form_class` is the closed vocabulary every "copy" affordance
across palette, application and context menus, toolbar buttons,
keybinding editor, CLI help, AI-tool surface, automation-recipe
response, docs reference page, and About / service-health panel
projects against:

| Copy form                                | Meaning                                                                                                                                             |
|------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| `copy_canonical_command_id`              | The stable opaque `command_id` the registry uses. Safe to paste into issue reports, support bundles, claim manifests.                                |
| `copy_canonical_verb`                    | The dotted snake_case `canonical_verb` (`workspace.open_folder`, `git.commit`, `ai.apply_patch`).                                                   |
| `copy_palette_search_token`              | An opaque token users paste into the palette's search input to re-find the command. Resolves through the palette's alias / phrasing map.             |
| `copy_cli_invocation_skeleton`           | A non-executable CLI skeleton (canonical verb plus argument slot narration). Re-resolved through enablement on paste; does not bypass gates.         |
| `copy_ai_tool_handle`                    | The `ai_tool_handle` alias (when declared) so AI agents can quote the handle in transcripts.                                                         |
| `copy_deep_link_ref`                     | The opaque `deep_link_id`. Resolves into a named surface state via the deep-link block; never bypasses gates.                                       |
| `copy_shortcut_chord_narration`          | The shortcut-narration hint the keybinding help layer reads. Safe in accessibility narration and docs.                                              |
| `copy_docs_help_anchor`                  | The docs-pack anchor the command descriptor pins. Safe in docs cross-links.                                                                          |

A surface that needs a copy affordance outside this set MUST open
a new decision row rather than mint a new copy class inline.

## Deep-link gating contract

Deep links are the cross-product of copy-and-share and recovery:
a paste of a deep-link id into the application shell resolves the
link through the deep-link resolver. **Deep links never bypass
trust, policy, or permission gates.** The schema freezes this
rule two ways:

1. The `deep_link_block.gating_requirements` list MUST be
   non-empty and drawn from the closed gating-requirement
   vocabulary.
2. The `deep_link_block.revalidates_workspace_trust`,
   `revalidates_policy_admission`, and
   `revalidates_permission_prompt` fields are schema-level
   constants pinned to `true`. A `false` value is rejected by the
   schema.
3. The `gating_contract_block.deep_link_bypass_forbidden`,
   `trust_revalidation_required`, `policy_revalidation_required`,
   `permission_prompt_revalidation_required`, `preview_path_preserved`,
   and `approval_path_preserved` fields are also schema-level
   constants pinned to `true`.

The admissible gating-requirement values are:

- `workspace_trust_revalidation_required` — ADR 0001 trust state
  is re-validated at resolution time.
- `policy_admission_revalidation_required` — ADR 0008 admin
  policy is re-validated.
- `permission_prompt_revalidation_required` — the interaction-
  safety permission prompt path re-runs.
- `preview_path_required` — a non-`no_preview_required` preview
  class forces the preview pane before apply.
- `approval_path_required` — a non-`no_approval_required` posture
  forces the approval path before apply.
- `credential_broker_handshake_required` — credential or
  secret-bearing commands re-run the ADR 0007 broker handshake.
- `managed_only_channel_required` — managed-only commands
  re-validate the running build's channel.
- `execution_context_resolution_required` — ADR 0009 execution
  context is resolved before any consequence.
- `client_scope_admission_required` — the command's
  `client_scopes` re-admits the current surface.
- `publisher_admission_required` — extension-provided commands
  re-admit the publisher (ADR 0012).
- `kill_switch_revalidation_required` — ADR 0011 kill-switch
  dependencies re-validate.
- `freshness_floor_revalidation_required` — the command's
  `declared_freshness_class` re-admits against the surface's
  floor.

Deep-link target kinds (`reveal_command_in_palette`,
`reveal_command_in_application_menu`,
`reveal_command_in_context_menu`,
`reveal_command_in_keybinding_editor`,
`open_command_docs_help_anchor`,
`prefill_command_invocation_preview`,
`open_command_about_service_health_panel`) resolve into named
surface states, not into dispatches. The
`prefill_command_invocation_preview` target seeds the preview pane
with argument refs; preview, approval, trust, and policy gates
still run before apply.

## CLI-equivalence and headless-mode vocabulary

`cli_equivalent_presence_class` is the closed vocabulary every
CLI help, headless invocation, recipe runner, and AI-tool
surface reads when answering "does this command have a CLI path?":

| Presence class                                   | Meaning                                                                                                                                 |
|--------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| `has_cli_equivalent_on_every_permitted_scope`    | Floor for commands whose `client_scopes` contain `cli`; the CLI verb is admissible on every surface the descriptor permits.              |
| `has_cli_equivalent_on_restricted_scopes_only`   | CLI path exists but is restricted (e.g. managed-only, remote-agent only). Surfaces outside the scope render the typed why-unavailable. |
| `cli_equivalent_requires_interactive_prompt`     | CLI path exists but some arguments cannot resolve headlessly without an interactive prompt.                                             |
| `cli_equivalent_denied_by_policy`                | Admin policy blocks the CLI path in the current context.                                                                                |
| `cli_equivalent_denied_by_client_scope`          | The command's `client_scopes` excludes `cli`.                                                                                            |
| `no_cli_equivalent_ui_only`                      | UI-only interactive command (welcome tour, onboarding takeover, in-pane animation). No headless verb is possible.                       |

`headless_mode_class` is the closed vocabulary every headless
invocation surface reads:

| Headless class                                      | Meaning                                                                                                                       |
|-----------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| `fully_headless_safe`                               | All prompts have non-interactive pathways; the command applies without interactive prompts.                                    |
| `headless_safe_with_declared_prompt_elision`        | Some prompts are elided; the elided prompts MUST appear on the invocation-session packet's warnings summary.                   |
| `requires_interactive_context`                      | At least one argument cannot resolve without an interactive prompt.                                                            |
| `denied_in_headless_mode`                           | Command cannot run headlessly; forces the `ui_only_interactive` automation-safety cue.                                         |

## Argument-inspection vocabulary

`argument_inspection_kind` is the closed vocabulary every
"what arguments does this command take?" affordance reads. Every
entry in `inspectable_arguments` MUST mirror the descriptor's
`typed_arguments` order 1:1 and quote one of:

- `typed_argument_slot_ref` — plain typed argument slot.
- `policy_pinned_argument_ref` — argument pinned by admin policy
  or by trust state; the inspection surface MUST render the
  "policy pinned" chip.
- `credential_handle_argument_ref` — ADR 0007 broker handle;
  never inspected in raw form.
- `selection_backed_argument_ref` — resolves from the current
  selection.
- `focused_context_backed_argument_ref` — resolves from the
  focused entity.
- `default_from_descriptor_argument_ref` — default from the
  descriptor.
- `ai_proposed_argument_ref` — AI-proposed; MUST surface
  provenance on the primary preview.
- `automation_recipe_supplied_argument_ref` — supplied by the
  recipe definition.
- `extension_supplied_argument_ref` — supplied by an extension.

## Automation-safety cue vocabulary

`automation_safety_cue` is the closed vocabulary every macro /
recipe / headless chip, every AI-tool descriptor, every
automation-recipe response, every docs reference page, and every
About / service-health panel reads. The admissible cues are:

- `macro_safe` — safe to include in a user-authored macro.
- `recipe_safe` — safe to include in an automation recipe
  (broader than `macro_safe`; implies serialisable context).
- `headless_safe` — safe on headless surfaces.
- `ui_only_interactive` — no automation path; forces an absent
  `invocation_ref` and a
  `ui_only_command_not_representable_as_invocation` reason.
- `approval_required` — any automation path MUST ride the
  approval gate.
- `preview_required_before_apply` — preview is mandatory.
- `credential_or_secret_gated` — requires an ADR 0007 broker
  handshake.
- `never_ai_callable` — AI tools MUST NOT expose this command.
- `managed_only` — admissible only on managed workspace surfaces.
- `requires_trusted_workspace` — restricted-trust workspaces deny
  automation.
- `policy_restricted` — admin policy narrows automation paths.
- `irreversible_high_blast_denied_for_automation` — irreversible /
  high-blast commands ride human-in-the-loop paths.
- `extension_invocation_only` — only invokable by the declaring
  extension.
- `remote_agent_invocation_only` — only invokable from a remote
  agent surface.

## Why-unavailable vocabulary and parity rule

`shareability_why_unavailable_reason` re-exports the
twenty-four-value `disabled_reason_code` vocabulary from
[`docs/commands/command_descriptor_contract.md`](./command_descriptor_contract.md#disabled-reason-vocabulary)
without modification, and adds nine shareability-specific
reasons:

- `cli_surface_not_in_client_scopes` — descriptor's `client_scopes`
  excludes `cli`.
- `automation_surface_not_in_client_scopes` — excludes
  automation.
- `ai_tool_surface_not_in_client_scopes` — excludes
  `ai_tool_surface`.
- `deep_link_target_kind_unsupported` — the resolver cannot
  resolve the declared target kind.
- `deep_link_gating_requirement_failed` — one of the declared
  gating requirements failed on resolution.
- `invocation_ref_revision_mismatch` — the dispatch's descriptor
  revision differs from the invocation ref's minted revision.
- `alias_not_resolvable` — the quoted alias does not resolve to a
  live descriptor.
- `ui_only_command_not_representable_as_invocation` — forced by
  the `ui_only_interactive` automation-safety cue.
- `argument_inspection_requires_interactive_prompt` — the
  inspection surface cannot render without an interactive prompt.

**Parity rule.** `unavailability_disclosure.parity_surfaces` is a
non-empty closed set drawn from the
`unavailability_parity_surface` vocabulary. Every surface listed
there MUST render the same `(reason_code, repair_hook_ref)` tuple
for the same `command_id` / `command_revision_ref`. A parity
audit between two surfaces' emitted rows MUST match field-for-
field. Silent divergence is non-conforming;
[`docs/commands/command_parity_diff.md`](./command_parity_diff.md)
consumes this rule mechanically.

The admissible parity surfaces are `command_palette`,
`global_application_menu`, `context_menu`, `toolbar_button`,
`keybinding_help`, `keybinding_editor`, `cli_help`,
`ai_tool_surface`, `automation_recipe_surface`,
`docs_reference_page`, and `about_service_health_panel`. A
shareability record's parity-surface set is the owner's commitment
that the typed reason renders identically on every listed
surface; a surface not listed is free to render a generic
unavailability chip, but the chip still quotes the typed reason
from the record rather than minting a parallel string.

## Relationship to the descriptor

The shareability record extends the command descriptor without
competing with it:

- The descriptor defines `command_id`, `command_revision_ref`,
  `canonical_verb`, `docs_help_anchor_ref`,
  `accessibility_label_path`, `typed_arguments`, `client_scopes`,
  `capability_scope_class`, `preview_class`,
  `approval_posture_class`, and `ai_tool_surfacing_class`.
- The shareability record re-exports the subset every copy / share /
  inspect / automate surface needs and adds the copy-form, deep-
  link-target, CLI-equivalence-presence, headless-mode,
  argument-inspection-kind, automation-safety-cue, unavailability-
  parity-surface, and shareability-specific why-unavailable
  vocabulary those surfaces project against.

A shareability record MUST NOT widen the descriptor. If the
descriptor declares `ai_tool_surfacing_class =
ai_callable_irreversible_high_blast_denied`, the shareability
record MUST publish `never_ai_callable` in
`automation_safety_cues` and MUST NOT publish a
`copy_ai_tool_handle` copy form. If the descriptor's
`client_scopes` excludes `cli`, the shareability record's
`cli_equivalent.presence_class` MUST be
`cli_equivalent_denied_by_client_scope` or
`no_cli_equivalent_ui_only`. Silent widening is non-conforming.

## Linkage to neighbouring contracts

- [`docs/commands/command_descriptor_contract.md`](./command_descriptor_contract.md)
  — the command descriptor this record extends; the
  `disabled_reason_code` vocabulary and the
  `ai_tool_surfacing_class` / `preview_class` /
  `approval_posture_class` / `client_scopes` re-exports originate
  here.
- [`docs/commands/command_parity_diff.md`](./command_parity_diff.md)
  — consumes `unavailability_disclosure.parity_surfaces` and
  `automation_safety_cues` mechanically.
- [`docs/ux/shell_interaction_safety_contract.md`](../ux/shell_interaction_safety_contract.md)
  — permission-prompt and preview-path contracts every deep-link
  resolver re-runs.
- [`docs/ux/clipboard_history_contract.md`](../ux/clipboard_history_contract.md)
  — clipboard redaction pass every `copy_token_ref` rides before
  bytes reach the clipboard sink.
- [`docs/ux/navigation_and_escalation_contract.md`](../ux/navigation_and_escalation_contract.md)
  — deep-link target kinds resolve into named navigation states
  owned by this contract.
- [`docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md)
  — docs anchors every `copy_docs_help_anchor` and
  `cli_equivalent.help_anchor_ref` resolves into.
- [`docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md)
  — trust state every deep-link resolver re-validates.
- [`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — credential broker handshake every `credential_or_secret_gated`
  cue requires.
- [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
  — admin policy every deep-link resolver re-validates.
- [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — lifecycle / freshness / repair-hook vocabulary re-exported
  here.
- [`docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md)
  — publisher admission every extension-supplied command
  re-validates.

## Schema of record

The eventual command-registry crates' Rust types are the schema
of record. The JSON Schema export at
[`/schemas/commands/shareability_metadata.schema.json`](../../schemas/commands/shareability_metadata.schema.json)
is the cross-tool boundary every non-owning surface reads. Adding
a new copy form, deep-link target kind, gating requirement,
CLI-equivalence class, headless-mode class, argument-inspection
kind, automation-safety cue, unavailability-parity surface, or
why-unavailable reason is additive-minor and bumps
`shareability_metadata_schema_version`; repurposing an existing
value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014.

## Source anchors

- [`docs/commands/command_descriptor_contract.md`](./command_descriptor_contract.md)
  — the descriptor this record extends.
- [`docs/commands/command_graph_and_ui_slots_seed.md`](./command_graph_and_ui_slots_seed.md)
  — UI slot taxonomy the `unavailability_parity_surface`
  vocabulary projects against.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md),
  [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md),
  [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md),
  [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — command governance, shareability, deep-link, CLI / headless,
  AI-tool surface, and automation posture the shareability
  records plug into.
