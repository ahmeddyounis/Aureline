# Design Release Evidence Pack Template

Template id: `template.design_release_evidence_pack`

This template is the reviewable packet a launch-critical feature uses
to prove design-complete status. It joins feature readiness,
component review, accessibility, QE, benchmark, compatibility,
migration, public-proof, policy/trust, and waiver evidence through
stable ids.

Use this template with:

- [`docs/ux/feature_readiness_checklist.md`](./feature_readiness_checklist.md)
- [`artifacts/design/component_review_checklist.md`](../../artifacts/design/component_review_checklist.md)
- [`artifacts/ux/review_gate_manifest.yaml`](../../artifacts/ux/review_gate_manifest.yaml)
- [`artifacts/governance/evidence_id_conventions.md`](../../artifacts/governance/evidence_id_conventions.md)
- [`docs/governance/verification_packet_template.md`](../governance/verification_packet_template.md)

## Packet Header

```yaml
schema_version: 1
packet_family: design_release_evidence_pack
packet_id: design_release.<feature_or_surface_id>
evidence_id: evidence.design.<feature_or_surface_id>.release_pack
title: <feature or surface title>
review_status: draft
feature_or_surface_id: <stable id>
surface_traceability_ref: artifacts/ux/surface_traceability_matrix.yaml#<surface-id>
checklist_refs:
  - checklist.feature_readiness.launch_critical
component_checklist_refs:
  - checklist.component_review.reusable
component_packet_refs: []
requirement_refs: []
command_refs: []
source_anchor_refs: []
owner_refs: []
review_forum_refs:
  - design_review
  - accessibility_review
  - quality_engineering_review
answer_summary:
  pass: 0
  not_applicable: 0
  waived: 0
  fail: 0
  unanswered: 0
blocking_item_ids: []
artifact_links:
  supporting_evidence_ids: []
  qe_lane_refs: []
  qe_scenario_refs: []
  benchmark_refs: []
  compatibility_row_refs: []
  migration_packet_refs: []
  public_proof_refs: []
  telemetry_schema_refs: []
  docs_refs: []
waivers:
  waiver_refs: []
  waiver_state: none
  waiver_summary: none
freshness:
  captured_at: YYYY-MM-DDTHH:MM:SSZ
  stale_after: P30D
  source_revision: <commit-or-doc-revision>
visibility_class: internal
```

`review_status` values:

- `draft` - packet is being filled out.
- `reviewable` - every required section has content and stable refs.
- `verified` - reviewers accepted the packet for the declared scope.
- `waived_narrow` - a current waiver narrows or time-boxes the claim.
- `blocked` - one or more required items failed, are unanswered, are
  stale, or have expired waiver coverage.

## 1. Scope And Claim

| Field | Required content |
| --- | --- |
| Feature or surface summary | What user job this packet covers. |
| Scope boundary | Exact product surface, route, component, command, schema, or export path covered. |
| Out of scope | Neighboring behavior this packet does not claim. |
| Lifecycle label | `internal`, `experimental`, `beta`, `stable`, `lts_surface`, `deprecated`, or the local lifecycle row that governs the claim. |
| Public claim impact | Whether this changes user-visible support, compatibility, schema, docs, release-note, marketplace, or extension claims. |

## 2. Annotated State Set

List every state the feature exposes. Default, empty, loading, success,
warning, degraded, error, blocked/locked, stale/partial, restricted,
AI-disabled, remote-disconnected, preview/apply/revert, and recovery
states must appear when relevant.

| State id | Taxonomy refs | User-visible meaning | Primary action | Recovery or inspect route | Evidence refs |
| --- | --- | --- | --- | --- | --- |
| `<state-id>` | `<taxonomy refs>` | `<meaning>` | `<action>` | `<route>` | `<evidence ids or refs>` |

Required notes:

- How the state is conveyed without color alone.
- What still works when the state is degraded.
- Which states block mutation or claim widening.
- Which states are exported to support, telemetry, release, or
  public-proof packets.

## 3. Keyboard And Command Map

| Entry point | Stable ref | Keyboard path | Pointer path parity | Focus return | Notes |
| --- | --- | --- | --- | --- | --- |
| Command palette | `<command-id or route-ref>` | `<keys>` | `<same/narrower>` | `<return rule>` | `<notes>` |

Required notes:

- All primary, alternate, inspect, recovery, cancel, undo, revert, and
  export actions are keyboard reachable.
- Command palette, keybinding, menu, context-menu, CLI, automation, and
  extension entry points cite stable ids or state why no route exists.

## 4. Accessibility Notes

| Area | Required content |
| --- | --- |
| Focus order | Ordered focus path including modal traps and return rules. |
| Screen-reader behavior | Role, name, description, state changes, live-region use, and announcement timing. |
| Keyboard-only proof | Journey refs from accessibility fixtures or future evidence slots. |
| Contrast and forced colors | How dark, light, high-contrast, forced-colors, and zoom preserve meaning. |
| Motion posture | Reduced-motion, low-motion, power-saver, and hot-path behavior. |
| Locale/input posture | Pseudoloc, bidi, IME, dead-key, AltGr, emoji, raw identifier, and exact-date behavior when relevant. |

