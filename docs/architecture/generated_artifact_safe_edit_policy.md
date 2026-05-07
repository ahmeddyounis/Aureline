# Generated artifact safe-edit policy

This document freezes the safe-edit posture Aureline uses when
generated, mirrored, imported, preview, and notebook/result artifacts
appear on search, open, review, AI, export, and support surfaces. The
goal is simple: non-canonical artifacts may not silently masquerade as
handwritten source.

Companion artifacts:

- [`/schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
  — boundary schema for the cross-surface posture record every
  non-owning surface reads.
- [`/docs/generated/diverged_from_generator_contract.md`](../generated/diverged_from_generator_contract.md)
  — contract for diverged-from-generator state, override review gating, and
  rebuild-intent/recovery obligations when direct edits are attempted on
  non-canonical artifacts.
- [`/fixtures/generated/drift_regeneration_manifest.yaml`](../../fixtures/generated/drift_regeneration_manifest.yaml)
  — reviewer-facing corpus of drift, regeneration, divergence, mirror,
  and fallback cases.
- [`/artifacts/generated/viewer_fallback_examples/`](../../artifacts/generated/viewer_fallback_examples/)
  — worked examples showing how search, open, AI, export, and support
  surfaces preserve the same posture tokens.
- [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)
  — full lineage/provenance record this document projects from.
- [`/docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md)
  — mutation-journal and generated-artifact lineage companion that
  already freezes drift, writable-boundary, and regeneration concepts.
- [`/schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json)
  — search/export boundary that must preserve the same generated and
  mirrored posture labels instead of inventing private badges.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md)
  — support/export packet families that must carry the same
  provenance/edit-posture fields.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  and
  [`/artifacts/generated/explorer_search_rows/`](../../artifacts/generated/explorer_search_rows/)
  — row-level projection of this posture record for explorer, quick-
  open, search, docs-search, cross-repo, graph-overlay, AI-citation,
  and support / export row surfaces.

If this document disagrees with the PRD, TAD, or UX spec, those sources
win and this document plus the schema update in the same change.

## Why freeze this now

Generated clients, build products, lockfiles, notebook outputs, preview
snapshots, mirrored docs packs, imported captures, and rendered exports
often look like ordinary files. That presentation is dangerous when the
correct next step is to open the canonical source, rerun a generator,
refresh a mirror, or stay read-only because round-trip editing is not
proven.

Without one contract:

- search can call something "generated" while open chrome calls it
  "editable";
- a structured viewer can look authoritative even when it is compare-
  only;
- AI/export/support packets can flatten provenance into prose and lose
  the difference between generated, mirrored, and diverged objects;
- direct edits to conditionally writable artifact classes can happen
  without a rebuild intent, override review, or recorded override
  provenance.

This policy closes that gap by freezing one posture record and one
shared vocabulary.

## Scope

Frozen at this revision:

- one shared posture record naming artifact class, origin class,
  provenance state, default edit posture, rebuild intent, override
  policy, active override provenance, and structured-viewer fallback;
- the rule that search, open, review, AI, export, and support surfaces
  preserve the same posture fields instead of minting surface-local
  labels;
- the rule that non-canonical artifacts explicitly carry
  `do_not_imply_canonical_source = true`;
- explicit safe-override review, rebuild-intent, and
  override-provenance requirements for artifact classes that admit
  direct edits under declared conditions.

Out of scope:

- the full notebook or codegen implementation;
- final structured-viewer UI layout;
- a global importer contract for every external artifact family;
- the concrete mutation journal or VFS write pipeline implementation.

## Shared posture vocabulary

Every protected surface reads the same posture record from
[`artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json).
The record does not replace full lineage. It is the compact projection
search/open/review/AI/support/export surfaces consume when they need one
stable answer to "what is this object, can I edit it, and what would
make it authoritative again?"

### 1. Artifact origin class

`artifact_origin_class` is the coarse source-of-truth family:

- `canonical_source`
  The current object is itself the authored source of truth.
- `generated_artifact`
  The object is derived from a canonical source plus a generator,
  resolver, kernel, or toolchain.
- `mirrored_artifact`
  The object is a mirrored or promoted copy of an upstream source.
- `imported_artifact`
  The object arrived through import, replay, or external handoff and is
  not automatically authoritative locally.
- `preview_projection`
  The object is a preview/render/runtime projection rather than an
  authored file.

`canonical_source` is the only origin class allowed to set
`do_not_imply_canonical_source = false`.

### 2. Provenance state

`provenance_state` is the drift/divergence axis:

