# Extension SDK, docs, samples, and conformance-kit publication contract

This document is the narrative companion to the SDK release-bundle
boundary schema at
[`/schemas/extensions/sdk_release_bundle.schema.json`](../../schemas/extensions/sdk_release_bundle.schema.json)
and the conformance-result schema at
[`/schemas/extensions/conformance_result.schema.json`](../../schemas/extensions/conformance_result.schema.json).
Worked examples live under
[`/fixtures/extensions/sdk_publication_cases/`](../../fixtures/extensions/sdk_publication_cases/).
It freezes the publication contract for every SDK-line artifact the
extension ecosystem releases — WIT / host schema packages, generated
bindings and helper libraries, API / schema docs packs, template /
sample / tutorial bundles, and conformance / compatibility tooling —
so each artifact ships as a versioned row with explicit version
linkage, build identity, mirror / offline availability, support
window, and (where applicable) sample-validation evidence rather
than as ad hoc collateral whose compatibility line has to be
inferred from branch names or blog posts. The schemas are
authoritative when the narrative and the schemas disagree; this
document MUST be updated in the same change that lands any schema
bump.

The seed is deliberately narrow. It does **not** ship a full SDK,
build a marketplace, run a hosted bindings generator, mint a
compatibility bridge, or land an external mirror service. Its job
is to freeze the publication vocabulary early enough that the
later SDK lane, docs lane, and conformance lane build against one
contract instead of inventing publication-shaped fields ad hoc.

## What this seed freezes

1. A **closed five-class artifact-set vocabulary** —
   `wit_package_release`, `generated_binding`, `docs_pack_export`,
   `tutorial_sample_bundle`, `conformance_kit_release` — and the
   record-kind discriminator that pairs each row to its concrete
   payload shape.
2. An **SDK-line identity envelope** binding every row to one
   `sdk_line_id` plus an `sdk_line_semver`, the WIT capability-world
   set the row covers (ADR-0019 `aureline:<world>@<semver>`), the
   permission-vocabulary version (ADR-0012), and the host ABI
   window class. Two rows that quote the same `sdk_line_id` and
   `sdk_line_semver` MUST quote the same WIT world set, the same
   permission-vocabulary version, and the same host ABI window
   class verbatim; widening the line silently is denied.
3. A **content-addressed build-identity envelope** binding every
   row to a `(digest_algorithm, digest_hex)` pair, a build-
   provenance ref, and a build-invocation id. Mirror, offline-
   bundle, and conformance-result rows quote the build identity
   verbatim; a row that re-digests during transport is denied with
   `sdk_release_artifact_identity_mutated_on_repackage`.
4. A **closed mirror-availability vocabulary** —
   `public_registry_only`, `public_and_approved_mirror`,
   `offline_bundle_eligible`, `air_gapped_mirror_only` — and the
   rule that a row whose `mirror_availability_class` is
   `offline_bundle_eligible` or `air_gapped_mirror_only` MUST
   carry an `offline_mirror_bundle_ref` to a sealed bundle row in
   `/schemas/extensions/registry_manifest.schema.json` so air-
   gapped operators can install the artifact set from one
   reproducible export.
5. A **closed support-window vocabulary** —
   `preview_only_no_support`, `experimental_short_window`,
   `general_short_window`, `general_long_window`,
   `extended_long_window`, `retired` — and the rule that a row in
   `general_*` or `extended_*` MUST cite at least one
   `conformance_result_ref` whose `result_class` is
   `pass_full_matrix` or `pass_subset_documented` and whose
   `compatibility_badge_class_emitted` matches the row's
   `compatibility_badge_class`. Quantitative window lengths land
   in the successor ADR.
6. A **closed compatibility-badge vocabulary** —
   `compatible_on_declared_targets`,
   `compatible_on_subset_of_declared_targets`,
   `compatibility_bridge_required`,
   `partial_compatibility_documented`,
   `unsupported_pending_qualification`,
   `incompatible_blocked_on_policy` — that publication rows, docs
   surfaces, and install-review chips MUST read verbatim. A
   surface that mints a parallel "verified", "stable", or "GA"
   chip outside this set is non-conforming.
