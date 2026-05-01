# Shared component state taxonomy

This taxonomy is the canonical state vocabulary for reusable component
contracts. Component packets, design review, QA evidence, docs, and
telemetry use these state names and refs so a component does not invent
local labels for the same user-visible meaning.

Companion artifacts:

- [`/schemas/design/component_state_machine.schema.json`](../../schemas/design/component_state_machine.schema.json)
  validates reusable component state-machine packets and evidence hooks.
- [`/artifacts/design/component_review_checklist.md`](../../artifacts/design/component_review_checklist.md)
  is the shared review checklist for design, engineering, QA, docs, and
  extension parity.
- [`/schemas/design/component_contract.schema.json`](../../schemas/design/component_contract.schema.json)
  allows component packets to point state nodes at this taxonomy and to
  link the evidence hooks defined here.
- [`/docs/ux/component_contract_template.md`](../ux/component_contract_template.md)
  describes the broader component packet shape.
- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  remains the token and theme vocabulary. This taxonomy narrows the
  shared user-visible state meanings that component contracts cite.
- [`/artifacts/design/component_state_screenshot_corpus.yaml`](../../artifacts/design/component_state_screenshot_corpus.yaml)
  binds spec-named state names (Empty, Loading, Pending, Degraded,
  Blocked, Error, Completed, Restored, Restricted, Policy blocked,
  Quiet-hours held, Reconnecting) on launch-critical surfaces to
  these taxonomy classes and reserves the per-state screenshot,
  honesty-review axis, and non-color-cue requirements.
- [`/docs/design/component_state_diff_packet_template.md`](./component_state_diff_packet_template.md)
  is the build-to-build, revision-to-revision, and release-evidence
  review-packet template that consumes the corpus and routes
  honesty-review and state-semantic diff findings through the
  existing token-drift fail gates.

Normative sources projected here:

- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md#15.4-shared-component-state-taxonomy`
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md#34.2-component-review-checklist`
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md#appendix-n--critical-component-state-matrix`
- `.t2/docs/Aureline_UI_UX_Spec_Document.md#23.2-component-review-checklist`

## State Classes

The set below is closed for shared component state. A component may keep
domain lifecycle names such as `queued` or `awaiting_input`, but every
state node in a reusable packet must map those names back to one or more
of these taxonomy classes.

| Taxonomy class | Display label | Meaning | Required treatment |
| --- | --- | --- | --- |
| `default` | Default | Resting state with no extra user-visible posture. | Base semantic token set. |
| `hover` | Hover | Pointer intent. | Subtle surface or border change only; no hover-only meaning. |
| `focus_visible` | Focus-visible | Keyboard or assistive-technology focus. | Strong visible focus treatment; never color-only. |
| `pressed_active` | Pressed / active | Immediate activation in progress. | Stronger contrast or lowered elevation; no layout shift. |
| `selected` | Selected | Part of a chosen set. | Persistent fill, border, check, or count cue. |
| `current` | Current | Current location, route, live context, or row driving detail panes. | Stronger than selected when navigation or live context matters. |
| `disabled` | Disabled | Unavailable and non-actionable because the control has no valid action in the current context. | Lower emphasis plus text, icon, or state copy; do not use for policy or trust constraints. |
| `read_only` | Read-only | Inspectable or copyable but not editable or writable. | Preserve content contrast; mute or remove editing affordances; name the read-only scope. |
| `loading` | Loading | Content or context is not ready yet and was not caused by a just-submitted user action. | Skeleton, placeholder, or progress with stable layout. |
| `pending` | Pending | A user action was submitted, staged, or queued and has not committed yet. | Submission feedback plus cancel, retry, or review route where useful. |
| `warning_error` | Warning / error | Risky, invalid, failed, or destructive-adjacent state that needs attention. | Semantic status plus text and icon; color alone is non-conforming. |
| `locked` | Locked | Policy, permission, trust, capability, ownership, or source constraint prevents editing or action. | Lock or shield cue plus reason, source, and inspect route. |
| `degraded` | Degraded | Partial capability remains while certainty, freshness, authority, or subsystem quality is reduced. | Name what still works, what is reduced, how to recover, and whether certainty changed. |

## State Distinction Rules

These distinctions are load-bearing. Component packets, review
checklists, schemas, fixtures, telemetry labels, and docs copy must keep
them separate.

