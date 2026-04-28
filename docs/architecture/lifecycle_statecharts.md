# Protected Object Lifecycle Statecharts

This pack freezes the state names, recovery transitions, and actor
authority rules for protected objects whose lifecycle is visible across
UI, support exports, automation logs, audit trails, and docs/help. The
goal is one vocabulary: a document shown as `stale`, a run marked
`cancelled`, or a collaboration session in `shared_degraded` means the
same thing wherever it appears.

## Companion Artifacts

- [`/schemas/governance/lifecycle_state.schema.json`](../../schemas/governance/lifecycle_state.schema.json)
  defines the cross-tool row shape for statecharts, state nodes,
  transition rows, and actor-authority rules.
- [`/artifacts/architecture/statecharts/`](../../artifacts/architecture/statecharts/)
  contains the family statecharts and per-transition authority tables.
- Existing owning contracts remain authoritative for packet internals:
  command descriptors, run/attempt records, notification event lineage,
  document-state badges, collaboration sessions, migration sessions, and
  repair transactions.

This pack composes with those contracts. It does not replace their
schemas or create a second state owner.

## Protected Families

| Family | Statechart artifact | Owning contract |
| --- | --- | --- |
| `workspace_session` | [`workspace_session.md`](../../artifacts/architecture/statecharts/workspace_session.md) | workspace entry/restore, restore fidelity, runtime authority |
| `document_buffer` | [`document_buffer.md`](../../artifacts/architecture/statecharts/document_buffer.md) | editor document-state and mutation lineage |
| `command_invocation` | [`command_invocation.md`](../../artifacts/architecture/statecharts/command_invocation.md) | command descriptor and invocation-session packet |
| `notification` | [`notification.md`](../../artifacts/architecture/statecharts/notification.md) | notification delivery and event lineage |
| `task_run_attempt` | [`task_run_attempt.md`](../../artifacts/architecture/statecharts/task_run_attempt.md) | run, attempt, outcome, artifact-event records |
| `collaboration_session` | [`collaboration_session.md`](../../artifacts/architecture/statecharts/collaboration_session.md) | collaboration session authority contract |
| `migration_import_session` | [`migration_import_session.md`](../../artifacts/architecture/statecharts/migration_import_session.md) | migration center object model |
| `repair_transaction` | [`repair_transaction.md`](../../artifacts/architecture/statecharts/repair_transaction.md) | repair transaction, preview, outcome contracts |

## Shared Recovery Transitions

Every family statechart uses the following recovery-transition classes.
Family-specific transitions may add detail, but they must not rename
these classes in UI, logs, support exports, or help docs.

| Recovery class | Meaning | Required fields |
| --- | --- | --- |
| `failure` | The owner observed a typed failure and moved to a failed, degraded, or refused state. | `failure_reason_class`, `evidence_refs`, `audit_event_refs` |
| `timeout` | A deadline elapsed before the next admissible state arrived. | `deadline_ref`, `failure_reason_class` or `denial_reason_class`, `audit_event_refs` |
| `cancel` | A user, policy, supervisor, or owner requested stop. | `cancellation_actor_ref`, `cancellation_authority_class`, `audit_event_refs` |
| `retry` | The object is tried again without erasing prior evidence. | `predecessor_transition_ref`, `idempotency_key_ref` or `checkpoint_ref`, `audit_event_refs` |
| `rollback` | The object returns through checkpoint restore or a declared compensating path. | `rollback_checkpoint_ref` or `reversal_class`, `evidence_refs`, `audit_event_refs` |
| `downgrade` | Authority, freshness, route, trust, policy, or capability narrowed without pretending the object stayed healthy. | `downgrade_reason_class`, `preserved_state_refs`, `audit_event_refs` |
| `stale_reconciliation` | The owner detected drift and either refreshed, re-bound, or marked the object stale. | `stale_basis_ref`, `last_known_good_ref`, `reconciliation_outcome_class` |

Rules:

