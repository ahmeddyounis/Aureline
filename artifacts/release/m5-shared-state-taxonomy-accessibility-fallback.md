# M5 Shared-State Taxonomy Component Accessibility & Auto-Narrowing

- Packet: `m5-shared-state-taxonomy-accessibility-fallback:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Families: 4 certified across 4 / 4 frozen families
- Status: 2 green / 4 yellow / 0 red

## Rows

- **a11y:interactive-state-live** (interactive_state) — family=interactive_state keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_state_truth effective_claim=exact_state_truth status=parity
- **a11y:shared-state-taxonomy-reviewable** (shared_component_state_taxonomy) — family=shared_component_state_taxonomy keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_state_guidance effective_claim=reviewable_state_guidance status=parity
- **a11y:shared-state-taxonomy-cause-unresolved** (shared_component_state_taxonomy) — family=shared_component_state_taxonomy keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_state_truth effective_claim=cause_narrowed_projection status=narrowed_disclosed
  - Auto-narrow: exact_state_truth → cause_narrowed_projection (dimension=state_semantics, trigger=state_cause_unstated) — The reason this state applies could not be resolved from a live signal — shown as a cause-narrowed projection that still names the component identity, its typed state, and the keyboard route, never as a fully-explained live state
- **a11y:selection-or-lock-state-owner-unresolved** (selection_or_lock_state) — family=selection_or_lock_state keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_state_truth effective_claim=owner_narrowed_projection status=narrowed_disclosed
  - Auto-narrow: exact_state_truth → owner_narrowed_projection (dimension=selection_or_lock_state, trigger=lock_owner_masked) — The policy or ownership behind this lock could not be resolved — shown as an owner-narrowed projection that keeps the item identity, its selection and lock state, and the inspect route visible, never as a plain silent disabled control
- **a11y:degraded-state-application-recovery-unavailable** (degraded_state_application) — family=degraded_state_application keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=exact_state_truth effective_claim=recovery_narrowed_projection status=narrowed_disclosed
  - Auto-narrow: exact_state_truth → recovery_narrowed_projection (dimension=recovery_readiness, trigger=consequence_or_recovery_omitted) — The recovery path out of this degraded state could not be preserved — shown as a recovery-narrowed projection that still names what still works and the state consequence, never as a healthy live state
- **a11y:interactive-state-proof-stale** (interactive_state) — family=interactive_state keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=disclosed_partial_capture full_claim=exact_state_truth effective_claim=stale_proof_projection status=narrowed_disclosed
  - Auto-narrow: exact_state_truth → stale_proof_projection (dimension=interaction_state, trigger=proof_stale) — The accessibility and export proof for this interactive state has gone out of date — shown as a stale-proof projection with its identity, typed state, and keyboard route preserved, never as freshly-verified parity
