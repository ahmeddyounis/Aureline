# Recovery-ladder packet: safe mode, suspect-extension quarantine, open-without-restore, cache/index repair, and restricted-mode fallback

This packet freezes one shared contract for Aureline's recovery ladder.
Each rung is a versioned decision object, not a free-text
troubleshooting suggestion. Every rung records its preconditions,
entry authority, preserved state, lost capability, reversal class, and
escalation triggers, and every rung binds to stable identifiers on the
support-bundle record, the object-issue handoff packet, and future
Project Doctor findings so support, export, and escalation flows can
cite recovery by id instead of quoting ad hoc advice.

If this packet, the
[`recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json)
schema, the
[`recovery_ladder_cases/`](../../fixtures/support/recovery_ladder_cases/)
fixture corpus, and the
[`recovery_examples/`](../../artifacts/support/recovery_examples/)
reviewer examples disagree, the frozen support-bundle contract, the
object-handoff contract, and the record-class registry win for tooling
and this packet plus its companion artifacts update in the same change.

## Companion artifacts

- [`/schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json)
  — boundary schema for `recovery_action_record` and
  `recovery_ladder_seed_case_record`. Reuses `recovery_rung_class`,
  `recovery_entry_reason_class`, and `recovery_exit_reason_class` from
  `schemas/support/support_bundle.schema.json` rather than minting a
  parallel vocabulary.
- [`/fixtures/support/recovery_ladder_cases/`](../../fixtures/support/recovery_ladder_cases/)
  — one seed case per required scenario: crash-loop safe mode,
  suspect-extension quarantine, open without restore, cache/index
  repair, and restricted-mode fallback.
- [`/artifacts/support/recovery_examples/`](../../artifacts/support/recovery_examples/)
  — reviewer-facing examples that make the preserved work, disabled
  capability, and escalation trigger obvious for each rung without
  restating the full schema.
- [`/docs/support/support_center_concept.md`](./support_center_concept.md)
  — product-facing Support Center concept that names the rungs this
  packet governs.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — rung and entry/exit vocabularies this packet re-exports and the
  `recovery_context` block bundles record against each rung entry.
- [`/docs/support/object_handoff_packet.md`](./object_handoff_packet.md)
  and
  [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json)
  — `evidence_and_recovery_context.recovery_rung_class`,
  `repair_transaction_refs`, and `checkpoint_refs` fields the handoff
  packet cites when a recovery rung produced the escalation.
