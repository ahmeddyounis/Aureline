# Automation run-history row, rerun-under-current-policy, and safe-summary export contract

This document freezes the boundary between the **run record** every
attempted dispatch of a recorded macro, declarative recipe, managed-only
template, extension/external automation, or headless-safe run already
publishes (per
[`docs/automation/recipe_and_macro_contract.md`](./recipe_and_macro_contract.md)
and
[`schemas/automation/run_record.schema.json`](../../schemas/automation/run_record.schema.json))
and the **user-facing run-history row** desktop, headless, AI assistant,
support, and organization-audit surfaces render that record into. The
core problem this contract solves: history must stay useful without
quietly turning into a shadow store of secrets, raw arguments, or stale
approvals.

The history row is a **derived projection** over the run record. The
run record is the canonical evidence row. The history row adds the
axes the history surface needs to render a row honestly — the
automation layer, the execution mode, a context summary, a result
class, an artifact-bundle availability state, retention and redaction
posture, and three actions a user might take from the row:

- **Rerun under current policy** — never under cached approval, never
  under stale environment, never as "just replay yesterday's success".
- **Open as recipe** — admissible only when the run's capability set
  is admissible to declarative recipes; macros that touched only the
  closed recorded-macro capability subset MAY be promoted.
- **Export safe summary** — the only export shape that crosses the
  tenant or surface boundary by default.

Companion artifacts:

- [`/schemas/automation/run_history_row.schema.json`](../../schemas/automation/run_history_row.schema.json)
  — boundary schema for `automation_run_history_row`. Every row a
  history surface renders publishes exactly one record against this
  schema.
- [`/schemas/automation/run_summary_export.schema.json`](../../schemas/automation/run_summary_export.schema.json)
  — boundary schema for `run_summary_export_record`. Every safe-summary
  export crossing a tenant or surface boundary publishes exactly one
  record against this schema.
- [`/fixtures/automation/run_history_cases/`](../../fixtures/automation/run_history_cases/)
  — worked example cases for the four shapes the contract freezes:
  a successful declarative recipe history row, a partially blocked run
  whose rerun under current policy is denied, a headless-safe rerun,
  and an extension/external automation row whose safe-summary export
  is redacted before crossing the tenant boundary.

Cross-linked contracts already in the repository:

- [`/docs/automation/recipe_and_macro_contract.md`](./recipe_and_macro_contract.md),
  [`/schemas/automation/recipe_manifest.schema.json`](../../schemas/automation/recipe_manifest.schema.json),
  and
  [`/schemas/automation/run_record.schema.json`](../../schemas/automation/run_record.schema.json)
  — the recipe/macro manifest and run-record contract this contract
  projects from. Every history row resolves to exactly one
  `recipe_run_record` or `macro_replay_record`, except rows whose
  `automation_layer_class` is `extension_or_external_automation_layer`
  and whose run was minted by an external runner (in which case the
  external runner's opaque handle rides the row's `run_id` slot
  verbatim).
- [`/docs/automation/cli_surface_contract.md`](./cli_surface_contract.md)
  and
  [`/schemas/automation/cli_output_registry_entry.schema.json`](../../schemas/automation/cli_output_registry_entry.schema.json)
  — the CLI / headless output registry headless-safe runs project
  through. The history row's `execution_mode_class` carries the
  `headless_cli_explicit_dispatch`, `headless_cli_scripted_dispatch`,
  and `headless_offline_replay_dispatch` slots without re-deciding the
  CLI surface's frozen vocabulary.
- [`/docs/ux/output_log_viewer_contract.md`](../ux/output_log_viewer_contract.md)
  and
  [`/schemas/ux/output_viewer_object.schema.json`](../../schemas/ux/output_viewer_object.schema.json)
  — the shared viewer-object model the desktop history pane composes
  with when rendering output, log, and artifact previews. The history
  row never re-decides truncation or autoscroll posture; the viewer
  object is the authority for that.
