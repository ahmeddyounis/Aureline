# Migration equivalence map and category parity scorecard contract

This document freezes two migration-truth artifacts that keep switching
confidence reviewable instead of collapsing into a single optimistic “import
worked” result:

1. The **equivalence map** — row-level, reusable classification of how each
   imported object relates to its destination representation.
2. The **category parity scorecard** — per-category confidence summaries with
   evidence refs and known-gap links so strong parity in one category cannot
   mask weakness in another.

These artifacts are designed to be reusable across first-run onboarding,
migration center, docs/help, support exports, and design-partner review without
reclassification.

Companion artifacts:

- [`/schemas/migration/equivalence_map_row.schema.json`](../../schemas/migration/equivalence_map_row.schema.json)
  — machine-readable contract for `equivalence_map_row_record` and
  `equivalence_map_packet_record`.
- [`/schemas/migration/parity_scorecard.schema.json`](../../schemas/migration/parity_scorecard.schema.json)
  — machine-readable contract for `parity_scorecard_row_record` and
  `parity_scorecard_packet_record`.
- [`/fixtures/migration/equivalence_cases/`](../../fixtures/migration/equivalence_cases/)
  — worked equivalence-map rows covering exact, translated, approximated,
  shimmed, unsupported, skipped-by-policy, manual-review, and bundle-handoff
  scenarios.

This contract composes with (and does not replace):

- [`/docs/migration/migration_center_object_model.md`](./migration_center_object_model.md)
  and [`/schemas/migration/importer_outcome.schema.json`](../../schemas/migration/importer_outcome.schema.json)
  — durable migration-session and importer-outcome packets. Outcome states
  remain the primary closed vocabulary for surfaces.
- [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](./first_run_import_diff_and_rollback_contract.md)
  plus the import plan/preview/rollback schemas — preview/apply/rollback
  discipline for importer-driven writes.
- [`/docs/migration/source_ecosystem_coverage_matrix.md`](./source_ecosystem_coverage_matrix.md)
  and [`/artifacts/migration/quality_bar_rubric.yaml`](../../artifacts/migration/quality_bar_rubric.yaml)
  — governed source lanes and the evidence burden requiring category-specific
  parity scores.
- [`/docs/migration/compatibility_scorecard_contract.md`](./compatibility_scorecard_contract.md)
  and [`/schemas/migration/compatibility_scorecard.schema.json`](../../schemas/migration/compatibility_scorecard.schema.json)
  — imported-extension / imported-workflow scorecards that equivalence rows and
  parity scores may cite when bridge, partial, blocked, or replacement posture
  applies.

If this document disagrees with the PRD, technical design, or UI/UX spec,
those sources win and this contract plus the companion schemas MUST update in
the same change.

Out of scope: implementing importers, choosing final parity weights, and
shipping UI copy for every row. This document freezes the vocabulary and
packet shapes those surfaces will read.

## 1. Equivalence map

The equivalence map is the durable, reuse-oriented classification of how a
source object maps (or fails to map) into Aureline’s destination object model.

It exists so:

- unsupported or approximated items remain visible in preview, final reports,
  and follow-up surfaces rather than disappearing after apply; and
- onboarding, docs/help, support, and design-partner review can cite the same
  row id and semantics without reclassifying outcomes.

### 1.1 Equivalence outcome vocabulary (closed)

Equivalence rows use one closed `equivalence_outcome` set:

| Outcome | Meaning |
|---|---|
| `exact` | The destination can carry the source object without semantic narrowing beyond documented canonicalization. |
| `translated` | A semantic equivalent exists and the importer can map the object into a destination-native representation with high confidence. |
| `approximated` | A best-effort mapping exists but is caveated or narrower than the source behavior; the caveat must remain attached to the row. |
| `shimmed` | The mapping requires an explicit bridge/shim path; native parity is not claimed and the shim remains visible as such. |
| `unsupported` | No safe destination representation exists for the source concept. |
| `skipped_by_policy` | The importer intentionally did not import the object because policy/trust/permission/egress ceilings forbid it. |
| `needs_manual_review` | A human decision is required before the row may be considered complete (conflict, ambiguity, or insufficient evidence). |

### 1.2 Relationship to importer-outcome rows

Importer-outcome rows remain the primary surface vocabulary (the six closed
`outcome_state` values). Equivalence rows add the reusable parity nuance that
survives export, docs, and support handoff.

Conforming mappings:

- `exact` → `outcome_state: imported` + `mapping_basis: exact_identity`
- `translated` → `outcome_state: mapped` + `mapping_basis: semantic_equivalent` or `capability_based`
- `approximated` → `outcome_state: mapped` + a required `caveat` and a
  confidence below “high” unless the evidence ref set is strong enough to
  justify it
