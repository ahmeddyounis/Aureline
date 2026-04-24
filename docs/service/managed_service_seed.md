# Managed-service SLO, data-retention, deletion-semantics, and local-core non-dependence seed

This document is the narrative seed for Aureline's optional control-plane
and managed-service surfaces. It freezes the vocabulary every later
managed claim composes against so optional services do not accrete into
hidden prerequisites, and so delete, retention, and offboarding behavior
is described per record class rather than as one vague promise.

Companion artifacts:

- [`/artifacts/service/slo_rows.yaml`](../../artifacts/service/slo_rows.yaml)
  — per-service SLO / SLI rows binding each optional control-plane and
  managed-service surface to an owner, a scope, a degradation-mode set,
  a service-opt-in posture, and a last-known-good fallback note.
- [`/artifacts/service/retention_rows.yaml`](../../artifacts/service/retention_rows.yaml)
  — per-record-class data-retention and deletion-semantics rows linking
  back to `artifacts/governance/record_class_registry.yaml`, export
  posture, legal-hold eligibility, and customer-exit rules.
- [`/schemas/service/deletion_job_record.schema.json`](../../schemas/service/deletion_job_record.schema.json)
  — boundary schema for the `deletion_job_record` carrying a per-class
  delete-request state, expected next state-change time, remaining-
  location notes, and typed legal-hold / policy-retention / entitlement
  blocker vocabulary.
- [`/fixtures/service/deletion_jobs/`](../../fixtures/service/deletion_jobs/)
  — worked deletion-job fixtures validating the schema against a
  representative request-supported class, a legal-hold-blocked class, a
  policy-retention-blocked class, and an entitlement-expired class.

Inherited contracts:

- [`/docs/architecture/identity_modes_adr.md`](../architecture/identity_modes_adr.md)
  / [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md)
  — freezes the three identity modes and the workspace-trust posture.
  This seed inherits the `account_free_local`, `self_hosted_org`, and
  `managed_convenience` vocabulary and MUST NOT recast any service as a
  prerequisite of local-core operation.
