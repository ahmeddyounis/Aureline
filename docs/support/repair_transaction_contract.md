# Repair-transaction, preview/apply/rollback, and escalation-packet contract

This packet freezes one shared contract for every repair Aureline takes
on behalf of a user. Project Doctor's "Fix" affordance, the Support
Center's repair preview, support-initiated repair commands, and the
recovery-ladder typed_repair_flow rung all run through the same typed
repair transaction defined here. A transaction is a versioned decision
object, not a free-text Fix-button action: every transaction names its
preconditions, the finding codes that initiated it, the state classes
it MAY mutate, the state classes it MUST NOT mutate, the online/offline
and trust/policy gates apply MUST satisfy, the checkpoint it writes
(or the explicit absence of one), an idempotency key, the reversal
class it claims, the preview artifact reviewers MUST see before apply,
and the closed forbidden-action set the transaction asserts it WILL
NOT take.

If this packet, the
[`repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json)
schema, the
[`repair_preview.schema.json`](../../schemas/support/repair_preview.schema.json)
schema, the
[`repair_outcome.schema.json`](../../schemas/support/repair_outcome.schema.json)
schema, and the
[`repair_cases/`](../../fixtures/support/repair_cases/)
fixture corpus disagree, the frozen support-bundle contract, the
object-handoff contract, the recovery-ladder contract, the project-
doctor scenario matrix, and the record-class registry win for tooling
and this packet plus its companion artifacts update in the same change.

## Companion artifacts

- [`/schemas/support/repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json)
  — boundary schema for `repair_transaction_record` and
  `repair_seed_case_record`.
- [`/schemas/support/repair_preview.schema.json`](../../schemas/support/repair_preview.schema.json)
  — boundary schema for `repair_preview_record`. Every transaction
  MUST own one preview record before apply.
- [`/schemas/support/repair_outcome.schema.json`](../../schemas/support/repair_outcome.schema.json)
  — boundary schema for `repair_outcome_record`. Every apply path
  (including refused-apply and escalation-only outcomes) MUST emit one.
- [`/fixtures/support/repair_cases/`](../../fixtures/support/repair_cases/)
  — one seed case per required scenario family: cache/index rebuild,
  extension quarantine, toolchain re-resolve, remote agent rollback,
  policy refresh, and escalation-only packet.
- [`/docs/support/project_doctor_packet.md`](./project_doctor_packet.md)
  and
  [`/fixtures/support/scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
  — finding-code, probe-family, and `suggested_repair_class` vocabulary
  every transaction cites by stable id.
- [`/docs/support/recovery_ladder_packet.md`](./recovery_ladder_packet.md)
  and
  [`/schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json)
  — rung-class, reversal-class, preserved-state, lost-capability, and
  escalation-trigger vocabulary this packet re-exports rather than
  re-mints.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — `recovery_context.repair_transaction_refs` and
  `active_checkpoint_refs` fields the bundle binds against.
- [`/docs/support/object_handoff_packet.md`](./object_handoff_packet.md)
  and
  [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json)
  — `evidence_and_recovery_context.repair_transaction_refs` and
  `checkpoint_refs` fields the escalation packet preserves when a
  transaction was the cause for export.
