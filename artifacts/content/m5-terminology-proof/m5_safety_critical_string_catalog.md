# Safety-Critical String Catalog and Controlled Terms

- Catalog: `m5-safety-critical-string-catalog:stable:0001`
- Label: `Stable Safety-Critical String Catalog and Controlled Terms`
- Reference locale: `en`
- Controlled terms: 17 | Messages: 21
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Controlled terms

- `term.unverified_source` (trust, token `unverified_source`): The source has not had its authority verified; trust must be established before it acts.
- `term.official_source` (trust, token `official_source`): A first-party, verifiable source that may act without a per-use trust prompt.
- `term.trust_required` (policy, token `trust_required`): The action cannot proceed until the operator grants trust to the source or target.
- `term.policy_blocked` (policy, token `policy_blocked`): The action is blocked by an active policy and cannot be run as requested.
- `term.restricted_scope` (policy, token `restricted`): The action is permitted only within a narrower, disclosed scope.
- `term.requires_review` (policy, token `requires_review`): The action must be explicitly reviewed and confirmed before it runs.
- `term.incompatible_target` (compatibility, token `incompatible`): The claim is incompatible with the active target and cannot be applied safely.
- `term.minor_skew` (compatibility, token `minor_skew_compatible`): The claim is compatible within an accepted minor-skew window, not exact.
- `term.proven_current` (freshness, token `proven_current`): The data is proven current for its declared scope and freshness basis.
- `term.cached` (freshness, token `cached`): The data is shown from a cache with a disclosed cache posture, not proven current.
- `term.stale` (freshness, token `stale`): Prior data shown after its freshness floor or causal continuity was lost.
- `term.warming` (freshness, token `warming`): The data is warming and not yet complete for the declared scope.
- `term.local_only` (client_scope, token `local_only`): A local-only posture with no managed recall, sync, or hosted evidence.
- `term.browser_companion` (client_scope, token `browser_companion`): A browser companion surface that does not imply full desktop parity.
- `term.preview` (lifecycle, token `preview`): A preview capability that is not yet broadly claimed as stable.
- `term.beta` (lifecycle, token `beta`): A beta capability under active hardening, narrower than stable.
- `term.disabled_by_policy` (lifecycle, token `disabled_by_policy`): The capability is disabled by an active policy on this deployment.

## Messages

- `msg.trust.unverified_source_prompt` [safety_critical_string / end_user / warning] on `trust_prompt`
  - Reference: Unverified source: “{source_name}” has not been verified. Trust required before this runs.
  - Terms: term.unverified_source, term.trust_required
- `msg.trust.grant_trust_action` [action_label / end_user / notice] on `trust_prompt`
  - Reference: Grant trust to {source_name}
- `msg.policy.action_blocked_banner` [error_recovery_block / end_user / blocking] on `degraded_state_banner`
  - Reference: Policy blocked: {what_failed}. Likely cause: {likely_cause}. Still available: {what_still_works}. Next: {next_safe_action}.
  - Terms: term.policy_blocked
- `msg.policy.restricted_scope_sheet` [safety_critical_string / operator / caution] on `execution_context_sheet`
  - Reference: Restricted to {scope_name}; Requires review to widen.
  - Terms: term.restricted_scope, term.requires_review
- `msg.runtime.degraded_local_only_banner` [safety_critical_string / end_user / warning] on `degraded_state_banner`
  - Reference: Local only. {feature_name} shows Stale or Warming data until reconnected.
  - Terms: term.local_only, term.stale, term.warming
- `msg.doctor.stale_index_finding` [error_recovery_block / developer / caution] on `project_doctor_finding`
  - Reference: Stale index: {what_failed}. Likely cause: {likely_cause}. Still available: {what_still_works}. Next: {next_safe_action}.
  - Terms: term.stale
- `msg.doctor.incompatible_target_finding` [error_recovery_block / developer / critical] on `project_doctor_finding`
  - Reference: Incompatible (not Minor version skew): {what_failed}. Likely cause: {likely_cause}. Still available: {what_still_works}. Next: {next_safe_action}.
  - Terms: term.incompatible_target, term.minor_skew
