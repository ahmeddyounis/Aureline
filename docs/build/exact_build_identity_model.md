# Exact-build identity model

This document freezes the exact-build identity model every release
lane, every artifact family, and every user-visible surface that
answers "what build is this?" projects from. The machine-readable
boundary is
[`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json);
worked examples (developer-local, stable release across three
artifact families, nightly preview, clean-room reproduction, hotfix
SBOM) live in
[`/fixtures/build/exact_build_examples/`](../../fixtures/build/exact_build_examples/).

The reproducible-build baseline record pinned in
[`docs/build/reproducible_build_baseline.md`](./reproducible_build_baseline.md)
is the minimal per-build identity; this document extends that
baseline into the single super-set every artifact family resolves a
`running_build_identity_ref` into. If the baseline and this document
disagree, the baseline's four pinned files (`rust-toolchain.toml`,
`.cargo/config.toml`, `Cargo.toml`, `Cargo.lock`) and its emitted
`build_identity.json` remain the contract on the builder side, and
this document's schema is the cross-artifact boundary; both MUST be
updated in the same change.

Companion artifacts:

- [`/docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md`](../adr/0017-release-posture-artifact-families-and-promotion-gates.md)
  — release-posture ADR that freezes channel posture, RC-as-stage, the
  coordinated rollback atom, and release-blocking gate policy.
- [`/docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  and
  [`/artifacts/release/artifact_graph_rules.yaml`](../../artifacts/release/artifact_graph_rules.yaml)
  — release-artifact graph completeness rules that bind this identity
  model to docs/help truth, benchmark-publication packs, debug sidecars,
  advisories, and promotion evidence.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  — family-level map that assigns release posture, owner lane,
  rollback-atom membership, retention floor, and support packet class
  to each exact-build artifact family.
- [`/docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md)
  — aggregate release-truth packet that cites exact-build identities by
  stable ref.

## Why freeze this now

The foundations baseline ships a single `build_identity.json` per
build of the workspace. That record answers "what did this builder
produce right now?" but it does not answer "which artifact family
am I holding, on which release channel, from which producer lane,
covered by which attestations, and how does that pin into the
artifact graph, the Help / About dialog, the docs-pack manifest,
the search-result-truth record, the crash-symbol archive, and the
public release-evidence packet without each surface re-inventing
version semantics?"

Left implicit:

- Binaries embed one version string, `.pdb` / split-DWARF files
  embed another build-id, docs packs pin a third
  `semver_version`, SBOM documents carry a fourth subject id, and
  release notes render a fifth. A rollback or crash triage would
  spend its first hour reconciling five dialects of "the build".
- Clean-room reproduction has no canonical axis list to diff
  against, so "we reproduced the build" degrades into "the bytes
  match in these files, shrug on the rest".
- The docs version-match axis frozen in ADR-0013
  (`exact_build_match`, `compatible_minor_drift`,
  `incompatible_drift_detected`, `pre_release_unverified`,
  `unknown_target_build`) reserves a `running_build_identity_ref`
  field but has no schema to resolve that ref into. Surfaces that
  render the chip would silently fall back to a version string.

Freezing the identity as a single record with a closed axis set
ends each of those failure modes: every surface resolves one
opaque `exact_build_identity_ref` into a record whose fields have
governed semantics.

## Scope

Frozen at this revision:

- One `exact_build_identity_record` shape with a closed set of
  product-name, release-channel, artifact-family, tree-state,
  tree-state-rejected-reason, cargo-profile, debug-info, panic,
  strip, target-OS, target-CPU, target-tier, build-epoch-derivation,
  producer-lane, signing, signing-material-state, binary-embed,
  hash-algorithm, and redaction classes.
- One identity per `(exact_build_identity_ref, artifact_family_class)`
  pair. A single release cuts N records (one per artifact family)
  that share commit / tree-state / toolchain / target / build epoch
  / producer lane / provenance and differ only on artifact family,
  profile axis, propagation envelope, and evidence linkage.
- The rule that `running_build_identity_ref` is the only cross-
  surface name admissible for the running build; no parallel
  "version string" is carried on any downstream record. "Version
  only" fallback is forbidden by schema (see "No version-only
  fallback" below).
- Propagation rules: binaries embed the identity, split symbols
  ship sidecar manifests, docs packs / schema exports / reference
  packs resolve through the registry, source bundles /
  reproducibility packs / release-evidence packets carry it in the
  archive manifest.
- Evidence linkage: every identity points back at the build log,
  reproducibility pack, artifact-graph node, support bundle (when
  applicable), Help / About payload (when applicable), and public
  release-note row (on release lanes).

Out of scope until a superseding decision row opens:

- Final signing / notarization workflows. This document pins the
  signing-class and signing-material-state vocabulary every surface
  reads; the actual key material, transparency-log bridge, and
  notarization pipeline are later lanes.
- Full byte-identical artifact reproducibility (the baseline
  already calls this out as a later milestone).
- The binary's in-process resolver for the embedded identity (the
  Help / About publisher, the crash-symbolication path). This
  document freezes the record they project from; the resolver is
  implementation.
- SBOM / attestation predicate bodies. This document pins
  `sbom_document_ref`, `provenance_statement_ref`,
  `transparency_log_ref`, and `attestation_predicate_refs` as
  opaque refs; the predicate schemas themselves (SPDX, CycloneDX,
  SLSA, in-toto) are governed by their upstream specs and are not
  re-minted here.

## No version-only fallback

The schema requires, on every identity:

- `commit` (full_hash, short_hash, hash_algorithm, committer_epoch),
- `tree_state_class`,
- `toolchain` (channel, rustc_version, cargo_version, host_triple,
  toolchain_pin_digest, lockfile_digest),
- `target` (target_triple, target_os_class, target_cpu_class,
  target_tier_class),
- `artifact_family_class`,
- `release_channel_class`,
- `profile` (cargo_profile_class, opt_level_ref, debug_info_class,
  panic_class, strip_class),
- `build_epoch` (source_date_epoch, build_timestamp_utc,
  derivation_class),
- `producer_lane` (lane_class, producer_identifier_ref,
  rebuild_of_identity_ref),
- `provenance` (signing_class, signing_material_state,
  attestation_predicate_refs, sbom_document_ref,
  provenance_statement_ref, transparency_log_ref),
- `evidence` (build_log_ref, reproducibility_pack_ref,
  artifact_graph_node_ref, plus the optional links).

A record that would ship with only `workspace_version` and none of
the axes above is a schema violation, not a legacy-fallback. The
`semver_core` / `semver_prerelease_ids` / `semver_build_metadata`
split exists so surfaces that only want to render a short version
string can do so from the identity record without carrying any
other version axis separately.

## Record fields

The full field set lives in
[`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json).
The notable fields are:

- **Identity.** `exact_build_identity_ref` is the opaque,
  globally-unique pin every downstream `running_build_identity_ref`
  resolves to. `record_kind` is `exact_build_identity_record`.
  `exact_build_identity_schema_version` is the integer schema
  version (currently `1`).
- **Product version.** `product_name_class = aureline`,
  `workspace_version` is the rendered display version, and
  `semver_core` / `semver_prerelease_ids` /
  `semver_build_metadata` carry the structured split. The build
  metadata tokens MUST NOT embed a wall-clock timestamp; the
  committer timestamp lives in `build_epoch.source_date_epoch`.
- **Channel.** `release_channel_class` is one of `dev_local`,
  `nightly`, `preview`, `beta`, `stable`, `lts`, `hotfix`.
  Downstream surfaces compute the docs-version-match axis
  (ADR-0013) against this value: `stable` / `lts` / `hotfix`
  render `exact_build_match`; `nightly` / `preview` / `beta`
  render `pre_release_unverified` by default. `rc_or_stable_candidate`
  is a promotion-review stage from
  `artifacts/release/promotion_gate_map.yaml`, not a new
  `release_channel_class` value.
- **Artifact family.** `artifact_family_class` is the closed set
  of artifact-family tokens
  (`ide_binary`, `ide_debug_symbols`, `cli_binary`,
  `cli_debug_symbols`, `sdk_library`, `source_map_bundle`,
  `docs_pack`, `schema_export`, `reference_pack`, `sbom_document`,
  `signed_attestation`, `source_bundle`, `crash_symbols_archive`,
  `reproducibility_pack`, `support_runbook_bundle`,
  `release_evidence_packet`). Family-level release posture, promotion
  owner lane, rollback-atom membership, and retention floor are
  governed by `artifacts/release/artifact_family_map.yaml`; the schema
  remains the closed vocabulary source for the family ids themselves.
- **Commit / tree state.** `commit` carries full hash, short hash
  (12 hex chars, rendering-only), hash algorithm, committer epoch,
  and tree hash. `tree_state_class` is
  `clean_checkout` / `dirty_local` / `patched_from_clean` /
  `shallow_clone_accepted` / `archive_export_no_git`.
  `tree_state_rejected_reasons` is empty on tolerated states and
  non-empty whenever a release lane observes a non-clean state.
- **Toolchain.** `toolchain` pairs the rust-toolchain.toml channel,
  rustc / cargo version strings, the host triple, the LLVM
  version, and content digests of `rust-toolchain.toml` and
  `Cargo.lock`. The digests let a verifier detect a bumped pin
  between two otherwise-identical identities without reading the
  files.
- **Target.** `target` carries the target triple plus typed
  `target_os_class`, `target_cpu_class`, and `target_tier_class`
  so parity audits can diff without parsing the triple. OS /
  CPU-agnostic artifact families (docs_pack, schema_export,
  reference_pack, sbom_document, reproducibility_pack,
  release_evidence_packet) set `target_os_class =
  other_unspecified`, `target_cpu_class = cpu_agnostic`,
  `target_tier_class = tier_not_applicable`, and
  `libc_or_runtime_ref = null`.
- **Profile.** `profile` splits the cargo profile into five
  independently-addressable axes: `cargo_profile_class`,
  `opt_level_ref`, `debug_info_class`, `panic_class`,
  `strip_class`.
- **Build epoch.** `build_epoch.source_date_epoch` is the
  deterministic epoch (integer seconds) the build pipeline
  normalized against, `build_timestamp_utc` is its ISO-8601 UTC
  rendering, and `derivation_class` names how the epoch was
  picked: `committer_timestamp_of_head` is the default;
  `release_pipeline_pinned_epoch` is the release lane's lock;
  `manually_pinned_epoch` is admissible only on `dev_local`
  and reproducibility-pack replays; `archive_header_pinned_epoch`
  pairs with `tree_state_class = archive_export_no_git`.
- **Producer lane.** `producer_lane.lane_class` names one of
  `developer_local`, `ci_preview`, `ci_merge_queue`, `ci_nightly`,
  `ci_beta`, `ci_release`, `ci_hotfix`, `reproduced_clean_room`,
  `offline_mirror_rebuild`. `producer_identifier_ref` is an opaque
  id for the producer (workstation pseudonym, CI job identity,
  verifier identity). `rebuild_of_identity_ref` is non-null on
  `reproduced_clean_room` / `offline_mirror_rebuild` and pins the
  prior identity this rebuild is of.
- **Provenance.** `provenance.signing_class` and
  `signing_material_state` are the release-evidence gate:
  `signed_verified` is the only state that clears the gate.
  `attestation_predicate_refs`, `sbom_document_ref`,
  `provenance_statement_ref`, and `transparency_log_ref` are
  opaque refs. Final signing / notarization workflows are out of
  scope at this freeze; this axis pins only the vocabulary every
  downstream lane references.
- **Propagation.** `propagation.binary_embed_class` names how the
  identity is carried with the artifact:
  `embedded_in_binary` is the default for binaries and SDK
  libraries; `sidecar_manifest_beside_binary` pairs with the
  split-symbols / source-map families; `external_registry_only`
  is the docs-pack / schema-export / reference-pack path;
  `embedded_in_archive_manifest` is the source-bundle /
  reproducibility-pack / release-evidence-packet path.
  `symbol_tag_refs` is the list of build-id / GNU build-id /
  signed symbol-tag rows that carry the identity as a signed
  label on the emitted artifact.
- **Evidence.** `evidence.build_log_ref`,
  `reproducibility_pack_ref`, and `artifact_graph_node_ref` are
  the three required links on every non-dev lane;
  `support_bundle_anchor_ref`, `help_about_payload_ref`,
  `public_release_note_ref`, `split_symbols_ref`, and
  `source_map_manifest_ref` are optional depending on the
  artifact family.
- **Policy / redaction.** `policy_context` (policy_epoch,
  trust_state, execution_context_id) and `redaction_class`
  (`log_safe`, `support_export_only`, `evidence_packet_only`,
  `release_public`) are re-exported from ADR-0007 / ADR-0008 /
  ADR-0009 without modification.

## Propagation into downstream surfaces

Each surface reads the identity record under its own redaction
envelope and renders only the axes it owns. The rule is that
every surface's emitted row carries the opaque
`running_build_identity_ref` (which resolves to
`exact_build_identity_ref` on the record), and any axis the
surface renders is sourced from this record field-for-field.

| Surface                                        | Projects from the identity                                                                                                                                                      | Rules                                                                                                                                                                                                                 |
|------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Running binary (ide / cli)                     | `propagation.binary_embed_class = embedded_in_binary` plus `symbol_tag_refs`. Resolver path returns `exact_build_identity_ref` to crash symbolication and Help / About.         | A binary with no embedded identity is non-conforming. Crash symbolication denies symbol lookup with a typed "identity missing" cause rather than guessing.                                                           |
| Debug symbols / source maps                    | `artifact_family_class = *_debug_symbols` / `source_map_bundle`, `binary_embed_class = sidecar_manifest_beside_binary`, `evidence.split_symbols_ref` on the paired binary.      | The paired binary's `evidence.split_symbols_ref` MUST resolve to the `exact_build_identity_ref` of this record. Orphan symbol archives are quarantined.                                                                |
| Artifact graph                                 | `evidence.artifact_graph_node_ref` on every non-dev identity.                                                                                                                   | The technical-architecture rule that the artifact graph point at a single build identity is enforced by the non-null requirement on non-dev lanes. Node-family completeness and claim ownership are governed by `docs/release/release_artifact_graph.md` and `artifacts/release/artifact_graph_rules.yaml`. |
| SBOM document                                  | `artifact_family_class = sbom_document` (when this record is itself the SBOM) or `provenance.sbom_document_ref` (when this record is a binary / symbols / etc.).                | A binary whose `provenance.sbom_document_ref` is null on a release lane is a publishable blocker.                                                                                                                     |
| Signed attestation                             | `artifact_family_class = signed_attestation` or `provenance.attestation_predicate_refs`.                                                                                        | Predicate refs are typed opaque ids; the predicate schema itself is governed upstream (SLSA, in-toto).                                                                                                                |
| Help / About                                   | `evidence.help_about_payload_ref` on the ide_binary / cli_binary identity. The About dialog's copy-build-info payload renders the identity verbatim.                            | ADR-0013 requires Help / About to deny render and route to a repair hook when the publisher cannot resolve the running build; this record is the resolution target.                                                   |
| Release center / public release note           | `evidence.public_release_note_ref` on every release-lane identity.                                                                                                              | Pre-release lanes (`preview` / `nightly`) leave this null; the release center renders the pre-release channel without a release-note linkage.                                                                         |
| Support bundle                                 | `evidence.support_bundle_anchor_ref` on every identity that ships inside a support export. `redaction_class` governs visibility.                                                | Support bundles enumerate (identity_ref, artifact_family_class) pairs under the `support_export_only` frame.                                                                                                          |
| Public evidence packet                         | `artifact_family_class = release_evidence_packet` aggregates the per-family identities; each referenced identity carries `redaction_class` that is `release_public` or below.   | Evidence packets quote identities verbatim; raw signing material and raw build logs never appear.                                                                                                                     |
| Docs pane / docs browser / Help (version-match)| Docs `running_build_identity_ref` on every `help_status_badge_record` resolves to this record. The docs-pack manifest's `semver_version` / `compat_window_semver` diffs against `semver_core` + `release_channel_class`. | ADR-0013's five-state version-match axis is computed from this record alone; a tooltip derived from `workspace_version` is non-conforming.                                                                            |
| Search / AI explanation (running build)        | Search-result-truth `running_build_identity_ref` on every result. Citations / deep-links inherit the same identity.                                                             | ADR-0014 requires every emitted result-truth record to carry `running_build_identity_ref`; this record is the resolution target.                                                                                      |

## Diffing two nearby builds

The schema's closed-axis set makes "are these two builds the same?"
mechanical. A diffing tool reads two `exact_build_identity_record`
files and enumerates the axes that differ, in this order:

1. `commit.full_hash` — different commits are different builds.
2. `tree_state_class` — a dirty / shallow / patched variant of the
   same commit is not the same build.
3. `toolchain.toolchain_pin_digest` and `toolchain.lockfile_digest`
   — a bumped pin or a lockfile drift is a different build.
4. `target.target_triple` — cross-target builds are siblings, not
   peers.
5. `artifact_family_class` — two artifact families at the same
   commit / toolchain / target are siblings and diff only on
   family / profile / propagation / evidence.
6. `profile` axes — `cargo_profile_class`, `opt_level_ref`,
   `debug_info_class`, `panic_class`, `strip_class`.
7. `build_epoch.source_date_epoch` — if all the above match, a
   differing epoch is a timestamp-pinning drift (the release lane
   rejects this; the dev lane tolerates).
8. `producer_lane.lane_class` — same content, different lane is a
   reproduction / rebuild relationship; `rebuild_of_identity_ref`
   is the typed link.
9. `provenance.signing_material_state` — same content, different
   signing state is an upgrade from `signed_unverified` to
   `signed_verified`.

Two identities are "the same build" iff axes 1–7 match. Axes 8–9
distinguish the rebuild / verification relationship without
rejecting the identity.

The
[`clean_room_reproduction_of_stable_release.json`](../../fixtures/build/exact_build_examples/clean_room_reproduction_of_stable_release.json)
fixture is the diff-target for the
[`ci_release_stable_linux_ide_binary.json`](../../fixtures/build/exact_build_examples/ci_release_stable_linux_ide_binary.json)
fixture: axes 1–7 match; `producer_lane.lane_class` and
`producer_identifier_ref` differ; `rebuild_of_identity_ref` on the
reproduction pins back to the original's
`exact_build_identity_ref`.

## Human-readable rendering

Help / About, the release center, and support summaries render the
record through a single, stable projection. The canonical form is:

```
Aureline 0.7.3 (stable)
  commit:     a4d1c3f0e27b6f91d8e2c1a4b7f3e0c5d9a8b2e1  (a4d1c3f0e27b)
  tree:       clean checkout
  target:     x86_64-unknown-linux-gnu  (linux-glibc / x86_64 / tier-1)
  toolchain:  rustc 1.84.0 (9fc6b4312 2025-01-07)
  profile:    release / opt-3 / split-debug / panic=abort / strip=symbols
  built:      2026-04-13 14:00:00 UTC  (source_date_epoch 1744560000)
  lane:       ci_release
  signing:    release_hardware_key_sign / signed_verified
  sbom:       sbom:spdx:aureline:stable:0.7.3:linux:a4d1c3f0e27b
  identity:   build-id:aureline:stable:0.7.3:x86_64-unknown-linux-gnu:release:a4d1c3f0e27b
```

Rendering rules:

- Every axis in the block above MUST be projected when the surface
  has room to render it. Surfaces with less room (the About
  footer chip, the status-bar version pill) render
  `<workspace_version> (<release_channel_class>) · <commit.short_hash>`
  but MUST still carry `exact_build_identity_ref` as a non-
  rendered copy field so a copy-build-info action produces the
  full block.
- Pre-release channels render the channel token uppercased
  (`NIGHTLY`, `PREVIEW`, `BETA`) and the
  `signing_material_state` on the same line (`signed_unverified`
  is rendered as a non-blocking disclosure chip on About).
- Reproduction identities render
  `reproduction of <rebuild_of_identity_ref>` on its own line
  above the `identity:` line.
- Copy-build-info emits the block verbatim plus the raw
  `exact_build_identity_ref` at the top of the clipboard payload.

## Upgrade discipline

Adding a new release channel, artifact family, tree-state class,
cargo-profile class, debug-info class, panic class, strip class,
target OS / CPU / tier class, build-epoch-derivation class,
producer-lane class, signing class, signing-material state,
binary-embed class, or hash algorithm is additive-minor: the schema
appends the token to the closed `enum`, the version number on
`exact_build_identity_schema_version` stays at its current value
(only breaking shape changes bump it), and this document is
updated in the same change.

Repurposing an existing token (e.g. changing what `stable` means,
or moving `ci_nightly` to a different lane bucket) is breaking and
requires a new decision row under the exact-build-identity
decision (`D-0011` in the governance register).

## Relation to the reproducible-build baseline

The baseline's `build_identity.json` is the builder-local identity:
one record per invocation of `./tools/build/build.sh`. The
exact-build identity is the cross-artifact super-set: one record
per `(exact_build_identity_ref, artifact_family_class)` pair on
release lanes. A release of the workspace cuts several identities
off the same builder run (one per family), and any of them can be
reconstructed from the builder's `build_identity.json` plus the
family's profile / propagation / evidence axes.

Both records stay in sync: a change to the baseline's axes (adding
a new pinned file, changing the committer-epoch semantics) MUST
update this document and the schema in the same change, and the
baseline's `build_identity.json` MUST remain derivable from the
exact-build identity record for the same `(commit, toolchain,
target, profile)` triple.