7. A **sample-validation rule** — every
   `tutorial_sample_bundle` row MUST set `sample_validation_state`
   to `must_compile_in_ci` or `must_validate_in_ci`, MUST cite at
   least one `conformance_result_ref` whose `result_class` is
   `pass_full_matrix` or `pass_subset_documented`, and MUST quote
   the SDK line and WIT world set the sample claims support for.
   "Official sample" rows whose `sample_validation_state` is
   `not_required_for_class` are denied.
8. A **conformance-result envelope** binding every passing or
   failing run to its subject artifact, its conformance-kit
   release, the host class it ran against, the declared test
   matrix it covered, the badge it authorizes, and the typed
   failure-reason class when it failed. Static docs exports and
   air-gapped mirror bundles cite `conformance_result_ref` rows
   verbatim rather than minting result chips of their own.
9. A **typed denial-reason vocabulary** binding every publication,
   sample-validation, conformance-result, and mirror failure to a
   repair affordance. Silent fallback to a generic "release
   blocked" or "unsupported" chip is forbidden.

## Record kinds

| Record kind                       | Schema                                                                                                         | Purpose                                                                                                          |
|-----------------------------------|----------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| `wit_package_release_row`         | [`sdk_release_bundle.schema.json`](../../schemas/extensions/sdk_release_bundle.schema.json) `#/$defs/wit_package_release_row` | Per-release WIT / host-schema package binding the SDK line to one capability-world set and ABI window. |
| `generated_binding_row`           | [`sdk_release_bundle.schema.json`](../../schemas/extensions/sdk_release_bundle.schema.json) `#/$defs/generated_binding_row` | Per-release generated bindings or helper library binding one target language to one source WIT release. |
| `docs_pack_export_row`            | [`sdk_release_bundle.schema.json`](../../schemas/extensions/sdk_release_bundle.schema.json) `#/$defs/docs_pack_export_row` | Per-release static docs pack export (API reference, static site, man pages, PDF) tied to a source WIT and binding set. |
| `tutorial_sample_bundle_row`      | [`sdk_release_bundle.schema.json`](../../schemas/extensions/sdk_release_bundle.schema.json) `#/$defs/tutorial_sample_bundle_row` | Per-release template / sample / tutorial / starter pack tied to one SDK line and validated by CI. |
| `conformance_kit_release_row`     | [`sdk_release_bundle.schema.json`](../../schemas/extensions/sdk_release_bundle.schema.json) `#/$defs/conformance_kit_release_row` | Per-release conformance / compatibility tooling release covering host, guest, sample-validator, and compatibility-analyzer kinds. |
| `conformance_result_record`       | [`conformance_result.schema.json`](../../schemas/extensions/conformance_result.schema.json) `#/$defs/conformance_result_record` | One run of a conformance kit against a subject release on a target host. The badge a publication row claims MUST be backed by at least one passing result here. |

## SDK-line identity envelope

Every release row shares one identity envelope:

```
{
  "sdk_line_id": "aureline.sdk.<line-slug>",
  "sdk_line_semver": "<MAJOR.MINOR.PATCH>",
  "wit_world_refs": ["aureline:<world>@<MAJOR.MINOR.PATCH>", ...],
  "permission_vocabulary_version_ref": "<opaque permission-vocabulary version row ref>",
  "host_abi_window_class": "<one of the closed ABI window classes>"
}
```

- `sdk_line_id` is kebab-case and stable across the line. Reusing
  a retired `sdk_line_id` is denied with
  `sdk_line_id_reused_after_retirement`.
- `sdk_line_semver` follows standard semver: additive worlds /
  bindings / docs / samples / kits bump `MINOR`; repurposing a
  named item bumps `MAJOR` and requires a new decision row.
- `wit_world_refs` lists the ADR-0019 world identities the row
  claims to cover. A WIT package release row's set is the
  authoritative set for the `(sdk_line_id, sdk_line_semver)`
  pair; every other row at the same line / semver MUST quote a
  subset.
- `permission_vocabulary_version_ref` resolves to the ADR-0012
  permission-vocabulary version row the line is pinned against.
