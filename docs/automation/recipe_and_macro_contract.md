# Recorded-macro, declarative-recipe, and run-record contract

This document freezes the boundary between two forms of user-authored
automation Aureline publishes — **recorded macros** and **declarative
recipes** — and the **run record** every attempted dispatch of either
writes. It is the authority that separates:

- a *recorded macro*, strictly constrained to explicit UI or editor
  state with no ambient network, process, or secret access;
- a *declarative recipe*, an ordered list of typed, gated steps that
  cite command descriptors and carry capability, trust, policy,
  approval, dry-run, and preview posture up front;
- a *run record*, the single evidence row every attempted dispatch of
  either manifest mints, carrying execution lineage, approvals,
  environment capsule refs, outputs, replay notes, queueability /
  reconciliation state, and export / support visibility.

The central rule is **declarative-first**. Aureline does not publish a
general-purpose scripting layer; recorded macros are a deliberately
narrow surface for reproducing UI or editor state, and declarative
recipes are the only form that may invoke command descriptors. Both
forms reuse the command descriptor, shareability, CLI / headless,
trust, policy, kill-switch, approval, and preview contracts already
frozen; neither invents a parallel notion of "recipe" or "macro".

Companion artifacts:

- [`/schemas/automation/recipe_manifest.schema.json`](../../schemas/automation/recipe_manifest.schema.json)
  — boundary schema for `recipe_manifest_record` and
  `recorded_macro_manifest_record`. Every stored recipe or macro
  publishes exactly one row against this schema.
- [`/schemas/automation/run_record.schema.json`](../../schemas/automation/run_record.schema.json)
  — boundary schema for `recipe_run_record` and `macro_replay_record`.
  Every attempted dispatch publishes exactly one row against this
  schema, whether it succeeds, is denied at a gate, is queued for a
  later window, replays stale authority (denied), or errors.
- [`/artifacts/automation/automation_capability_rows.yaml`](../../artifacts/automation/automation_capability_rows.yaml)
  — seeded capability-class vocabulary rows. Every capability a
  recipe or macro declares resolves to one row here; minting a
  parallel capability is non-conforming.
- [`/fixtures/automation/recipe_cases/`](../../fixtures/automation/recipe_cases)
  — worked example cases for the five shapes the contract freezes:
  a UI-state recorded macro, a local-scope declarative recipe, a
  workspace-scope declarative recipe with an approval posture, an
  organization-scope managed-only template, and a deferred-intent /
  queued-action recipe with a bounded replay window.
- [`/fixtures/automation/run_record_cases/`](../../fixtures/automation/run_record_cases)
  — worked example run records for the matching shapes.

Cross-linked contracts already in the repository:

- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  and
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
  — every declarative recipe step cites a `command_id` and a
  `command_revision_ref` resolvable against this descriptor. The
  capability scope, authority class, preview posture, approval
  posture, and dry-run posture fields a recipe projects originate
  there and are never re-decided here.
- [`/docs/commands/shareability_and_automation_contract.md`](../commands/shareability_and_automation_contract.md)
  and
  [`/schemas/commands/shareability_metadata.schema.json`](../../schemas/commands/shareability_metadata.schema.json)
  — the `macro_safe`, `recipe_safe`, `headless_safe`, `ui_only_interactive`,
  `approval_required`, and `irreversible_high_blast_denied_for_automation`
  cues, the `argument_inspection_kind` vocabulary, the why-unavailable
  reasons, and the gating-contract block originate there. A recipe
  whose step cites a command whose shareability record pins
  `ui_only_interactive` or `irreversible_high_blast_denied_for_automation`
  is non-conforming and denies at load.
- [`/docs/automation/cli_surface_contract.md`](./cli_surface_contract.md)
  and
  [`/schemas/automation/cli_output_registry_entry.schema.json`](../../schemas/automation/cli_output_registry_entry.schema.json)
  — the CLI / headless output registry every recipe step that
  dispatches through CLI projects through. The `exit_code_class` and
  `machine_output_stability_class` vocabularies originate there;
  the run record's per-step `cli_exit_code_class_ref` carries the
  opaque ref without re-deciding the class.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  and
  [`/artifacts/security/trust_state_matrix.yaml`](../../artifacts/security/trust_state_matrix.yaml)
  — the workspace-trust and restricted-mode contract every recipe
  projects through via its `trust_gate_class` slot.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
  — the admin-policy contract every recipe projects through via its
  `policy_gate_class` slot. Organization-scope managed-only templates
  ride the managed-only channel defined there.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — the kill-switch contract every recipe projects through via the
  `deferred_until_kill_switch_cleared` deferred-intent class and the
  `denied_at_kill_switch` run-outcome class.
