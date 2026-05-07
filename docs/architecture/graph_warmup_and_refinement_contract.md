# Graph-shard reuse, semantic warm-up, and refinement disclosure contract

This document freezes the cross-surface disclosure and lineage contract
for **graph-shard reuse** and **semantic refinements that arrive later**
during startup, reopen, partial-index, and degraded-provider states.

Without this contract, early navigation and language-intelligence can
silently present:

- cached graph answers as if they were freshly computed;
- metadata-only or structural fallbacks as if they were complete semantic truth;
- late refinements that reorder, replace, or redirect prior results without an
  inspectable supersession trail.

The goal is simple: **provisional answers must never masquerade as live semantic
truth**, and later refinements must be **disclosed, attributable, and
reconstructable** by support and benchmark traces.

Machine-readable companion:

- [`/artifacts/search/semantic_refinement_states.yaml`](../../artifacts/search/semantic_refinement_states.yaml)
  — frozen reuse classes, invalidation reasons, disclosure expectations, and
  fixture pointers.

This contract composes with and does not replace:

- Startup ordering and readiness cues:
  [`/artifacts/startup/cold_start_ordering_packet.md`](../../artifacts/startup/cold_start_ordering_packet.md),
  [`/artifacts/startup/startup_admission_order.yaml`](../../artifacts/startup/startup_admission_order.yaml),
  [`/docs/recovery/restore_hydration_phases_contract.md`](../recovery/restore_hydration_phases_contract.md).
- Search truth vocabulary and streaming replacement lineage:
  [`/schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json),
  [`/artifacts/search/result_truth_labels.yaml`](../../artifacts/search/result_truth_labels.yaml),
  [`/docs/search/query_planner_contract_seed.md`](../search/query_planner_contract_seed.md).
- Semantic navigation identity, rename-preview posture, and scope caveats:
  [`/docs/navigation/semantic_navigation_and_rename_contract.md`](../navigation/semantic_navigation_and_rename_contract.md),
  [`/schemas/navigation/semantic_result_ref.schema.json`](../../schemas/navigation/semantic_result_ref.schema.json),
  [`/schemas/navigation/rename_preview.schema.json`](../../schemas/navigation/rename_preview.schema.json).
- Provider arbitration and cached reuse disclosure:
  [`/docs/language/provider_graph_and_arbitration_contract.md`](../language/provider_graph_and_arbitration_contract.md).
- Cross-surface semantic-readiness vocabulary (exact/imported/heuristic/stale/partial):
  [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md),
  [`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json).

Where this contract disagrees with the PRD, architecture/design documents, or
the upstream contracts above, those sources win and this document plus its
companion artifact MUST be updated in the same change.

## Scope

Frozen at this revision:

- Graph-shard reuse classes and reuse rejection / invalidation reasons.
- The disclosure rules for **provisional navigation** across:
  quick open, breadcrumbs, symbol navigation, rename-preview, and code-action
  suggestion/review surfaces.
- The required **refinement lineage** when later computation supersedes earlier
  answers (replacement, reorder, or redirect posture).

Out of scope:

- Final ranking algorithms, per-language provider implementation details, or
  any specific language server protocol behavior.
- UI layout, colors, or per-platform rendering implementation.
- Defining new search or navigation vocabularies that are already frozen in
  the referenced schemas.

## Terms (normative)

### Graph shard

A **graph shard** is a derived, discardable, versioned semantic graph snapshot
for a workspace/config/scope family. It is an accelerator and may be reused, but
it is never allowed to silently upgrade a claim.

### Provisional answer

A **provisional answer** is any result that is not backed by the **live semantic
compute** required for the declared scope and authority floor. Provisional does
not mean useless; it means **caveated**. Provisional answers MUST carry:

- an explicit truth/freshness/scope caveat in the surface’s canonical packet; and
- a lineage mechanism that allows a later refinement to supersede it without
  silent disappearance.

### Semantic refinement

A **semantic refinement** is any late-arriving update that:

