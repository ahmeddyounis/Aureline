# Diagnostics source taxonomy, semantic-layer state, suppression, and code-action convergence contract

This document freezes how Aureline describes one issue when that issue
is reported by more than one producer, at more than one time, with more
than one freshness posture, and with more than one safe next action.

Problems, inline markers, hover cards, review annotations, CLI mirrors,
support exports, and future quick-fix surfaces must not invent separate
truth about:

- who reported a finding;
- whether the evidence is live, cached, stale, superseded, or imported;
- whether the current anchor is exact, remapped, approximate, or gone;
- whether the semantic layer is current enough to support mutation; and
- whether suppressing or baselining the finding changes repo or policy
  truth rather than merely hiding a row locally.

Machine-readable companions:

- [`/schemas/language/diagnostic_cluster.schema.json`](../../schemas/language/diagnostic_cluster.schema.json)
  - `diagnostic_cluster_record`, the canonical issue-row packet that
    groups materially identical findings without losing provenance,
    freshness, anchor-remap, semantic-layer, imported-baseline, or
    suppression truth.
- [`/schemas/language/code_action_summary.schema.json`](../../schemas/language/code_action_summary.schema.json)
  - `code_action_summary_record`, the reviewable summary packet every
    quick-fix, fix-all, formatter, organize-imports, and generated-
    aware quality mutation must expose before broad apply.
- [`/schemas/language/suppression_review.schema.json`](../../schemas/language/suppression_review.schema.json)
  - `suppression_review_record`, the governed review packet for
    suppressions and baseline mutations so neither can masquerade as a
    harmless local dismissal.
- [`/fixtures/language/diagnostic_convergence_cases/`](../../fixtures/language/diagnostic_convergence_cases/)
  - worked YAML fixtures covering the required convergence scenarios.

This contract composes with and does not replace:

- [`/docs/language/provider_graph_and_arbitration_contract.md`](./provider_graph_and_arbitration_contract.md)
  - provider health, freshness, scope, locality, and result-provenance
    language already frozen for language-capable surfaces.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  - packet freshness, stale propagation, and rerun-trigger discipline.
