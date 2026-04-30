# AI review-assist finding-row, scope-selector, and publish-to-review contract

This document is the **product-wide contract** for AI review-assist
behavior. It freezes the rule set every AI review-assist surface
reads when it produces a review finding, names the analyzed scope
that produced it, and proposes a path for the result to leave the
local client. It exists to make sure local diff analysis, hosted
review with provider overlays, browser-handoff handoffs, imported
review bundles, and provider-write-missing continuity all resolve
to one shared trust model — instead of each surface inventing its
own freshness language, its own publish vocabulary, or its own
"AI found an issue" copy with no analyzed scope behind it.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / review / hosted-review
surface's mint of its own copy, this document wins and the surface
is non-conforming.

The companion artifacts are:

- [`/schemas/ai/review_finding.schema.json`](../../schemas/ai/review_finding.schema.json)
  — boundary schema for the `review_finding_record` and
  `review_finding_audit_event_record` shapes.
- [`/schemas/ai/review_scope_selection.schema.json`](../../schemas/ai/review_scope_selection.schema.json)
  — boundary schema for the `review_scope_selection_record` and
  `review_scope_selection_audit_event_record` shapes.
- [`/schemas/ai/publish_to_review_sheet.schema.json`](../../schemas/ai/publish_to_review_sheet.schema.json)
  — boundary schema for the `publish_to_review_sheet_record` and
  `publish_to_review_audit_event_record` shapes.
