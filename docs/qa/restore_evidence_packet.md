# Restore qualification corpus and recovery-evidence packet

This document is the narrative companion to
[`/artifacts/qa/restore_qualification_matrix.yaml`](../../artifacts/qa/restore_qualification_matrix.yaml).
It seeds a single qualification corpus that proves recovery is a
**governed evidence lane** — not a collection of ad-hoc restart
demos — by binding every recovering launch to one inspectable
recovery-evidence packet that downstream milestone, release,
support, and channel-widening reviews can attach without
reinterpretation.

If this document and the YAML disagree, the YAML wins for tooling
and this document must update in the same change.

## Companion artifacts

- [`/artifacts/qa/restore_qualification_matrix.yaml`](../../artifacts/qa/restore_qualification_matrix.yaml)
  — machine-readable drill rows, recovery-evidence packet shape,
  drill map by evidence class, claimed-profile-scoped rerun
  expectations, and cadence guidance.
- [`/fixtures/recovery/qualification_cases/`](../../fixtures/recovery/qualification_cases/)
  — reviewable qualification fixtures the matrix drills point at.
- [`/artifacts/qa/window_display_matrix.yaml`](../../artifacts/qa/window_display_matrix.yaml)
  and
  [`/docs/qa/multi_window_verification.md`](./multi_window_verification.md)
  — the window/topology QA seed; the topology drill in this corpus
  is the qualification companion to the window-display matrix's
  display-detach, mixed-DPI, and missing-dependency rows.
- [`/artifacts/qa/provider_handoff_parity_suite.yaml`](../../artifacts/qa/provider_handoff_parity_suite.yaml)
  — the provider-acting-identity / browser-handoff QA seed; the
  expired-remote-session drill in this corpus is the qualification
  companion to that suite's revoked-grant and host-mismatch
  packet drills when the expired authority is provider-bound.

## Normative sources projected here

- [`/docs/recovery/restore_chooser_contract.md`](../recovery/restore_chooser_contract.md)
  and
  [`/schemas/recovery/recovery_level.schema.json`](../../schemas/recovery/recovery_level.schema.json)
  / [`/schemas/recovery/restore_chooser_state.schema.json`](../../schemas/recovery/restore_chooser_state.schema.json)
  — closed five-class progressive recovery-level vocabulary,
  selection criteria, risk classes, action set, and remembered-
  choice expiry vocabulary the corpus reuses.
- [`/docs/recovery/checkpoint_inspector_contract.md`](../recovery/checkpoint_inspector_contract.md)
  and
  [`/schemas/recovery/checkpoint_inventory.schema.json`](../../schemas/recovery/checkpoint_inventory.schema.json)
  — inventory item / scope / effect-breadth / control vocabularies
  the recovery-evidence packet projects as retained-artifact
  classes.
- [`/docs/reliability/export_before_reset_contract.md`](../reliability/export_before_reset_contract.md)
  and the export-before-reset checklist / verification-result
  schemas — the typed reset-gate references the packet cites
  whenever a destructive action is in scope.
- [`/docs/reliability/recovery_scenario_contract.md`](../reliability/recovery_scenario_contract.md)
  and
  [`/artifacts/recovery/safe_first_action_matrix.yaml`](../../artifacts/recovery/safe_first_action_matrix.yaml)
  — recovery-scenario family, safe-remainder, first-action
  vocabulary the failed-update and failed-import-or-migration
  drills cite verbatim.
- [`/docs/reliability/autosave_journal_and_guided_replay_contract.md`](../reliability/autosave_journal_and_guided_replay_contract.md)
  — autosave compare-to-disk sheet the dirty-buffer-replay drill
  cites as its compare-before-discard hook.
- [`/docs/reliability/local_history_restore_preview_contract.md`](../reliability/local_history_restore_preview_contract.md)
  — local-history restore preview the corpus cites as a compare
  surface where applicable.
- [`/docs/reliability/corruption_rescue_compare_contract.md`](../reliability/corruption_rescue_compare_contract.md)
  — corruption-rescue compare sheet the failed-update drill cites.
