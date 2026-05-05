# Language-provider graph, capability arbitration, and result-provenance contract

This document freezes how Aureline attributes, arbitrates, downgrades,
and explains the language-intelligence providers that can contribute to
definition, reference, hover, rename, completion, code-action,
diagnostic, notebook-context, and AI-assist-context surfaces.

The goal is to keep later implementation work from turning incidental
provider order into product truth. Syntax, project-graph,
language-server, framework-pack, notebook-adapter,
generated-source-bridge, and AI-assist lanes all stay attributable,
scoped, freshness-labeled, and exportable before live integrations land.

Machine-readable companions:

- [`/schemas/language/provider_status_row.schema.json`](../../schemas/language/provider_status_row.schema.json)
  — `provider_status_row_record`, the row every inspector, status
  surface, and support export reads when it needs one stable answer to
  "what provider is available here, for which scope, and with what
  health/freshness/locality posture?"
- [`/schemas/language/capability_negotiation_packet.schema.json`](../../schemas/language/capability_negotiation_packet.schema.json)
  — `capability_negotiation_packet_record`, the cross-tool packet for
  one surface request, its candidate providers, the negotiated winner,
  downgrade cue, cached-reuse posture, and alternate-provider
  disclosure.
- [`/schemas/language/result_provenance.schema.json`](../../schemas/language/result_provenance.schema.json)
  — `result_provenance_record`, the cross-surface record that follows a
  concrete result set or explanation into UI, CLI, AI, notebook,
  support, and export lanes.
- [`/fixtures/language/provider_arbitration_cases/`](../../fixtures/language/provider_arbitration_cases/)
  — worked YAML cases covering the required arbitration scenarios.

This contract composes with and does not replace:

- [`/docs/language/three_layer_model_contract.md`](./three_layer_model_contract.md)
  — shared three-layer language depth model and “which layer answered?” truth
  contract that this provider graph maps into.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — shared truth/degraded-state vocabulary.
