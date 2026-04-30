# Retention/deletion matrix, delete-request state, and legal-hold honesty contract

This contract freezes how Aureline explains what data exists, who owns
retention, and why deletion may be delayed, partial, or impossible
right now. It binds every data-lifecycle surface — privacy settings,
support handoff, admin console, offboarding flow, CLI/headless export,
and AI evidence packets — to one matrix row per ownership class plus
location and one delete-request state record per outstanding request.
Held or policy-retained state can never masquerade as successful
deletion under this contract.

Companion artifacts:

- [`/schemas/governance/retention_matrix_row.schema.json`](../../schemas/governance/retention_matrix_row.schema.json)
  defines one matrix row.
- [`/schemas/governance/delete_request_state.schema.json`](../../schemas/governance/delete_request_state.schema.json)
  defines one per-request state record.
- [`/fixtures/governance/retention_deletion_cases/`](../../fixtures/governance/retention_deletion_cases/)
  carries worked cases for immediate local delete, sync-delayed delete,
  provider backlog, legal hold, and exported-local-copy remains.
- [`./record_class_governance.md`](./record_class_governance.md) and
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  remain the class-level retention, hold, delete, and offboarding
  posture this contract composes over. Matrix rows narrow the
  registry's posture for one ownership class plus location combination.
- [`./record_state_and_policy_simulation_models.md`](./record_state_and_policy_simulation_models.md)
  and [`/schemas/governance/record_state.schema.json`](../../schemas/governance/record_state.schema.json)
  remain the per-record state machine. Delete-request state records
  cite governed-record ids, not the other way around; the matrix
  layer answers user-facing "what / who / when" while the record-state
  layer answers "what is the live axis state of this single record."
- [`./privacy_history_and_lifecycle_contract.md`](./privacy_history_and_lifecycle_contract.md)
  and [`/schemas/governance/export_delete_request_summary.schema.json`](../../schemas/governance/export_delete_request_summary.schema.json)
  remain the privacy-history layer. Stable delete states
  (`delete_requested`, `policy_retention`, `legal_hold`,
  `delete_completed`, `exported_copy_remains_local`) and the
  lifecycle vocabulary are reused verbatim from that contract.
- [`./storage_and_retention_vocabulary.md`](./storage_and_retention_vocabulary.md)
  remains the storage, retention, and raw-secret-exclusion vocabulary
  every export path inherits.
- [`./data_portability_and_exit_matrix.md`](./data_portability_and_exit_matrix.md)
  remains the per-domain export, deletion, and exit contract. The
  retention/deletion matrix narrows the same artifact domains by
  ownership class and explicitly typed export and delete paths.
- [`../service/managed_service_seed.md`](../service/managed_service_seed.md)
  and [`/artifacts/service/retention_rows.yaml`](../../artifacts/service/retention_rows.yaml)
  remain the managed-service retention and deletion-semantics seed.

If this document and a schema disagree, the schema wins and this
document must be updated in the same change.

## Why this contract exists

Aureline already names class-level retention, per-record state, the
privacy-history layer, the storage/retention vocabulary, and the
managed-service retention seed. What is still missing is one place
that, for any one ownership class plus location, answers four
questions in user-facing terms:

1. **What data exists?** Local-only, mirrored, managed-only,
   imported, or generated-on-demand.
2. **Who owns retention?** End user, workspace admin, tenant admin,
   policy engine, support, operator admin, or import origin.
3. **How does delete behave?** Local-only delete, distinct local /
   managed deletes, managed delete with receipt, policy-held until
   release, regenerate-instead-of-delete, import-source-owns-delete,
   or not-deletable receipts.
4. **Why is deletion delayed, partial, or impossible right now?**
   Policy retention floor, legal hold, support investigation,
   export-pending hold, sync backlog, provider backlog, managed
   service unavailable, manual local capture required, redaction
   policy, outside platform scope, import source unreachable.

Without this matrix, surfaces drift into one-off "data privacy"
labels that:

- collapse five different ownership classes into one "your data" chip,
  hiding that workspace-shared comments and tenant-managed audit
  packets are not the user's to delete;
- imply "managed by Aureline" without naming the policy owner, so a
  user cannot tell whether to escalate to support, the workspace
  admin, or the tenant compliance team;