- [`/docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)
  — the execution-context capsule every run cites; a run that
  dispatched without a capsule is non-conforming.
- [`/docs/ai/evidence_replayability_contract.md`](../ai/evidence_replayability_contract.md)
  — the graded replayability axis the run record's
  `replay_posture_class` projects through. Automation runs are graded
  on the same axis as AI turns; re-inventing a per-surface replay
  grade is non-conforming.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` — power-user automation requirements,
  MUST-NOT rules on ambient network / process / secret access from
  recorded macros, declarative-first posture, and queued-action
  staleness rules.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  recipe / macro architecture, run-record lineage, deferred-intent
  and queued-action reconciliation model.
- `.t2/docs/Aureline_Technical_Design_Document.md` — manifest
  content-addressing, capability declaration model, and
  reconciliation-intent vocabulary.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — preview, approval, and
  dry-run UX posture; recorded-macro authoring and replay UX.

## Why freeze this now

Without this contract, user-authored automation would drift along the
failure modes every IDE-class product hits sooner or later:

- **Unrestricted scripting.** A "recipe" would quietly become an
  embedded shell / JavaScript / Python runtime whose scope is "whatever
  the host process can touch". The contract pins that recipes are
  **declarative** — a YAML or JSON list of typed, gated steps that
  cite command descriptors — and that recorded macros are **strictly
  constrained to UI or editor state**. Neither is a scripting surface.
- **Ambient network, process, or secret access.** A macro recorder
  naively re-playing "what the user did" would quietly re-play a
  credential, a network POST, or a filesystem write alongside the
  keystrokes. The contract's `recorded_macro_surface_class` vocabulary
  is a closed set of UI / editor state axes; the record's
  `capability_declarations` is locked to `recorded_macro_ui_state_replay`,
  `recorded_macro_editor_state_replay`, and
  `read_only_workspace_inspection`. Ambient capabilities are denied
  mechanically at manifest load.
- **Capability laundering.** A recipe whose steps quietly touched
  irreversible mutation, credential brokers, or managed-only channels
  would bypass the command descriptor's authority and preview rules by
  composing them. The contract pins that the manifest's
  `capability_declarations` is the **whole union** of every step's
  capabilities; a step reaching a capability not declared in the
  manifest denies with `denied_capability_not_declared_in_manifest`.
- **Silent stale replay.** A queued action waiting on connectivity
  would quietly replay hours or days later under stale authority. The
  contract's `queueable_with_replay_window_bound` class pins a
  bounded replay window, and the run record's
  `replay_window_expired_stale_authority_denied` outcome and
  `queued_replay_window_expired_denied` state force the replay to
  fail fast when the window closes.
- **Evidence laundering.** A recipe run that "just happened" would
  leave no trail that survived to a support bundle or an organization
  audit log. The run-record contract binds every step to an
  `invocation_session_packet_ref`, every run to an
  `execution_context_capsule_ref`, and every mutation to an
  effect-lineage ref; denied / aborted / cancelled / skipped / queued
  steps are forbidden from citing effect-lineage refs (no effect
  could have been observed). The `export_visibility_class` and
  `support_visibility_class` axes are independent so a run that is
  local-only for the user is still honestly visible in a redacted
  support bundle when the user opts in.

It also freezes the **command-graph / trust / CLI / offline reconcile
parity rule**: a palette row, an AI-tool handle, a CLI verb, a recipe
step, and an offline reconcile queue entry that all "do the same
thing" MUST share a `command_id` and an `authority_class`. The
`capability_declarations` vocabulary seeded in
`automation_capability_rows.yaml` is the single axis those surfaces
compare through.

## Scope

Frozen at this revision:

- One `recipe_manifest_record` shape for declarative recipes and one
  `recorded_macro_manifest_record` shape for recorded macros, with
  closed vocabularies for authoring posture, authoring language,
  storage scope, capability declarations, trust gate, policy gate,
  approval posture, dry-run posture, preview posture, signature
  class, export posture, redaction class, step kind, recorded-macro
  surface class, deferred intent, queueable action, reconciliation
  intent, and the schema-level `allOf` couplings that enforce the
  must-rules mechanically.
- One `recipe_run_record` shape and one `macro_replay_record` shape
  with closed vocabularies for run outcome, step outcome, gate denial
  reason, replay posture, queueability state, reconciliation state,
  export visibility, and support visibility, and schema-level `allOf`
  couplings that enforce the must-rules mechanically.
- A seed row set in `automation_capability_rows.yaml` binding each
  capability class to a trust ceiling, a policy ceiling, a dry-run
  posture floor, a preview posture floor, an approval posture floor,
  a macro admissibility flag, and a deferred-intent admissibility
  flag.
- Worked fixture cases covering the five shapes named above for
  recipes and macros, and the matching run records.

Out of scope until a superseding decision row opens:

- The live macro recorder and the live recipe runner. This contract
  names the schema refs and fixture refs the runner will consume; the
  runner lands in its own lane after foundations.
- A "power-user scripting" surface beyond declarative recipes. A
  future user-authored scripting surface, if any, is a superseding
  decision — not a back-door through the recipe step kind set.
- Full breadth of command descriptors reachable from recipes. The seed
  binds the shapes; additional descriptors become reachable as they
  come online, one shareability record and one descriptor revision at
  a time.
- The offline-reconcile queue format beyond `queued_replay_window_bound_remaining`
  and `queued_replay_window_expired_denied`. The reconcile queue's
  wire format is its own boundary; this contract fixes the
  manifest-side and run-record-side states it projects through.

## The two authoring shapes

The first cut this contract makes:

| Axis                                  | Recorded macro                                             | Declarative recipe                                                     |
|---------------------------------------|------------------------------------------------------------|------------------------------------------------------------------------|
| Primary authoring act                 | Recording explicit UI or editor state                      | Authoring a YAML / JSON list of typed, gated steps                    |
| What a step may touch                 | One UI or editor surface class                             | One command descriptor (via `command_id` + revision)                   |
| Ambient network, process, secret      | **Forbidden** — closed capability subset, closed surface set | Declared up front via `capability_declarations`                        |
| Approval / preview / dry-run          | No approval / dry-run path; preview is the replay preview  | Inherited from the command descriptor per step                        |
| Admissible storage                    | User / workspace / portable profile export / support bundle | User / workspace / organization-managed / portable profile / support |
| Admissible on managed-only channel    | **No**                                                     | Yes, with `managed_only_template_declarative_recipe` posture          |
| Signable / verifiable                 | Optionally `author_signature`                              | Optionally `author_signature`, `organization_signature_only`, `managed_only_channel_signature`, or `author_and_organization_signature` |
| Automation safety cue                 | `macro_safe` (UI / editor only)                            | `recipe_safe`, optionally `approval_required` / `preview_required_before_apply` / `managed_only` |

Both shapes publish an opaque `macro_id` / `recipe_id`, an opaque
`*_revision_ref`, a content-address of the manifest bytes, a `title`,
a `summary`, a `policy_context` (trust state + policy epoch), a
`redaction_class`, a `signature_class`, and a `minted_at` timestamp;
they share the schema file (`recipe_manifest.schema.json`) so a
single boundary validator covers both.

## Capability declarations as the control axis

Every step in a recipe and every step in a macro quotes one or more
`automation_capability_class` values. The closed vocabulary is the
single axis the command graph, the trust contract, the policy
contract, the CLI / headless surface, the AI-tool descriptor, and the
support export read when deciding whether to enable, deny, require
approval, or require preview. The seeded rows in
`automation_capability_rows.yaml` bind each capability to:

- a **trust ceiling** (is this capability admissible on a restricted
  workspace, a trusted workspace, or only after a trust revalidation?);
- a **policy ceiling** (is this capability admissible under admin
  policy observation, or only on the managed-only channel?);
- a **dry-run floor** (does this capability MUST have a dry-run path
  before apply, or is it read-only?);
- a **preview floor** (does this capability MUST have a preview path
  before apply?);
- an **approval floor** (does this capability MUST have an approval
  path?);
- a **macro admissibility flag** (may a recorded macro declare this
  capability at all?);
- a **deferred-intent admissibility flag** (may a step declaring
  this capability ride a deferred-intent or queued-action hook?).

The manifest's top-level `capability_declarations` is the **whole
union** of every step's capabilities. A step whose capability is not
present in the top-level set is non-conforming; a run-record step
outcome `denied_capability_not_declared_in_manifest` is the mechanical
enforcement of that rule at dispatch time.

## Deferred-intent and queued-action hooks

Some power-user flows naturally want to run *later*: "apply this
organization-policy change once connectivity is restored", "dispatch
this remote-agent workflow once the credential broker completes its
handshake", "run this managed-only template once the channel opens
after business-hours kill-switch clears". The contract admits this
through two hook step kinds:

- `emit_deferred_intent_hook` — the step declares the window it MUST
  wait on via `deferred_intent_class`. Silent "try-then-fail-then-retry"
  loops are non-conforming; the step MUST declare the waiting window
  up front.
- `emit_queued_action_hook` — the step declares how its effect is safe
  to queue for later replay via `queueable_action_class`. The
  `queueable_with_replay_window_bound` class carries a bounded replay
  window; when that window expires, the run record transitions to
  `replay_window_expired_stale_authority_denied` / `queued_replay_window_expired_denied`
  rather than replaying hours or days later under stale authority.

A queued step that needs to re-check reality before replay declares a
`reconciliation_intent_class` other than `reconciliation_not_applicable`.
The `abort_on_reconciliation_drift` class forbids any silent
resolution: drift denies.

## Recorded macros: the narrow surface

Recorded macros are the deliberately narrow surface. They are:

- **Strictly constrained to UI or editor state.** Every macro step's
  `surface_class` is drawn from a closed set —
  `editor_document_state`, `editor_selection_and_cursor_state`,
  `editor_multi_cursor_edits`, `editor_find_and_replace_state`,
  `command_palette_selection_state`, `ui_panel_open_close_state`,
  `ui_focus_move_state`, `ui_layout_reshape_state`,
  `keybinding_chord_replay_state`. Minting a parallel surface
  (`shell_command`, `network_fetch`, `credential_read`) is
  non-conforming.
- **Forbidden from ambient capability.** The record's
  `capability_declarations` is locked at schema level to the
  closed subset `{recorded_macro_ui_state_replay, recorded_macro_editor_state_replay, read_only_workspace_inspection}`.
  A recorded macro cannot declare network, process, filesystem
  mutation, credential-broker, remote-agent, managed-only-channel,
  AI-tool, extension, terminal, debug, notebook-kernel, settings
  mutation, or policy / trust revalidation capabilities.
- **Content-addressed for replay integrity.** Every macro step carries
  a `state_digest` content-address of the captured UI / editor state.
  The macro replayer MUST refuse to drift from the captured state
  without an explicit reconciliation step; raw DOM / raw buffer bytes
  never cross this boundary.
- **Not admissible on the managed-only channel.** A recorded macro's
  `storage_scope_class` is constrained to `user_scope_local_only`,
  `workspace_scope_local_only`, `portable_profile_export_only`, or
  `support_bundle_export_only`. An organization that wants to
  distribute a macro re-authors it as a declarative recipe via
  `recorded_macro_promoted_to_declarative_recipe`; the promotion is
  recorded in the recipe's `promoted_from_macro_id` slot.

Because recorded macros never involve approval, credential broker, or
managed-only-channel gates, the matching `macro_replay_record` shape
pins an admissible `run_outcome_class` subset that excludes those
outcomes at schema level.

## Run records: the single evidence row

Every attempted dispatch of a manifest publishes exactly one
`recipe_run_record` or `macro_replay_record`. The record carries:

- the opaque `run_id`, the manifest id and revision the run dispatched
  against, and optionally the manifest's `content_address` pin so
  support export can prove the exact manifest bytes the run rode;
- the `invocation_session_root_packet_ref` and per-step
  `invocation_session_packet_ref` so the run's full invocation lineage
  is reconstructable through the descriptor contract's session
  binding;
- the `execution_context_capsule_ref` (ADR-0009) bounding the run, and
  the `environment_capsule_ref` capturing the env vars, trust state,
  policy epoch, and kill-switch state the run observed (null only when
  the run is `non_replayable_raw_byte_dependent`);
- the `approvals_applied` list — one `approval_ticket_ref_entry` per
  approval ticket consumed, each with an `approval_posture_class`,
  `approver_identity_ref`, `granted_at`, and optional `expires_at`;
- per-step `effect_lineage_refs` and `output_artifact_refs` — raw
  paths / raw bytes never appear; denied / aborted / cancelled /
  skipped / queued steps are forbidden from citing effect-lineage
  refs (no effect could have been observed);
- the `replay_posture_class` — graded on the same axis as the AI
  evidence-replayability contract, so automation and AI surfaces
  converge on one grade;
- the `queueability_state_class`, `reconciliation_state_class`, and
  `replay_window_expires_at` when the run rode a deferred-intent or
  queued-action hook;
- the `export_visibility_class` and `support_visibility_class` —
  independent axes so a user's local-only run is still honestly
  visible in a redacted support bundle when the user opts in.

## Registry-level invariants

The schemas' invariants blocks pin the following constants to `true`.
A seed that sets any of them to `false` is non-conforming; the block
is how this contract freezes its MUST rules mechanically.

Recipe-manifest invariants
(`recipe_manifest.schema.json#/$defs/recipe_manifest_invariants_block`):

1. `recorded_macros_cannot_declare_network_process_or_secret_capabilities`
2. `every_recipe_step_cites_descriptor_command_id_and_revision`
3. `no_raw_shell_fragments_in_recipe_steps`
4. `no_raw_filesystem_paths_or_urls_in_recipe_steps`
5. `no_silent_forward_from_recorded_macro_to_declarative_recipe`
6. `deferred_intent_steps_declare_the_waiting_window_up_front`
7. `queueable_steps_declare_the_reconciliation_intent_up_front`
8. `managed_only_templates_ride_the_managed_only_channel`
9. `manifest_bytes_carry_a_content_address`
10. `capability_declarations_are_the_whole_union_of_per_step_capabilities`

Run-record invariants
(`run_record.schema.json#/$defs/run_record_invariants_block`):

1. `every_run_cites_manifest_id_and_revision`
2. `every_step_cites_invocation_session_packet_ref`
3. `every_run_carries_execution_context_capsule_ref`
4. `every_denied_or_aborted_run_cites_typed_gate_reason`
5. `queued_runs_never_silently_replay_stale_authority`
6. `denied_or_aborted_steps_cite_no_effect_lineage`
7. `no_raw_payload_bytes_in_run_record`
8. `macro_runs_are_limited_to_ui_or_editor_state_outcomes`
9. `export_and_support_visibility_are_distinct_axes`

## Schema of record

The eventual Aureline automation crates' Rust types are the schema of
record. The JSON Schema exports at
[`/schemas/automation/recipe_manifest.schema.json`](../../schemas/automation/recipe_manifest.schema.json)
and
[`/schemas/automation/run_record.schema.json`](../../schemas/automation/run_record.schema.json)
are the cross-tool boundary every non-owning surface reads. Adding a
new enum value to any frozen vocabulary is additive-minor and bumps
the relevant `_schema_version` const; repurposing an existing value is
breaking and requires a new decision row.

## Source anchors

- [`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
- [`docs/commands/shareability_and_automation_contract.md`](../commands/shareability_and_automation_contract.md)
- [`docs/commands/command_parity_diff.md`](../commands/command_parity_diff.md)
- [`docs/automation/cli_surface_contract.md`](./cli_surface_contract.md)
- [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
- [`docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)
- [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
- [`docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
- [`docs/ai/evidence_replayability_contract.md`](../ai/evidence_replayability_contract.md)
- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md),
  [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md),
  [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md),
  [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — declarative-first automation, recorded-macro narrowing rules,
  deferred-intent / queued-action staleness rules, and run-record
  lineage requirements.