- [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  and
  [`/artifacts/deployment/locality_matrix.yaml`](../../artifacts/deployment/locality_matrix.yaml)
  — re-exports the closed deployment-profile, control-plane service,
  control-plane state, data-plane capability, data-plane state, and
  restore-class vocabularies every row here resolves against.
- [`/schemas/deployment/local_core_continuity_packet.schema.json`](../../schemas/deployment/local_core_continuity_packet.schema.json)
  — the `local_core_continuity_packet_record` remains the boundary
  schema for continuity posture. SLO degradation maps to the same
  control-plane state classes used there.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  and
  [`/schemas/governance/record_class.schema.json`](../../schemas/governance/record_class.schema.json)
  — retention rows here link to record classes by `record_class_id`.
  Delete, export, hold, and offboarding postures on a retention row
  MUST be consistent with the class-level posture on the corresponding
  record-class row; this seed narrows, it does not override.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
  — remains the per-record state machine. The deletion-job schema here
  composes over that model; it does not reinvent it.

Normative sources:

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53, §5.57, and Appendix AN.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6 and §9.7.
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.4.2 and §11.4.3.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §18.31 and §18.42.

If this document disagrees with those sources, those sources win and
this document plus the YAML rows update in the same change. If the
narrative and YAML rows disagree, this document wins and the YAML
updates in the same change.

## Scope

Frozen at this revision:

- one SLO / SLI row shape per optional control-plane or managed-service
  surface, with owner, scope, service-opt-in posture, a closed
  degradation-mode set, a last-known-good fallback note, and a named
  local-core non-dependence clause;
- one retention row shape per governed record class reachable through a
  managed-service surface, with typed delete posture, hold eligibility,
  export posture, customer-exit posture, and the record-class row it
  narrows;
- a frozen `deletion_job_state` vocabulary, a typed
  `deletion_blocker_class` vocabulary, an
  `expected_next_state_change_at` field, and a
  `remaining_locations` note set so deletion is inspectable rather than
  surfaced as ad hoc support copy;
- a frozen `service_opt_in_posture` vocabulary
  (`always_off_unless_opted_in`, `off_until_account_created`,
  `on_when_entitled`, `on_with_per_surface_consent`) so services cannot
  flip on by default;
- a frozen `degradation_mode` vocabulary aligned with the locality seed
  (`healthy`, `degraded_slow`, `degraded_stale_cache`,
  `unavailable_retry_later`, `unavailable_quota_exhausted`,
  `unavailable_entitlement_expired`, `unavailable_policy_blocked`,
  `mirror_only`, `boundary_recheck_required`) and a named recovery
  cue per mode;
- an explicit local-core non-dependence clause with first-run and
  startup coverage.

Out of scope at this revision:

- operating a real managed service or SaaS control plane;
- wire protocols, quota algorithms, or billing ledgers;
- pricing, contractual SLA remedies, or legal-hold process mechanics;
- release-evidence claim manifests or support-bundle bodies (those
  compose over this seed).

## The four questions the seed answers

Any Aureline surface that claims a managed, hosted, or optional
control-plane behavior MUST answer these four questions mechanically
against the rows in this seed:

1. **Is the surface opt-in?** Which `service_opt_in_posture` applies,
   what activates it, and what happens when the user never opts in?
2. **What is the SLO?** Which availability and freshness posture does
   the service claim when opted-in and entitled, which
   `degradation_mode` states may it enter, and which `recovery_cue`
   collapses each state back to a product-term next step?
3. **How is the data governed?** Which governed record classes does
   this service touch, and what retention, export, legal-hold, and
   customer-exit posture narrows on top of the class-level row in
   `record_class_registry.yaml`?
4. **What does delete look like?** Which `deletion_job_state` sequence
   applies, which blockers may stall the job, what is the expected
   next state-change time, and what remaining-location note MUST the
   user or admin see until the job terminates?

Generic copy such as "service unavailable", "try again", "data will be
deleted soon", or "please contact support" is forbidden on these paths
when a more precise state is available from the rows below.

## Local-core non-dependence

The desktop core is the product's floor commitment. This seed ratifies
that every row below MUST preserve the following local-core
non-dependence clause. Service rows that cannot meet it are not
admissible as optional services; they are architecture defects.

### First-run

On first launch with no account, no network, and no prior profile:

- the shell reaches the editor and file tree without calling any
  managed-service surface;
- local editing, save, undo, local search, local Git, and task
  execution inside a trusted workspace are usable;
- settings load from built-in defaults, user settings, and workspace
  settings only; managed-settings-sync MUST NOT be a prerequisite and
  MUST NOT block settings resolution when it is unavailable;
- extensions whose manifests declare no network or managed-service
  capability activate normally; managed marketplace lookup is a
  separate, opt-in surface that narrows itself when unavailable;
- BYOK and local AI providers are reachable; the managed AI broker
  service is optional and its absence narrows AI claims, it does not
  block local or BYOK AI.

### Startup

On every subsequent launch, regardless of network or control-plane
reachability:

- startup MUST NOT block on any row here whose
  `service_opt_in_posture` is `always_off_unless_opted_in` or
  `off_until_account_created`;
- startup MUST NOT block waiting for the managed-settings-sync,
  managed-marketplace, managed AI-broker, relay, hosted-review, or
  telemetry-sink surfaces to become reachable;
- a degraded managed surface MUST resolve to a named
  `degradation_mode` and a `recovery_cue` rather than a modal error or
  a loading spinner;
- last-known-good local state (settings, workspace trust, policy-bundle
  cache, entitlement snapshot, docs-pack) is usable under the freshness
  rules pinned by the locality seed; stale cache is labeled, not
  surfaced as live truth.

### Forbidden couplings

The following couplings are architectural regressions and MUST be
caught at review time:

- any row that sets `local_core_blocking` to a truthy value;
- any row whose degradation path routes through a spinner or a generic
  error rather than a named mode;
- any `customer_exit_posture` that collapses local-only materializations
  into the managed retention window;
- any deletion-job implementation that marks local-only artifacts as
  deleted based on a managed receipt alone.

## SLO / SLI row shape

Each row in `/artifacts/service/slo_rows.yaml` is a
`managed_service_slo_row`. Fields:

- `service_id` — stable lower_snake id drawn from the closed service
  vocabulary (`managed_settings_sync`, `managed_marketplace`,
  `managed_ai_broker`, `managed_relay`, `managed_auth_identity`,
  `managed_policy_distribution`, `managed_docs_pack`,
  `managed_catalog`, `managed_telemetry_sink`,
  `managed_collaboration_review`, `managed_support_export`,
  `managed_entitlement_usage`, `managed_offboarding_export`). New
  rows are additive-minor.
- `title` — short human-readable title.
- `description` — one-paragraph statement of what the service does
  and what it does not cover.
- `owner_dri` / `owning_lane` — resolve through
  `artifacts/governance/ownership_matrix.yaml`.
- `scope.control_plane_service_class` — re-export of a value in the
  locality seed's `control_plane_service_vocabulary` so SLO rows and
  continuity packets resolve to the same service class.
- `scope.identity_modes_in_scope` — subset of
  `{account_free_local, self_hosted_org, managed_convenience}`. Rows
  touching `account_free_local` MUST NOT make the desktop core
  dependent on the managed form.
- `scope.deployment_profiles_in_scope` — subset of the closed
  `deployment_profile_vocabulary` from the locality seed.
- `service_opt_in_posture` — closed vocabulary:
  - `always_off_unless_opted_in`
  - `off_until_account_created`
  - `on_when_entitled`
  - `on_with_per_surface_consent`
- `availability_slo` — frozen vocabulary
  (`best_effort_no_slo`, `reachability_target_99_0`,
  `reachability_target_99_5`, `reachability_target_99_9`,
  `mirror_backed_bounded_stale`,
  `customer_operated_self_hosted_sla`).
- `freshness_slo` — frozen vocabulary
  (`fresh_required`, `bounded_stale_labeled`,
  `bounded_stale_with_floor_ref`,
  `unbounded_stale_with_rationale`,
  `freshness_not_applicable`).
- `degradation_modes` — non-empty subset of:
  - `healthy`
  - `degraded_slow`
  - `degraded_stale_cache`
  - `unavailable_retry_later`
  - `unavailable_quota_exhausted`
  - `unavailable_entitlement_expired`
  - `unavailable_policy_blocked`
  - `mirror_only`
  - `boundary_recheck_required`
- `recovery_cues` — one `recovery_cue` per degradation mode. Each cue
  is a short product-term sentence naming the next user-visible step.
- `last_known_good_fallback_note` — what the surface falls back to
  when unreachable (for example, a cached policy bundle, an offline
  mirror, a local model, or a queued-action note).
- `local_core_blocking` — boolean. MUST be `false` for every row in
  this seed; reviewer gate.
- `linked_record_class_ids` — record-class ids from
  `record_class_registry.yaml` the service touches. Every id here
  MUST have a row in `retention_rows.yaml`.
- `linked_drill_refs` — optional drill ids from
  `artifacts/support/deployment_drill_catalog_seed.yaml`.
- `notes` — optional additional note.

## Retention row shape

Each row in `/artifacts/service/retention_rows.yaml` is a
`managed_service_retention_row`. Fields:

- `record_class_id` — MUST resolve to a row in
  `artifacts/governance/record_class_registry.yaml`. The class-level
  posture on that row is the ceiling; this row narrows.
- `managed_copy_kind` — `no_managed_copy`, `support_export_copy`,
  `collaboration_archive_copy`, `ai_retained_evidence_copy`,
  `usage_export_copy`, `offboarding_exit_copy`,
  `destruction_receipt_copy`.
- `data_class_summary` — short product-term description of what the
  managed copy contains.
- `service_surface_refs` — non-empty list of `service_id` values from
  the SLO rows that produce or retain this copy.
- `retention_window` — frozen vocabulary
  (`no_managed_retention`, `session_scoped`,
  `case_scoped_until_close`,
  `packet_expiry_timer`,
  `billing_period_scoped`,
  `contract_term_scoped`,
  `retention_floor_minimum`,
  `legal_hold_bounded_extension`).
- `export_posture` — one of the record-class
  `export_availability` values
  (`not_exportable`, `exportable_on_request`, `local_export_only`,
  `packet_is_export`, `receipt_emitted_on_action`).
- `legal_hold_eligibility` — boolean matching the record-class
  `hold_posture.eligible` value (false on classes that are not
  eligible for any hold).
- `customer_exit_posture` — frozen vocabulary:
  - `local_copy_survives_managed_exit`
  - `managed_copy_deleted_on_exit`
  - `managed_copy_retained_under_legal_hold_only`
  - `managed_copy_retained_under_policy_minimum`
  - `exit_packet_replaces_managed_copy`
- `deletion_semantics.request_supported` — boolean.
- `deletion_semantics.local_and_managed_actions_are_distinct` —
  boolean. MUST be `true` whenever a managed copy exists so local
  delete and managed delete do not collapse into one action.
- `deletion_semantics.completion_evidence` — re-export of the
  record-class `completion_evidence` vocabulary.
- `deletion_semantics.hold_blocks_completion` — boolean.
- `deletion_semantics.default_expected_next_state_change_window` —
  short product-term sentence (for example,
  `"within 24 hours of request acknowledgement"`) used when a
  `deletion_job_record` does not carry a per-job override.
- `deletion_semantics.remaining_location_notes` — short product-term
  sentences naming where the data may continue to live until the job
  terminates (mirror snapshots, destruction-receipt ledger, exit
  packet, customer-operated retention store, etc.).
- `notes` — optional additional note.

Every row in the file MUST have a matching `record_class_id` entry in
the record-class registry, and every record class cited by a row in
`slo_rows.yaml` MUST have at least one row here.

## Deletion-job record

The `deletion_job_record` (see
`schemas/service/deletion_job_record.schema.json`) is the per-request
state record produced by any delete path that touches a managed copy.
It composes over the governed-record state model and the record-class
registry rather than replacing them.

### State vocabulary

`deletion_job_state` is closed:

- `requested_not_acknowledged`
- `acknowledged_scheduled`
- `in_progress_local_only`
- `in_progress_managed`
- `in_progress_mixed_local_and_managed`
- `completed_all`
- `completed_partial_hold`
- `completed_partial_policy_retention`
- `blocked_by_hold`
- `blocked_by_policy_retention`
- `blocked_by_entitlement_expired`
- `blocked_by_service_unavailable`
- `cancelled_by_user`
- `cancelled_by_admin`
- `unknown_pending_recheck`

`completed_*` values are terminal-with-receipt; `blocked_*` values are
not terminal and MUST carry an `expected_next_state_change_at`.
`unknown_pending_recheck` is reserved for the boundary-recheck path
and MUST resolve to one of the other states before any claim narrows.

### Blocker vocabulary

`deletion_blocker_class` is closed:

- `administrative_legal_hold`
- `support_investigation_hold`
- `retention_minimum_policy`
- `policy_freeze_window`
- `export_pending_hold`
- `entitlement_expired`
- `service_unavailable_retry_later`
- `boundary_recheck_required`
- `downstream_dependency_hold`

Blockers are narrow-by-class; a job MAY carry more than one. A job in
a `blocked_*` state MUST carry at least one blocker, and every blocker
MUST cite an `artifact_ref` or a `policy_ref` the user or admin can
inspect.

### Remaining-location notes

`remaining_location_notes` is a non-empty list on every non-terminal
state. Each note names a storage surface (for example,
`mirror_snapshot`, `destruction_receipt_ledger`, `exit_packet`,
`customer_operated_store`, `vendor_operated_store`,
`air_gap_offline_bundle`) so the user or admin can see exactly where
the data may still live until the job terminates. The schema does not
pin a free-text body; the note vocabulary is the contract.

### Expected next state-change time

`expected_next_state_change_at` is required on every non-terminal
state and on `unknown_pending_recheck`. Terminal
`completed_*`, `cancelled_by_*` states MUST set it to `null`. The
field is an RFC 3339 UTC timestamp; jobs that cannot honestly predict
a next change time MUST route through `unknown_pending_recheck`
rather than backdate an expectation.

## Rendering contract

Product surfaces that render a managed-service claim consume the
frozen rows here. Specifically:

- availability and degradation cues come from `slo_rows.yaml` (mode +
  recovery cue), not from free text in error paths;
- retention and legal-hold copy resolve through `retention_rows.yaml`
  and back to `record_class_registry.yaml` rather than from per-surface
  strings;
- delete and offboarding copy renders from a `deletion_job_record`
  carrying a typed state, blocker set, and remaining-location notes;
- "delete complete" copy MUST cite a destruction-receipt id and MUST
  distinguish `completed_all` from `completed_partial_hold` and
  `completed_partial_policy_retention` rather than implying total
  erasure.

## Evolution rules

- Adding a new `service_id`, `degradation_mode`,
  `deletion_job_state`, `deletion_blocker_class`,
  `retention_window`, or `customer_exit_posture` is additive-minor and
  bumps the `schema_version` of the affected artifact. Repurposing an
  existing value is breaking and requires a new decision row in
  `artifacts/governance/decision_index.yaml`.
- New record classes reachable through a managed service land a row in
  `record_class_registry.yaml` in the same change that adds the
  corresponding `retention_rows.yaml` row.
- SLO rows whose `service_opt_in_posture` changes to a more
  permissive value (for example from `always_off_unless_opted_in` to
  `on_when_entitled`) require an explicit decision row and an update
  to the forbidden-couplings list above.
- This document, the YAML rows, and the JSON Schema are kept in sync
  by review. Tooling MAY reject PRs that introduce a service, record
  class, or deletion state in only one of the three surfaces.