| State | Meaning | Required product consequence |
|---|---|---|
| `not_applicable` | canonical source row; no generator or mirror drift axis applies | editable source posture may remain normal |
| `in_sync` | generated or mirrored artifact still matches the declared generator/mirror inputs | show origin and canonical source, not ordinary-source chrome |
| `stale_inputs` | canonical inputs changed since the artifact was produced | regenerate/re-resolve before strong claims |
| `generator_changed` | generator or toolchain changed | rebuild/regenerate before authoritative claims |
| `diverged_from_generator` | user or workflow overrode direct-edit policy | explicit divergence badge plus override provenance |
| `unknown_lineage` | provenance or canonical source can no longer be proven | open read-only or support-review only |
| `mock_provenance` | preview/runtime view is driven by mock data or non-authoritative preview inputs | do not imply authored/source-faithful truth |
| `mirror_drift` | mirrored artifact no longer matches its upstream or promotion basis | refresh or repromote, not ad hoc edit |

The pair `(artifact_origin_class, provenance_state)` is the shared
provenance vocabulary. Search/open/review/AI/support/export surfaces may
add local wording around it, but they may not substitute private
badges.

### 3. Default edit posture

`default_edit_posture` answers what Aureline should recommend by
default:

- `edit_here`
- `edit_canonical_source`
- `regenerate_from_source`
- `structured_safe_edit`
- `inspect_read_only`
- `replace_by_mirror_promotion`
- `clear_or_reexecute`

This field is not a UI hint only. It is the posture search/open/export/
support must preserve when they summarize or export the object.

### 4. Rebuild intent

`rebuild_intent.intent_class` is the shared "what would make this
authoritative again?" answer:

- `none_required`
- `regenerate_recommended`
- `regenerate_required_before_authoritative_claims`
- `rebuild_required_before_authoritative_claims`
- `reexecute_required_for_fresh_output`
- `mirror_refresh_required`
- `manual_recovery_required`

If a surface can show an action, it points at `rebuild_intent.action_ref`
instead of inventing prose-only recovery.

### 5. Override policy and active override provenance

Artifact classes that may admit direct edits under declared conditions
carry both:

- `override_policy`
  What kind of override is allowed, what review is required, whether
  override provenance must be recorded, and which ranges are safe.
- `active_override_provenance`
  Who declared the override, when, why, under which review reference,
  and whether a rebuild/relink intent was acknowledged.

The rule is strict:

1. A class that allows direct edits under declared conditions MUST
   carry `override_policy.safe_override_review`.
2. Such a class MUST declare whether override provenance is required.
3. Any artifact in `provenance_state = diverged_from_generator` MUST
   carry `active_override_provenance`.
4. A surface may not offer a direct-write affordance when the record
   says the required review/provenance fields are absent.

### 6. Structured-viewer fallback

Structured viewers and compare surfaces use one fallback object instead
of copy-only warnings:

- `fallback_state`
  one of `not_needed`, `compare_only`,
  `structured_viewer_with_raw_text_fallback`,
  `structured_viewer_with_raw_bytes_fallback`,
  `canonical_source_redirect`, or `read_only_viewer_only`.
- `reason_class`
  one of `round_trip_not_proven`, `mapping_uncertain`,
  `mock_or_preview_data_only`, `lineage_unknown`,
  `performance_or_size_limit`, or `policy_read_only`.
- `alternative_views`
  explicit raw/rendered/canonical-source/provenance views.
- `default_open_mode`
  what the shell should open first.

If a fallback state other than `not_needed` is active,
`do_not_imply_canonical_source` MUST be true.

## Artifact-class policy matrix

| Artifact class | Default edit posture | Override policy | Rebuild intent | Viewer/open default |
|---|---|---|---|---|
| `build_output` | `regenerate_from_source` or `inspect_read_only` | `not_available` | `rebuild_required_before_authoritative_claims` | read-only editor or diff/export |
| `codegen_source_sibling` | `edit_canonical_source` | `declared_full_override_with_divergence` only | usually `none_required` when in sync; `regenerate_required_before_authoritative_claims` after drift | read-only editor with jump-to-source |
| `structured_lockfile` | `structured_safe_edit` only where ranges are declared; otherwise `regenerate_from_source` | `declared_safe_ranges_only` | `regenerate_required_before_authoritative_claims` | structured viewer first, raw text fallback |
| `notebook_output` | `clear_or_reexecute` | `not_available` | `reexecute_required_for_fresh_output` | structured viewer or compare-only, raw fallback available |
| `preview_render_snapshot` | `inspect_read_only` | `not_available` | `reexecute_required_for_fresh_output` | rendered compare or canonical-source redirect |
| `mirrored_pack_artifact` | `replace_by_mirror_promotion` | `mirror_promotion_only` | `mirror_refresh_required` | structured/read-only view with mirror provenance |

Additional class rules:

1. A generated source sibling in sync is still not canonical source.
   The default mutation path remains `edit_canonical_source`.
2. A lockfile or resolved manifest that allows declared safe edits does
   not become canonical source. Safe ranges are an exception boundary,
   not a truth promotion.