- [`/docs/integrations/provider_event_ingestion_contract.md`](../integrations/provider_event_ingestion_contract.md),
  [`/schemas/integrations/import_session.schema.json`](../../schemas/integrations/import_session.schema.json),
  and
  [`/schemas/integrations/webhook_replay_record.schema.json`](../../schemas/integrations/webhook_replay_record.schema.json)
  — the provider-event ingestion contract that pins how imported rows
  cross the boundary into Aureline. The history row's
  `rerun_blocked_imported_record_no_dispatch_admissible` slot is the
  enforcement point for "imported rows MUST NOT offer a one-click
  rerun, because dispatch authority did not survive the import
  boundary".
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — the support-bundle contract this contract's
  `delivery_surface_class: support_bundle_redacted` and
  `support_bundle_operator_only` slots project into.
- [`/docs/governance/record_class_governance.md`](../governance/record_class_governance.md)
  and
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — the class-level retention / redaction posture history rows quote
  rather than re-labelling privately.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — the secret/redaction defaults the safe-summary export inherits for
  secret-bearing data classes; the export's
  `redaction_required_with_secret_broker_handles` slot ties to the
  ADR-0007 broker-handle convention.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
  — the admin-policy contract the
  `policy_observation_class` slot projects through.
- [`/docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)
  — the execution-context capsule every history row cites; preserved
  verbatim from the underlying run record.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — the kill-switch contract the
  `kill_switch_observation_class` slot projects through.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — the workspace-trust contract the
  `trust_state_class` slot projects through.
- [`/docs/ai/evidence_replayability_contract.md`](../ai/evidence_replayability_contract.md)
  — the graded replayability axis the
  `replay_posture_class` slot is graded on. Re-exported, never
  re-invented.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` — power-user automation history
  requirements, MUST-NOT rules on raw argument / raw secret leakage
  through history, rerun-under-current-policy posture, and safe-export
  defaults.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  history-pane architecture, retention model, and audit-log lineage.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — desktop history pane,
  rerun affordance, open-as-recipe affordance, and safe-summary export
  affordance UX posture.

## Why freeze this now

Without this contract, run history would drift along the failure modes
every IDE-class product hits sooner or later:

- **Run history as a shadow store of secrets.** A history pane that
  preserved raw arguments would quietly accumulate raw URLs, raw paths,
  raw env-var values, raw clipboard bytes, raw prompt text, and raw
  credential material — far past the retention window any single
  surface considered. The contract pins that the row carries opaque
  ids, closed vocabulary, content-addresses, and short reviewable
  sentences only; raw bytes never cross either the row boundary or the
  safe-summary export boundary. The schemas' `omitted_fields` block
  enumerates the redacted slots so a reviewer can audit the redaction
  posture without re-loading the run record.
- **Rerun-as-cached-approval.** A "rerun" button that simply replayed
  yesterday's recipe under yesterday's approval ticket would silently
  bypass today's policy / trust / kill-switch / managed-only state. The
  contract's `rerun_action_class` vocabulary forces the row to resolve
  rerun admissibility through one of fifteen closed slots — an
  admissibility class plus a closed-vocabulary list of every blocker
  the resolver observed at projection time. Yesterday's success means
  nothing on its own; the row publishes today's resolved blockers or
  it is non-conforming.
- **Stale environment reuse.** A "rerun" button that replayed
  yesterday's environment capsule (env vars, trust state, policy
  epoch, kill-switch state) would silently dispatch under stale
  posture even when today's reality has drifted. The history row
  preserves the `environment_capsule_ref` from the underlying run
  record but explicitly forbids rerun under that capsule when it has
  drifted: the resolver MUST publish
  `blocker_environment_capsule_drift_detected_now` and the row MUST
  quote `rerun_under_current_policy_blocked_environment_capsule_drift_detected`.
- **Imported records masquerading as live.** A history row imported
  through the provider-event ingestion contract MUST NOT offer a
  one-click rerun: dispatch authority did not survive the import
  boundary. The
  `rerun_blocked_imported_record_no_dispatch_admissible` slot is the
  enforcement point.