- [`/fixtures/ai/review_assist_cases/`](../../fixtures/ai/review_assist_cases/)
  — worked-example corpus covering at least: a local-diff review
  with a hosted-thread publish, a hosted review with a stale
  base/head forcing the finding into outdated/rerun-recommended,
  a provider-write-missing continuity flow that keeps the finding
  local, and a redaction-required publication that blocks the
  outbound action.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/ai_copy_guardrails_contract.md`](./ai_copy_guardrails_contract.md) —
  evidence-first confidence language, the `Suggested` / `Draft` /
  `Needs review` register, and the rule that low-confidence
  proposals MUST remove direct mutation controls. The review-finding
  row's `confidence_class` mirrors that vocabulary verbatim.
- [`/docs/ai/prompt_injection_and_taint_contract.md`](./prompt_injection_and_taint_contract.md)
  — input-source classes, the taint / trust / precedence model,
  and the rule that designated AI policy files under
  `.aureline/ai/policy/*` are reachable only with admitted signing
  evidence. A review-finding row that names
  `instruction_or_check_source_class = designated_policy_file` MUST
  resolve to a `tainted_input_source_record` carrying
  `policy_file_role_class = designated_policy_file`.
- [`/docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md) —
  provider entry, model entry, execution-locus, region posture,
  retention stance, and the invariant that external-tool output is
  tainted by default. Review-assist runs that crossed a routed
  provider preserve the route disclosure on the matching evidence
  link refs; this contract never re-mints the route copy.
- [`/docs/vcs/review_workspace_contract.md`](../vcs/review_workspace_contract.md) —
  `review_workspace_record`, `review_anchor_record`,
  `merge_queue_action_record`, and the closed source-class /
  freshness / drift / provider-authority vocabularies. This contract
  binds findings, scope rows, and publish sheets to those records;
  no parallel review-state vocabulary is minted here.
- [`/docs/vcs/review_pack_contract.md`](../vcs/review_pack_contract.md) —
  `review_pack_record` and the must-run / advisory / provider-required
  / local-lint check classes. The
  `instruction_or_check_source_class` vocabulary on the finding row
  cites those classes verbatim.
- [`/docs/vcs/hosted_review_and_merge_policy_contract.md`](../vcs/hosted_review_and_merge_policy_contract.md) —
  hosted-review inbox / merge-policy forward-dependency slots. The
  finding row, scope row, and publish sheet reserve
  `hosted_review_inbox_record_id_ref` slots that resolve once that
  contract lands.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state, policy epoch, and trust state on every
  record.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md) —
  the broker-owned redaction pass runs before any preview text or
  patch preview crosses this boundary. The publish-to-review sheet's
  `redaction_note_class` records what the broker found.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md) —
  admin policy MAY narrow which finding classes, destination classes,
  and provider-write paths the AI review-assist surface admits;
  policy MAY NOT silently widen.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  approval-ticket vocabulary. Every `publish_to_destination` action
  on the publish-to-review sheet MUST cite a non-null
  `approval_ticket_ref` through the same path native review tools
  use.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  `freshness_class`, `client_scope`, `redaction_class` re-exported
  without modification.

If this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does **not** ship a model, a review-engine runtime,
or any provider-write code. It freezes the contract those
implementations will read and write. The eventual review-assist
crate's Rust types are the schema of record; the JSON Schema
exports are the cross-tool boundary every non-owning surface reads.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.8.14 (AI
  review assist, diff-scoped checks, and publish-to-review contract).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §15.20 (AI review
  assist, diff-scoped checks, and publish-to-review UX) and Appendix
  ED.4 / ED.5 / ED.6.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` §23.17 and
  Appendix BO.3.
- `.t2/docs/Aureline_PRD.md` AI-powered code review row.

If this contract disagrees with those sources, those sources win.

## Why freeze this now

Three problems compound the moment AI review-assist surfaces start
landing without one shared vocabulary:

1. **Findings without an analyzed scope.** A reviewer who reads
   "AI found a potential null-path here" cannot tell whether the
   surface analyzed the selected diff, the entire branch, the
   uncommitted changes, or a hosted pull request. Two reviewers
   reading the same finding against two different scopes draw two
   different conclusions about whether to act.
2. **Findings without a directing instruction or check source.**
   A finding that originates from a repo-authored AGENTS.md-style
   bundle, a designated AI policy file under `.aureline/ai/policy/`,
   a review-pack must-run check, an advisory check, a provider
   required-check mirror, or a local lint diagnostic is *not the
   same kind of finding* as one the AI overlay produced from its
   own reasoning with no rule directing it. Surfacing them through
   one undifferentiated badge means the user cannot tell which
   findings have policy weight and which are model prose.
3. **Publish flows that hide where the result is going.** "Post
   suggestion" without disclosing whether the result becomes a
   thread comment, an inline suggested patch, a provider-specific
   check annotation, a summary review body, an imported-bundle
   export packet, or a support packet means the user cannot give
   informed consent to the outbound action — and a hosted provider
   that silently rewrites the payload (e.g. quoting raw diff bytes)
   exposes redaction failures the user never saw.

Closing all three with one record family — one finding row, one
scope-selection row, one publish-to-review sheet — solves the
classification problem at the boundary, not at every consuming
surface.

## Who reads this document

- **AI review-assist authors** classifying every finding into the
  closed `finding_class`, naming the analyzed scope, citing the
  directing instruction-or-check source, and admitting the closed
  `local_or_publish_action_class` set.
- **Review-workspace / hosted-review / browser-handoff surface
  authors** consuming `review_finding_record` rows alongside their
  own `review_anchor_record` rows.
- **Publish / outbound surface authors** rendering the
  `publish_to_review_sheet_record` exact outbound preview, attribution,
  redaction note, and provider-write-continuity gate.
- **Admin / policy / settings surface authors** narrowing which
  finding classes, destination classes, attribution paths, and
  provider-write classes are admitted per deployment profile.
- **Evidence / replay / support / parity-audit authors** quoting
  the finding-class, scope-class, and publish-action vocabulary
  mechanically rather than re-deriving the contract.

## 1. Review finding row

Every AI review-assist run that produces a finding emits exactly
one `review_finding_record` per finding. The schema
(`review_finding.schema.json`) freezes the row shape.

### 1.1 Finding class

The finding class is closed. A surface that observes an
unrecognised class denies with `review_finding_record_unknown_value`.

| Class | Meaning |
|---|---|
| `risk_or_bug_concern` | The headline "this might be a bug" class. |
| `style_or_convention_drift` | Convention deviation against repo / pack rules. |
| `missing_test_coverage` | Coverage hint against the analyzed scope. |
| `dependency_or_security_concern` | Dependency or security suggestion. |
| `impact_area_hint` | Blast-radius hint (where else might this affect?). |
| `repo_instruction_check_fired` | The repo-authored instruction bundle or designated policy file directly drove the finding. |
| `docs_or_comment_drift` | Stale docs or comment drift against the scope. |
| `accessibility_or_ux_concern` | Accessibility / UX issue. |
| `performance_or_complexity_concern` | Performance / complexity hint. |
| `unknown_must_review` | Fail-closed; the surface could not classify the finding and routes to user review. |

### 1.2 Severity and confidence

Severity classes are the AI review-assist projection of the
review-pack finding-severity vocabulary. **Every value carries
the explicit `_advisory` suffix on every blocking-shaped class**
because AI review-assist findings are advisory-only — the human
review decision remains explicit, and the matching enforced check
(if any) lives on the review-pack evaluation, not on this row.

```
severity_blocker_advisory
severity_high_advisory
severity_medium_advisory
severity_low_advisory
severity_informational
```

Rule: a row that renders `severity_blocker_advisory` as if it
blocked landing on its own denies with
`severity_blocker_advisory_must_not_imply_landing_block`.

Confidence classes mirror the AI copy-guardrails approved labels
verbatim:

```
evidence_backed
inferred
low_confidence
validation_passed
validation_not_run
needs_review
```

Rule: numerical confidence ("82% confident") is forbidden unless
the value comes from a calibrated non-AI evaluator and is cited
through `evidence_link_refs` (per the AI copy-guardrails contract).

### 1.3 Analyzed scope and instruction-or-check source

Two refs make "AI found an issue without scope and policy context"
structurally impossible:

- `analyzed_scope_id_ref` — required (non-null) opaque ref to the
  matching `review_scope_selection_record`. A missing ref denies
  with `analyzed_scope_ref_required`.
- `instruction_or_check_source_class` — closed vocabulary naming
  the directing source. When the value names a directing source
  (everything except `ai_review_overlay_only_no_directing_source`
  and `not_applicable_no_directing_source`), the row MUST cite a
  non-null `instruction_or_check_source_ref`. A missing ref denies
  with `instruction_or_check_source_class_required`.

The `instruction_or_check_source_class` set is:

```
repo_instruction_bundle_authored
designated_policy_file
trusted_workspace_pinned_policy
trusted_user_profile_policy
review_pack_must_run_check
review_pack_advisory_check
provider_required_check_mirror
local_lint_or_diagnostic
ai_review_overlay_only_no_directing_source
not_applicable_no_directing_source
```

`designated_policy_file` is reachable only when the upstream
`tainted_input_source_record` for the same assembly carries
`policy_file_role_class = designated_policy_file` and a non-null
`signing_evidence_ref` (per the prompt-injection contract). A row
that claims `designated_policy_file` without the signed input
denies with `designated_policy_file_source_requires_signed_policy_input`.

### 1.4 Evidence links

Every finding cites at least one evidence link. The schema enforces
this through `minItems: 1` on `evidence_link_refs`; a row with no
evidence links denies with `evidence_link_required_at_least_one`.

The `evidence_link_kind` set is closed:

```
affected_file_or_hunk_ref
cited_repository_workspace_content_ref
cited_workspace_diagnostic_capture_ref
cited_review_pack_check_ref
cited_repository_instruction_bundle_ref
cited_designated_policy_file_ref
cited_evidence_packet_ref
cited_docs_pack_excerpt_ref
cited_runbook_step_excerpt_ref
cited_workspace_search_result_ref
cited_review_anchor_ref
cited_review_evaluation_finding_ref
```

Raw bodies / paths / URLs never cross this boundary; every value
is an opaque ref into the owning contract.

### 1.5 Lifecycle and dismissal rationale

The lifecycle vocabulary is closed and does **not** contain any
auto-approve, auto-request-changes, or auto-merge transition. A
surface that attempts to mint such a transition denies with
`auto_approve_or_request_changes_or_merge_forbidden`.

```
open
dismissed
published_local_only
published_to_review_destination
outdated_diff_changed
rerun_recommended
superseded_by_rerun
suppressed_by_policy
archived_tombstone
```

Rules:

1. `published_to_review_destination` and `published_local_only`
   MUST cite a non-null `publish_to_review_sheet_id_ref`. A row
   without that ref denies with
   `publish_to_review_sheet_ref_required_for_published_destination`.
2. `outdated_diff_changed` and `rerun_recommended` MUST NOT admit
   `publish_to_review_destination` as the proposed action; the
   row routes through `mark_outdated_pending_rerun` /
   `request_rerun` / `keep_local` / `dismiss_with_rationale` /
   `open_diff` / `open_source` / `export_local_packet` /
   `archive_tombstone`. A mismatch denies with
   `outdated_lifecycle_must_not_admit_publish_action`.
3. `dismissed` and `suppressed_by_policy` MUST cite a
   `dismissal_rationale_class` other than
   `not_applicable_dismissal_not_recorded`. A missing rationale
   denies with `dismissal_rationale_required_for_dismissed_or_suppressed`.
4. The dismissal-rationale vocabulary is recorded without shaming
   or anthropomorphic copy. The closed values are
   `dismissed_user_disagrees_evidence_insufficient`,
   `dismissed_user_acknowledges_acceptable_risk`,
   `dismissed_user_planned_in_separate_change`,
   `dismissed_duplicate_of_other_finding`,
   `dismissed_outdated_diff_changed_after_run`,
   `suppressed_by_policy_bundle_admin`, and
   `suppressed_by_repo_instruction_bundle`.
5. `archived_tombstone` MUST cite a non-null `archived_at`.

### 1.6 Local-or-publish action and attribution

The action vocabulary is closed:

```
keep_local
publish_to_review_destination
dismiss_with_rationale
export_local_packet
mark_outdated_pending_rerun
request_rerun
open_diff
open_source
archive_tombstone
```

Rules:

1. `confidence_class = low_confidence` MUST NOT admit
   `publish_to_review_destination` as the proposed action. A
   mismatch denies with
   `low_confidence_must_not_admit_publish_to_review_destination`.
   Low-confidence findings route through `keep_local`,
   `dismiss_with_rationale`, `export_local_packet`,
   `mark_outdated_pending_rerun`, `request_rerun`, `open_diff`,
   `open_source`, or `archive_tombstone` until the user lifts the
   confidence floor or a rerun moves it.
2. `publish_to_review_destination` MUST cite a non-null
   `publish_to_review_sheet_id_ref`.
3. The attribution-class vocabulary is `posted_as_user_with_ai_assist`,
   `posted_as_ai_review_overlay`, `kept_local_no_attribution`, or
   the recording-only `forbidden_silent_auto_post`. A surface that
   carries `forbidden_silent_auto_post` MUST NOT admit
   `publish_to_review_destination` as the proposed action; the
   matching publish sheet records
   `silent_auto_post_forbidden_human_decision_required`.

## 2. Review scope selector

Every AI review-assist run is bound to exactly one
`review_scope_selection_record`. The schema
(`review_scope_selection.schema.json`) freezes the row shape.

### 2.1 Scope class

The scope-class vocabulary is closed:

```
selected_diff_in_local_workspace
uncommitted_changes_in_local_workspace
whole_branch_against_base
hosted_review_object
imported_review_bundle_scope
browser_handoff_token_scope
composite_local_with_provider_overlay_scope
```

Rules:

1. `selected_diff_in_local_workspace`,
   `uncommitted_changes_in_local_workspace`,
   `whole_branch_against_base`, and
   `composite_local_with_provider_overlay_scope` MUST cite a
   non-null `local_locator`.
2. `hosted_review_object` and
   `composite_local_with_provider_overlay_scope` MUST cite a
   non-null `provider_overlay_ref` into the bound
   `review_workspace_record`.
3. `imported_review_bundle_scope` MUST cite a non-null
   `imported_bundle_envelope_ref`; `browser_handoff_token_scope`
   MUST cite a non-null `browser_handoff_packet_ref`.

### 2.2 Base/head context

Every scope row carries exactly one base/head kind:

```
base_head_revision_pair
base_head_branch_pair
base_head_uncommitted_against_index
base_provider_pull_or_merge_request
base_imported_bundle_envelope
base_browser_handoff_packet
```

The matching scope_class ↔ base_head_context_kind pairings are
schema-enforced through allOf gates.

### 2.3 Scope freshness

The per-run freshness vocabulary is closed and matches the
review-workspace overlay-freshness posture mechanically:

```
scope_fresh_no_material_change_since_run
scope_warm_minor_unrelated_change_since_run
scope_diff_changed_after_run_outdated
scope_base_or_head_changed_after_run_outdated
scope_provider_overlay_refreshed_rerun_recommended
scope_provider_overlay_unavailable_local_continues
scope_imported_bundle_pinned_no_drift_assessable
scope_browser_handoff_pinned_no_drift_assessable
scope_unverifiable_evaluator_outage_rerun_required
```

Rules:

1. `scope_diff_changed_after_run_outdated` and
   `scope_base_or_head_changed_after_run_outdated` force matching
   findings into `outdated_diff_changed` / `rerun_recommended` /
   `superseded_by_rerun` lifecycle. A surface that publishes a
   finding under a `scope_*_outdated` posture without honouring
   the lifecycle gate denies with
   `outdated_lifecycle_must_not_admit_publish_action` on the
   matching `review_finding_record`.
2. `scope_provider_overlay_unavailable_local_continues` honours
   the workspace's `provider_overlay_unavailable_local_continues`
   posture; the workspace MUST resolve `provider_authority_class`
   to `local_truth_only_no_provider_overlay` or
   `local_parity_estimate` (per review-workspace contract §1.4).
3. `scope_imported_bundle_pinned_no_drift_assessable` and
   `scope_browser_handoff_pinned_no_drift_assessable` record the
   pin so a reused result is not painted as freshly-evaluated.

### 2.4 Scope lineage and rerun reason

Every scope row carries exactly one `scope_lineage_class` paired
through allOf with the matching `rerun_reason_class`:

```
freshly_minted_no_predecessor          ↔ not_a_rerun_initial_run
derived_from_prior_scope_diff_changed  ↔ diff_changed_after_run_rerun_required
derived_from_prior_scope_base_head_changed ↔ base_or_head_changed_after_run_rerun_required
derived_from_prior_scope_provider_overlay_refreshed ↔ provider_overlay_refreshed_rerun_recommended
derived_from_prior_scope_user_widened  ↔ user_widened_scope
derived_from_prior_scope_user_narrowed ↔ user_narrowed_scope
derived_from_prior_scope_rerun_no_change ↔ user_requested_rerun_no_change
derived_from_prior_scope_policy_or_pack_updated ↔ policy_or_pack_updated_rerun_required
```

A row whose lineage and rerun reason disagree denies with
`rerun_reason_must_match_lineage`. Every `derived_from_*` value
MUST cite a non-null `predecessor_scope_id_ref`;
`freshly_minted_no_predecessor` MUST cite a null one (denials
`predecessor_required_for_derived_lineage` and
`predecessor_forbidden_for_freshly_minted_lineage`).

## 3. Publish-to-review sheet

Every publish proposal a finding triggers is bound to exactly one
`publish_to_review_sheet_record`. The schema
(`publish_to_review_sheet.schema.json`) freezes the row shape.

### 3.1 Outbound destination and content kind

The destination-class vocabulary is closed:

```
local_keep_only_no_outbound
hosted_review_thread_comment
hosted_review_inline_suggested_patch
hosted_review_provider_check_annotation
hosted_review_summary_review_body
local_review_workspace_anchor_only
imported_review_bundle_export_packet
support_export_packet_only
```

The content-kind vocabulary pairs to the destination through allOf
gates so the sheet can never publish a thread comment as if it
were a check annotation, or vice versa:

| Destination | Required content kind |
|---|---|
| `hosted_review_thread_comment` | `review_comment_text` |
| `hosted_review_inline_suggested_patch` | `inline_suggestion_patch_preview` |
| `hosted_review_provider_check_annotation` | `provider_check_annotation_payload` |
| `hosted_review_summary_review_body` | `summary_review_body_text` |
| `local_review_workspace_anchor_only` | `local_anchor_label_only` |
| `local_keep_only_no_outbound` | `local_anchor_label_only` |
| `imported_review_bundle_export_packet` | `bundle_export_packet` |
| `support_export_packet_only` | `support_export_packet` |

A mismatch denies with `outbound_content_kind_does_not_match_destination`.

Hosted destinations MUST cite a non-null `publish_target_ref`
(opaque ref to the provider-side thread / inline target / check
run identity); raw provider URLs and raw provider-thread ids never
appear here.

### 3.2 Outbound preview

The sheet exposes the **exact** outbound preview the user reviews
before publish through two redaction-aware reviewable labels:

- `outbound_text_preview_label` — required for
  `review_comment_text`, `summary_review_body_text`, and
  `local_anchor_label_only`.
- `outbound_patch_preview_label` — required for
  `inline_suggestion_patch_preview`.

Both labels respect the broker-owned redaction pass (ADR-0007).
Raw outbound bytes never cross this boundary.

### 3.3 Attribution state

The attribution-state vocabulary is closed:

```
posted_as_user_with_ai_assist_disclosed
posted_as_user_with_ai_assist_disclosure_pending
posted_as_ai_review_overlay_disclosed
kept_local_no_attribution
forbidden_silent_auto_post
```

Rules:

1. `posted_as_user_with_ai_assist_disclosure_pending` MUST refuse
   `publish_to_destination`; the action routes to `keep_local`,
   `copy_only_no_outbound`, `export_local_packet`, or
   `cancel_no_outbound` until the user confirms or removes the
   AI-assist mark. A direct publish under disclosure-pending denies
   with `ai_assist_disclosure_required_before_publish`.
2. `forbidden_silent_auto_post` is recording-only and MUST coexist
   with `publish_or_copy_or_export_action_class =
   block_publish_silent_auto_post_forbidden`. The matching audit
   event records
   `silent_auto_post_forbidden_human_decision_required`.

### 3.4 Redaction note

The redaction-note vocabulary records what the broker found:

```
no_redaction_required
internal_identifier_redacted
credential_handle_redacted_no_raw_secret
private_endpoint_redacted
personal_data_redacted
policy_blocked_field_redacted
redaction_required_user_must_review
redaction_pass_failed_publish_blocked
```

Rules:

1. `redaction_required_user_must_review` MUST refuse
   `publish_to_destination` until the user reviews the broker's
   findings. A direct publish denies with
   `redaction_required_user_must_review_before_publish`.
2. `redaction_pass_failed_publish_blocked` MUST pair with
   `publish_or_copy_or_export_action_class =
   block_publish_redaction_failed_user_must_review`. The matching
   audit event records `redaction_pass_failed_publish_blocked`.

### 3.5 Action class and provider-write continuity

The action vocabulary is closed:

```
publish_to_destination
copy_only_no_outbound
export_local_packet
keep_local
cancel_no_outbound
block_publish_provider_write_missing_keep_local
block_publish_redaction_failed_user_must_review
block_publish_silent_auto_post_forbidden
```

The provider-write-continuity vocabulary is closed:

```
provider_write_admitted
provider_write_admitted_under_browser_handoff
provider_write_admitted_under_browser_handoff_pending
provider_write_admitted_via_imported_bundle_export_packet
provider_write_missing_keep_local_or_export
provider_write_missing_imported_bundle_no_outbound_admitted
provider_write_missing_browser_handoff_token_only_no_outbound_admitted
provider_write_unknown_must_block
```

Rules:

1. `publish_to_destination` is the only mutating action. It MUST
   cite a non-null `approval_ticket_ref` and MUST pair with
   `provider_write_continuity_class` in
   `{provider_write_admitted, provider_write_admitted_under_browser_handoff,
   provider_write_admitted_via_imported_bundle_export_packet}`.
   A missing approval ticket denies with
   `approval_ticket_required_for_publish_to_destination`.
2. `provider_write_missing_keep_local_or_export` MUST refuse
   `publish_to_destination` and admit only `keep_local`,
   `copy_only_no_outbound`, `export_local_packet`,
   `block_publish_provider_write_missing_keep_local`, or
   `cancel_no_outbound`. A direct publish denies with
   `provider_write_missing_publish_blocked_keep_local_or_export`.
3. `provider_write_admitted_under_browser_handoff_pending` MUST
   refuse `publish_to_destination` until the handoff completes.
4. The contract NEVER admits an auto-approve, auto-request-changes,
   or auto-merge action through this sheet. A surface that attempts
   to mint such an action denies with
   `auto_approve_or_request_changes_or_merge_forbidden`.

## 4. Composition with other contracts

Findings, scope rows, and publish sheets do not stand alone. Each
record cites the owning contracts:

| Cross-contract relation | How the finding row resolves |
|---|---|
| Workspace identity | `review_workspace_id_ref` into `schemas/vcs/review_workspace.schema.json`. |
| Anchored comment that carries the finding | optional `review_anchor_id_ref` into `schemas/vcs/review_anchor.schema.json`. |
| Review-pack check that fired | `instruction_or_check_source_class` + `instruction_or_check_source_ref` into `schemas/vcs/review_pack.schema.json`. |
| Tainted-input safeguard for designated policy files | upstream `tainted_input_source_record` carries `policy_file_role_class = designated_policy_file`. |
| Approval ticket admitting the publish | `approval_ticket_ref` into `schemas/integration/approval_ticket.schema.json`. |
| Hosted-review inbox forward dependency | `hosted_review_inbox_record_id_ref` slot reserved on every record (currently always null). |

A finding row whose `instruction_or_check_source_class =
review_pack_must_run_check` MUST cite a `review_pack_record` whose
`review_pack_check.review_pack_check_enforcement_class` resolves
to a must-run class; a row whose source class is
`review_pack_advisory_check` MUST cite the matching advisory check.
The composition is enforced by the consuming evaluator — the
review_finding schema records the requirement on the denial
vocabulary and on the cited refs.

## 5. Audit-event vocabulary

Each schema exports its own audit-event id set:

- `review_finding_audit_event_id` — `review_finding_minted`,
  `review_finding_evidence_link_recorded`,
  `review_finding_lifecycle_state_changed`,
  `review_finding_published_locally`,
  `review_finding_published_to_destination`,
  `review_finding_dismissed`, `review_finding_suppressed_by_policy`,
  `review_finding_marked_outdated_after_diff_change`,
  `review_finding_rerun_requested`,
  `review_finding_superseded_by_rerun`, `review_finding_archived`,
  and `review_finding_audit_denial_emitted`.
- `review_scope_selection_audit_event_id` —
  `review_scope_selection_minted`,
  `review_scope_selection_freshness_changed`,
  `review_scope_selection_lineage_recorded`,
  `review_scope_selection_outdated_after_diff_change`,
  `review_scope_selection_outdated_after_base_or_head_change`,
  `review_scope_selection_provider_overlay_refreshed`,
  `review_scope_selection_archived`, and
  `review_scope_selection_audit_denial_emitted`.
- `publish_to_review_audit_event_id` —
  `publish_to_review_sheet_minted`,
  `publish_to_review_sheet_outbound_preview_rendered`,
  `publish_to_review_sheet_redaction_pass_completed`,
  `publish_to_review_sheet_published_to_destination`,
  `publish_to_review_sheet_kept_local`,
  `publish_to_review_sheet_copied_only`,
  `publish_to_review_sheet_exported_local_packet`,
  `publish_to_review_sheet_cancelled`,
  `publish_to_review_sheet_provider_write_missing_observed`,
  `publish_to_review_sheet_redaction_failed_observed`,
  `publish_to_review_sheet_silent_auto_post_blocked`, and
  `publish_to_review_sheet_audit_denial_emitted`.

Every `*_audit_denial_emitted` event MUST cite the matching
denial-reason value. The schemas enforce this through allOf gates.

## 6. Forbidden collapses

The closed vocabularies above exist so a downstream surface cannot
collapse two distinct refusal states into one bare label. The
following collapses are **forbidden** and a contract-conformant
surface MUST refuse them:

- Rendering a `low_confidence` finding with
  `local_or_publish_action_class = publish_to_review_destination`.
- Rendering a `severity_blocker_advisory` finding as if it blocked
  landing on its own.
- Rendering a `published_to_review_destination` finding without
  citing a `publish_to_review_sheet_id_ref`.
- Rendering an `outdated_diff_changed` or `rerun_recommended`
  finding with a publish action.
- Rendering an `instruction_or_check_source_class =
  designated_policy_file` finding without an upstream
  `tainted_input_source_record` carrying signed-policy authority.
- Rendering a hosted-thread comment as an inline suggested patch
  (or vice versa).
- Rendering a `provider_write_missing_keep_local_or_export` sheet
  with a `publish_to_destination` action.
- Rendering a `redaction_pass_failed_publish_blocked` sheet with
  any action other than
  `block_publish_redaction_failed_user_must_review`.
- Rendering a `posted_as_user_with_ai_assist_disclosure_pending`
  sheet with a `publish_to_destination` action.
- Rendering an auto-approve, auto-request-changes, or auto-merge
  transition on the finding lifecycle.
- Exposing raw outbound text bodies, raw inline suggestion patch
  bodies, raw check annotation payloads, raw provider URLs, raw
  provider thread URLs, raw author identity strings, raw notebook
  cell text, raw diff bodies, raw terminal bytes, raw prompt text,
  or raw URLs on any of the three records.
- Omitting the analyzed scope ref or the instruction-or-check
  source on any finding row.

## 7. Change discipline

Adding a new finding class, severity class, confidence class,
evidence-link kind, instruction-or-check-source class, lifecycle
state, dismissal rationale, action class, attribution class,
scope class, base/head kind, scope-freshness class, scope-lineage
class, rerun reason, destination class, content kind, redaction
note, publish action class, provider-write-continuity class,
denial reason, or audit-event id is **additive-minor** and bumps
the matching `*_schema_version`. Repurposing an existing value is
breaking and requires a new decision row.

## 8. Acceptance mapping

| Acceptance clause | Resolved by |
|---|---|
| Every finding names the analyzed scope and the instruction or check source that shaped it. | §1.3 + the `analyzed_scope_id_ref` / `instruction_or_check_source_class` required-fields invariants on the schema, plus the `analyzed_scope_ref_required` and `instruction_or_check_source_class_required` denials. |
| Publish flows preview the exact outbound destination and content instead of hiding whether the result becomes a comment, suggestion, or provider-specific annotation. | §3.1 + the per-destination content-kind allOf gates and the `outbound_text_preview_label` / `outbound_patch_preview_label` required-fields invariants. |
| Fixtures cover local diff review, hosted review with stale base/head, provider-write-missing continuity, and redaction-required publication. | `/fixtures/ai/review_assist_cases/local_diff_review_with_publish.yaml`, `hosted_review_stale_base_head_outdated.yaml`, `provider_write_missing_keep_local.yaml`, and `redaction_required_publication_blocked.yaml`. |