3. Notebook outputs, preview snapshots, and rendered exports may be
   useful review surfaces, but they may not imply that editing the view
   edits the canonical code/data inputs.
4. Mirrored artifacts are never repaired by local ad hoc edits on the
   official path. Refresh, re-import, or repromote the mirror.

## Surface obligations

### Search

Search results, quick-open rows, result exports, and deep-link restores
must preserve at minimum:

- `artifact_class`
- `artifact_origin_class`
- `provenance_state`
- `default_edit_posture`
- `canonical_source_resolution`

Search must not collapse `mirrored_artifact + mirror_drift` into a
generic "generated" chip, and it must not label
`diverged_from_generator` rows with the same token used for
`stale_inputs`.

The row-level projection that explorer, quick-open, full-search,
symbol-jump, docs-search, cross-repo, graph-overlay, AI-citation, and
support / export row surfaces carry alongside a search-result packet
is frozen in
[`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md).
The row hint uses the same posture tokens as this policy; it does not
introduce a parallel provenance vocabulary.

### Open / editor / structured viewer

Open chrome chooses its initial mode from `default_edit_posture` and
`structured_viewer_fallback.default_open_mode`.

Rules:

1. Non-canonical rows must preserve `do_not_imply_canonical_source`.
2. If fallback is active, raw/canonical/provenance alternatives listed
   on the record must remain reachable.
3. `unknown_lineage` and `mirror_drift` rows may not open as ordinary
   editable source.
4. `structured_safe_edit` rows may expose only the declared safe ranges
   and must keep rebuild intent visible in the same review path.

### Review and AI

Review panes and AI context builders must preserve artifact posture
instead of flattening it into plain text. When a generated or mirrored
artifact is included in review or AI context:

- the context packet keeps the artifact non-canonical;
- divergence and unknown-lineage states remain explicit;
- a refactor or AI-apply flow may not silently widen from canonical
  source to generated artifact without the posture record saying the
  class is writable and what review/provenance it requires.

### Export and support packets

Support/export packets prefer refs plus posture fields over duplicating
large generated bodies when policy or scale says reference is safer.

At minimum they preserve:

- `artifact_class`
- `artifact_origin_class`
- `provenance_state`
- `default_edit_posture`
- `rebuild_intent`
- `generated_artifact_lineage_ref` when available
- `active_override_provenance` when present
- `structured_viewer_fallback` when the exported surface was a viewer
  fallback rather than a source-faithful editor

An export/support packet may summarize a generated body, but it may not
imply authored-source truth if the posture record says otherwise.

## Corpus requirements

The posture corpus under
[`/fixtures/generated/drift_regeneration_manifest.yaml`](../../fixtures/generated/drift_regeneration_manifest.yaml)
must always cover at least:

1. clean regenerate;
2. stale generated artifact;
3. edited generated file with source unknown;
4. preview snapshot with mock provenance;
5. mirrored artifact drift;
6. structured-viewer fallback when round-trip-safe editing is not
   proven.

The companion examples under
[`/artifacts/generated/viewer_fallback_examples/`](../../artifacts/generated/viewer_fallback_examples/)
must keep search/open/export/support projections on the same vocabulary
so parity audits can compare them mechanically.

## Change rules

- Adding a new artifact class, provenance state, edit posture, rebuild
  intent, override policy, viewer fallback state, or fallback reason is
  additive-minor and must update this document, the schema, and at
  least one worked example in the same change.
- Repurposing an existing token is breaking and requires a new decision
  row plus companion updates.
- Surfaces may add local copy around the frozen tokens, but they may
  not replace the tokens with surface-specific badges, freeform status
  strings, or unlabeled icons.

## Source anchors

- `.t2/docs/Aureline_PRD.md:1753` — structured artifact diff/merge
  needs humane review surfaces for notebooks, lockfiles, manifests,
  source maps, and snapshots.
- `.t2/docs/Aureline_PRD.md:1766` — structured-artifact renderers are
  pure viewers by default unless round-trip safety is proven.
- `.t2/docs/Aureline_PRD.md:1768` — canonical source of truth must
  remain explicit for notebooks and generated artifacts.
- `.t2/docs/Aureline_PRD.md:2998` — oversized/generated artifacts open
  in safe preview or limited mode by default.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:3920` — one
  generated-artifact provenance, regeneration, and writable-boundary
  model for search, review, AI, and save flows.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9727` —
  Appendix DE artifact classes and drift-state matrix.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:12713` — generated artifact
  chrome must expose class, canonical source/generator, drift state,
  and open/regenerate actions.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:15374` —
  read-only/generated file chrome needs explicit regenerate/compare
  affordances.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:15618` — compare/viewer-only
  surfaces must say round-trip-safe editing is not proven.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:17972` — rendered/raw/text
  fallback views and "No editable round-trip" honesty cue.