- **Macro recordings offered as extension rerun.** A recorded macro
  whose capability set is locked to UI / editor / read-only state
  MUST NOT offer extension/external rerun. The schema's `if/then`
  pinning of admissible `rerun_action_class` per
  `automation_layer_class` is the mechanical enforcement.
- **Capability laundering through "open as recipe".** An
  extension/external automation row that touched a capability not
  admissible to declarative recipes MUST NOT silently lift that
  capability into a new recipe; the row's `open_as_recipe_action_class`
  MUST quote
  `open_as_recipe_inadmissible_no_declarative_capability_path_admitted`
  rather than minting a recipe whose `capability_declarations` is
  partially fabricated.
- **Export without a redaction floor.** A "share my run" button that
  preserved raw arguments by default would leak across tenant
  boundaries. The safe-summary export's `omitted_fields` block pins a
  twelve-class set of raw slots that MUST be omitted by default; the
  schema's `allOf` couplings enforce that
  `omitted_raw_step_argv`, `omitted_raw_secret_values`,
  `omitted_raw_credential_material`, `omitted_raw_approver_identity_material`,
  `omitted_raw_step_parameter_values`, `omitted_raw_environment_variable_values`,
  `omitted_raw_filesystem_paths`, `omitted_raw_urls`,
  `omitted_raw_stdout_stderr`, `omitted_raw_prompt_text`,
  `omitted_raw_buffer_bytes`, and `omitted_raw_clipboard_bytes` appear
  on every safe-summary export.
- **Retention drift.** A history row purged from local storage but
  still listed in the support bundle (or vice versa) would confuse
  reviewers about what is still on disk. The contract's
  `retention_class` and `artifact_bundle_state_class` axes are
  independent and explicit; a row that has been purged but whose safe
  summary survives publishes
  `artifact_bundle_purged_by_retention_summary_only` and the
  artifact-bundle ref is null.

## Scope

Frozen at this revision:

- One `automation_run_history_row` shape with closed vocabularies for
  automation layer, execution mode, context summary (trust / policy /
  kill-switch observation classes), result class (re-export of
  `run_outcome_class`), step-count block, artifact-bundle state,
  retention class, redaction mode, rerun-under-current-policy action,
  current-policy blocker, open-as-recipe action, export action, export
  visibility, support visibility, and display layer; plus
  schema-level `allOf` couplings that enforce the must-rules
  mechanically (denied runs cite a typed gate reason; queued runs
  leave `completed_at` null and pin queued state; replay-window-expired
  runs force the rerun action; macro layers admit only the macro-safe
  rerun subset; extension/external layers admit only the
  extension-safe rerun subset; export-action admissibility is coupled
  to export-visibility).
- One `run_summary_export_record` shape with closed vocabularies for
  delivery surface, summary kind, included fields, omitted fields,
  rerun-eligibility, blockers, redaction mode, and the same export /
  support visibility axes as the history row; plus schema-level `allOf`
  couplings enforcing that the run identity, step-count block, result
  class, approvals block, rerun eligibility, and redaction mode are
  always preserved, and that the twelve raw-slot redaction floor is
  always enforced.
- Worked fixture cases covering the four shapes named in the spec
  acceptance criteria.

Out of scope until a superseding decision row opens:

- The live history pane runtime, the live retention reaper, and the
  live safe-summary export emitter. This contract names the schema
  refs and fixture refs the runtimes will consume; the runtimes land
  in their own lanes after foundations.
- A general-purpose "edit-then-rerun" surface that mutates the
  manifest before rerun. The
  `open_as_recipe_action_class` slot is the only path from a history
  row to a new manifest; mutating the existing manifest in place is
  out of scope.
- Cross-tenant history federation. Each tenant's history pane reads
  its own rows; the safe-summary export is the boundary that crosses
  tenants.
- The full breadth of extension/external runner authorities. The seed
  freezes the row shape; additional runner authorities become
  reachable as their authority records are admitted.

## The four automation layers