1. `failure`, `timeout`, `rollback`, `downgrade`, and
   `stale_reconciliation` always emit an audit event.
2. `retry` never overwrites the prior attempt, delivery, invocation,
   repair, or migration evidence. It creates a successor transition or
   successor object ref.
3. `rollback` cannot claim exact reversal unless a checkpoint or the
   owning contract's exact reversal field is present.
4. `downgrade` preserves local durable work unless the user or admin
   explicitly discards it through a governed record.
5. `stale_reconciliation` may move to a healthy state only when the
   owner revalidated the stale basis. Otherwise it remains in a stale,
   degraded, blocked, or review-required state.

## Actor Authority

Actor classes are shared across the family artifacts so a future surface
can tell whether it is proposing, approving, retrying, repairing,
archiving, or exporting under the right authority.

| Actor class | May initiate | May approve or reject | May retry | May repair | May archive/export |
| --- | --- | --- | --- | --- | --- |
| `interactive_user` | User-owned workspace, document, command, notification, run, migration, and repair flows | User-owned preview and cancellation decisions | Yes, when the object is user-owned and retryable | Only through reviewed repair transactions | Export user-owned objects after preview when data leaves the local trust boundary |
| `workspace_owner` | Workspace/session and buffer transitions | Workspace-scoped authority changes | Yes | May request repair; repair executor applies | May archive workspace/session evidence and export support-safe records |
| `session_owner` | Collaboration publish, end, and archive transitions | Collaboration admission and archive scope | Session rejoin/recover only | No direct repair outside owning contracts | May seal/export collaboration archives under policy |
| `participant` | Participant-local collaboration actions | Own local discard or rejoin decisions only | Own rejoin/retry paths | No | Own local export only when policy allows |
| `command_router` | Enablement, preview, execution, and outcome transitions | Reject by descriptor, preview, approval, or policy rule | May schedule retry when descriptor admits it | No | Emits invocation evidence refs; does not own export approval |
| `policy_service` | Policy-driven deny, cancel, downgrade, and expiry transitions | Yes, for policy/admin-controlled objects | Only for admitted automation or policy refresh paths | May authorize repair; does not execute it | May approve managed export or archive under policy |
| `supervisor` | Recovery, restart, quarantine, timeout, and rollback transitions | Reject unsafe restart or recovery | Yes, inside restart and recovery budgets | May initiate repair transaction | May emit audit/export refs; high-risk export still needs policy/user approval |
| `support_operator` | Support export, repair preview, escalation, and guided recovery | Reject unsafe repair/export; approve only with delegated support authority | Yes, when an authority ticket allows | May initiate reviewed repair | May prepare exports; user/admin approval required when data leaves boundary |
| `admin` | Managed policy, retention, archive, and export transitions | Yes, for managed/admin surfaces | Managed retries only | May approve managed repair | May archive/export managed records under retention policy |
| `automation_scheduler` | Scheduled and admitted automation transitions | No self-approval for high-risk effects | Yes, when idempotency and lineage are present | No direct repair | No export/archive unless a policy ticket explicitly admits it |
| `ai_assistant` | Preview/proposal transitions only | No | No self-retry of mutating work | No | No |
| `extension_host` | Permission-scoped proposals and observations | No | Only within declared extension lifecycle budget | No | No |
| `remote_agent` | Observed remote state, reconnect, and route downgrade transitions | No | Reconnect/retry within issued authority | No | Emits evidence refs; export is owned elsewhere |
| `provider_service` | Provider-authoritative outcome and stale/downgrade observations | Provider rejection is evidence, not local approval | Provider retry only through the local command/run object | No | Provider export handled by provider-plane contracts |
| `owning_subsystem` | Owner-service observations such as VFS, notification routing, runtime execution, relay, or importer sub-steps | May reject only inside its owned contract and policy ceiling | Only when the owning contract declares retry safe | May route to a repair transaction; does not bypass one | Emits evidence refs; export approval remains with user/admin/support policy |
| `second_party_reviewer` | Review-only approval for workflows that require another human or role | May approve or reject within assigned review scope | No | No | No |
| `migration_importer` | Migration discovery, diff, apply, outcome, and restore transitions | No self-approval for apply | Yes, through session retry/restore rules | No | Emits migration report/export refs |
| `repair_executor` | Repair apply and outcome transitions after preview | Cannot approve its own preview | Idempotent apply retry only | Yes, within transaction bounds | Emits repair evidence; export approval owned by support/user/admin |