## 5. Visual Capture Set

Attach or reserve captures for every relevant state in every claimed
theme posture.

| Capture ref | State ids | Theme or posture | Density or viewport | Freshness | Notes |
| --- | --- | --- | --- | --- | --- |
| `<capture-ref>` | `<states>` | `<theme>` | `<density>` | `<current/stale>` | `<notes>` |

Minimum capture set:

- dark reference;
- light parity;
- high-contrast dark or high-contrast light, plus forced-colors notes;
- reduced-motion or motion-suppressed state where animation normally
  communicates progress;
- compact or dense layout where the surface supports it; and
- stale, partial, degraded, locked, or policy-blocked states when they
  exist.

## 6. Performance And Efficiency

| Area | Required content |
| --- | --- |
| Hot-path impact | Which protected path, latency budget, render budget, or interaction budget the design touches. |
| Hidden work | Background refresh, polling, extension work, telemetry, screenshot, capture, or indexing work triggered by the feature. |
| Budget protection | How the design protects foreground editing, navigation, command entry, and recovery paths. |
| Benchmark or efficiency note | Required when the feature makes user-visible latency, benchmark, power, battery, thermal, or throughput claims. |
| Evidence refs | Benchmark packets, protected-path rows, QE lanes, or explicit `not_applicable` reason. |

## 7. Policy, Trust, And Data Boundary

| Area | Required content |
| --- | --- |
| Policy and trust states | Restricted, policy-blocked, entitlement, permission, credential, tenant, source-authority, and managed settings behavior. |
| Local continuity | What remains possible in local-only, offline, remote-disconnected, provider-down, or AI-disabled modes. |
| Evidence/export boundary | What the packet can export, what remains local, what is redacted, and which refs preserve data class truth. |
| User-visible mitigation | Copy and route shown when a waiver or known limit changes user experience. |

## 8. Validation, Telemetry, And Supportability

| Area | Required content |
| --- | --- |
| QE lane refs | Test lanes and scenario hooks expected to prove the feature. |
| Telemetry plan | Event ids, outcome fields, privacy class, redaction posture, and no-raw-secret statement. |
| Usability or dogfood plan | Cohort, scenario, success metric, failure threshold, and rerun trigger. |
| Support plan | Support bundle, object handoff, issue template, doctor packet, or escalation route refs. |
| Evidence refresh | Freshness window and trigger that stale evidence uses to block or narrow claims. |

## 9. Rollout And Lifecycle

| Field | Required content |
| --- | --- |
| Lifecycle state | Current state and target state. |
| Rollout boundary | Channel, cohort, deployment profile, feature flag, policy, or kill-switch scope. |
| Downgrade path | How the feature disables, narrows, reverts, rolls back, or falls back. |
| Docs/release impact | Docs, help, release-note, known-limit, or support-note refs. |
| Waiver posture | `none`, `active`, `expired`, or `closed`, plus mitigation and exit signal. |

## 10. Migration, Extension, Compatibility, And Public Proof

| Area | Required content |
| --- | --- |
| Compatibility impact | Compatibility row ids, skew posture, schema stability, deprecation, or support-window impact. |
| Migration impact | Import/export/session/outcome packet refs, profile or settings migration impact, and rollback checkpoint story. |
| Extension impact | Host-owned component parity, extension bridge, SDK, manifest, command, token, theme, or permission impact. |
| Public proof | Public-proof packet refs, scoreboard refs, benchmark-publication refs, or explicit no-public-claim statement. |
| Schema stability | Public-schema label, versioning rule, migration note, and compatibility suite refs when applicable. |

## 11. Checklist Answer Table

Copy every required item from
`checklist.feature_readiness.launch_critical` and add evidence refs.

| Checklist item id | Answer | Evidence refs | Reviewer | Notes or waiver ref |
| --- | --- | --- | --- | --- |
| `readiness.state.default` | `<pass/not_applicable/waived/fail/unanswered>` | `<refs>` | `<reviewer>` | `<notes>` |

Rules:

- `pass` requires evidence refs.
- `not_applicable` requires a reason and reviewer.
- `waived` requires a waiver ref and mitigation.
- `fail`, `unanswered`, stale evidence, and expired waivers block
  design-complete.

## 12. Reviewer Decision

| Reviewer or forum | Decision | Date | Blocking refs | Notes |
| --- | --- | --- | --- | --- |
| `<forum>` | `<accept/reject/needs_follow_up/waived>` | `YYYY-MM-DD` | `<refs>` | `<notes>` |

Final decision values:

- `design_complete`
- `blocked`
- `waived_narrow`
- `needs_review`

The final decision must match the corresponding row in
`artifacts/ux/review_gate_manifest.yaml`.