The contract publishes one row shape across four automation layers,
discriminated by `automation_layer_class`:

| Layer                                     | Underlying record                                       | Execution surfaces                                                                 | Rerun admissible?                                            |
|-------------------------------------------|---------------------------------------------------------|-------------------------------------------------------------------------------------|--------------------------------------------------------------|
| `recorded_macro_layer`                    | `macro_replay_record`                                    | Desktop macro replay only                                                           | Macro-safe rerun subset only                                 |
| `declarative_recipe_layer`                | `recipe_run_record`                                      | Desktop palette / keybinding / explicit action button / AI assistant / CLI / queued | Full rerun-action vocabulary                                 |
| `managed_only_template_layer`             | `recipe_run_record` (managed-only-template authoring)    | Managed-only channel only                                                           | Managed-only rerun subset; managed-template-retired blocker  |
| `extension_or_external_automation_layer`  | External run handle (or imported `recipe_run_record`)    | Extension / external runner / imported provider event                              | Extension-safe rerun subset only; imported rows blocked      |
| `headless_safe_run_layer`                 | `recipe_run_record` (against `headless_safe` shareability)| CLI / headless / offline replay                                                     | Full rerun-action vocabulary                                 |

A row whose `automation_layer_class` does not match the underlying
record's authoring posture is non-conforming.

## Context summary block: what the row preserves about the dispatch

Every history row carries a `context_summary` block projected from the
underlying run record. The block carries:

- the `execution_context_capsule_ref` (ADR-0009) bounding the run;
- the `environment_capsule_ref` capturing the env vars, trust state,
  policy epoch, and kill-switch state the run observed (null only when
  the run is `non_replayable_raw_byte_dependent`);
- a `trust_state_class` re-export of the
  [`trust_state_matrix.yaml`](../../artifacts/security/trust_state_matrix.yaml)
  axis;
- a `policy_observation_class` re-export of the ADR-0008 admin-policy
  observation axis;
- a `kill_switch_observation_class` re-export of the ADR-0011
  kill-switch axis;
- an optional `context_summary_sentence_ref` to a short reviewable
  sentence describing the workspace / profile / scope (raw workspace
  labels and raw profile names never cross this boundary).

Raw env-var values, raw paths, and raw URLs never appear here. The
history pane reads this block to render "where this ran" honestly
without leaking the run's raw environment.

## Rerun under current policy: the central rule

The single most important rule this contract freezes:

> Run history MUST NEVER imply cached approval or stale environment
> reuse. Every history row MUST resolve rerun-under-current-policy
> admissibility explicitly through `rerun_action_class` and quote
> every blocker the resolver observed through `current_policy_blockers`.

This is enforced mechanically:

1. The row's `rerun_action_class` is drawn from a closed fifteen-class
   set covering admissible-with-no-revalidation, admissible-after-X
   (revalidate environment / grant fresh approval / clear kill-switch /
   resolve managed-only channel), and blocked-by-Y (publisher revoked /
   capability disabled by policy / managed-only-template retired /
   recipe revision retired / replay window expired / descriptor
   revision retired / environment capsule drift / macro-recording-only
   / extension-or-external runner unavailable / imported-record).
2. The row's `current_policy_blockers` list is the authoritative reason
   a rerun would not be admitted today. The schema enforces that
   `rerun_under_current_policy_admissible_no_revalidation_required`
   pairs with exactly `[blocker_no_blocker_present_now]`; any other
   `rerun_action_class` MUST cite at least one non-no-blocker entry.
3. Macro layers are restricted to the macro-safe rerun subset; the
   schema's `if/then` couplings forbid extension/external rerun on a
   macro.
4. Extension/external layers are restricted to the extension-safe
   rerun subset; the schema's `if/then` couplings forbid
   admissible-no-revalidation rerun on an imported record (the
   resolver is forced to quote
   `rerun_blocked_imported_record_no_dispatch_admissible`).
5. `replay_window_expired_stale_authority_denied` runs MUST quote
   `rerun_under_current_policy_blocked_replay_window_expired` — the
   schema enforces it via an `allOf` coupling.

