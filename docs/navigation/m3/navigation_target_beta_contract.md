# Navigation Target Beta Contract

This document freezes the typed target model used by definition,
declaration, implementation, reference, hierarchy, rename-preview,
breadcrumb, bookmark, history, AI, review, CLI/headless, graph, and
support-export surfaces.

Machine-readable companion:

- `schemas/navigation/navigation_target.schema.json`
- `crates/aureline-navigation/src/target_model/`
- `fixtures/navigation/m3/target_accuracy/`
- `artifacts/navigation/m3/navigation_target_fidelity_report.md`

The model composes with the existing semantic result and navigation-continuity
contracts. Language-specific records may keep provider-local fields, but any
surface that renders, exports, or reasons about a jump must be able to project
the row into this model.

## Required Objects

`NavigationTarget` carries `target_id`, `relation_kind`, `object_ref`,
`anchor_ref`, `provider_class`, `proof_class`, `confidence`, `freshness`,
`ambiguity_class`, `scope_completeness`, `scope_ref`, generated/imported
posture, downgrade reasons, evidence refs, and an export-safe summary.

`ReferenceOccurrence` carries `occurrence_id`, `target_ref`, `anchor_ref`,
`access_kind`, `scope_ref`, generated/imported posture, `proof_class`,
`confidence`, `freshness`, `scope_completeness`, downgrade reasons, evidence
refs, and an export-safe summary.

`HierarchyEdge` carries `edge_id`, `source_ref`, `target_ref`, `edge_kind`,
`proof_class`, `depth`, `scope_completeness`, `freshness`, `confidence`,
runtime/framework evidence refs, downgrade reasons, and an export-safe summary.

`RenamePreviewSet` carries `rename_preview_id`, `root_target_ref`,
`candidate_occurrence_refs`, `blocked_refs`, `conflict_notes`,
`sparse_or_partial_reasons`, `generated_scope_notes`, count summary,
`proof_class`, `confidence`, `freshness`, `scope_completeness`,
`apply_posture`, redaction class, evidence refs, and an export-safe summary.

`NavigationDisambiguationSet` carries `set_id`, `requested_relation`,
`candidate_target_refs`, `selection_policy`, `created_at`, `ambiguity_class`,
`confidence`, `freshness`, `scope_completeness`, downgrade reasons, evidence
refs, and an export-safe summary.

`TargetContinuityRef` binds breadcrumbs, outline nodes, bookmarks, history,
and peek contexts back to the same target identity. Drifted or missing
continuity records must provide a remap, disambiguation, or downgrade path
instead of silently reopening a nearby guess.

## Stable Vocabularies

`relation_kind` values are `definition`, `declaration`, `implementation`,
`reference`, `type`, `call`, `route-binding`, `owner-link`, and `doc-link`.

`access_kind` values are `read`, `write`, `call`, `inherit`, `import`,
`export`, `test-only`, and `generated`.

`proof_class` keeps runtime and framework evidence distinct from semantic
proof: `direct_semantic`, `indexed_semantic`, `lexical_fallback`,
`syntax_fallback`, `imported_evidence`, `framework_derived`,
`runtime_observed`, `ai_inferred`, and `unavailable`.

`confidence`, `freshness`, `ambiguity_class`, and `scope_completeness` are
mandatory on target, reference, hierarchy, rename, and disambiguation rows so
partial, stale, ambiguous, generated, and fallback answers cannot render as a
generic successful jump.

## Consumer Rules

Editor UI, CLI/headless, AI context, review workspace, support export, graph
overlay, and shell-continuity projections must preserve relation, access,
proof, and scope-completeness labels. Support and review exports may omit
previews and source bodies, but they must preserve metadata, counts, blocked
refs, generated notes, sparse reasons, and evidence refs.

Go-to-definition must emit a `NavigationTarget` with
`relation_kind=definition`. Declaration, implementation, type, and route/doc
targets must not be aliased to definition unless the downgrade is explicit.

Find-references must emit `ReferenceOccurrence` rows with access kinds. Grep or
syntax fallback rows are allowed only when they carry fallback proof and
downgrade reasons.

Call/type hierarchy must emit `HierarchyEdge` rows. Framework-derived and
runtime-observed edges may enrich the hierarchy, but they do not replace direct
semantic proof.

Rename preview must bind to a `RenamePreviewSet`. Blocked refs, generated
scope notes, conflicts, sparse/partial reasons, and counts remain present even
when raw code bodies are redacted.

Breadcrumbs, outline, bookmarks, history, and peek targets must bind through
`TargetContinuityRef`. Reopen paths preserve target identity or stop at an
explicit disambiguation/degraded state.
