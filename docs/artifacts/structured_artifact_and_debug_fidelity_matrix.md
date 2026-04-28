# Structured-Artifact and Debug-Fidelity Matrix

This matrix gives notebook, structured config, lockfile, debug, crash,
and coverage/profile surfaces one shared fidelity vocabulary. It is a
coordination layer over existing seeds, not a viewer implementation.
Downstream editors, diff tools, merge flows, debugger inspectors,
support exports, and release evidence should cite these rows instead of
inventing format-specific language.

Companion artifacts:

- [`/artifacts/artifacts/debug_fidelity_rows.yaml`](../../artifacts/artifacts/debug_fidelity_rows.yaml)
  carries the machine-readable fidelity rows: first-class view,
  default open mode, provenance floor, build/tool linkage, support
  target, and edit/read-only/compare/regenerate posture.
- [`/artifacts/artifacts/merge_resolution_policy_rows.yaml`](../../artifacts/artifacts/merge_resolution_policy_rows.yaml)
  carries the matching merge and resolution policy rows.
- [`/fixtures/artifacts/fidelity_examples/`](../../fixtures/artifacts/fidelity_examples/)
  contains worked cases for semantic view, compare view, read-only
  inspector, and regenerate/review flow.

Authoritative anchors this matrix composes with:

- [`/docs/review/structured_artifact_review_seed.md`](../review/structured_artifact_review_seed.md)
  and
  [`/artifacts/review/structured_artifact_classes.yaml`](../../artifacts/review/structured_artifact_classes.yaml)
  for structured review surfaces, unknown-metadata survival, and
  merge-driver admission.