Yesterday's success is **never** an admissibility argument on its own.

## Artifact-bundle state and retention: distinct axes

The history row's `artifact_bundle_state_class` and `retention_class`
are independent. A run that produced no artifact (denied at a gate, a
macro state replay only, an extension/external authority that does
not emit Aureline artifacts) publishes the matching
`artifact_bundle_not_produced_*` slot and the `artifact_bundle_ref` is
null. A run whose artifact bundle has been purged by retention
publishes `artifact_bundle_purged_by_retention_summary_only` and the
`artifact_bundle_ref` is null but the row's
`run_summary_export_record` may still survive at the export boundary.

Retention windows the row tracks:

- `retain_until_purged_by_user` — user-controlled retention only;
- `retain_until_workspace_redaction_window` — workspace policy bounded;
- `retain_until_organization_audit_window` — org admin policy bounded;
- `retain_until_support_export_consumed` — bounded to the lifecycle of
  a support packet that resolved through this row;
- `retain_until_replay_window_expires` — bounded to the
  `queued_replay_window_bound_remaining` window;
- `retain_indefinitely_under_audit_lock` — only when an organization
  audit lock holds the row;
- `purged_by_retention_summary_only_remains` — terminal state.

When a windowed retention class is in effect, the row's
`retention_window_expires_at` is non-null.

## Open-as-recipe: macro promotion path

A recorded macro whose capability set is locked to the closed
recorded-macro subset MAY be promoted to a declarative recipe via
`open_as_recipe_admissible_macro_promotable_to_declarative_recipe`.
The promotion creates a new `recipe_manifest_record` whose
`authoring_posture_class` is
`recorded_macro_promoted_to_declarative_recipe` (per
[`recipe_manifest.schema.json`](../../schemas/automation/recipe_manifest.schema.json));
the new manifest cites the macro id it was promoted from in
`promoted_from_macro_id`.

An extension/external automation row whose underlying capability set
is admissible to declarative recipes MAY be authored as a recipe via
`open_as_recipe_admissible_extension_or_external_authored_as_declarative_recipe`.
A row whose underlying capability set is **not** admissible to
declarative recipes MUST quote
`open_as_recipe_inadmissible_no_declarative_capability_path_admitted`
rather than silently lifting the capability into a recipe whose
`capability_declarations` would be partially fabricated.

A row that is already a declarative recipe quotes
`open_as_recipe_inadmissible_already_declarative_recipe`. A row that
is already a managed-only template quotes
`open_as_recipe_inadmissible_already_managed_only_template`. A row
whose underlying capability set requires extension/external authority
quotes `open_as_recipe_inadmissible_extension_or_external_authority_required`.

## Safe-summary export: what crosses the boundary

The `run_summary_export_record` is the only export shape that crosses
a tenant or surface boundary by default. It preserves:

- **Run identity** — `run_id`, `manifest_id`, `manifest_revision_ref`,
  optional `manifest_content_address`.
- **Step counts** — the `step_count_block` (total / succeeded /
  denied / skipped / queued / aborted / partial-success / dry-run-only)
  preserved as integers, never as raw step argv.
- **Result + replay posture** — `result_class`,
  `gate_denial_reason_class`, `replay_posture_class`,
  `queueability_state_class`, `reconciliation_state_class`.
- **Artifact + effect-lineage refs** — opaque refs only,
  preserved into `artifact_root_refs_preserved` and
  `effect_lineage_root_refs_preserved`. Raw artifact bytes and raw
  paths never cross the boundary.
- **Approvals summary** — the `approvals_preserved_block`
  (consumed-count, posture set, opaque ticket refs). Raw approver
  identity material never crosses the boundary.
- **Current-policy rerun eligibility** — the
  `current_policy_rerun_eligibility_class` and the closed-vocabulary
  list of every observed blocker. The export preserves rerun
  admissibility honestly so a downstream reviewer never reads the
  export as cached approval.
