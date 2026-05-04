# Framework-certainty row, source-sync chip, and framework-aware surface contract

This document freezes how Aureline explains what runtime / framework
knowledge it has and where that knowledge came from, across every
framework-aware surface — route explorers, component trees, preview
cards, notebook framework-context inspectors, diagnostics lanes,
AI-assist context lanes, and support exports.

The goal is to keep later framework-pack, preview-runtime, and
notebook-runtime work from turning incidental adapter behavior into
product truth. A framework-aware surface that cannot prove its evidence
must say so in the same language a route-explorer row, a component-tree
row, a notebook framework-context inspector, and an AI-assist
explanation all read.

This contract sits **above** the cross-surface preview-snapshot record
and the language provider-graph contract, and **below** any consumer
surface chrome. It does not implement framework packs, preview
runtimes, or component-tree computation engines; it freezes how those
runtimes' results are allowed to surface.

## Companion artifacts

- [`/schemas/framework/framework_certainty_row.schema.json`](../../schemas/framework/framework_certainty_row.schema.json)
  — boundary schema for the `framework_certainty_row_record` every
  framework-aware surface emits per subject.
- [`/schemas/framework/source_sync_chip.schema.json`](../../schemas/framework/source_sync_chip.schema.json)
  — boundary schema for the `source_sync_chip_record` every
  framework-aware surface composes with the certainty row to disclose
  source / runtime / hot-reload / target alignment.
- [`/fixtures/framework/framework_certainty_cases/`](../../fixtures/framework/framework_certainty_cases/)
  — worked corpus of certainty-row and source-sync-chip cases.

## Composes with

- [`/docs/language/provider_graph_and_arbitration_contract.md`](../language/provider_graph_and_arbitration_contract.md)
  — provider-family taxonomy, framework-certainty classes, primary-
  source classes, and source-of-certainty chains. This contract
  re-exports those vocabularies; it does not mint a parallel dialect.
- [`/docs/architecture/preview_runtime_contract.md`](../architecture/preview_runtime_contract.md)
  — cross-surface preview-snapshot record, source-sync state vocabulary,
  source-revision and runtime-identity blocks, hot-reload state, and
  device-target row. This contract projects from those records and
  never duplicates them.
- [`/docs/preview/preview_runtime_surface_contract.md`](../preview/preview_runtime_surface_contract.md)
  — preview-runtime strip, device-target descriptor, and hot-reload
  state surface contract. The framework source-sync chip is a
  framework-side projection of the same source-sync, runtime-revision,
  hot-reload, and target-device truth.
- [`/docs/notebooks/output_viewer_truth_contract.md`](../notebooks/output_viewer_truth_contract.md)
  — notebook-output truth contract. Notebook framework-context rows
  reuse the notebook-binding vocabulary.
- [`/docs/product/launch_language_bundle_rubric.md`](../product/launch_language_bundle_rubric.md)
  — language and framework-pack rubric. Framework certainty rows MUST
  cite a framework pack covered by the rubric before claiming
  `framework_proven`.

If this document disagrees with the PRD, TAD, TDD, or the linked
contracts above, those documents win and this document plus the
companion schemas update in the same change.

## Why freeze this now

Framework-aware surfaces are one of the highest-risk places for
accidental product truth:

- a route-explorer row can advertise a route with a confidently named
  handler when only generic project-graph evidence existed;
- a component tree can render a polished tree from inferred
  language-server data and look indistinguishable from one extracted
  from a framework runtime;
- a preview card can claim a runtime is "synced" when only the source
  side of the alignment is known;
- a notebook framework-context inspector can synthesize a "we believe
  this notebook is using framework X" claim out of imported metadata
  alone;
- AI-assist explanations can quote framework-pack-shaped facts even
  when no framework pack actually ran.

The PRD and the language provider-graph contract already require honest
framework certainty. This contract turns that requirement into one
frozen surface contract so route, component-tree, preview, notebook,
diagnostics, and AI lanes share one row, one chip, and one fallback
vocabulary instead of inventing per-surface "auto-detected", "best
guess", or "stale cache" copy.

## Scope

Frozen at this revision:

- the `framework_certainty_row_record` shape, its six required subject
  kinds (detected framework, route mapping, component-tree node,
  runtime target, source synchronization, fallback / inference
  reason), and the rule that every framework-aware surface MUST emit
  one row per subject before rendering a framework claim;
