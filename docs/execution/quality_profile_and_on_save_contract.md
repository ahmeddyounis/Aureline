# Quality-profile and on-save orchestration contract

This document freezes how Aureline resolves quality tooling profiles and
how save-time quality participants are allowed to inspect, rewrite, or
block a file. It exists so formatting, linting, scanner imports,
organize-imports, fix-all actions, local CI, managed CI, CLI runs,
release packets, and support exports share one answer to:

- which profile and tool version was selected;
- which repo, workspace, user/profile, imported, provider, environment,
  or policy source won;
- why a profile was locked, narrowed, downgraded, or unavailable;
- which save participants ran, in what order, and against which staged
  buffer;
- which participant may mutate files, which may only scan, and which
  must open review;
- whether local, imported, remote, or provider-authoritative findings
  are comparable; and
- which deltas can be exported without inspecting raw logs.

Machine-readable companions:

- [`/schemas/execution/quality_profile.schema.json`](../../schemas/execution/quality_profile.schema.json)
  - `quality_profile_source_record`,
  `effective_quality_profile_record`,
  `quality_profile_resolution_event_record`,
  `scanner_import_session_record`, and
  `quality_result_delta_record`.
- [`/schemas/execution/on_save_plan.schema.json`](../../schemas/execution/on_save_plan.schema.json)
  - `on_save_plan_record`,
  `on_save_participant_result_record`,
  `additional_edit_review_record`, and
  `on_save_execution_event_record`.
- [`/schemas/execution/save_participant_plan.schema.json`](../../schemas/execution/save_participant_plan.schema.json)
  and
  [`/schemas/execution/fix_safety_class.schema.json`](../../schemas/execution/fix_safety_class.schema.json)
  - hot-path save-participant phases, fix-safety classes,
  no-hidden-mutation guards, whole-file rewrite disclosure,
  generated companion update posture, external-change conflict
  handling, and review triggers.
- [`/fixtures/execution/quality_profile_cases/`](../../fixtures/execution/quality_profile_cases/)
  - worked YAML fixtures covering locked profile selection, imported
  config downgrade, scanner import delta comparison, safe on-save apply,
  and review-required additional edits.

This contract composes with and does not replace:

- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  and its schemas for diagnostic clustering, code-action summaries, and
  suppression governance. Quality profiles name which tools and rule
  packs produce findings; diagnostic records preserve the finding truth.
- [`/docs/language/diagnostic_freshness_and_delta_contract.md`](../language/diagnostic_freshness_and_delta_contract.md)
  for finding-level remap state, SARIF-like/provider import records, and
  diagnostic delta packets that compare current, imported, baseline,
  stale, suppressed, and support-exported findings.
- [`/docs/execution/run_and_attempt_contract.md`](./run_and_attempt_contract.md)
  for run, attempt, outcome, artifact-event, and rerun lineage. A
  manual format/lint run, local CI quality run, or managed CI quality
  run is still a run/attempt; this contract adds profile and delta
  identity.
- [`/docs/settings/settings_vocabulary.md`](../settings/settings_vocabulary.md)
  and ADR-0008 for effective setting source chains. Quality-profile
  precedence reuses the same source-label and policy-ceiling discipline.
- [`/docs/execution/save_participant_and_fix_safety_contract.md`](./save_participant_and_fix_safety_contract.md)
  for the narrower save hot-path phase order, fix-safety classes,
  no-hidden-mutation rules, and whole-file/generated/multi-file/
  external-change review triggers.
- [`/docs/admin/policy_explainability_contract.md`](../admin/policy_explainability_contract.md)
  for policy locks, effective-state labels, and escalation routes.