- [`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json)
  - authoritative task, build, test, and runtime diagnostic provenance.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  - target and policy identity for build, test, debug, and remote runs.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  - generated-source lineage, mirrored-artifact honesty, and remap-safe
    disclosure for generated or paired files.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections 8.62, 9.57,
  9.58, and 9.59, plus `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  section 22.19.

If this document disagrees with the PRD, TAD, TDD, or UI/UX spec, those
documents win and this contract plus the companion schemas update in the
same change.

## Why freeze this now

Diagnostics are one of the easiest places for accidental product truth:

- the compiler and the language server can emit the same issue with
  slightly different labels;
- build/test/debug evidence can still be visible after the branch,
  target, or generated output changed;
- imported SARIF or scanner results can still be useful even when their
  original editor span is no longer exact;
- fix-all and formatter actions can touch generated or protected files
  while still looking like a harmless single-row quick fix; and
- suppressions or baseline accepts can mutate repo or policy truth while
  the surface only says "Ignore".

This contract makes those differences explicit before the full Problems
UI and code-action executor land.

## Scope

Frozen at this revision:

- the diagnostics source taxonomy for compiler/build,
  language-server, linter/formatter/style, framework/schema analyzer,
  runtime/test/debug, and policy/trust/security findings;
- origin, evidence-plane, freshness, epoch, anchor-remap, and
  semantic-layer vocabularies shared across compact and detailed
  surfaces;
- the cluster rules that collapse materially identical findings into one
  canonical issue row while keeping every contributing finding
  inspectable and exportable;
- the dominant-display and detail-sheet requirements a compact row must
  project into Problems, inline markers, hovers, review annotations,
  CLI, and support/export lanes;
- the review packet for code actions, fix-all mutations, formatter or
  organize-imports mutations, and generated/protected-path disclosures;
- the review packet for suppressions and baseline mutations, including
  owner, actor, expiry, reopen rule, evidence linkage, and replay hints;
- reserved anchor-remap and imported-baseline fields so imported or
  provider-fed findings can coexist with live local findings without
  claiming all anchors are current exact editor ranges.

Out of scope:

- the full Problems panel UI, sorting, filtering, and interaction model;
- executing a code action, suppression mutation, or baseline mutation;
- rule-engine specifics for any one compiler, linter, security scanner,
  or policy engine;
- the final quality-profile object; this contract cites profile or
  baseline refs, but does not define those registries.

## 1. Source taxonomy and evidence planes

Source kind says *who produced the finding*. Evidence plane says *what
kind of evidence it is*. Origin class says *where the current copy of
that evidence came from*.

### 1.1 Source kind

These are the only source kinds protected issue surfaces may name.

| `source_kind_class` | Plain-language label | Canonical examples | Never imply |
|---|---|---|---|
| `compiler_or_build` | Compiler / build | compiler JSON, build graph validator, problem matcher backed by build output | editor-current semantics unless a semantic epoch says so |
| `language_server` | Language service | LSP diagnostics, semantic language daemon warnings | build/runtime confirmation |
| `linter_formatter_style` | Linter / formatter / style | ESLint, Clippy lint pass, style or formatting rule, organize-imports rule | semantic safety beyond the declared action packet |
| `framework_or_schema_analyzer` | Framework / schema analyzer | route analyzer, migration/schema validator, framework conventions checker | that the underlying language server agreed |
| `runtime_test_or_debug` | Runtime / test / debug | failing test, debugger exception, runtime assertion, notebook-kernel failure | that the code still fails on the current branch or target |
| `policy_trust_or_security` | Policy / trust / security | trust gate, local secret scan, license/compliance gate, imported security scan | live current local proof when the origin is imported |

### 1.2 Evidence plane

Evidence plane is orthogonal to source kind and remains visible even in
clustered rendering.

| `evidence_plane_class` | Meaning |
|---|---|
| `static_analysis` | Derived from source, schema, or semantic analysis without executing the target. |
| `build_time_execution` | Derived from compile/build/tool execution. |
| `runtime_or_test_execution` | Derived from a run, test, debug session, notebook execution, or live process. |
| `policy_or_trust_evaluation` | Derived from trust, policy, compliance, or governed review logic. |
| `imported_snapshot_evidence` | Derived from imported evidence whose original producer session is not current-local. |

### 1.3 Origin class

Origin remains required even when the finding is compacted into one row.

| `origin_class` | Meaning |
|---|---|
| `live_local_session` | Produced against the current local session or its admitted current epoch. |
| `live_remote_session` | Produced against the current remote workspace or target session. |
| `managed_provider_live` | Produced live by a managed or service-backed provider. |
| `imported_snapshot` | Imported from SARIF-like, release, support, or scanner packet evidence. |
| `replayed_support_bundle` | Replayed from preserved support/export evidence rather than rerun live. |

Rules:

1. `source_kind_class`, `evidence_plane_class`, and `origin_class`
   MUST all remain present even when a surface renders one compact row.
2. `runtime_test_or_debug` evidence MUST stay distinct from purely
   static evidence on detail surfaces. Clustering may reduce noise, but
   it may not erase the runtime/static distinction.
3. `imported_snapshot` origin may coexist with live rows, but it may not
   paint imported anchors or freshness as if they were current exact
   editor truth.

## 2. Freshness, epochs, and semantic-layer state

### 2.1 Freshness

Diagnostics use a compact freshness vocabulary distinct from, but
compatible with, the wider evidence-freshness policy.

| `diagnostic_freshness_class` | Meaning |
|---|---|
| `current` | Evidence is current for the admitted epoch and scope. |
| `recent` | Evidence is still reviewable and close to current, but already below the exact-current floor. |
| `stale` | Evidence is visibly older than the current admitted epoch or target state. |
| `superseded` | Newer evidence exists and this finding stays only for lineage, comparison, or replay. |
| `imported_snapshot` | Evidence was imported and is not claiming live local freshness. |

### 2.2 Epoch bindings

Every cluster and every contributing diagnostic MAY carry opaque epoch
bindings. The schema names the role, not the epoch grammar.

Frozen epoch roles:

- `workspace_scope`
- `diagnostic_collection`
- `language_semantic_model`
- `build_graph`
- `execution_run`
- `imported_scan`
- `anchor_remap_family`
- `policy_bundle`

Rules:

1. A cluster MAY aggregate findings from different epoch roles, but an
   epoch mismatch MUST remain inspectable and MUST influence
   `semantic_layer_state_class`, `cluster_freshness_class`, or both.
2. A code action MAY NOT be presented as safe-to-apply when the acting
   provider cannot cite a compatible current semantic epoch for the
   mutation it proposes.

### 2.3 Semantic-layer state

Semantic-layer state tells the user whether a diagnostic or action is
running on current semantic truth, remapped truth, narrowed truth, or
no semantic truth at all.

| `semantic_layer_state_class` | Meaning |
|---|---|
| `semantic_current_exact` | Semantic layer is current and the anchor is exact. |
| `semantic_current_remapped` | Semantic layer is current enough to remap the anchor, but not to claim the original exact range survived unchanged. |
| `semantic_narrowed_scope` | Semantic layer is current only for a narrower slice, file, or workset. |
| `semantic_cached_recent` | Semantic layer is warm/cached and still reviewable, but not current exact truth. |
| `semantic_stale_epoch_mismatch` | Semantic layer is below the current epoch or target floor. |
| `syntactic_or_text_only` | No trusted semantic layer; finding comes from text/syntax/regex-like fallback only. |
| `runtime_observed_no_semantic_basis` | Runtime/test/debug proved the issue, but no current semantic basis exists for safe mutation. |
| `imported_snapshot_only` | Imported evidence exists without current local semantic revalidation. |
| `policy_asserted_non_semantic` | Finding comes from policy/trust/security logic that is not a source semantic claim. |

Rules:

1. Inline and compact surfaces MUST never style
   `semantic_current_remapped`, `semantic_cached_recent`,
   `semantic_stale_epoch_mismatch`, `imported_snapshot_only`, or
   `runtime_observed_no_semantic_basis` as if they were
   `semantic_current_exact`.
2. Code actions and suppressions MUST disclose the semantic-layer state
   they rely on before broad apply.

## 3. Canonical issue row and cluster rules

The canonical row is the `diagnostic_cluster_record`. Clustering is an
ergonomic projection, not a truth rewrite.

### 3.1 What may cluster

Findings may cluster only when all of these are materially compatible:

1. rule or normalized-equivalence family;
2. category and effect scope;
3. anchor family or explicit remap family;
4. target or environment scope;
5. suppression/baseline semantics; and
6. safe-next-action posture.

If merging two findings would hide a different mutation scope, a
different suppression scope, a different target/environment, or a
different evidence plane with different user risk, they do not cluster.

### 3.2 Dedupe reasons

The cluster MUST name the dedupe rule that admitted the merge.

| `dedupe_reason_class` | Meaning |
|---|---|
| `same_rule_same_exact_anchor` | Same rule family and same current exact anchor. |
| `same_rule_same_remap_family` | Same rule family and same append-only remap family, but not necessarily the same exact current range. |
| `same_message_same_anchor_family` | Provider IDs differ but the normalized message and anchor family are materially identical. |
| `same_entity_same_effect_scope` | Findings describe the same logical entity and effect scope even if anchors differ slightly. |
| `runtime_static_same_failure_family` | Static and runtime evidence corroborate the same failure family and remain split in detail view. |
| `imported_baseline_overlap` | Imported baseline or scanner evidence overlaps a live or remapped current family. |

### 3.3 Primary diagnostic selection

Each cluster carries one `primary_diagnostic_ref` and one `primary_anchor`.
Selection rules are:

1. current exact beats remapped;
2. remapped beats stale or unmapped;
3. live local or live remote beats imported snapshot when both are
   otherwise materially equivalent;
4. higher-confidence and higher-support evidence beats lower-confidence
   evidence;
5. when severities differ, the dominant severity wins compact rendering,
   but every differing severity remains inspectable.

### 3.4 Display and severity convergence

Compact surfaces read `dominant_display_state_class` and
`severity_convergence_class` rather than guessing from prose.

Frozen dominant display states:

- `current_exact_live`
- `current_remapped_live`
- `current_with_severity_conflict`
- `current_mixed_static_and_runtime`
- `stale_or_superseded`
- `imported_snapshot_only`
- `suppressed_or_baselined_governed`

Frozen severity convergence states:

- `single_severity`
- `conflicting_severity_present`
- `policy_overridden_severity_present`

### 3.5 Detail-sheet requirements

Every cluster names the sections a detail sheet MUST expose before the
user loses important truth.

Frozen `detail_section_class` rows:

- `cluster_provenance_table`
- `freshness_and_epoch_strip`
- `severity_conflict_explainer`
- `runtime_static_evidence_split`
- `anchor_and_remap_history`
- `imported_baseline_and_delta`
- `suppression_and_baseline_governance`
- `available_code_actions`
- `generated_and_protected_impact`
- `raw_payload_and_export_links`

Rules:

1. A grouped row may collapse visually, but drill-in MUST preserve every
   contributing diagnostic, its source kind, evidence plane, origin,
   freshness, semantic-layer state, and anchor-remap state.
2. A detail sheet may reorder sections, but it may not omit a section
   listed in `detail_sheet_requirements`.

## 4. Anchors, remap, and imported-baseline coexistence

Imported or stale findings remain useful only if the anchor story stays
honest.

### 4.1 Anchor-remap state

The stable remap vocabulary is:

- `exact`
- `contextual`
- `stale`
- `unmapped`
- `imported_static`

Meaning:

- `exact` - the current anchor is still the original exact anchor.
- `contextual` - Aureline has evidence for a new current anchor, but it
  is a remap, not the original exact range.
- `stale` - the original anchor family exists, but the remap evidence is
  now below the current epoch floor.
- `unmapped` - Aureline can preserve lineage, but cannot place a current
  anchor honestly.
- `imported_static` - anchor came from imported evidence and is retained
  as imported lineage rather than current local editor truth.

### 4.2 Imported baseline binding

The cluster schema reserves an optional `imported_baseline_binding`
block. It pins:

- the import session;
- the baseline family;
- the run descriptor or imported run;
- compatibility or drift posture; and
- the baseline delta state.

This block exists so SARIF-like or imported scanner evidence can remain
next to live findings without pretending those runs were rerun locally.

Rules:

1. Imported evidence MAY cluster with live evidence only when the rule,
   effect scope, and anchor family are materially compatible.
2. When imported evidence clusters with live evidence, the imported
   baseline block and remap state MUST remain visible in drill-in.
3. No surface may claim an imported contextual anchor is the current
   exact editor range.

## 5. Code-action summary packet

Every mutation-bearing fix proposal reads one
`code_action_summary_record` before apply.

### 5.1 What the packet carries

The packet freezes:

- action class and user-facing label;
- acting provider identity, source kind, freshness, locality, and
  semantic-layer state;
- linked diagnostic clusters;
- mutation safety class and mutation scope;
- preview requirement and current apply posture;
- counts for affected diagnostics, files, anchors, generated paths,
  protected paths, and blocked paths; and
- validation and replay hints.

### 5.2 Safety class

The stable safety-class vocabulary is:

- `trivia_safe`
- `local_syntax_safe`
- `semantic_local`
- `cross_file_semantic`
- `generated_or_protected`
- `unknown_or_unstable`

Rules:

1. `generated_or_protected` and `unknown_or_unstable` actions MUST NOT
   render as inline one-click apply without an explicit review packet.
2. An action whose provider freshness or semantic epoch is below the
   required floor MUST disclose a blocked or preview-before-apply
   posture rather than pretending the mutation is locally obvious.
3. Generated, protected, and blocked counts are first-class review
   fields. They are never hidden inside a tooltip or provider-specific
   footnote.

### 5.3 Validation and replay

Every action packet carries `validation_hint_classes`,
`replay_hint_classes`, and a short summary. These hints allow CLI,
review, support/export, and future automation surfaces to describe the
same post-apply verification path:

- rerun the producer that emitted the diagnostic;
- rerun build, test, or debug evidence;
- refresh the semantic snapshot or framework model;
- review generated outputs or protected paths explicitly;
- replay against the same or newer semantic epoch; and
- preserve enough review/support linkage to reconstruct what the user
  saw before apply.

## 6. Suppression and baseline review packet

Suppressions and baselines are governed truth mutations, not ephemeral
row toggles. This contract uses one `suppression_review_record` shape
for both suppression and baseline mutations because both must remain
reviewable, attributable, exportable, and replayable.

### 6.1 What the packet carries

The packet freezes:

- the mutation kind;
- the target scope and its summary;
- the truth-mutation class (local-only, workspace artifact, policy
  artifact, or imported-baseline artifact);
- owner and actor identity;
- reason summary and evidence links;
- policy-lock state;
- expiry or review timing;
- reopen rule;
- related source kinds and diagnostic refs; and
- validation and replay hints.

### 6.2 Truth mutation classes

The packet makes the mutation boundary explicit:

- `local_session_visibility_only`
- `workspace_repo_artifact`
- `managed_policy_artifact`
- `imported_baseline_artifact`

Rules:

1. A suppression or baseline action that changes repo, policy, or
   imported-baseline truth MUST NOT look like a local dismiss action.
2. Expiry, owner, actor, and reopen rule MUST remain visible anywhere
   the suppressed or baselined finding is rendered compactly.
3. Expired suppressions reopen automatically according to the same
   governed packet; the product may not quietly drop the suppression and
   lose the audit trail.
4. Baseline accept and supersede actions remain exportable and
   attributable even when surfaced from compact Problems or review UI.

## 7. Fixture corpus

The fixture corpus under
[`/fixtures/language/diagnostic_convergence_cases/`](../../fixtures/language/diagnostic_convergence_cases/)
covers the minimum scenarios required by the task:

| Fixture | Record kind | Scenario it freezes |
|---|---|---|
| `compiler_lsp_duplicate_current.yaml` | `diagnostic_cluster_record` | Compiler/build and language-server findings collapse into one canonical current issue row without losing either source. |
| `conflicting_severities_cluster.yaml` | `diagnostic_cluster_record` | Materially identical findings cluster, but conflicting severities remain explicit. |
| `stale_runtime_evidence_cluster.yaml` | `diagnostic_cluster_record` | Current static evidence and stale runtime evidence coexist without repainting the stale runtime proof as live. |
| `multi_file_generated_impact_fix.yaml` | `code_action_summary_record` | Multi-file fix proposal discloses generated/protected/blocked impact and preview or validation requirements. |
| `time_bounded_suppression_review.yaml` | `suppression_review_record` | Time-bounded suppression shows owner, actor, expiry, evidence link, repo-mutation truth, and reopen rule. |
| `imported_scan_anchor_remap.yaml` | `diagnostic_cluster_record` | Imported security scan finding stays imported, contextual-remapped, and baseline-bound rather than pretending it is a current exact local anchor. |

These fixtures are normative examples. Future diagnostics, fix, and
suppression implementations may add fields or cases, but they may not
weaken the honesty rules frozen above without a new decision row.
