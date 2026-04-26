# Reusable component review checklist

Checklist id: `checklist.component_review.reusable`

This checklist is the reusable review gate for component contracts. It
is intentionally shared by design, engineering, QA, docs, telemetry, and
extension-parity reviewers so components are reviewed against the same
state vocabulary and evidence hooks.

Canonical refs:

- State taxonomy:
  [`docs/design/component_state_taxonomy.md`](../../docs/design/component_state_taxonomy.md)
- State-machine schema:
  [`schemas/design/component_state_machine.schema.json`](../../schemas/design/component_state_machine.schema.json)
- Component packet schema:
  [`schemas/design/component_contract.schema.json`](../../schemas/design/component_contract.schema.json)
- Component packet template:
  [`docs/ux/component_contract_template.md`](../../docs/ux/component_contract_template.md)
- Review gate manifest:
  [`artifacts/ux/review_gate_manifest.yaml`](../ux/review_gate_manifest.yaml)
- Design release evidence pack template:
  [`docs/ux/design_release_evidence_pack_template.md`](../../docs/ux/design_release_evidence_pack_template.md)

## Gate Rule

A reusable component packet is not accepted for launch-critical or
extension-parity-sensitive use until:

- every required review gate below is answered `pass`,
  `not_applicable`, or `waived`;
- the component packet includes `review_gate_refs` with the component
  checklist id, the review gate manifest ref, a stable design evidence
  pack id, and waiver state;
- every `waived` answer cites an active waiver with owner, scope,
  mitigation, expiry, user-visible impact, and exit signal; and
- keyboard, assistive-technology, screenshot, token-drift, extension,
  and state-machine evidence hooks use stable refs rather than
  free-form notes.

`fail`, `unanswered`, stale evidence, or an expired waiver blocks
component acceptance.

## Required Review Gates

| Gate id | Gate | Pass condition | Evidence hook |
| --- | --- | --- | --- |
| `component_review.semantic_tokens` | Semantic tokens | Uses semantic token refs only; no raw color, local CSS alias, or theme shadow registry crosses the contract boundary. | `token_drift_check_refs` |
| `component_review.keyboard_reachability` | Keyboard reachability | Every state, primary action, recovery route, source explanation, and inline detail is keyboard reachable. | `keyboard_journey_refs` |
| `component_review.focus_visibility` | Focus visibility | Focus-visible state is distinct from hover, selected, and current states in all supported themes. | `screenshot_baseline_refs`, `keyboard_journey_refs` |
| `component_review.screen_reader_semantics` | Screen-reader semantics | Role, accessible name, description, disabled/locked/read-only semantics, and state announcement are defined. | `assistive_technology_refs` |
| `component_review.dynamic_announcements` | Dynamic announcements | Loading, pending, locked, degraded, warning/error, and recovery transitions announce through the right channel without progress spam. | `assistive_technology_refs`, `state_machine_validation_refs` |
| `component_review.error_loading_states` | Error and loading states | Loading is distinct from pending, and warning/error is distinct from degraded. Empty, blocked, and failed states have copy and recovery behavior. | `state_machine_validation_refs` |
| `component_review.reduced_motion` | Reduced motion | State transitions remain understandable when motion is reduced, low-motion, power-saver, or hot-path suppression is active. | `screenshot_baseline_refs`, `token_drift_check_refs` |
| `component_review.high_contrast` | High contrast | State meaning survives dark, light, high-contrast, and forced-colors modes without hue-only reliance. | `screenshot_baseline_refs`, `token_drift_check_refs` |
| `component_review.density_variants` | Density variants | Compact, standard, and supported comfortable layouts keep focus order, text fit, and state cues stable. | `screenshot_baseline_refs`, `keyboard_journey_refs` |
| `component_review.performance_responsiveness` | Performance and responsiveness | State changes preserve stable dimensions, avoid layout shift, protect hot-path interaction feedback, and do not hide background work behind component-local animation. | `screenshot_baseline_refs`, `state_machine_validation_refs` |
| `component_review.copy_review` | Copy review | State copy names what changed, what still works, source/lock reason where relevant, and the next safe action. | `docs_link_verification_refs` |
| `component_review.deprecated_fallback_states` | Deprecated or fallback states | Deprecated tokens, unsupported packages, fallback icon slots, or substitute component variants stay visible where compatibility matters. | `docs_link_verification_refs`, `extension_parity_fixture_refs` |
| `component_review.source_lock_explanations` | Source and lock explanations | Locked states show policy/trust/permission/source owner and an inspect route. Disabled and read-only states are not used as substitutes for locked. | `docs_link_verification_refs`, `assistive_technology_refs` |
| `component_review.extension_parity` | Extension parity | Extension, embedded, companion, and handoff surfaces either match the packet state machine or disclose and hand off. | `extension_parity_fixture_refs` |

## State Distinction Checklist

Reviewers must reject a component packet when any row below is
collapsed.

| Distinction | Must verify |
| --- | --- |
| Locked vs disabled | `locked` has a source/reason and inspect route. `disabled` is ordinary context unavailability and does not carry a policy, trust, permission, or ownership source. |
| Read-only vs disabled | `read_only` preserves inspection, copy, navigation, or export. `disabled` does not imply useful content remains actionable. |
| Pending vs loading | `pending` follows a submitted or staged user action. `loading` covers initial fetch, warm-up, or context preparation. They use distinct labels and announcements. |
| Selected vs current | `selected` is chosen-set membership. `current` is route, detail, or live-context ownership. Collections, tabs, breadcrumbs, and result rows preserve both. |
| Degraded vs warning/error | `degraded` names preserved capability and reduced capability. `warning_error` names risk, validation failure, or failed state. |

## Component-Specific Review Prompts

| Component class | Required prompts |
| --- | --- |
| Badge or pill | Does a policy, permission, or trust badge map to `locked`? Does partial or stale support truth map to `degraded`? Is plain-language expansion available? |
| Banner | Does the banner say whether work is blocked, narrowed, pending, or degraded? Is recovery keyboard reachable? |
| Status item | Is the current live context represented by `current` rather than selected or active styling? Does the item open an inspectable detail route? |
| Dialog action | Does a pending submission avoid looking like generic loading? Are locked actions source-explained rather than merely disabled? Does degraded authority show the fallback path? |
| Settings row | Are policy-managed values `locked`, inspectable-but-not-editable values `read_only`, waiting-to-apply values `pending`, and stale or partial source truth `degraded`? |

## Acceptance Questions

- Does every component state node map to the shared taxonomy?
- Are required non-color cues present for every meaningful state?
- Are semantic tokens, density, motion, high-contrast, and forced-colors
  rules cited by stable refs?
- Are keyboard and assistive-technology paths complete for state
  changes and recovery routes?
- Do loading, pending, streaming, warning/error, and degraded states
  preserve stable layout and hot-path responsiveness?
- Are screenshot baselines and token-drift checks linked for default,
  focus-visible, locked, read-only, pending, degraded, and warning/error
  states when they apply?
- Are docs links, source explanations, and lock explanations verified?
- Are extension parity fixtures present when the component can be
  rendered or approximated outside the host surface?
- Does the component packet's `review_gate_refs` block point at
  `checklist.component_review.reusable`, a stable `evidence.*` design
  release pack id, and explicit waiver state?