- the six framework-certainty classes re-exported from the language
  provider-graph contract (`framework_not_applicable`,
  `framework_proven`, `framework_inferred`, `framework_imported`,
  `framework_stale`, `framework_unavailable`);
- the eight primary-source classes re-exported from the same contract;
- the closed list of fallback / inference reasons every non-proven row
  MUST cite;
- the closed framework-adapter family re-exported from the preview-
  snapshot record so route, component-tree, and notebook surfaces
  cannot mint new adapter labels;
- the `source_sync_chip_record` shape, its required disclosure floor
  (preview lane, source-sync state, source revision, runtime
  revision, hot-reload status, target device / context, fallback-open
  actions), and the rule that every chip offers at least one
  fallback-open action;
- the rule that `framework_proven` requires a framework-pack primary
  source, a non-empty proving-artifact list, and at least one
  framework-pack entry in the source-of-certainty chain;
- the redaction floor on every record: raw URLs, raw absolute paths,
  raw hostnames, raw IP addresses, raw device serial numbers, raw
  bearer tokens, raw session cookies, raw rendered bytes, raw stack
  frames, raw notebook payloads, and raw secret material never cross
  any of these boundaries.

Out of scope (named explicitly so the schemas do not creep):

- implementing framework packs, preview runtimes, or component-tree
  computation engines;
- final framework-pack scoring, route resolution, or component-tree
  diff algorithms beyond the eligibility and honesty rules frozen
  here;
- concrete UI layout, iconography, or animation for chips, badges,
  rows, or inspectors;
- the visual-edit transform manifest schema (only opaque refs are
  carried);
- per-framework adapter pipelines or runtime telemetry shapes.

## 1. Framework-certainty row

Every framework-aware surface that wants to render a framework claim
MUST emit one `framework_certainty_row_record` per claim. A surface
may emit several rows per render (one for the detected framework, one
for each route shown, one for each component-tree node shown, one for
the bound runtime target, one for the source-sync subject, and one for
each fallback / inference reason worth disclosing).

### 1.1 Subject kinds

| `subject_kind_class` | What the row answers |
|---|---|
| `detected_framework_subject` | What framework do we believe this code is using, and on what evidence? |
| `route_mapping_subject` | What handler / page does this URL or route pattern resolve to, and on what evidence? |
| `component_tree_node_subject` | What component-tree node do we know about, and on what evidence? |
| `runtime_target_subject` | What runtime / preview target is the framework currently bound to? |
| `source_synchronization_subject` | Is the source we know about aligned with the runtime that is currently rendering? |
| `fallback_inference_reason_subject` | Why did the row land below `framework_proven`? |

`fallback_inference_reason_subject` rows exist only to disclose a
downgrade. They are not allowed to claim `framework_proven` and they
are not allowed to leave `fallback_inference_reason_class` at the
no-fallback sentinel.

### 1.2 Surface vocabulary

The same row contract is read by every framework-aware surface:

- `route_explorer_surface`
- `component_tree_surface`
- `preview_card_surface`
- `notebook_framework_context_surface`
- `diagnostics_lane_surface`
- `ai_assist_context_surface`
- `support_export_surface`

`notebook_framework_context_surface` rows MUST carry a non-null
`notebook_binding_block` (notebook document ref, optional cell ref,
optional kernel session ref, notebook notes). Other surfaces MAY carry
a notebook-binding block, but they are not required to.

### 1.3 Framework-certainty classes

Re-exported verbatim from the language provider-graph contract. The
contract does not mint a parallel framework-certainty dialect.

| `framework_certainty_class` | Meaning |
|---|---|
| `framework_not_applicable` | This row makes no framework-specific claim. |
| `framework_proven` | Proven by framework-pack analysis or a framework-pack-declared proving artifact. |
| `framework_inferred` | Useful framework guess derived from graph / LSP / build evidence but not framework-proven. |
| `framework_imported` | Framework fact imported from a captured snapshot or external artifact. |
| `framework_stale` | Former framework signal exists but is below freshness requirements. |
| `framework_unavailable` | No admissible framework certainty is available. |

### 1.4 Primary-source classes

Re-exported verbatim from the language provider-graph contract.

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

### 1.5 Fallback / inference reasons

Every row that is not `framework_proven` MUST cite a concrete
fallback / inference reason. The closed vocabulary lets framework
surfaces explain downgrades in one shared language instead of per-
surface free text.

- `no_fallback_proven_directly` — reserved for `framework_proven` rows.
- `framework_pack_unavailable` — no framework pack is loaded for this
  workspace / file.
