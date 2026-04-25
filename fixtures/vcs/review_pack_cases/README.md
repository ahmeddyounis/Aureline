# Review-pack, ownership-policy, and evaluation-result worked fixtures

These YAML fixtures exercise the review-pack and review-evaluation
contract frozen in
[`/docs/vcs/review_pack_contract.md`](../../../docs/vcs/review_pack_contract.md)
and the boundary schemas at
[`/schemas/vcs/review_pack.schema.json`](../../../schemas/vcs/review_pack.schema.json)
and
[`/schemas/vcs/review_evaluation_result.schema.json`](../../../schemas/vcs/review_evaluation_result.schema.json).

Every fixture is one record validated against `oneOf` in the
appropriate schema. Each carries only opaque pack / workspace /
build-identity / evaluator-descriptor / check / ownership-rule /
finding / waiver / approval-ticket / policy-epoch / actor /
command / browser-handoff packet / support-export bundle handles
plus monotonic placeholder timestamps and redaction-aware labels —
no raw absolute paths, no raw branch / commit URLs, no raw author
identity strings, no raw check command lines, no raw tool output
bodies, no raw waiver bodies, no raw owner identity strings, no raw
notebook cell text, no raw terminal bytes, and no raw URLs.

## Pack fixtures (one per `review_pack_class` reachable here)

| Fixture | Pack class | Lifecycle | Acceptance bullet |
|---|---|---|---|
| `pack_project_default_with_ownership.yaml` | `project_default_pack` | `review_pack_active` | Acceptance bullet 1 — same pack is evaluated by local + CI fixtures below; pack carries enforced format / lint / type / unit-test checks plus an enforced default-owner review rule and an advisory AI review check. |
| `pack_owner_supplied_overlay.yaml` | `owner_supplied_pack` | `review_pack_active` | Acceptance bullet 2 — owner-supplied overlay on a security crate subtree carries `ownership_required_signoff_blocking` and `ownership_advisory_notification_non_blocking` rules so the advisory-vs-enforced split is mechanical. |
| `pack_ad_hoc_session_drafting.yaml` | `ad_hoc_session_pack` | `review_pack_drafting` | Per-session overlay used for exploratory review; cannot be promoted past drafting. |
| `pack_imported_from_bundle.yaml` | `imported_from_bundle_pack` | `review_pack_active` | Imported review pack hydrated from an exported bundle; no auto re-fetch. |

## Result fixtures (one per `evaluation_source_class` plus parity / reuse / refuse cases)

| Fixture | Source class | Parity / reuse / divergence | Acceptance bullet |
|---|---|---|---|
| `result_local_workstation_pass.yaml` | `local_workstation_evaluation` | `parity_match_all_evaluators_agree_pass` / `not_reused_evaluation_native` / `local_ci_provider_all_match_pass` | Acceptance bullet 1 — local result evaluated against `pack_project_default_with_ownership.yaml`. |
| `result_ci_pipeline_pass_match.yaml` | `ci_pipeline_evaluation` | Same parity / reuse / divergence as the local result | Acceptance bullet 1 — CI evaluates the same pack and same `running_build_identity_ref` as the local result; semantics are unchanged. |
| `result_local_disagrees_with_ci.yaml` | `ci_pipeline_evaluation` | `parity_local_disagrees_with_ci_user_review_required` / `local_passes_ci_failed_user_review_required` | Acceptance bullet 4 — local-vs-CI disagreement surfaces the explicit user-review-required parity state. |
| `result_provider_overlay_disagrees.yaml` | `provider_overlay_evaluation` | `parity_local_passes_provider_overlay_disagrees_user_review_required` / `local_passes_provider_overlay_disagrees_user_review_required` | Acceptance bullet 4 — provider overlay disagreement surfaces the explicit user-review-required parity state. |
| `result_ai_review_overlay_advisory.yaml` | `ai_review_overlay_evaluation` | `parity_local_only_no_external_evaluator_admissible` / `local_only_no_external_evaluator_admissible` | Acceptance bullet 3 — AI review overlay reuses the same normalized result model and surfaces an advisory-only finding, never asserts blocking on enforced checks. |
| `result_browser_companion_reuse.yaml` | `browser_companion_evaluation` | `parity_reused_from_export_pinned_to_build_identity` / `reused_from_browser_companion_pinned_to_build_identity` / `evaluation_reused_from_export_pinned_to_build_identity` | Acceptance bullet 3 — browser-companion result is mechanically labelled as a reuse pinned to a captured build identity, never silently rendered as a fresh pass. |
| `result_export_reuse_pinned_to_build_identity.yaml` | `export_reuse_evaluation` | `parity_reused_from_export_pinned_to_build_identity` / `reused_from_support_export_pinned_to_build_identity` | Acceptance bullet 4 — support-export reuse pinned to its captured build identity. |
| `result_reuse_refused_build_identity_drifted.yaml` | `export_reuse_evaluation` | `parity_evaluator_unavailable_user_review_required` / `reuse_refused_build_identity_drifted` | Acceptance bullet 4 — captured build identity drifted; reuse refused; the surface advertises the refuse posture rather than re-rendering the captured pass. |