| Distinction | Rule |
| --- | --- |
| Locked vs disabled | Use `locked` when a policy, trust, permission, ownership, missing capability, or source authority explains the block. Use `disabled` only for ordinary context unavailability where no source or lock explanation applies. |
| Read-only vs disabled | Use `read_only` when users can still inspect, copy, navigate, or export content. Use `disabled` only when there is no useful action from the control itself. |
| Pending vs loading | Use `pending` after a user action has been submitted, staged, queued, or is awaiting commit. Use `loading` for initial fetch, warm-up, or context preparation that is not itself the result of a submitted action. |
| Selected vs current | Use `selected` for stable membership in a chosen set. Use `current` for the item driving details, navigation, live execution, or route context. A row can be current without being selected, and selected without being current. |
| Degraded vs warning/error | Use `degraded` when a reduced but usable mode remains. Use `warning_error` when attention, validation, or failure is the primary meaning. A degraded state may also carry warning copy, but it must still name preserved capability. |

## Evidence Hooks

Reusable component contracts must link evidence through typed refs
rather than free-form notes. The same hook names are used by the state
machine schema and component contract schema.

| Hook field | Required evidence |
| --- | --- |
| `screenshot_baseline_refs` | Dark, light, high-contrast, density, and relevant state captures. |
| `keyboard_journey_refs` | Keyboard-only path, activation, dismissal, focus-return, and recovery routes. |
| `assistive_technology_refs` | Screen-reader tree, announcement, name/description, and platform AT checks. |
| `token_drift_check_refs` | Semantic token, contrast, density, and forced-colors drift checks. |
| `docs_link_verification_refs` | Verified docs, help, source, policy, or lock explanation links. |
| `extension_parity_fixture_refs` | Extension, embedded, companion, or handoff parity fixtures when the component can appear outside the host-owned surface. |
| `state_machine_validation_refs` | State-node and transition probes proving the packet maps local states to this taxonomy. |

## Component Mapping

The mapping below is explicit so badges, banners, status items, dialog
actions, and settings rows do not reinterpret locked, degraded, pending,
or current states.

| Component class | Required taxonomy mapping | Notes |
| --- | --- | --- |
| Badge or pill | `default`, `current`, `pending`, `warning_error`, `locked`, `degraded` | A badge may carry semantic status, but policy or permission posture maps to `locked`, not a custom "policy badge" state. A stale or partial claim maps to `degraded`, with freshness or certainty copy. |
| Banner | `default`, `loading`, `pending`, `warning_error`, `locked`, `degraded` | A banner must name whether it blocks work, narrows capability, or simply reports state. Degraded banners name preserved capability. |
| Status item | `default`, `hover`, `focus_visible`, `current`, `loading`, `pending`, `warning_error`, `locked`, `degraded` | The current live context uses `current`; a provider or policy block uses `locked`; reconnecting or partial service quality uses `degraded` with inspectable details. |
| Dialog action | `default`, `hover`, `focus_visible`, `pressed_active`, `disabled`, `pending`, `locked`, `degraded`, `warning_error` | A pending submit button is not loading chrome. A policy- or permission-blocked action is locked and must expose source/reason. A degraded action names the fallback or reduced authority path. |
| Settings row | `default`, `hover`, `focus_visible`, `current`, `read_only`, `pending`, `degraded`, `warning_error`, `locked`, `disabled` | Managed or policy-enforced values are `locked`. View-only values are `read_only`. A restart or sync write waiting to commit is `pending`. Stale or partial source truth is `degraded`. |

## Component Packet Rules

1. A component packet may use local `state_ref` names, but each node
   should carry `taxonomy_state_refs` from this document.
2. A state that includes `locked` must provide a source or lock
   explanation and an inspect route.
3. A state that includes `read_only` must name the read-only scope and
   preserve inspectable content contrast.
4. A state that includes `pending` must name the submitted or staged
   user action.
5. A state that includes `current` must identify the route, detail owner,
   live context, or current item it represents.
6. A state that includes `degraded` must name preserved capability,
   reduced capability, recovery path, and certainty or freshness impact.
7. Color alone never satisfies a state. Each state needs text, icon,
   shape, border, position, progress, or another non-color cue.
8. Extension or embedded surfaces that cannot preserve the same state
   meaning must disclose the gap or hand off to the host-owned
   component.