- **Redaction posture** — the `redaction_mode_class`,
  `export_visibility_class`, and `support_visibility_class`.
- **Timestamps** — `dispatched_at`, `completed_at`,
  `summary_emitted_at`.

The export's `omitted_fields` list quotes one entry per redacted
slot. The schema enforces that the following slots MUST appear on
every safe-summary export:

- `omitted_raw_step_argv`
- `omitted_raw_step_parameter_values`
- `omitted_raw_secret_values`
- `omitted_raw_environment_variable_values`
- `omitted_raw_filesystem_paths`
- `omitted_raw_urls`
- `omitted_raw_stdout_stderr`
- `omitted_raw_prompt_text`
- `omitted_raw_buffer_bytes`
- `omitted_raw_credential_material`
- `omitted_raw_clipboard_bytes`
- `omitted_raw_approver_identity_material`

Optional omitted slots cover macro-capture buffers, recipe step
argument templates, descriptor argument-inspection payloads,
extension/external payload bytes, and provider event payload bytes
(per the integrations contract).

The export's `included_fields` list quotes one entry per preserved
slot. The schema enforces that the following slots MUST appear:

- `included_run_identity_run_id_manifest_id_manifest_revision_ref`
- `included_step_count_block`
- `included_result_class_and_gate_denial_reason_class`
- `included_approvals_preserved_block_postures_and_opaque_ticket_refs`
- `included_current_policy_rerun_eligibility_and_blockers`
- `included_redaction_mode_class`

Adding new included or omitted slots is additive-minor (bumps
`run_summary_export_schema_version`); repurposing an existing slot is
breaking and requires a new decision row.

## Delivery surfaces and summary kinds

A single run MAY emit different safe-summary exports for different
delivery surfaces (a `support_bundle_redacted` summary and an
`organization_audit_log` summary may have distinct redaction
postures); each emission publishes one record against the export
schema. The `delivery_surface_class` slot pins one emission per
record:

- `support_bundle_redacted` — default support-bundle export;
  redaction floor is the metadata-safe default plus the twelve
  required omitted slots.
- `support_bundle_operator_only` — operator-only support export with
  broader internal preservation.
- `organization_audit_log` — managed-only-channel audit log emission;
  pairs with `summary_organization_audit_only_with_full_lineage_refs`
  and `exportable_via_organization_share_managed_only`.
- `shiproom_dashboard` — shiproom dashboard summary tile.
- `portable_profile_export` — portable profile export of the user's
  history.
- `headless_offline_export` — headless / CLI offline export.
- `ai_assistant_evidence_pack` — AI assistant evidence pack summary
  for runs that influenced AI evidence.

The `summary_kind_class` slot pins which slot of preserved fields the
emission carries: summary-only, summary-with-artifact-refs,
summary-with-artifact-and-effect-lineage-refs,
summary-with-redacted-payload-handles-only, or
summary-organization-audit-only-with-full-lineage-refs. The schema's
`allOf` couplings enforce that the redacted-payload summary kind
quotes `redaction_required_with_secret_broker_handles` or
`redaction_required_on_export`, and that the
organization-audit-only summary kind pairs with the matching delivery
surface and export visibility.

## Registry-level invariants

The schemas' invariants blocks pin the following constants to `true`.
A seed that sets any of them to `false` is non-conforming.

Run-history row invariants
(`run_history_row.schema.json#/$defs/run_history_row_invariants_block`):

1. `every_row_resolves_to_run_id_and_manifest_id_and_revision`
2. `every_row_quotes_run_outcome_class_from_run_record_without_recategorising`
3. `every_row_resolves_rerun_under_current_policy_explicitly`
4. `rerun_admissibility_never_implies_cached_approval_or_stale_environment`
5. `current_policy_blockers_are_the_authoritative_reason_a_rerun_would_not_be_admitted_today`
6. `macro_history_rows_never_offer_extension_or_external_rerun`
7. `extension_or_external_history_rows_never_silently_lift_capability_into_a_recipe`
8. `imported_records_never_offer_one_click_rerun`
9. `raw_step_argv_never_appears_in_a_history_row`
10. `raw_paths_urls_secrets_buffer_bytes_prompt_text_credential_material_never_appear_in_a_history_row`
11. `approver_identity_bytes_never_appear_in_a_history_row`
12. `purged_rows_keep_only_the_safe_summary`
13. `export_action_admissibility_is_coupled_to_export_visibility_class`

