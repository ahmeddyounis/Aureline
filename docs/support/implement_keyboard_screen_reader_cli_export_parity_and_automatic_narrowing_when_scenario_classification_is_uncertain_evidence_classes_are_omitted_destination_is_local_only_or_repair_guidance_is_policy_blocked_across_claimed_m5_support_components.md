# M5 support-intake / escalation component accessibility & auto-narrowing (M05-906)

Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the M5
support-intake and escalation components, layered on top of the frozen
[M5 support-intake / escalation component matrix](../../schemas/ui/m5-support-intake-escalation-component-matrix.schema.json)
(M05-900) and the 901–905 implementation / consumer lanes.

- Contract module: `crates/aureline-support/src/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_scenario_classification_is_uncertain_evidence_classes_are_omitted_destination_is_local_only_or_repair_guidance_is_policy_blocked_across_claimed_m5_support_components/`
- Boundary schema: [`schemas/ui/m5-support-intake-escalation-component-accessibility-fallback.schema.json`](../../schemas/ui/m5-support-intake-escalation-component-accessibility-fallback.schema.json)
- Support export: [`artifacts/release/m5-support-intake-escalation-component-accessibility-fallback/support_export.json`](../../artifacts/release/m5-support-intake-escalation-component-accessibility-fallback/support_export.json)
- Matrix CSV: [`artifacts/release/m5-support-intake-escalation-component-accessibility-fallback/matrix.csv`](../../artifacts/release/m5-support-intake-escalation-component-accessibility-fallback/matrix.csv)
- Markdown report: [`artifacts/release/m5-support-intake-escalation-component-accessibility-fallback.md`](../../artifacts/release/m5-support-intake-escalation-component-accessibility-fallback.md)
- Protected fixtures: `fixtures/ui/m5-support-intake-escalation-component-accessibility-fallback/`

## What this lane certifies

Where the freeze matrix defines the reusable support-scenario picker row, issue-report builder
step, escalation-packet summary, handoff-timeline row, and unsafe-fix blocked-note primitives,
and the 901–905 lanes resolve their per-surface truth, this capstone certifies — per component
family — that support-intake and escalation claims stay **keyboard-complete,
assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than presenting an
uncertain scenario classification, an evidence-omitted report, a local-only destination, or a
policy-blocked repair as a still ready-to-escalate case.

Each `SupportIntakeComponentAccessibilityRow` keys on one frozen
`M5SupportIntakeEscalationComponentFamily` and reuses the frozen `M5SupportRequiredLabel`,
`M5SupportDowngradeTrigger`, and shared `M5SupportConsumerSurface` vocabulary rather than minting
parallel synonyms, so the certified labels stay byte-identical to the matrix and the sibling
primitive packets.

### Keyboard / screen-reader / CLI reach

Every family exposes a keyboard-complete, screen-reader-reachable, and CLI/headless-reachable
path into the same scenario family, incident scope, selected and omitted evidence classes,
Doctor finding lineage, approved repair class, packet destination, redaction state, and next
human step the rich component shows — never a hover-only chip that strands assistive-tech or
headless users. Hierarchy-heavy families (the escalation-packet summary's nested finding /
repair / evidence lineage) additionally bind their tree to a flat list / textual path.

### Export parity

The support / release / evaluation export reconstructs each component's meaning from typed
tokens and opaque refs without a screenshot, preserving the same stable scenario codes,
data-class labels, packet IDs, redaction state, and narrowing reasons shown in-product so
scenario / evidence / packet truth can be reconstructed without screenshots or private team
memory.

### Honest auto-narrowing

When scenario classification is uncertain, evidence classes are omitted, a destination is
local-only, or repair guidance is policy-blocked, the component's support claim auto-narrows from
`ready_to_escalate` / `reviewable_case` to an `unclassified_scenario` / `evidence_incomplete_case`
/ `local_only_diagnosis` / `policy_blocked_repair` case, discloses the narrowing with a precise
trigger and binding dimension, and preserves the canonical scenario / finding / packet /
redaction / repair lineage — the underlying case lineage is never dropped opaquely. A component
with every dimension intact must NOT carry a spurious narrowing.

### Cross-surface disclosure

The same narrowed state surfaces in the Doctor UI, support center, report builder, escalation
desk, recovery center, Help center, headless CLI, and support / release exports so product,
docs, and release publication stay aligned on support-intake / escalation downgrade behavior — a
ready-looking case can never outrun the scenario / evidence / destination / repair proof it is
being viewed away from.

## Claim ladder and condition ceilings

| Support claim (`M5SupportIntakeClaim`) | Rank | Reached by condition (`M5SupportIntakeConditionState`) |
| --- | --- | --- |
| `ready_to_escalate` | 5 | `classified` |
| `reviewable_case` | 4 | (family-only full claim; a self-sufficient reviewable case) |
| `local_only_diagnosis` | 3 | `local_only_destination` |
| `evidence_incomplete_case` | 2 | `evidence_omitted` |
| `unclassified_scenario` | 1 | `scenario_uncertain` |
| `policy_blocked_repair` | 0 | `repair_policy_blocked` |

Each dimension names the on-topic frozen downgrade trigger from the matrix:

| Dimension (`M5SupportIntakeClaimDimension`) | Family | Default trigger (`M5SupportDowngradeTrigger`) |
| --- | --- | --- |
| `scenario_classification` | support-scenario picker row | `scenario_or_scope_unstated` |
| `evidence_completeness` | issue-report builder step | `evidence_class_masked` |
| `destination_reach` | escalation-packet summary | `packet_destination_unstated` |
| `handoff_continuity` | handoff-timeline row | `next_human_step_unstated` |
| `repair_guidance` | unsafe-fix blocked note | `approved_repair_class_masked` |

## Acceptance criteria mapping

- **No support path becomes pointer-only, screen-reader ambiguous, or export-opaque once
  scenario coding and escalation begin.** Each row proves keyboard / screen-reader / CLI reach
  into the canonical truth (`reaches_canonical_truth_via_at`), an export summary that
  reconstructs without a screenshot (`export_preserves_meaning`), and a hierarchy-heavy family
  binds its nested lineage to a non-visual fallback. A view-only trap, an empty context ref, or
  a screenshot-only export strands the row (red).
- **Uncertain classification, omitted evidence, local-only destinations, and policy-blocked
  repair guidance all narrow claims with controlled vocabulary across desktop and headless
  surfaces.** Each weak condition auto-narrows the claim to exactly its permitted ceiling, binds
  the ceiling-imposing dimension with its frozen trigger, and preserves canonical identity and
  case lineage (`claim_is_honest`, `preserves_lineage_continuity`). Over-asserting a ready /
  reviewable case, a spurious narrow, a dropped lineage, or an undisclosed narrowed surface all
  fail validation.

## Regenerating the artifacts

The checked-in support export, CSV, Markdown report, and fixture mirror are produced from the
single seeded packet builder so all copies stay byte-aligned:

```
GEN_SUPPORT_INTAKE_COMPONENT_A11Y_ARTIFACTS=1 \
  cargo test -p aureline-support --lib generate_artifacts
```

The packet is metadata-only: raw logs, transcripts, attachment bytes, and credential-bearing
material never cross this boundary.
