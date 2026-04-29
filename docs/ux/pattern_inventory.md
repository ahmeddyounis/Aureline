# Required UX pattern inventory

This document is the frozen index of shared, high-risk UX patterns that
must be reused across surfaces instead of being re-implemented inside one
panel, sheet, or sidebar. It is the reviewer-facing companion to
[`/docs/ux/pattern_contract_template.md`](./pattern_contract_template.md)
and the machine-readable crosswalk in
[`/artifacts/ux/pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml).

## Why this exists

Without one inventory:

- the same protected user journey (recovery, refactor preview, deferred
  reconnect, support escalation) gets re-invented inside every surface
  that touches it;
- the spec prose at sections 22.3–22.12 of
  [`Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  is the only definition of pattern behavior, with no reusable contract
  to cite;
- M1+ implementation reviewers have no way to tell whether a new sheet
  or banner truly reuses the protected pattern or merely rhymes with it;
- waivers and freeze exceptions land as free-text notes instead of
  visible refs.

This inventory closes that gap. Every required pattern in this list
points at:

- the spec section that fixes its behavior;
- the schemas, fixtures, and artifacts that already encode it;
- the requirement IDs the pattern protects; and
- the surfaces in
  [`/artifacts/ux/surface_traceability_matrix.yaml`](../../artifacts/ux/surface_traceability_matrix.yaml)
  that must reuse it.

A surface that wants to opt out of one of these patterns MUST raise a
visible freeze exception or waiver against the corresponding row in
[`/artifacts/ux/pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml);
silent local exceptions are non-conforming.

## Scope

This file freezes the **shared contract and traceability** for the
following ten required patterns. It does **not** freeze the full
implementation of any pattern in M0; downstream tasks are responsible
for those rollouts.

Sibling pattern families that already publish their own freeze (for
example, the durable job-row contract or the trust-prompt contract)
remain authoritative inside their named sections; this inventory only
adds cross-pattern reuse and crosswalk obligations on top of those
component contracts.

## Required pattern index

| Pattern id | Title | Spec section | Primary contract |
| --- | --- | --- | --- |
| `pattern.project_doctor` | Project Doctor | UX spec §22.3, §18.6, §18.21 | [`docs/support/project_doctor_packet.md`](../support/project_doctor_packet.md) |
| `pattern.refactor_preview_write_scope` | Refactor preview and write scope | UX spec §22.4 | [`docs/editor/refactor_and_replace_transaction_contract.md`](../editor/refactor_and_replace_transaction_contract.md) |
| `pattern.optional_cloud_outage_local_first` | Optional-cloud outage and local-first continuity | UX spec §22.5 | [`docs/ux/control_data_plane_status_contract.md`](./control_data_plane_status_contract.md) |
| `pattern.generated_artifact_drift_regenerate_first` | Generated-artifact drift and regenerate-first | UX spec §22.6 | [`docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md) |
| `pattern.exposed_service_review_revoke` | Exposed-service review and revoke | UX spec §22.7 | [`docs/governance/control_artifact_index.md`](../governance/control_artifact_index.md) |
| `pattern.shared_session_downgrade_control_transfer` | Shared-session downgrade, control transfer, and consent renewal | UX spec §22.8 | [`docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md) |
| `pattern.mutation_journal_compensation` | Unified mutation-journal and compensation | UX spec §22.9, §11.19 | [`docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md) |
| `pattern.deferred_intent_reconciliation` | Deferred-intent and reconciliation | UX spec §22.10, §18.32 | [`docs/runtime/connectivity_and_reconciliation_contract.md`](../runtime/connectivity_and_reconciliation_contract.md) |
| `pattern.workspace_state_serialization_restore_provenance` | Workspace-state serialization and restore provenance | UX spec §22.11 | [`docs/ux/persistence_inspector_contract.md`](./persistence_inspector_contract.md) |
| `pattern.support_intake_escalation` | Support intake, escalation packet, and field readiness | UX spec §22.12 | [`docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md) |

## Pattern entries

Each entry below names: the protected user journey, the participating
surfaces (by id from
[`surface_traceability_matrix.yaml`](../../artifacts/ux/surface_traceability_matrix.yaml)),
the required state vocabulary, the governing schemas and contracts, the
requirement IDs the pattern protects, accessibility hooks, and the
forbidden shortcuts that would silently weaken the pattern. The
machine-readable form of these joins lives in
[`pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml).

### `pattern.project_doctor` — Project Doctor

- **Protected journey.** A user, support engineer, or automation can
  diagnose a workspace, runtime, extension, or trust failure through one
  read-only-by-default surface that names findings, blast radius, and
  the narrowest safe repair before any mutation.
- **Required state vocabulary.** finding severity classes
  `blocked`/`degraded`/`warning`/`info`; repair classes
  `exact_undo`/`compensation`/`regeneration`/`checkpoint_restore`/`no_undo`;
  recovery rungs from the recovery-ladder packet.
- **Governing artifacts.**
  [`docs/support/project_doctor_packet.md`](../support/project_doctor_packet.md),
  [`docs/support/project_doctor_probe_contract.md`](../support/project_doctor_probe_contract.md),
  [`docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md),
  [`schemas/support/doctor_finding.schema.json`](../../schemas/support/doctor_finding.schema.json),
  [`schemas/support/doctor_explanation.schema.json`](../../schemas/support/doctor_explanation.schema.json),
  [`schemas/support/repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json),
  [`schemas/support/repair_outcome.schema.json`](../../schemas/support/repair_outcome.schema.json),
  [`schemas/support/repair_preview.schema.json`](../../schemas/support/repair_preview.schema.json),
  [`fixtures/support/project_doctor_cases/`](../../fixtures/support/project_doctor_cases),
  [`fixtures/support/repair_cases/`](../../fixtures/support/repair_cases),
  [`fixtures/support/recovery_ladder_cases/`](../../fixtures/support/recovery_ladder_cases),
  [`artifacts/support/diagnosis_latency_scoreboard.yaml`](../../artifacts/support/diagnosis_latency_scoreboard.yaml),
  [`artifacts/support/diagnosis_slo_targets.yaml`](../../artifacts/support/diagnosis_slo_targets.yaml).
- **Requirement refs.** `REL-SUPPORT-001`, `REL-REPAIR-015`,
  `OPS-SUP-005`, `GOV-EVID-901`, `GOV-CORPUS-901`.
- **Accessibility hooks.**
  `corpus.accessibility.execution.task_run_review` (proxy until a doctor
  task lands), trust-prompt and command-palette accessibility tasks for
  the launch surfaces; assistive-technology must read finding severity,
  repair class, and reversal class as distinct fields, not as one
  collapsed status string.
- **Forbidden shortcuts.** Auto-running mutating repairs without a
  preview; collapsing repair-class vocabulary into "fix"; promoting
  upload/escalation paths above the local-only path; dropping evidence
  IDs across navigation.

### `pattern.refactor_preview_write_scope` — Refactor preview and write scope

- **Protected journey.** Any broad write (rename, move, structural
  refactor, batch apply, AI-proposed mutation, importer apply, policy
  fixup) is reviewed before commit with file-class disclosure, scope
  attribution, and an explicit reversal class.
- **Required state vocabulary.** changed-file class
  `created`/`modified`/`renamed`/`deleted`/`generated`/`protected`/`read_only`;
  scope-source class
  `user_command`/`extension`/`ai`/`import`/`migration`/`policy`;
  reversal class shared with `pattern.mutation_journal_compensation`.
- **Governing artifacts.**
  [`docs/editor/refactor_and_replace_transaction_contract.md`](../editor/refactor_and_replace_transaction_contract.md),
  [`docs/settings/precedence_lock_and_write_scope_contract.md`](../settings/precedence_lock_and_write_scope_contract.md),
  [`docs/ux/file_state_badge_and_write_review_contract.md`](./file_state_badge_and_write_review_contract.md),
  [`docs/ux/live_update_review_contract.md`](./live_update_review_contract.md),
  [`schemas/recovery/restore_preview.schema.json`](../../schemas/recovery/restore_preview.schema.json),
  [`schemas/ecosystem/package_restore_preview.schema.json`](../../schemas/ecosystem/package_restore_preview.schema.json),
  [`fixtures/recovery/restore_preview_cases/`](../../fixtures/recovery/restore_preview_cases),
  [`fixtures/settings/precedence_cases/`](../../fixtures/settings/precedence_cases).
- **Requirement refs.** `TOOL-CTX-002`, `REL-MUT-014`, `REL-CORE-003`,
  `SEC-TRUST-001`.
- **Accessibility hooks.** Preview rows must expose changed-file class,
  scope attribution, and reversal class as discrete labels;
  keyboard path must reach `Apply`, `Cancel`, `Compare`, and
  `Open canonical source` without trapping focus inside the diff body.
- **Forbidden shortcuts.** Hiding generated/protected/read-only counts
  inside an aggregated number; collapsing AI/extension/import scope
  attribution into a single "system" actor; deferring scope review to
  post-apply.

### `pattern.optional_cloud_outage_local_first` — Optional-cloud outage and local-first continuity

- **Protected journey.** When a managed service or optional cloud
  capability degrades, local editing/search/tasks/Git/configured-direct-remote
  work continues, the failing capability is named specifically, and
  local history/settings/active work is never framed as at risk because
  of a service-plane event.
- **Required state vocabulary.** connectivity classes
  `Connected`/`Constrained`/`OfflineLocalSafe`/`ReauthRequired`/`ReconciliationPending`/`ServiceUnavailable`;
  outage notice classes
  `scheduled`/`read_only`/`drain`/`migration`/`failover`/`reconciling`/`resolved`;
  control-plane vs data-plane scope; what-still-works disclosure.
- **Governing artifacts.**
  [`docs/ux/control_data_plane_status_contract.md`](./control_data_plane_status_contract.md),
  [`docs/integrations/provider_account_mapping_and_offline_capture_contract.md`](../integrations/provider_account_mapping_and_offline_capture_contract.md),
  [`docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md),
  [`docs/ecosystem/package_restore_and_mirror_continuity_contract.md`](../ecosystem/package_restore_and_mirror_continuity_contract.md),
  [`docs/identity/offline_entitlement_and_policy_seed.md`](../identity/offline_entitlement_and_policy_seed.md),
  [`schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json),
  [`schemas/runtime/connectivity_state.schema.json`](../../schemas/runtime/connectivity_state.schema.json),
  [`schemas/ecosystem/offline_continuity_card.schema.json`](../../schemas/ecosystem/offline_continuity_card.schema.json),
  [`fixtures/ops/outage_notices/`](../../fixtures/ops/outage_notices),
  [`fixtures/runtime/connectivity_cases/`](../../fixtures/runtime/connectivity_cases).
- **Requirement refs.** `OPS-CLOUD-002`, `REL-CORE-003`, `OPS-SUP-005`,
  `ARCH-UX-005`.
- **Accessibility hooks.** Outage banners must announce the failing
  capability, the still-working scope, and any safe-defer action as
  three discrete announcements rather than one merged status; quiet
  hours rules from
  [`artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  apply to escalation tier.
- **Forbidden shortcuts.** Generic "service is down" copy that implies
  total loss of function; merging control-plane and data-plane scope;
  removing local history visibility while a service-plane event is
  active.

### `pattern.generated_artifact_drift_regenerate_first` — Generated-artifact drift and regenerate-first

- **Protected journey.** Generated, paired, or canonical-source-bound
  artifacts surface drift before a destructive write becomes the
  default; users see `Open canonical source`, `Compare drift`, and
  `Regenerate` ahead of raw edit when the canonical source remains
  authoritative; generator unavailability or policy block is disclosed
  inline.
- **Required state vocabulary.** edit-posture classes
  `viewer`/`semantic_merge`/`regenerate_or_review`/`no_auto_merge`;
  drift classes
  `in_sync`/`drifted_safe_to_regenerate`/`drifted_review_required`/`generator_unavailable`/`policy_blocked`;
  source-of-truth pairing rules from the structured-artifact matrix.
- **Governing artifacts.**
  [`docs/artifacts/structured_artifact_and_debug_fidelity_matrix.md`](../artifacts/structured_artifact_and_debug_fidelity_matrix.md),
  [`docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md),
  [`docs/review/structured_artifact_review_seed.md`](../review/structured_artifact_review_seed.md),
  [`docs/governance/drift_blocking_rules.md`](../governance/drift_blocking_rules.md),
  [`docs/runtime/resource_drift_and_live_action_contract.md`](../runtime/resource_drift_and_live_action_contract.md),
  [`schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json),
  [`artifacts/review/structured_artifact_classes.yaml`](../../artifacts/review/structured_artifact_classes.yaml),
  [`artifacts/artifacts/debug_fidelity_rows.yaml`](../../artifacts/artifacts/debug_fidelity_rows.yaml),
  [`artifacts/artifacts/merge_resolution_policy_rows.yaml`](../../artifacts/artifacts/merge_resolution_policy_rows.yaml),
  [`fixtures/artifacts/fidelity_examples/`](../../fixtures/artifacts/fidelity_examples),
  [`fixtures/runtime/resource_drift_cases/`](../../fixtures/runtime/resource_drift_cases).
- **Requirement refs.** `REL-REPAIR-015`, `REL-MUT-014`, `TOOL-CTX-002`,
  `GOV-DATA-002`.
- **Accessibility hooks.** Drift state must be exposed as a row label,
  not only as iconography; canonical-source links must be focusable
  ahead of raw-edit; reduced-motion mode must not strip the drift
  indicator.
- **Forbidden shortcuts.** Letting raw edit be the default action when
  canonical source remains authoritative; collapsing
  `generator_unavailable` and `policy_blocked` into a generic
  unavailable posture; suppressing detach/manual-edit only because the
  artifact looks like text.

### `pattern.exposed_service_review_revoke` — Exposed-service review and revoke

- **Protected journey.** Routes, previews, and shared services begin
  with an exposure review (audience, auth, expiry), keep an active
  banner while live, and preserve route identity, related previews, and
  last reachability after revocation.
- **Required state vocabulary.** exposure states
  `proposed`/`reviewed`/`live`/`expiring`/`revoked`/`stale_post_revocation`;
  audience classes
  `private`/`team`/`org`/`public`/`unsigned_link`; revocation outcomes
  `closed`/`closed_with_residual_cache`/`closed_pending_provider_confirmation`.
- **Governing artifacts.**
  [`docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md),
  [`docs/governance/control_artifact_index.md`](../governance/control_artifact_index.md),
  [`docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md),
  [`schemas/collaboration/control_grant.schema.json`](../../schemas/collaboration/control_grant.schema.json),
  [`schemas/collaboration/session_state.schema.json`](../../schemas/collaboration/session_state.schema.json),
  [`schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json),
  [`schemas/language/capability_negotiation_packet.schema.json`](../../schemas/language/capability_negotiation_packet.schema.json),
  [`schemas/service/api_capability_row.schema.json`](../../schemas/service/api_capability_row.schema.json),
  [`fixtures/auth/capture_boundary_cases/`](../../fixtures/auth/capture_boundary_cases),
  [`fixtures/admin/org_admin_cases/`](../../fixtures/admin/org_admin_cases).
- **Requirement refs.** `SEC-TRUST-001`, `REL-CORE-003`, `OPS-SUP-005`,
  `GOV-DATA-002`.
- **Accessibility hooks.** Active-route banner must announce audience,
  auth class, and expiry as discrete fields; revoke action must be
  reachable by keyboard from any surface that exposes the route.
- **Forbidden shortcuts.** Allowing a copied URL to retroactively become
  governed; hiding the active-route banner once a side panel is closed;
  collapsing post-revocation residue into "closed".

### `pattern.shared_session_downgrade_control_transfer` — Shared-session downgrade, control transfer, and consent renewal

- **Protected journey.** Role changes, control grants, recording-state
  changes, and retention-broadening events feel explicit, attributable,
  and survivable: users can decline broadened terms and continue
  locally, leave, or fall back to read-only.
- **Required state vocabulary.** session role
  `host`/`co_host`/`participant`/`viewer`/`local_continuation`;
  authority delta classes
  `granted`/`renewed`/`revoked`/`auto_downgraded`/`expired`;
  recording state `off`/`on`/`paused`/`local_only`; retention class.
- **Governing artifacts.**
  [`docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md),
  [`docs/architecture/collaboration_session_layer_adr.md`](../architecture/collaboration_session_layer_adr.md),
  [`docs/notebooks/notebook_collaboration_contract.md`](../notebooks/notebook_collaboration_contract.md),
  [`docs/auth/managed_auth_and_session_continuity_contract.md`](../auth/managed_auth_and_session_continuity_contract.md),
  [`schemas/collaboration/session_state.schema.json`](../../schemas/collaboration/session_state.schema.json),
  [`schemas/collaboration/session_policy_manifest.schema.json`](../../schemas/collaboration/session_policy_manifest.schema.json),
  [`schemas/collaboration/follow_and_presenter_state.schema.json`](../../schemas/collaboration/follow_and_presenter_state.schema.json),
  [`schemas/collaboration/shared_object.schema.json`](../../schemas/collaboration/shared_object.schema.json),
  [`fixtures/collab/`](../../fixtures/collab).
- **Requirement refs.** `REL-MUT-014`, `SEC-TRUST-001`, `ARCH-UX-005`,
  `OPS-SUP-005`.
- **Accessibility hooks.** Authority-change events must be live-region
  announcements with actor, change kind, and effective-time; focus
  return rule applies after a forced downgrade; quiet-hours and
  interruptibility-tier rules apply to the announcement.
- **Forbidden shortcuts.** Treating role flips as ambient status flicker;
  hiding who triggered the change once the sheet closes; auto-extending
  recording or retention without renewed consent.

### `pattern.mutation_journal_compensation` — Unified mutation-journal and compensation

- **Protected journey.** Any destructive, wide-scope, governed, or
  share-affecting action emits a stable mutation ID, lands in one
  inspectable history model, and matches reversal language to the true
  recovery class so users do not over-trust "undo".
- **Required state vocabulary.** reversal classes
  `undo`/`revert_with_compensating_action`/`regenerate`/`rollback`/`manual_recovery`;
  mutation visibility classes; lineage links across editor, review, AI,
  repair, package, route, and support surfaces.
- **Governing artifacts.**
  [`docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md),
  [`docs/reliability/autosave_journal_and_guided_replay_contract.md`](../reliability/autosave_journal_and_guided_replay_contract.md),
  [`docs/vcs/history_edit_and_recovery_contract.md`](../vcs/history_edit_and_recovery_contract.md),
  [`schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json),
  [`schemas/recovery/autosave_journal_entry.schema.json`](../../schemas/recovery/autosave_journal_entry.schema.json),
  [`schemas/recovery/guided_replay_choice.schema.json`](../../schemas/recovery/guided_replay_choice.schema.json),
  [`fixtures/workspace/mutation_lineage_examples/`](../../fixtures/workspace/mutation_lineage_examples),
  [`fixtures/recovery/autosave_replay_cases/`](../../fixtures/recovery/autosave_replay_cases),
  [`fixtures/ux/repair_cards/`](../../fixtures/ux/repair_cards).
- **Requirement refs.** `REL-MUT-014`, `REL-SUPPORT-001`,
  `REL-REPAIR-015`, `TOOL-CTX-002`.
- **Accessibility hooks.** Reversal-class label must be the first piece
  of information assistive technology reads on a recovery action; the
  journal row keyboard path must remain stable across editor, review,
  AI, package, and support surfaces.
- **Forbidden shortcuts.** Calling a compensation `undo`; suppressing
  mutation IDs in cross-surface citations; building a parallel local
  history for one feature.

### `pattern.deferred_intent_reconciliation` — Deferred-intent and reconciliation

- **Protected journey.** Any action that survives a disconnect behaves
  like a reviewed intent: command identity, target, actor, queued time,
  expiry, last validation state, and previewed effect remain visible;
  reconnect does not silently replay drifted intents.
- **Required state vocabulary.** intent states
  `Queued`/`Needs_review`/`Replayed`/`Expired`/`Cancelled`/`Dropped`;
  drift checks for target, policy epoch, auth scope, context hash;
  queueability classes for local-core, cached read, idempotent managed
  write, collaboration control, remote execution, paid model dispatch,
  background sync.
- **Governing artifacts.**
  [`docs/runtime/connectivity_and_reconciliation_contract.md`](../runtime/connectivity_and_reconciliation_contract.md),
  [`schemas/runtime/connectivity_state.schema.json`](../../schemas/runtime/connectivity_state.schema.json),
  [`schemas/runtime/deferred_intent.schema.json`](../../schemas/runtime/deferred_intent.schema.json),
  [`schemas/runtime/reconciliation_result.schema.json`](../../schemas/runtime/reconciliation_result.schema.json),
  [`schemas/ai/deferred_intent_outbox.schema.json`](../../schemas/ai/deferred_intent_outbox.schema.json),
  [`schemas/work_items/offline_handoff_packet.schema.json`](../../schemas/work_items/offline_handoff_packet.schema.json),
  [`fixtures/runtime/connectivity_cases/`](../../fixtures/runtime/connectivity_cases).
- **Requirement refs.** `OPS-CLOUD-002`, `REL-CORE-003`, `REL-MUT-014`,
  `SEC-TRUST-001`.
- **Accessibility hooks.** Each deferred row must expose target,
  preview, and expiry as discrete labels; queue review must be
  reachable from the activity surface keyboard path; reconnect-replay
  decisions must announce drift findings before any apply.
- **Forbidden shortcuts.** Silent retry of destructive or non-idempotent
  intents; collapsing `Expired` and `Cancelled`; allowing a
  collaboration-grant change to outbox.

### `pattern.workspace_state_serialization_restore_provenance` — Workspace-state serialization and restore provenance

- **Protected journey.** Remembered state stays inspectable and
  bounded: artifact class, schema version, producer build, redaction
  posture, and `local_only`/`portable`/`shared` label are visible
  before export or import; restore provenance remains visible after
  reopen so users can tell whether they got an exact restore, a
  compatible restore, layout only, or evidence only.
- **Required state vocabulary.** restore-fidelity classes
  `exact_restore`/`compatible_restore`/`layout_only`/`evidence_only`;
  portability classes `local_only`/`portable`/`shared`; degraded
  classes
  `compatible_restore`/`missing_target`/`capability_disabled_by_policy`/`basis_snapshot_drifted`;
  placeholder-card rule for missing dependencies.
- **Governing artifacts.**
  [`docs/ux/persistence_inspector_contract.md`](./persistence_inspector_contract.md),
  [`docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md),
  [`docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md),
  [`docs/workspace/prebuild_fingerprint_contract.md`](../workspace/prebuild_fingerprint_contract.md),
  [`docs/state/workspace_memory_contract.md`](../state/workspace_memory_contract.md),
  [`docs/ux/state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md),
  [`schemas/workspace/workset_artifact.schema.json`](../../schemas/workspace/workset_artifact.schema.json),
  [`schemas/workspace/entry_and_restore_result.schema.json`](../../schemas/workspace/entry_and_restore_result.schema.json),
  [`schemas/workspace/prebuild_fingerprint.schema.json`](../../schemas/workspace/prebuild_fingerprint.schema.json),
  [`schemas/state/portable_state_package.schema.json`](../../schemas/state/portable_state_package.schema.json),
  [`fixtures/workspace/layout_serialization_examples/`](../../fixtures/workspace/layout_serialization_examples),
  [`fixtures/workspace/entry_restore_examples/`](../../fixtures/workspace/entry_restore_examples),
  [`fixtures/state/restore_provenance_cards/`](../../fixtures/state/restore_provenance_cards),
  [`fixtures/profile/restore_provenance_examples/`](../../fixtures/profile/restore_provenance_examples).
- **Requirement refs.** `FR-ENTRY-001`, `FR-ENTRY-002`, `FR-BOOT-006`,
  `REL-CORE-003`, `MIG-INT-008`, `CERT-WS-001`.
- **Accessibility hooks.** Inspector rows must read artifact class,
  schema version, producer build, and portability as discrete labels;
  restore-provenance card must remain announceable after reopen via the
  shell-conformance launcher task.
- **Forbidden shortcuts.** Treating remembered state as one opaque
  blob; silently dropping a missing-dependency pane instead of leaving
  a placeholder; conflating layout reopen with state restore.

### `pattern.support_intake_escalation` — Support intake, escalation packet, and field readiness

- **Protected journey.** Support feels like part of the product: intake
  starts with scenario-coded classification (with an honest
  `uncategorized` path); every recommended repair names blast radius,
  reversal path, and why it is safer than the more destructive
  alternative; escalation packets preview included/excluded evidence,
  current owner, and next expected step before export or upload; human
  handoff preserves scenario family, finding codes, build/profile
  identity, related packet IDs, and attempted-repair history.
- **Required state vocabulary.** delivery classes
  `saved_local_only`/`browser_handoff_blocked_retry_later`/`attached_by_reference`/`manual_review_required`;
  redaction-choice classes; recovery-rung references; field-readiness
  evidence classes.
- **Governing artifacts.**
  [`docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md),
  [`docs/support/support_bundle_preview_contract.md`](../support/support_bundle_preview_contract.md),
  [`docs/support/supportability_slo_and_pack_contract.md`](../support/supportability_slo_and_pack_contract.md),
  [`docs/support/support_center_concept.md`](../support/support_center_concept.md),
  [`docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md),
  [`docs/security/intake_and_triage.md`](../security/intake_and_triage.md),
  [`docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md),
  [`schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json),
  [`schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json),
  [`fixtures/support/support_bundle_examples/`](../../fixtures/support/support_bundle_examples),
  [`fixtures/support/object_handoff_examples/`](../../fixtures/support/object_handoff_examples),
  [`fixtures/support/escalation_packet_completeness_cases/`](../../fixtures/support/escalation_packet_completeness_cases),
  [`fixtures/support/drill_scenarios/`](../../fixtures/support/drill_scenarios),
  [`fixtures/support/redaction_profiles/`](../../fixtures/support/redaction_profiles),
  [`artifacts/support/diagnosis_latency_scoreboard.yaml`](../../artifacts/support/diagnosis_latency_scoreboard.yaml),
  [`artifacts/support/diagnosis_slo_targets.yaml`](../../artifacts/support/diagnosis_slo_targets.yaml).
- **Requirement refs.** `REL-SUPPORT-001`, `OPS-SUP-005`, `TOOL-EVT-001`,
  `GOV-EVID-901`, `GOV-DATA-002`, `REL-REPAIR-015`.
- **Accessibility hooks.** Packet preview must announce included
  classes, excluded classes, owner, and next expected step before any
  export action; redaction-required state must escalate via interruption
  tier `tier_actionable` from
  [`artifacts/ux/interruptibility_escalation_seed.yaml`](../../artifacts/ux/interruptibility_escalation_seed.yaml).
- **Forbidden shortcuts.** Mining additional evidence after preview
  approval; collapsing `attached_by_reference` and
  `saved_local_only` into one delivery state; allowing a packet to
  upload when redaction review is still pending.

## How surfaces consume this inventory

Every launch-critical surface in
[`/artifacts/ux/surface_traceability_matrix.yaml`](../../artifacts/ux/surface_traceability_matrix.yaml)
declares which pattern rows it must reuse via
[`pattern_surface_crosswalk.yaml`](../../artifacts/ux/pattern_surface_crosswalk.yaml).
Surface-local exceptions require a visible freeze exception or waiver
ref on the corresponding crosswalk row; downstream M1+ implementation
PRs cannot claim a surface-local exception silently.

## Companion references

- Architecture: [`docs/architecture/subsystem_contract_cards.md`](../architecture/subsystem_contract_cards.md).
- UX surface traceability: [`docs/ux/surface_traceability.md`](./surface_traceability.md).
- Component contract template: [`docs/ux/component_contract_template.md`](./component_contract_template.md).
- Verification packet template: [`docs/governance/verification_packet_template.md`](../governance/verification_packet_template.md).
- Review gate manifest: [`artifacts/ux/review_gate_manifest.yaml`](../../artifacts/ux/review_gate_manifest.yaml).
- Recovery ladder packet: [`docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md).