- [`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json)
  for canonical save-target identity and compare-before-write tokens.
- [`/docs/reliability/local_history_contract.md`](../reliability/local_history_contract.md)
  and the mutation-lineage fixtures for checkpoint, rollback, and
  grouped mutation language.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  and
  [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  for generated/protected artifact disclosure.
- `.t2/docs/Aureline_PRD.md`,
  `.t2/docs/Aureline_Technical_Architecture_Document.md`,
  `.t2/docs/Aureline_Technical_Design_Document.md`, and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those sources
  disagree with this document, those sources win and this contract plus
  the companion schemas update in the same change.

## Why freeze this now

Formatter and linter behavior often becomes product debt because the
happy path feels small: a user saves a file and whitespace changes. In a
real workspace, the same action can depend on imported `.editorconfig`
state, tool-native config files, workspace defaults, a profile import,
an admin lock, a container image, a remote provider, a scanner baseline,
or a managed CI rule pack. If those layers stay implicit, the user only
learns about drift after local output disagrees with CI or after an
on-save participant rewrites more files than expected.

This contract makes quality tooling explicit before implementation
locks in accidental behavior. The profile resolver emits a source chain
and an effective profile. Scanner imports emit import and delta records.
Save-time orchestration emits a plan before mutation and participant
results after execution. High-impact edits are typed review objects, not
hidden provider behavior.

## Scope

Frozen at this revision:

- the effective quality-profile record family and its precedence stack;
- provider/tool identity fields for formatter, linter, organize-imports,
  scanner, rule-pack, adapter, and invocation fingerprints;
- policy locks, managed/provider-authoritative results, imported
  profile/config state, and environment-condition downgrades;
- scanner import, baseline-family, local-confirmation, and result-delta
  link fields;
- local/imported/effective/remote-provider profile refs and drift
  labels shared by desktop, CLI, local CI, managed CI, release, and
  support surfaces;
- staged-buffer on-save plans with deterministic participant ordering,
  preview thresholds, mutation disclosure, timeout policy, rebase/abort
  policy, validation-after-apply, and checkpoint hooks;
- additional-edit review records for provider-dependent, multi-file,
  generated/protected, or policy-scoped mutations; and
- event records that explain resolution, drift, preview, checkpoint,
  rollback, rebase, commit, and abort outcomes.

Out of scope:

- implementing any concrete formatter, linter, or scanner;
- selecting final default tools for every language;
- defining SARIF itself; and
- replacing diagnostic, suppression, local-history, or run/attempt
  schemas.

## 1. Quality-profile resolution

An effective quality profile is the resolved answer for a scope and
surface. It is not just a settings blob. It carries:

- `precedence_stack` - every contributing source in resolver order;
- `effective_tool_bindings` - the selected formatter, linter, scanner,
  rule pack, adapter, config digest, and invocation digest;
- `profile_drift` - local, imported, effective, and remote/provider
  refs plus a drift class and reason classes;
- `delta_links` - scanner import, baseline, local-confirmation,
  support, release, and result-delta refs; and
- `environment_conditions` - target conditions that affected selection.

### 1.1 Precedence layers

The resolver uses this ordered source vocabulary. A surface may render a
friendlier label, but the exported record preserves the class name:

| Layer | Typical source | Rule |
|---|---|---|
| `policy_lock_or_managed_profile` | admin bundle, regulated profile, managed CI rule pack | May pin, narrow, or disable tools/rules. Cannot be silently overridden locally. |
| `action_local_override` | explicit one-command/session choice | Wins for that invocation only and never persists without review. |
| `repository_quality_config` | checked-in Aureline quality config | First-class repo truth; visible in source chain. |
| `workspace_quality_settings` | workspace settings or workset-specific settings | May narrow scope; cannot widen trust or egress. |
| `workspace_profile_override` | active workspace/profile choice | Visible as a profile source, not a hidden editor preference. |
| `remote_or_container_default` | devcontainer, remote agent, managed workspace default | Must name target and authority; may not impersonate local defaults. |
| `user_or_profile_default` | user settings or local profile default | Portable where the settings contract allows. |
| `imported_tool_config` | `.editorconfig`, formatter/linter native config | Unmapped, incompatible, or policy-overridden keys remain visible. |
| `imported_profile_default` | imported profile artifact | Read as imported state with mapping notes; never widens trust. |
| `environment_condition` | OS, architecture, trust, target, mirror, power state, language/file kind | May downgrade, disable, or select a target-specific tool. |
| `fallback_default` | built-in default | Always named and versioned. |
| `system_auto_detection` | detected tool/config with no stronger source | Lowest-authority candidate; never silently wins over explicit config. |

Each layer resolves to one of:

- `selected_winner`
- `shadowed_by_higher_precedence`
- `policy_overridden`
- `downgraded_incompatible`
- `unavailable_in_environment`
- `rejected_invalid`
- `imported_read_only`
- `detected_default`
- `source_state_unknown_requires_review`

Unknown or rejected layers stay in the stack. A resolver may not drop a
candidate merely because it was inconvenient to explain.

### 1.2 Locks and downgrades

Every locked, constrained, provider-authoritative, read-only imported,
or capability-locked profile names:

- `lock_state_class`;
- `lock_reason_class`;
- `downgrade_reason_class`, when behavior narrowed from the requested
  profile; and
- a short summary safe for settings, CLI, support, and release exports.

Rules:

1. `unlocked` pairs with `lock_reason_class = none`.
2. Policy locks can narrow or pin tools, rule packs, mutation posture,
   or baseline families; they do not widen a weaker lower-layer source.
3. Imported evidence is read-only until a compatible local or
   provider-authoritative session confirms it.
4. Tool absence, version mismatch, rule-pack mismatch, schema drift, or
   target capability loss is a typed downgrade, not a generic failure.
5. Users and support exports must be able to answer why a profile was
   selected, locked, downgraded, or refused without reading raw logs.

### 1.3 Provider and tool identity

Every effective tool binding carries:

- `tool_family_class` - formatter, linter, organize-imports, scanner,
  checker, or policy tool family;
- `provider_kind_class` - first-party adapter, language server,
  extension host, external CLI, managed provider, CI adapter, or scanner
  importer;
- `locality_class` - local, sidecar, remote, container, managed service,
  or imported snapshot;
- `provider_authority_class` - local authoritative, remote target
  authoritative, managed CI authoritative, provider import
  authoritative, imported read-only, or inferred best-effort;
- `support_class`;
- tool id/version, adapter id/version, toolchain digest, rule-pack ref,
  config digest, and invocation digest.

Provider identity is part of result identity. Two profiles that format
the same file differently because one used a language server and the
other used a native formatter are drifted profiles, not equivalent
profiles with noisy output.

## 2. Scanner imports and result deltas

SARIF-like and structured scanner inputs are useful as evidence, not as
proof that the current local buffer is failing. Imported scans emit
`scanner_import_session_record` rows that preserve:

- import format and mapping state;
- source tool identity and rule-pack version;
- scan time and import time;
- effective profile ref used for normalization;
- baseline family ref and compatibility state;
- raw payload ref;
- result state; and
- profile drift and delta links.

Result deltas emit `quality_result_delta_record` rows for
quality-profile linkage and `diagnostic_delta_record` rows for
finding-level parity. Delta claims are valid only when compatibility
checks pass for profile, tool, rule-pack, mapping family, baseline
family, suppression policy, and target scope.

Allowed result states:

| State | Meaning |
|---|---|
| `imported_only` | Evidence exists only in an imported scan. |
| `locally_confirmed` | A compatible local or remote-target run reproduced or validated the finding. |
| `baseline_matched` | Finding persists from a compatible baseline. |
| `delta_new` | Finding was absent from the compatible baseline. |
| `delta_resolved` | Finding was present in the baseline and absent from the candidate. |
| `delta_persisting` | Finding remains present. |
| `suppressed` | Visibility or enforcement was narrowed by a governed suppression. |
| `waived` | A waiver or release-visible exception applies. |
| `unmapped` | The anchor no longer maps cleanly. |
| `compatibility_blocked` | Profile/tool/rule/mapping mismatch blocks comparison. |

Rules:

1. Imported scans may populate Problems, review cards, release packets,
   and support bundles, but they do not unlock silent fix-all or
   auto-apply paths.
2. Local CI, managed CI, provider imports, desktop runs, and support
   replays compare through profile and delta refs, not raw log parsing.
3. A support or release packet that claims quality movement must include
   effective profile refs, tool/rule-pack versions, baseline/delta
   summary, active suppressions/waivers, and parity notes.
4. Compatibility-blocked comparisons must say which axis blocked
   comparison. A blocked comparison may not report false improvement or
   false regression.

## 3. On-save orchestration

On-save behavior is a staged-buffer plan. The plan opens before any
participant mutates content and records:

- the trigger (`explicit_save`, `autosave`, command, or headless save);
- the target scope and canonical save target;
- the effective quality profile;
- execution-context and command-dispatch refs;
- preview thresholds;
- participant order; and
- orchestration guards.

The default participant order is:

1. staged-buffer snapshot and compare-before-write token;
2. trivia-safe formatter;
3. organize-imports or source actions that are admitted for on-save;
4. local-syntax-safe lint autofixes admitted by policy/profile;
5. read-only scanners or validation passes;
6. validation-after-apply;
7. save-target identity check and write;
8. local-history, mutation-journal, and event emission.

Workspaces may configure a different order only through the effective
quality profile, and the resolved order must remain inspectable in the
plan. A surface may not silently reorder participants because one
provider is faster or currently available.

### 3.1 Participant classes

| Participant | Mutating posture |
|---|---|
| `formatter` | May mutate only within its declared safety and scope class. Whole-document rewrites respect preview thresholds. |
| `organize_imports` | May mutate when additional edits are declared and semantic/cross-file scope is review-gated. |
| `linter_autofix` | Same-rule or same-tool fixes must declare counts, scope, safety, checkpoint, and validation. |
| `scanner_read_only` | Must not mutate during on-save. Scanner fixes become separate quality-action proposals. |
| `code_action` | May mutate only through a normalized code-action summary and this plan. |
| `generated_sync` | Requires generated/protected preview and rollback semantics. |
| `notebook_or_structured_formatter` | Must preserve stable IDs and unknown namespaces or require review. |
| `ai_apply` | Must already have a reviewed plan, target, ticket, and rollback boundary. |
| `extension_additional_edit` | Provider-dependent edits require typed review unless proven local and bounded. |
| `validation_after_apply` | Read-only verification unless separately admitted as a mutator. |

### 3.2 Preview thresholds

Every plan carries explicit thresholds:

- maximum files touched without preview;
- maximum created files without preview;
- maximum deleted files without preview;
- maximum changed bytes without preview;
- whether whole-document rewrites require preview;
- whether generated/protected targets always require preview; and
- whether provider-dependent edits always require review.

Minimum rules:

1. Hidden multi-file writes are forbidden.
2. Any participant that may touch a file outside the visible target must
   declare `may_touch_outside_visible_file = true` and use a preview or
   typed review posture unless the effective profile proves a narrower
   safe class.
3. File creation, deletion, generated/protected mutation, policy-scoped
   mutation, or provider-dependent additional edits require review by
   default.
4. Whole-document rewrites on files where round-trip safety is not
   proven require preview or an explicit manual command posture.
5. Scanner read-only participants may block, warn, or emit findings, but
   may not mutate as part of a scanner pass.

### 3.3 Additional-edit disclosure

Additional edits are any edits not obvious from the primary file/span
the user invoked. Examples include import insertion, companion generated
file updates, lint fix-all changes, manifest rewrites, lockfile updates,
or provider-selected edits outside the visible buffer.

The disclosure class is one of:

- `no_additional_edits`
- `single_file_declared`
- `multi_file_declared`
- `creates_or_deletes_files_declared`
- `generated_or_protected_declared`
- `provider_dependent_requires_review`
- `unknown_requires_review`

Rules:

1. `provider_dependent_requires_review` and `unknown_requires_review`
   cannot auto-apply on save.
2. Multi-file, generated/protected, policy-scoped, or create/delete
   plans must cite an `additional_edit_review_record` or a linked issue,
   diagnostic cluster, rule ref, or review ticket.
3. Additional-edit review records carry the same file counts,
   checkpoint hooks, provider dependency class, and issue linkage as the
   plan step they gate.
4. If a provider returns additional edits that exceed its declared plan,
   the participant result resolves to `preview_required`,
   `rebase_required`, `blocked_pending_issue_linkage`, or `failed`;
   it does not silently widen the write.

### 3.4 Rebase, abort, and timeout

Every participant declares:

- timeout in milliseconds;
- timeout policy;
- external-change rebase/abort policy;
- validation-after-apply class; and
- checkpoint policy.

Rules:

1. Save participants run against staged content. If the on-disk file
   changes mid-flight, the plan rebases, pauses, or aborts according to
   `rebase_policy_class`; it never clobbers external edits silently.
2. Timeouts produce explicit outcomes (`skipped`, `timed_out`,
   `preview_required`, `rebase_required`, or `failed`). Silent omission
   is a bug.
3. A failed participant does not downgrade the save into an unsafe
   write-back path. The plan either saves staged user content without
   that participant according to policy, holds for review, or aborts.
4. Validation failures after mutation can trigger rollback, review, or
   explicit degraded save depending on the plan; they are not hidden.

### 3.5 Checkpoints and rollback hooks

Checkpoint hooks are part of the plan, not best-effort afterthoughts.
Participants name:

- `checkpoint_policy_class`;
- pre-apply checkpoint ref;
- post-apply checkpoint ref;
- rollback plan ref;
- mutation group ref; and
- local-history group ref.

Rules:

1. Multi-file, cross-file semantic, generated/protected, policy-scoped,
   AI, extension-provider, and unknown/unstable mutations require a
   checkpoint before apply unless policy blocks body capture. If body
   capture is blocked, a metadata-only checkpoint or omission stub is
   still emitted.
2. Format-on-save groups remain attributable mutation groups. They do
   not vanish into ordinary text history.
3. Rollback labels must describe the real recovery class. Exact undo,
   grouped exact undo, compensating revert, regenerate, manual recovery,
   and audit-only outcomes are not interchangeable.

## 4. Drift labels and events

Quality surfaces must distinguish these profile identities:

- local profile;
- imported profile;
- effective profile; and
- remote/provider-authoritative profile.

Drift labels use `profile_drift_class`:

- `not_compared`
- `exact_match`
- `local_profile_differs`
- `imported_profile_differs`
- `effective_profile_differs`
- `remote_provider_profile_differs`
- `tool_version_drift`
- `rule_pack_drift`
- `config_mapping_drift`
- `environment_drift`
- `provider_authority_drift`
- `compatibility_blocked`
- `drift_unknown_requires_review`

Events are emitted when:

- a profile resolves;
- a policy lock applies;
- a downgrade applies;
- an imported config is read;
- a scanner import attaches;
- profile drift is detected;
- a parity comparison is created;
- a support export is created;
- an on-save plan opens;
- a participant starts or completes;
- review or preview is required;
- an external change forces rebase/abort;
- a checkpoint is created;
- rollback completes; and
- the plan commits or aborts.

These events are shared by desktop UI, CLI JSON, local CI, managed CI,
release packets, and support exports. A support engineer should be able
to trace local-vs-CI quality drift through profile and event metadata
without reproducing raw logs.

## 5. Required surface behavior

Desktop:

- show the effective provider/tool when different providers can mutate
  code differently;
- show source/lock/downgrade explanations in quality profile details;
- open preview for broad, generated/protected, provider-dependent, or
  issue-linked edits; and
- show on-save skipped, timed-out, rebase-required, or blocked outcomes.

CLI and headless:

- emit the same profile ids, drift labels, participant outcomes, and
  delta refs in structured output;
- keep machine-readable output clean from progress narration; and
- refuse hidden relaxation of on-save or headless mutation rules.

Local CI and managed CI:

- preserve effective profile refs, tool/rule-pack versions, invocation
  digests, and environment conditions;
- compare through `quality_result_delta_record`, not textual log
  scraping; and
- mark provider-authoritative results separately from local
  authoritative results.

Support and release exports:

- include profile refs, lock/downgrade summaries, scanner import refs,
  delta counts, baseline/suppression refs, and redaction notes;
- preserve raw scanner payloads by ref rather than embedding them; and
- declare omitted or compatibility-blocked axes explicitly.

## Change management

Adding a new tool family, provider kind, source layer, lock reason,
downgrade reason, drift class, participant class, action class, preview
requirement, checkpoint policy, or scanner result state is
additive-minor and updates the schemas and fixtures in the same change.

Repurposing an existing value is breaking. The value must remain stable
for support packets, release packets, review artifacts, and saved
profile/delta comparisons that already cite it.