Safe-summary export invariants
(`run_summary_export.schema.json#/$defs/run_summary_export_invariants_block`):

1. `every_export_preserves_run_id_and_manifest_id_and_revision`
2. `every_export_preserves_step_count_block_not_raw_step_argv`
3. `every_export_preserves_artifact_root_refs_as_opaque_only`
4. `every_export_preserves_approvals_summary_not_raw_approver_identity`
5. `every_export_preserves_current_policy_rerun_eligibility_and_blockers`
6. `rerun_eligibility_never_implies_cached_approval_or_stale_environment`
7. `raw_step_argv_never_appears_in_a_safe_summary_export`
8. `raw_secrets_credentials_paths_urls_buffer_bytes_prompt_text_never_appear_in_a_safe_summary_export`
9. `approver_identity_bytes_never_appear_in_a_safe_summary_export`
10. `omitted_fields_block_quotes_every_redacted_slot_so_redaction_posture_is_auditable`
11. `delivery_surface_class_pins_one_emission_per_safe_summary_export`

## Forbidden collapses

The contract names a closed list of failure modes a history surface
or an export emitter MUST NOT silently fall into:

- Rendering a "rerun" button without resolving rerun-under-current-policy.
- Treating yesterday's approval ticket as today's authority.
- Replaying yesterday's environment capsule when today's environment
  has drifted.
- Offering one-click rerun on an imported provider-event row.
- Offering extension/external rerun on a recorded macro.
- Lifting an extension/external capability into a new recipe whose
  `capability_declarations` is partially fabricated.
- Preserving raw step argv, raw URLs, raw paths, raw env-var values,
  raw clipboard bytes, raw prompt text, raw buffer bytes, raw secret
  values, raw credential material, raw approver identity material, or
  raw stdout / stderr in a history row or a safe-summary export.
- Hiding a purged artifact bundle behind an unchanged
  `artifact_bundle_state_class` instead of transitioning to
  `artifact_bundle_purged_by_retention_summary_only`.
- Emitting a safe-summary export whose `omitted_fields` block does
  not quote each of the twelve required raw-slot omissions.
- Emitting an `organization_audit_log` summary without quoting
  `exportable_via_organization_share_managed_only`.

## Schema of record

The eventual Aureline automation crates' Rust types are the schema of
record. The JSON Schema exports at
[`/schemas/automation/run_history_row.schema.json`](../../schemas/automation/run_history_row.schema.json)
and
[`/schemas/automation/run_summary_export.schema.json`](../../schemas/automation/run_summary_export.schema.json)
are the cross-tool boundary every non-owning surface reads. Adding a
new enum value to any frozen vocabulary is additive-minor and bumps
the relevant `_schema_version` const; repurposing an existing value is
breaking and requires a new decision row.

## Source anchors

- [`docs/automation/recipe_and_macro_contract.md`](./recipe_and_macro_contract.md)
- [`docs/automation/cli_surface_contract.md`](./cli_surface_contract.md)
- [`docs/ux/output_log_viewer_contract.md`](../ux/output_log_viewer_contract.md)
- [`docs/integrations/provider_event_ingestion_contract.md`](../integrations/provider_event_ingestion_contract.md)
- [`docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
- [`docs/governance/record_class_governance.md`](../governance/record_class_governance.md)
- [`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
- [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
- [`docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)
- [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
- [`docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
- [`docs/ai/evidence_replayability_contract.md`](../ai/evidence_replayability_contract.md)
- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md),
  [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md),
  [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — automation history posture, rerun-under-current-policy posture,
  and safe-export defaults.