- [`/docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md)
  and
  [`/artifacts/notebook/kernels_and_trust_matrix.yaml`](../../artifacts/notebook/kernels_and_trust_matrix.yaml)
  for notebook trust, cell identity, kernel provenance, and paired
  text export posture.
- [`/docs/debug/artifact_resolution_seed.md`](../debug/artifact_resolution_seed.md)
  and
  [`/schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
  for native symbols, source maps, crash artifacts, generated mappings,
  coverage, and profile resolution parity.
- [`/docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md),
  [`/docs/support/exact_build_symbolication_smoke.md`](../support/exact_build_symbolication_smoke.md),
  and
  [`/artifacts/support/crash_artifact_retention_seed.json`](../../artifacts/support/crash_artifact_retention_seed.json)
  for exact-build symbolication, dump redaction, and debugger UI truth.
- [`/docs/notebooks/output_viewer_truth_contract.md`](../notebooks/output_viewer_truth_contract.md)
  for notebook output evidence, stale/replayed/orphaned output states,
  and support include policy.
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  for generated/regenerated artifact posture and structured-viewer
  fallback semantics.

If this document conflicts with those sources, those sources win and
this file plus the YAML rows update in the same change.

## Shared Vocabulary

`artifact_interaction_posture` is the compact answer every opener,
reviewer, support bundle, and AI/export surface must preserve:

| Posture | Meaning |
|---|---|
| `editable` | The artifact is an authored surface and may edit in its first-class view when trust, schema, and round-trip requirements pass. |
| `read_only` | The artifact can be inspected or resolved, but the bytes are evidence or sidecar material and are not edited by Aureline. |
| `regenerate_first` | The artifact is derived from canonical inputs; conflicts or drift route to regeneration or explicit review before any authoritative claim. |
| `compare_only` | The artifact can be compared, validated, or summarized, but the review surface must not offer write-back or VCS auto-merge. |

`default_open_mode` is the shell decision before a user chooses a raw or
expert fallback:

| Mode | Use |
|---|---|
| `semantic_view` | Open the structured, source-aware, or cell-aware view first. |
| `compare_view` | Open a comparison/validation view first, usually with baseline/current or artifact/source binding. |
| `read_only_inspector` | Open an inspector or report that resolves identity and provenance before content. |
| `regenerate_review_flow` | Open the regenerate/review flow with canonical inputs pinned. |

The split is intentional. A JSON config can be `editable` and open in
`semantic_view`; a source map can be `compare_only` and open in
`compare_view`; a native symbol package is `read_only` and opens in
`read_only_inspector`; a lockfile conflict is `regenerate_first` and
opens in `regenerate_review_flow`.

## Fidelity Rows

The YAML row set is normative; this table is the human summary.

| Artifact family | First-class view | Merge / resolution posture | Interaction posture | Provenance floor | Build / tool linkage | Initial support target |
|---|---|---|---|---|---|---|
| Jupyter notebook | Cell-aware notebook view with metadata/output filters | Semantic merge where safe; unresolved conflicts explicit | `editable` | notebook format version, stable cell ids, trust state, kernel/environment refs when known | Jupyter schema, kernelspec, execution context, optional last-run fingerprint | `v1.x` |
| JSON config | Schema-aware semantic config view | Semantic merge where schema and unknown-field survival are proven | `editable` | file identity, schema/validator ref, unknown-key survival report, redaction class | parser/schema/formatter version; policy epoch when policy-bearing | `v1.0` |
| YAML config | Structure-aware semantic config view with comments/order preserved | Semantic merge where documented; structure-aware conflict otherwise | `editable` | file identity, schema/validator ref, comment/order preservation report, redaction class | parser/schema/formatter version; policy epoch when policy-bearing | `v1.0` |
| TOML config | Table-aware semantic config view with comments/order preserved | Semantic merge where documented; structure-aware conflict otherwise | `editable` | file identity, schema/validator ref, table/order preservation report, redaction class | parser/schema/formatter version; package/build tool version when manifest-bearing | `v1.0` |
| Lockfile / dependency manifest | Package graph compare with transitive impact | Regenerate or review; no opaque auto-merge | `regenerate_first` | input manifest refs, resolver state, graph hash, integrity hashes | package manager/resolver version, platform/env factors, lockfile digest | `v1.x` |
| Native symbol artifact | Symbol artifact inspector and debugger binding | Resolve by exact build/module identity; no merge | `read_only` | build id/debug id/UUID, target triple, symbol digest, release graph ref when applicable | compiler/debug format, exact build identity, symbol package version | `v1.x` |
| Source map | Source-map inspector with mapped frame/bundle validation | Compare/validate mapping only; stale or partial mapping is explicit | `compare_only` | bundle digest, source-map digest, source refs, mapping-quality state | bundler/transpiler version, generated command digest, exact build identity when release-bearing | `v1.x` |
| Crash dump / core | Crash viewer plus symbolication report | Read-only evidence; symbolication exact-build only | `read_only` | crash id, dump format, module list, redaction/export posture, symbol manifest ref | build id, target/host lane, symbol/source-map manifest versions | `v1.x` |
| Coverage / profile artifact | Coverage/profile compare and hotspot views | Compare-only; run-set aggregation is explicit analysis, not VCS merge | `compare_only` | run id, target, execution context, capture source, freshness, included/excluded run set | coverage/profiler tool version, commit/build identity, sample/branch availability | `v1.x` |

## Row Rules

1. Every artifact family in the machine-readable rows declares exactly
   one `artifact_interaction_posture` and one `default_open_mode`.
2. Editable rows must cite the structured-artifact review seed and name
   their unknown-metadata, schema, trust, or round-trip requirements.
3. Regenerate-first rows must name the canonical input refs and the tool
   version or resolver version needed to recreate the artifact.
4. Read-only debug rows must resolve through `debug_artifact_ref`,
   exact-build identity, or crash/symbolication manifest refs. They do
   not invent merge semantics.
5. Compare-only rows may aggregate or compare evidence, but that
   operation never means raw artifact write-back or silent VCS auto-merge.
6. Support exports must preserve provenance and redaction posture even
   when raw bodies are omitted.
7. Raw paths, raw notebook bodies, raw dump bytes, raw symbol bytes, raw
   source-map bodies, raw coverage/profile payloads, package names from
   private registries, secrets, and policy payloads stay out of these
   rows. The matrix carries refs, digests, labels, counts, and posture.

## Open-Mode Examples

The examples under
[`/fixtures/artifacts/fidelity_examples/`](../../fixtures/artifacts/fidelity_examples/)
exercise the four default-open modes:

| Example | Open mode | Why |
|---|---|---|
| `semantic_view_notebook_trusted_outputs_hidden.yaml` | `semantic_view` | A notebook is trusted enough for editing, but outputs remain hidden by default and active content is sandboxed. |
| `semantic_view_structured_config_schema_known.yaml` | `semantic_view` | A JSON config has a known schema and unknown-key survival is proven. |
| `compare_view_coverage_profile_two_runs.yaml` | `compare_view` | Two profile captures share build/run identity fields and should compare, not edit. |
| `compare_view_source_map_stale_mapping.yaml` | `compare_view` | A source map can validate stale/partial mapping without being edited. |
| `read_only_inspector_native_symbol_mismatch.yaml` | `read_only_inspector` | A symbol package is present but mismatched; the inspector shows the exact mismatch token. |
| `read_only_inspector_crash_core_pending_consent.yaml` | `read_only_inspector` | A core dump is evidence with pending upload consent and redaction posture. |
| `regenerate_review_lockfile_solver_conflict.yaml` | `regenerate_review_flow` | Lockfile conflicts pin the manifest and resolver before review. |

## Change Rules

- Adding a new artifact family is additive when it lands a row in both
  YAML files, adds at least one fixture if it introduces a new open-mode
  behavior, and cites the owning source contract.
- Repurposing an existing posture, open mode, or provenance requirement
  is breaking and requires the owning source contract to change first.
- A later specialized viewer may narrow behavior for a format, but it
  must preserve the row language here in summaries, support exports,
  and review evidence.
