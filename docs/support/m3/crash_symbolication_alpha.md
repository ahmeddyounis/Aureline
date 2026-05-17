# Crash envelope, exact-build symbol manifest, and local symbolication alpha

The crash symbolication alpha lane preserves trustworthy field
forensics by capturing a metadata-safe crash envelope tied to an exact
build, binding that envelope to a release-side **symbol manifest**, and
admitting a local symbolication path where in-tree symbols are
available. The lane fails closed: when build identities disagree, the
binding refuses to claim exact symbolication; when no manifest is
bound, the binding preserves crash refs without implying any symbol
coverage exists.

The implementation lives in
[`crates/aureline-crash/src/envelope/mod.rs`](../../../crates/aureline-crash/src/envelope/mod.rs)
and the boundary schema lives at
[`schemas/support/crash_symbolication_manifest_alpha.schema.json`](../../../schemas/support/crash_symbolication_manifest_alpha.schema.json).
The release-side symbol manifest lives at
[`artifacts/release/m3/symbol_manifest/symbol_manifest.json`](../../../artifacts/release/m3/symbol_manifest/symbol_manifest.json)
and the protected fixture corpus lives at
[`fixtures/support/crash_symbolication_alpha/`](../../../fixtures/support/crash_symbolication_alpha/).

## What this alpha row owns

- A typed [`SymbolManifest`] record that is the release-side
  declaration of per-module symbol identities for one exact-build
  identity. Each row is one `(module_id, module_kind,
  artifact_family_class, exact_build_identity_ref,
  symbolication_identity_ref, optional support_archive_identity_ref,
  storage_class, notes)` tuple. Native binaries carry `build_id`,
  `debug_id`, and `code_file_name`; source-map bundles carry
  `bundle_revision_ref`, `source_map_digest`, and
  `generated_asset_ref`. The manifest is metadata only — every row
  pins `storage_class: metadata_only_no_symbol_bytes` and the manifest
  itself pins `redaction_class: metadata_safe_default`.
- A typed [`CrashEnvelopeSymbolBinding`] record that the support
  pipeline mints from one [`CrashEnvelope`], an optional
  [`SymbolManifest`], and an optional in-tree
  [`SymbolicationReport`]. The binding labels the result as one of
  `linked`, `partial`, `missing_manifest`, or `build_mismatch`, emits a
  per-module binding row for every envelope module *and* every
  manifest module the envelope did not name, and quotes the doc and
  schema refs verbatim on every record.
- The [`bind_crash_envelope`] entry point that the support pipeline
  uses to fold already loaded records into a binding without reading
  raw dump bytes, raw stack bodies, or platform-debugger output.
- The boundary JSON schema and a protected fixture corpus covering the
  `linked`, `partial`, and `build_mismatch` paths; the
  `missing_manifest` path is covered by passing `None` rather than by
  shipping an empty-pseudo-record.

## Acceptance and how this row meets it

- **Crash artifacts can be linked to an exact build, symbol manifest,
  and local symbolication result or explicit mismatch state.** Every
  binding carries the `crash_envelope_ref`, the
  `primary_exact_build_identity_ref`, and (when bound) the
  `symbol_manifest_ref`, `manifest_primary_exact_build_identity_ref`,
  and `symbolication_report_ref`. The `binding_state` enum is closed
  to `linked`, `partial`, `missing_manifest`, and `build_mismatch`,
  and the evaluator routes module-identity disagreements to
  `build_mismatch` instead of silently downgrading to `partial`.
- **Local users can inspect or export crash artifacts without raw
  private data leaking by default.** Every binding pins
  `raw_private_material_excluded = true`,
  `ambient_authority_excluded = true`, and `raw_dump_exported = false`.
  The default `support_export_posture` is `metadata_only_default`;
  raw minidump or core bodies remain governed by
  `support.item.crash_dump_or_core` and stay local-only unless a
  separate reviewed upload path is approved. The
  [`CrashEnvelopeSymbolBinding::is_export_safe`] helper folds those
  three pinned fields into one reviewer-facing check.