- replaces a previously rendered result identity;
- materially changes ordering of an already rendered set; or
- changes the navigation target that would be opened by a prior selection.

Refinements are expected during startup and after reopen. They are allowed only
when the supersession is **explicit**.

## Graph-shard reuse classes (normative)

This contract names four reuse classes that appear across search, navigation,
and language-intelligence lanes. These are not new schema enums; they are a
**contracted mapping** onto existing truth/freshness/scope fields.

### 1) Metadata-only (no semantic graph)

Allowed when:

- startup / reopen before graph warm-up completes;
- partial-index or degraded-provider states where a truthful fallback exists.

Disclosure requirements:

- Search surfaces MUST express non-semantic posture through the search truth
  vocabulary (readiness + partial-truth disclosure) and MUST NOT mint “Ready”
  copy that implies whole-workspace semantic coverage.
- Navigation/language-intelligence surfaces MUST express fallback posture through
  provider family/health and confidence/completeness (e.g. syntax/structural
  fallback remains labeled as such).

### 2) Cached-graph reuse (persisted shard)

Allowed when:

- the shard is compatible with the current workspace/config identity; and
- freshness is disclosed as cached/stale/partial where applicable.

Disclosure requirements:

- Cached reuse MUST NOT claim `authoritative_live` freshness unless the producer
  has revalidated the shard against the current identity and freshness floor.
- Cached reuse MUST keep the source visible (provider family and cached reuse
  downgrade cues); it MUST NOT repaint cached graph answers as fresh local
  semantic compute.

### 3) Hybrid (mixed cached + live + metadata)

Allowed when:

- more than one contributor lane participates (e.g. hot-set lexical plus cached
  graph, or cached graph plus a live provider for a narrower scope).

Disclosure requirements:

- Rows that combine contributors MUST remain mechanically expressible as mixed
  provenance (search uses `result_truth_class = hybrid` plus contributor reasons;
  navigation uses provider provenance + explicit scope/completeness caveats).
- Hybrid results MUST NOT silently collapse into a single “exact” presentation
  if any contributor is non-authoritative for the declared scope.

### 4) Live semantic compute

Allowed when:

- the chosen provider lane is `ready`, `authoritative_live`, and complete for
  the declared scope as required by the provider arbitration contract.

Disclosure requirements:

- Live semantic readiness MAY lag behind shell readiness; surfaces MUST keep the
  “semantic refinements arrive later” posture explicit during warm-up.

## Reuse rejection and invalidation (normative)

Graph-shard reuse is best-effort. When reuse is rejected or invalidated, the
surface MUST preserve **why** it happened in export-safe form and MUST fall back
to an admissible provisional posture rather than pretending the shard never
existed.

The frozen invalidation reasons and their required disclosure mapping live in:

- [`/artifacts/search/semantic_refinement_states.yaml`](../../artifacts/search/semantic_refinement_states.yaml)

At minimum, rejection/invalidation must distinguish:

- no shard present;
- shard schema/version mismatch;
- workspace/config identity mismatch;
- stale input digests (source changed since capture);
- policy/trust posture mismatch;
- shard corrupt/quarantined;
- remote dependency unreachable (cached-only fallback allowed with disclosure).

## Surface rules: when provisional results are allowed (normative)

The rule is not “never show anything until semantic is ready”.
The rule is “show useful partial answers early, but label them and preserve
lineage.”

### Quick open / search result lists

Allowed provisional posture:

- Hot-set or partial lexical results are allowed early.
- Cached graph or semantic supplement is allowed only under explicit disclosure.

Refinement lineage:

- When a later refinement supersedes previously streamed rows, search MUST use
  the streaming frame lineage rules from the query-planner seed:
  `corrective_replace_frame` is the only frame kind that may invalidate prior
  rows, and it MUST enumerate `superseded_fused_row_ids` rather than silently
  dropping them. See
  [`/docs/search/query_planner_contract_seed.md`](../search/query_planner_contract_seed.md).

User-visible effect requirements:

- A readiness/truth label MUST change only when the underlying packet fields
  changed, and the surface MUST be able to show “what changed” via the canonical
  explainability/truth packet paths (no private relabeling).