- [`/docs/support/project_doctor_packet.md`](./project_doctor_packet.md)
  and
  [`/fixtures/support/scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
  — scenario and repair-class vocabulary that maps each Doctor finding
  to exactly one rung in this packet.
- [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support-packet family registry; the rung decision object belongs
  to the `rollback_review` family for export and to
  `object_issue_handoff` when a rung entry triggered an escalation.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` §10.15 (diagnostics), §10.22 (support
  export), and §10.23 (recovery ladder).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §8.10 (fault
  domain and supervisor), §24.2.2 (recovery rungs), §24.2.3 (checkpoint
  and reversal), §24.4 (repair preview), and Appendix I (support
  packet posture).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §22.20 (Support Center)
  and §23.26 (Doctor surface).
- `.t2/docs/Aureline_Milestones_Document.md` §3.20 (supportability),
  §3.21 (evidence), and §7.4 (blocked-user recovery).

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: recovery_ladder_packet
packet_id: support.recovery_ladder.seed
evidence_id: evidence.support.recovery_ladder.packet
title: Recovery-ladder packet covering safe mode, suspect-extension quarantine, open-without-restore, cache/index repair, and restricted-mode fallback
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - REL-SUPPORT-001
    - REL-REPAIR-015
    - OPS-SUP-005
    - GOV-EVID-901
    - GOV-CORPUS-901
  claim_row_refs:
    - packet_row:recovery_ladder.rung_decision_object
    - packet_row:recovery_ladder.reversal_class_honesty
    - packet_row:recovery_ladder.preserved_state_contract
    - packet_row:recovery_ladder.lost_capability_contract
    - packet_row:recovery_ladder.escalation_trigger_contract
    - packet_row:recovery_ladder.support_bundle_linkage
    - packet_row:recovery_ladder.issue_handoff_linkage
    - packet_row:recovery_ladder.project_doctor_linkage
  covered_lanes:
    - support_export
    - release_evidence
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: recovery_ladder_seed@1
  trigger_revision: recovery_ladder_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen support-bundle recovery-rung vocabulary,
    the object-handoff evidence-and-recovery fields, and the record-
    class registry already landed in this repository. No live recovery
    supervisor, repair-preview transaction engine, or Project Doctor
    runtime is wired to this packet yet. Claims are structural: every
    rung row reuses existing frozen tokens and adds the reversal,
    preserved-state, lost-capability, and escalation-trigger
    vocabularies this milestone seeds.
artifact_links:
  supporting_evidence_ids:
    - evidence.support.recovery_ladder.seed
    - evidence.support.project_doctor.scenario_matrix
    - evidence.support.support_bundle_contract
    - evidence.support.object_handoff_packet
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/support/recovery_ladder_cases/
    - fixtures/support/scenario_matrix.yaml
    - fixtures/support/support_bundle_examples/recovery_ladder_remote_connector_loss.json
    - fixtures/support/escalation_packet_completeness_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/support/support_bundle_contract.md
    - docs/support/object_handoff_packet.md
    - docs/support/support_center_concept.md
    - docs/support/project_doctor_packet.md
    - schemas/support/support_bundle.schema.json
    - schemas/support/object_handoff_packet.schema.json
    - schemas/support/recovery_action.schema.json
    - schemas/support/support_packet_index.schema.json
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one `recovery_action_record` shape every rung emits, with stable
  preconditions, authority, preserved-state set, lost-capability set,
  reversal class, entry/exit reasons, escalation triggers, linkage
  bindings, and reviewer-facing explanation fields that the Support
  Center and Doctor surfaces MAY render verbatim;
- one closed `reversal_class` vocabulary pinned to the five reversal
  semantics required by the milestone: exact undo, compensating
  action, regeneration, checkpoint restore, and no-undo/export-only;
- one closed `authority_class`, `preserved_state_class`,
  `lost_capability_class`, and `escalation_trigger_class` set so
  recovery narrating never collapses back into generic prose;
- one closed `linkage_requirement_class` set binding every rung to the
  support-bundle record, the object-handoff packet, and the future
  Project Doctor finding record so recovery steps keep stable IDs and
  explanation fields across surfaces; and
- one seeded case per required scenario — crash-loop safe mode,
  suspect-extension quarantine, open without restore, cache/index
  repair, and restricted-mode fallback — shaped so Support/export
  flows reference recovery step ids instead of free-text troubleshooting
  advice.

It does not claim a live recovery supervisor, a live repair-preview
transaction engine, or a hosted escalation portal is wired up. It
claims only that the rung decision object, the reversal-class honesty
rules, and the linkage rules to support bundles, issue handoff, and
future Project Doctor findings now exist in one reviewable form and
reuse the frozen support vocabulary already landed in this repository.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:recovery_ladder.rung_decision_object` | `REL-SUPPORT-001`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.support.recovery_ladder.seed` | Each rung is one versioned `recovery_action_record`, not free-text advice. |
| `packet_row:recovery_ladder.reversal_class_honesty` | `REL-REPAIR-015` | `seed_only` | `internal` | `evidence.support.recovery_ladder.seed` | Closed reversal-class set distinguishes exact undo, compensation, regeneration, checkpoint restore, and no-undo without overstating reversibility. |
| `packet_row:recovery_ladder.preserved_state_contract` | `REL-SUPPORT-001`, `REL-REPAIR-015` | `seed_only` | `internal` | `evidence.support.recovery_ladder.seed` | Every rung names the state classes it MUST NOT mutate so review knows what work is preserved. |
| `packet_row:recovery_ladder.lost_capability_contract` | `REL-SUPPORT-001`, `REL-REPAIR-015` | `seed_only` | `internal` | `evidence.support.recovery_ladder.seed` | Every rung names the capabilities it narrows so review knows what is disabled while the rung is active. |
| `packet_row:recovery_ladder.escalation_trigger_contract` | `REL-SUPPORT-001`, `OPS-SUP-005` | `seed_only` | `internal` | `evidence.support.recovery_ladder.seed` | Closed escalation-trigger set routes the user to export instead of silently widening in place. |
| `packet_row:recovery_ladder.support_bundle_linkage` | `REL-SUPPORT-001`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.support.support_bundle_contract` | Every rung binds to `support_bundle_record.recovery_context.current_rung_class` and at least one `repair_transaction_ref` or `checkpoint_ref` when the rung produced a transaction. |
| `packet_row:recovery_ladder.issue_handoff_linkage` | `REL-SUPPORT-001`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.support.object_handoff_packet` | Every rung binds to `object_handoff_packet_record.evidence_and_recovery_context.recovery_rung_class`; escalation triggers route through the handoff packet, not through prose. |
| `packet_row:recovery_ladder.project_doctor_linkage` | `REL-SUPPORT-001`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.support.project_doctor.scenario_matrix` | Every rung names the Project Doctor finding code it is the default rung for, so Doctor findings and recovery actions agree on identity and explanation fields. |

## Recovery-action contract

Every row in
[`recovery_ladder_cases/`](../../fixtures/support/recovery_ladder_cases/)
projects onto one `recovery_action_record` and freezes the following
fields. Field names and tokens MUST match
`schemas/support/recovery_action.schema.json` exactly.

- `recovery_action_id` — stable dotted id (for example
  `recovery_action:safe_mode.crash_loop_entry`). Ids are additive-only;
  repurposing is breaking.
- `rung_class` — one of the eight tokens re-exported from the frozen
  support-bundle vocabulary: `safe_mode`, `extension_bisect`,
  `extension_quarantine`, `open_without_restore`,
  `cache_reset_candidate`, `restricted_reopen`,
  `rollback_reinstall_candidate`, `typed_repair_flow`.
- `preconditions` — one or more reviewable sentences naming the
  condition that MUST hold before entry. Each sentence SHOULD quote a
  frozen condition token from the support-bundle
  `recovery_entry_reason_class` vocabulary
  (`repeated_startup_failure`, `explicit_user_choice`,
  `extension_regression_suspected`, `crash_loop_detected`,
  `cache_integrity_failure`, `route_recovery_required`,
  `policy_blocked`, `manual_review_required`).
- `authority_class` — who may enter the rung. Drawn from the closed
  set below.
- `preserved_state_classes` — state classes the rung MUST NOT mutate.
  `user_authored_files` appears on every rung by rule.
- `lost_capability_classes` — capabilities the rung disables or
  narrows while active.
- `reversal_class` — single token describing how the rung can be
  backed out. See the closed vocabulary below.
- `exit_reason_classes` — the exit-reason tokens the rung may emit
  when leaving; all drawn from the frozen support-bundle vocabulary.
- `escalation_trigger_classes` — one or more tokens describing the
  conditions that MUST route the user to an escalation packet instead
  of widening locally.
- `linkage_bindings` — at least one stable binding onto the
  support-bundle record, the object-handoff packet, a Project Doctor
  finding, a repair-transaction ref, a checkpoint ref, a crash envelope
  ref, or a known-limit ref.
- `default_redaction_choice_class` — default redaction posture the
  rung hands off to support preview, re-exported from the frozen
  `object_handoff_packet_record.redaction_choice_class` vocabulary.
- `preview_required` — boolean. `true` means the rung MUST run through
  the review → preview → checkpoint → apply → verify →
  rollback/compensate transaction grammar before touching state.
- `observe_only_mode_supported` — boolean. `true` means Doctor surfaces
  MAY cite the rung in observe-only mode without proposing a write.
- `explanation_fields` — four reviewable sentences (preserved work
  summary, disabled capability summary, escalation summary, user-
  facing next step) that Support Center, Doctor, and export preview
  MAY render verbatim. These are the stable strings that replace
  free-text troubleshooting advice.

### `reversal_class` (frozen)

Five reversal semantics, mutually exclusive. A row that cannot cite
one of these MUST NOT claim an undo path.

| Token | Meaning |
|---|---|
| `exact_undo` | Prior state is bit-identical after exit. Only disposable derived state changed (for example a watcher restart that reseeds without touching user files). |
| `compensating_action` | Prior state is semantically equivalent after exit even though exact bytes may differ (for example a reapproval flow that restores trust posture without minting a new ticket). |
| `regeneration` | A disposable derived artifact is rebuilt from authoritative sources (for example a cache/index rebuild after an integrity failure). |
| `checkpoint_restore` | An explicit pre-entry checkpoint is restored. The rung MUST bind a `checkpoint_ref` in its `linkage_bindings`. |
| `no_undo_export_only` | No local reversal path exists. The rung MUST list `no_local_repair_path_available` in its `escalation_trigger_classes` and MUST default to an escalation packet. |

Rule: a rung whose `reversal_class = no_undo_export_only` MUST NOT
advertise an exact rollback in its explanation fields. A rung whose
`reversal_class = checkpoint_restore` MUST bind `checkpoint_ref` in
`linkage_bindings`; the schema enforces this.

### `authority_class` (frozen)

| Token | Meaning |
|---|---|
| `user_opt_in_only` | Rung entry requires an explicit user action. |
| `supervisor_auto_enter_bounded` | Supervisor may enter after a bounded strike budget or crash-loop threshold. |
| `policy_forced_entry` | Managed policy forced a narrower posture. |
| `admin_only_entry` | A managed-admin surface authored the entry. |
| `co_signed_user_plus_supervisor` | User and supervisor must both agree before entry. |

### `preserved_state_class` (frozen)

| Token | Meaning |
|---|---|
| `user_authored_files` | Authored buffers, workspace files, and VCS state. Required on every rung. |
| `open_buffer_selection` | Active tab, cursor, and selection state. |
| `durable_workspace_indexes` | Authoritative workspace graph and search indexes the user depends on across sessions. |
| `workspace_trust_store` | Trust posture and approval tickets. |
| `credential_store` | Secret broker material and credential handles. |
| `remote_helper_authorization` | Remote helper/attach authorization tokens and approval state. |
| `managed_policy_overrides` | Admin-authored policy rows. |
| `session_restore_store` | User-owned session restore state. |
| `support_export_store` | Support-bundle exports and in-flight evidence. |
| `docs_pack_authoring` | Docs authoring (only governed mirror snapshots may be refreshed). |

### `lost_capability_class` (frozen)

| Token | Meaning |
|---|---|
| `extension_auto_activation` | Extensions do not auto-activate. |
| `extension_host_launch` | Extension host does not launch at all. |
| `session_restore_auto_reopen` | Previously open buffers do not auto-reopen. |
| `remote_helper_attach` | Remote helper/attach is suspended. |
| `docs_pack_live_fetch` | Docs pack and mirror live fetch is suspended. |
| `background_rebuild` | Background rebuilds of derived indexes/caches are paused. |
| `ai_runtime_access` | AI-runtime brokers are disabled. |
| `telemetry_upload` | Diagnostic upload is suspended. |
| `managed_control_plane_writes` | Managed control-plane writes are disabled. |
| `workspace_trust_widening` | Trust widening flows are disabled. |

### `escalation_trigger_class` (frozen)

| Token | Meaning |
|---|---|
| `repeated_entry_within_budget` | Rung has been re-entered beyond its strike budget. |
| `user_denied_next_rung` | User denied the proposed next rung; escalate instead of widening. |
| `exact_undo_unavailable` | The rung entered while expecting exact undo and that expectation no longer holds. |
| `policy_forced_narrower_than_user_request` | Managed policy forced a narrower posture than the user requested. |
| `lost_capability_blocks_current_task` | A disabled capability is required for the user's current task. |
| `rung_exit_reason_denied_by_policy` | Exit attempt produced a `denied_by_policy` reason. |
| `diagnosis_remains_insufficient` | Doctor confidence remains `unknown_requires_probe` after the rung's diagnosis window. |
| `no_local_repair_path_available` | Required on every `no_undo_export_only` rung. |

### `linkage_requirement_class` (frozen)

Every `recovery_action_record` MUST carry at least one linkage
binding drawn from the following set:

- `support_bundle_record` — pointer to
  `support_bundle_record.recovery_context.current_rung_class`.
- `object_handoff_packet_record` — pointer to
  `object_handoff_packet_record.evidence_and_recovery_context.recovery_rung_class`.
- `project_doctor_finding` — pointer to the Doctor
  `finding_code` (`doctor.finding.*`) this rung is the default rung
  for.
- `repair_transaction_ref` — opaque id of the repair transaction the
  rung wraps.
- `checkpoint_ref` — opaque id of the pre-entry checkpoint the rung
  restored or wrote.
- `crash_envelope_ref` — opaque id of the crash envelope that drove
  entry.
- `known_limit_ref` — known-limit row the rung cites instead of
  inventing a "this does not work yet" comment.

## Linkage rules

The recovery ladder is not a standalone artifact. It MUST compose
with the three support packet families already frozen in this
repository:

1. **Support bundle** — every rung entry a bundle captures appears in
   `support_bundle_record.recovery_context.rung_history` with
   `rung_class`, `entered_at`, `entry_reason_class`, `exited_at`,
   `exit_reason_class`, `changed_storage_class_ids`,
   `repair_transaction_ref`, and `checkpoint_ref` populated. The
   bundle's `current_rung_class` MUST match the rung the export is
   produced from.
2. **Object-issue handoff** — when a rung triggers an escalation, the
   exported `object_handoff_packet_record` MUST carry
   `evidence_and_recovery_context.recovery_rung_class` equal to the
   rung's `rung_class`, and MUST populate
   `repair_transaction_refs` and `checkpoint_refs` whenever the rung
   wrote a transaction or checkpoint. The packet's
   `redaction_choice_class` MUST equal the rung's
   `default_redaction_choice_class` unless a consent marker widens it.
3. **Project Doctor finding** — every Doctor finding whose suggested
   repair class routes through a rung MUST cite that rung's
   `recovery_action_id` in its `recovery_rung_class` projection. A
   Doctor finding that cannot name a rung MUST fall back to
   `defer_to_escalation_packet` with
   `rollback_expectation_class = no_exact_rollback_prefer_export`.

Rule: a support/export flow MUST cite a rung by its stable
`recovery_action_id` (and by its `rung_class` token) rather than by
free-text troubleshooting advice. A bundle or handoff packet that
advertises recovery in narrative without naming a rung id is
non-conforming.

## Seeded cases

The seed corpus in
[`recovery_ladder_cases/`](../../fixtures/support/recovery_ladder_cases/)
covers the five scenarios the milestone requires. Every case row
binds 1:1 to a `recovery_action_record` and names its default
reversal class, preserved state, lost capability, and escalation
trigger.

| Case id | Rung | Reversal class | Default redaction | Preview required |
|---|---|---|---|---|
| `recovery_case:safe_mode.crash_loop_entry` | `safe_mode` | `checkpoint_restore` | `metadata_only_default` | `true` |
| `recovery_case:extension_quarantine.suspect_host_regression` | `extension_quarantine` | `compensating_action` | `metadata_only_default` | `true` |
| `recovery_case:open_without_restore.session_restore_declined` | `open_without_restore` | `exact_undo` | `metadata_only_default` | `false` |
| `recovery_case:cache_reset_candidate.cache_index_repair` | `cache_reset_candidate` | `regeneration` | `metadata_only_default` | `true` |
| `recovery_case:restricted_reopen.managed_fallback` | `restricted_reopen` | `no_undo_export_only` | `support_bundle_by_reference` | `false` |

Every case also names the Project Doctor `finding_code` it is the
default rung for, and lists the support-bundle case ref, the
escalation-packet completeness case ref, and (where applicable) the
companion observe-only variant so Doctor can render the rung without
proposing a write.

## Reviewer examples

The
[`recovery_examples/`](../../artifacts/support/recovery_examples/)
directory seeds one reviewer-facing example per rung. Examples are
deliberately shaped so support review can read three things in five
lines: what work is preserved, what capability is disabled, and when
escalation is required. Examples quote the rung's
`recovery_action_id` and the stable tokens from the closed
vocabularies above; they are not prose substitutes for the packet.

## What this seed does not promise

- No live recovery supervisor, repair-preview transaction engine, or
  Project Doctor runtime is wired to this packet. The decision
  object, the reversal honesty rules, and the linkage rules are
  reviewable objects only.
- No checkpoint implementation, storage-reset flow, or extension-
  quarantine supervisor lands in this milestone.
- No numeric strike-budget value is committed. Budgets remain
  `to_be_set_by_benchmark_council` via the existing scoreboard.
- No schema changes to `support_bundle.schema.json` or
  `object_handoff_packet.schema.json` are required; this packet
  projects onto existing vocabularies and adds the
  `recovery_action_record` and `recovery_ladder_seed_case_record`
  shapes in a new schema rather than mutating landed ones.
- Adding a new `recovery_action_record` row under an existing rung is
  additive-minor provided it reuses the frozen `rung_class`,
  `reversal_class`, `authority_class`, `preserved_state_class`,
  `lost_capability_class`, `escalation_trigger_class`, and
  `linkage_requirement_class` vocabularies. Repurposing any of those
  tokens is breaking and requires a new decision row in
  `artifacts/governance/decision_index.yaml`.
