# Save-participant and fix-safety contract

This contract freezes the save hot path for formatters, fixers,
generators, validation passes, and AI-assisted participants. It narrows
the broader quality-profile and on-save orchestration contract into the
record shape every save participant must use before it can inspect,
mutate, skip, block, or refresh work.

The goal is simple: a save may be fast, but it may not be surprising.
Every participant has a phase, a declared effect ceiling, a fix-safety
class, an output origin, a compare-before-write posture, and a
checkpoint or rollback posture. If runtime behavior exceeds the plan,
the save moves to review or blocks; it does not silently widen scope.

Machine-readable companions:

- [`/schemas/execution/save_participant_plan.schema.json`](../../schemas/execution/save_participant_plan.schema.json)
  - `save_participant_plan_record`,
  `save_participant_result_record`,
  `save_participant_review_record`, and
  `save_participant_execution_event_record`.
- [`/schemas/execution/fix_safety_class.schema.json`](../../schemas/execution/fix_safety_class.schema.json)
  - `fix_safety_class_definition_record` and
  `fix_safety_evaluation_record`.
- [`/fixtures/execution/save_participant_cases/`](../../fixtures/execution/save_participant_cases/)
  - worked YAML fixtures covering safe format-on-save, generated
  companion update, blocked whole-file rewrite, external change during
  save, and preview-required multi-file fix.

This contract composes with:

- [`/docs/execution/quality_profile_and_on_save_contract.md`](./quality_profile_and_on_save_contract.md)
  for effective profile resolution, participant ordering from profiles,
  scanner import, and quality-result deltas.
- [`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json)
  for canonical save-target identity and compare-before-write tokens.
- [`/docs/verification/source_fidelity_and_undo_packet.md`](../verification/source_fidelity_and_undo_packet.md)
  for source-fidelity fields, rewrite-scope labels, and recovery-honesty
  labels.
- [`/docs/ux/editor_external_change_contract.md`](../ux/editor_external_change_contract.md)
  for stale buffers, compare-required state, save-blocked state, and
  external-change review choices.
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  and [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  for generated-artifact lineage, canonical-source boundaries, and
  regeneration posture.
- [`/docs/reliability/local_history_contract.md`](../reliability/local_history_contract.md)
  and [`/docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md)
  for checkpoint, undo-group, local-history, and mutation-lineage
  vocabulary.

## Scope

Frozen here:

- save-participant phases and their allowed ordering;
- which phases may mutate staged content, durable files, or generated
  companions;
- fix-safety classes used by formatters, fixes, source actions,
  generators, and AI-assisted save participants;
- no-hidden-mutation rules for participant visibility, skipped-state
  visibility, output-origin disclosure, and scope ceilings;
- review triggers for whole-file rewrites, generated-artifact impacts,
  multi-file edits, external-change races, and policy or trust
  narrowing; and
- checkpoint, rollback, compare-before-write, and post-save refresh
  hooks shared by UI, CLI/headless, review, support, and diagnostics.

Out of scope:

- implementing any formatter, linter, code-action provider, generator,
  or AI adapter;
- choosing default language tools; and
- replacing the quality-profile, diagnostic, generated-artifact,
  filesystem, local-history, or external-change contracts.

## 1. Save-participant phases

A save-participant plan is ordered by phase first, then by participant
order within the phase. A participant may run only in its declared phase.
A workspace profile may reorder participants within a phase only when
the effective quality profile exposes the order. A participant may not
move itself earlier or later at runtime because a provider is faster,
slower, missing, or stale.

| Phase | Purpose | Mutation allowance | Required hooks |
|---|---|---|---|
| `preflight` | Bind staged buffer, canonical save target, effective profile, trust/policy state, and participant list. | No mutation. | Compare-before-write token, base revision, participant visibility list, rollback policy summary. |
| `format_fix` | Run admitted formatters, organize-imports, local lint fixes, source actions, or reviewed AI apply against staged content. | May mutate only the staged buffer and only inside declared safety/scope. | Participant result, output origin, effect counts, source-fidelity rewrite class, checkpoint hook. |
| `generated_artifact_update` | Refresh declared generated companions that are downstream of the staged canonical source. | May mutate generated outputs only when lineage, generator identity, and review posture are declared. | Generated lineage refs, regeneration/checkpoint refs, preview or review record. |
| `validation` | Verify staged output after mutation. | Read-only unless a separate mutating participant was admitted in an earlier phase. | Validation result, stale/partial reason, rollback or hold decision on failure. |
| `compare_before_write` | Revalidate canonical target identity and observed revision immediately before durable write. | No content mutation. | Current save-target token, external-change event ref on mismatch, rebase/abort/review event. |
| `durable_write` | Commit the staged bytes or approved generated updates. | Writes only the targets already admitted by the plan. | Save manifest, source-fidelity packet fields, mutation group, local-history group. |
| `post_save_indexing_refresh` | Refresh indexes, diagnostics, generated-state projections, search, graph, and UI subscriptions after durable commit. | Read-only with respect to authored and generated workspace files. | Refresh result, stale/degraded labels, support/export event refs. |

