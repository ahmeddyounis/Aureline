# Accessibility and IME Packet Template

This document is the reviewer-facing packet template for Aureline's
accessibility and platform-input evidence. It makes keyboard,
focus-return, screen-reader, zoom, motion, contrast, IME, dead-key,
AltGr, emoji, bidi, and accessibility-tree review one packet family
instead of ad hoc spreadsheets.

Companion artifacts:

- [`/docs/accessibility/review_charter.md`](./review_charter.md)
  — named review-lane charter covering owner, cadence,
  acceptance-pack families, public backlog mapping, and waiver posture.
- [`/artifacts/accessibility/platform_input_matrix.yaml`](../../artifacts/accessibility/platform_input_matrix.yaml)
  — machine-readable platform, assistive-technology, locale, and
  input-path matrix with the shared result-state vocabulary.
- [`/artifacts/accessibility/assistive_tech_matrix.yaml`](../../artifacts/accessibility/assistive_tech_matrix.yaml)
  — review-planning matrix tying each claimed desktop platform scope to
  screen-reader, keyboard-only, zoom, high-contrast, reduced-motion,
  and input-method rows.
- [`/artifacts/accessibility/shell_conformance_checklist.yaml`](../../artifacts/accessibility/shell_conformance_checklist.yaml)
  — launch-critical shell checklist for keyboard, focus, narration,
  zoom, contrast, motion, and input behavior.
- [`/artifacts/accessibility/accessibility_tree_coverage_rows.yaml`](../../artifacts/accessibility/accessibility_tree_coverage_rows.yaml)
  — required accessibility-tree capture rows for the launch-critical
  shell surfaces.
- [`/fixtures/accessibility/ime_and_text_cases/`](../../fixtures/accessibility/ime_and_text_cases/)
  — seeded IME, bidi, copy-parity, virtualization, range-selection, and
  mixed-DPI cases this packet cites.
- [`/fixtures/accessibility/task_corpus_manifest.yaml`](../../fixtures/accessibility/task_corpus_manifest.yaml)
  — stable task ids and acceptance-pack families future benchmark,
  conformance, and release packets will cite.
- [`/docs/governance/verification_packet_template.md`](../governance/verification_packet_template.md)
  — shared evidence header, claim-row, freshness, and signoff rules.