- `framework_pack_capability_missing` — the framework pack is loaded
  but does not support this subject.
- `proving_artifact_missing` — the framework pack ran but did not
  surface a proving artifact for this subject.
- `proving_artifact_stale` — a proving artifact exists but is past its
  freshness floor.
- `language_server_only_evidence` — only LSP evidence backs this row;
  framework-pack evidence is missing.
- `build_adapter_only_evidence` — only build-adapter evidence backs
  this row.
- `project_graph_only_evidence` — only project-graph projection backs
  this row.
- `generic_language_signal_only` — only generic syntax / structural
  signal backs this row.
- `imported_snapshot_only_evidence` — only an imported snapshot backs
  this row.
- `ai_inference_only_evidence` — only AI inference backs this row.
- `source_revision_drifted` — framework evidence existed but the
  source revision moved.
- `runtime_unreachable` — the framework runtime / preview is
  unreachable.
- `notebook_kernel_not_consulted` — the notebook kernel was not
  available, so notebook-aware framework facts were not derived from
  runtime evidence.
- `policy_or_trust_narrowed` — workspace policy / trust narrowed the
  admissible evidence.

### 1.6 Proving artifacts and source-of-certainty chain

Every row carries:

- `proving_artifact_refs` — opaque refs to framework-pack-typed proving
  artifacts. `framework_proven` requires a non-empty list.
  `framework_unavailable` and `framework_not_applicable` require an
  empty list.
- `source_of_certainty_chain` — entries from the closed certainty-
  source vocabulary re-exported from the language provider-graph
  contract (`framework_pack_analysis`, `framework_pack_artifact`,
  `project_graph_fact`, `language_server_fact`, `syntax_fact`,
  `build_adapter_fact`, `notebook_adapter_projection`,
  `generated_source_lineage`, `imported_snapshot`, `ai_inference`,
  `user_curated_override`).

`framework_proven` requires the chain to contain at least one
`framework_pack_analysis` or `framework_pack_artifact` entry. A chain
that consists only of `language_server_fact`, `build_adapter_fact`,
`syntax_fact`, `imported_snapshot`, or `ai_inference` entries is
admissible for `framework_inferred` / `framework_imported` /
`framework_stale` / `framework_unavailable` rows but never for
`framework_proven`. This rule blocks the most common quiet-truth
failure: a route or component-tree row claiming framework-pack
authority while only LSP or graph evidence ran.

### 1.7 Subject-specific blocks

`route_mapping_subject` rows MUST carry a `route_mapping_block`
declaring the `route_mapping_class`:

| `route_mapping_class` | Admissible certainty class |
|---|---|
| `route_mapped_to_handler_proven` | `framework_proven` only |
| `route_mapped_to_handler_inferred` | `framework_inferred` |
| `route_mapped_from_imported_evidence` | `framework_imported` |
| `route_handler_unknown` | not `framework_proven` |
| `route_pattern_unknown_to_runtime` | not `framework_proven` |
| `route_blocked_no_runtime` | not `framework_proven` |
| `route_stale_against_current_source` | `framework_stale` |

`component_tree_node_subject` rows MUST carry a
`component_tree_block` declaring the `component_tree_node_class`:

| `component_tree_node_class` | Admissible certainty class |
|---|---|
| `component_node_proven` | `framework_proven` only |
| `component_node_inferred_from_graph` | `framework_inferred` |
| `component_node_inferred_from_lsp` | `framework_inferred` |
| `component_node_imported` | `framework_imported` only |
| `component_node_stale` | `framework_stale` only |
| `component_node_unmappable` | `framework_inferred` or `framework_unavailable` |

`runtime_target_subject` rows MUST carry a non-null
`runtime_target_block` (target-environment class, optional sandbox-
posture class, optional preview-snapshot or execution-context refs).

`source_synchronization_subject` rows MUST carry a non-null
`source_sync_chip_record_ref`. The chip carries the actual source-
revision / runtime-revision / hot-reload / target-device truth; the
row carries the certainty wrapper.

## 2. Source-sync chip

Every framework-aware surface that wants to disclose source-vs-runtime
alignment MUST emit one `source_sync_chip_record`. The chip is the
framework-side projection of the cross-surface preview-snapshot
source-sync state, the preview-runtime-strip source-revision chip, and
the hot-reload state record. It exists so the same alignment story
travels through route-explorer rows, component-tree rows, preview
cards, notebook framework-context inspectors, diagnostics lanes, AI-
assist context lanes, and support exports without each surface re-
inventing "stale", "drifted", "unattached", or "unknown" labels.

