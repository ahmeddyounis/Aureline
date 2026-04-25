# Review-workspace, comment-anchor, and merge-queue worked fixtures

These YAML fixtures exercise the review-workspace, comment-anchor, and
merge-queue contract frozen in
[`/docs/vcs/review_workspace_contract.md`](../../../docs/vcs/review_workspace_contract.md)
and the boundary schemas at
[`/schemas/vcs/review_workspace.schema.json`](../../../schemas/vcs/review_workspace.schema.json)
and
[`/schemas/vcs/review_anchor.schema.json`](../../../schemas/vcs/review_anchor.schema.json).

Every fixture is one record validated against `oneOf` in the
appropriate schema. Each carries only opaque workspace / branch /
revision / provider-overlay / approval-ticket / connected-provider /
browser-handoff packet / actor / command / approval-ticket / policy-
epoch / build-identity / review-anchor / line-range handles plus
monotonic placeholder timestamps and redaction-aware labels — no raw
absolute paths, no raw branch / commit URLs, no raw author identity
strings, no raw comment bodies, no raw provider rule bodies, no raw
approval-ticket bodies, no raw notebook cell text, no raw terminal
bytes, and no raw URLs.

## Workspace-source fixtures (one per `review_workspace_source_class`)

| Fixture | Source class | Provider authority | Lifecycle | Acceptance bullet |
|---|---|---|---|---|
| `local_branch_no_provider_overlay.yaml` | `local_branch_or_worktree` | `local_truth_only_no_provider_overlay` | `open_under_review` | Local-only review baseline. |
| `composite_local_with_provider_overlay_fresh.yaml` | `composite_local_with_provider_overlay` | `provider_authoritative` | `open_under_review` | Provider overlay fresh; provider-authoritative cues admissible. |
| `provider_overlay_stale_local_continues.yaml` | `composite_local_with_provider_overlay` | `local_parity_estimate` | `provider_overlay_degraded_local_continues` | Acceptance bullet 1 + bullet 4 — provider overlay stale beyond grace, local review continues, cached cues marked degraded. |
| `provider_overlay_unavailable_local_continues.yaml` | `composite_local_with_provider_overlay` | `local_truth_only_no_provider_overlay` | `provider_overlay_degraded_local_continues` | Acceptance bullet 1 + bullet 4 — provider unreachable, local review continues, no cues mislabelled as authoritative. |
| `review_bundle_imported_offline.yaml` | `review_bundle_imported` | `imported_bundle_snapshot` | `open_under_review` | Imported review bundle; no auto re-fetch. |
| `browser_handoff_token_source.yaml` | `browser_handoff_token_source` | `browser_handoff_token_only` | `open_under_review` | Browser-handoff packet round-trip; no raw URLs. |

## Anchor fixtures

| Fixture | Anchor target / drift / freshness | Acceptance bullet |
|---|---|---|
| `anchor_durable_across_refactor_remapped.yaml` | `text_line_range_anchor` / `anchor_remapped_with_recorded_mapping` (non-empty `remap_chain_target_id_refs`) / `local_and_provider_match_fresh`. | Acceptance bullet 2 — durable anchor across a refactor with the recorded mapping; no silent retargeting. |
| `anchor_drifted_user_must_resolve.yaml` | `text_line_range_anchor` / `anchor_drifted_user_must_resolve` paired with `user_must_pick_successor_or_dismiss_drifted`. | Acceptance bullet 2 — drift surfaces explicit user action; surface refuses an implicit jump. |
| `anchor_drifted_silent_retarget_denied.yaml` | `review_anchor_audit_event_record` carrying `silent_anchor_relocation_forbidden` denial. | Acceptance bullet 2 — silent retarget after a refactor denies. |
| `anchor_local_vs_provider_disagree_user_review_required.yaml` | `text_line_range_anchor` bound exact / `local_and_provider_disagree_user_review_required`. | Honest cue when local view disagrees with provider overlay; user must resolve before mutating. |

## Merge-queue fixtures

| Fixture | Action class / blocked reason / snapshot freshness | Acceptance bullet |
|---|---|---|
| `merge_queue_action_attempt_landing_under_fresh_provider_rules.yaml` | `attempt_landing` / `not_blocked_action_admissible` / `provider_rule_snapshot_fresh` with non-null `actor_ref` + `command_id_ref` + `approval_ticket_ref`. | Acceptance bullet 3 — landing action attributable, admissible only under fresh provider rules. |
| `merge_queue_action_blocked_freshness_missing.yaml` | `mark_blocked_pending_freshness` / `provider_rule_snapshot_stale` / `provider_rule_snapshot_unverifiable_blocked`. | Acceptance bullet 3 — landing action never appears available when provider rule freshness is unverifiable. |
| `merge_queue_action_attribution_missing_denial.yaml` | `merge_queue_action_audit_event_record` carrying `merge_queue_action_attribution_missing` denial. | Acceptance bullet 3 — mutation-class action without approval-ticket attribution denies. |

## Cross-walk to the spec

- The four-fixture overlay set
  (`composite_local_with_provider_overlay_fresh`,
  `provider_overlay_stale_local_continues`,
  `provider_overlay_unavailable_local_continues`,
  `review_bundle_imported_offline`) covers every
  `provider_overlay_freshness_class` value reachable through a
  composite or imported workspace plus the explicit outage path
  required by the plan's fourth acceptance bullet.
- The remap, drift, denial, and disagreement anchor set covers the
  durable-anchor, refuse-silent-retarget, and honest-freshness rules
  named in the plan's second acceptance bullet.
- The fresh / blocked / denied merge-queue set covers attribution and
  the not-available-without-fresh-provider-rules rule from the plan's
  third acceptance bullet.
- Forward dependency slots (`hosted_review_inbox_record_id_ref` and
  `merge_policy_record_id_ref`) are set to `null` on every fixture;
  they will become non-null when the hosted-review inbox and merge-
  policy contracts land.
