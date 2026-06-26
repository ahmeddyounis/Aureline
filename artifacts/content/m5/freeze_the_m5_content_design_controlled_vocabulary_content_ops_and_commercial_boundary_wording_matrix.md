# M5 Content-Design, Controlled-Vocabulary, Content-Ops, and Commercial-Boundary Wording Matrix

- Packet: `m5-content-wording-matrix:stable:0001`
- Label: `M5 Content-Design, Controlled-Vocabulary, Content-Ops, and Commercial-Boundary Wording Matrix`
- Objects: 8 (6 stable)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Objects

- **safety_critical_ui_string**: `stable`
  - Owner: Product copy owner
  - Scope: Safety-critical UI string for trust, policy, destructive, recovery, and degraded-state surfaces; carries a stable message id and controlled terms so the same reserved meaning survives UI, CLI, docs, exports, accessibility strings, and support packets
  - Vocabularies: lifecycle_state, trust_class, policy_state, freshness_state
  - Rollback: message_id_stable_preserved
- **glossary_term**: `stable`
  - Owner: Design systems owner
  - Scope: Controlled glossary / state-label term with one reserved meaning, alias posture, and allowed surfaces; the same visible word keeps the same meaning everywhere and is never repurposed without a controlled alias
  - Vocabularies: lifecycle_state, trust_class, client_scope
  - Rollback: term_labeled_never_softened
- **action_label_pattern**: `stable`
  - Owner: Product copy owner
  - Scope: Verb-first, outcome-specific action-label pattern; consequential actions never ship a standalone vague label and a narrowed client scope is disclosed, never implied as full desktop parity
  - Vocabularies: policy_state, client_scope
  - Rollback: scope_count_stays_honest
- **error_recovery_block**: `stable`
  - Owner: Supportability owner
  - Scope: Four-part error / recovery block that names what failed, why it likely failed, what still works, and the next safe action; degraded states always disclose remaining capability instead of a generic failure
  - Vocabularies: policy_state, freshness_state
  - Rollback: term_labeled_never_softened
- **ai_copy_guardrail**: `beta`
  - Owner: AI product owner
  - Scope: AI copy guardrail governing certainty, evidence, context, and autonomy language; AI wording never overstates confidence or autonomy, never claims false validation or freshness, and never obscures the route or spend truth
  - Vocabularies: trust_class, policy_state, freshness_state
  - Rollback: overclaim_blocked_before_ship
- **count_scope_phrase_set**: `stable`
  - Owner: Design systems owner
  - Scope: Count / scope / freshness phrase set that keeps visible, loaded, selected, and all-matching counts scope-honest, names omission reasons, and never lets a cached or stale count imply proven-current authority
  - Vocabularies: freshness_state, compatibility_state
  - Rollback: scope_count_stays_honest
- **content_ops_artifact**: `stable`
  - Owner: Docs owner
  - Scope: Content-ops metadata artifact that pins version and source metadata on docs, help, exports, and screenshots/demos so a captured artifact discloses its build, source, and compatibility basis instead of implying current authority
  - Vocabularies: compatibility_state, freshness_state
  - Rollback: boundary_wording_matches_product
- **commercial_boundary_wording**: `beta`
  - Owner: Commercial boundary owner
  - Scope: Commercial-boundary wording review for hosted / open / self-hosted / managed language; hosting boundary, edition label, and client scope cannot drift from the actual deployment profile, and open or local-independent language is never used when managed services participated
  - Vocabularies: hosting_boundary, edition_label, client_scope
  - Rollback: boundary_wording_matches_product