- `host_abi_window_class` is one of
  `component_model_abi_window_alpha_0`,
  `component_model_abi_window_beta_1`,
  `component_model_abi_window_general_1`,
  `core_module_abi_window_alpha_0`,
  `core_module_abi_window_beta_1`,
  `compatibility_bridge_window_documented`,
  `external_host_process_window_documented`. Adding a class is
  additive-minor; repurposing a class is breaking.

## Build identity

Every row binds to a content-addressed build identity:

```
{
  "build_identity": {
    "content_address": {
      "digest_algorithm": "sha256",
      "digest_hex": "<lowercase hex>",
      "digest_size_bytes": <int>
    },
    "build_provenance_ref": "<opaque ref to the build attestation row>",
    "build_invocation_id": "<opaque stable build run id>",
    "build_input_lock_ref": "<opaque ref to the input lockfile snapshot>"
  }
}
```

Two rows that share the same content-address pair MUST quote the
same build provenance and the same input lock. A mirror, offline
bundle, or static docs export that re-digests is denied with
`sdk_release_artifact_identity_mutated_on_repackage`.

## Mirror availability and offline bundles

`mirror_availability_class` is one of:

- `public_registry_only` — release is fetched from the public
  registry; offline restore requires an explicit operator export.
- `public_and_approved_mirror` — release is mirrored verbatim by
  approved mirrors per the registry-manifest seed.
- `offline_mirror_bundle_eligible` — release is packaged into a
  sealed offline bundle reusable by air-gapped operators.
- `air_gapped_mirror_only` — release ships exclusively as an
  air-gapped mirror bundle; no public-registry path exists.

Rule: rows whose `mirror_availability_class` is
`offline_mirror_bundle_eligible` or `air_gapped_mirror_only` MUST
cite a non-null `offline_mirror_bundle_ref` resolving to a sealed
`offline_bundle_manifest_row`
(`/schemas/extensions/registry_manifest.schema.json`). The bundle
preserves the build-identity content-address verbatim; widening
the trust badge during bundle export is denied with the
registry-seed `local_archive_trust_tier_capped` /
`mirror_narrowing_attempted_widening` reasons as appropriate.

## Support window

`support_window_class` is one of:

- `preview_only_no_support` — no compatibility commitment.
- `experimental_short_window` — best-effort, breakable.
- `general_short_window` — supported with a short window.
- `general_long_window` — supported with the standard long window.
- `extended_long_window` — supported under an extended-window
  contract for managed customers.
- `retired` — frozen at last known compatibility; no future
  support, mirrored for evidence only.

Rule: a row whose `support_window_class` is
`general_short_window`, `general_long_window`, or
`extended_long_window` MUST cite at least one
`conformance_result_ref` whose `result_class` is
`pass_full_matrix` or `pass_subset_documented` and whose
`compatibility_badge_class_emitted` equals the row's
`compatibility_badge_class`. A general-supported row that has no
passing conformance result is denied with
`support_window_unbacked_by_conformance_result`.

Rule: a row whose `support_window_class` is `retired` MUST set
`compatibility_badge_class` to `unsupported_pending_qualification`
or `incompatible_blocked_on_policy`. A retired row that still
claims a `compatible_on_*` badge is denied with
`retired_release_must_not_claim_compatible_badge`.

## Compatibility badges

Closed badge vocabulary every publication, docs, and install-
review surface MUST read verbatim:

- `compatible_on_declared_targets`
- `compatible_on_subset_of_declared_targets`
- `compatibility_bridge_required`
- `partial_compatibility_documented`
- `unsupported_pending_qualification`
- `incompatible_blocked_on_policy`

A surface that renders any chip outside this set is denied with
`compatibility_badge_class_unknown`. A surface that hides a
non-`compatible_on_declared_targets` badge is denied with
`review_disclosure_incomplete`.

## Per-class fields

Every row carries the shared identity / build / mirror /
support / badge envelope above plus the per-class fields named
here.

### `wit_package_release_row`

- `wit_package_id` — opaque ref to the WIT package id (e.g.
  `aureline:worlds@<semver>`).