- `msg.ai.evidence_basis_line` [ai_copy_line / end_user / notice] on `ai_review_flow`
  - Reference: Based on {source_count} sources; freshness is Cached, not Current.
  - Terms: term.cached, term.proven_current
- `msg.ai.autonomy_disclosure_line` [ai_copy_line / end_user / notice] on `ai_review_flow`
  - Reference: Proposed {step_count} steps; each Requires review before it runs.
  - Terms: term.requires_review
- `msg.exec.trust_required_sheet_heading` [safety_critical_string / operator / caution] on `execution_context_sheet`
  - Reference: Trust required for {target_name}; only an Official source runs unprompted.
  - Terms: term.trust_required, term.official_source
- `msg.count.visible_scope_phrase` [count_scope_phrase / end_user / info] on `runtime_status`
  - Reference: Showing {visible_count} of {total_count}; {omitted_reason}. Count is Cached.
  - Terms: term.cached
- `msg.count.search_stale_phrase` [count_scope_phrase / end_user / info] on `runtime_status`
  - Reference: {match_count} matches (Stale).
  - Terms: term.stale
- `msg.support.trust_state_heading` [safety_critical_string / support / info] on `support_export_heading`
  - Reference: Trust state for {subject_name}: Trust required, Policy blocked, Unverified source.
  - Terms: term.trust_required, term.policy_blocked, term.unverified_source
- `msg.support.freshness_state_heading` [safety_critical_string / support / info] on `support_export_heading`
  - Reference: Freshness state for {subject_name}: Current, Cached, Stale.
  - Terms: term.proven_current, term.cached, term.stale
- `msg.recovery.reconnect_action` [action_label / end_user / notice] on `recovery_action_block`
  - Reference: Reconnect to {target_name}
- `msg.recovery.request_access_action` [action_label / end_user / notice] on `recovery_action_block`
  - Reference: Request access for {action_name}
- `msg.recovery.rebuild_index_action` [action_label / developer / notice] on `recovery_action_block`
  - Reference: Rebuild index
- `msg.runtime.disabled_by_policy_status` [safety_critical_string / operator / caution] on `runtime_status`
  - Reference: {capability_name} is Disabled by policy (Policy blocked).
  - Terms: term.disabled_by_policy, term.policy_blocked
- `msg.runtime.lifecycle_status` [safety_critical_string / operator / notice] on `runtime_status`
  - Reference: {capability_name} lifecycle: Preview or Beta.
  - Terms: term.preview, term.beta
- `msg.exec.client_scope_sheet` [safety_critical_string / end_user / info] on `execution_context_sheet`
  - Reference: {surface_name} scope: Local only or Browser companion.
  - Terms: term.local_only, term.browser_companion
- `msg.a11y.degraded_announcement` [safety_critical_string / screen_reader / warning] on `degraded_state_banner`
  - Reference: Now Local only. {feature_name} data is Stale.
  - Terms: term.local_only, term.stale

## Cross-surface term reuse

- `term.beta`: runtime_status
- `term.browser_companion`: execution_context_sheet
- `term.cached`: ai_review_flow, runtime_status, support_export_heading
- `term.disabled_by_policy`: runtime_status
- `term.incompatible_target`: project_doctor_finding
- `term.local_only`: degraded_state_banner, execution_context_sheet
- `term.minor_skew`: project_doctor_finding
- `term.official_source`: execution_context_sheet
- `term.policy_blocked`: degraded_state_banner, runtime_status, support_export_heading
- `term.preview`: runtime_status
- `term.proven_current`: ai_review_flow, support_export_heading
- `term.requires_review`: ai_review_flow, execution_context_sheet
- `term.restricted_scope`: execution_context_sheet
- `term.stale`: degraded_state_banner, project_doctor_finding, runtime_status, support_export_heading
- `term.trust_required`: execution_context_sheet, support_export_heading, trust_prompt
- `term.unverified_source`: support_export_heading, trust_prompt
- `term.warming`: degraded_state_banner
