# Structured-artifact diff, merge, and review seed

This document freezes the review posture for non-line-oriented artifacts
before notebook, generated, ecosystem, and evidence surfaces start
carrying unsafe text-only assumptions. Code review and everyday compare
flows touch more than source files: Aureline must present humane,
content-aware review surfaces for Jupyter notebooks, JSON / YAML / TOML
configs, lockfiles and dependency manifests, coverage and profile
artifacts, SBOMs and generated metadata, source maps and debug
sidecars, images and design snapshots, evidence packets, and generated
source.

The contract pins five things every downstream surface can cite without
inventing side metadata:

1. the shape of a `review_surface_record` — exactly one row per
   structured-artifact class;
2. the closed review-surface and merge-posture vocabularies every
   review, diff, merge, Git-provider, support-export, AI-review, and
   release-evidence surface renders verbatim;
3. one visible posture-reason token per row that keeps
   `semantic_merge_where_safe` and `regenerate_or_review` mechanically
   distinguishable instead of collapsing into prose;
4. the canonical-source rule and paired-text representation declaration
   for classes where a derived text form exists (notebooks exported to
   scripts, lockfiles paired with manifests); and
5. the generated-source back-link rule that keeps generated source and
   debug sidecars joined to the shared safe-edit / debug-artifact
   records rather than re-minting a parallel provenance vocabulary on
   every review surface.

This is a review-posture seed, not an implementation. It does not ship
a notebook diff UI, an image diff UI, or a merge driver. It is the
contract every later review lane reads so those implementations inherit
one decision rather than re-debate fundamentals per surface.

Companion artifacts:

- [`/schemas/review/review_surface_record.schema.json`](../../schemas/review/review_surface_record.schema.json)
  — machine-readable boundary for the `review_surface_record`.
- [`/artifacts/review/structured_artifact_classes.yaml`](../../artifacts/review/structured_artifact_classes.yaml)
  — row matrix binding each artifact class to exactly one review
  surface, merge posture, visible reason, canonical-source rule,
  paired-text declaration, unknown-metadata survival rule, trust /
  sandbox constraint set, notebook preset pair, generated-source
  back-link, and Git-merge-driver admission decision.
