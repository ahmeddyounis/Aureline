# Explorer / search row lineage-hint packet

This document freezes the **row-sized** projection of the generated,
mirrored, imported, and preview artifact posture that explorer trees,
quick-open rows, full search results, symbol-jump rows, docs-search
rows, cross-repo result groups, graph overlays, AI citations, and
support / export references all carry. It is the contract that keeps
derived files from silently rendering as canonical handwritten source
before the file is opened.

The artifact posture record frozen in
[`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
is the full per-object truth. The row-level hint record defined here is
what a list row, chip, or citation reads so that the origin class,
provenance state, default edit posture, canonical source ref, generator
identity, and follow-up actions travel with the row at every surface
the user sees *before* they open the object.

Companion artifacts:

- [`/schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
  — cross-surface posture record the row hint projects from. Every
  token the row hint uses (`artifact_class`, `artifact_origin_class`,
  `provenance_state`, `default_edit_posture`, `rebuild_intent_class`)
  is the same enum the posture record defines.
- [`/docs/generated/diverged_from_generator_contract.md`](./diverged_from_generator_contract.md)
  — contract for diverged-from-generator state and override-review gating; row
  hints rely on its invariants to forbid direct-edit offers on diverged,
  unknown-lineage, and mirror-controlled rows.
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  — safe-edit policy, artifact-class policy matrix, and surface
  obligations this packet attaches to.
- [`/fixtures/generated/drift_regeneration_manifest.yaml`](../../fixtures/generated/drift_regeneration_manifest.yaml)
  — drift / regeneration / viewer-fallback corpus. Each row-hint case
  in this packet joins to the posture corpus on `artifact_posture_ref`.
- [`/artifacts/generated/viewer_fallback_examples/`](../../artifacts/generated/viewer_fallback_examples/)
  — full posture records the row hint projects from.
- [`/fixtures/generated/lineage_hint_examples/lineage_hint_manifest.yaml`](../../fixtures/generated/lineage_hint_examples/lineage_hint_manifest.yaml)
  — reviewer-facing case manifest for row-level hints.
- [`/artifacts/generated/explorer_search_rows/`](../../artifacts/generated/explorer_search_rows/)
  — concrete `row_lineage_hint_record` examples for the required
  artifact classes.
- [`/schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json)
  — search-result packet the row hint rides alongside on search
  surfaces. A search surface reads its `search_result_packet_record`
  for readiness / ranking truth and a `row_lineage_hint_record` for
  artifact-origin truth; the row hint never substitutes for the
  search-result truth and vice versa.
- [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)
  — full lineage record the `lineage_ref` on a row hint points at when
  available.

If this document disagrees with the safe-edit policy, the PRD, or the
TAD appendix on generated artifacts, those documents win and this
packet updates in the same change.

## Why freeze this now

Explorer and search surfaces list objects **before** the user opens
them. Without a row-level hint:

- a quick-open row for `Cargo.lock` or `generated/client/api_client.ts`
  looks identical to an authored source row;
- a full-search result for a mirrored docs-pack paragraph reads as if
  it sits in the local authoritative source;
- a cross-repo group collapses `generated_artifact + stale_inputs`,
  `mirrored_artifact + mirror_drift`, and
  `generated_artifact + diverged_from_generator` into a single
  "generated" chip;
- AI citation and support-export rows lose origin / drift / rebuild
  intent because the full posture record is only consulted after the
  object opens.

Freezing a row-sized projection now means explorer, search,
cross-repo, docs-search, AI-citation, and support-export lanes all
read the same compact record and cannot silently promote a derived
file to canonical source.

## Scope

This packet freezes:

1. the `row_lineage_hint_record` shape — what an explorer row, quick-
   open row, search row, symbol-jump row, docs-search row, cross-repo
   result marker, graph overlay chip, AI citation tile, or
   support-export reference carries on the wire;
2. the row-surface vocabulary that lists which surfaces emit the hint;
3. mapping rules tying every token back to the shared posture record
   and the safe-edit policy so a row hint never invents a parallel
   provenance vocabulary;
4. a seed corpus covering the five required artifact classes — build
   outputs, generated source siblings (clean and diverged), lockfiles
   / resolved manifests, mirrored docs-pack artifacts, and preview
   render snapshots — plus the notebook-output fallback class the
   safe-edit corpus already carries.

Out of scope:

- final row rendering (typography, iconography, hover copy) in every
  UI surface;
- the full mutation-journal or VFS write-path implementation;
- the concrete codegen, build, resolver, mirror, or preview-runtime
  integrations that populate the full lineage record.

## 1. Row lineage-hint record

Every explorer-family row carries exactly one
`row_lineage_hint_record`. The record is a projection — it does not
replace the full posture record or the full generated-artifact lineage
record. It is the answer to three questions a reviewer of the row
must be able to answer without opening the object:

1. *What kind of object is this, and is it canonical or derived?*
2. *Where does its canonical source live, and what generated or mirrored it?*
3. *If I do something with it, should I edit here, edit the canonical source, regenerate, refresh, or stay read-only?*

### 1.1 Identity

- `record_kind` — `row_lineage_hint_record`.
- `row_lineage_hint_schema_version` — integer. Current value is `1`.
- `row_hint_id` — stable, opaque id. Safe to log, safe on RPC, safe
  inside search audits, support bundles, and export packets.
- `row_label` — short, redaction-safe presentation label (e.g.
  `Cargo.lock`, `generated/client/api_client.ts`). Raw absolute paths
  and raw artifact bodies never cross this boundary.

### 1.2 Posture link

- `artifact_posture_ref` — opaque id of the companion
  `artifact_edit_posture_record` frozen in the safe-edit policy.
  Surfaces resolve this ref to read override policy, active override
  provenance, structured-viewer fallback, and the
  `surface_projection_examples` the full record carries.
- `generated_artifact_lineage_ref` — optional opaque ref to the full
  generated-artifact lineage record when available. Consumers use this
  ref to resolve canonical source filesystem identities, input digests,
  and regeneration hints at the lineage layer.

Rules (frozen):

1. The row hint must carry the same token values for
   `artifact_class`, `artifact_origin_class`, `provenance_state`, and
   `default_edit_posture` as the referenced
   `artifact_edit_posture_record`. A row hint that drifts from its
   posture record is non-conforming.
2. A row hint that cannot resolve `artifact_posture_ref` must render
   as `provenance_state = unknown_lineage` and `default_edit_posture =
   inspect_read_only`. Rows never fall open into editable source when
   the posture is missing.

### 1.3 Shared posture tokens

The following fields are the cross-surface vocabulary. Every token
comes from
[`artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json);
row hints never mint new values.

- `artifact_class` — `build_output`, `codegen_source_sibling`,
  `structured_lockfile`, `notebook_output`, `preview_render_snapshot`,
  `mirrored_pack_artifact`.
- `artifact_origin_class` — `canonical_source`, `generated_artifact`,
  `mirrored_artifact`, `imported_artifact`, `preview_projection`.
- `provenance_state` — `not_applicable`, `in_sync`, `stale_inputs`,
  `generator_changed`, `diverged_from_generator`, `unknown_lineage`,
  `mock_provenance`, `mirror_drift`.
- `default_edit_posture` — `edit_here`, `edit_canonical_source`,
  `regenerate_from_source`, `structured_safe_edit`, `inspect_read_only`,
  `replace_by_mirror_promotion`, `clear_or_reexecute`.
- `do_not_imply_canonical_source` — boolean. `false` only when
  `artifact_origin_class = canonical_source`.

Rules (frozen):

1. The pair `(artifact_origin_class, provenance_state)` is the
   authoritative chip origin. Row chrome may render prose around it
   but may not substitute private badges, unlabeled icons, or freeform
   status strings.
2. `do_not_imply_canonical_source = true` is carried on every
   non-canonical row. A row chrome that hides or folds the flag is
   non-conforming.
3. `mirrored_artifact` rows must pair with
   `provenance_state ∈ { in_sync, mirror_drift }` and
   `default_edit_posture = replace_by_mirror_promotion`. The row hint
   never collapses `mirror_drift` into `stale_inputs`.
4. `provenance_state = diverged_from_generator` rows must resolve to a
   posture record that carries `active_override_provenance`. A row
   hint without that link is non-conforming.

### 1.4 Canonical source ref

- `canonical_source_ref.resolution_state` — `known`, `multiple_known`,
  `external_known`, `unknown`.
- `canonical_source_ref.display_refs[]` — redaction-safe labels for
  the canonical source(s). Never raw absolute paths, never raw URLs
  with credentials.
- `canonical_source_ref.lineage_ref` — opaque lineage ref when a full
  generated-artifact lineage record exists.

Rules (frozen):

1. `resolution_state = unknown` pins the row's
   `default_edit_posture = inspect_read_only`. The row chrome may not
   offer a direct-edit affordance.
2. `resolution_state = external_known` is reserved for mirror sources
   and imported artifacts. The row chrome routes repair through the
   mirror or re-import action, not ad hoc local editing.
3. The row hint carries at least one entry in `display_refs` whenever
   `resolution_state ∈ { known, multiple_known, external_known }`.

### 1.5 Generator / resolver / mirror identity

- `generator_or_resolver_identity.owner_class` — `generator`,
  `resolver`, `kernel`, `preview_runtime`, `mirror_source`,
  `import_source`, `unknown`.
- `generator_or_resolver_identity.display_label` — short redaction-safe
  label (e.g. `openapi-generator`, `cargo resolver`, `Storybook
  preview runtime`).
- `generator_or_resolver_identity.version_or_revision` — optional
  version / revision token when the row should expose it.

Rules (frozen):

1. `owner_class = unknown` requires
   `provenance_state ∈ { unknown_lineage, diverged_from_generator }`.
   A row hint may not claim `in_sync` without naming the owner.
2. The row chrome must render `display_label` when the row shows the
   origin chip; it may not suppress the generator identity even in
   compact presentation.

### 1.6 Row actions

The row hint carries typed actions the row chrome may offer. The array
is ordered; the first action is the row-default.

Action classes (closed):

- `open_in_canonical_source` — open the canonical source ref named in
  `canonical_source_ref.display_refs`. Required when
  `default_edit_posture = edit_canonical_source` and at least one
  canonical ref resolves.
- `regenerate_from_source` — invoke the regenerator / resolver /
  kernel referenced by `rebuild_intent.action_ref`. Required when
  `default_edit_posture ∈ { regenerate_from_source, structured_safe_edit }`
  and the row is visible on an editable-surface family.
- `refresh_mirror` — refresh or re-promote the mirror. Required when
  `artifact_origin_class = mirrored_artifact` and
  `provenance_state = mirror_drift`.
- `clear_or_reexecute` — required when `artifact_class =
  notebook_output`.
- `open_structured_viewer` — open the structured viewer instead of a
  raw editor. Required when the referenced posture record's
  `structured_viewer_fallback.fallback_state` is other than
  `not_needed`.
- `open_read_only_viewer` — required when `default_edit_posture =
  inspect_read_only` or `resolution_state = unknown`.
- `open_provenance_panel` — open the provenance / lineage panel. Safe
  to offer on every non-canonical row.
- `copy_posture_ref` — copy the `artifact_posture_ref`. Support
  surfaces use this when attaching row-level evidence to a bundle.

Rules (frozen):

1. Every non-canonical row carries at least one action. An empty
   action array on a non-canonical row is non-conforming.
2. `regenerate_from_source`, `refresh_mirror`, and `clear_or_reexecute`
   actions reference the posture record's `rebuild_intent.action_ref`;
   row hints never mint a parallel command id.
3. A row hint may not offer `edit_here`, `direct_edit`, or any
   synonym as an action. Direct edits are mediated by the posture
   record's `override_policy` and always open through the editor, not
   the row.

### 1.7 Row chip tokens

`row_chip_tokens[]` is the exact array of tokens the row chip renders.
The tokens are structured so surfaces can render consistently without
re-deriving them from the posture fields.

Each token is one of:

- `origin:<artifact_origin_class>`
- `class:<artifact_class>`
- `drift:<provenance_state>`
- `posture:<default_edit_posture>`
- `rebuild:<rebuild_intent_class>`
- `viewer_fallback:<viewer_fallback_state>` (only when fallback is
  active)
- `override_required` (only when the posture record's
  `override_policy.override_provenance_required = true`)

Rules (frozen):

1. `origin`, `class`, and `drift` tokens are mandatory on every non-
   canonical row. `posture` is mandatory on every row except
   `canonical_source + not_applicable`.
2. Surfaces may reorder the tokens for presentation but may not drop
   any mandatory token. Collapsed presentations (e.g. a list-dense
   explorer row) render the tokens as a compact chip but still emit
   the full list on export / support projections.
3. Token names are not localized. Surface copy localizes the label
   around the token; the token itself stays stable for support /
   export parity.

### 1.8 Row surface classes

`row_surface_classes[]` enumerates the surface families that may emit
the row hint. Every row-hint consumer must appear here.

Closed set:

- `explorer_tree_row`
- `quick_open_row`
- `full_search_row`
- `symbol_jump_row`
- `docs_search_row`
- `cross_repo_result_row`
- `graph_overlay_row`
- `ai_citation_row`
- `support_export_row`
- `export_packet_row`

Rules (frozen):

1. A surface that renders a row but does not appear in this family is
   non-conforming. Adding a new surface class is additive-minor and
   bumps `row_lineage_hint_schema_version`.
2. A row hint may appear on multiple surfaces in a single session; the
   same `row_hint_id` covers every such emission.
3. `support_export_row` and `export_packet_row` emissions preserve
   every mandatory field even when the surface's visible presentation
   is a one-line reference.

### 1.9 Forbidden implications

Row hints reuse the safe-edit policy's `forbidden_implications`
vocabulary:

- `imply_canonical_source`
- `offer_unreviewed_direct_edit`
- `omit_provenance_state`
- `export_body_without_provenance`
- `hide_fallback_reason`
- `hide_regeneration_path`

Row hints MUST enumerate the forbidden implications the row chrome is
responsible for not committing. The list is the same vocabulary used
by the posture record's per-surface projections so parity audits can
compare row and posture side-by-side.

### 1.10 Redaction and emission metadata

- `redaction_class` — same vocabulary used by other row packets
  (`metadata_safe_default` for the seed corpus).
- `emitted_at` — ISO 8601 or monotonic timestamp.

Raw absolute paths, raw URLs with credentials, raw policy bodies, and
raw artifact bytes never cross this boundary.

## 2. Mapping rules

The row hint is a projection. Every token in the record maps to a
concrete field in the full posture record or the full generated-
artifact lineage record. The mapping is one-way: the row hint does not
introduce a field the posture / lineage schemas do not already
describe.

| Row-hint field | Posture record source | Safe-edit policy anchor |
|---|---|---|
| `artifact_class` | `artifact_class` | §Shared posture vocabulary, §Artifact-class policy matrix |
| `artifact_origin_class` | `artifact_origin_class` | §Artifact origin class |
| `provenance_state` | `provenance_state` | §Provenance state |
| `default_edit_posture` | `default_edit_posture` | §Default edit posture |
| `do_not_imply_canonical_source` | `do_not_imply_canonical_source` | §Artifact origin class, §Structured-viewer fallback |
| `canonical_source_ref.*` | `canonical_source_resolution.*` | §Surface obligations → Search |
| `generator_or_resolver_identity.*` | `generator_or_upstream_ref.*` | §Surface obligations |
| `rebuild_intent_class` | `rebuild_intent.intent_class` | §Rebuild intent |
| `row_actions[].action_ref` | `rebuild_intent.action_ref` | §Rebuild intent |
| `row_actions[].open_in_canonical_source` | `canonical_source_resolution.display_refs` | §Surface obligations → Open |
| `row_chip_tokens` | Projection of `(artifact_class, artifact_origin_class, provenance_state, default_edit_posture, rebuild_intent.intent_class)` | §Surface obligations |
| `forbidden_implications` | `surface_projection_examples[].forbidden_implications` | §Surface obligations |

### 2.1 Search alignment

Search rows emit a `search_result_packet_record` and a
`row_lineage_hint_record` together. The contract is:

- the search-result packet carries search readiness / ranking / truth
  vocabulary from `search_result_truth.schema.json`;
- the row lineage hint carries artifact origin / drift / edit-posture
  vocabulary from `artifact_edit_posture.schema.json`;
- neither packet subsumes the other, and neither is allowed to mint a
  token from the other's vocabulary.

A search surface that claims `readiness_state = fully_indexed` still
must carry a row lineage hint with
`provenance_state = diverged_from_generator` when the row is a
diverged generated file. Index readiness and artifact drift are
orthogonal axes.

### 2.2 Support / export alignment

Support and export projections of the row carry the same mandatory
fields the posture record's `support_export` and `export_packet`
surface projections require:

- `artifact_class`
- `artifact_origin_class`
- `provenance_state`
- `default_edit_posture`
- `rebuild_intent` (class and `action_ref`)
- `generated_artifact_lineage_ref` when available
- `active_override_provenance` reference via `artifact_posture_ref`
  when the row is `diverged_from_generator`
- `structured_viewer_fallback` reference via `artifact_posture_ref`
  when the fallback state is other than `not_needed`

The row hint does not duplicate those blocks inline; it references
the posture record so support / export bundles carry one source of
truth per artifact.

### 2.3 AI-citation alignment

AI citations and future AI-apply flows read the row lineage hint
before they quote or modify a row. The row hint:

- keeps the artifact non-canonical when the origin class is not
  `canonical_source`;
- names the canonical source so the AI may re-target its edit at the
  authored object;
- forbids `imply_canonical_source` on every non-canonical citation;
- forbids direct-edit offers on rows whose posture record does not
  carry `declared_safe_ranges_only` or
  `declared_full_override_with_divergence` with the required review /
  provenance fields populated.

## 3. Seed corpus

The seed corpus in
[`/fixtures/generated/lineage_hint_examples/lineage_hint_manifest.yaml`](../../fixtures/generated/lineage_hint_examples/lineage_hint_manifest.yaml)
names the case ids every downstream surface cites. The concrete row
records live under
[`/artifacts/generated/explorer_search_rows/`](../../artifacts/generated/explorer_search_rows/).

The seed covers each required artifact class at least once:

| Case id | Artifact class | Origin / provenance | Row example |
|---|---|---|---|
| `generated.row_hint.build_output.release_binary` | `build_output` | `generated_artifact` / `in_sync` | [`build_output_release_binary_row.json`](../../artifacts/generated/explorer_search_rows/build_output_release_binary_row.json) |
| `generated.row_hint.codegen_sibling.clean` | `codegen_source_sibling` | `generated_artifact` / `in_sync` | [`codegen_source_sibling_clean_row.json`](../../artifacts/generated/explorer_search_rows/codegen_source_sibling_clean_row.json) |
| `generated.row_hint.codegen_sibling.diverged` | `codegen_source_sibling` | `generated_artifact` / `diverged_from_generator` | [`codegen_source_sibling_diverged_row.json`](../../artifacts/generated/explorer_search_rows/codegen_source_sibling_diverged_row.json) |
| `generated.row_hint.structured_lockfile.stale` | `structured_lockfile` | `generated_artifact` / `stale_inputs` | [`structured_lockfile_stale_row.json`](../../artifacts/generated/explorer_search_rows/structured_lockfile_stale_row.json) |
| `generated.row_hint.mirrored_pack.drifted` | `mirrored_pack_artifact` | `mirrored_artifact` / `mirror_drift` | [`mirrored_docs_pack_drifted_row.json`](../../artifacts/generated/explorer_search_rows/mirrored_docs_pack_drifted_row.json) |
| `generated.row_hint.preview_snapshot.mock` | `preview_render_snapshot` | `preview_projection` / `mock_provenance` | [`preview_snapshot_mock_row.json`](../../artifacts/generated/explorer_search_rows/preview_snapshot_mock_row.json) |
| `generated.row_hint.notebook_output.fallback` | `notebook_output` | `generated_artifact` / `in_sync` (viewer fallback) | [`notebook_output_fallback_row.json`](../../artifacts/generated/explorer_search_rows/notebook_output_fallback_row.json) |

Each row-hint example joins to a posture record under
[`/artifacts/generated/viewer_fallback_examples/`](../../artifacts/generated/viewer_fallback_examples/)
via `artifact_posture_ref`. Parity audits compare row chrome and
posture projections token-for-token.

## 4. Change rules

- Adding a new `row_surface_class`, `action_class`, or `row_chip_token`
  prefix is additive-minor and bumps
  `row_lineage_hint_schema_version`; it must update this document and
  at least one worked example in the same change.
- Adding a new `artifact_class`, `artifact_origin_class`,
  `provenance_state`, `default_edit_posture`, or
  `rebuild_intent_class` flows through the safe-edit policy and the
  posture schema first; this packet picks the new token up by
  reference, not by duplication.
- Repurposing an existing token is breaking and requires a new
  decision row plus companion updates across the safe-edit policy,
  the posture schema, the search-result-truth schema, and the seed
  corpus.
- Surfaces may add local copy around the frozen tokens but may not
  substitute surface-specific badges, unlabeled icons, or freeform
  status strings.

## 5. Source anchors

- `.t2/docs/Aureline_PRD.md:1753` — structured artifact diff / merge
  needs humane review surfaces for notebooks, lockfiles, manifests,
  source maps, and snapshots.
- `.t2/docs/Aureline_PRD.md:1766` — structured-artifact renderers are
  pure viewers by default unless round-trip safety is proven.
- `.t2/docs/Aureline_PRD.md:1768` — canonical source of truth must
  remain explicit for notebooks and generated artifacts.
- `.t2/docs/Aureline_PRD.md:2998` — oversized / generated artifacts
  open in safe preview or limited mode by default.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:3920` — one
  generated-artifact provenance, regeneration, and writable-boundary
  model for search, review, AI, and save flows.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9727` —
  Appendix DE artifact classes and drift-state matrix.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:12713` — generated artifact
  chrome must expose class, canonical source / generator, drift state,
  and open / regenerate actions.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:15374` — read-only /
  generated file chrome needs explicit regenerate / compare
  affordances.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:15618` — compare / viewer-
  only surfaces must say round-trip-safe editing is not proven.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:17972` — rendered / raw /
  text fallback views and "no editable round-trip" honesty cue.
