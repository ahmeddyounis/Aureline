# Debug artifact resolution seed

This document freezes the parity contract debugger UI, support bundles,
and the release artifact graph share for debugger-adjacent artifacts.
It extends the exact-build identity model and the local crash
symbolication smoke path beyond native crash bytes so native symbols
(PDB / dSYM / DWARF), JS / TS / CSS source maps, crash artifacts
(minidump / core dump / crash snapshot), generated-source mappings
(protobuf, OpenAPI, gRPC, bindgen, codegen), and coverage / profile
artifacts (LCOV, gcov, llvm-cov, pprof, perfetto trace, flamegraph)
all read one identity and one resolution state.

This is a parity seed, not an implementation. It is deliberately
narrower than a symbol-server and smaller than a debugger UI freeze.
The contract pins four things every downstream surface can cite
without inventing side metadata:

1. the shape of a `debug_artifact_manifest_record` and the reusable
   `debug_artifact_entry_record` every row resolves;
2. the resolution rules that say where a resolver may look and how
   it must verify what it found;
3. the closed mismatch / degraded-quality vocabulary the debugger UI,
   support bundle, and release evidence render verbatim; and
4. the parity seed that keeps the same manifest id visible on the
   debugger UI row, the support-bundle anchor, and the release
   artifact-graph node for one entry.

Companion artifacts:

- [`/schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
  — machine-readable boundary for the manifest and entry records.
- [`/fixtures/debug/mapping_cases/`](../../fixtures/debug/mapping_cases/)
  — worked mapping cases covering resolved, mismatched, degraded,
  unresolved, and generated-source rows.
- [`/artifacts/debug/artifact_resolution_examples/`](../../artifacts/debug/artifact_resolution_examples/)
  — concrete entry records and a primary manifest each case cites.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  — exact-build identity source every entry joins to.
- [`/docs/support/exact_build_symbolication_smoke.md`](../support/exact_build_symbolication_smoke.md)
  and
  [`/artifacts/support/crash_artifact_retention_seed.json`](../../artifacts/support/crash_artifact_retention_seed.json)
  — minimal local symbolication smoke path and shared crash-artifact
  retention / redaction seed the crash rows inherit.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — support/export bundle contract the manifest anchors into.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  and
  [`/docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  — release artifact-graph node map the manifest attaches to.
- [`/docs/release/build_farm_and_remote_cache_policy.md`](../release/build_farm_and_remote_cache_policy.md)
  — cache trust rules debug-artifact resolvers MUST honour when they
  pull from a remote cache; remote caches are not authoritative.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` §5.41, §5.43, §5.44, §10.13, §10.15, and
  §10.22.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §24.2 and
  §24.4.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §23.66 and §23.67.

If this seed disagrees with those sources, those sources win and this
document plus the schema update in the same change.

## Why freeze this now

The existing exact-build identity model already covers the binaries,
debug-symbols, source-map, and crash-symbols families every release
cut emits. The local crash-symbolication smoke path proves one
minimal path from a crash envelope to a symbolication report using
those fields. Neither artifact answers the broader question a
debugger surface asks every time a frame does not map:

> Where is this artifact, is it the right one for this build, how
> confident am I, and what should the user see if the answer is
> partial or negative?

Without this seed:

- debugger UI rows would invent a per-surface `resolved / unknown`
  badge and silently use the first file whose name matched;
- support bundles would attach "whatever symbols happened to be on
  disk" with no join back to the build they belonged to;
- release evidence would have to decide, per artifact family, what a
  "present but stale" row looks like;
- generated-source rows would lose the generating spec/tool when the
  spec was unknown, because the only escape token was "unknown";
- coverage and profile rows would have to redesign resolution before
  any surface could render them.

Freezing the manifest now means the debugger UI, the support bundle,
and the release artifact graph all carry the same `debug_artifact_ref`
for one entry and resolve the same closed vocabulary when they do.

## Scope

This seed freezes:

1. the `debug_artifact_manifest_record` (a header plus a list of
   entry records) and the `debug_artifact_entry_record` row shape
   every consuming surface reads;
2. the closed artifact-class vocabulary for native symbols, source
   maps, crash artifacts, generated-source mappings, and coverage /
   profile artifacts;
3. the workspace / target / build-identity linkage every entry
   carries;
4. the ordered resolution-source list plus verification rules for
   local cache, workspace outputs, trusted artifact stores, the
   release artifact graph, air-gapped mirrors, and user side-loads;
5. the closed mismatch-reason and degraded-quality vocabulary a
   resolver MUST emit when it cannot fully resolve an entry;
6. the parity seed binding debugger UI, support bundle, and release
   artifact-graph surfaces to the same manifest id.

Out of scope (the next tasks):

- a hosted symbol / source-map service or any concrete symbol-server
  protocol;
- the full debugger UI (breakpoint resolution, frame rendering, step
  behaviour, source-link fetch);
- concrete cache storage layout, eviction policy, or quota
  arithmetic beyond the posture-level rules in this document;
- signature / notarisation workflows for symbol packages;
- coverage / profile rendering UI.

## 1. Manifest and entry shape

Every debugger-adjacent artifact the workspace surfaces is described
by one `debug_artifact_entry_record`. One or more entries compose a
`debug_artifact_manifest_record`. Raw symbol bytes, raw dump bytes,
raw source bodies, and raw network URLs never cross this boundary.

### 1.1 Identity

- `record_kind` — `debug_artifact_entry_record` or
  `debug_artifact_manifest_record`.
- `debug_artifact_manifest_schema_version` — integer. Current value
  is `1`.
- `debug_artifact_ref` — opaque, stable id for one entry. Debugger
  UI, support bundle, and release artifact graph surfaces MUST
  resolve this id field-for-field instead of minting a parallel
  surface-local id.
- `manifest_id` — stable dot-id for a manifest (e.g.
  `debug.manifest.workspace.stable.linux.0001`).
- `display_label` — short redaction-safe label (e.g.
  `aureline-shell.pdb`, `renderer.main.bundle.js.map`,
  `api_client.ts (openapi-generator)`, `renderer-panic-minidump`).
  Raw absolute paths never appear here.

### 1.2 Artifact class (closed)

Every entry declares exactly one `artifact_class`:

| Class | Covers |
|---|---|
| `native_symbol_pdb` | Windows PDB (program database). |
| `native_symbol_dsym` | Apple dSYM bundle. |
| `native_symbol_dwarf` | Linux / BSD split DWARF or embedded DWARF archive. |
| `source_map_js` | JavaScript source map (`.js.map`). |
| `source_map_ts` | TypeScript-generated source map. |
| `source_map_css` | CSS / Sass / PostCSS source map. |
| `crash_artifact_minidump` | Windows / cross-platform minidump. |
| `crash_artifact_core_dump` | POSIX core dump. |
| `crash_artifact_snapshot` | Lightweight crash snapshot (in-process envelope). |
| `generated_source_mapping` | Generator-produced mapping between generated source and spec (protobuf, OpenAPI, gRPC, bindgen, codegen). |
| `coverage_lcov` | LCOV coverage artifact. |
| `coverage_gcov` | gcov coverage artifact. |
| `coverage_llvm_profraw` | llvm-cov raw profile artifact. |
| `profile_pprof` | pprof profile capture. |
| `profile_perfetto_trace` | Perfetto / chrome-trace capture. |
| `profile_flamegraph` | Flamegraph input capture. |

Adding a new artifact class is additive-minor and bumps the schema
version; repurposing an existing class is breaking.

### 1.3 Release-family join

`artifact_family_class` is optional and draws from the exact-build
vocabulary (`ide_debug_symbols`, `cli_debug_symbols`,
`source_map_bundle`, `crash_symbols_archive`,
`support_runbook_bundle`, `reproducibility_pack`,
`release_evidence_packet`). A debug-artifact entry never invents its
own family vocabulary. Workspace-local captures that are not release-
graph-bearing (most coverage / profile runs, local crash snapshots
before attach) leave this field null.

Rules (frozen):

1. When `artifact_family_class` is set, `surface_linkage.release_artifact_graph_node_ref`
   MUST be non-null; release-graph-bearing entries must resolve a
   graph node.
2. When `artifact_family_class` is null, the entry is a workspace-
   local capture and `storage_mode` MUST be `local_only_no_upload`,
   `local_cache_bounded`, `workspace_output_bounded`, or
   `manifest_only_no_body`.

### 1.4 Workspace binding

Every entry carries `workspace_binding`:

- `workspace_ref` — opaque workspace id. All entries in one manifest
  share this ref.
- `target_ref` — opaque target / build-target id (cargo
  package+binary, npm bundle name, codegen target, language-service
  target).
- `binding_role` — one of `primary_workspace_output`,
  `workspace_attached_cache`, `workspace_detached_upload`,
  `workspace_local_capture`, `workspace_reference_only`.
- `run_context_ref` — opaque run context (debug session, test run,
  profiler capture). Null when the entry is not tied to a specific
  run.
- `profile_ref` — opaque build-profile id (cargo profile, bundler
  config). Null when the artifact is profile-agnostic.

Rules (frozen):

1. An entry without a `workspace_ref` and a `target_ref` is
   non-conforming. Debug / profile / coverage artifacts with no
   target are not admissible at this boundary.
2. `workspace_local_capture` and `workspace_detached_upload`
   entries MUST name a `run_context_ref`; the debugger UI and the
   support bundle need the run context to render the row.

### 1.5 Build-identity linkage

`build_identity_linkage` names how the entry joins to a build
identity and which fields a resolver must match field-for-field
before claiming the entry is resolved.

- `linkage_state` — one of:
  - `linked_exact_build` (releases, CI artifacts). Requires
    `exact_build_identity_ref` non-null.
  - `linked_baseline_build` (developer lanes). Requires
    `baseline_build_identity_ref` non-null.
  - `linked_pending_identity` (capture in-flight). Resolves when the
    build completes.
  - `linked_external_build` (dependency artifact). Requires
    `external_build_identity_ref` non-null.
  - `unlinked_workspace_local` (local dev captures only). MUST pair
    with `binding_role = workspace_local_capture` and
    `storage_mode = local_only_no_upload` or `local_cache_bounded`.
- `expected_build_match_fields[]` — ordered list of fields a resolver
  MUST match field-for-field before the entry is considered
  resolved. The closed vocabulary includes `workspace_version`,
  `release_channel_class`, `commit.full_hash`, `commit.tree_hash`,
  `toolchain.toolchain_pin_digest`, `toolchain.lockfile_digest`,
  `target.target_triple`, `build_epoch.source_date_epoch`,
  `producer_lane.lane_class`, `profile.debug_info_class`,
  `profile.strip_class`, `profile.panic_class`,
  `module_identity.build_id`, `module_identity.debug_id`,
  `module_identity.source_map_digest`, `module_identity.uuid`,
  `generated_source.generator_identity`,
  `generated_source.spec_digest`, and
  `generated_source.generator_command_digest`.

Rules (frozen):

1. Every non-developer entry (`linked_exact_build`,
   `linked_external_build`) MUST declare at least one field from
   `module_identity.*`, `generated_source.*`, or the build-identity
   axes so a resolver has something to compare beyond "a file with
   the right name exists".
2. `unlinked_workspace_local` is admissible only on developer-only
   surfaces. The support-bundle preview renders these rows under
   `included_metadata_only` posture with a visible
   `unlinked_workspace_local` chip.

### 1.6 Resolution sources (ordered)

`resolution_sources[]` is the ordered list of sources the resolver
consulted. Order names precedence; the first entry is the preferred
source.

Closed `source_class`:

- `local_resolver_cache` — the bounded, inspectable, clearable
  debug-artifact cache. Cleared without deleting unrelated user
  state.
- `workspace_output_directory` — a local build output tree
  (`target/*`, `dist/*`, generated sources, coverage output).
- `workspace_attached_registry` — a workspace-scoped symbol / map
  registry the workspace manifest pinned.
- `trusted_artifact_store` — a verified release / CI artifact store
  (release artifact graph, CI trusted-cache classes).
- `release_artifact_graph` — resolution through the release-
  governance artifact graph node directly.
- `air_gapped_mirror` — offline mirror lane.
- `side_loaded_user_attached` — user-supplied attachment (explicit
  opt-in).

Each source entry carries a `trust_state`:

- `unverified` (may not resolve the entry on non-developer
  surfaces),
- `verified_digest_match` (clears most surfaces),
- `verified_signed_and_digest_match` (clears release-evidence
  surfaces),
- `rejected_unverified` (the source was consulted and rejected).

Rules (frozen):

1. A source whose `trust_state` is `unverified` or
   `rejected_unverified` MUST NOT resolve the entry on the debugger
   UI, support bundle, or release-evidence surfaces. The entry
   falls through to the next source in the list.
2. Remote caches are not authoritative. Cache trust rules in
   [`/docs/release/build_farm_and_remote_cache_policy.md`](../release/build_farm_and_remote_cache_policy.md)
   apply: a debug-artifact resolver MAY pull from a remote cache
   but MUST re-verify against the primary digest before marking
   `trust_state = verified_digest_match`.
3. `side_loaded_user_attached` is admissible only after an explicit
   user consent step. The support-bundle preview renders these
   rows under `included_redacted_body` or `opt_in_only` posture,
   never `included_metadata_only`.

### 1.7 Resolution state and mismatch reasons (closed)

`resolution_state` is the terminal answer the debugger UI, support
bundle, and release-evidence surfaces render:

- `resolved` — cleared for all surfaces; `mismatch_reasons` MUST be
  empty.
- `resolved_degraded_quality` — cleared with caveats;
  `degraded_quality_reasons[]` MUST be non-empty.
- `unresolved_not_cached` — the resolver did not find the artifact
  anywhere on the consulted source list.
- `unresolved_mismatch` — a candidate was found but one or more
  `expected_build_match_fields` did not match.
- `unresolved_cache_miss` — the local cache did not carry the
  artifact and higher-trust sources were unavailable or policy-
  blocked.
- `unresolved_trust_refused` — one or more candidates were rejected
  because their trust state did not clear.
- `unresolved_offline` — no higher-trust source was reachable.
- `unresolved_policy_blocked` — policy blocked egress or side-load.
- `pending_capture` — the capture is in-flight (build not yet
  complete, user upload not yet consented).

Every unresolved or degraded state MUST name at least one
`mismatch_reason_class` from the closed list. The list covers the
axes a debugger surface already distinguishes:

- build-identity axes (`mismatch_build_id`, `mismatch_commit_hash`,
  `mismatch_tree_hash`, `mismatch_toolchain_pin_digest`,
  `mismatch_lockfile_digest`, `mismatch_target_triple`,
  `mismatch_build_epoch`, `mismatch_producer_lane`,
  `mismatch_debug_info_class`, `mismatch_strip_class`);
- module-identity axes (`mismatch_module_build_id`,
  `mismatch_module_debug_id`, `mismatch_module_uuid`,
  `mismatch_source_map_digest`);
- generator axes (`mismatch_generator_identity`,
  `mismatch_generator_spec_digest`,
  `mismatch_generator_command_digest`);
- quality axes (`stale_source_map_mapping`,
  `partial_source_mapping_only`, `symbol_index_missing`,
  `split_symbols_missing`);
- trust / policy axes (`trust_store_rejected_source`,
  `policy_denied_egress`, `policy_denied_side_load`);
- capture axes (`pending_build_completion`,
  `pending_user_upload_consent`).

Rules (frozen):

1. A resolution-state other than `resolved` that does not name a
   reason from the closed list is non-conforming. Debugger UI MAY
   add human-readable copy around the token but MAY NOT collapse
   all unresolved states into one generic "symbols not found"
   string.
2. `resolved_degraded_quality` MUST carry at least one
   `degraded_quality_reason_class`: `line_tables_only`,
   `inline_frames_elided`, `third_party_frames_unmapped`,
   `source_map_missing_names`, `source_map_partial_content`,
   `generated_source_spec_unknown`,
   `generated_source_version_unknown`,
   `profile_sample_rate_reduced`, or `coverage_branches_unavailable`.
3. The resolver MUST NOT silently use mismatched symbols. An entry
   with a build-id or module-identity mismatch renders as
   `unresolved_mismatch` with the specific `mismatch_*` token and
   is not substituted with a best-effort guess.

### 1.8 Generator identity

Every `generated_source_mapping` entry and every `source_map_*`
entry carries a `generator_identity` block:

- `owner_class` — one of `native_compiler`, `web_bundler`,
  `codegen_tool`, `schema_compiler`, `coverage_tool`,
  `profiler_tool`, `kernel_runtime`, `mirror_source`, `unknown`.
- `display_label` — short label (e.g. `rustc 1.84.0`,
  `webpack 5.91.0`, `openapi-generator 7.6.0`, `protoc 26.1`).
- `version_or_revision_ref` — optional opaque ref to the pinned
  version.
- `spec_source_refs[]` — opaque refs to the spec the generator
  consumed (`.proto` digest row, `openapi.yaml` digest row,
  `schema.graphql` digest row). Required non-empty for
  `generated_source_mapping`.
- `command_ref` — opaque ref to the canonical generator-command
  record. Raw argv never appears here.
- `command_digest` — content digest of the canonical invocation.

Rules (frozen):

1. `generated_source_mapping` entries without a non-empty
   `spec_source_refs` are non-conforming. The point of the class
   is that a row can name its generating spec.
2. `owner_class = unknown` is admissible but pairs with
   `mismatch_generator_identity` or
   `generated_source_spec_unknown`. The manifest never silently
   claims "generated" without naming what failed to resolve.
3. `command_digest` is required on every
   `generated_source_mapping` entry whose resolution state is
   `resolved`; a resolved generated-source mapping must be able to
   prove the invocation was the one pinned by the spec.

### 1.9 Content linkage

`content_linkage` pins the artifact to a digest, a content-addressed
ref, and (when relevant) the module-identity fields crash / debugger
surfaces already use:

- `artifact_digest` — content digest pair.
- `content_addressed_ref` — opaque ref stable across all surfaces.
- `module_identity` — `build_id`, `debug_id`, `uuid`,
  `code_file_name`, `source_map_digest`. Present on
  `native_symbol_*`, `source_map_*`, and `crash_artifact_*`.
- `sibling_identity_refs[]` — opaque refs to sibling debug-artifact
  entries that must resolve together (a minidump + its
  crash-symbols archive, a JS source map + its bundle, a codegen
  output + its spec digest row). A parity audit MUST validate
  these refs point to the same `build_identity_linkage`.

Rules (frozen):

1. Siblings that do not share the same
   `build_identity_linkage.exact_build_identity_ref` (or baseline /
   external equivalent) render as `unresolved_mismatch`; the
   manifest MAY NOT claim a split-symbols pair is resolved when
   the archive is from a different build.
2. Module-identity fields are canonical. When the module-identity
   fields are present, `expected_build_match_fields` MUST include
   the corresponding `module_identity.*` axis.

### 1.10 Storage and export posture

- `storage_mode` — one of `local_cache_bounded`,
  `workspace_output_bounded`, `trusted_store_managed_reference`,
  `release_graph_managed_reference`, `air_gapped_mirror_reference`,
  `local_only_no_upload`, `manifest_only_no_body`.
- `support_export_posture` — one of `included_metadata_only`,
  `included_redacted_body`, `opt_in_only`, `excluded_by_policy`,
  `excluded_by_user`.
- `redaction_class` — one of `metadata_safe_default`,
  `support_redaction_applied`, `operator_only_restricted`,
  `internal_support_restricted`, `evidence_packet_only`,
  `release_public`.

Rules (frozen):

1. Raw dump bytes, raw core bytes, and raw symbol bytes never
   travel inside a support bundle under `included_metadata_only`.
   `crash_artifact_core_dump` entries default to
   `local_only_no_upload` + `opt_in_only` and the
   `internal_support_restricted` redaction class (mirroring the
   crash-artifact retention seed).
2. An `excluded_by_policy` or `excluded_by_user` entry MUST still
   appear in the manifest — the omission is visible, not silent.
3. Coverage and profile bytes follow the workspace-attached posture
   by default: `workspace_output_bounded` +
   `included_metadata_only` with a `profile_sample_rate_reduced`
   or `coverage_branches_unavailable` annotation when degraded.

### 1.11 Surface linkage and parity

`surface_linkage` names the debugger UI row, support-bundle anchor,
release artifact-graph node, crash-envelope ref, and symbolication-
report ref this entry reaches. Every surface resolves the same
`debug_artifact_ref` for one logical entry. Parity rules:

1. The debugger UI row, the support-bundle manifest row, and the
   release artifact-graph node MUST all render the same
   `debug_artifact_ref`, `artifact_class`, `resolution_state`,
   `mismatch_reasons`, and `build_identity_linkage` tokens.
   Surface chrome may reorder copy but MAY NOT mint a parallel
   badge, label, or status string.
2. Support bundles MAY include the manifest row by reference without
   forcing raw artifact upload: `storage_mode =
   trusted_store_managed_reference` or
   `release_graph_managed_reference` is admissible under
   `support_export_posture = included_metadata_only`.
3. Release evidence packets attach the manifest row through the
   release artifact-graph node ref; the release packet MAY NOT
   duplicate the entry body inline.

## 2. Resolution rules

A resolver walks `resolution_sources[]` in order and verifies every
candidate before resolving. The rules below are minimum floors;
concrete implementations may be stricter but MUST NOT be weaker.

### 2.1 Local cache

- The local resolver cache is bounded, inspectable, and clearable
  without deleting unrelated user state.
- A cache hit MUST re-verify `artifact_digest` before claiming
  `verified_digest_match`. A stale cache row is treated as a cache
  miss, not a resolve.
- The cache MAY serve entries offline. When a higher-trust source
  later invalidates a cached entry, the cache row is evicted and
  the entry's `resolution_state` flips to `unresolved_cache_miss`
  until the next walk.

### 2.2 Workspace outputs

- Workspace outputs are authoritative for `linked_baseline_build`
  entries on developer lanes.
- When a workspace output is also a release-promoted artifact, the
  release artifact-graph node remains the canonical source for
  non-developer surfaces; the workspace output is a convenience
  mirror.
- Workspace outputs that do not match the resolved baseline build
  identity resolve as `unresolved_mismatch` with the specific
  mismatch token, not `unresolved_not_cached`.

### 2.3 Trusted artifact store and release artifact graph

- Both sources MUST verify a release-signed digest before
  `verified_signed_and_digest_match` may be claimed. The debug-
  artifact resolver honours the signing-material state on the
  joined exact-build identity.
- The release artifact graph is the canonical source for release-
  anchored manifests. A manifest whose `primary_build_identity_ref`
  names a release-channel identity MUST consult the graph first
  and fall back only on explicit offline / mirror posture.

### 2.4 Air-gapped mirror

- Mirror resolution follows the mirror trust rules pinned by the
  build-farm / remote-cache policy. The mirror body carries the
  same digest and signed attestation as the release artifact; the
  manifest marks `trust_state = verified_signed_and_digest_match`
  only when both digest and signature verify against the mirror-
  shipped attestation.
- Mirror-resolved entries MUST still point
  `surface_linkage.release_artifact_graph_node_ref` at the release
  graph node the mirror copies. Mirrors do not re-anchor identity.

### 2.5 User side-load

- User side-load is admissible only after an explicit consent
  step. The manifest records the consent outcome in
  `resolution_sources[i].trust_state` (`unverified` until digest
  match, then `verified_digest_match`).
- Side-load does not clear `verified_signed_and_digest_match`
  unless the user-supplied file carries the release signature and
  the signature verifies.

### 2.6 Mismatch and error disclosure

- A mismatched symbol, a stale source map, or a partial source
  mapping becomes an explicit `resolution_state` +
  `mismatch_reason_class` pair. The debugger UI renders this pair
  verbatim; the support-bundle preview prints the pair as a
  reviewable manifest row; the release-evidence surface records
  the pair on the artifact-graph node so a reviewer can audit
  without opening the binary.
- A resolver MUST NOT fall back to a best-effort guess when the
  build-id, module-identity, or source-map digest mismatches. The
  row stays unresolved; downstream surfaces render the specific
  mismatch token.

## 3. Parity seed

The worked manifest in
[`/artifacts/debug/artifact_resolution_examples/primary_manifest.json`](../../artifacts/debug/artifact_resolution_examples/primary_manifest.json)
binds the debugger UI, support bundle, and release artifact graph to
the same entries. Every row:

- appears on the debugger UI with a `debugger_ui_row_ref`;
- appears on the support-bundle anchor with a
  `support_bundle_anchor_ref` (or an explicit
  `excluded_by_policy / excluded_by_user` omission);
- appears on the release artifact-graph node with a
  `release_artifact_graph_node_ref` when the entry is release-
  graph-bearing.

The mapping cases under
[`/fixtures/debug/mapping_cases/`](../../fixtures/debug/mapping_cases/)
cover one exemplar per axis the seed must keep distinguishable:

| Case | Covers |
|---|---|
| `debug.mapping.native_symbol_pdb.resolved` | Release-anchored PDB resolved from the release artifact graph with a signed-and-digest-match source. |
| `debug.mapping.native_symbol_dwarf.split_symbols_missing` | Linux DWARF sidecar missing the split-symbols archive; `split_symbols_missing` mismatch token. |
| `debug.mapping.native_symbol_dsym.mismatch_build_id` | Apple dSYM whose module UUID does not match the running binary; `mismatch_module_uuid` + `mismatch_module_build_id` tokens. |
| `debug.mapping.source_map_js.stale_mapping` | JS source map whose source-map digest matches but spans a stale build; `stale_source_map_mapping` degraded-quality path. |
| `debug.mapping.source_map_css.partial_mapping` | CSS source map resolved with `partial_source_mapping_only`; renders as `resolved_degraded_quality`. |
| `debug.mapping.crash_minidump.resolved_with_siblings` | Minidump resolved alongside its crash-symbols archive via sibling-identity refs. |
| `debug.mapping.crash_core_dump.pending_upload_consent` | Core dump in `pending_capture` awaiting `pending_user_upload_consent`. |
| `debug.mapping.generated_source.openapi_resolved` | OpenAPI-generated TypeScript client resolved with spec digest + generator command digest. |
| `debug.mapping.generated_source.spec_unknown` | Generated client whose generating spec is unknown; resolves `degraded` with `generated_source_spec_unknown`. |
| `debug.mapping.coverage_lcov.workspace_output` | LCOV coverage pulled from the workspace output directory with a `workspace_local_capture` role. |
| `debug.mapping.profile_pprof.side_loaded` | Side-loaded pprof profile under `opt_in_only` export posture. |

Each case joins to one entry record under
[`/artifacts/debug/artifact_resolution_examples/`](../../artifacts/debug/artifact_resolution_examples/)
so a reviewer can read the fixture manifest and the entry record in
parallel.

## 4. Change rules

- Adding a new `artifact_class`, `resolution_source_class`,
  `resolution_state_class`, `mismatch_reason_class`,
  `degraded_quality_reason_class`, `generator_owner_class`,
  `storage_mode_class`, `support_export_posture_class`, or
  `workspace_binding_role_class` is additive-minor and bumps
  `debug_artifact_manifest_schema_version`; it must update this
  seed and at least one worked fixture in the same change.
- Repurposing an existing token is breaking and requires a new
  decision row plus companion updates to the exact-build identity
  model, the support-bundle contract, the release artifact-family
  map, and the crash-artifact retention seed.
- Debugger UI, support-bundle, and release artifact-graph surfaces
  MUST adopt new tokens by reference, not by duplication.

## 5. Source anchors

- `.t2/docs/Aureline_PRD.md` §5.41 — Support Center scope including
  symbol/source-map retention policy appropriate to channel.
- `.t2/docs/Aureline_PRD.md` §5.43 — native symbols, web source
  mapping, crash artifacts, generated-source mappings, and
  coverage / profile artifacts must bind to workspace + target +
  build identity with explicit mismatch disclosure and
  manifest-without-upload behaviour for exports.
- `.t2/docs/Aureline_PRD.md` §10.13, §10.15, §10.22 — supportability,
  redaction, and field-diagnostics posture the manifest inherits.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §24.2 and
  §24.4 — supportability plane and evidence composition rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §23.66 and §23.67 —
  debugger UI must name build-id mismatches, stale source maps, and
  unavailable debug data with stable labels, not generic failures.