- [`/fixtures/review/structured_diff_cases/`](../../fixtures/review/structured_diff_cases/)
  — worked review-and-merge cases per class: notebook cell-aware merge,
  JSON schema-supported semantic merge, YAML with unknown vendor
  sections, lockfile regenerate-or-review, coverage sidecar compare,
  SBOM compare-only, image perceptual compare, evidence immutable
  reader, generated source with safe-edit back-link.
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  and
  [`/schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
  — safe-edit posture record every `generated_source_artifact` row
  back-links to.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  — row-level lineage hint carried on explorer, search, AI, and
  support-export rows. The review-surface record is the compare-side
  companion; the lineage hint stays the list-side companion.
- [`/docs/debug/artifact_resolution_seed.md`](../debug/artifact_resolution_seed.md)
  and
  [`/schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
  — debug-artifact manifest every `source_map_or_debug_sidecar` row
  back-links to.
- [`/docs/security/suspicious_content_packet.md`](../security/suspicious_content_packet.md)
  — shared safe-preview detector contract the trust / sandbox
  constraints on this matrix inherit.
- [`/docs/verification/install_review_packet.md`](../verification/install_review_packet.md)
  — install-review packet family the ecosystem-adjacent rows align
  with; SBOM / lockfile review never re-mints install-review tokens.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  §16.7.1 (structured artifact diff, merge, and review architecture),
  §20 (notebook and interactive computing architecture),
  Appendix AC (structured artifact review corpus and conformance
  rules), Appendix AX (notebook kernel, trust, and reproducibility
  matrix), Appendix AY (content-integrity and safe-preview matrix),
  Appendix CJ (review workspace, anchoring, and merge-queue matrix),
  and Appendix DE (generated-artifact provenance, regeneration, and
  writable-boundary matrix).
- `.t2/docs/Aureline_PRD.md` — diff / merge / review normalized-view
  guidance, structured-document unknown-key preservation rules, and
  notebook canonical-open-document-format rules.

If this seed disagrees with those sources, those sources win and this
document plus the schema update in the same change.

## Why freeze this now

Review and compare flows currently treat every file as a text blob. For
source that works. For notebooks, structured configs, lockfiles,
generated source, source maps, coverage / profile outputs, SBOMs,
images, and evidence packets it produces silently wrong answers:

- a quick-open on `*.ipynb` renders raw JSON where cell identity,
  output trust, and documented metadata namespaces should survive;
- a conflict on a lockfile auto-merges two solver runs whose
  intersection is not a valid solver answer;
- a diff on a JSON / YAML / TOML config silently strips comments or
  reorders keys instead of preserving the author's layout and vendor
  extensions;
- a coverage or profile sidecar is rendered without its run / build
  binding and reads as accurate mapping when it is not;
- an SBOM or generated manifest appears edited because an unknown
  vendor field round-tripped wrong on save;
- a generated client looks like canonical source on the compare side
  because the safe-edit posture record is only consulted on open;
- a support bundle or release-evidence packet is implicitly editable
  because no surface declared it immutable.

Freezing one record and one matrix now means every later review lane
(desktop compare, merge surface, Git-provider integration,
support-export review, AI-review citation, release-evidence review)
reads the same row and cannot silently promote a derived or evidence
artifact to a mergeable text blob.

## Scope

This seed freezes:

1. the `review_surface_record` row shape every consuming surface reads,
   with required slots for review surface, merge posture, one visible
   reason, canonical-source rule, paired-text declaration, unknown-
   metadata survival, trust / sandbox constraints, notebook presets,
   generated-source back-link, Git-merge-driver admission, and a short
   reviewable summary;
2. the closed vocabularies listed in
   [`schemas/review/review_surface_record.schema.json`](../../schemas/review/review_surface_record.schema.json)
   for every enumerated slot, with exactly one row per
   `review_artifact_class`;
3. the matrix at
   [`artifacts/review/structured_artifact_classes.yaml`](../../artifacts/review/structured_artifact_classes.yaml)
   binding each artifact class to its default review surface, merge
   posture, and posture reason;
4. notebook cell-aware review: metadata-filter presets, output-handling
   presets, and the mandatory `cell_id_stability_required` flag;
5. generated-source back-links tying
   `generated_source_artifact` rows to the safe-edit posture record and
   `source_map_or_debug_sidecar` rows to the debug-artifact manifest;
6. the canonical-source and paired-text rules keeping notebook → script
   exports and manifest ↔ lockfile pairs explicit;
7. the matrix invariants downstream tooling audits against.

Out of scope (the next lanes):

- shipping any notebook or image diff UI in this milestone;
- implementing any Git merge driver or merge-queue integration;
- implementing the underlying notebook, config, lockfile, SBOM, source
  map, coverage, profile, image, or evidence renderers;
- full cross-repo or remote-review wiring.

The matrix is deliberately narrower than those lanes. It fixes the
posture decisions they must inherit.

## 1. Review-surface record

Every structured-artifact row in the matrix is exactly one
`review_surface_record`. The record is the compact projection every
review, diff, merge, Git-provider, support-export, AI-review, and
release-evidence surface reads. It is the answer to five questions a
reviewer must be able to answer without opening the object:

1. *What kind of artifact is this, and which review surface should it
   open on?*
2. *What is the safe default merge posture, and what single reason
   explains it?*
3. *Is the artifact its own canonical source, or is there a paired text
   form with a named canonical direction?*
4. *Does unknown metadata or vendor extension round-trip safely, and
   which trust or sandbox constraints apply?*
5. *If this is a derived artifact (generated source, debug sidecar),
   which shared provenance record does it join?*

### 1.1 Record slots

- `record_kind` — `review_surface_record`.
- `review_surface_schema_version` — integer. Current value is `1`.
- `review_surface_id` — stable, opaque id.
- `review_artifact_class` — exactly one of the classes defined in
  [`schemas/review/review_surface_record.schema.json`](../../schemas/review/review_surface_record.schema.json).
- `review_surface_class` — default review surface class.
- `merge_posture_class` — default merge posture. One of
  `no_auto_merge`, `semantic_merge_where_safe`, `regenerate_or_review`,
  or `compare_only`.
- `posture_reason_class` — exactly one visible reason token per row,
  keeping `semantic_merge_where_safe` and `regenerate_or_review`
  mechanically distinguishable.
- `canonical_source_rule_class` — `single_canonical_source`,
  `paired_text_representation_with_canonical_direction`,
  `derived_from_canonical_source`, or
  `not_applicable_mirror_or_evidence`.
- `paired_text_representation` — pair-state plus optional canonical-
  direction and round-trip-corpus refs.
- `unknown_metadata_survival_class` — `required_survival`,
  `survival_recommended`, or `not_applicable`.
- `trust_sandbox_constraints` — non-empty closed list.
- `notebook_review_presets` — metadata-filter preset, output-handling
  preset, and `cell_id_stability_required` flag.
- `generated_source_backlink` — back-link state plus optional
  `safe_edit_posture_ref` or `debug_artifact_ref`.
- `git_merge_driver_admission_class` — whether a Git merge driver may
  be registered for this class.
- `conformance_corpus_ref` — required whenever merge posture is
  `semantic_merge_where_safe` or merge-driver admission is any
  `permitted_*` value.
- `summary` — one short reviewable sentence.

### 1.2 Reason-code discipline

`semantic_merge_where_safe` rows always carry one of
`schema_support_proven_with_unknown_field_survival`,
`schema_support_partial_structure_aware_only`, or
`cell_and_output_identity_at_risk`. `regenerate_or_review` rows always
carry `solver_meaning_may_change_under_normalization` or
`generated_from_canonical_source`. The two postures never share a
reason code, and no row carries more than one visible reason. This is
the acceptance rule the matrix invariants enforce.

## 2. Artifact-class matrix

The matrix at
[`artifacts/review/structured_artifact_classes.yaml`](../../artifacts/review/structured_artifact_classes.yaml)
carries one `review_row` per class. The summary below is narrative; the
YAML is normative.

| Artifact class | Review surface | Merge posture | Visible reason |
|---|---|---|---|
| **Jupyter notebook** | cell-aware compare viewer | semantic merge where safe | cell and output identity at risk |
| **JSON structured config** | semantic compare viewer | semantic merge where safe | schema support proven with unknown-field survival |
| **YAML structured config** | semantic compare viewer | semantic merge where safe | schema support partial; structure-aware only |
| **TOML structured config** | semantic compare viewer | semantic merge where safe | schema support partial; structure-aware only |
| **Lockfile / dependency manifest** | structure-aware compare viewer | regenerate or review | solver meaning may change under normalization |
| **Coverage / profile artifact** | sidecar-bind viewer | compare only | sidecar requires build-identity match |
| **SBOM / generated metadata** | structure-aware compare viewer | compare only | round-trip unproven for bytes |
| **Source map / debug sidecar** | sidecar-bind viewer | no auto merge | generated from canonical source |
| **Image / design snapshot** | perceptual or side-by-side viewer | no auto merge | binary or opaque payload |
| **Evidence packet** | evidence reader viewer | no auto merge | evidence immutability required |
| **Generated source artifact** | structure-aware compare viewer | regenerate or review | generated from canonical source |

## 3. Notebook cell-aware review

`jupyter_notebook` rows MUST declare a non-`not_applicable`
metadata-filter preset, a non-`not_applicable` output-handling preset,
and `cell_id_stability_required = true`. Every other row MUST set both
presets to `not_applicable` and `cell_id_stability_required = false`.

The default presets for notebooks are:

- `metadata_filter_preset = preserve_jupyter_and_aureline_namespaces_only`.
  The documented Jupyter namespaces and `metadata.aureline` survive in
  the compare view; unrelated vendor metadata is filtered *from the
  view*, never stripped from disk. Unknown metadata round-trips on
  save unless the user deliberately strips it.
- `output_handling_preset = ignore_outputs_by_default_with_opt_in_inclusion`.
  Output cells are hidden from the compare view by default; the user
  opts them in when reviewing a run-to-run output change. Active
  outputs still require the Section 20 trust model even when included.
- `cell_id_stability_required = true`. Review surfaces refuse to open
  a notebook for compare if stable cell identity cannot be guaranteed
  across the two revisions.

Structured merge on notebooks is `permitted_with_round_trip_corpus`
only: the class has unknown-metadata-survival guarantees and the
round-trip corpus named by `paired_text_representation.round_trip_corpus_ref`
must pass before a merge driver registers. Notebook-to-script exports
are admissible as `export_only_no_round_trip` — the exported script is
never a silent round-trip back into the canonical `.ipynb`.

## 4. Canonical source and paired-text rules

Canonical-source rules are closed:

- `single_canonical_source` — the artifact is the source of truth
  (notebooks; JSON / YAML / TOML configs; images / design snapshots).
  A paired text representation may exist as `none` or
  `export_only_no_round_trip`, but edits never flow silently through
  the derived form.
- `paired_text_representation_with_canonical_direction` — the class
  names a canonical direction across a pair. Lockfile / dependency
  manifest rows carry `canonical-direction:manifest->lockfile:v1`: the
  manifest is the authoring surface, the lockfile is the resolved
  derivation. Edits to the manifest drive lockfile regeneration; edits
  to the lockfile route to regenerate-or-review rather than to silent
  auto-merge.
- `derived_from_canonical_source` — the artifact has no authoring
  surface of its own (generated source, source maps, debug sidecars,
  SBOMs, generated metadata, coverage / profile output). Review rules
  point the user at the canonical source or at the generating lane.
- `not_applicable_mirror_or_evidence` — evidence packets and mirrored
  artifacts whose local row has no authoring surface.

## 5. Generated-source and debug-sidecar back-links

Two rows carry mandatory back-links rather than re-minting provenance
vocabulary here:

- `generated_source_artifact` rows MUST carry
  `generated_source_backlink.backlink_state =
  required_link_to_safe_edit_posture_record` and a non-null
  `safe_edit_posture_ref` pointing at an
  `artifact_edit_posture_record` validated by
  [`schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json).
  Artifact class, origin class, provenance state, default edit
  posture, rebuild intent, override policy, override provenance, and
  structured-viewer fallback all live on the safe-edit record; the
  review row cites it and never duplicates those slots.
- `source_map_or_debug_sidecar` rows MUST carry
  `generated_source_backlink.backlink_state =
  required_link_to_debug_artifact_manifest` and a non-null
  `debug_artifact_ref` pointing at a `debug_artifact_entry_record`
  validated by
  [`schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json).
  Mapping fidelity, build identity, resolution state, and mismatch /
  degraded-quality vocabulary live on the debug-artifact manifest; the
  review row cites it and never mints independent mapping claims.

Every other row MUST carry
`generated_source_backlink.backlink_state = not_applicable`.

## 6. Trust and sandbox constraints

Every row declares at least one `trust_sandbox_constraint_class`:

- `jupyter_notebook` rows MUST include
  `notebook_active_outputs_sandboxed`. Section 20 notebook trust rules
  still apply when a notebook appears inside a Git review or compare
  flow.
- `image_or_design_snapshot` and `coverage_profile_artifact` rows MUST
  include `binary_preview_sandboxed`.
- `sbom_or_generated_metadata` rows MUST include
  `untrusted_rich_content_sanitized` because embedded HTML, links, or
  descriptive fields may carry active content.
- `structured_config_*` rows MUST include
  `secrets_or_policy_tokens_may_require_redaction`; the shared
  redaction posture matrix owns the actual rules.
- `evidence_packet` rows MUST include
  `evidence_read_only_redaction_applies`; evidence surfaces inherit
  the redaction posture matrix and are immutable by construction.
- Every other row declares `no_additional_trust_constraints`.

## 7. Git and merge-driver admission

`git_merge_driver_admission_class` is a single-token decision per row:

- `permitted_with_round_trip_corpus` — notebooks. The class has
  unknown-metadata survival guarantees and the round-trip corpus must
  pass before registration.
- `permitted_where_semantics_defined` — structured configs. The merge
  driver admits documented keys and falls back to structure-aware
  compare for undocumented sections.
- `forbidden_until_round_trip_proven` — lockfiles, SBOMs, generated
  source. Merge driver is not admissible until the round-trip corpus
  and solver-meaning evidence land.
- `forbidden_compare_only_surface` — coverage / profile, source maps /
  debug sidecars, images / design snapshots, evidence packets. The
  review surface never writes back, so a merge driver is never
  admissible.

Any `permitted_*` value requires a non-null
`paired_text_representation.round_trip_corpus_ref` and a non-null
`conformance_corpus_ref`. Later Git-integration tasks derive their
admission decisions from these rows rather than from per-host prose.

## 8. Matrix invariants

The matrix invariants at the foot of
[`artifacts/review/structured_artifact_classes.yaml`](../../artifacts/review/structured_artifact_classes.yaml)
are normative and include:

- exactly one row per `review_artifact_class`;
- exactly one visible `posture_reason_class` per row;
- `semantic_merge_where_safe` and `regenerate_or_review` never share a
  reason code;
- notebook presets required only on notebook rows;
- generated-source back-links required on the two derived rows;
- paired-text representations with a non-`none` pair state require a
  named canonical direction;
- any `permitted_*` merge-driver admission requires corpus refs;
- compare-only and no-auto-merge classes forbid a merge driver;
- evidence packets are immutable by construction;
- unknown-metadata survival explicitly declared per row;
- trust / sandbox constraints explicitly declared per row;
- milestone slugs never appear in ids, paths, descriptions, or
  summaries.

Later review, merge, Git-provider, support-export, AI-review, and
release-evidence lanes audit against these invariants rather than
re-debate review fundamentals per surface.
