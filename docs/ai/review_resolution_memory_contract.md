# AI review-assist resolution-memory, outdated-state, and local/export fallback contract

This document is the **product-wide contract** for AI review-assist
resolution memory. It freezes the rule set every AI review-assist
surface reads when it remembers what happened to a finding after the
run produced it — whether the user dismissed it, published it locally
or to a hosted destination, kept it local because no provider write
capability was held, exported it as a portable packet, watched it go
outdated when the diff moved underneath, or saw it suppressed by an
admin policy bundle. It exists to make sure local-only continuity,
hosted publish, browser-handoff publish, imported-bundle export, and
provider-write-missing fallback all resolve to one shared trust model
— instead of each surface inventing its own resolution language, its
own outdated chip, or its own "we tried to publish" copy with no
record of whether the publish actually succeeded.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / review / hosted-review
surface's mint of its own copy, this document wins and the surface
is non-conforming.

The companion artifacts are:

- [`/schemas/ai/review_resolution_memory.schema.json`](../../schemas/ai/review_resolution_memory.schema.json)
  — boundary schema for the `review_resolution_record`,
  `review_resolution_state_transition_record`,
  `review_resolution_material_diff_change_record`, and
  `review_resolution_audit_event_record` shapes.
- [`/schemas/ai/local_review_findings_store.schema.json`](../../schemas/ai/local_review_findings_store.schema.json)
  — boundary schema for the
  `local_review_findings_store_record`,
  `local_review_findings_export_packet_record`, and
  `local_review_findings_store_audit_event_record` shapes.
