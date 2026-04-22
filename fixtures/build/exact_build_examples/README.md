# Exact-build identity fixtures

Worked fixtures for the exact-build identity model frozen in
[`/docs/build/exact_build_identity_model.md`](../../../docs/build/exact_build_identity_model.md).
Every fixture here conforms to
[`/schemas/build/exact_build_identity.schema.json`](../../../schemas/build/exact_build_identity.schema.json).

The fixtures exist so the reproducible-build baseline, release-
evidence, artifact-graph, SBOM, attestation, crash-symbolication,
support-export, Help / About, and docs / search version-match lanes
can write against a shared corpus of identity records without
inventing per-family version strings. Each file carries a
`__fixture__` section summarising the scenario, the axes it
exercises, and the contract sections it illustrates. The top-level
record conforms to the schema so tooling can validate the file as
an integration check.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/build/exact_build_identity.schema.json`](../../../schemas/build/exact_build_identity.schema.json).
  A fixture that fails validation is a bug in the fixture, not in
  the schema.
- **Projection-parity corpus.** A later parity audit between the
  binary's embedded identity, the Help / About publisher payload,
  the docs-pack manifest's `running_build_identity_ref`, the
  search-result-truth `running_build_identity_ref`, the SBOM's
  subject identifier, and the release-evidence packet compares
  emitted rows for the same `exact_build_identity_ref`
  field-for-field. These fixtures are the reference identities
  that audit reads.
- **Diffing two nearby builds.** The eight fixtures span one commit
  (`a4d1c3f0e27b`, stable `0.7.3`) cut across five artifact
  families (`ide_binary`, `ide_debug_symbols`, `source_map_bundle`,
  `crash_symbols_archive`, `docs_pack`) plus a clean-room rebuild of
  that same release, and a separate hotfix (`0.7.3-hotfix.1`) and
  nightly (`0.8.0-nightly.20260421`). A diffing tool can read any
  two fixtures and enumerate the axes that differ mechanically.

## Required fields (per the schema)

A record MUST carry, at minimum:

- `exact_build_identity_schema_version`, `record_kind`,
  `exact_build_identity_ref`,
- `product_name_class`, `workspace_version`, `semver_core`,
- `release_channel_class`, `artifact_family_class`,
- `commit` (full_hash, short_hash, hash_algorithm,
  committer_epoch),
- `tree_state_class`, `tree_state_rejected_reasons`,
- `toolchain` (channel, rustc_version, cargo_version, host_triple,
  toolchain_pin_digest, lockfile_digest),
- `target` (target_triple, target_os_class, target_cpu_class,
  target_tier_class),
- `profile` (cargo_profile_class, opt_level_ref,
  debug_info_class, panic_class, strip_class),
- `build_epoch` (source_date_epoch, build_timestamp_utc,
  derivation_class),
- `producer_lane` (lane_class, producer_identifier_ref,
  rebuild_of_identity_ref),
- `provenance` (signing_class, signing_material_state,
  attestation_predicate_refs, sbom_document_ref,
  provenance_statement_ref, transparency_log_ref),
- `propagation` (binary_embed_class, symbol_tag_refs),
- `evidence` (build_log_ref, reproducibility_pack_ref,
  artifact_graph_node_ref, and the optional *_ref links),
- `policy_context`, `redaction_class`, and `minted_at`.

No "version only" fallback is admissible. A record that would
carry only `workspace_version` without the full axis set is a
schema violation.

## Fixtures

- [`developer_local_ide_binary.json`](./developer_local_ide_binary.json)
  — developer workstation build on aarch64-apple-darwin with
  `tree_state_class = dirty_local`, `signing_class =
  unsigned_development_only`, and `binary_embed_class =
  embedded_in_binary`. Exercises the dev-lane-tolerates-dirty-tree
  rule.
- [`ci_release_stable_linux_ide_binary.json`](./ci_release_stable_linux_ide_binary.json)
  — the full production release of the IDE binary on
  x86_64-unknown-linux-gnu. `signing_material_state =
  signed_verified` clears the release-evidence gate. `debug_info_class
  = split_debug_info_external` cross-references the split-symbols
  identity via `evidence.split_symbols_ref`; `source_map_manifest_ref`
  cross-references the paired renderer source-map sidecar.
- [`ci_release_stable_linux_ide_debug_symbols.json`](./ci_release_stable_linux_ide_debug_symbols.json)
  — companion ide_debug_symbols identity minted alongside the
  production release. Shares commit / toolchain / target / producer
  lane / provenance with its paired binary; differs on
  `artifact_family_class`, `profile` (no strip), and
  `propagation.binary_embed_class = sidecar_manifest_beside_binary`.
- [`ci_release_stable_linux_source_map_bundle.json`](./ci_release_stable_linux_source_map_bundle.json)
  — renderer source-map sidecar minted alongside the production linux
  release. Shares the stable build tuple with the paired runtime
  binary; differs on `artifact_family_class = source_map_bundle` and
  the sidecar propagation path used by symbolication.
- [`ci_release_stable_linux_crash_symbols_archive.json`](./ci_release_stable_linux_crash_symbols_archive.json)
  — crash-symbol archive retained for the same stable linux release.
  Shares the stable build tuple with the paired runtime binary and
  carries the same GNU build-id tag the crash smoke path validates.
- [`ci_release_stable_docs_pack.json`](./ci_release_stable_docs_pack.json)
  — docs_pack cut from the same stable release. OS / CPU-agnostic
  axes (`target_os_class = other_unspecified`, `target_cpu_class =
  cpu_agnostic`, `target_tier_class = tier_not_applicable`) and
  `binary_embed_class = external_registry_only`. The
  docs-pack manifest's `running_build_identity_ref` resolves to
  this record.
- [`ci_nightly_preview_macos_ide_binary.json`](./ci_nightly_preview_macos_ide_binary.json)
  — nightly ide_binary on aarch64-apple-darwin with `signing_class
  = ci_transparent_sign` and `signing_material_state =
  signed_unverified`. Surfaces render the docs
  `pre_release_unverified` version-match state against this
  identity.
- [`clean_room_reproduction_of_stable_release.json`](./clean_room_reproduction_of_stable_release.json)
  — independent verifier rebuild of the stable ide_binary.
  `producer_lane.lane_class = reproduced_clean_room` and
  `rebuild_of_identity_ref` resolves to the original release's
  `exact_build_identity_ref`. Exercises the diff-two-nearby-builds
  contract field-for-field.
- [`ci_hotfix_release_sbom_document.json`](./ci_hotfix_release_sbom_document.json)
  — SBOM document cut on a hotfix release of the stable line.
  `release_channel_class = hotfix`, `artifact_family_class =
  sbom_document`, `binary_embed_class =
  embedded_in_archive_manifest`. Exercises the non-binary artifact
  family pattern and shows how the hotfix line references the
  stable line via `semver_prerelease_ids = ["hotfix", "1"]`.

## Related schemas and artifacts

- [`/schemas/build/build_identity.schema.json`](../../../schemas/build/build_identity.schema.json)
  — the minimal reproducible-build baseline record. Every exact-
  build identity MUST be derivable from the baseline record for the
  same `(commit, toolchain, target, profile)` triple; this is the
  super-set the baseline extends into.
- [`/schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json)
  and [`/schemas/docs/help_status_badge.schema.json`](../../../schemas/docs/help_status_badge.schema.json)
  — consume `running_build_identity_ref` as an opaque pin that
  resolves to an `exact_build_identity_record`.
- [`/schemas/search/search_result_truth.schema.json`](../../../schemas/search/search_result_truth.schema.json)
  — consumes `running_build_identity_ref` on every emitted
  search-result-truth record.