Ordering rules:

1. `preflight` is first and never mutates.
2. `format_fix` runs before generated companion updates so generated
   outputs are based on the final staged canonical source.
3. `generated_artifact_update` may be skipped when no generated
   companion is declared; if it runs, it precedes validation.
4. `validation` runs after all staged and generated mutations.
5. `compare_before_write` runs after validation and immediately before
   `durable_write`. Any external-change mismatch stops the write path
   until rebase, abort, or review resolves.
6. `durable_write` commits only the targets already admitted by earlier
   phases. It cannot discover new targets.
7. `post_save_indexing_refresh` runs only after a committed durable
   write or an explicitly recorded no-op save. It may refresh state but
   may not edit files.

Checkpoint and rollback rules:

1. A participant that can change bytes names a checkpoint posture before
   it runs. Multi-file, generated, whole-file rewrite, policy-scoped,
   provider-dependent, AI-assisted, or unknown mutations require a
   checkpoint or a metadata-only checkpoint when body capture is blocked.
2. A save and its participants remain one attributable mutation group
   where possible. Participant edits do not vanish into ordinary typing
   history.
3. Rollback labels match the real recovery route: exact undo,
   compensating revert, restore checkpoint, regenerate/recompute,
   audit-only, or no state change.
4. A failed participant may be skipped only when the plan declared a
   skip policy and the resulting save guarantee remains visible. It may
   not silently downgrade to an unsafe write.

## 2. Fix-safety classes

Every mutating or potentially mutating participant evaluates to one
fix-safety class. The class is visible before apply and exported with
the result.

| Class | Meaning | Save posture |
|---|---|---|
| `safe_local_text_edit` | Deterministic local text edit bounded to the visible file or a declared range, with no generated/protected impact and no whole-file rewrite. | May auto-apply on save when compare-before-write stays current and the output origin is deterministic. |
| `whole_file_rewrite_disclosed` | The participant rewrites the full document on its supported path or falls back from targeted patching to a full-file rewrite. | Must disclose the whole-file rewrite label; preview is required when thresholds, file class, or policy demand it. |
| `generated_companion_update` | The participant updates generated output downstream of a canonical source or generator. | Requires generated lineage, regeneration posture, checkpoint/rollback route, and generated-artifact preview or review unless policy has an explicit safe lane. |
| `workspace_wide_preview_required` | The participant may touch multiple files, files outside the visible target, created/deleted files, or workspace-wide semantic state. | Cannot auto-apply on save; opens batch preview or typed review. |
| `external_change_conflict_requires_review` | The on-disk target changed, became uncertain, or lost conditional-write authority after the plan opened. | Blocks durable write until compare/rebase/merge/retry/cancel resolves through the external-change contract. |
| `policy_blocked` | Policy, trust, read-only root, generated-posture policy, or profile lock denies the mutation. | No mutation. Emit blocked outcome, policy/trust refs, and support-safe summary. |
| `fix_safety_unknown_requires_review` | The participant cannot prove one of the classes above. | Treat as review-required and no durable write until classified. |

Class rules:

1. A participant may only move to a stricter class at runtime. It may
   not relax from preview-required or blocked into safe local apply.
2. Whole-file rewrite disclosure is orthogonal to file count. A
   one-file rewrite can still require review.
3. Generated companion updates are not ordinary source edits. The plan
   names canonical inputs, generator identity, lineage refs, drift state,
   and regeneration or rollback route.
4. External-change conflicts are not formatter failures. They are save
   authority failures and route through compare/rebase/abort/review.
5. Policy-blocked participants do not run in "best effort" mode unless
   policy explicitly defines a read-only inspection fallback.

## 3. No-hidden-mutation rules

The user, CLI caller, support export, and review surface must be able to
answer four questions without provider-specific documentation:

1. Which participants were planned?
2. Which participants ran, skipped, timed out, blocked, or failed?
3. Which files or generated artifacts could be touched?
4. Did the output come from an exact rule, imported config, heuristic
   fallback, generator lineage, policy decision, or AI suggestion?

Every plan and result therefore records an `output_origin_class`:

| Origin | Meaning | Save limitation |
|---|---|---|
| `exact_rule` | A named exact rule, source action, or formatter rule produced the output. | May auto-apply only within the fix-safety ceiling. |
| `imported_config` | A checked-in or imported tool config selected the behavior. | Mapping notes and winning source refs stay visible. |
| `heuristic_fallback` | Aureline used a fallback because exact provider/config proof was unavailable. | Safe local edits may show inline summary; broad, generated, whole-file, or semantic changes require review. |
| `generated_lineage` | Output came from a declared generator or generated-artifact lineage record. | Requires generated companion update posture. |
| `policy_decision` | Output or blocking came from policy/trust/profile authority. | Cannot be hidden behind a generic formatter or fixer label. |
| `ai_suggestion` | Output came from an AI plan or suggestion. | Never runs as a hidden on-save participant. It must cite reviewed plan/ticket/rollback refs. |
| `read_only_validation` | Participant observed, scanned, or validated without mutation. | May not write during the save path. |