## Finding fixtures

| Fixture | Outcome / severity / enforcement / waiver | Acceptance bullet |
|---|---|---|
| `finding_ownership_signoff_pending.yaml` | `ownership_signoff_pending_user_action_required` paired with the security-team `ownership_required_signoff_blocking` rule | Acceptance bullet 2 — ownership outcomes are explicit per-finding rows. |
| `finding_advisory_chip_only.yaml` | `fail_advisory_non_blocking` / `severity_low` against an `advisory_non_blocking_with_visible_chip` check | Acceptance bullet 2 — advisory checks surface a chip without flipping into a blocker. |
| `finding_waiver_admitted_active.yaml` | `skipped_waiver_admitted_active` citing a non-null `waiver_record_id_ref` | Waiver linkage is mechanical. |

## Audit / denial fixtures

| Fixture | Audit-event id / denial reason | Acceptance bullet |
|---|---|---|
| `audit_green_by_omission_denied.yaml` | `review_evaluation_audit_denial_emitted` / `green_by_omission_forbidden_evaluation_must_be_explicit` | Acceptance bullet 3 — completed result that omits a declared check denies. |
| `audit_check_enforcement_class_mismatch_denied.yaml` | `review_evaluation_audit_denial_emitted` / `check_enforcement_class_must_match_pack_declaration` | Acceptance bullet 2 — finding that flips an advisory check into a blocker denies. |
| `audit_ownership_outcome_mismatch_denied.yaml` | `review_evaluation_audit_denial_emitted` / `ownership_outcome_must_match_ownership_rule_class` | Acceptance bullet 2 — ownership outcome that does not pair with the ownership-rule class denies. |

## Cross-walk to the spec

- The pack fixtures (`pack_project_default_with_ownership`,
  `pack_owner_supplied_overlay`, `pack_ad_hoc_session_drafting`,
  `pack_imported_from_bundle`) cover the four pack-class values
  reachable through the project's authoring path; the
  `managed_admin_published_pack` value is reserved for the
  managed-admin-surface lane and is exercised through allOf gates
  in the schema rather than a fixture row.
- The result fixtures cover one row per `evaluation_source_class`
  value plus the explicit reuse-pinned-to-captured-build-identity
  and reuse-refused-build-identity-drifted cases the parity /
  staleness rules require.
- The finding fixtures cover the ownership outcome, the advisory
  outcome, and the waiver-admitted outcome so a downstream surface
  can render each per-finding row mechanically.
- The audit / denial fixtures cover the headline rules: no
  green-by-omission, no enforcement-class flip, no
  ownership-outcome relabelling.
- Forward dependency slots
  (`hosted_review_inbox_record_id_ref`,
  `merge_policy_record_id_ref`) are set to `null` on every fixture;
  they will become non-null when the hosted-review inbox and merge-
  policy contracts land.