- **Release packets preserve the same build IDs and symbol references
  that crash support expects.** The release-side symbol manifest at
  `artifacts/release/m3/symbol_manifest/symbol_manifest.json` declares
  the beta candidate's `primary_exact_build_identity_ref` and the
  per-module `module_id` / `exact_build_identity_ref` /
  `symbolication_identity_ref` tuples that the artifact graph names. The
  protected `release_symbol_manifest_matches_beta_artifact_graph_identity`
  test re-proves this every run. The alpha crash-envelope linked path
  keeps its preview fixture under `fixtures/support/crash_symbolication_alpha/`
  so old crash drills still prove `linked`, `partial`,
  `missing_manifest`, and `build_mismatch` without pretending to cover
  the beta candidate.

## Failure-drill posture

The binding lane fails closed before claiming exact symbolication:

- A manifest whose `primary_exact_build_identity_ref` does not match
  the crash envelope's identity yields `binding_state =
  build_mismatch` with a per-module
  `primary_exact_build_identity_mismatch` reason.
- A module whose envelope identity escapes the runtime family yields a
  per-module `module_exact_build_identity_outside_runtime_family`
  reason on the `IdentityMismatch` row.
- A module whose envelope `module_kind` does not match the manifest
  `module_kind` is flagged with `module_kind_mismatch` and routed to
  `build_mismatch`.
- An in-tree symbolication report whose `runtime_identity_ref` or
  `symbolication_identity_ref` escapes the runtime family downgrades
  any otherwise-linked row to `IdentityMismatch` and the binding to
  `build_mismatch`.
- When no manifest is bound, every envelope module surfaces as
  `extra_in_envelope`, the binding labels `missing_manifest`, and the
  honesty notes say so out loud rather than silently green.
- When the envelope and manifest declare different module sets, the
  binding labels `partial` and emits both `missing_from_manifest` and
  `extra_in_envelope` rows so reviewers see exactly what is missing or
  extra.

## First consumers

- The `aureline-crash` envelope module is the canonical projection for
  release-side packaging review and support-export consumption.
  `bind_crash_envelope` folds one envelope, manifest, and optional
  symbolication report into one metadata-safe binding that the
  support-export pipeline can serialize verbatim.
- The boundary schema is the contract the headless export writer and
  the support-export chrome share — both reconstruct the same record
  shape from the on-disk file verbatim, never re-derive it from a side
  channel.
- The release-side symbol manifest under
  `artifacts/release/m3/symbol_manifest/` is the checked-in proof that
  release packets preserve the same build IDs and symbol references
  that crash support expects.

## Related contracts

- [Crash symbolication linkage alpha packet](../../../artifacts/support/crash_symbolication_linkage_alpha.md)
  — the parent alpha packet that already wired the crash incident
  trail, the in-tree symbolication report, and the support-bundle
  preview surface. This M3 row adds the release-side symbol manifest
  and the typed envelope binding that ties the two together.
- [Recovery-ladder alpha](../recovery_ladder_alpha.md) — the parent
  rung contract the crash binding feeds into when a blocked user
  exports evidence.
- [Support-bundle contract](../support_bundle_contract.md) — the
  parent contract for every metadata-safe support projection. The
  crash envelope symbol binding is one such projection.

## Out of scope for this alpha row

- Live ingestion of raw minidump or core bytes. The binding is
  computed from already typed envelope, manifest, and report records.
- Public symbol-server fetch paths. The alpha lane is local-first; any
  remote symbol-server integration is governed by a later beta row.
- Hosted upload of crash bundles. Raw dump bytes remain governed by
  `support.item.crash_dump_or_core` and stay local-only until an
  explicit reviewed upload path is approved.

[`SymbolManifest`]: ../../../crates/aureline-crash/src/envelope/mod.rs
[`CrashEnvelopeSymbolBinding`]: ../../../crates/aureline-crash/src/envelope/mod.rs
[`CrashEnvelope`]: ../../../crates/aureline-crash/src/incident_trail.rs
[`SymbolicationReport`]: ../../../crates/aureline-crash/src/incident_trail.rs
[`bind_crash_envelope`]: ../../../crates/aureline-crash/src/envelope/mod.rs
[`CrashEnvelopeSymbolBinding::is_export_safe`]: ../../../crates/aureline-crash/src/envelope/mod.rs