### Breadcrumbs / compact chrome

Allowed provisional posture:

- Breadcrumb segments may render from cached graph or structural fallback while
  warm-up is pending.

Requirements:

- The breadcrumb’s semantic identity MUST remain caveated when derived from a
  cached/partial graph. Compact chrome may hide detail, but it MUST be able to
  reach an inspector that shows freshness, scope, and provenance using the
  canonical schemas (no breadcrumb-only private truth model).

### Symbol navigation (definition/reference/hierarchy)

Allowed provisional posture:

- Structural or cached fallback is allowed when the graph/provider is not yet
  complete for the declared scope, as long as the result is explicitly labeled
  partial/stale/heuristic and carries scope limits.

Refinement lineage:

- A later live-semantic resolution MUST NOT silently reinterpret the prior
  navigation artifact. If a refinement changes the recommended target, the
  surface must treat it as a new result with an explicit “supersedes” lineage
  (history, review, support export, and AI citations must continue to point at
  the durable id they originally observed).

### Rename-preview and refactor surfaces

Allowed provisional posture:

- A rename may be *previewed* under partial/stale/cached graph posture, but it
  MUST NOT become “silent apply”. A preview must make omissions, scope limits,
  and freshness explicit.

Requirements:

- A rename driven from cached/stale/partial semantic state MUST force an
  inspectable preview posture before apply, and MUST carry the provider freshness
  and completeness caveats required by the semantic navigation contract and the
  refactor transaction contract.

### Code actions and quick fixes

Allowed provisional posture:

- Suggestions may be offered under degraded/cached/partial semantic posture, but
  they MUST remain advisory or review-gated when the provider cannot satisfy the
  declared authority floor.

Requirements:

- Late refinements that change edit safety, scope coverage, or affected targets
  MUST be expressed as a new advisory/preview packet; they MUST NOT silently
  flip a previously “safe” action into a broader mutation without a renewed
  review step.

## Refinement events: what must change and what must not (normative)

### 1) Disclosed label changes only

Surfaces MUST NOT “upgrade” a label in place without a corresponding change in
the canonical packet fields they export. If a refinement changes readiness,
truth, freshness, or scope, the surface must:

- emit a new packet/frame that carries the new fields; and
- preserve the predecessor/supersession linkage so support and traces can
  reconstruct the sequence.

### 2) Stable identities and explicit supersession

Whenever a provisional result is later replaced, the system must retain enough
identity to answer:

- which provisional row was shown;
- why it was admissible at the time;
- which later refinement superseded it; and
- what disclosure/label changed at the moment of supersession.

Search uses fused-row supersession (`superseded_fused_row_ids`) and stable result
identity keys. Navigation uses durable semantic result ids and navigation
artifact ids; later refinements do not retroactively rewrite prior ids.

### 3) No silent redirects on reopen targets

If a user opened a target from a provisional answer (quick open, symbol jump,
breadcrumb segment), a later semantic refinement MUST NOT silently redirect the
already-open editor/tab/history entry to a different target. The refined target
may be presented as an explicit alternate/superseding option.

## Fixture coverage

Worked cases that exercise this contract live under:

- [`/fixtures/search/partial_semantic_warmup_cases/`](../../fixtures/search/partial_semantic_warmup_cases/)

They cover:

- no shard present;
- stale shard reuse with explicit caveats;
- incompatible shard rejection with fallback to metadata-only;
- partially warmed roots (mixed readiness within one scope);
- late semantic replacement of earlier hot-set candidates with explicit
  supersession lineage.

## Change management

- This contract mints no new schema enums. Any new token required by a surface
  must be introduced in the schema that owns that vocabulary, with fixtures and
  decision updates as required by that schema’s change rules.
- The companion YAML (`artifacts/search/semantic_refinement_states.yaml`) is the
  machine-readable index for reuse classes, invalidation reasons, and fixture
  coverage. Adding a new row is additive-minor; repurposing an existing row is
  breaking and requires a supersession note in the updated row.