- [`/docs/search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)
  — fallback, freshness, and partial-truth language already frozen for
  search and navigation.
- [`/docs/workspace/scope_truth_packet.md`](../workspace/scope_truth_packet.md)
  — the scope object this contract quotes when a provider only covers a
  file, notebook cell, workset, slice, or full workspace.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  — generated and mirrored artifact posture. This contract cites those
  labels; it does not invent a second generated-source dialect.
- [`/docs/language/framework_pack_contract.md`](./framework_pack_contract.md)
  — framework-pack descriptor, package-manager identity, and lineage
  disclosure contract. Framework-pack lanes here remain attributable and
  cannot grow pack-private metadata outside shared graph/execution/lineage
  records.
- [`/docs/ai/context_assembly_contract.md`](../ai/context_assembly_contract.md)
  — AI context/evidence contract. AI-assist lanes here remain advisory
  and attributable; they do not bypass that evidence model.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
  — crash-loop and quarantine posture. Language-provider crash handling
  reuses the same supervisor vocabulary instead of inventing a private
  "retrying" folklore state.

If this document disagrees with the PRD, TAD, TDD, or the linked
contracts above, those documents win and this document plus the
companion schemas update in the same change.

## Why freeze this now

Language intelligence is one of the highest-risk places for accidental
product truth:

- a syntax-only local answer can quietly render like whole-workspace
  semantics;
- a warm graph can answer quickly but only for the loaded slice;
- an LSP and a framework pack can disagree while the product presents a
  single unlabeled answer;
- notebook cells and paired/generated artifacts can look like ordinary
  source even when their truth is projected or partial;
- AI assists can drift from deterministic providers if the product does
  not pin them to an explicit advisory lane.

The design docs already require honest degradation, shared graph truth,
notebook-aware limits, generated-artifact honesty, and evidence-backed
AI. This contract turns those principles into one frozen provider
language so later M1/M2 implementation cannot quietly widen claims.

## Scope

Frozen at this revision:

- provider-family taxonomy for syntax, project-graph,
  language-server, framework-pack, notebook-adapter,
  generated-source-bridge, and AI-assist lanes;
- plain-language labels, health states, freshness labels,
  scope-claim classes, completeness labels, locality cues, generated
  notes, notebook notes, and conflict classes;
- the `provider_status_row_record` every status inspector and export
  reads;
- the `capability_negotiation_packet_record` every language-capable
  request emits before a protected surface treats a provider as the
  current winner;
- the `result_provenance_record` every protected consumer reads once a
  result exists;
- winner-selection, alternate-provider disclosure, semantic-to-text
  downgrade, cached/stale reuse, crash-loop exclusion, and
  partial-scope disclosure rules;
- framework-certainty classes, proving-artifact refs, and
  source-of-certainty chains shared by navigation, diagnostics,
  notebook context, and AI assistance.

Out of scope:

- live LSP, graph, framework-pack, notebook, generator, or AI provider
  integrations;
- final provider-scoring algorithms beyond the eligibility and honesty
  rules frozen here;
- concrete UI layout, iconography, or animation for chips, badges, or
  inspectors;
- the final refactor engine, code-action executor, or runtime
  integration that will populate these records.

## 1. Provider families and roles

### 1.1 Provider families

These are the only provider families protected surfaces may name.

| `provider_family` | Plain-language label | Canonical job | Never imply |
|---|---|---|---|
| `syntax` | Syntax / local text | File-local parsing, structural fallback, local token boundaries, text-level fallback | whole-workspace semantics on its own |
| `project_graph` | Project graph | Shared semantic graph facts, cross-file relations, ownership of loaded graph truth | that unloaded or omitted scope was included |
| `language_server` | Language service | Compatibility layer for semantic actions, diagnostics, completion, hover, rename, references | framework-specific certainty without separate proof |
| `framework_pack` | Framework pack | Framework-aware navigation, route/component/config facts, framework diagnostics, framework-specific code actions | that imported or inferred evidence is framework-proven |
| `notebook_adapter` | Notebook adapter | Cell/document projection, notebook-aware language synchronization, notebook-local anchors | cross-cell or whole-workspace certainty it did not actually compute |
| `generated_source_bridge` | Generated source bridge | Mapping between canonical source and generated/paired artifacts, generated-scope disclosure | canonical-source authority when lineage is missing |
| `ai_assist` | AI assist | Advisory explanation, suggestion, or completion overlay with citations/uncertainty | authoritative semantic truth |

### 1.2 Provider roles

Family and role are separate. One family may appear in more than one
role across different surfaces.

| `provider_role_class` | Meaning |
|---|---|
| `primary_semantic` | Current best semantic source for the requested surface and scope. |
| `secondary_semantic` | Semantic contributor retained as an alternate or corroborating source. |
| `framework_overlay` | Framework-aware overlay that may refine or challenge a language-server or graph answer. |
| `notebook_projection` | Notebook-scoped projection over cell/document identities. |
| `generated_overlay` | Mapping or lineage overlay for generated/paired artifacts. |
| `text_fallback` | Narrower lexical/syntax fallback. Honest downgrade, never silent replacement. |
| `assist_only` | Advisory AI lane. Never primary authority. |

## 2. Shared status vocabulary

### 2.1 Health states

| `health_state` | Plain-language cue | Meaning |
|---|---|---|
| `ready` | Ready | Provider is healthy enough to serve new answers for its declared scope. |
| `warming` | Warming | Provider is starting or rebuilding. May answer only through cached or narrower fallback paths. |
| `degraded` | Degraded | Provider is reachable but below ideal posture; result authority narrows. |
| `cached_only` | Cached only | Provider may reuse prior results but must not imply fresh computation. |
| `policy_blocked` | Blocked by policy | Policy/trust posture prevents use. |
| `capability_missing` | Not supported here | Provider exists but does not support this surface or environment. |
| `crash_loop_quarantined` | Quarantined after crash loop | Supervisor excluded the provider until explicit recovery. |
| `unavailable` | Unavailable | Provider cannot be reached or has no admissible answer path. |

### 2.2 Freshness labels

This contract reuses the already-frozen freshness scale rather than
minting a language-only copy dialect.

| `freshness_class` | Plain-language cue | Meaning |
|---|---|---|
| `authoritative_live` | Live | Computed against the current admitted epoch for the claimed scope. |
| `warm_cached` | Warm cached | Recent cached result still inside an explicit grace window. |
| `degraded_cached` | Degraded cached | Cached result remains usable but already below ideal posture. |
| `stale` | Stale | Result is past its freshness floor. |
| `unverified` | Unverified | Provider or imported state cannot prove current freshness. |

### 2.3 Scope claim and completeness

The product must answer two separate questions:

1. *What scope is this provider or result claiming to cover?*
2. *Is it complete for that claimed scope?*

Allowed scope claims:

| `scope_claim_class` | Meaning |
|---|---|
| `single_file` | Only the active file/document. |
| `notebook_cell` | Only the addressed notebook cell or paired text projection. |
| `loaded_slice` | Only the currently loaded repo slice or diff/review slice. |
| `active_workset` | The active named workset or current open roots as scoped by the workset contract. |
| `whole_workspace` | The entire admitted workspace. Reserved for providers that can actually prove it. |
| `unavailable` | No admissible claim because the provider could not answer honestly. |

Allowed completeness labels:

| `completeness_class` | Meaning |
|---|---|
| `complete_for_claimed_scope` | Complete for the scope claimed above. |
| `partial_for_claimed_scope` | Only part of the claimed scope is covered. |
| `unavailable_for_claimed_scope` | The provider cannot currently serve the claimed scope. |

### 2.4 Locality and host cues

Every protected surface must tell the user where the provider ran.

| `locality_class` | Plain-language cue |
|---|---|
| `local_in_process` | On this machine |
| `local_sidecar` | Local helper |
| `workspace_remote_agent` | Workspace remote |
| `managed_service` | Managed service |
| `imported_snapshot` | Imported / cached snapshot |

### 2.5 Scope-limit classes

Scope limits are the concrete reason a provider or result is narrower
than a larger possible claim.

| `scope_limit_class` | Meaning |
|---|---|
| `single_file_only` | Provider only reasoned over the current file/document. |
| `active_workset_only` | Scope narrowed to the active workset. |
| `unloaded_roots_omitted` | Some roots/modules were outside the loaded semantic set. |
| `generated_overlay_only` | Truth is limited to a generated or paired overlay. |
| `generated_candidates_omitted` | Generated siblings or rewrite targets were intentionally excluded. |
| `notebook_cell_projection_only` | Provider only reasoned over the current cell projection. |
| `cross_cell_context_unavailable` | Notebook-wide context was not available. |
| `diff_or_review_slice_only` | Result only reflects the current review/diff slice. |
| `policy_narrowed` | Policy/trust posture narrowed the admissible scope. |
| `remote_shard_unreachable` | Some remote scope was unreachable. |

### 2.6 Generated and notebook note classes

Generated-source notes:

- `generated_overlay_in_scope`
- `generated_target_only`
- `generated_candidates_omitted`
- `generated_lineage_unknown`

Notebook notes:

- `notebook_cell_projection`
- `cross_cell_context_limited`
- `paired_export_projection`
- `kernel_runtime_not_consulted`

These notes are additive. They do not replace scope claim or
completeness.

### 2.7 Conflict classes

One conflict class is required whenever two providers disagree in a way
that changes user risk.

| `conflict_class` | Typical surfaces |
|---|---|
| `none` | Any surface with a single uncontested source. |
| `target_set_disagreement` | Definition, reference, rename. |
| `scope_coverage_disagreement` | Reference, rename, code action. |
| `framework_language_disagreement` | Definition, hover, code action, diagnostics. |
| `semantic_text_disagreement` | Definition, reference, hover. |
| `edit_safety_disagreement` | Rename, code action. |
| `freshness_or_epoch_disagreement` | Any surface mixing epochs. |
| `generated_boundary_disagreement` | Rename, code action, hover. |
| `notebook_boundary_disagreement` | Hover, rename, completion, notebook context. |
| `narrative_fact_disagreement` | Hover and AI-assist explanation lanes. |

## 3. Provider-status rows

Every inspectable provider row must answer:

1. Which provider family and role is this?
2. For which scope can it currently speak?
3. Is it fresh, complete, local/remote/imported, and healthy?
4. Which protected surfaces may currently treat it as authoritative,
   advisory, fallback-only, or unsupported?

The `provider_status_row_record` is therefore the normalized row read by
status inspectors, hover provenance panels, result inspectors, support
exports, and any "why did this provider not win?" surface.

Required row content:

- stable provider identity and display label;
- family and role;
- health state and freshness class;
- scope claim, completeness, and any scope limits;
- locality and optional host identity ref;
- current epoch bindings;
- per-surface support rows using the shared support classes
  `authoritative`, `advisory`, `fallback_only`, and `unsupported`;
- generated/notebook notes when relevant;
- export-safe summary.

Rules:

1. Health and freshness are separate axes. `ready + stale` is allowed
   and means "reachable but stale"; `cached_only + warm_cached` is
   allowed and means "safe to reuse within the declared grace window".
2. `crash_loop_quarantined` is a hard exclusion for new winner
   selection. A quarantined provider may appear in a status table or an
   alternate-provider disclosure, but it may not silently remain the
   current winner.
3. `whole_workspace + complete_for_claimed_scope` is admissible only
   when the row carries no scope-limit classes.
4. `imported_snapshot` may be inspectable and useful, but it may not
   claim `authoritative_live`.
5. AI-assist rows are advisory or fallback-only by design. They do not
   become authoritative through ranking.

## 4. Capability-negotiation packet

Every language-capable request emits one
`capability_negotiation_packet_record` before the surface renders a
winner as current truth. The packet answers:

1. What surface asked the question?
2. What scope and authority floor did it ask for?
3. Which providers were considered?
4. Which provider won, why, and with what downgrade or reuse cue?
5. Which alternate providers existed, and why were they not primary?

### 4.1 Request floor

The request names one of three authority floors:

| `requested_authority_floor_class` | Meaning |
|---|---|
| `authoritative_required` | Do not silently downgrade to advisory or fallback truth. |
| `authoritative_preferred` | Prefer authoritative truth but disclose a narrower/advisory winner if needed. |
| `advisory_allowed` | Advisory or fallback truth is acceptable as long as it is labeled honestly. |

### 4.2 Winner-selection rules

Final ranking algorithms remain out of scope, but eligibility and
honesty rules are frozen:

1. A provider in `policy_blocked`, `capability_missing`,
   `crash_loop_quarantined`, or `unavailable` health may not win.
2. A provider may not win for a surface it marks `unsupported`.
3. `authoritative_for_claimed_scope` is allowed only when the chosen
   provider is `ready`, `authoritative_live`, and
   `complete_for_claimed_scope`.
4. `whole_workspace` authority is allowed only when the chosen provider
   is a `project_graph`, `language_server`, or `framework_pack` lane
   that also carries no scope-limit classes.
5. If the request asked for `whole_workspace` and the winner only
   covers `active_workset`, `loaded_slice`, `single_file`, or
   `notebook_cell`, the packet must carry a non-`none` downgrade cue.
6. AI-assist providers never satisfy `authoritative_required`.

### 4.3 Downgrade cues

Every non-baseline arbitration result carries one typed cue:

| `downgrade_cue_class` | Meaning |
|---|---|
| `none` | No downgrade. Winner satisfied the request as asked. |
| `scope_narrowed` | Winner is honest but covers less scope than requested. |
| `semantic_to_text_fallback` | Semantic provider lost; text-level fallback won. |
| `semantic_to_file_local_syntax` | Only file-local syntax/structure is admissible. |
| `semantic_to_cached_result` | Cached semantic result reused instead of fresh compute. |
| `framework_certainty_lowered` | Framework result remained useful but dropped from proven to inferred/imported/stale. |
| `generated_scope_narrowed` | Generated/paired artifact boundaries narrowed the answer. |
| `notebook_scope_narrowed` | Notebook projection narrowed the answer. |
| `provider_unavailable` | No fresh provider was reachable; a narrower or cached path won. |
| `provider_crash_loop_quarantined` | Quarantined provider excluded the preferred path. |
| `policy_blocked` | Policy/trust posture narrowed the admissible provider set. |

### 4.4 Alternate-provider disclosure

Alternate providers are not optional metadata. When a second provider
could plausibly have been treated as truth, the packet carries it with a
typed "not primary" reason such as:

- `lower_authority`
- `narrower_scope`
- `older_epoch`
- `lower_framework_certainty`
- `health_excluded`
- `policy_excluded`
- `mutation_safety_lower`
- `redundant_same_answer`

### 4.5 Cached and stale reuse

Cached or stale reuse is allowed only under explicit disclosure:

1. A cached result may reuse the prior semantic answer only if the
   packet records a non-`not_applicable` cached-reuse reason.
2. Cached reuse never upgrades authority. It is advisory or fallback
   unless the chosen provider still meets the live-authoritative rules
   above.
3. Cached reuse keeps the original provider family visible. The product
   must not repaint a cached graph/LSP answer as if it were fresh local
   syntax.

### 4.6 Crash-loop handling

Crash loops narrow capability rather than pretending the provider still
exists.

1. `crash_loop_quarantined` providers remain inspectable in status
   surfaces.
2. The negotiation packet excludes them from primary selection.
3. If their exclusion caused the downgrade, the downgrade cue must say
   so.
4. Recovery is a separate event. A quarantined provider does not become
   admissible again simply because a UI panel was reopened.

## 5. Result-provenance record

Once a surface has a result, explanation, candidate set, or preview, it
must carry one `result_provenance_record`.

The record binds together:

- the result identity and surface/result kind;
- the primary provider snapshot;
- any contributing providers and alternates;
- authority, freshness, scope claim, completeness, locality, and
  epochs;
- generated-source and notebook notes;
- framework certainty and its proving artifacts;
- source-of-certainty chain;
- export-safe summary.

Rules:

1. Every protected consumer reads the same record. Navigation,
   diagnostics, notebook context, AI assistance, CLI, support export,
   and review surfaces may project it differently, but they may not
   invent private provenance fields.
2. Conflict classes remain attached to the result even when one winner
   was selected. The product may not flatten disagreement into a single
   unlabeled row.
3. `authoritative_for_claimed_scope` obeys the same `ready +
   authoritative_live + complete_for_claimed_scope` rule as capability
   negotiation.
4. `semantic_to_text_fallback` and `semantic_to_file_local_syntax`
   require the primary provider to be a `syntax` family provider in the
   `text_fallback` role.
5. Result provenance always carries an export-safe summary. Support and
   AI evidence packets do not have to reconstruct provenance from UI
   copy.

## 6. Surface-specific arbitration rules

The same vocabulary applies everywhere, but the risk model differs by
surface.

| Surface | Arbitration rule |
|---|---|
| `definition` | Prefer the strongest exact target set. Declaration-vs-definition downgrades and multi-target ambiguity stay explicit. Text fallback is never painted as semantic exactness. |
| `reference` | Reference sets name access role and covered scope. Grep or structural fallback remains labeled; omitted roots/cells/generated targets remain visible. |
| `hover` | Factual symbol data and narrative explanation may coexist, but `narrative_fact_disagreement` must stay visible. AI prose may supplement; it may not overwrite deterministic hover truth. |
| `rename` | Broad rename requires an inspectable candidate set. Generated, readonly, notebook-limited, or unloaded-scope candidates remain visible as limits or blocked items rather than disappearing. |
| `completion` | Deterministic language intelligence, snippets, cached fallback, and AI proposals remain distinct lanes. AI-only completions stay advisory even when they are the only available assist path. |
| `code_action` | Side-effect and scope safety outrank convenience. If providers disagree about edit safety, generated targets, or omitted scope, the surface downgrades to preview/advisory rather than silent apply. |

## 7. Framework certainty and proving artifacts

Framework-aware claims reuse one certainty language everywhere.

### 7.1 Frozen certainty classes

| `framework_certainty_class` | Meaning |
|---|---|
| `framework_not_applicable` | This result is not making a framework-specific claim. |
| `framework_proven` | Proven by framework-pack analysis or a framework-pack-declared proving artifact. |
| `framework_inferred` | Useful framework guess derived from graph/LSP/build evidence but not framework-proven. |
| `framework_imported` | Framework fact imported from a captured snapshot or external artifact. |
| `framework_stale` | Former framework signal exists but is below freshness requirements. |
| `framework_unavailable` | No admissible framework certainty is available. |

### 7.2 Primary certainty sources

| `framework_primary_source_class` | May yield `framework_proven`? |
|---|---|
| `framework_pack_analysis` | yes |
| `framework_pack_artifact` | yes |
| `project_graph_projection` | no |
| `language_server_signal` | no |
| `build_adapter_signal` | no |
| `imported_snapshot` | no |
| `ai_inference` | no |
| `none` | no |

### 7.3 Rules

1. `framework_proven` requires:
   a non-empty proving-artifact set and a primary certainty source of
   `framework_pack_analysis` or `framework_pack_artifact`.
2. A language-server-only, build-adapter-only, imported, or AI-only
   chain may be useful, but it may not be surfaced as
   `framework_proven`.
3. `framework_imported` is reserved for imported/captured framework
   facts and stays labeled as imported everywhere.
4. `framework_stale` means the framework signal exists but lost its
   freshness floor. It does not silently degrade to inferred/proven.
5. The same certainty block and source-of-certainty chain travel with
   navigation results, diagnostics, notebook-context packets, and AI
   assist explanations.

## 8. Source-of-certainty chains

Every result provenance record carries a source-of-certainty chain.
Entries may cite:

- `framework_pack_analysis`
- `framework_pack_artifact`
- `project_graph_fact`
- `language_server_fact`
- `syntax_fact`
- `build_adapter_fact`
- `notebook_adapter_projection`
- `generated_source_lineage`
- `imported_snapshot`
- `ai_inference`
- `user_curated_override`

The chain is not freeform prose. Each entry names the source class, an
optional provider id, an optional artifact ref, and an export-safe
summary. That makes framework and AI explainers reconstructable without
dragging raw payloads into support/export paths.

## 9. Worked fixture corpus

The companion fixture set covers the minimum required cases:

- `syntax_only_local_file.yaml`
  — syntax/file-local fallback with honest `single_file` scope.
- `graph_warm_partial_workspace.yaml`
  — graph-warm references on an `active_workset` with unloaded roots
  omitted, proving whole-workspace authority is structurally blocked.
- `lsp_framework_disagreement.yaml`
  — LSP and framework-pack disagreement with alternate-provider
  disclosure and framework certainty preserved.
- `notebook_generator_limited_truth.yaml`
  — notebook/generator-limited rename preview showing notebook and
  generated-source boundaries explicitly.
- `ai_overlay_assist_only.yaml`
  — AI-assist-only completion negotiation with advisory-only authority
  and deterministic providers unavailable.
- `language_server_crash_loop_quarantined.yaml`
  — provider-status row proving crash-loop quarantine is explicit and
  primary selection is excluded until recovery.

## 10. Acceptance checklist

A reviewer can audit conformance without implementation code:

1. **Provider identity**: Can you tell which provider family and role
   supplied the visible answer?
2. **Scope honesty**: Can you tell whether the answer covered a file,
   notebook cell, loaded slice, active workset, or whole workspace?
3. **Freshness and health**: Can you tell whether the answer was live,
   cached, stale, warming, quarantined, blocked, or unavailable?
4. **Downgrade and alternates**: Can you tell what degraded, what
   alternate providers existed, and why they were not primary?
5. **Framework certainty**: If the answer made a framework-specific
   claim, can you tell whether it was proven, inferred, imported, stale,
   or unavailable, and what artifact or chain justified that label?
6. **Whole-workspace guardrail**: Is it structurally impossible for a
   partial, stale, blocked, or quarantined provider to present as
   whole-workspace authoritative truth?

If any answer above requires reading implementation code or inferring
hidden provider state from UI chrome, the surface is non-conforming.