- show "Delete completed" the moment the local request is queued,
  even when sync backlog or provider backlog still hold managed
  copies under retention;
- describe a downloaded export `.zip` as part of the managed delete,
  even though the bytes live entirely outside the managed surface;
- substitute "should be quick" or "still working on it" for the typed
  partial-result blocker that actually gates the next state change.

The matrix and the delete-request state schema below force every
surface to render typed answers to those four questions instead of
narrative substitutes.

## The retention/deletion matrix

Every retention/deletion explanation surface — privacy settings detail
panel, support handoff packet, admin console row, offboarding
checklist, CLI/headless `--explain` output — resolves to exactly one
`retention_matrix_row_record` per affected ownership class plus
location combination.

### Required ownership classes

The closed `ownership_class` vocabulary is:

| Ownership class | Required scope |
|---|---|
| `user_owned` | State authored or controlled by a single end-user account: settings, keybindings, snippets, profile artifacts, personal AI memory, local history. |
| `workspace_owned` | State scoped to one workspace, root, or workset and shared with collaborators of that scope: workspace launch policy, comments, review evidence, collaboration sessions. |
| `tenant_owned` | State scoped to a tenant or organization and managed by an admin: policy bundles, entitlement aggregates, support archives, audit ledgers. |
| `imported_artifact` | State that originated outside Aureline (third-party import, replay capture, support bundle from another machine, migrated profile) whose retention is driven by the origin. |
| `derived_cache` | State regeneratable from primary records (embeddings, derived caches, indexed search material, generated previews); deletion is invalidate-not-export. |

Every ownership class MUST resolve through this enum. A surface that
cannot type a row denies with `ownership_class_unresolved` rather than
collapsing to `user_owned`.

### Location, retention, export, delete

Each row carries five typed posture blocks:

1. **`location_class`** — `local_only`,
   `local_authoritative_optional_managed_mirror`,
   `managed_authoritative_with_local_cache`,
   `managed_only_no_local_form`, `hybrid_local_and_managed`,
   `imported_third_party_location`, or `generated_packet_only`.
2. **`default_retention`** — `trigger_class`, `duration_class`,
   whether a retention floor or ceiling applies, and reviewable
   retention notes. Free-text phrasing such as "kept securely" or
   "retained as needed" is non-conforming.
3. **`export_path`** — `export_path_class` (no_export through
   managed_offboarding_packet), manifest requirement, raw-secret
   exclusion requirement, scriptable-without-support-ticket flag,
   minimum export format, and documented path reference.
4. **`delete_path`** — `delete_path_class` (local_delete_only through
   not_deletable_after_export), whether local and managed deletes are
   distinct actions, completion-evidence vocabulary, the
   delete-request state schema reference every surface MUST resolve
   through, and the legal-hold-honoring flag.
5. **`policy_owner`** — `retention_owner_class` and
   `delete_owner_class` from the same closed enum. Anonymous "managed
   by the system" phrasing is non-conforming.

### Schema notes

Every row carries at least one `schema_note` whose `note_kind` is one
of: `boundary_schema`, `record_class_registry`,
`service_retention_row`, `storage_mode_register`,
`policy_bundle_schema`, `deletion_job_schema`,
`destruction_receipt_schema`, `export_manifest_schema`, or
`import_source_schema`. The first note SHOULD be the most specific
boundary or registry that governs the row. Surfaces that need to know
"which schema constrains this row" read the note list verbatim.

### Honesty invariants

Every row asserts:

- `held_or_retained_not_reported_destroyed` (constant `true`) — a row
  with a retention floor or hold eligibility MUST surface
  `Policy retention` or `Legal hold` rather than reporting
  `Delete completed`.
- `raw_payloads_not_embedded` (constant `true`) — rows carry typed
  ids and reviewable sentences only. Raw payload bodies, raw
  credentials, raw policy bundle bytes, raw prompts, and raw hold
  justifications never appear in a row.
- `local_export_copy_disclosed` — true when the export path can
  produce a file the user controls. Rows with downloaded export forms
  MUST disclose that the local copy survives a managed delete.
- `policy_owner_named` (constant `true`) — the row names the
  retention and delete owner classes by stable enum value.

## The delete-request state record