### 2.1 Required fields

- `preview_lane_class` — `browser_preview_lane`, `native_preview_lane`,
  `embedded_preview_lane`, `notebook_preview_lane`, or
  `no_preview_lane_attached`. The last value is reserved for chips
  emitted from diagnostics / AI / support surfaces with no preview
  attached.
- `source_sync_state_class` — re-export of the cross-surface source-
  sync vocabulary (`source_in_sync`, `source_drifted_since_render`,
  `source_revision_unknown`, `source_revision_redacted`,
  `source_unmappable_for_runtime`).
- `source_revision_block` — `source_revision_class`, optional
  `source_revision_ref`, optional `drifted_paths_count`. The class is
  the cross-surface vocabulary (`vcs_commit_pinned`,
  `working_tree_with_pending_edits`, `release_tag_pinned`,
  `build_artifact_digest_pinned`, `remote_ref_pinned`,
  `unknown_revision`).
- `runtime_revision_block` — `runtime_revision_class`, optional
  `runtime_revision_ref`, optional `framework_adapter_revision_ref`.
- `hot_reload_status_class` — re-export of the six-class hot-reload
  vocabulary (`applied`, `partial`, `restart_required`,
  `rebuild_required`, `failed`, `unavailable`).
- `target_device_context_block` — `device_target_class`,
  `viewport_preset_class`, `target_environment_class`, optional
  `device_handle_ref`, optional reviewer-facing label.
- `fallback_open_actions` — non-empty array from the closed action
  vocabulary in §2.4.

### 2.2 Runtime-revision states

| `runtime_revision_class` | Meaning |
|---|---|
| `runtime_revision_pinned_to_source` | The running framework runtime was built against the recorded source revision. Only admissible when `source_sync_state_class = source_in_sync`. |
| `runtime_revision_drifted_from_source` | The runtime was built against an older source revision. |
| `runtime_revision_unknown` | The runtime did not report its revision. |
| `runtime_revision_redacted` | Reserved for support-export contexts where revision identity is intentionally projected as opaque. |
| `runtime_revision_unattached_no_runtime` | No runtime is currently attached. Admissible only for static previews, notebook framework-context chips without an attached kernel, and diagnostics / AI / support surfaces without a preview. |

### 2.3 Target device / context

The target-device-context block re-exports the cross-surface device-
target vocabulary plus `no_device_target_attached` and
`no_runtime_target_attached` for chips emitted from surfaces with no
preview attached. Viewport presets remain visibly distinct from real
runtime targets, mirroring the device-target descriptor partition in
the preview-runtime surface contract.

### 2.4 Fallback-open actions

Every chip offers at least one fallback-open action so the user is
never given a dead-end label. The closed vocabulary:

- `no_action_already_in_sync` — reserved for `source_in_sync` chips.
- `open_canonical_source` / `open_diff_against_source` — jump to
  source.
- `open_runtime_inspector` / `open_runtime_logs` — open runtime
  evidence.
- `export_metadata_only` — escalate by exporting metadata only.
- `request_managed_runtime` — request a managed runtime when none is
  available.
- `reload_preview_runtime` / `rebuild_preview_runtime` /
  `restart_preview_runtime` / `reattach_runtime_session` — runtime
  recovery.
- `reveal_in_route_explorer` / `reveal_in_component_tree` /
  `open_notebook_kernel_inspector` — route to framework consumers.
- `open_framework_pack_status` — open the framework-pack status
  surface for diagnosis.
- `request_policy_review` — escalate when policy / trust narrowed
  evidence.

The schema enforces:

- `source_in_sync` requires `runtime_revision_pinned_to_source` and
  `no_action_already_in_sync` in the action set.
- `source_drifted_since_render` requires at least one of
  `open_canonical_source`, `open_diff_against_source`, or
  `reload_preview_runtime`, and forbids `no_action_already_in_sync`.
- `source_revision_unknown` forbids `no_action_already_in_sync`.
- `source_unmappable_for_runtime` requires at least one of
  `open_runtime_inspector` or `export_metadata_only`.
- `hot_reload_status_class = failed` requires at least one of
  `open_runtime_logs`, `restart_preview_runtime`, or
  `rebuild_preview_runtime`.
