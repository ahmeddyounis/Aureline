# Accessibility review charter (seed)

This seed charter makes accessibility a named review lane with one
owner, one forum, one acceptance-pack family reservation, and one
public backlog mapping. It complements the accessibility packet
template and machine-readable matrices; it does not replace them.

Companion artifacts:

- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — canonical control-artifact row for the accessibility review packet
  family.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — the `accessibility_input_review` lane and `accessibility_review`
  forum owner records.
- [`/docs/governance/forum_charters.md`](../governance/forum_charters.md),
  [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml),
  and
  [`/docs/governance/forum_packet_templates.md`](../governance/forum_packet_templates.md)
  — standing forum cadence, packet profile, and escalation rules.
- [`/docs/accessibility/a11y_ime_packet_template.md`](./a11y_ime_packet_template.md)
  — reviewer-facing packet template and shared evidence-header shape.
- [`/artifacts/accessibility/assistive_tech_matrix.yaml`](../../artifacts/accessibility/assistive_tech_matrix.yaml)
  — machine-readable review-planning matrix for macOS, Windows, and
  Linux accessibility coverage rows.
- [`/artifacts/accessibility/platform_input_matrix.yaml`](../../artifacts/accessibility/platform_input_matrix.yaml)
  — canonical AT, input-method, and locale row registry.
- [`/docs/i18n/locale_input_readiness.md`](../i18n/locale_input_readiness.md),
  [`/artifacts/i18n/test_mode_matrix.yaml`](../../artifacts/i18n/test_mode_matrix.yaml),
  and
  [`/fixtures/i18n/pseudoloc_rtl_ime_manifest.yaml`](../../fixtures/i18n/pseudoloc_rtl_ime_manifest.yaml)
  — canonical locale/input readiness baseline, cross-surface
  pseudoloc/RTL/CJK/IME test-mode rows, and the seed harness plan that
  accessibility packets compose over.
- [`/artifacts/accessibility/shell_conformance_checklist.yaml`](../../artifacts/accessibility/shell_conformance_checklist.yaml)
  and
  [`/artifacts/accessibility/accessibility_tree_coverage_rows.yaml`](../../artifacts/accessibility/accessibility_tree_coverage_rows.yaml)
  — launch-critical shell checklist and tree-capture rows.
- [`/fixtures/accessibility/task_corpus_manifest.yaml`](../../fixtures/accessibility/task_corpus_manifest.yaml)
  — stable task ids for command discovery, entry/restore, trust,
  docs/help, task review, terminal review, and dense-collection review.