## Preview And Checkpoint Rules

| Transition shape | Preview required | Checkpoint required | Notes |
| --- | --- | --- | --- |
| Read-only observation or delivery | No | No | Still emits audit/evidence when visible outside the owner. |
| Local reversible edit | Optional unless multi-file, generated, imported, recovered, or policy-sensitive | Save journal or mutation checkpoint when work could be lost | The document-buffer chart narrows single-buffer save separately. |
| Durable mutation | Yes | Yes | Applies to workspace restore, migration apply, broad command apply, and repair apply. |
| External provider mutation | Yes | When local state is changed or a rollback handle exists | Provider approval tickets ride under runtime authority. |
| Policy, trust, admin, or capability widening | Yes | Yes when local state changes | The approving actor must differ from AI/extension/automation proposer. |
| Downgrade | Details surface required | Preserve-state refs required | A downgrade is allowed without a user prompt only when it narrows authority. |
| Retry | Review required when side effects may repeat | Checkpoint or idempotency key required | Prior evidence is append-only. |
| Rollback | Review required unless automatic rollback was part of the approved transaction | Rollback checkpoint or declared reversal class required | Exact, compensating, regenerate, manual, and audit-only reversal stay distinct. |
| Archive/export | Preview required when data leaves local trust boundary or retention policy changes | Checkpoint not required unless archive mutates state | Export rows cite redaction and record-class posture. |

## Evidence, Export, And Audit Binding

Every statechart node and transition declares which of these field
classes it carries. Family artifacts map them to concrete owning
schemas.

| Field class | Purpose |
| --- | --- |
| `lifecycle_state_class` | Canonical state token shown across UI, docs, logs, and exports. |
| `prior_state_class` / `next_state_class` | Transition endpoints. |
| `transition_class` | Non-recovery transition class such as create, preview, approve, execute, complete, archive, or export. |
| `recovery_transition_class` | One of the shared recovery classes above. |
| `actor_ref` / `actor_class` | Who caused the transition and which authority lane they used. |
| `authority_ticket_ref` | Runtime authority ticket or provider approval envelope when required. |
| `preview_ref` | The preview artifact the user, admin, or approver reviewed. |
| `checkpoint_ref` / `rollback_checkpoint_ref` | Checkpoint or rollback handle required for protected mutation and rollback. |
| `evidence_refs` | Evidence packets, result packets, support rows, or repair/migration outcome refs. |
| `export_refs` | Support/export/report/archive refs; never raw payload bodies. |
| `audit_event_refs` | Append-only audit stream events. |
| `denial_reason_class` / `failure_reason_class` | Typed stop reason. Generic failure text is non-conforming. |
| `deadline_ref`, `cancellation_actor_ref`, `cancellation_authority_class` | Recovery detail for timeout and cancel transitions. |
| `idempotency_key_ref`, `downgrade_reason_class`, `preserved_state_refs` | Recovery detail for retry and downgrade transitions. |
| `last_known_good_ref`, `reconciliation_outcome_class` | Recovery detail for stale-state reconciliation. |

## Change Rules

1. Adding a family state or transition is additive only when it lands in
   this overview, the family artifact, and the lifecycle schema in one
   change.
2. Repurposing an existing state name or recovery class is breaking and
   requires a governance decision row.
3. A future surface that wants a transition crossing actor authority,
   preview, checkpoint, or recovery boundaries must add a row to the
   relevant statechart artifact before implementation.
4. Support exports and automation logs must preserve these state names
   even when visible UI copy is localized.
