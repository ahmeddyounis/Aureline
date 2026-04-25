# Review-pack, ownership-policy, and local / CI / provider / AI parity contract

This document freezes the declarative review-policy layer Aureline
binds every reviewer-facing surface to. Local review (a checked-out
branch under the user's clone), CI runners, hosted-review provider
overlays, AI review overlays, browser companions, and support / export
readers all evaluate against one `review_pack_record` and publish one
`review_evaluation_result_record` per evaluation. The pack declares
which checks MUST run, which checks are advisory, which paths require
which ownership signoff, which divergence labels its consumers MUST
surface, which waivers can suppress which check, and which
output-artifact families an evaluation publishes. The result is the
normalized outcome model every evaluator emits — so no surface mints
a parallel evaluator-state vocabulary, no surface relabels a missing
evaluator as "green by omission", and no surface buries an enforced
ownership requirement inside tool-specific copy.

The machine-readable boundaries are:

- [`/schemas/vcs/review_pack.schema.json`](../../schemas/vcs/review_pack.schema.json)
  — the `review_pack_record` shape, the per-pack
  `review_pack_check_descriptor` and `ownership_rule_descriptor` rows
  it composes, and the closed pack-class / lifecycle / check-class /
  enforcement-class / ownership-rule-class / divergence-label-class /
  waiver-linkage-class / output-artifact-family-class vocabularies.
- [`/schemas/vcs/review_evaluation_result.schema.json`](../../schemas/vcs/review_evaluation_result.schema.json)
  — the `review_evaluation_result_record` and
  `review_evaluation_finding_record` shapes, the closed
  evaluation-source-class, evaluation-lifecycle-state,
  check-outcome-class, parity-state-class, evaluation-staleness-class,
  reuse-provenance-class, ownership-outcome-class, finding-severity
  vocabularies, and the closed denial-reason vocabulary that forbids
  the green-by-omission, silent-waiver, divergence-mislabel,
  reuse-provenance-missing, and policy-epoch-stale failure modes.

Worked cases (a project-default pack with format / lint / type / unit
test checks plus an ownership rule; an owner-supplied overlay pack;
an ad-hoc-session pack that is not promoted; an imported-from-bundle
pack; a local-workstation evaluation that all three evaluators
match-pass; a CI evaluation that disagrees with the local result; a
provider overlay evaluation that disagrees with local; a result
reused from a support export pinned to its captured build identity;
an AI review overlay finding that is advisory non-blocking; a result
that omits a declared check and trips
`green_by_omission_forbidden_evaluation_must_be_explicit`; a finding
that asserts a different enforcement class than the pack declared
and trips `check_enforcement_class_must_match_pack_declaration`; a
result whose reuse refused because the captured build identity
drifted; a finding that asserts an ownership outcome without a
matching ownership rule and trips
`ownership_outcome_must_match_ownership_rule_class`) live under
[`/fixtures/vcs/review_pack_cases/`](../../fixtures/vcs/review_pack_cases/).

The eventual review-pack and review-evaluation crates' Rust types are
the schema of record. This document and the JSON Schema exports are
the cross-tool boundary every non-owning surface reads. The
review-pack runner, the CI / provider / AI evaluators, the
hosted-review inbox surface, and the merge-policy resolver are
out-of-scope downstream lanes; this contract reserves the data model
they bind to.

Companion artifacts:

- [`/schemas/vcs/review_workspace.schema.json`](../../schemas/vcs/review_workspace.schema.json)
  and
  [`/docs/vcs/review_workspace_contract.md`](review_workspace_contract.md)
  — the review-workspace, comment-anchor, and merge-queue contract.
  Every `review_evaluation_result_record` cites
  `target_review_workspace_id_ref` into this contract; the workspace
  owns the `provider_overlay_freshness_class` chip and this contract
  re-uses it through the matched `parity_state_class` and
  `evaluation_staleness_class` values.
- [`/schemas/vcs/review_anchor.schema.json`](../../schemas/vcs/review_anchor.schema.json)
  — the per-finding anchor target. A finding MAY cite a
  `review_anchor_record` rather than re-mint the anchor.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — the approval-ticket model the matched
  `merge_queue_action_record` cites when the evaluation result
  participates in landing.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — the browser-companion handoff packet model every
  `browser_companion_evaluation` source resolves to. Raw URLs never
  cross this boundary.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — re-exported `freshness_class`, `client_scope`, and
  `redaction_class` vocabularies (ADR-0011).
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — re-exported `workspace_trust_state_class` and policy / trust
  envelope (ADR-0001 / ADR-0018). An evaluation never appears
  available under an unset trust decision.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — the owner-route registry every `ownership_rule_descriptor` cites
  through `owner_route_ref`. Raw owner identity strings never appear
  here.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — review
  policy, evaluator parity, and ownership-rule matrix.
- `.t2/docs/Aureline_PRD.md` — review-pack MUST/SHOULD language for
  evaluator parity, advisory-vs-enforced split, durable waivers, and
  honest divergence labels.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same
change.

## Why freeze this now

Until this contract lands, every surface that touches a review
evaluation would be free to invent its own evaluator-state
vocabulary:

- A local review surface could mint per-tool "passed" / "failed"
  enums, with no shared way to say "local passes but CI is still
  pending". A reviewer would have to interpret tool-specific copy to
  know whether a green chip on the local surface means the same
  thing as a green chip on the CI surface.
- A CI evaluator could omit a check the project-default pack
  declared and the result would render as "green by omission",
  because nothing forced the evaluator to publish a typed
  `skipped_*` outcome for the missing check. A landing flow that
  trusted that result would land code that bypassed the policy.
- A provider overlay could publish a "merged" cue without naming
  which checks it ran or which ownership signoff it admitted, so the
  reviewer could not tell whether the cue reflected the same policy
  the local clone evaluated against.
- An AI review overlay could surface findings as if they were the
  same evidence class as a CI run, blurring the advisory boundary
  between AI suggestions and enforced checks.
- A browser companion could publish a "passed" handoff packet
  pinned to a captured build identity that no longer matches the
  current branch, and the reviewer would see the cue without any
  notice that the result was a reuse, not a fresh evaluation.
- A support / export reader could re-render a captured result as if
  it were a current evaluation, hiding the captured-build-identity
  pin and the staleness against the current build.
- Ownership rules would be expressed in tool-specific copy ("this
  PR needs a review from team-x"), with no machine-readable way to
  say "ownership signoff is enforced-blocking" versus "ownership
  notification is advisory".

Freezing one record family
(`review_pack_record`, `review_evaluation_result_record`,
`review_evaluation_finding_record`) and the closed enforcement,
divergence, parity, staleness, and reuse-provenance vocabularies
they read solves all seven problems in one shape.

## Scope

Frozen at this revision:

1. The `review_pack_record` shape every evaluator reads to know what
   to evaluate, including:
   - one `review_pack_class` from the closed six-value vocabulary
     (`project_default_pack`, `scope_overlay_pack`,
     `owner_supplied_pack`, `ad_hoc_session_pack`,
     `imported_from_bundle_pack`,
     `managed_admin_published_pack`);
   - one `review_pack_lifecycle_state` from the closed four-value
     vocabulary
     (`review_pack_drafting`, `review_pack_active`,
     `review_pack_deprecated_pending_replacement`,
     `review_pack_archived_tombstone`);
   - one `scope_binding` naming one of `whole_repository_scope`,
     `subtree_scope`, `sparse_slice_scope`, `named_workset_scope`,
     `ad_hoc_session_scope` plus an opaque `scope_target_ref`;
   - a non-empty `checks` list of `review_pack_check_descriptor`
     rows naming one of the twelve `review_pack_check_class` values
     (`format_check`, `lint_check`, `type_check`, `unit_test_check`,
     `integration_test_check`, `security_audit_check`,
     `license_audit_check`, `dependency_freshness_check`,
     `ownership_review_required_check`, `ai_review_advisory_check`,
     `evidence_attached_check`, `custom_external_check`), one of
     the five `review_pack_check_enforcement_class` values
     (`enforced_blocking`, `enforced_blocking_unless_waived`,
     `advisory_non_blocking_with_visible_chip`,
     `advisory_non_blocking_silent`, `informational_only_no_chip`),
     and one of the seven `waiver_linkage_class` values
     (`no_waiver_admissible`, `waiver_required_admin_signed`,
     `waiver_required_owner_signed`, `waiver_required_with_expiry`,
     `waiver_admitted_active`,
     `waiver_expired_re_evaluation_required`,
     `waiver_revoked_re_evaluation_required`) under allOf gates that
     force `enforced_blocking` to pair with `no_waiver_admissible`
     and `enforced_blocking_unless_waived` to pair with one of the
     three `waiver_required_*` values;
   - an `ownership_rules` list of `ownership_rule_descriptor` rows
     naming one of the five `ownership_rule_class` values
     (`ownership_required_review_blocking`,
     `ownership_required_signoff_blocking`,
     `ownership_advisory_notification_non_blocking`,
     `ownership_optional_notification_no_chip`,
     `ownership_disabled_for_scope`), an opaque `owner_route_ref`
     into the owner-route registry, and a sub-`scope_binding`;
   - a non-empty `divergence_labels_required` set drawn from the
     nine `divergence_label_class` values so the pack's consumers
     are never surprised by a label they were not prepared to
     surface;
   - a non-empty `output_artifact_families_required` set drawn from
     the seven `output_artifact_family_class` values so support /
     export readers know which families an evaluation MUST publish;
   - the `policy_context` (epoch + workspace trust state) and the
     `redaction_class` the pack publishes under;
   - the `client_scopes` the pack is admitted on;
   - the audit-event row shape on the `review_pack` audit stream
     (`review_pack_drafted`, `review_pack_admitted_active`,
     `review_pack_check_added`, `review_pack_check_removed`,
     `review_pack_ownership_rule_added`,
     `review_pack_ownership_rule_removed`,
     `review_pack_waiver_linkage_changed`,
     `review_pack_deprecated_pending_replacement`,
     `review_pack_archived`,
     `review_pack_audit_denial_emitted`).

2. The `review_evaluation_result_record` shape every evaluator
   publishes, including:
   - the `review_pack_id_ref` it evaluates against (required
     non-null; a missing ref denies with
     `evaluation_must_pin_review_pack_id_ref`);
   - the `target_review_workspace_id_ref` into
     `review_workspace_record`;
   - one `evaluation_source_class` from the closed six-value
     vocabulary
     (`local_workstation_evaluation`, `ci_pipeline_evaluation`,
     `provider_overlay_evaluation`,
     `ai_review_overlay_evaluation`,
     `browser_companion_evaluation`, `export_reuse_evaluation`);
   - one `evaluation_lifecycle_state` from the closed six-value
     vocabulary
     (`evaluation_pending_admission`, `evaluation_running`,
     `evaluation_completed`, `evaluation_failed_with_error`,
     `evaluation_cancelled`,
     `evaluation_superseded_by_re_evaluation`);
   - the `evaluator_descriptor_ref` (the tool / runner / external
     service identity that produced the result);
   - the `running_build_identity_ref` the evaluation was produced
     against (required non-null; a missing ref denies with
     `evaluation_must_pin_running_build_identity`);
   - one `parity_state_class` from the closed ten-value vocabulary
     (`parity_match_all_evaluators_agree_pass`,
     `parity_match_all_evaluators_agree_fail`,
     `parity_local_only_no_external_evaluator_admissible`,
     `parity_local_passes_ci_pending_no_judgement_yet`,
     `parity_local_passes_ci_passes_provider_overlay_unavailable_local_continues`,
     `parity_local_disagrees_with_ci_user_review_required`,
     `parity_local_passes_provider_overlay_disagrees_user_review_required`,
     `parity_provider_overlay_stale_local_continues`,
     `parity_evaluator_unavailable_user_review_required`,
     `parity_reused_from_export_pinned_to_build_identity`);
   - one `evaluation_staleness_class` from the closed five-value
     vocabulary
     (`evaluation_fresh_within_grace`,
     `evaluation_stale_within_grace_re_evaluation_recommended`,
     `evaluation_stale_beyond_grace_re_evaluation_required`,
     `evaluation_unverifiable_re_evaluation_required`,
     `evaluation_pinned_to_captured_build_identity_no_drift_assessable`);
   - one `reuse_provenance_class` from the closed seven-value
     vocabulary
     (`not_reused_evaluation_native`,
     `reused_from_support_export_pinned_to_build_identity`,
     `reused_from_browser_companion_pinned_to_build_identity`,
     `reused_from_provider_overlay_pinned_to_build_identity`,
     `reused_from_ai_review_overlay_pinned_to_build_identity`,
     `reuse_refused_build_identity_drifted`,
     `reuse_refused_evaluator_provenance_unverifiable`),
     paired through allOf gates so reuse rows MUST cite a non-null
     `reused_from_record_id_ref` and
     `reused_from_captured_build_identity_ref`;
   - the `divergence_label` (a member of the matched pack's
     `divergence_labels_required` set);
   - the `findings_ref_list` (non-empty when
     `evaluation_lifecycle_state` is `evaluation_completed` and the
     pack declared at least one check);
   - the `output_artifact_refs` set (a superset of
     `output_artifact_families_required` on the matched pack); and
   - the audit-event row shape on the `review_evaluation` audit
     stream
     (`review_evaluation_admitted`, `review_evaluation_started`,
     `review_evaluation_finding_recorded`,
     `review_evaluation_completed`,
     `review_evaluation_failed_with_error`,
     `review_evaluation_cancelled`,
     `review_evaluation_superseded_by_re_evaluation`,
     `review_evaluation_reused_from_source`,
     `review_evaluation_reuse_refused`,
     `review_evaluation_audit_denial_emitted`).

3. The `review_evaluation_finding_record` shape every per-check or
   per-ownership outcome publishes, including:
   - the `review_pack_check_id_ref` (required non-null; the matched
     check_class and enforcement_class are the source of truth);
   - the optional `ownership_rule_id_ref` plus
     `ownership_outcome_class` from the closed eight-value
     vocabulary
     (`ownership_review_satisfied`,
     `ownership_review_pending_user_action_required`,
     `ownership_signoff_satisfied`,
     `ownership_signoff_pending_user_action_required`,
     `ownership_advisory_notification_published`,
     `ownership_optional_notification_recorded`,
     `ownership_disabled_for_scope_no_outcome`,
     `ownership_evaluator_unavailable_user_review_required`),
     paired through allOf gates with the matched
     `ownership_rule_class`;
   - one `check_outcome_class` from the closed seven-value
     vocabulary
     (`pass_admissible`, `fail_blocking`,
     `fail_advisory_non_blocking`, `skipped_not_in_scope`,
     `skipped_waiver_admitted_active`,
     `skipped_evaluator_unavailable_user_review_required`,
     `error_evaluator_internal_user_review_required`);
   - one `finding_severity_class` from the closed five-value
     vocabulary
     (`severity_blocker`, `severity_high`, `severity_medium`,
     `severity_low`, `severity_informational`),
     paired through allOf gates with `check_outcome_class` so
     `severity_blocker` MUST pair with `fail_blocking`;
   - the optional `anchor_record_id_ref`,
     `waiver_record_id_ref` (required when
     `check_outcome_class` is `skipped_waiver_admitted_active`),
     and `evidence_packet_record_id_ref` slots.

4. The acceptance invariants this contract enforces:

   - **The same review-pack fixture can be evaluated locally and in
     CI without changing semantics.** The pack record carries the
     full policy declaration; the evaluation result carries one
     `evaluation_source_class` per evaluator. A local evaluation
     and a CI evaluation against the same pack produce two rows
     that share `review_pack_id_ref` /
     `target_review_workspace_id_ref` and differ only in
     `evaluation_source_class`, `evaluator_descriptor_ref`, and the
     produced findings. The matched parity auditor folds them
     into one `parity_state_class` chip.
   - **Ownership / advisory vs enforced outcomes are explicit in
     outputs.** The `ownership_rule_class` and
     `review_pack_check_enforcement_class` vocabularies make the
     advisory-vs-enforced split a typed field on every pack row,
     and the `ownership_outcome_class` and
     `check_outcome_class` vocabularies make the satisfied vs
     pending vs advisory outcome a typed field on every finding
     row. A finding that asserts a different enforcement class
     than the pack declared denies with
     `check_enforcement_class_must_match_pack_declaration`; an
     ownership outcome that does not pair with the rule's class
     denies with
     `ownership_outcome_must_match_ownership_rule_class`.
   - **Browser, provider, and AI overlays reuse the same
     normalized result model without creating "green by omission"
     states.** The seven-value `check_outcome_class` vocabulary
     forces every per-check outcome to be one of the typed states.
     A result whose `evaluation_lifecycle_state` is
     `evaluation_completed` and whose matched pack declared at
     least one check MUST publish a non-empty `findings_ref_list`;
     a completed result with no findings denies with
     `green_by_omission_forbidden_evaluation_must_be_explicit`.
   - **Reuse from a support export, browser companion, provider
     overlay, or AI review overlay is mechanically labelled.** The
     `reuse_provenance_class` vocabulary makes reuse a typed field
     and forces every reuse row to cite both
     `reused_from_record_id_ref` and
     `reused_from_captured_build_identity_ref`. Drift between the
     captured build identity and the current
     `running_build_identity_ref` forces
     `reuse_refused_build_identity_drifted` on subsequent reads,
     so the surface never paints a stale captured pass as a fresh
     pass.
   - **Local-vs-CI-vs-provider parity and staleness are
     mechanical.** The `parity_state_class` vocabulary covers the
     pre-CI baseline, in-flight, agreement, disagreement,
     unavailable-evaluator, and reuse states. The
     `evaluation_staleness_class` vocabulary covers fresh,
     stale-within-grace, stale-beyond-grace, unverifiable, and
     captured-build-pinned states. A surface that asserts
     provider-authoritative parity under the workspace's
     `provider_overlay_stale_beyond_grace_local_continues` posture
     denies with
     `parity_state_must_match_workspace_overlay_freshness`.
   - **Forward links are reserved.** Every record carries a
     `hosted_review_inbox_record_id_ref` slot (currently always
     `null` until the hosted-review inbox contract lands) and a
     `merge_policy_record_id_ref` slot (currently always `null`
     until the merge-policy contract lands). Surfaces never embed
     an inline inbox or policy row; they cite the future refs.

Out of scope until a superseding decision row opens:

- Implementing the review-pack runner. The contract reserves the
  data model the runner binds to.
- Implementing the CI / provider / AI / browser-companion
  evaluators. Each is a downstream lane that publishes through
  this contract.
- Implementing the hosted-review inbox surface or the merge-policy
  resolver. They are forward dependencies (slots reserved).
- Building the full review UI (per-finding chip, per-check
  drill-down, ownership-routing modal, waiver-proposal flow). The
  contract reserves the data model the UI binds to.
- Cross-repo review-pack federation, multi-host evaluator
  aggregation, or queue-shared waiver pools. Out of scope at this
  revision.

## 1. The review-pack record

Every reviewer-facing surface in Aureline (the local diff explorer,
the CI runner, the hosted-review reader, the merge-queue panel, the
AI review-aware overlays, the browser companion, the support-export
review section) MUST resolve the policy it is evaluating against to
exactly one `review_pack_record`. The record is the answer to five
questions a reviewer must be able to answer without opening any
other object:

1. *Where does the policy live?* — `review_pack_class` plus
   `scope_binding`.
2. *Which checks MUST run, and which are advisory?* — the `checks`
   list of `review_pack_check_descriptor` rows.
3. *Which paths require which ownership signoff?* — the
   `ownership_rules` list of `ownership_rule_descriptor` rows.
4. *Which divergence labels are admissible?* —
   `divergence_labels_required`.
5. *Which output-artifact families MUST every evaluation publish?*
   — `output_artifact_families_required`.

### 1.1 Pack-class vocabulary

`review_pack_class` is closed and exhaustive:

- `project_default_pack` — the repository's baseline pack. Every
  workspace inherits from exactly one project-default pack.
- `scope_overlay_pack` — overlays a sub-tree / sparse slice on top
  of the project default. Carries its own `scope_binding`. The
  pack-merge resolver applies the overlay's checks and ownership
  rules in addition to (or as an override of) the project default,
  per the published merge order.
- `owner_supplied_pack` — a pack contributed by an owning team for
  paths inside their ownership scope. MUST cite a non-empty
  `ownership_rules` list (a missing ownership-rules list denies).
- `ad_hoc_session_pack` — a per-session overlay used during
  exploratory review. MUST NOT be promoted past
  `review_pack_drafting` (or `review_pack_archived_tombstone` once
  the session ends). A promotion to `review_pack_active` denies
  with `ad_hoc_session_pack_must_not_promote_to_active`.
- `imported_from_bundle_pack` — was hydrated from an exported
  review bundle. MUST cite a non-null `imported_bundle_envelope`.
  No auto re-fetch against the canonical provider.
- `managed_admin_published_pack` — provisioned by a managed-admin
  surface. MUST be admitted only on `managed_admin_surface`
  client_scope; a pack that names other client_scopes denies with
  `managed_admin_published_pack_client_scope_restricted`.

### 1.2 Check declarations

`review_pack_check_class` is closed and exhaustive (twelve values).
The descriptor pairs the class with one
`review_pack_check_enforcement_class` (five values) and one
`waiver_linkage_class` (seven values) under allOf gates that
forbid the silent-bypass failure modes:

- `enforced_blocking` MUST pair with `no_waiver_admissible` so a
  hard-blocker check cannot be silently waived.
- `enforced_blocking_unless_waived` MUST pair with one of
  `waiver_required_admin_signed`,
  `waiver_required_owner_signed`, or
  `waiver_required_with_expiry` so the waiver path is
  mechanical.
- The advisory and informational classes are admissible with any
  waiver-linkage class except where the matched evaluation result
  flips the outcome (a finding that asserts `fail_blocking` on an
  advisory check denies with
  `check_enforcement_class_must_match_pack_declaration`).

### 1.3 Ownership rules

`ownership_rule_class` is closed and exhaustive (five values). The
descriptor pairs the class with an opaque `owner_route_ref` into
the owner-route registry and a sub-`scope_binding`. The advisory
vs enforced split is explicit on the class so the outcome is never
buried in tool-specific copy:

- `ownership_required_review_blocking` /
  `ownership_required_signoff_blocking` are the enforced classes;
  the matched finding MUST resolve to
  `ownership_review_satisfied` / `ownership_signoff_satisfied`
  before landing.
- `ownership_advisory_notification_non_blocking` /
  `ownership_optional_notification_no_chip` are the advisory
  classes; the matched finding records the notification without
  blocking.
- `ownership_disabled_for_scope` is the explicit disabled posture
  used when a scope-overlay pack disables the project default's
  ownership rule for a sub-scope.

### 1.4 Divergence labels and output artifact families

`divergence_labels_required` declares which divergence labels the
pack's consumers MUST be prepared to surface on the matched
result. The nine values cover agreement, in-flight, disagreement,
single-evaluator-only, evaluator-unavailable, and reuse-from-export
states. A result that asserts a label outside this set denies with
`divergence_label_must_match_evaluator_set`.

`output_artifact_families_required` declares which output-artifact
families an evaluation MUST publish. The seven values cover the
per-evaluation summary, the per-finding list, the ownership-review
request, the waiver proposal, the release evidence packet, the AI
review evidence packet, and the audit-event stream. A result that
omits a declared family denies with
`required_output_artifact_family_missing`.

## 2. The review-evaluation-result record

Every evaluator (local, CI, provider, AI, browser companion,
support / export reuse) MUST publish exactly one
`review_evaluation_result_record` per
(`review_pack_id_ref`, `target_review_workspace_id_ref`,
`evaluator_descriptor_ref`, `running_build_identity_ref`,
`evaluation_source_class`) tuple. The record is the answer to five
questions a reviewer must be able to answer without opening the
evaluator's tool-specific UI:

1. *Which pack and workspace was this evaluated against?* —
   `review_pack_id_ref` plus
   `target_review_workspace_id_ref`.
2. *Who produced this evaluation, and against which build?* —
   `evaluation_source_class` plus
   `evaluator_descriptor_ref` plus
   `running_build_identity_ref`.
3. *Is this a fresh evaluation or a reuse, and from which source?*
   — `reuse_provenance_class` plus the matched
   `reused_from_*` refs.
4. *Did all evaluators agree, and is the result fresh?* —
   `parity_state_class` plus
   `evaluation_staleness_class`.
5. *Which findings did the evaluation produce, and which
   output-artifact families did it publish?* —
   `findings_ref_list` plus `output_artifact_refs`.

### 2.1 Lifecycle and findings

`evaluation_lifecycle_state` is closed and ordered:

- `evaluation_pending_admission` — the result row was minted but
  the evaluator has not started.
- `evaluation_running` — the evaluator is in flight.
- `evaluation_completed` — the evaluator finished and published a
  full findings list.
- `evaluation_failed_with_error` — the evaluator errored out
  before producing a complete findings list.
- `evaluation_cancelled` — the evaluator was cancelled by user or
  policy.
- `evaluation_superseded_by_re_evaluation` — a later evaluation
  replaced this row.

A row whose state is `evaluation_completed` MUST cite a non-empty
`findings_ref_list` when the matched pack declared at least one
check (the schema enforces the local form by requiring the
findings field; the matched pack-vs-result auditor enforces the
cross-record cardinality and emits the
`green_by_omission_forbidden_evaluation_must_be_explicit` denial
when a check the pack declared is not represented by a finding).

### 2.2 Parity state

`parity_state_class` is closed and ten-valued. The reviewer reads
this chip to know whether local, CI, and provider overlay agree:

| `parity_state_class` | Meaning |
|---|---|
| `parity_match_all_evaluators_agree_pass` | Local + CI + provider all pass. |
| `parity_match_all_evaluators_agree_fail` | All three agree the pack failed. |
| `parity_local_only_no_external_evaluator_admissible` | Pre-CI baseline; only local evaluation has run. |
| `parity_local_passes_ci_pending_no_judgement_yet` | Local passes; CI still running; no provider overlay yet. |
| `parity_local_passes_ci_passes_provider_overlay_unavailable_local_continues` | Local + CI agree; provider overlay unreachable; local review continues. |
| `parity_local_disagrees_with_ci_user_review_required` | Local and CI disagree; user MUST resolve before mutation. |
| `parity_local_passes_provider_overlay_disagrees_user_review_required` | Local passes; provider overlay disagrees; user MUST resolve. |
| `parity_provider_overlay_stale_local_continues` | Workspace overlay is stale-beyond-grace; local continues. |
| `parity_evaluator_unavailable_user_review_required` | One evaluator is unreachable and the user MUST acknowledge. |
| `parity_reused_from_export_pinned_to_build_identity` | Result is a reuse from an export pinned to a captured build identity. |

A result that asserts `parity_match_all_evaluators_agree_*` or
`parity_local_passes_ci_passes_provider_overlay_unavailable_local_continues`
MUST be produced by an evaluator whose
`evaluation_source_class` is one of
`local_workstation_evaluation`, `ci_pipeline_evaluation`, or
`provider_overlay_evaluation` (the schema enforces the local form;
the matched pack-vs-result auditor enforces the partner-result
cross-record cardinality and emits
`divergence_label_must_match_evaluator_set` when the partners are
missing).

A result that asserts provider-authoritative parity under the
workspace's
`provider_overlay_stale_beyond_grace_local_continues` posture
denies with
`parity_state_must_match_workspace_overlay_freshness`.

### 2.3 Reuse provenance

`reuse_provenance_class` is closed and seven-valued. Reuse is a
typed field on every result, not a heuristic on the surface:

- `not_reused_evaluation_native` — the evaluator produced this
  result fresh.
- `reused_from_support_export_pinned_to_build_identity` /
  `reused_from_browser_companion_pinned_to_build_identity` /
  `reused_from_provider_overlay_pinned_to_build_identity` /
  `reused_from_ai_review_overlay_pinned_to_build_identity` — the
  result was reused from the named source. MUST cite a non-null
  `reused_from_record_id_ref` and
  `reused_from_captured_build_identity_ref`.
- `reuse_refused_build_identity_drifted` /
  `reuse_refused_evaluator_provenance_unverifiable` — explicit
  refuse states the surface MUST advertise rather than silently
  fall back to a freshly-minted "pass" cue.

A reuse row whose source class does not match the reuse-provenance
class (e.g. asserting
`reused_from_support_export_pinned_to_build_identity` while the
source class is `ci_pipeline_evaluation`) denies with
`reuse_must_pin_source_evaluator_provenance`.

### 2.4 Staleness

`evaluation_staleness_class` is closed and five-valued:

- `evaluation_fresh_within_grace` — admissible as current.
- `evaluation_stale_within_grace_re_evaluation_recommended` —
  surface a chip without blocking.
- `evaluation_stale_beyond_grace_re_evaluation_required` — force
  re-evaluation before mutation.
- `evaluation_unverifiable_re_evaluation_required` — outage
  state.
- `evaluation_pinned_to_captured_build_identity_no_drift_assessable`
  — the result was pinned to a captured build identity (a reuse-
  from-export row); staleness against the current build cannot be
  assessed without re-running.

## 3. The review-evaluation-finding record

Every per-check or per-ownership-rule outcome inside a result MUST
publish exactly one `review_evaluation_finding_record`. The row is
either a *check finding* (citing
`review_pack_check_id_ref`, with `ownership_rule_id_ref` null) or
an *ownership finding* (citing both refs on the same
ownership-bearing check).

The closed `check_outcome_class` vocabulary forces every per-check
outcome to one of the seven typed states; the closed
`ownership_outcome_class` vocabulary forces every per-ownership
outcome to one of the eight typed states. The closed
`finding_severity_class` vocabulary pairs through allOf gates with
`check_outcome_class` so `severity_blocker` MUST pair with
`fail_blocking` and `fail_blocking` MUST pair with
`severity_blocker`. A `skipped_waiver_admitted_active` outcome
MUST cite a non-null `waiver_record_id_ref`.

A finding whose `review_pack_check_id_ref` resolves to a check
whose enforcement_class is `enforced_blocking` and whose outcome
is `skipped_waiver_admitted_active` denies with
`enforced_blocking_check_cannot_be_silently_waived`. A finding
whose ownership_outcome_class does not pair with the matched
ownership_rule_class (e.g.
`ownership_signoff_satisfied` against
`ownership_required_review_blocking`) denies with
`ownership_outcome_must_match_ownership_rule_class`.

## 4. Forward dependencies

Two refs are reserved on every record but currently always `null`:

- `hosted_review_inbox_record_id_ref` — the hosted-review inbox
  contract is a forward dependency. Once it lands, every result
  evaluated against a hosted-review inbox MUST cite the inbox row.
- `merge_policy_record_id_ref` — the merge-policy contract is a
  forward dependency. Once it lands, every result whose pack
  composes with a merge policy MUST cite the merge-policy row.

The reserved slots survive the next contract's landing without a
breaking change: today they are nullable; later they become
required non-null when the upstream contract lands and bumps
`review_pack_schema_version` /
`review_evaluation_result_schema_version`.

## 5. Audit streams

Two audit streams are reserved by this contract:

- `review_pack_audit_event` — closed event-id vocabulary including
  `review_pack_drafted`, `review_pack_admitted_active`,
  `review_pack_check_added`, `review_pack_check_removed`,
  `review_pack_ownership_rule_added`,
  `review_pack_ownership_rule_removed`,
  `review_pack_waiver_linkage_changed`,
  `review_pack_deprecated_pending_replacement`,
  `review_pack_archived`,
  `review_pack_audit_denial_emitted`. Denial events MUST cite one
  denial reason from the `review_pack_denial_reason` vocabulary.
- `review_evaluation_audit_event` — closed event-id vocabulary
  including `review_evaluation_admitted`,
  `review_evaluation_started`,
  `review_evaluation_finding_recorded`,
  `review_evaluation_completed`,
  `review_evaluation_failed_with_error`,
  `review_evaluation_cancelled`,
  `review_evaluation_superseded_by_re_evaluation`,
  `review_evaluation_reused_from_source`,
  `review_evaluation_reuse_refused`,
  `review_evaluation_audit_denial_emitted`. Denial events MUST
  cite one denial reason from the
  `review_evaluation_denial_reason` vocabulary.

The denial-reason vocabularies are listed in the schemas. Adding a
new denial reason or a new audit-event id is additive-minor and
bumps the per-record schema-version const; repurposing an existing
value is breaking and requires a new decision row.

## 6. Redaction posture

Every record published against this contract carries one
`redaction_class` from the re-exported capability-lifecycle
vocabulary. Defaults:

- Local-only packs and their evaluation results default to
  `metadata_safe_default`.
- Packs that touch organisationally restricted ownership routes MAY
  raise to `internal_support_restricted`.
- Packs and results that wire into credentialed CI / provider rule
  snapshots MUST raise to `operator_only_restricted`.
- Findings quoted by support exports MUST raise to
  `internal_support_restricted` and the `summary_label` MUST
  resolve through the redaction-aware reviewable label registry.

Raw absolute paths, raw branch / commit URLs, raw author identity
strings, raw check command lines, raw tool output bodies, raw
waiver bodies, raw owner identity strings, raw notebook cell text,
raw terminal bytes, and raw URLs never appear on any record
published against this contract. Every payload travels by opaque
ref or through the redaction-aware label registry.

## 7. Acceptance cross-walk

| Acceptance bullet from the plan | Where it lands |
|---|---|
| The same review-pack fixture can be evaluated locally and in CI without changing semantics. | §1 pack record + §2 result record + the matched fixtures `pack_project_default_with_ownership.yaml`, `result_local_workstation_pass.yaml`, `result_ci_pipeline_pass_match.yaml`. |
| Ownership / advisory vs enforced outcomes are explicit in outputs rather than buried in tool-specific copy. | §1.2 `review_pack_check_enforcement_class` + §1.3 `ownership_rule_class` + §3 `check_outcome_class` / `ownership_outcome_class`. Fixtures `pack_owner_supplied_overlay.yaml`, `finding_ownership_signoff_pending.yaml`, and `finding_advisory_chip_only.yaml`. |
| Browser, provider, and AI overlays can reuse the same normalized result model without creating "green by omission" states. | §2.1 lifecycle + the `green_by_omission_forbidden_evaluation_must_be_explicit` denial. Fixtures `result_provider_overlay_disagrees.yaml`, `result_ai_review_overlay_advisory.yaml`, `result_browser_companion_reuse.yaml`, and `result_completed_omits_declared_check_denied.yaml`. |
| Parity and staleness rules for local vs CI vs provider status and for browser / companion or export reuse are mechanical. | §2.2 `parity_state_class` + §2.3 `reuse_provenance_class` + §2.4 `evaluation_staleness_class`. Fixtures `result_export_reuse_pinned_to_build_identity.yaml`, `result_reuse_refused_build_identity_drifted.yaml`, and `result_local_disagrees_with_ci.yaml`. |

## 8. Versioning

Each schema in this family carries a document-level
`*_schema_version` const. Adding a new enum value, a new optional
property, or a new additive sub-record is additive-minor and bumps
the relevant `*_schema_version` const. Repurposing an existing
value is breaking and requires a new decision row. The schemas
join the `vcs` family row in
[`artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml)
and each artifact joins
[`artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
in the same change.