No-hidden-mutation guard rules:

1. The participant list is visible before the first mutating phase.
2. Skipped, timed-out, blocked, degraded, and failed participants emit
   explicit result rows. Absence of a result row is a contract violation.
3. Declared file counts, generated/protected flags, whole-file rewrite
   flags, and outside-visible-file flags are compared with actual
   results. If actual effects exceed the declaration, the participant
   resolves to review, rebase-required, blocked, or failed.
4. Scanner and validation participants are read-only unless represented
   as separate mutating participants with a fix-safety class.
5. Post-save refresh and indexing may update indexes, caches, and
   materialized views, but may not mutate authored files or generated
   companions.
6. A provider may not hide additional edits inside a formatter result.
   Import insertion, companion generation, manifest updates, lockfile
   writes, policy-file writes, or sibling-file edits are declared as
   additional effects and reviewed when required.
7. CLI/headless output uses the same plan ids, participant ids, phase
   classes, fix-safety classes, output origins, and review triggers as
   desktop UI.

## 4. Review triggers

The following triggers always create a review record or block the write
until an existing reviewed ticket is cited:

| Trigger | Required behavior |
|---|---|
| `whole_file_rewrite` | Disclose whole-file rewrite or fallback; require preview when thresholds, large-file mode, generated/protected posture, or policy says review. |
| `generated_artifact_impact` | Name generated lineage, canonical source, generator/refresher identity, drift state, and regenerate/rollback route. |
| `multi_file_edit` | Open batch preview or typed review for files outside the current visible target, created/deleted files, or workspace-wide fixes. |
| `external_change_race` | Stop before durable write; route through compare/rebase/abort/review with current save-target tokens. |
| `policy_or_trust_narrowing` | Block or narrow the participant with policy/trust refs and a visible reason. No background bypass. |
| `provider_dependency_or_semantic_freshness_gap` | Require review when semantic state, provider scope, or imported evidence cannot prove current safe mutation. |
| `source_fidelity_conversion` | Require explicit user or policy action for encoding, BOM, newline, final-newline, or permission changes. |
| `output_origin_heuristic_or_ai` | Require review for heuristic or AI output unless the final mutation remains `safe_local_text_edit` and the plan carries an inline summary. |

Review records carry:

- affected participant refs;
- fix-safety class;
- review trigger class;
- declared and actual file-effect summaries when available;
- generated lineage, external-change, policy, diagnostic, or issue refs;
- checkpoint and rollback refs; and
- review surface or ticket refs.

## 5. Safe-save and blocked-save distinctions

A safe save is one where every mutating participant is
`safe_local_text_edit`, every participant result is present, every
actual effect stays inside the declared plan, compare-before-write
passes, and durable write preserves source-fidelity policy.

A save is preview-required when any participant is
`whole_file_rewrite_disclosed`, `generated_companion_update`, or
`workspace_wide_preview_required` and the review trigger is not already
resolved by a current review ticket.

A save is blocked when any participant is `policy_blocked`,
`external_change_conflict_requires_review`, or
`fix_safety_unknown_requires_review`, or when compare-before-write
cannot prove current write authority.

These distinctions must be visible in:

- editor save status and participant inspector;
- CLI/headless structured output;
- review sheets and batch previews;
- support/export packets; and
- hot-path instrumentation and diagnostics.

## 6. Fixture coverage

The companion fixture set anchors the minimum review corpus:

| Fixture | Primary class | Coverage |
|---|---|---|
| `safe_format_on_save.yaml` | `safe_local_text_edit` | One-file formatter, deterministic imported config, validation, compare-before-write, durable write, and post-save refresh. |
| `generated_companion_update.yaml` | `generated_companion_update` | Canonical source save triggers generated companion update with lineage, checkpoint, and preview posture. |
| `blocked_whole_file_rewrite.yaml` | `whole_file_rewrite_disclosed` | Formatter wants whole-file rewrite but policy/large-file posture blocks durable write pending review. |
| `external_change_mid_save.yaml` | `external_change_conflict_requires_review` | Compare-before-write detects a newer external revision after participants ran. |
| `preview_required_multi_file_fix.yaml` | `workspace_wide_preview_required` | Lint fix may touch multiple workspace files and therefore opens batch preview. |

## Change management

Adding a new phase, fix-safety class, output-origin class, review trigger,
participant class, checkpoint posture, run-state class, or rewrite class
is additive-minor only when the new value cannot be represented by an
existing value and the schemas, docs, and fixtures update together.

Repurposing an existing class is breaking. Existing support packets,
review records, saved participant histories, and release evidence may
already cite these values.