- `wit_world_set` — list of `aureline:<world>@<semver>` quoted
  verbatim from `/wit/aureline/`.
- `abi_compatibility_window_class` — narrower form of the
  envelope's `host_abi_window_class` for the package's own ABI.
- `successor_release_ref` — opaque ref to the next-line release
  when this row is `retired`. Null otherwise.

### `generated_binding_row`

- `target_language_class` — one of
  `rust`, `typescript`, `javascript_browser`, `python`, `go`,
  `dotnet`, `wasm_core_module_adapter`. Adding a target is
  additive-minor; repurposing is breaking.
- `binding_kind_class` — one of
  `typed_bindings_full`, `helper_library`, `adapter_shim`.
- `bindgen_tool_id_ref` — opaque ref to the bindgen tool / version
  row used to generate the binding.
- `source_wit_release_ref` — ref to the
  `wit_package_release_row` this binding was generated from.
  Two binding rows that share `(sdk_line_id, sdk_line_semver,
  target_language_class, binding_kind_class)` MUST cite the same
  `source_wit_release_ref`.

### `docs_pack_export_row`

- `docs_pack_kind_class` — one of
  `api_reference_html`, `static_site_export`, `man_page_set`,
  `pdf_export`.
- `docs_format_class` — one of
  `html_static_site`, `markdown_export`, `manpage_groff`,
  `pdf_a_2_archival`, `epub_3`.
- `source_wit_release_refs` — non-empty list of
  `wit_package_release_row` refs covered by the docs pack.
- `source_binding_release_refs` — list of
  `generated_binding_row` refs covered (may be empty for a docs
  pack that documents only WIT worlds).
- `static_docs_export_ref` — opaque ref to the sealed export
  archive. Required when `mirror_availability_class` is
  `offline_mirror_bundle_eligible` or `air_gapped_mirror_only`.

### `tutorial_sample_bundle_row`

- `bundle_kind_class` — one of
  `template_pack`, `sample_pack`, `tutorial_pack`,
  `starter_pack`.
- `sample_validation_state` — one of
  `must_compile_in_ci`, `must_validate_in_ci`,
  `not_required_for_class`. Official rows MUST set this to
  `must_compile_in_ci` or `must_validate_in_ci`. The
  `not_required_for_class` token is reserved for unofficial
  community archives that the SDK lane does not publish under
  this contract; a row with `not_required_for_class` is denied
  by the schema.
- `declared_compatibility_targets` — non-empty list of
  `host_contract_family_ref` (ADR-0012) plus
  `wit_world_ref` pairs the bundle claims support for. MUST be a
  subset of the SDK-line envelope's `wit_world_refs`.
- `sample_validation_evidence_refs` — non-empty list of
  `conformance_result_record` refs with `result_class` of
  `pass_full_matrix` or `pass_subset_documented`. A bundle row
  with no passing evidence ref is denied with
  `tutorial_sample_unvalidated_in_ci`.

### `conformance_kit_release_row`

- `tooling_class` — one of
  `host_conformance_tester`, `guest_conformance_tester`,
  `sample_validator`, `compatibility_analyzer`.
- `declared_test_matrix_refs` — non-empty list of opaque test-
  matrix row refs the kit advertises coverage for. A kit release
  with no declared matrix is denied with
  `conformance_kit_no_declared_matrix`.
- `mirrored_static_docs_export_ref` — optional ref to a docs-pack
  export that re-publishes the kit's matrix and result vocabulary.

## Conformance result envelope

Every conformance / compatibility run records one
`conformance_result_record` (binding schema:
[`/schemas/extensions/conformance_result.schema.json`](../../schemas/extensions/conformance_result.schema.json)).

Required fields:

- `record_kind = conformance_result_record`.
- `conformance_result_id` — opaque stable id.
- `subject_release_ref` — ref to the `sdk_release_bundle` row
  under test.
- `subject_release_artifact_class` — one of the five artifact
  classes (frozen vocabulary; same set as the bundle schema).
- `subject_wit_world_refs` — the worlds covered by this run.
  MUST be a subset of the subject release's `wit_world_refs`.