- [`/docs/recovery/restore_hydration_phases_contract.md`](../recovery/restore_hydration_phases_contract.md)
  — hydration-phase event the packet links by opaque ref so
  shell-ready, search-ready, and live-rebind cues are attached to
  the qualification record, not duplicated.
- [`/docs/recovery/collab_restore_and_presentation_contract.md`](../recovery/collab_restore_and_presentation_contract.md)
  — restored-collaboration record the expired-remote-session
  drill cites for restored role badges and follow targets.
- [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
  — the named claimed desktop profiles every drill must run on.

## What this seed freezes

- one stable drill-class family for crash recovery, dirty-buffer
  replay, failed update, failed import or migration, schema-skew
  cross-version restore, monitor-topology change, missing
  extension host, expired remote session, and evidence-only
  fallback;
- one **recovery-evidence packet** shape every recovering launch
  emits (recovery level, retained classes, discarded classes,
  action chosen, compare-before-discard hook, export-before-reset
  linkage, placeholder/downgrade note, upstream record refs,
  rerun expectation, parity expectations proved);
- one **drill map by evidence class** so crash, update, migration,
  and cross-version restore stay distinct and never collapse into
  a single "continuity" claim;
- one **owner and rerun-expectation contract** so any new
  protected surface or state-artifact family change reruns the
  affected drills on every claimed desktop profile before the
  evidence carries forward;
- one cadence posture: targeted validation on affected drills for
  recovery-vocabulary or protected-surface changes, and full
  claimed-profile validation before release candidates widen
  desktop or recovery claims.

This seed does **not** implement an automated end-to-end UI replay
across every claimed desktop profile. It freezes the reviewable
corpus and fixture set future harnesses, RC packets, support
exports, and channel-widening reviews will cite.

## Why freeze this now

A recovery surface that is tested as a collection of demos
produces these non-conforming patterns:

- **Continuity collapse.** Crash recovery, a failed update
  rollback, a failed migration, and a schema-skew compatible
  translation all get filed under "the workspace reopened". The
  reviewer cannot tell whether a destructive action ran.
- **Silent omission.** A drill records what was retained but not
  what was discarded; reviewers infer "nothing was lost" from the
  absence of a class. The recovery-evidence packet refuses this
  by requiring an explicit `nothing_discarded_explicit` sentinel.
- **Untyped compare hook.** A reviewer cannot tell whether the
  user had a compare surface (autosave compare-to-disk sheet,
  corruption-rescue compare, restore-destination review,
  placeholder inspect-only) before any destructive action ran.
- **Untyped reset gate.** Failed-update and failed-migration
  drills run without naming the export-before-reset checklist
  reference and verification result; the packet cannot route to
  the upstream gate later.
- **Drift under protected-surface changes.** A new restore-
  artifact-family member or a new claimed desktop profile lands
  and the existing drill set is treated as still passing without
  an explicit rerun decision.
- **Cross-surface reinterpretation.** Milestone, release,
  support, and channel-widening reviews each rewrite the packet
  in their own vocabulary, so the same recovery is documented
  three different ways.

The recovery-evidence packet forecloses these patterns by
projecting one closed recovery-level vocabulary, one closed
retained / discarded artifact-class set, one typed compare-before-
discard hook, one typed export-before-reset linkage, one typed
placeholder-or-downgrade note, and one typed rerun-expectation
class into a record every reviewing surface reads verbatim.

## Recovery-evidence packet — at a glance

The packet is a **typed projection**, not a new record family. It
cites every upstream record by opaque ref so downstream review
surfaces re-attach the evidence without reinterpretation.

| Packet field | Source |
|---|---|
| `chosen_recovery_level_class` | `recovery_level_record.recovery_level_class` (chooser §2). |
| `selection_criterion_class` | `recovery_level_record.selection_criterion_class` (chooser §2.1). |
| `risk_class` / `reversibility_class` | `restore_chooser_state_record.risk_note` (chooser §6). |
| `retained_artifact_classes` | Closed set re-exported from chooser retained-evidence anchors and checkpoint-inventory item classes. |
| `discarded_or_deferred_artifact_classes` | Closed set; `nothing_discarded_explicit` sentinel records "no discard". |
| `action_chosen_id` | `chooser_primary_action_record.action_id` (chooser §7), aliasing `restore_now` → `open_evidence` for evidence-only recovery per §7.3 rule 2. |
| `compare_before_discard_hook_class` | Closed set; cites the typed compare surface (autosave compare-to-disk sheet, local-history restore preview, corruption-rescue compare sheet, restore-destination review, etc.). |
| `export_before_reset_linkage_class` | Closed set; either typed-checklist-required (verified / blocked / declined) or explicit not-applicable. |
| `placeholder_or_downgrade_note_class` | Closed set; explicit `no_placeholder_or_downgrade` when neither applies. |
| `claimed_profile_id` | One value from [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml). |
| `upstream_*_ref` | Opaque refs to the chooser, checkpoint inventory, export-before-reset checklist, verification result, recovery-scenario card, and hydration-phase events. Free-text linkage prose is non-conforming. |
| `rerun_expectation_class` | Closed set; declares why the drill must rerun under a typed protected-surface or vocabulary change (see "Owner and rerun expectations" below). |
| `parity_expectations_proved` | Closed set; the truth-statements the packet proves about the drill row. |

## Drill matrix — at a glance

| Drill class | Expected recovery level | Compare hook | Export-before-reset linkage |
|---|---|---|---|
| `crash_recovery_drill` | `exact_session_restore` | `no_compare_required` | `not_applicable_no_destructive_action` |
| `dirty_buffer_replay_drill` | `dirty_buffer_recovery` | `autosave_compare_to_disk_sheet` | `not_applicable_no_destructive_action` |
| `evidence_only_fallback_drill` | `evidence_only_recovery` | `no_compare_required` | `not_applicable_evidence_only_inspect` |
| `failed_update_drill` | `checkpoint_rollback` | `corruption_rescue_compare_sheet` | `typed_checklist_required_and_verified` |
| `failed_import_or_migration_drill` | `checkpoint_rollback` | `restore_destination_review` | `typed_checklist_required_and_verified` |
| `schema_skew_cross_version_drill` | `context_restore_with_placeholders` | `portable_state_export_review` | `not_applicable_no_destructive_action` |
| `monitor_topology_change_drill` | `context_restore_with_placeholders` | `placeholder_inspect_only` | `not_applicable_no_destructive_action` |
| `missing_extension_host_drill` | `context_restore_with_placeholders` | `placeholder_inspect_only` | `not_applicable_no_destructive_action` |
| `expired_remote_session_drill` | `context_restore_with_placeholders` | `placeholder_inspect_only` | `not_applicable_no_destructive_action` |

## Drill map by evidence class

Crash, update, migration, and cross-version restore stay **distinct
evidence classes** with explicit drill coverage. A reviewer reading
the packet learns which class fired before learning what changed.

| Evidence class | Drills | What this lane proves |
|---|---|---|
| Crash recovery | `crash_recovery_exact_session_qualification`, `crash_recovery_dirty_buffer_replay_qualification`, `crash_recovery_evidence_only_fallback_qualification` | A crash-recovering launch is exact, journal-replay, or evidence-only — never silently a full reset. The dirty-buffer journal is not a save. The evidence-only fallback never claims rehydrated runtime. |
| Failed update | `failed_update_rollback_after_export_qualification` | A failed update routes through the export-before-reset checklist and the verification result before the rollback path runs. The corruption-rescue compare sheet is reachable; the quarantined-copy record is preserved. |
| Failed import or migration | `failed_import_or_migration_qualification` | A failed import / migration is distinct from a failed update and from cross-version schema skew. Failed dry-run artifacts and incompatible translation targets are surfaced as discarded-or-deferred classes, not silently dropped. |
| Cross-version / schema skew | `schema_skew_compatible_translation_qualification` | Schema skew is qualified separately from a fresh crash. The chooser refuses `claim_pixel_perfect_layout` and the compatible-translation note is preserved on the packet verbatim. |
| Topology change | `monitor_topology_change_layout_only_qualification` | Monitor topology change rides its own drill — the topology-adjustment note is visible on the packet, the stale window-topology snapshot is named on the discarded classes, and the row reruns on every claimed desktop profile. |
| Missing dependency | `missing_extension_host_placeholder_qualification`, `expired_remote_session_placeholder_qualification` | Missing extension host and expired remote session ride distinct drill classes even though both produce `context_restore_with_placeholders`. Placeholder pane slots are retained; live authority is never silently rebound. |

## Owner and rerun expectations

The corpus is owned by `qa.recovery`, with reviewers from
`reliability.recovery_chooser`, `state.restore_artifact_family`, and
`support.recovery_evidence`. Every drill row carries a typed
`rerun_expectation_class` so a reviewer reading the packet later
can tell why the drill ran (or did not need to) on the affected
claimed profile.

The contract — frozen verbatim in
[`/artifacts/qa/restore_qualification_matrix.yaml`](../../artifacts/qa/restore_qualification_matrix.yaml)
under `rerun_expectation_contract` — names the rerun expectations
that bind:

- **Closed-vocabulary extension** to any of `recovery_level_class`,
  `selection_criterion_class`, `risk_class`, `expiry_trigger_class`,
  `inventory_item_class`, `effect_breadth_class`, `artifact_class`,
  or `verification_result_class` reruns every drill that touches
  the affected vocabulary, on every claimed profile.
- **New protected surface or state-artifact family** (a new
  restore-artifact-family member, a new recovery-scenario family,
  or a new collab-restore kind) reruns every drill that touches
  the affected scope, on every claimed profile.
- **New claimed desktop profile** in
  [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
  reruns every drill once on the new profile before the claim
  status widens.
- **Doc-only narrowing or wording changes** that do not extend a
  closed vocabulary may declare
  `rerun_optional_narrow_doc_only_change` and ride the existing
  fixture on the originally claimed profiles.
- **Planning-metadata changes** (milestone slugs, schedule notes,
  ownership rotations) declare
  `rerun_not_required_planning_metadata_only_change`. They MUST
  NOT be used to skip a rerun forced by any of the rules above.

## Cadence

| Cadence | What runs |
|---|---|
| Targeted change validation | Affected drills run on the affected claimed profiles whenever the restore chooser, recovery-level vocabulary, checkpoint inventory, export-before-reset checklist, autosave journal, local-history restore preview, corruption-rescue compare sheet, restored-collaboration record, hydration-phase event, or recovery-scenario card changes. |
| Release-candidate validation | Every drill exercises a worked qualification fixture or cited sibling fixture on every claimed profile before the desktop or recovery claim widens or ships. The recovery-evidence packet is the artifact ship-readiness reviews cite — not a fresh checklist minted in the release lane. |

## Acceptance

- **Crash, update, migration, and cross-version restore stay
  distinct evidence classes** with explicit drill coverage.
  `drill_map_by_evidence_class` in the matrix locks the lane
  separation; collapsing two lanes into one is non-conforming.
- **Recovery-evidence packets attach to later reviews without
  reinterpretation.** Milestone, release, support, and channel-
  widening reviews cite the packet's typed fields verbatim and
  read upstream records by opaque ref; they do not rewrite the
  packet in their own vocabulary.
- **The corpus maps cleanly to the restore chooser, restore
  artifact family, and export-before-reset controls already
  frozen.** Every drill cites the chooser's
  `recovery_level_class` / `selection_criterion_class` / closed
  action set, the checkpoint-inventory inventory and control
  vocabulary, and the export-before-reset checklist /
  verification-result vocabulary; no drill mints a parallel
  vocabulary.

## Out of scope

- Automated end-to-end UI replay of every drill on every claimed
  desktop profile. This seed freezes the reviewable corpus and
  fixture set future harnesses cite; the harness itself is not
  delivered here.
- A new chooser, inventory, checklist, hydration-phase, or
  scenario-card record shape. The packet is a typed projection
  of the upstream records, not a new record family.
- Final user-facing copy / microcopy or visual layout — those are
  pinned by the UX style guide and the surface-specific contract
  (Start Center, crash-loop screen, diagnostics panel, support-
  export preview, docs/help example).
