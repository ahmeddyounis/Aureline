# M5 State-Distinction Explanation Helper Primitive

- Packet: `m5-state-distinction-explanation-helper-primitive:stable:0001`
- Label: `M5 state-distinction explanation helper primitive: consumer surface, confusable distinction (current-vs-selected / read-only-vs-disabled / locked-vs-disabled / pending-vs-loading), frozen precedence rule, primary and contrasted states, delivery form (inline chip / expanded drawer / blocked-limited copy), required non-color cues, required disclosures (state cause / owner / block reason / recovery action), recovery-disclosure class, and the stay-distinct, no-one-off-language, taxonomy-alignment, and blocked-action-alignment guarantees`
- Surfaces: 5 (5 stable)
- Distinctions: current_vs_selected, read_only_vs_disabled, locked_vs_disabled, pending_vs_loading
- Deliveries: inline_chip, expanded_drawer, blocked_limited_copy
- Non-color cues: primary_state_label, contrasted_state_label, distinction_marker, blocked_limited_glyph, recovery_affordance, taxonomy_reference_cue
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Surfaces

- **Onboarding / Help**: `stable`
  - Owner: Learnability owner
  - Scope: The onboarding / help surface teaches the confusable distinctions in place, using the same taxonomy words the components expose: an expanded drawer explains current-vs-selected and links back to the canonical taxonomy, and an inline chip names a pending action so a first-time user never mistakes it for generic background loading
  - Worked explanations: 2
    - `explain:onboarding.current-vs-selected` (`current_vs_selected` via `expanded_drawer`) → `current` vs `selected` (non-color cues 4, blocked-action help `false`, recovery `true`)
    - `explain:onboarding.pending-vs-loading` (`pending_vs_loading` via `inline_chip`) → `pending` vs `loading` (non-color cues 2, blocked-action help `false`, recovery `true`)
- **Blocked-Action Row**: `stable`
  - Owner: Blocked-action help owner
  - Scope: The blocked-action explanation row uses blocked/limited copy objects that stay aligned with the component-state truth: a locked-vs-disabled explanation names the policy owner, the block reason, and the recovery path so a lock never hides behind a bare disabled control, and a read-only-vs-disabled explanation preserves inspectability and states honestly when no recovery is available
  - Worked explanations: 2
    - `explain:blocked-action.locked-vs-disabled` (`locked_vs_disabled` via `blocked_limited_copy`) → `locked` vs `disabled` (non-color cues 3, blocked-action help `true`, recovery `true`)
    - `explain:blocked-action.read-only-vs-disabled` (`read_only_vs_disabled` via `blocked_limited_copy`) → `read_only` vs `disabled` (non-color cues 3, blocked-action help `true`, recovery `false`)
- **Settings Row**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings row explains its confusable states in place: an inline chip names a read-only effective value so it never reads as a plain disabled control, and an expanded drawer explains a locked setting against a disabled one — naming the owner, the reason, and the recovery path — so a policy lock stays explainable rather than collapsing into a silent disabled row
  - Worked explanations: 2
    - `explain:settings.read-only-vs-disabled` (`read_only_vs_disabled` via `inline_chip`) → `read_only` vs `disabled` (non-color cues 2, blocked-action help `false`, recovery `true`)
    - `explain:settings.locked-vs-disabled` (`locked_vs_disabled` via `expanded_drawer`) → `locked` vs `disabled` (non-color cues 4, blocked-action help `false`, recovery `true`)
- **Activity Row**: `stable`
  - Owner: Activity center owner
  - Scope: The activity row keeps pending distinct from loading: a blocked/limited copy object attributes a pending submission to the exact user action in flight, with its consequence and retry path, and an expanded drawer teaches pending-vs-loading so a submitted action in the activity center never masquerades as generic background work
  - Worked explanations: 2
    - `explain:activity.pending-vs-loading-copy` (`pending_vs_loading` via `blocked_limited_copy`) → `pending` vs `loading` (non-color cues 3, blocked-action help `true`, recovery `true`)
    - `explain:activity.pending-vs-loading-drawer` (`pending_vs_loading` via `expanded_drawer`) → `pending` vs `loading` (non-color cues 4, blocked-action help `false`, recovery `true`)
- **Workspace Entry**: `stable`
  - Owner: Workspace entry owner
  - Scope: The workspace-entry surface keeps current distinct from selected: an inline chip names the current workspace / live context owner so it never collapses into a merely selected entry, and an expanded drawer teaches current-vs-selected across the entry list with a fallback-scope recovery path when the live context is unavailable
  - Worked explanations: 2
    - `explain:workspace-entry.current-vs-selected-chip` (`current_vs_selected` via `inline_chip`) → `current` vs `selected` (non-color cues 2, blocked-action help `false`, recovery `true`)
    - `explain:workspace-entry.current-vs-selected-drawer` (`current_vs_selected` via `expanded_drawer`) → `current` vs `selected` (non-color cues 4, blocked-action help `false`, recovery `true`)