- [`/fixtures/ai/review_resolution_cases/`](../../fixtures/ai/review_resolution_cases/)
  — worked-example corpus covering at least: a local-only review
  with no provider overlay, a published finding through a hosted
  destination, an org-policy-suppressed case, a provider outage
  forcing the resolution into rerun-recommended, and a changed-diff
  case that forces prior findings into outdated-diff-changed.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/review_assist_publish_contract.md`](./review_assist_publish_contract.md) —
  the AI review-assist finding row, scope-selector, and publish-to-
  review sheet contract. The resolution-state vocabulary on the
  resolution row mirrors the lifecycle vocabulary on the finding
  row verbatim; the publish-eligibility vocabulary mirrors the
  publish-to-review sheet's provider-write-continuity vocabulary
  plus the four contract-level refusals (redaction-failed,
  disclosure-pending, low-confidence floor, outdated-lifecycle).
  The resolution row never disagrees with the finding row about
  lifecycle state; the resolution row never disagrees with the
  publish-to-review sheet about whether publish was admitted.
- [`/docs/ai/memory_and_reconciliation_contract.md`](./memory_and_reconciliation_contract.md) —
  the AI memory-object, invalidation-event, delete-request, and
  export-assembly contract. The local-review-findings store row's
  storage-authority class, delete-posture class, export-posture
  class, and retention-posture class mirror the memory-object
  vocabulary verbatim (narrowed to the classes the local store may
  carry); the local store row never re-mints those vocabularies.
- [`/docs/ai/prompt_injection_and_taint_contract.md`](./prompt_injection_and_taint_contract.md) —
  the input-source / taint / trust / precedence model and the rule
  that designated AI policy files under `.aureline/ai/policy/*` are
  reachable only with admitted signing evidence. A resolution row
  that names `resolution_source_class = designated_policy_file`
  MUST resolve to a `tainted_input_source_record` carrying
  `policy_file_role_class = designated_policy_file` with a non-null
  `signing_evidence_ref`.
- [`/docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md) —
  provider entry, model entry, external-tool entry, execution-locus,
  region posture, and retention stance. The local store row's
  `provider_model_identity` block cites those refs verbatim and
  records the execution-locus class so the local copy remains
  accountable to the provider routing the run was bound to.
- [`/docs/vcs/review_workspace_contract.md`](../vcs/review_workspace_contract.md) —
  `review_workspace_record`, `review_anchor_record`, and the closed
  source-class / freshness / drift / provider-authority vocabularies.
  Resolution rows, local-store rows, and material-change records
  bind to those records; no parallel review-state vocabulary is
  minted here.
- [`/docs/vcs/review_pack_contract.md`](../vcs/review_pack_contract.md) —
  `review_pack_record` and the must-run / advisory / provider-required
  / local-lint check classes. The
  `instruction_or_check_source_signature_ref` diff-fingerprint kind
  cites the review-pack evaluation input signature so a pack-update
  rerun is detectable from the resolution row.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md) —
  delete-honesty, hold-class, and destruction-receipt vocabulary.
  The local store row's `delete_request_lifecycle_class` mirrors
  the delete-request transitions defined there.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state, policy epoch, and trust state on every
  record.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md) —
  the broker-owned redaction pass runs before any export packet
  crosses this boundary. A packet whose `export_disclosure_class`
  is `redaction_required_user_must_review_before_export` or
  `export_blocked_secret_adjacent_user_must_review` MUST NOT cross
  the boundary.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md) —
  admin policy MAY narrow the resolution-source classes, the
  publish-eligibility classes, and the storage-authority class
  (e.g. forcing org-policy-suppressed rows to
  `admin_or_control_artifact`); policy MAY NOT silently widen.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  approval-ticket vocabulary. A resolution row that records
  `published_to_review_destination` cites the approval ticket the
  publish-to-review sheet spent.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  `freshness_class`, `client_scope`, `redaction_class` re-exported
  without modification.

If this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does **not** ship a model, a review-engine runtime,
a local-store implementation, or any provider-write code. It freezes
the contract those implementations will read and write. The eventual
review-assist crate's Rust types are the schema of record; the JSON
Schema exports are the cross-tool boundary every non-owning surface
reads.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.8.14 (AI
  review assist, diff-scoped checks, and publish-to-review contract).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §15.20 (AI review
  assist, diff-scoped checks, and publish-to-review UX) and Appendix
  ED.4 / ED.5 / ED.6.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` §23.17 and
  Appendix BO.3.
- `.t2/docs/Aureline_PRD.md` AI-powered code review row and
  local-first / delete-honesty rows.

If this contract disagrees with those sources, those sources win.

## Why freeze this now

Three problems compound the moment AI review-assist surfaces start
remembering what happened to a finding without one shared vocabulary:

1. **Findings that drift out of date but stay visually fresh.** A
   reviewer who sees a finding panel that still paints a finding as
   "Open · evidence-backed · risk_or_bug_concern" three days after
   the author force-pushed a new head cannot tell whether the model
   re-evaluated the finding against the new head or whether the row
   is a stale snapshot. Two reviewers drawing two different
   conclusions about whether to act is the failure mode the
   resolution-memory record exists to prevent.
2. **Publish flows that pretend they succeeded.** "We posted your
   AI suggestion to the hosted thread" without recording whether the
   provider write capability was held on this client, whether the
   broker redaction pass actually completed, or whether the publish
   sheet refused the action means the user cannot tell which findings
   are durably published and which are local-only fictions of the
   surface. The export packet that downstream support / portability
   tooling reads MUST never paint a publish that was never performed
   as having been performed.
3. **Local-only continuity that loses the analyzed scope, the
   provider/model identity, or the policy/trust context the run was
   bound to.** A local store row that survives a workspace re-open
   without remembering which model evaluated it, which provider
   routed it, which review-pack check directed it, or which policy
   epoch admitted it cannot answer "was this finding evaluated under
   the same model/provider/policy as the current session's findings"
   — the fallback usefulness of the local store collapses the moment
   the binding refs disappear.

Closing all three with one record family — one resolution row, one
state-transition row, one material-diff-change row, one local store
row, and one export packet row — solves the resolution-memory
classification problem at the boundary, not at every consuming
surface.

## Who reads this document

- **AI review-assist authors** classifying every finding into the
  closed `resolution_state_class`, naming the actor and source that
  drove the transition, recording the diff fingerprint binding, and
  admitting the closed `publish_eligibility_class` set.
- **Local-store authors** writing the local row that survives across
  sessions, citing the analyzed scope, the diff fingerprint, the
  provider/model identity, the policy/trust context, and the publish
  eligibility on this client.
- **Export / support / portability surface authors** rendering the
  export-packet's exact `export_publish_state_class` and
  `export_disclosure_class` so a packet that was kept local never
  paints publish as having succeeded.
- **Admin / policy / settings surface authors** narrowing which
  resolution-source classes, publish-eligibility classes, and
  storage-authority classes are admitted per deployment profile.
- **Watcher / reconciler authors** firing the matching
  `material_change_verdict_class` when the analyzed scope drifts,
  the provider overlay refreshes, the review pack updates, the
  instruction or check source signature changes, or the evaluator
  goes out — so prior findings never stay visually fresh under a
  drift the surface failed to surface.
- **Evidence / replay / support / parity-audit authors** quoting
  the resolution-state, transition-trigger, and export-publish-state
  vocabulary mechanically rather than re-deriving the contract.

## 1. Resolution-memory state machine

Every `review_finding_record` (per the AI review-assist publish
contract) resolves to exactly one `review_resolution_record`. The
schema (`review_resolution_memory.schema.json`) freezes the row
shape.

### 1.1 Resolution state vocabulary

The state vocabulary is closed and mirrors the
`review_finding.lifecycle_state` vocabulary verbatim so a resolution
row never disagrees with the finding row it summarises. A surface
that observes an unrecognised value denies with
`review_resolution_record_unknown_value`.

| State | Meaning |
|---|---|
| `open` | Initial state; no resolution action has been taken. |
| `dismissed` | The user explicitly dismissed the finding with a typed rationale on the bound finding row. |
| `published_local_only` | The finding was anchored on the local `review_workspace_record` only (no outbound destination write). |
| `published_to_review_destination` | The finding was published through `publish_to_review_sheet_record` to a hosted destination, an imported-bundle export packet, or a support packet. |
| `outdated_diff_changed` | A material-change watcher fired one of the `*_outdated` verdicts; the resolution is no longer fresh against the analyzed scope. |
| `rerun_recommended` | A material-change watcher fired one of the `*_rerun_recommended` / `*_rerun_required` verdicts; the resolution should be re-evaluated before publish. |
| `suppressed_by_policy` | An admin policy bundle, repo-instruction bundle, or designated AI policy file suppressed the finding with rationale. |
| `superseded_by_rerun` | A successor resolution exists; this row is the predecessor. |
| `archived_tombstone` | The retain-as-tombstone state; the row is preserved for audit but not active. |

The state vocabulary admits **NO** auto-approve, auto-request-changes,
or auto-merge transition. A surface that mints such a transition
denies with `auto_approve_or_request_changes_or_merge_forbidden`
on the matching audit event. Those actions live on
`schemas/vcs/merge_queue_entry.schema.json` and require human
approval through the merge-policy contract.

### 1.2 Actor and source classes

Every resolution row carries exactly one `resolution_actor_class`
(who drove the state) and exactly one `resolution_source_class`
(what authored or directed the transition). The two are decoupled
because one user action can be directed by one of several sources.

The actor vocabulary is closed:

```
user_in_active_session
admin_via_signed_policy
ai_review_overlay_recording_only
scope_freshness_watcher
provider_overlay_watcher
rerun_engine
system_reconciler
```

Rule: `ai_review_overlay_recording_only` MUST NOT mint a
`published_to_review_destination` transition. A recording-only actor
that attempts a destination write denies with
`ai_overlay_recording_only_must_not_mint_publish_or_destination_write`.

The source vocabulary is closed:

```
user_in_active_session
repo_instruction_bundle_authored
designated_policy_file
trusted_workspace_pinned_policy
trusted_user_profile_policy
admin_policy_bundle
review_pack_check_outcome
provider_overlay_watcher
scope_freshness_watcher
rerun_engine
ai_review_overlay_recording_only
system_reconciler
```

Rule: `designated_policy_file` is reachable only when the upstream
`tainted_input_source_record` for the same assembly carries
`policy_file_role_class = designated_policy_file` and a non-null
`signing_evidence_ref` (per the prompt-injection contract). A row
that claims `designated_policy_file` without the signed input
denies with `designated_policy_file_source_requires_signed_policy_input`.

### 1.3 Reopen-eligibility class

The reopen-eligibility vocabulary records what reopen path the row
admits from its current state. A surface that mints a transition
the eligibility class refuses denies with
`reopen_action_refused_by_eligibility`.

```
reopen_admitted_user
reopen_admitted_user_or_admin
reopen_admitted_admin_only
reopen_blocked_archived
reopen_blocked_suppressed_by_policy
reopen_blocked_outdated_must_rerun
reopen_blocked_superseded
not_applicable_state_admits_open_action_directly
```

Rules:

1. `archived_tombstone` MUST carry `reopen_blocked_archived`.
2. `suppressed_by_policy` MUST carry
   `reopen_blocked_suppressed_by_policy` or
   `reopen_admitted_admin_only`.
3. `superseded_by_rerun` MUST carry `reopen_blocked_superseded`
   (the successor row is the live row).
4. `outdated_diff_changed` MAY carry
   `reopen_blocked_outdated_must_rerun` when the predecessor has
   been archived and the rerun has not yet completed.
5. `open`, `rerun_recommended`, and `outdated_diff_changed` rows
   that have not been pinned to a predecessor archive carry
   `not_applicable_state_admits_open_action_directly`.

### 1.4 Publish-eligibility class

The publish-eligibility vocabulary records what publish path the
local store admits on this client. The vocabulary is mirrored
verbatim onto the local-review-findings store so the two records
never disagree.

```
publish_eligible_provider_write_admitted
publish_eligible_under_browser_handoff
publish_eligible_under_browser_handoff_pending_block_publish
publish_eligible_via_imported_bundle_export_packet
publish_blocked_provider_write_missing_keep_local_or_export
publish_blocked_imported_bundle_no_outbound
publish_blocked_browser_handoff_token_only_no_outbound
publish_blocked_redaction_failed_user_must_review
publish_blocked_disclosure_pending_user_must_review
publish_blocked_low_confidence_floor_unmet
publish_blocked_outdated_lifecycle
publish_unknown_must_block
not_applicable_local_only_no_publish_proposed
```

Rules:

1. `published_to_review_destination` MUST carry one of the
   `publish_eligible_*` values (specifically
   `publish_eligible_provider_write_admitted`,
   `publish_eligible_under_browser_handoff`, or
   `publish_eligible_via_imported_bundle_export_packet`).
2. `outdated_diff_changed` and `rerun_recommended` MUST carry
   `publish_blocked_outdated_lifecycle`. A row that paints a
   stale resolution as publish-eligible denies with
   `outdated_lifecycle_must_block_publish_to_destination`.
3. A finding whose `confidence_class = low_confidence` (per the AI
   copy-guardrails contract) MUST carry
   `publish_blocked_low_confidence_floor_unmet` until the user
   lifts the confidence floor or a rerun moves it; a row that
   paints low-confidence as publish-eligible denies with
   `low_confidence_floor_must_block_publish_to_destination`.
4. `publish_unknown_must_block` is the fail-closed value; a row
   that admits publish from this state denies with
   `publish_eligibility_must_match_publish_to_review_sheet_continuity`.

### 1.5 Diff fingerprint binding

Every resolution row cites at least one diff fingerprint. The schema
enforces this through `minItems: 1` on `diff_fingerprints`; a row
with no fingerprints denies with
`diff_fingerprint_required_at_least_one`.

The fingerprint-kind vocabulary is closed:

```
head_revision_id_ref
base_revision_id_ref
selected_diff_range_signature_ref
uncommitted_index_signature_ref
provider_pull_or_merge_request_head_ref
imported_bundle_envelope_pin_ref
browser_handoff_packet_pin_ref
review_pack_evaluation_input_signature_ref
policy_epoch_ref
instruction_or_check_source_signature_ref
```

Raw revision URLs, raw branch URLs, raw diff bodies, raw policy
bundle bytes, and raw absolute paths never cross this boundary;
every value is an opaque ref.

### 1.6 Audit linkage

Every state transition is recorded as a
`review_resolution_state_transition_record` carrying the from-state,
to-state, trigger, actor, source, the reopen-eligibility class at
the moment of the transition, the publish-eligibility class at the
moment of the transition, and the matching `predecessor_resolution_id_ref`
/ `successor_resolution_id_ref` / `publish_to_review_sheet_id_ref`
when the transition admits one. The audit-event vocabulary is closed
(see §5).

## 2. Material-diff-change rules

Every drift event in the analyzed scope produces exactly one
`review_resolution_material_diff_change_record`. The schema freezes
the verdict-to-state pairings through allOf gates so a surface
cannot leave a prior finding visually fresh under a drift the
watcher already observed.

### 2.1 Verdict vocabulary

The verdict vocabulary is closed:

```
no_material_change_resolution_remains_fresh
minor_unrelated_change_resolution_remains_warm
diff_changed_after_run_resolution_outdated
base_or_head_changed_after_run_resolution_outdated
provider_overlay_refreshed_resolution_rerun_recommended
review_pack_or_policy_updated_resolution_rerun_required
instruction_or_check_source_changed_resolution_rerun_required
evaluator_outage_resolution_rerun_required
scope_pinned_no_drift_assessable
```

### 2.2 Forced-state pairings

The schema enforces three pairing rules through allOf gates:

1. `diff_changed_after_run_resolution_outdated` and
   `base_or_head_changed_after_run_resolution_outdated` force
   `forced_to_state_class = outdated_diff_changed` and
   `publish_eligibility_after = publish_blocked_outdated_lifecycle`,
   AND the record MUST cite a non-null
   `successor_analyzed_scope_id_ref`. A surface that records a
   diff-changed verdict without forcing the matching state denies
   with `material_change_verdict_must_force_outdated_or_rerun`.
2. `provider_overlay_refreshed_resolution_rerun_recommended`,
   `review_pack_or_policy_updated_resolution_rerun_required`,
   `instruction_or_check_source_changed_resolution_rerun_required`,
   and `evaluator_outage_resolution_rerun_required` force
   `forced_to_state_class = rerun_recommended` and
   `publish_eligibility_after = publish_blocked_outdated_lifecycle`.
3. `no_material_change_resolution_remains_fresh`,
   `minor_unrelated_change_resolution_remains_warm`, and
   `scope_pinned_no_drift_assessable` MUST NOT force the state out
   of the original; the schema admits `forced_to_state_class` as
   the original (`open` / `dismissed` / `published_local_only` /
   `published_to_review_destination` / `suppressed_by_policy` /
   `archived_tombstone` / `superseded_by_rerun`).

### 2.3 Visible-fresh refusal

The whole point of this section is to make "visually fresh under
drift" structurally impossible. A surface that publishes or admits
a publish action against a resolution row whose
`publish_eligibility_class = publish_blocked_outdated_lifecycle`
denies with `outdated_lifecycle_must_block_publish_to_destination`
on the matching publish-to-review sheet's audit stream.

## 3. Local-only finding store

Every AI review-assist finding kept on this client resolves to
exactly one `local_review_findings_store_record`. The schema
(`local_review_findings_store.schema.json`) freezes the row shape.

### 3.1 Required bindings

Every local store row carries:

- `review_finding_id_ref` — opaque ref to the bound finding row.
- `review_resolution_id_ref` — opaque ref to the bound resolution
  row.
- `analyzed_scope_id_ref` — opaque ref to the analyzed scope row.
- `diff_fingerprints` — at least one fingerprint binding the local
  copy to the bytes the finding was minted against (the same
  fingerprint vocabulary as the resolution row).
- `provider_model_identity` — the provider entry, model entry,
  external-tool entries, and execution-locus class the run was
  bound to.
- `policy_context` — the policy epoch, trust state, execution
  context, and workspace trust state at the moment of mint.
- `publish_eligibility_class` — the publish-eligibility on this
  client (mirrored verbatim from the bound resolution row).
- `storage_authority_class` — `user_owned_recovery_state` by
  default; `admin_or_control_artifact` only when the bound
  resolution row carries `suppressed_by_policy`.
- `delete_posture_class` — the delete posture, mirrored from the
  AI memory-object vocabulary.
- `export_posture_class` — the export posture, mirrored from the
  AI memory-object vocabulary.
- `retention_posture_class` — `workspace_scoped` by default;
  narrowed to `user_pin_only`, `case_close_or_archive`, or
  `never_gc_authoritative` per posture.
- `delete_request_lifecycle_class` — the current delete-request
  lifecycle on this row.

### 3.2 Provider/model identity preservation

The `provider_model_identity` block MUST cite a non-null
`provider_entry_ref` and a non-null `model_entry_ref` so the local
copy remains accountable to the provider routing the run was bound
to. The `external_tool_entry_refs` array MAY be empty when the run
cited no external tools. The `execution_locus_class` mirrors the
provider-registry vocabulary so the local copy never re-mints the
routing classification.

A row that fails to cite the provider entry or the model entry
denies with `provider_entry_or_model_entry_ref_required`.

### 3.3 Publish-eligibility consistency

The local store row's `publish_eligibility_class` MUST match the
bound resolution row's `publish_eligibility_class` exactly. A
mismatch denies with
`publish_eligibility_must_match_resolution_row` on the local-store
audit stream so the local copy never claims publish capability that
the resolution row already refused.

### 3.4 Storage-authority and admin-suppression rule

`storage_authority_class = admin_or_control_artifact` MUST pair
with `delete_posture_class` in
`{delete_denied_class_immutable, delete_blocks_on_hold,
delete_request_supported_with_destruction_receipt}` so an admin-held
suppression cannot quietly admit a generic local-only delete. A
mismatch denies with
`storage_authority_class_must_be_admin_for_suppressed_by_policy`.

### 3.5 Delete-request lifecycle

The lifecycle vocabulary is closed:

```
no_delete_request_pending
delete_requested_local_only
delete_requested_local_and_managed
delete_blocked_on_hold_pending_release
delete_completed_destruction_receipt_emitted
delete_denied_class_immutable_admin_review_required
delete_denied_evidence_packet_overrides_packet_governs
```

Rules:

1. `delete_posture_class = delete_blocks_on_hold` MUST pair with
   `delete_request_lifecycle_class` in
   `{no_delete_request_pending, delete_blocked_on_hold_pending_release}`.
   A mismatch denies with
   `delete_posture_class_must_be_blocked_under_active_hold`.
2. `delete_completed_destruction_receipt_emitted` MUST cite a
   non-null `archived_at`.

## 4. Export packet

Every portable packet exported from the local store resolves to
exactly one `local_review_findings_export_packet_record`.

### 4.1 What the packet preserves

The packet carries the same bindings as the local store row
(analyzed scope, diff fingerprints, provider/model identity, policy
context, publish eligibility) plus two export-specific fields:

- `export_publish_state_class` — what actually happened on this
  client.
- `export_disclosure_class` — what the packet discloses and what
  the user must review before the packet leaves this client.

### 4.2 Publish-state honesty

The `export_publish_state_class` vocabulary is closed:

```
export_records_publish_was_performed
export_records_publish_was_not_performed
export_records_publish_blocked_kept_local
export_records_publish_blocked_redaction_user_must_review
export_records_publish_blocked_disclosure_pending
export_records_publish_blocked_outdated_lifecycle
export_records_publish_blocked_low_confidence_floor
export_records_local_only_no_publish_proposed
```

Rule: a packet whose source store row carries
`publish_eligibility_class` in the `publish_blocked_*` set MUST
NOT carry `export_records_publish_was_performed`. The schema
enforces this through an allOf gate; a mismatch denies with
`export_publish_state_must_record_publish_was_not_performed_when_publish_blocked`.

Rule: `export_records_publish_was_performed` is admissible only
when `publish_eligibility_class` is one of the `publish_eligible_*`
values AND the packet cites a non-null
`publish_to_review_sheet_id_ref`.

### 4.3 Disclosure honesty

The `export_disclosure_class` vocabulary is closed:

```
discloses_provider_model_identity_and_publish_state
discloses_metadata_only_redaction_required
discloses_evidence_packet_only
redaction_required_user_must_review_before_export
export_blocked_secret_adjacent_user_must_review
```

Rule: `redaction_required_user_must_review_before_export` and
`export_blocked_secret_adjacent_user_must_review` are refusal-shaped
values; the packet MUST NOT cross the boundary while either is set.
The packet may still be assembled for user review, but the audit
stream records
`local_review_findings_export_packet_blocked_redaction_required` or
`local_review_findings_export_packet_blocked_secret_adjacent`
rather than `local_review_findings_export_packet_assembled`. A
surface that exports under either denies with
`export_disclosure_class_must_block_export_when_redaction_required`.

### 4.4 Provider-write-missing fallback

The compound rule the contract guarantees: a finding whose publish
path was refused because the provider write capability was missing
remains usable locally and exportable as a portable packet without
pretending publication succeeded. Concretely:

1. The bound `publish_to_review_sheet_record` carries
   `provider_write_continuity_class =
   provider_write_missing_keep_local_or_export` and
   `publish_or_copy_or_export_action_class` in
   `{keep_local, copy_only_no_outbound, export_local_packet,
   block_publish_provider_write_missing_keep_local}`.
2. The bound `review_finding_record` carries
   `lifecycle_state = published_local_only` (or `open` if the user
   chose `keep_local`) and
   `local_or_publish_action_class` in
   `{keep_local, export_local_packet}`.
3. The bound `review_resolution_record` carries
   `resolution_state_class = published_local_only` (or `open`),
   `publish_eligibility_class =
   publish_blocked_provider_write_missing_keep_local_or_export`.
4. The bound `local_review_findings_store_record` carries the same
   `publish_eligibility_class` and preserves the analyzed scope,
   diff fingerprints, provider/model identity, and policy context.
5. The exported `local_review_findings_export_packet_record`
   carries
   `export_publish_state_class =
   export_records_publish_blocked_kept_local` and
   `export_disclosure_class =
   discloses_provider_model_identity_and_publish_state`.

The schema enforces (1), (3), (4), and (5) through allOf gates; the
finding-row contract enforces (2). The chain makes "we pretended
publish succeeded" structurally impossible.

## 5. Audit-event vocabulary

The schemas export their own audit-event id sets:

- `review_resolution_audit_event_id` —
  `review_resolution_minted`,
  `review_resolution_state_transitioned`,
  `review_resolution_published_locally`,
  `review_resolution_published_to_destination`,
  `review_resolution_dismissed`,
  `review_resolution_suppressed_by_policy`,
  `review_resolution_marked_outdated_after_diff_change`,
  `review_resolution_marked_outdated_after_base_or_head_change`,
  `review_resolution_rerun_recommended`,
  `review_resolution_superseded_by_rerun`,
  `review_resolution_reopened_after_archive`,
  `review_resolution_archived`,
  `review_resolution_material_diff_change_recorded`, and
  `review_resolution_audit_denial_emitted`.
- `local_review_findings_store_audit_event_id` —
  `local_review_findings_store_record_minted`,
  `local_review_findings_store_record_updated`,
  `local_review_findings_store_record_publish_eligibility_changed`,
  `local_review_findings_store_record_diff_fingerprint_invalidated`,
  `local_review_findings_store_record_delete_requested`,
  `local_review_findings_store_record_delete_completed`,
  `local_review_findings_store_record_delete_blocked_on_hold`,
  `local_review_findings_store_record_delete_denied_class_immutable`,
  `local_review_findings_export_packet_assembled`,
  `local_review_findings_export_packet_blocked_redaction_required`,
  `local_review_findings_export_packet_blocked_secret_adjacent`,
  `local_review_findings_store_record_archived`, and
  `local_review_findings_store_audit_denial_emitted`.

Every `*_audit_denial_emitted` event MUST cite the matching
denial-reason value. The schemas enforce this through allOf gates.

## 6. Forbidden collapses

The closed vocabularies above exist so a downstream surface cannot
collapse two distinct refusal states into one bare label. The
following collapses are **forbidden** and a contract-conformant
surface MUST refuse them:

- Rendering a resolution row whose `publish_eligibility_class =
  publish_blocked_provider_write_missing_keep_local_or_export` as
  having published to a destination.
- Painting a resolution row whose
  `resolution_state_class = outdated_diff_changed` or
  `rerun_recommended` as publish-eligible.
- Rendering a resolution row whose
  `resolution_state_class = suppressed_by_policy` without one of
  the policy-source classes
  (`admin_policy_bundle` /
  `repo_instruction_bundle_authored` /
  `designated_policy_file` /
  `trusted_workspace_pinned_policy` /
  `trusted_user_profile_policy`).
- Rendering a resolution row whose
  `resolution_actor_class = ai_review_overlay_recording_only`
  carrying `resolution_state_class = published_to_review_destination`.
- Rendering an export packet whose source row's
  `publish_eligibility_class` is in the `publish_blocked_*` set
  but whose `export_publish_state_class =
  export_records_publish_was_performed`.
- Rendering an export packet whose
  `export_disclosure_class =
  redaction_required_user_must_review_before_export` or
  `export_blocked_secret_adjacent_user_must_review` as having
  crossed the boundary.
- Rendering a local store row whose
  `storage_authority_class = admin_or_control_artifact` carrying a
  `delete_posture_class` outside
  `{delete_denied_class_immutable, delete_blocks_on_hold,
  delete_request_supported_with_destruction_receipt}`.
- Rendering an auto-approve, auto-request-changes, or auto-merge
  transition on the resolution lifecycle.
- Exposing raw outbound text bodies, raw inline-suggestion patch
  bodies, raw check-annotation payloads, raw provider URLs, raw
  provider-thread URLs, raw author identity strings, raw notebook
  cell text, raw diff bodies, raw terminal bytes, raw prompt text,
  raw URLs, raw absolute paths, raw branch / commit URLs, raw API
  keys, raw OAuth tokens, raw mTLS material, raw model weights, or
  raw embeddings on any of the four records.
- Omitting the `analyzed_scope_id_ref`, the
  `review_resolution_id_ref`, the `provider_model_identity` block,
  the `diff_fingerprints` array, or the `policy_context` block on
  any local store row.

## 7. Change discipline

Adding a new resolution-state class, actor class, source class,
reopen-eligibility class, publish-eligibility class, diff-fingerprint
kind, material-change verdict, transition trigger, storage-authority
class, delete-posture class, export-posture class, retention-posture
class, delete-request lifecycle class, export-publish-state class,
export-disclosure class, denial reason, or audit-event id is
**additive-minor** and bumps the matching `*_schema_version`.
Repurposing an existing value is breaking and requires a new
decision row.

## 8. Acceptance mapping

| Acceptance clause | Resolved by |
|---|---|
| Resolution-memory state machine for `open`, `dismissed`, `published`, `outdated`, `suppressed`, and `rerun recommended`, including actor/source, timestamp, reopen action, and audit linkage. | §1 + the `resolution_state_class`, `resolution_actor_class`, `resolution_source_class`, `reopen_eligibility_class`, and `transition_trigger_class` vocabularies + the `review_resolution_state_transition_record` shape with mandatory `transitioned_at` / `actor_ref` / `source_ref` / `reopen_action_admitted` fields + the audit-event id vocabulary. |
| Local-only finding store and export packet preserving analyzed scope, diff fingerprint, provider/model identity, policy/trust context, and publish eligibility when provider write access is absent. | §3 + the `local_review_findings_store_record` required-fields invariants on `analyzed_scope_id_ref` / `diff_fingerprints` / `provider_model_identity` / `policy_context` / `publish_eligibility_class` + the publish-eligibility consistency invariant + §4 + the `export_records_publish_was_not_performed_when_publish_blocked` allOf gate. |
| Material-diff-change rules that move prior findings to `outdated` or `rerun recommended` rather than retaining false freshness. | §2 + the `material_change_verdict_class` vocabulary + the verdict-to-state pairing allOf gates on `review_resolution_material_diff_change_record` + the `outdated_lifecycle_must_block_publish_to_destination` invariant. |
| Provider-write-missing cases remain usable locally and exportable without pretending publication succeeded. | §4.4 (the five-step compound chain) + `fixtures/ai/review_resolution_cases/local_only_review.yaml` and `fixtures/ai/review_resolution_cases/provider_outage.yaml`. |
| Diff changes materially invalidate or downgrade prior findings in fixtures instead of leaving them visually fresh. | §2.3 + `fixtures/ai/review_resolution_cases/changed_diff_with_stale_findings.yaml`. |
| Fixtures cover a local-only review, published finding, suppressed org-policy case, provider outage, and changed diff with stale findings. | `fixtures/ai/review_resolution_cases/local_only_review.yaml`, `published_finding.yaml`, `suppressed_org_policy.yaml`, `provider_outage.yaml`, `changed_diff_with_stale_findings.yaml`. |