- `subject_sdk_line_id` and `subject_sdk_line_semver` — quoted
  verbatim from the subject release.
- `target_host_class` — one of
  `production_host`, `reference_host_in_repo`,
  `qualification_lab_host`, `mirror_test_host`,
  `compatibility_bridge_host`. Used to disambiguate where the
  result was produced; a `production_host` result MUST be
  reproducible against a `reference_host_in_repo` row before it
  can authorize a `general_*` support window.
- `target_host_id_ref` — opaque ref.
- `conformance_kit_release_ref` — ref to the
  `conformance_kit_release_row` whose tooling produced the
  result.
- `executed_at` — monotonic timestamp.
- `executed_by_ref` — opaque ref to the automation actor.
- `result_class` — one of
  `pass_full_matrix`, `pass_subset_documented`, `fail_blocking`,
  `fail_non_blocking`, `blocked_pre_run_environment`,
  `withdrawn_invalidated`.
- `coverage_class` — one of
  `full_declared_matrix`, `partial_subset_documented`,
  `probe_only_smoke`, `matrix_unknown_pending_replay`.
- `compatibility_badge_class_emitted` — the badge the result
  authorizes (frozen vocabulary; same set as the bundle).
- `failure_reason_class` — required when `result_class` is
  `fail_blocking`, `fail_non_blocking`, or
  `blocked_pre_run_environment`. One of
  `host_abi_drift_detected`,
  `permission_vocabulary_version_drift_detected`,
  `wit_world_unknown_to_host`,
  `wit_world_retired`,
  `compatibility_bridge_unavailable`,
  `sample_failed_to_compile`,
  `sample_failed_to_validate`,
  `docs_pack_export_drift_detected`,
  `air_gapped_mirror_unreachable`,
  `target_host_unreachable`,
  `target_host_environment_invalid`,
  `conformance_kit_release_invalidated`,
  `result_invalidated_by_publisher`,
  `failure_reason_unknown_pending_review`.
- `evidence_artifact_refs` — opaque refs to evidence captures.
  Raw stack frames, raw secrets, raw test outputs MUST NOT
  appear; refs only.
- `static_docs_export_refs` — optional refs to docs packs the
  result is republished into.
- `air_gapped_mirror_bundle_refs` — optional refs to offline
  bundle rows the result is mirrored into.
- `audit_event_refs` — optional refs to audit events on the
  `extension_sdk_publication` stream.
- `redaction_class` — declared redaction class for the row.

Rule: a `pass_full_matrix` result MUST set
`coverage_class = full_declared_matrix` and MUST NOT cite a
failure reason. A `pass_subset_documented` result MUST set
`coverage_class = partial_subset_documented`. A `fail_*` or
`blocked_pre_run_environment` result MUST cite a non-null
`failure_reason_class`. A `withdrawn_invalidated` result MUST
cite `failure_reason_class = result_invalidated_by_publisher` or
`conformance_kit_release_invalidated`.

Rule: the `compatibility_badge_class_emitted` MUST be the
narrowest badge that holds for the run. A pass on a partial
matrix that emits `compatible_on_declared_targets` (rather than
`compatible_on_subset_of_declared_targets`) is denied with
`conformance_result_badge_widens_coverage`.

## Static docs exports and air-gapped mirror bundles

Static docs exports MUST cite `conformance_result_ref` rows
verbatim rather than re-rendering result chips locally; an
exported docs pack that pretends a release is supported when no
passing conformance result exists is denied with
`docs_pack_export_drift_detected`.

Air-gapped mirror bundles MUST package every release row that
backs the bundle's claimed compatibility envelope plus the
matching `conformance_result_record` and
`conformance_kit_release_row` rows. A bundle that ships a
release row without the conformance evidence that backs its
support-window claim is denied with
`air_gapped_mirror_missing_conformance_evidence`.

## Version linkage

Two rows that share the same `(sdk_line_id, sdk_line_semver)`
MUST share the same:

- `wit_world_refs` set (or, for binding / docs / sample / kit
  rows, a documented subset of the WIT release row's set).
- `permission_vocabulary_version_ref`.
- `host_abi_window_class`.

A row that diverges silently from another row's envelope is
denied with `sdk_line_envelope_divergence`. The schema enforces
the per-row shape; the surface enforces the cross-row check.

## Audit events reserved

Emitted on the `extension_sdk_publication` audit stream. Raw
build artifacts, raw signing-key material, raw conformance run
outputs, raw docs export bytes, and raw sample / template bytes
MUST NOT appear on any event.

- `sdk_release_published`
- `sdk_release_re_signed_denied`
- `sdk_release_offline_bundle_exported`
- `sdk_release_air_gapped_mirror_published`
- `sdk_release_retired`
- `sdk_release_envelope_divergence_detected`
- `wit_package_release_indexed`
- `generated_binding_indexed`
- `docs_pack_export_indexed`
- `tutorial_sample_bundle_indexed`
- `tutorial_sample_validation_recorded`
- `conformance_kit_release_indexed`
- `conformance_result_recorded`
- `conformance_result_invalidated`
- `compatibility_badge_emitted`
- `support_window_promoted`
- `support_window_demoted`

## Denial reasons reserved

In addition to the ADR-0012 and registry-seed denial sets:

- `sdk_release_artifact_identity_mutated_on_repackage`
- `sdk_line_id_reused_after_retirement`
- `sdk_line_envelope_divergence`
- `support_window_unbacked_by_conformance_result`
- `retired_release_must_not_claim_compatible_badge`
- `tutorial_sample_unvalidated_in_ci`
- `tutorial_sample_targets_outside_sdk_line_world_set`
- `conformance_kit_no_declared_matrix`
- `conformance_result_badge_widens_coverage`
- `conformance_result_failure_reason_required`
- `conformance_result_subject_world_outside_release_set`
- `docs_pack_export_drift_detected`
- `air_gapped_mirror_missing_conformance_evidence`
- `compatibility_badge_class_unknown`
- `review_disclosure_incomplete`

Every denial emits the corresponding audit event with a typed
reason and a repair-affordance label.

## Consumer expectations

The downstream surfaces below MUST read this seed rather than
invent SDK-publication-shaped fields:

- **SDK publication tooling.** Emits one row per artifact at
  release time, citing the SDK line envelope, the build identity,
  the mirror class, the support window, the badge, and (when the
  row claims support) the backing conformance results.
- **Static docs site and downloadable docs packs.** Read
  `docs_pack_export_row` and the conformance results it cites
  verbatim; never invent a "verified" or "stable" chip.
- **Tutorial / sample / template publication.** Read
  `tutorial_sample_bundle_row` rows verbatim. Pull-request CI
  blocks merge of an "official" sample row whose
  `sample_validation_evidence_refs` is empty or contains no
  passing run.
- **Conformance and compatibility tooling releases.** Emit
  `conformance_kit_release_row` rows so downstream clients can
  pin the matrix / vocabulary version they validated against.
- **Air-gapped mirror exporter.** Bundles WIT, bindings, docs
  pack, samples, and conformance kit / results together; rejects
  exports that drop conformance evidence the bundle's release
  rows depend on.
- **Install / update review and permission inspector.** Project
  `compatibility_badge_class`, `support_window_class`, and
  `mirror_availability_class` from the release rows the install
  resolves through. A surface that hides any of these is denied
  with `review_disclosure_incomplete`.
- **Registry / mirror integration.** Reuses
  `content_address`, `signature_ref` /
  `attestation_bundle_ref`, and `publisher_continuity_ref` from
  the registry-manifest seed for SDK rows; SDK release rows are
  not a separate trust vocabulary.

## Out of scope

- Shipping a full extension SDK, a hosted bindings generator, a
  marketplace, or a compatibility-bridge runtime in M0.
- Quantitative support-window lengths (days / months) per
  `support_window_class`. Lengths land in the successor ADR.
- The on-disk bundle envelope format, signing envelope, and
  compression for offline / air-gapped bundles. The seed pins
  the row shape; the envelope format is a successor concern.
- Public documentation of every WIT world, every binding target,
  or every sample. The seed pins the publication contract; the
  per-world / per-target / per-sample matrix grows under it.