- [`/docs/governance/dogfood_issue_taxonomy.md`](../governance/dogfood_issue_taxonomy.md)
  and
  [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — public issue routing and severity vocabulary the label reservations
  below compose over.
- [`/docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md)
  — stable release-evidence packet that later cites current acceptance
  packs, known limits, and waivers from this lane.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — Section 23 and the desktop accessibility / input architecture
  obligations.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — Section 19.8, Section 23.31, Appendix CD, and the launch-critical
  keyboard/focus/announcement contract.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
  — accessibility acceptance-pack and release-evidence expectations.
- `.t2/docs/Aureline_Milestones_Document.md`
  — review-cadence, waiver, release-evidence, and launch-critical-copy
  rules for accessibility-facing surfaces.

## Purpose

The accessibility review lane exists to keep keyboard, assistive-tech,
input-method, zoom, contrast, and motion truth reviewable before:

- implementation broadens past hook-only seams;
- stable-facing claim language widens;
- release packets need to cite current accessibility evidence and known
  limits by stable id.

This lane is the named home for accessibility acceptance packs. Teams
must extend the chartered packet family instead of keeping per-surface
checklists in issue comments, screenshots, or one-off spreadsheets.

## Roles

All roles below currently resolve to the solo-maintainer posture
recorded in [`/docs/governance/dri_map.md`](../governance/dri_map.md)
and the `single-maintainer-backup` waiver in
[`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).

- **Forum chair.** Runs the `accessibility_review` forum, records
  outcomes in the accessibility packet family, and requests release or
  milestone escalation when the lane blocks widening.
- **Lane DRI.** Owns `docs/accessibility/`, `artifacts/accessibility/`,
  and `fixtures/accessibility/` as one review family under
  `accessibility_input_review`.
- **Evidence owner.** Maintains packet freshness, known-limit refs,
  waiver refs, and release-packet joins for claimed accessibility rows.
- **Surface owner liaison.** Joins reviews for the affected surface
  family when command discovery, docs/help, trust, execution, or dense
  review flows change.
- **Design-system liaison.** Confirms zoom, contrast, motion, and
  controlled-language posture for launch-critical surfaces and keeps the
  accessibility lane aligned with `design_system_seeds`.

## Minimum review scope before M1 broadens

The following surface families must have at least one named task id in
[`/fixtures/accessibility/task_corpus_manifest.yaml`](../../fixtures/accessibility/task_corpus_manifest.yaml)
and at least one platform row in
[`/artifacts/accessibility/assistive_tech_matrix.yaml`](../../artifacts/accessibility/assistive_tech_matrix.yaml)
before the affected surface may widen beyond hook-only posture:

- **Command discovery** — command palette, quick open, disabled reasons,
  result counts, invocation, and focus return.
- **Project entry and restore** — open workspace, import, restore recent
  work, and recover from missing targets.
- **Trust and permission prompts** — workspace trust, permission sheets,
  sign-in fallbacks, browser handoff, and continue-local or deny flows.
- **Docs/help reading** — docs/help panes, source or freshness badges,
  source-language fallback, and external-open routes where allowed.
- **Task and terminal review** — task run or rerun, failure review,
  terminal transcript summary, copy/export, reconnect, and follow-up
  actions.
- **Dense collection review** — selected-count narration, query scope,
  range anchor, blocked rows, and review-sheet focus return.

The charter deliberately reserves these as the minimum launch-critical
surfaces. Later milestones may add editor, notebook, collaboration, or
voice-specific packs without renaming the family ids below.

## Packet and acceptance-pack expectations

Every accessibility review packet must:

- use the shared verification-header rules from
  [`a11y_ime_packet_template.md`](./a11y_ime_packet_template.md);
- cite one or more checklist ids from
  [`shell_conformance_checklist.yaml`](../../artifacts/accessibility/shell_conformance_checklist.yaml);
- cite one or more platform review rows from
  [`assistive_tech_matrix.yaml`](../../artifacts/accessibility/assistive_tech_matrix.yaml);
- cite one or more stable task ids from
  [`task_corpus_manifest.yaml`](../../fixtures/accessibility/task_corpus_manifest.yaml);
- carry `known_limit_refs` and `waiver_refs` explicitly whenever a row
  is not `passed`;
- update the release packet and claim or known-limit surfaces in the
  same change when a stable-facing accessibility claim narrows.

`pending_evidence` is valid for seed planning artifacts. It is not
promotion-grade evidence.

### Acceptance-pack family reservations

These family ids are now reserved and must remain stable:

| Family id | Scope | Reserved packet / known-limit prefixes |
|---|---|---|
| `acceptance_pack.accessibility.command_discovery` | command palette and quick open | `verification.accessibility.acceptance.command_discovery` / `known_limit.accessibility.command_discovery` |
| `acceptance_pack.accessibility.project_entry_and_restore` | open, import, restore, and recent-work recovery | `verification.accessibility.acceptance.project_entry_and_restore` / `known_limit.accessibility.project_entry_and_restore` |
| `acceptance_pack.accessibility.trust_and_permission` | trust prompts, permission sheets, sign-in recovery, and safe fallback routes | `verification.accessibility.acceptance.trust_and_permission` / `known_limit.accessibility.trust_and_permission` |
| `acceptance_pack.accessibility.docs_and_help` | docs/help, source-language fallback, freshness, and external-open routes | `verification.accessibility.acceptance.docs_and_help` / `known_limit.accessibility.docs_and_help` |
| `acceptance_pack.accessibility.task_and_terminal_review` | task review, terminal review, transcript follow-up, and rerun or reconnect truth | `verification.accessibility.acceptance.task_and_terminal_review` / `known_limit.accessibility.task_and_terminal_review` |
| `acceptance_pack.accessibility.dense_review_collection` | dense collections, selected-count narration, range selection, and batch review | `verification.accessibility.acceptance.dense_review_collection` / `known_limit.accessibility.dense_review_collection` |

Stable release reviews may cite these family ids directly. Future work
must not replace them with milestone-local or surface-local aliases.

## Public backlog and issue-label mapping

Public routing still resolves through
[`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
and
[`/docs/governance/dogfood_issue_taxonomy.md`](../governance/dogfood_issue_taxonomy.md).
The label reservations below add searchability and backlog slicing; they
do not invent a second routing system.

| Review area | Primary label | Reserved secondary labels | Default route classes |
|---|---|---|---|
| Keyboard route, focus order, focus return | `dogfood:accessibility` | `a11y:keyboard` | `oss_bug`, `design_system_defect` |
| Screen-reader labels, announcements, tree coverage | `dogfood:accessibility` | `a11y:screen-reader` | `oss_bug`, `design_system_defect` |
| Zoom, contrast, and low-motion posture | `dogfood:accessibility` | `a11y:zoom`, `a11y:high-contrast`, `a11y:reduced-motion` | `design_system_defect`, `oss_bug` |
| IME, bidi, dead-key, AltGr, compose, and text fidelity | `dogfood:accessibility` | `a11y:ime-input` | `oss_bug`, `supportability_issue` |
| Docs/help reading and learning surfaces | `dogfood:accessibility` | `a11y:docs-help` | `docs_truth_defect`, `oss_bug` |
| Trust, permission, and recovery prompts | `dogfood:accessibility` | `a11y:trust-prompt` | `supportability_issue`, `oss_bug` |
| Task review, terminal review, and dense review flows | `dogfood:accessibility` | `a11y:task-terminal`, `a11y:dense-review` | `supportability_issue`, `oss_bug` |

Every public accessibility issue should also carry one `sev:*` label
from the dogfood taxonomy.

## Cadence and outputs

- **Standing cadence:** bi-weekly through the `accessibility_review`
  forum.
- **Ad-hoc cadence:** within 5 business days of a regression on a
  claimed platform row or any waiver request touching a launch-critical
  accessibility flow.
- **Release-window cadence:** pre-beta, pre-RC, and pre-stable whenever
  a candidate widens a claimed accessibility-facing row or carries an
  accessibility known limit or waiver.

Every session must leave a typed output:

- accessibility packet update under `docs/accessibility/` or
  `artifacts/accessibility/`;
- scorecard or risk update when lane posture changes;
- release-packet, claim-row, or known-limit update when stable-facing
  wording changes.

## Waivers and known limits

- Accessibility waivers must be time-bounded, named by stable scope,
  and paired with a user-visible workaround or fallback.
- A waiver may narrow a support class; it may not imply parity on an
  unreviewed platform or AT row.
- Stable-facing accessibility exceptions must update both the
  accessibility packet family and the release packet family in the same
  change, with the same family id and `known_limit_refs`.
- Repeated waivers on the same acceptance-pack family are a correction
  trigger, not a standing exception culture.

## Seed posture and change discipline

This is a seed charter, not a claim that current accessibility evidence
is green across the full product. The packet home, task ids, and review
rows now exist so later implementation and release work can attach
current proof without renaming the lane.

Any change to this charter, the assistive-tech matrix, or the task
corpus manifest must update the `accessibility_review_packets` control-
artifact row in the same change when the family posture moves.