Every outstanding delete-related request — local delete, managed
delete, export-before-delete, support purge, offboarding delete —
emits one `delete_request_state_record` and re-renders it whenever
the stable state, expected next state-change, remaining-location
disclosure, or partial blocker set changes.

### Stable delete states

The closed `stable_delete_state` enum is reused verbatim from the
privacy-history contract:

| State id | Required label | Meaning |
|---|---|---|
| `delete_requested` | `Delete requested` | A request was submitted, accepted, or queued. No completion is implied. |
| `policy_retention` | `Policy retention` | A retention floor keeps at least one managed or audit subset. This is not successful deletion. |
| `legal_hold` | `Legal hold` | A hold blocks destructive lifecycle steps for at least one matching managed record. This is not successful deletion. |
| `delete_completed` | `Delete completed` | The requested destructive action completed for the named scope. Receipts and tombstones may remain as metadata-only evidence. |
| `exported_copy_remains_local` | `Exported copy remains local` | The product produced or downloaded an export copy that remains under user/device control and is outside the managed delete job. |

`policy_retention` and `legal_hold` can never carry a completion claim
of `delete_completed_all_destroyed` or
`delete_completed_with_retained_metadata`. A terminal record with
retained subsets MUST say `Policy retention` or `Legal hold` and cite
the blocker.

### Expected next state-change time

Every record carries an `expected_next_state_change` object with one
`change_class`:

- `definite_at_chronology` — exact time, `expected_at` required.
- `by_chronology_or_sooner` — deadline, `expected_at` required.
- `after_propagation_completes` — propagation queue is the gate;
  `gate_ref` required.
- `after_provider_backlog_clears` — provider queue is the gate;
  `gate_ref` required.
- `after_sync_backlog_clears` — sync queue is the gate; `gate_ref`
  required.
- `on_hold_review_or_clear` — hold is the gate; `gate_ref` required.
- `on_policy_floor_reached` — retention floor is the gate;
  `gate_ref` required.
- `on_export_window_close` — export window is the gate; `gate_ref`
  required.
- `no_further_state_change_expected` — terminal for this request.
- `indeterminate_pending_admin` — admin or operator action required;
  surfaces MUST render this as "requires admin action" rather than
  implying a clock.

A surface that says "should be quick" or "still working on it" is
non-conforming.

### Remaining-location disclosure

The `remaining_location_disclosure` lists every location that still
holds the payload after the request reaches its current state. The
closed `remaining_location_class` vocabulary:

- `local_device_only` — only the originating device still holds the
  payload.
- `local_export_copy` — a downloaded export copy remains under user
  control. **Required** for `exported_copy_remains_local`.
- `managed_archive_held` / `managed_archive_policy_retained` — a
  managed archive still holds the payload under hold or policy
  retention.
- `managed_archive_replicated_pending_purge` — a managed archive is
  propagating delete to replicas; the request is waiting for that
  purge.
- `destruction_receipt_only` — only a metadata-only receipt remains.
- `import_source_origin` — the only remaining copy lives at the
  import origin and is outside Aureline policy.
- `no_remaining_location` — nothing remains.

Records in `policy_retention`, `legal_hold`, or
`exported_copy_remains_local` MUST list at least one
non-`no_remaining_location` entry. Each entry cites the matrix row id
that explains it.

### Partial blockers

The closed `partial_blocker_class` vocabulary distinguishes three
families of blocker:

- **Policy/hold blockers**: `policy_retention_floor`, `legal_hold`,
  `support_investigation`, `export_pending_hold`.
- **Infrastructure delays**: `sync_backlog`, `provider_backlog`,
  `managed_service_unavailable`.
- **Out-of-scope conditions**: `manual_local_capture_required`,
  `redaction_policy`, `outside_platform_scope`,
  `import_source_unreachable`, `user_declined`,
  `entitlement_expired`.

A blocker that cannot be typed denies with
`partial_blocker_class_unresolved` rather than collapsing to a
generic "pending" or "in progress." Each blocker carries a stable id,
optional policy/hold/service-surface refs, an `expected_clear_at`
chronology (null when the clearance time cannot be promised), the
matrix row ids it affects, and a reviewable sentence note.

### Export and handoff rules

Support and admin flows often need to share retention and delete
state for a request without exposing unrelated private payloads. The
record carries one or more `handoff_packet` entries with:

