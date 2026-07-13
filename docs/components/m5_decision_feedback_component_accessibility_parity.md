# M5 decision-feedback component accessibility & auto-narrowing parity (M05-1138)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5 decision-feedback
component matrix (`m5_decision_feedback_component_matrix`). Where the freeze matrix defines the reusable
**badge-chip-pill, popover, dialog-sheet, banner-inline-notice, toast, empty-state, loading-state, and
consequence-block** primitives, and the 1133–1136 implementation lanes resolve their per-surface truth,
this lane certifies — per primitive family — that every decision / feedback claim survives beyond the
pointer-rich desktop view and **auto-narrows when its severity / scope / recovery / focus-return /
durable-object proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_decision_feedback_accessibility_parity_and_narrowing_when_decision_feedback_truth_is_stale/`
- **Schema:** `schemas/ui/m5-decision-feedback-component-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-decision-feedback-component-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/ui/m5-decision-feedback-component-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every family exposes a keyboard-complete,
   screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and CLI/headless-reachable path into
   the same primitive identity, disposition / state, severity, scope, rationale, recovery path,
   focus-return anchor, and durable-object linkage the rich primitive shows — never a color-only badge, a
   hover-only popover, a toast-only cue, or a motion-only spinner. The support / release / CLI export
   reconstructs each primitive's meaning from typed tokens and opaque refs **without a raw payload**.

2. **Honest auto-narrowing.** When a badge's severity evidence is stale, a banner's scope cannot be
   confirmed, a popover's focus-return anchor is stale, a toast's durable-object linkage is missing, a
   loading state can only prove a partial capability, or a consequence block can only disclose a partial
   recovery / rollback posture, the claim auto-narrows from `trusted_decision_surface` /
   `reviewable_decision_surface` to the matching projection, discloses the narrowing with a precise trigger
   and binding dimension, and preserves the canonical identity / last-known state. A primitive with every
   dimension intact must **not** carry a spurious narrowing, and a weakened primitive can never keep a
   trusted, ready-to-read claim — a durable outcome is never represented as toast-only truth, and a partial
   capability never hides behind a full-screen spinner.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the shell, help, support, review,
   settings, updates, CLI-export, support-export, and product surfaces so product, help, and release
   publication stay aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_decision_surface` | Fully current, severity-clear, scoped, focus-anchored, durable-linked — ready to read. |
| `reviewable_decision_surface` | Self-sufficient, reviewable read-only primitive (a badge / empty state a user can inspect), not an authoritative action-driving surface. |
| `severity_unverified_projection` | Badge / notice severity evidence is stale (badge-chip-pill). |
| `scope_unverified_projection` | Banner / notice scope cannot be confirmed (banner-inline-notice). |
| `focus_return_unverified_projection` | Popover's safe focus-return anchor cannot be confirmed (popover). |
| `durable_object_unverified_projection` | Toast's durable-object back-link is missing (toast). |
| `partial_capability_unverified_projection` | Loading state can only prove a partial capability (loading-state). |
| `recovery_path_disclosed_projection` | Consequence block can only disclose a partial / redacted recovery posture — an **honest disclosed-absence**, not a truth overstatement (consequence-block). |

## Weakening dimensions and their frozen triggers

Each family maps 1:1 to a claim dimension; a weak condition state narrows to the matching projection and
names the on-topic frozen matrix downgrade trigger:

| Dimension (family) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `severity_meaning_clarity` (badge-chip-pill) | `severity_evidence_stale` | `state_taxonomy_drifted` | yes |
| `notice_scope_clarity` (banner-inline-notice) | `scope_evidence_stale` | `scope_unstated` | yes |
| `focus_return_anchor_clarity` (popover) | `focus_return_anchor_stale` | `popover_carried_only_critical_instruction` | yes |
| `durable_object_linkage_clarity` (toast) | `durable_object_linkage_stale` | `durable_work_shown_as_toast_only` | yes |
| `partial_capability_fidelity_clarity` (loading-state) | `partial_capability_unconfirmed` | `full_screen_spinner_when_partial_capable` | yes |
| `blast_radius_recovery_clarity` (consequence-block) | `recovery_path_disclosed_partial` | `proof_stale` | no (honest disclosed-absence) |
| `rationale_scope_action_clarity` (dialog-sheet) | *(green — fully qualified trusted)* | — | — |
| `purpose_next_action_clarity` (empty-state) | *(green — fully qualified reviewable)* | — | — |

The `recovery_path_disclosed_partial` state is deliberately **excluded** from
`cannot_be_shown_trusted`: a partial / redacted recovery posture shown honestly with an inspectable
recovery note is a disclosed-absence operation, not a truth overstatement.

## Structure-heavy families

The **dialog-sheet** (structured action set), **popover** (anchored secondary content), and
**consequence-block** (named blast radius) render a dense structured surface, so they must additionally
bind their structured layout to an equivalent flat list / textual path (a `structured` fallback modality
**plus** a non-visual list / textual / CLI path).

## Certified rows

Eight rows, one per family: **1 green** (dialog-sheet — rationale, scope, and actions fully stated,
trusted) and **7 yellow** — the empty state stays a fully-qualified reviewable surface but discloses a
screen-reader reduction, and the remaining six auto-narrow to their permitted projections. **No red rows
may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To
regenerate after an intentional change:

```
GEN_DECISION_FEEDBACK_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_decision_feedback_accessibility_parity_and_narrowing_when_decision_feedback_truth_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