- [`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md)
  — canonical shell/input/accessibility boundary.
- [`/docs/architecture/input_adapter_failure_modes.md`](../architecture/input_adapter_failure_modes.md)
  — degraded and blocked-state rules for input-path and accessibility
  bridge failures.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — Section 23, Section 27.23, `FIT-A11Y-001`, `A11Y-CORE-002`,
  `A11Y-SR-003`, `A11Y-TEXT-004`, `A11Y-I18N-007`, and
  `A11Y-INCL-005`.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — desktop support contract, accessibility signoff, assistive-tech and
  inclusive-learning lanes, and IME/input fidelity rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — Appendix G, Appendix CD, Appendix CL, Appendix EL, Section 19.12,
  Section 23.31, and Section 23.41.

## Shared result states

This packet reuses the result-state vocabulary from
`artifacts/accessibility/platform_input_matrix.yaml` and the stable task
and platform-review ids from `task_corpus_manifest.yaml` and
`assistive_tech_matrix.yaml`.

| State | Meaning | Required packet behavior |
|---|---|---|
| `passed` | claimed path behaved within the named scope | keep the claim row green and cite evidence ids |
| `degraded` | usable path remained, but the behavior narrowed visibly | name the narrowed route, user-visible cue, and recovery action |
| `blocked` | the path could not complete because the platform, policy, or precondition prevented it | name the blocker and the safe fallback or recovery path |
| `failed` | a claim-bearing path behaved incorrectly | capture the regression and narrow the claim row until fixed |
| `unclaimed` | outside the defended scope | keep the row explicit; do not imply parity elsewhere |

`pending_evidence` is allowed only in seed packets or milestone-planning
drafts. Claim-bearing release packets should resolve every row into one
of the five states above.

## Packet rules

- Every surface row MUST cite:
  one `checklist_id`, one or more platform or AT row refs, and one or
  more accessibility-tree row refs.
- Every `degraded`, `blocked`, or `failed` row MUST name:
  the failure class, the user-visible posture, and the recovery or
  narrowing action.
- Every IME or text-input row MUST record:
  platform profile, AT tool if active, locale, input method or keyboard
  layout, and whether preedit survived focus churn, filtering, and
  window topology changes.
- Every copy-parity row MUST distinguish:
  raw, rendered, and escaped output and state which representation was
  announced and exported.
- Every launch-critical shell packet MUST attach accessibility-tree
  captures for the shell surfaces named in the checklist. A visible
  host-owned control missing from the tree is a correctness failure, not
  an optimization note.

## Shared header

Use the shared evidence header from
`schemas/governance/evidence_packet_header.schema.json`.

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.accessibility.shell_input.<scope>
evidence_id: evidence.accessibility.shell_input.<scope>
title: Accessibility and IME packet for <scope>
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - A11Y-CORE-002
    - A11Y-SR-003
    - A11Y-TEXT-004
    - A11Y-I18N-007
    - A11Y-INCL-005
  claim_row_refs:
    - at.voiceover.macos.core_shell
    - at.nvda.windows.core_shell
    - at.orca.linux.gnome.core_shell
  covered_lanes:
    - accessibility_input_review
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-22T00:00:00Z
  stale_after: P14D
  freshness_class: warm_cached
  source_revision: commit:<sha-or-doc-revision>
  trigger_revision: accessibility_seed@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Review packet over claimed desktop profile rows, launch-critical
    shell surfaces, and seeded IME or text cases.
artifact_links:
  supporting_evidence_ids:
    - evidence.accessibility.platform_input_matrix.seed
    - evidence.accessibility.shell_conformance.seed
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/accessibility/ime_and_text_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/accessibility/a11y_ime_packet_template.md
    - artifacts/accessibility/platform_input_matrix.yaml
    - artifacts/accessibility/shell_conformance_checklist.yaml
    - artifacts/accessibility/accessibility_tree_coverage_rows.yaml
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Packet sections

### 1. Environment matrix

| Field | Required content |
|---|---|
| Platform profile | exact `profile_id` from `artifacts/platform/claimed_desktop_profiles.yaml` |
| Assistive technology | exact AT tool and version, or `keyboard_only` when no AT was active |
| Accessibility bridge | `nsaccessibility`, `uia`, or `at_spi` |
| Locale and direction | locale tag, directionality, and whether pseudoloc or RTL stress mode was active |
| Input method / layout | IME name, keyboard layout, dead-key or AltGr posture, emoji path if exercised |
| Theme and visibility | light/dark/high-contrast state, zoom %, text scale if separate |
| Motion | reduced-motion on/off |
| Display topology | single monitor, mixed DPI, moved window, detached display, or wake/reconnect note |

### 2. Coverage table

| Surface row | Checklist id | Platform or AT rows | Input or locale rows | Tree rows | Fixture refs | Result | Notes |
|---|---|---|---|---|---|---|---|
| `open_repo_from_launcher` | `shell_conformance.open_repo_from_launcher` | `<at-row>` | `<input-row>` | `<tree-row>` | `<fixture>` | `passed` | `<narrowing or blocker>` |
| `quick_open_palette` | `shell_conformance.quick_open_palette` | ... | ... | ... | ... | `degraded` | `<why>` |

Rules:

- Do not collapse multiple platform rows into one generic "desktop"
  result.
- If one AT row passes and another degrades, split them into separate
  coverage rows.
- If a row is `unclaimed`, say why it remains outside the defended
  matrix instead of leaving it blank.

### 3. Keyboard, focus, and announcement review

| Surface row | Keyboard path | Focus order and return | Announcements and labels | Shortcut narration | Result | Failure class if not passed |
|---|---|---|---|---|---|---|
| `<surface-row>` | `<step summary>` | `<return target or fallback target>` | `<label, live-region, blocked-reason summary>` | `<narration rule>` | `passed` | `none` |

Minimum content per row:

- full keyboard route from entry to action and back;
- the expected focus-return target and fallback target if the invoker
  disappears;
- screen-reader label or live-region behavior for state changes and
  blocked reasons;
- any shortcut narration or discoverability cue that must remain
  reachable without hover.

### 4. Zoom, motion, and contrast review

| Surface row | Zoom stability | Reduced motion | High contrast | Result | Notes |
|---|---|---|---|---|---|
| `<surface-row>` | `stable at 200%` | `animations suppressed` | `labels and focus visible` | `passed` | `<narrowing or blocker>` |

Minimum checks:

- 200% zoom or equivalent high-scale row does not hide primary state,
  recovery actions, or focus rings;
- reduced motion suppresses non-essential movement without hiding state
  changes;
- high-contrast rows keep text, focus, selection, blocked reasons, and
  trust labels legible without color-only cues.

### 5. Input fidelity and copy-parity review

| Surface row | Input path | What was exercised | Result | Copy parity / text truth | Failure class if not passed |
|---|---|---|---|---|---|
| `<surface-row>` | `TSF/IME` | `preedit, commit, AltGr, dead key` | `passed` | `raw/rendered/escaped identical where required` | `none` |

Minimum checks:

- IME preedit remains visible through focus churn, filtering, and
  overlay transitions;
- dead keys, compose sequences, and AltGr remain text production rather
  than shortcut dispatch;
- bidi, grapheme, and emoji commits preserve exact Unicode text;
- raw, rendered, and escaped copy routes preserve the representation the
  user selected and do not silently normalize away directional or
  invisible controls.

### 6. Accessibility-tree capture review

| Tree row | Capture moment | Focus owner | Required named regions / roles | Missing nodes | Result | Capture refs |
|---|---|---|---|---|---|---|
| `<tree-row>` | `primary_text_input_focus` | `<role or control>` | `<summary>` | `none` | `passed` | `<capture-id or file>` |

Minimum capture points:

- initial shell render;
- primary interactive control focused;
- blocked or degraded state visible;
- post-action summary for long-running or review flows.

### 7. Gaps, waivers, and narrowing actions

For every gap:

- **State:** `degraded`, `blocked`, `failed`, or `unclaimed`
- **Failure class:** one of the classes from the shell checklist
- **User-visible posture:** the exact banner, chip, blocked action, or
  narrowed route the user sees
- **Narrowing action:** claim row narrowed, packet held, or follow-up
  evidence required
- **Recovery path:** repair, fallback, retry, export, or continue-local
  action

### 8. Reviewer signoff and refresh trigger

- **Reviewer / forum:** `@handle` or forum id
- **Decision:** `accept` | `reject` | `needs_follow_up` | `waived`
- **Reviewed row refs:** checklist ids, AT row refs, input row refs, or
  tree row refs
- **Refresh trigger:** prefer a trigger id from
  `artifacts/governance/evidence_rerun_triggers.yaml`
- **Expected freshness window:** default `P14D` for active shell work
  unless a stricter release gate applies

## Launch-critical shell surface roster

Every full packet should cover these surfaces unless the packet scope is
explicitly narrower:

1. open repo from launcher
2. quick open or palette
3. read docs
4. guided tour or onboarding action
5. task run
6. diff review
7. sign-in or trust prompt
8. terminal-output flow
9. dense collection or batch-review flow