- `handoff_audience` — `user_self_serve`, `support_operator`,
  `tenant_admin`, `managed_control_plane`, or
  `external_compliance_reviewer`.
- `handoff_export_class` — `no_export_emitted`,
  `matrix_state_only_packet`,
  `matrix_state_plus_redacted_summary`, `support_handoff_packet`,
  `admin_audit_packet`, or `compliance_export_packet`.
- `exposes_unrelated_private_payloads` (constant `false`) — a handoff
  MUST NOT include payloads outside the matrix rows in scope. The
  schema rejects records that fail this assertion.
- `redaction_profile_ref` — repo-relative reference to the redaction
  profile the packet is built against.
- `included_matrix_row_refs` / `excluded_matrix_row_refs` — typed
  lists making "what was shared" and "what was withheld" visible to
  reviewers without reading the packet body.

`external_compliance_reviewer` rows MUST cite the redaction profile
they are built against; surfaces that emit a compliance export
without that linkage are non-conforming.

### Audit linkage

Every record cites its audit linkage shape (privacy-history events,
governed-record ids, deletion-job refs, destruction-receipt refs,
support bundle ids, export manifest ids, policy refs). A surface that
quotes a delete-request state MUST be able to reconstruct what
privacy-history events, governed records, and policy bundles were in
effect when the state was recorded.

### Honesty invariants

Every record asserts:

- `held_or_retained_not_reported_destroyed` (constant `true`).
- `raw_payloads_not_embedded` (constant `true`).
- `remaining_location_disclosed_when_required` (constant `true`).
- `expected_next_state_change_disclosed` (constant `true`).
- `handoff_does_not_widen_scope` (constant `true`).

A record that fails any constant=true invariant is rejected.

## Surfaces this contract governs

Surfaces that render any of the following MUST resolve through the
matrix and the delete-request state record:

- privacy settings detail rows that say what data exists for the
  signed-in user;
- support handoff packets that need to share what is being deleted
  and why deletion may be delayed;
- admin console rows that explain who owns retention and which
  policy bundle gates a delete;
- offboarding checklists and exit packets that disclose remaining
  managed and local copies;
- CLI/headless `--explain` output for retention or delete actions;
- AI evidence packets and replay sidecars whose retained-evidence
  copy is governed by a tenant or workspace policy.

Surfaces that render free-text "data privacy" copy without resolving
through the matrix are non-conforming.

## Vocabulary reuse rules

The lifecycle terms (`Local only`, `Uploaded`, `Redacted`, `Held`,
`Destroyed`, `Pending`) and the stable delete states above are the
only terms a renderer may use for the corresponding state. Common
non-conforming substitutions include:

- "removed" or "gone" for held data;
- "private" for local-only state;
- "sent" for uploaded state;
- "complete" for policy-retained data;
- "should be quick" or "still working on it" instead of an
  expected-next-state-change class;
- "managed by the system" instead of a typed policy owner.

CLI/headless output may include the required labels, but MUST NOT
substitute localized prose for the canonical lower-snake ids in
machine-readable mode.

## Fixture coverage

The retention/deletion cases demonstrate the minimum contract checks
required by the spec:

- **Immediate local delete** — a user deletes a regenerable derived
  cache; the request reaches `Delete completed` with
  `no_remaining_location` and no partial blocker.
- **Sync-delayed delete** — a user deletes a workspace-owned record
  whose managed mirror is mid-sync; the request stays in
  `Delete requested` while a `sync_backlog` blocker gates the next
  state change.
- **Provider backlog** — an admin deletes a tenant-owned export packet
  whose downstream provider has a multi-hour purge backlog; the
  request stays in `Delete requested` with a
  `provider_backlog` blocker and `managed_archive_replicated_pending_purge`
  remaining-location entry.
- **Legal hold** — a delete request hits a tenant legal hold; the
  request renders `Legal hold`, cites the hold ref, lists the held
  managed archive, and never claims completion.
- **Exported local copy remains** — an admin offboarding packet
  exports user-owned and tenant-owned state; the request renders
  `Exported copy remains local`, lists the downloaded packet as a
  `local_export_copy` remaining location, and emits a typed
  support-handoff packet that excludes raw secrets.

Each fixture cites the matrix row(s) it affects, the schema notes
it inherits, and the honesty invariants it satisfies.