- `shimmed` → `outcome_state: bridge_required` + a required `shim_requirement`
- `unsupported` → `outcome_state: unsupported`
- `skipped_by_policy` → `outcome_state: skipped` + `reason_class: policy_excludes_import`
- `needs_manual_review` → `outcome_state: manual_review`

Surfaces MAY render `equivalence_outcome` as a secondary chip or detail line,
but they MUST NOT replace the closed importer-outcome vocabulary with ad hoc
terms.

### 1.3 Publication rules (required)

1. Any equivalence row with `approximated`, `shimmed`, `unsupported`,
   `skipped_by_policy`, or `needs_manual_review` MUST remain visible in:
   - preview (dry-run diff),
   - final migration report/export, and
   - post-import follow-up surfaces.
2. A surface MAY summarize equivalence rows, but it MUST preserve:
   - the strongest non-`exact` outcome present, and
   - stable refs to the underlying rows so support and docs can reconstruct
     the exact set.
3. Extension and workflow imports SHOULD cite compatibility scorecard rows
   when bridge/blocked/partial/replaced posture applies; equivalence rows do
   not replace scorecards.

## 2. Category parity scorecard

Parity scores turn a migration session’s row-level truth into a per-category
confidence summary that remains evidence-backed and reviewable.

The scorecard exists to prevent an aggregate “success” result from hiding weak
categories (for example: strong keymap parity masking weak run/debug
translation).

### 2.1 Category set (closed)

The scorecard uses one category vocabulary aligned with migration parity
expectations:

- `keymap`
- `theme_and_visuals`
- `settings`
- `snippets_and_templates`
- `tasks_and_run_configs`
- `launch_debug`
- `extensions_and_providers`
- `other`

### 2.2 Required fields per category

Each category row MUST carry:

- a numeric `score` (0–100),
- a `confidence_class`,
- `evidence_refs` (one or more stable refs),
- `known_gap_refs` (optional but required when a category score would otherwise
  hide a known blocker), and
- an export-safe `label` (`label_class` + `label_summary`) that is safe to
  carry into support bundles and machine-readable exports.

Scorecards MUST NOT emit an overall score field. Surfaces MAY compute their own
local summary UI, but exports and durable records do not collapse the categories
into one number.

### 2.3 Reserved linkage fields (non-breaking growth)

Both equivalence rows and parity score rows reserve optional fields for later
expansion without changing the core parity vocabulary:

- `bundle_suggestion_refs` (workflow bundle handoff candidates),
- `docs_help_refs` (stable docs/help pointers),
- `migration_guide_anchor_refs` (migration-guide deep links).

Consumers MUST treat these fields as optional and MUST NOT fail closed when
they are empty.

## 3. Competitor handoff contract (import-source specific)

This section freezes the competitor handoff vocabulary as it relates to the
equivalence map and parity scorecard. It does not implement import adapters.

### 3.1 VS Code / Code-OSS import

Required behavior:

- settings, shortcuts, snippets, tasks, launch/debug configs, themes where
  compatible, and compatible extension mappings all emit equivalence rows with
  explicit `exact` / `translated` / `approximated` / `shimmed` / `unsupported`
  outcomes;
- extension mappings that depend on a compatibility bridge are `shimmed` and
  MUST remain visible as bridge-backed, not as native parity;
- parity scoring MUST be category-specific, and the extensions/provider score
  MUST NOT be averaged into keymap or settings scores.

### 3.2 JetBrains-family import

Required behavior:

- keymaps and common navigation/editing habits are expected to be
  `translated`/`approximated` rather than uniformly `exact`;
- run/debug translation MUST surface `needs_manual_review` when the imported
  concept cannot be made runnable without execution-context review or missing
  target-side support;
- plugin runtime compatibility remains outside scope; imported plugin state is
  either `unsupported` or represented as a separate compatibility scorecard
  row with blocked posture.

### 3.3 Vim/Neovim preset import

Required behavior:

- modal-editing feel and high-frequency gestures should land as `exact` or
  `translated` where the destination has a native equivalent;
- arbitrary plugin execution, Lua runtime parity, and plugin state import are
  `unsupported` or `skipped_by_policy` (when policy forbids executing source
  artifacts), and MUST remain visible;
- parity scoring is still category-specific: strong modal/editing parity may
  not imply run/debug or extension parity.

### 3.4 Bundle / design-partner handoff import

Required behavior:

- bundle handoff rows MUST name the source signer/support posture through
  export-safe refs and MUST keep override behavior visible (bundle adoption is
  not silent mutation);
- bundle recommendations MUST NOT claim that the bundle perfectly reproduces
  the source tool when scorecards or equivalence rows say otherwise;
- drift and rollback posture remain visible through the same migration session
  and outcome packet linkage.