- `restart_required` requires `restart_preview_runtime`.
- `rebuild_required` requires `rebuild_preview_runtime`.
- `open_runtime_logs` requires a non-null `runtime_log_ref`.
- `request_managed_runtime` requires a non-null
  `managed_runtime_request_ref`.
- `open_framework_pack_status` requires a non-null
  `framework_pack_status_ref`.
- `no_preview_lane_attached` chips require
  `target_environment_class = no_runtime_target_attached`,
  `device_target_class = no_device_target_attached`, and
  `runtime_revision_class = runtime_revision_unattached_no_runtime`.

## 3. Framework-aware surfaces

Each surface projects the same row + chip pair. Surfaces MAY render
the same record differently, but they MUST NOT invent private
framework-knowledge fields.

| Surface | Required reads |
|---|---|
| Route explorer | one `route_mapping_subject` row per route, one chip per visible runtime target. |
| Component tree | one `component_tree_node_subject` row per node (or per top-level subtree), one chip per visible runtime target. |
| Preview card | one `detected_framework_subject` row, one `runtime_target_subject` row, one `source_synchronization_subject` row, plus the chip the source-sync row references. |
| Notebook framework-context inspector | one `detected_framework_subject` row with a notebook-binding block, plus any of the other subject rows when the inspector exposes routes / components / runtime / sync. |
| Diagnostics lane | one row per framework-specific diagnostic, citing the relevant subject. |
| AI-assist context lane | one row per quoted framework fact. AI-assist context rows remain advisory; they reuse the same vocabulary but do not become authoritative through ranking. |
| Support export | every row + chip emitted by the surfaces above MUST round-trip through the support export with the same vocabulary. |

## 4. Acceptance checklist

A reviewer can audit conformance without implementation code:

1. **Subject identity.** Can you tell which subject the row is making
   a claim about (detected framework, route mapping, component-tree
   node, runtime target, source synchronization, or fallback /
   inference reason)?
2. **Certainty class.** Can you tell whether the row is
   `framework_proven`, `framework_inferred`, `framework_imported`,
   `framework_stale`, `framework_unavailable`, or
   `framework_not_applicable`?
3. **Primary source.** Can you tell whether the evidence came from
   framework-pack analysis, a framework-pack proving artifact,
   project-graph projection, language-server signal, build-adapter
   signal, an imported snapshot, AI inference, or none?
4. **Fallback reason.** If the row is not `framework_proven`, can you
   tell which fallback / inference reason applied?
5. **Source / runtime alignment.** If a chip is present, can you tell
   the source revision class, runtime revision class, hot-reload
   status, and target environment / device class?
6. **Fallback-open action.** If a chip is present, can you tell which
   fallback-open action(s) are admissible without reading
   implementation code?
7. **Whole-framework guardrail.** Is it structurally impossible for a
   language-server-only, build-adapter-only, project-graph-only,
   imported, or AI-only chain to surface as `framework_proven`?

If any answer above requires reading implementation code or inferring
hidden framework-pack state from UI chrome, the surface is non-
conforming.

## 5. Worked fixture corpus

The companion fixture set covers the minimum required cases:

- `fully_proven_framework_route.yaml` — `framework_proven`
  `route_mapping_subject` row backed by framework-pack analysis with
  `route_mapped_to_handler_proven` and a paired in-sync source-sync
  chip.
- `inferred_component_tree_partial_sources.yaml` —
  `framework_inferred` `component_tree_node_subject` row with
  `component_node_inferred_from_graph` and a project-graph-only
  source-of-certainty chain.
- `stale_preview_mapping.yaml` — `framework_stale`
  `source_synchronization_subject` row paired with a
  `source_drifted_since_render` chip routing to canonical source and
  a runtime restart action.
- `notebook_bound_framework_context.yaml` — `framework_inferred`
  `detected_framework_subject` row on the notebook framework-context
  surface with a notebook-binding block citing
  `kernel_runtime_not_consulted` and a paired
  `no_preview_lane_attached` chip.
- `no_framework_certainty.yaml` — `framework_unavailable`
  `fallback_inference_reason_subject` row citing
  `generic_language_signal_only` so a strong framework claim is
  structurally blocked when only generic language or build evidence
  exists.
- `source_sync_chip_in_sync.yaml` — `source_in_sync`
  `source_sync_chip_record` for the live, in-sync browser-lane case.
- `source_sync_chip_runtime_unattached.yaml` — chip emitted from a
  diagnostics surface with `no_preview_lane_attached`,
  `runtime_revision_unattached_no_runtime`, and an
  `open_canonical_source` fallback action.