- [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support-packet family registry; repair transactions belong to the
  `rollback_review` family for export and to `object_issue_handoff`
  whenever apply produced (or refused into) an escalation packet.

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
packet_family: repair_transaction_packet
packet_id: support.repair_transaction.seed
evidence_id: evidence.support.repair_transaction.packet
title: Repair-transaction, preview/apply/rollback, and escalation-packet contract
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
    - packet_row:repair_transaction.transaction_decision_object
    - packet_row:repair_transaction.preview_apply_rollback_grammar
    - packet_row:repair_transaction.checkpoint_and_idempotency
    - packet_row:repair_transaction.forbidden_action_contract
    - packet_row:repair_transaction.escalation_route_linkage
    - packet_row:repair_transaction.scenario_matrix_linkage
    - packet_row:repair_transaction.recovery_ladder_linkage
    - packet_row:repair_transaction.support_bundle_handoff_linkage
  covered_lanes:
    - support_export
    - release_evidence
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-26T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: repair_transaction_seed@1
  trigger_revision: repair_transaction_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen support-bundle, object-handoff, recovery-
    ladder, and project-doctor vocabularies already landed in this
    repository. No live repair engine, transaction supervisor, or
    Project Doctor runtime is wired to this packet yet. Claims are
    structural: every transaction row reuses existing frozen tokens and
    adds the repair-class-family, transaction-reversal-class, impacted-
    state, runtime-requirement, and forbidden-action vocabularies this
    milestone seeds.
artifact_links:
  supporting_evidence_ids:
    - evidence.support.repair_transaction.seed
    - evidence.support.recovery_ladder.seed
    - evidence.support.project_doctor.scenario_matrix
    - evidence.support.support_bundle_contract
    - evidence.support.object_handoff_packet
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/support/repair_cases/
    - fixtures/support/recovery_ladder_cases/
    - fixtures/support/scenario_matrix.yaml
    - fixtures/support/escalation_packet_completeness_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/support/project_doctor_packet.md
    - docs/support/recovery_ladder_packet.md
    - docs/support/support_bundle_contract.md
    - docs/support/object_handoff_packet.md
    - schemas/support/repair_transaction.schema.json
    - schemas/support/repair_preview.schema.json
    - schemas/support/repair_outcome.schema.json
    - schemas/support/recovery_action.schema.json
    - schemas/support/support_bundle.schema.json
    - schemas/support/object_handoff_packet.schema.json
    - schemas/support/support_packet_index.schema.json
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one `repair_transaction_record` shape every Doctor "Fix" action,
  Support Center repair preview, support-initiated repair command, and
  recovery-ladder `typed_repair_flow` rung emits, with a stable id,
  initiating finding codes, repair-class family, suggested repair
  class, impacted/preserved state classes, lost capabilities, runtime
  requirements (online/offline and trust/policy), checkpoint reference,
  idempotency key, reversal class, preview-artifact ref, apply-mode
  class, escalation route, linkage bindings, default redaction choice,
  and reviewer-facing explanation fields;
- one `repair_preview_record` shape every transaction owns before any
  apply path runs, with the preview-state class, the proposed
  checkpoint (or its explicit absence), the claimed reversal class, the
  preview-blocker set, and the per-class impacted-change and preserved-
  state assertion rows reviewers MUST see before authorizing apply;
- one `repair_outcome_record` shape every apply path emits, including
  pre-apply refusal paths, with the outcome class, the applied-change
  rows, the preserved-state classes the outcome verifies stayed
  untouched, the checkpoint that was used, the reversal class that
  executed (when a reversal ran), the failure-reason class (when
  apply failed), the runtime-requirements held block, the forbidden-
  action assertion result, the typed `remaining_unknowns`, and the
  escalation-packet ref the outcome routed to (when applicable);
- one closed `repair_class_family` vocabulary covering disposable-
  state rebuild, extension isolation, extension rollback/reinstall,
  execution-context re-resolve, remote/runtime repair, policy/
  entitlement refresh, guided export/escalation, and observe-only no-
  repair, ordered narrowest-safe-first;
- one closed `transaction_reversal_class` vocabulary pinned to the
  five reversal semantics required by the milestone — `exact`,
  `compensating`, `regenerate`, `manual`, and `audit_only` — so a
  Fix button never overstates how recoverable a repair is;
- one closed `forbidden_action_class` vocabulary every transaction
  asserts (and every outcome record verifies), pinned so apply paths
  cannot widen trust, publish routes, run repo hooks, silently rebind
  remote helpers, silently reinstall extensions, mutate managed policy,
  rewrite user-authored files, read or rotate credentials, auto-retarget
  without user choice, mutate authoritative profile state, embed raw
  secrets, or auto-widen redaction without explicit consent;
- one closed `apply_mode_class` and `preview_state_class` vocabulary
  so reviewers can see at a glance whether a transaction is in dry-run
  preview, apply-with-checkpoint, apply-with-rollback-on-failure,
  observe-only, or refused-escalation-only mode;
- one closed `linkage_requirement_class` set binding every transaction
  to the support-bundle record, the object-handoff packet, the recovery-
  action record, the Project Doctor finding, the repair-preview record,
  the repair-outcome record, the checkpoint ref, the crash envelope, the
  known-limit ref, and the scenario row in
  `fixtures/support/scenario_matrix.yaml`; and
- one seeded case per required scenario family — cache/index rebuild,
  extension quarantine, toolchain re-resolve, remote agent rollback,
  policy refresh, and escalation-only packet — shaped so Support and
  export flows reference repair transactions by stable id rather than
  free-text Fix-button copy.

It does not claim a live repair engine, a live transaction supervisor,
a live preview compiler, a live checkpoint store, or a hosted apply
runtime is wired up. It claims only that the repair-transaction
decision object, the preview/apply/rollback grammar, the forbidden-
action contract, and the escalation-packet linkage rules now exist in
one reviewable form and reuse the frozen support vocabulary already
landed in this repository.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:repair_transaction.transaction_decision_object` | `REL-REPAIR-015`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.support.repair_transaction.seed` | Each repair is one versioned `repair_transaction_record`, not a free-text Fix-button action. |
| `packet_row:repair_transaction.preview_apply_rollback_grammar` | `REL-REPAIR-015`, `REL-SUPPORT-001` | `seed_only` | `internal` | `evidence.support.repair_transaction.seed` | Every durable-state or policy-sensitive repair MUST run review → preview → checkpoint → apply → verify → rollback/compensate; the grammar is enforced by `apply_mode_class` and the preview/outcome schemas. |
| `packet_row:repair_transaction.checkpoint_and_idempotency` | `REL-REPAIR-015`, `OPS-SUP-005` | `seed_only` | `internal` | `evidence.support.repair_transaction.seed` | `checkpoint_ref` is required for `apply_with_checkpoint` and `apply_with_rollback_on_failure`; `idempotency_key` is required on every transaction so re-submission is a no-op. |
| `packet_row:repair_transaction.forbidden_action_contract` | `REL-REPAIR-015`, `REL-SUPPORT-001` | `seed_only` | `internal` | `evidence.support.repair_transaction.seed` | Every transaction asserts the closed `forbidden_action_class` set; outcomes verify the assertion held. |
| `packet_row:repair_transaction.escalation_route_linkage` | `REL-SUPPORT-001`, `OPS-SUP-005` | `seed_only` | `internal` | `evidence.support.object_handoff_packet`, `evidence.support.repair_transaction.seed` | When no safe local repair exists the transaction defaults to `guided_export_escalation`; the escalation route MUST cite an `escalation_packet_completeness_cases/*.yaml` template. |
| `packet_row:repair_transaction.scenario_matrix_linkage` | `REL-SUPPORT-001`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.support.project_doctor.scenario_matrix` | Every transaction cites at least one `doctor.finding.*` initiating finding code from `fixtures/support/scenario_matrix.yaml`. |
| `packet_row:repair_transaction.recovery_ladder_linkage` | `REL-SUPPORT-001`, `REL-REPAIR-015` | `seed_only` | `internal` | `evidence.support.recovery_ladder.seed` | Every transaction either binds a `recovery_action_id` (when it wraps a rung) or declares `recovery_action_id: null` with `repair_class_family = guided_export_escalation`. |
| `packet_row:repair_transaction.support_bundle_handoff_linkage` | `REL-SUPPORT-001`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.support.support_bundle_contract`, `evidence.support.object_handoff_packet` | Every applied transaction binds its `repair_transaction_id` and `checkpoint_ref` into `support_bundle_record.recovery_context` and into `object_handoff_packet_record.evidence_and_recovery_context`. |

## Repair-transaction contract

Every row in
[`repair_cases/`](../../fixtures/support/repair_cases/) projects onto
one `repair_seed_case_record`, and every live repair the implementation
emits projects onto one `repair_transaction_record`. Field names and
tokens MUST match
[`repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json)
exactly.

### Required fields

- `repair_transaction_id` — stable `repair_transaction:<family>.<reason>`
  id (for example `repair_transaction:disposable_state_rebuild.cache_index_repair`).
  Ids are additive-only; repurposing is breaking.
- `repair_class_family` — one of the closed family tokens below.
- `suggested_repair_class` — fine-grained class drawn from the project-
  doctor `suggested_repair_class` vocabulary, extended with three
  transaction-level tokens (`rollback_or_reinstall_extension`,
  `rollback_remote_runtime`, `refresh_policy_entitlement`) the milestone
  requires.
- `initiating_finding_codes` — one or more
  `doctor.finding.*` codes from `fixtures/support/scenario_matrix.yaml`.
  At least one code is required.
- `impacted_state_classes` — closed list of state classes apply MAY
  mutate. Empty when `apply_mode_class` is `dry_run_preview_only`,
  `apply_observe_only_no_write`, or `apply_refused_escalation_only`.
- `preserved_state_classes` — closed list of state classes apply MUST
  NOT mutate. `user_authored_files` MUST appear on every transaction
  by rule.
- `lost_capability_classes` — capabilities apply narrows while in
  flight, drawn from the recovery-ladder `lost_capability_class`
  vocabulary.
- `runtime_requirements` — `online_offline_requirement_class`,
  `trust_policy_requirement_class`, `requires_active_user_consent`, and
  `requires_active_admin_consent`. The block fixes the connectivity,
  trust, and consent gates apply MUST satisfy.
- `forbidden_action_assertions` — closed list of `forbidden_action_class`
  tokens the transaction asserts it WILL NOT take. By rule the list
  MUST include at least `widen_workspace_trust`, `publish_route`,
  `run_repo_hook_silently`, `mutate_user_authored_files`, and
  `read_or_rotate_credentials`. Additional tokens apply per repair-class
  family (see the family rules below).
- `checkpoint_ref` — opaque pre-apply checkpoint id. Required when
  `apply_mode_class` is `apply_with_checkpoint` or
  `apply_with_rollback_on_failure`. Null otherwise.
- `idempotency_key` — stable token. Re-submitting the same
  `(repair_transaction_id, initiating_finding_codes, idempotency_key)`
  triple MUST be a no-op apply; preview MAY still run.
- `transaction_reversal_class` — single token from the closed five-
  token vocabulary (see below).
- `preview_artifact_ref` — stable `preview_id` of the
  `repair_preview_record` this transaction owns. The preview record
  MUST exist before any apply path runs.
- `apply_mode_class` — single token (see below).
- `escalation_route` — `escalation_required_when` (one or more
  `escalation_trigger_class` tokens), `default_handoff_packet_template_ref`
  (path to one `fixtures/support/escalation_packet_completeness_cases/*.yaml`
  case), and `default_redaction_choice_class`.
- `linkage_bindings` — one or more typed bindings (see linkage rules
  below). Every applied transaction MUST bind at least one
  `support_bundle_record` linkage and at least one
  `repair_preview_record` linkage.
- `default_redaction_choice_class` — re-exported from the object-
  handoff vocabulary.
- `explanation_fields` — five reviewable sentences (preserved work,
  change summary, capability disablement, escalation summary, next
  step). The Support Center, Doctor, and export preview surfaces MAY
  render these verbatim.

Rule: a transaction whose
`repair_class_family = guided_export_escalation` MUST set
`apply_mode_class = apply_refused_escalation_only`,
`transaction_reversal_class = audit_only`, `checkpoint_ref = null`,
and `impacted_state_classes = []`. The schema enforces this.

Rule: a transaction whose
`repair_class_family = observe_only_no_repair` MUST set
`apply_mode_class = apply_observe_only_no_write`,
`transaction_reversal_class = audit_only`, `checkpoint_ref = null`,
and `impacted_state_classes = []`.

Rule: `transaction_reversal_class = exact` is only allowed when
`apply_mode_class` is `apply_with_checkpoint` or
`apply_with_rollback_on_failure`. An exact-undo claim that is not
backed by a checkpoint is non-conforming.

### `repair_class_family` (frozen, ordered narrowest-safe-first)

| Token | Job | Default reversal | Default apply mode |
|---|---|---|---|
| `observe_only_no_repair` | Doctor reports the finding without proposing a write. | `audit_only` | `apply_observe_only_no_write` |
| `disposable_state_rebuild` | Rebuild a disposable derived cache, watcher backlog, mirror snapshot, or other disposable derived artifact named in the finding. | `regenerate` | `apply_with_checkpoint` |
| `extension_isolation` | Quarantine a suspect extension and offer bisect; never auto-reenable. | `compensating` | `apply_with_rollback_on_failure` |
| `extension_rollback_reinstall` | Roll back or reinstall an extension to a prior good version; requires explicit user consent. | `manual` | `apply_with_rollback_on_failure` |
| `execution_context_reresolve` | Re-resolve toolchain, language server, or other execution-context handle against the declared target; never rewrite user source. | `compensating` | `apply_with_rollback_on_failure` |
| `remote_runtime_repair` | Restart, redeploy, or roll back a remote helper or remote agent runtime; never silently rebind a session. | `compensating` | `apply_with_rollback_on_failure` |
| `policy_entitlement_refresh` | Refresh trust posture, approval ticket, or policy/entitlement state; never widen trust beyond the user's request. | `compensating` | `apply_with_rollback_on_failure` |
| `guided_export_escalation` | No safe local repair exists. The transaction prepares an escalation packet instead of applying. | `audit_only` | `apply_refused_escalation_only` |

Cross-surface copy, preview minima, checkpoint expectations, and marketing/support caveats are frozen in:
[`/docs/recovery/repair_class_matrix.md`](../recovery/repair_class_matrix.md) and
[`/artifacts/recovery/repair_classes.yaml`](../../artifacts/recovery/repair_classes.yaml).

Rule: every `extension_rollback_reinstall` transaction MUST set
`runtime_requirements.requires_active_user_consent = true` and
include `silent_extension_reinstall` in `forbidden_action_assertions`.

Rule: every `remote_runtime_repair` transaction MUST set
`runtime_requirements.online_offline_requirement_class = requires_online`
and include `silent_helper_rebind` in `forbidden_action_assertions`.

Rule: every `policy_entitlement_refresh` transaction MUST include
`mutate_managed_policy` in `forbidden_action_assertions` and MUST set
`trust_policy_requirement_class` to either
`requires_explicit_user_consent_no_trust_widen` or
`requires_admin_authored_policy_unchanged`.

### `transaction_reversal_class` (frozen)

| Token | Meaning |
|---|---|
| `exact` | Prior state is bit-identical after reversal. Only disposable state changed (for example a watcher restart that reseeds without touching user files). Allowed only with `apply_with_checkpoint` or `apply_with_rollback_on_failure`. |
| `compensating` | Prior state is semantically equivalent after reversal even though exact bytes may differ (for example a reapproval flow that restores trust posture without minting a new ticket). |
| `regenerate` | A disposable derived artifact is rebuilt from authoritative sources (for example a cache/index rebuild after an integrity failure). |
| `manual` | Reversal requires a user-driven action (for example reinstalling an extension by hand, or accepting a managed reapproval flow). The transaction MUST publish reversal instructions in `explanation_fields.user_facing_next_step`. |
| `audit_only` | No state was changed. The transaction wrote only audit records, refused to apply, or completed in observe-only mode. Required for `guided_export_escalation` and `observe_only_no_repair` transactions. |

Rule: `transaction_reversal_class` and `apply_mode_class` are coupled.
A `manual` reversal claim alongside `apply_observe_only_no_write` is
non-conforming because nothing changed; a `regenerate` claim alongside
`dry_run_preview_only` is non-conforming because nothing was rebuilt.

### `apply_mode_class` (frozen)

| Token | Meaning |
|---|---|
| `dry_run_preview_only` | Preview runs; apply never runs. The preview record is the deliverable. `checkpoint_ref` MUST be null. |
| `apply_with_checkpoint` | A pre-apply checkpoint is captured, then apply runs. `checkpoint_ref` MUST be non-null. |
| `apply_with_rollback_on_failure` | A pre-apply checkpoint is captured; apply runs; the checkpoint is restored on failure. `checkpoint_ref` MUST be non-null. |
| `apply_observe_only_no_write` | The transaction emits audit records only; no state mutation. Required for `observe_only_no_repair`. |
| `apply_refused_escalation_only` | The transaction refuses to apply at all and produces an escalation packet. Required for `guided_export_escalation`. |

### `forbidden_action_class` (frozen)

The forbidden-action vocabulary is the contract that "Fix" cannot
silently widen safety. Every transaction asserts the closed list of
actions it WILL NOT take; the outcome record verifies the assertion
held. By rule every transaction MUST assert at least:

- `widen_workspace_trust` — never silently widen trust posture or mint
  approval tickets the user did not request.
- `publish_route` — never publish a route, command surface, or remote
  endpoint without explicit user/admin authoring.
- `run_repo_hook_silently` — never invoke repo hooks, lifecycle
  scripts, or build scripts without explicit user consent.
- `mutate_user_authored_files` — never rewrite authored buffers under
  any repair class.
- `read_or_rotate_credentials` — never read, copy, or rotate secrets
  during repair.

Family-specific assertions add to the list:

| Family | Additional required assertions |
|---|---|
| `extension_rollback_reinstall` | `silent_extension_reinstall` |
| `remote_runtime_repair` | `silent_helper_rebind`, `auto_retarget_without_user` |
| `policy_entitlement_refresh` | `mutate_managed_policy` |
| `guided_export_escalation` | `auto_widen_redaction_choice`, `embed_raw_secret_in_export` |
| `disposable_state_rebuild` | `mutate_authoritative_profile_store` |

### `online_offline_requirement_class` and `trust_policy_requirement_class` (frozen)

`online_offline_requirement_class`:

- `requires_online` — apply MUST have working network paths.
- `prefers_online_supports_offline_observe` — apply needs network;
  observe-only mode works offline.
- `supports_offline_local_only` — apply works offline.
- `requires_offline_local_only` — apply MUST refuse to call any
  remote service.

`trust_policy_requirement_class`:

- `no_trust_or_policy_change` — the transaction does not touch trust
  or policy state at all.
- `requires_existing_trust_unchanged` — apply depends on the current
  trust posture and refuses if trust drifts mid-transaction.
- `requires_explicit_user_consent_no_trust_widen` — apply needs an
  explicit user consent marker but MUST NOT widen trust beyond the
  user's request.
- `requires_admin_authored_policy_unchanged` — apply refuses if
  managed policy changed since preview.
- `requires_managed_admin_consent` — a managed admin surface MUST
  sign off before apply. Forces
  `runtime_requirements.requires_active_admin_consent = true`.

## Repair-preview contract

[`repair_preview.schema.json`](../../schemas/support/repair_preview.schema.json)
binds the `repair_preview_record` shape. Every repair transaction MUST
own one preview before any apply path runs.

The preview record carries:

- `preview_state_class` — outcome of the preview pass. The closed set:
  `dry_run_complete_pending_review`, `dry_run_safe_apply_authorized`,
  `dry_run_blocked_by_policy`, `dry_run_refused_widens_trust`,
  `dry_run_refused_publishes_route`,
  `dry_run_refused_runs_repo_hook_silently`,
  `dry_run_refused_mutates_user_files`,
  `dry_run_refused_reads_or_rotates_credentials`,
  `dry_run_refused_auto_retarget`, and
  `escalation_only_no_preview_apply`.
- `claimed_reversal_class` — the reversal class apply will execute on
  failure. MUST equal `audit_only` for any `dry_run_refused_*` or
  `escalation_only_no_preview_apply` state.
- `checkpoint_proposal` — `checkpoint_class`, opaque `checkpoint_ref`
  (or null), `capture_summary`, and `scope_state_classes`. Schema
  classes: `ephemeral_pre_apply`, `durable_pre_apply`,
  `no_checkpoint_observe_only`, `no_checkpoint_escalation_only`, and
  `checkpoint_capture_refused`.
- `runtime_requirements` — mirrors the transaction.
- `forbidden_action_assertions` — mirrors the transaction. The preview
  is the surface that detects whether apply WOULD cross a forbidden
  boundary; every preview MUST assert at least
  `widen_workspace_trust`, `publish_route`, `run_repo_hook_silently`,
  and `mutate_user_authored_files`.
- `impacted_change_rows` — per-class rows describing what changes if
  apply runs. Empty for any `dry_run_refused_*` or
  `escalation_only_no_preview_apply` state.
- `preserved_assertion_rows` — per-class rows describing what does NOT
  change. `user_authored_files` MUST appear in this list by rule.
- `lost_capability_classes` — capabilities apply narrows while in
  flight.
- `preview_blockers` — closed reasons preview refuses to authorize
  apply: `missing_finding_evidence`,
  `finding_confidence_insufficient`, `no_safe_local_repair_available`,
  `policy_blocks_action`, `online_runtime_unavailable`,
  `trust_state_below_floor`, `idempotency_key_collision`,
  `checkpoint_capture_unavailable`, `rollback_target_unavailable`,
  `managed_admin_consent_required`, and
  `managed_policy_changed_during_preview`.
- `idempotency_key` — mirrors the transaction; preview is the surface
  that detects collisions.
- `explanation_fields` — six reviewable sentences (preserved work,
  change summary, checkpoint summary, reversal summary, escalation
  summary, next step). The Support Center MAY render these verbatim.

Rule: a "Fix" button MUST NOT advance to apply unless
`preview_state_class = dry_run_safe_apply_authorized`. Any other
preview state forces either further review or escalation.

## Repair-outcome contract

[`repair_outcome.schema.json`](../../schemas/support/repair_outcome.schema.json)
binds the `repair_outcome_record` shape. Every apply path emits one,
including pre-apply refusal paths.

The outcome record carries:

- `outcome_class` — closed result token. The set:
  `preview_only_no_apply`, `applied_success_recovered`,
  `applied_partial_recovered_with_typed_unknowns`,
  `applied_failed_rolled_back`, `applied_failed_compensated`,
  `applied_failed_no_reversal_export_only`, `escalated_no_apply`,
  `refused_pre_apply_widens_trust`, `refused_pre_apply_publishes_route`,
  `refused_pre_apply_runs_repo_hook_silently`,
  `refused_pre_apply_silent_helper_rebind`,
  `refused_pre_apply_silent_extension_reinstall`,
  `refused_pre_apply_mutates_user_files`,
  `refused_pre_apply_reads_or_rotates_credentials`,
  `refused_pre_apply_managed_policy_violation`,
  `refused_pre_apply_idempotency_key_collision`, and
  `refused_pre_apply_runtime_requirement_unmet`.
- `applied_change_rows` — per-class rows describing what mutated.
  Empty for `preview_only_no_apply`, `escalated_no_apply`,
  `applied_failed_rolled_back`, and any `refused_pre_apply_*` outcome.
- `preserved_state_classes_observed` — preserved state the outcome
  verifies stayed untouched. `user_authored_files` MUST appear by
  rule.
- `checkpoint_used_ref` — opaque ref to the checkpoint apply read or
  restored. Null when no checkpoint applied.
- `reversal_executed_class` — single `transaction_reversal_class` token
  describing which reversal actually ran. Required for
  `applied_failed_rolled_back` and `applied_failed_compensated`.
- `failure_reason_class` — closed token for failure cause. Required
  for `applied_failed_*`.
- `remaining_unknowns` — typed unknown classes drawn from the
  project-doctor `remaining_unknowns` vocabulary. Required non-empty
  for `applied_partial_recovered_with_typed_unknowns`.
- `forbidden_action_assertions_held` — boolean. False iff at least one
  `forbidden_action_class` assertion was violated; outcome MUST be one
  of the `refused_pre_apply_*` classes in that case.
- `forbidden_action_violations` — closed list of violated forbidden
  actions. Empty for any non-refused outcome.
- `runtime_requirements_held` — five booleans (online_state_held,
  trust_state_held, user_consent_held, admin_consent_held,
  idempotency_key_held).
- `escalation_packet_ref` — stable id of the
  `object_handoff_packet_record` the outcome routed to. Required for
  `escalated_no_apply` and `applied_failed_no_reversal_export_only`.
- `explanation_fields` — five reviewable sentences (post-apply state,
  preserved state, reversal summary, escalation summary, next step).

Rule: an outcome whose `outcome_class` matches the regex
`^refused_pre_apply_` MUST set
`forbidden_action_assertions_held = false` and
`forbidden_action_violations` MUST be non-empty. The schema enforces
this.

Rule: a "Fix" button MUST NOT bypass the preview/apply/rollback
semantics for any transaction whose `repair_class_family` is one of
`disposable_state_rebuild`, `extension_isolation`,
`extension_rollback_reinstall`, `execution_context_reresolve`,
`remote_runtime_repair`, or `policy_entitlement_refresh`. Bypass paths
are non-conforming and MUST emit
`refused_pre_apply_runtime_requirement_unmet`.

## Linkage rules

The repair transaction is not a standalone artifact. It MUST compose
with the four support packet families already frozen in this
repository:

1. **Support bundle** — every applied transaction MUST appear in
   `support_bundle_record.recovery_context.repair_transaction_refs`,
   and the rung-history transition MUST cite the `checkpoint_ref` and
   `repair_transaction_ref`. The bundle's `current_rung_class` MUST
   match the rung the transaction wrapped (or `safe_mode` /
   `restricted_reopen` / `typed_repair_flow` when the transaction is
   not rung-bound). A transaction that ran end-to-end and was not
   captured into a bundle is non-conforming for export review.
2. **Object-issue handoff** — every transaction whose `escalation_route`
   produced (or refused into) an export MUST emit an
   `object_handoff_packet_record` whose
   `evidence_and_recovery_context.repair_transaction_refs` and
   `checkpoint_refs` cite the transaction and its checkpoint. The
   packet's `redaction_choice_class` MUST equal the transaction's
   `default_redaction_choice_class` unless an explicit consent marker
   widens it.
3. **Recovery action** — every transaction whose `repair_class_family`
   is not `guided_export_escalation` and not `observe_only_no_repair`
   MUST cite the `recovery_action_id` it wraps in `linkage_bindings`.
   A transaction that cannot name a rung MUST move to
   `guided_export_escalation`.
4. **Project Doctor finding** — every transaction MUST cite at least
   one `doctor.finding.*` code from
   `fixtures/support/scenario_matrix.yaml` in
   `initiating_finding_codes`. Every Doctor finding whose
   `suggested_repair_class` is not `observe_only_no_repair` and not
   `defer_to_escalation_packet` MUST resolve through one transaction id;
   the finding's `recovery_rung_class` and the transaction's wrapped
   recovery action MUST agree.

Rule: a support/export flow MUST cite a transaction by its stable
`repair_transaction_id` (and by its `repair_class_family` token)
rather than by free-text Fix-button copy. A bundle, handoff packet,
or Doctor surface that advertises repair in narrative without naming
a transaction id is non-conforming.

## Apply / outcome / escalation rules

The acceptance contract for "Fix" actions:

- **No silent widening.** A repair MUST NOT widen workspace trust,
  publish a route, run a repo hook, silently rebind a remote helper,
  silently reinstall an extension, mutate managed policy, rewrite
  user-authored files, read or rotate credentials, auto-retarget
  without user choice, mutate authoritative profile state, embed raw
  secrets, or auto-widen redaction. Detection at preview-time MUST
  emit `dry_run_refused_*`; detection at apply-time MUST emit
  `refused_pre_apply_*`.
- **Preview is mandatory for durable or policy-sensitive state.** Any
  transaction whose `repair_class_family` is one of
  `disposable_state_rebuild`, `extension_isolation`,
  `extension_rollback_reinstall`, `execution_context_reresolve`,
  `remote_runtime_repair`, or `policy_entitlement_refresh` MUST run
  through the review → preview → checkpoint → apply → verify →
  rollback/compensate grammar. Bypass is non-conforming.
- **Checkpoint honesty.** A `transaction_reversal_class = exact` claim
  MUST be backed by `apply_with_checkpoint` or
  `apply_with_rollback_on_failure`. A `compensating` claim MUST
  describe the inverse action in
  `explanation_fields.reversal_summary`. A `regenerate` claim MUST
  name the authoritative source the regeneration reads from. A
  `manual` claim MUST publish reversal instructions in
  `explanation_fields.user_facing_next_step`. An `audit_only` claim
  MUST NOT advertise any state-restoring undo path.
- **Idempotency.** Every transaction carries an `idempotency_key`.
  Re-submission against the same finding set MUST be a no-op apply.
  Preview MAY re-run (it has no state mutation).
- **Escalation preference.** When no safe local repair exists the
  transaction MUST default to `guided_export_escalation` with
  `apply_refused_escalation_only` and `audit_only`, and MUST cite the
  escalation_packet_completeness case template the export will
  project onto. A "high-quality escalation packet that names the gap
  precisely" is preferred over a forced apply that crosses a forbidden
  boundary.
- **Outcome honesty.** Every apply path emits one
  `repair_outcome_record`. Refusal paths emit
  `refused_pre_apply_*` with `forbidden_action_violations` populated.
  Failure paths emit `applied_failed_*` with `failure_reason_class`
  populated. Partial-recovery paths emit
  `applied_partial_recovered_with_typed_unknowns` with non-empty
  `remaining_unknowns`.

## Seeded cases

The seed corpus in
[`repair_cases/`](../../fixtures/support/repair_cases/) covers the six
scenarios the milestone requires. Every case row binds 1:1 to a
`repair_transaction_record` and names its default repair-class family,
suggested repair class, transaction reversal class, apply mode,
runtime requirements, forbidden-action assertions, and linkage refs.

| Case id | Family | Suggested class | Reversal | Apply mode | Default redaction | Checkpoint required |
|---|---|---|---|---|---|---|
| `repair_case:disposable_state_rebuild.cache_index_repair` | `disposable_state_rebuild` | `reset_ephemeral_cache` | `regenerate` | `apply_with_checkpoint` | `metadata_only_default` | `true` |
| `repair_case:extension_isolation.suspect_host_quarantine` | `extension_isolation` | `quarantine_and_bisect_extension` | `compensating` | `apply_with_rollback_on_failure` | `metadata_only_default` | `true` |
| `repair_case:execution_context_reresolve.toolchain_required_component` | `execution_context_reresolve` | `install_or_repair_toolchain` | `compensating` | `apply_with_rollback_on_failure` | `metadata_only_default` | `true` |
| `repair_case:remote_runtime_repair.remote_agent_rollback` | `remote_runtime_repair` | `rollback_remote_runtime` | `compensating` | `apply_with_rollback_on_failure` | `support_bundle_by_reference` | `true` |
| `repair_case:policy_entitlement_refresh.trust_approval_reacquire` | `policy_entitlement_refresh` | `reacquire_trust_approval` | `compensating` | `apply_with_rollback_on_failure` | `support_bundle_by_reference` | `true` |
| `repair_case:guided_export_escalation.no_local_repair_available` | `guided_export_escalation` | `defer_to_escalation_packet` | `audit_only` | `apply_refused_escalation_only` | `support_bundle_by_reference` | `false` |

Every case names the Project Doctor finding code(s) it is the default
transaction for, lists the recovery action it wraps (or declares
`recovery_action_id: null` for `guided_export_escalation`), and lists
the support-bundle case ref, the escalation-packet completeness case
ref, and the preview/outcome record refs.

## What this seed does not promise

- No live repair engine, transaction supervisor, preview compiler,
  checkpoint store, or apply runtime is wired up. The transaction
  decision object, the preview/apply/rollback grammar, the forbidden-
  action contract, and the escalation linkage rules are reviewable
  objects only.
- No checkpoint implementation, storage-reset flow, or extension-
  quarantine supervisor lands in this milestone.
- No numeric idempotency horizon, retry budget, or apply-latency
  threshold is committed. Thresholds remain reserved to the benchmark
  council via the existing scoreboard.
- No schema changes to `support_bundle.schema.json`,
  `object_handoff_packet.schema.json`, or `recovery_action.schema.json`
  are required; this packet projects onto existing vocabularies and
  adds the `repair_transaction_record`, `repair_seed_case_record`,
  `repair_preview_record`, and `repair_outcome_record` shapes in three
  new schemas rather than mutating landed ones.
- Adding a new `repair_transaction_record` row under an existing
  family is additive-minor provided it reuses the frozen
  `repair_class_family`, `suggested_repair_class`,
  `transaction_reversal_class`, `apply_mode_class`,
  `forbidden_action_class`, `impacted_state_class`, and
  `preserved_state_class` vocabularies. Repurposing any of those
  tokens is breaking and requires a new decision row in
  `artifacts/governance/decision_index.yaml`.
