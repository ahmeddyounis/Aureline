# M5 Trust-Class Ladders, Downgrade Rules & Compare-Only Fallbacks

- Packet: `m5-trust-class-ladder:stable:0001`
- Case: `case:m5-trust-class-ladder:stable`
- Downgraded surfaces: 4 of 8
- Fallback kinds: blocked_metadata_only, compare_only, no_fallback, sanitized_visibility

## Surfaces

- **notebook_rich_output** (ordinary_browsing): requested `trusted_local_active` → effective `trusted_local_active` (fallback `no_fallback`)
  - Active content: trusted_local_execution · raw reachable: true
- **docs_browser_panel** (ordinary_browsing): requested `isolated_remote_active` → effective `isolated_remote_active` (fallback `no_fallback`)
  - Active content: isolated_remote_execution · raw reachable: true
- **ai_evidence_viewer** (ordinary_browsing): requested `sanitized_rich` → effective `compare_only` (fallback `compare_only`)
  - Active content: inert_never_executes · raw reachable: true
  - Triggers: raw_rendered_divergence_unresolved
- **pipeline_artifact_browser** (ordinary_browsing): requested `sanitized_rich` → effective `compare_only` (fallback `compare_only`)
  - Active content: inert_never_executes · raw reachable: true
  - Triggers: safe_preview_unavailable
- **provider_overlay** (strong_decision_strict_identity): requested `sanitized_rich` → effective `sanitized_rich` (fallback `no_fallback`)
  - Active content: inert_never_executes · raw reachable: true
- **marketplace_install_review** (strong_decision_strict_identity): requested `isolated_remote_active` → effective `sanitized_rich` (fallback `sanitized_visibility`)
  - Active content: inert_never_executes · raw reachable: true
  - Triggers: isolation_runtime_unavailable
- **remote_preview_target** (strong_decision_strict_identity): requested `isolated_remote_active` → effective `blocked` (fallback `blocked_metadata_only`)
  - Active content: blocked_pending_review · raw reachable: true
  - Triggers: policy_blocked
- **structured_compare_view** (ordinary_browsing): requested `sanitized_rich` → effective `sanitized_rich` (fallback `no_fallback`)
  - Active content: inert_never_executes · raw reachable: true

## Downgrade rules

- `policy_block_forces_blocked`: on `policy_blocked` → `blocked_metadata_only`
- `unresolved_divergence_forces_compare_only`: on `raw_rendered_divergence_unresolved` → `compare_only`
- `safe_preview_unavailable_forces_compare_only`: on `safe_preview_unavailable` → `compare_only`
- `isolation_unavailable_downgrades_active_to_sanitized`: on `isolation_runtime_unavailable` → `sanitized_visibility`
- `local_trust_absent_downgrades_active_to_sanitized`: on `local_trust_not_established` → `sanitized_visibility`
- `suspicious_content_downgrades_active_to_sanitized`: on `suspicious_content_detected` → `sanitized_visibility`
- `proof_stale_narrows_active_to_sanitized`: on `proof_stale` → `sanitized_visibility`
- `embedded_review_surface_never_executes`: on `embedded_review_surface` → `sanitized_visibility`
