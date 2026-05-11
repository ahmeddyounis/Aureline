# Proof packet: M1 critical-dependency / import register and release-notice draft seed

Purpose: anchor proof captures for the unattended M1 lane that
validates the canonical critical-dependency / import register seed
and the first executable consumer (the draft release-notice / report
pipeline). The lane proves the seed is consumable by the existing
release-notice seed, the docs/help truth surfaces, the support-bundle
exporter, the release-evidence pack, and the governance / CI
validation lane — without re-encoding the template vocabulary or the
publication targets on each surface.

Reviewer entry point:
[`/docs/governance/m1_dependency_and_notice_seed.md`](../../../docs/governance/m1_dependency_and_notice_seed.md).

## Canonical sources

- `artifacts/governance/critical_dependency_register.yaml` — seed rows
  the runner consumes. Carries:
  - the M1 envelope (`schema_version`, `matrix_id`, `owner_dri`,
    `overview_page`, `row_schema_ref`, `build_identity_ref`,
    `validation_lane_ref`, `companion_registers`,
    `draft_output_refs`),
  - closed envelope vocabularies (`source_register_class_vocabulary`,
    `template_class_vocabulary`, `criticality_class_vocabulary`,
    `protected_path_class_vocabulary`, `license_class_vocabulary`,
    `provenance_status_class_vocabulary`,
    `admission_state_class_vocabulary`,
    `publication_target_class_vocabulary`,
    `release_notice_action_class_vocabulary`,
    `failure_drill_id_vocabulary`),
  - required coverage lists (`required_template_class_coverage`,
    `required_publication_target_coverage`,
    `required_release_notice_action_coverage`),
  - the named runtime consumers the seed asserts are live (landing
    page, draft pipeline, three upstream companion registers, CI
    validator), and
  - one register row per critical upstream choice or imported byte
    set, each with a uniform
    `(register_entry_id, source_register, source_id, name,
    template_class, criticality_class, protected_path_class,
    license_class, provenance_status_class, admission_state_class,
    publication_targets, release_notice_action_class, owner_dri,
    fork_or_replace_trigger, evidence_refs, failure_drill)` shape.

- `schemas/governance/critical_dependency_register.schema.json` —
  envelope schema; freezes vocabularies, required coverage lists,
  named-consumer shape, companion-register paths, draft-output paths,
  and matrix identity.

- `schemas/governance/critical_dependency_register_entry.schema.json`
  — row schema; freezes the closed vocabularies and conditional
  invariants (source_register / source_id agreement;
  runtime_dependency forces notice + SBOM + provenance;
  bundled_asset forces provenance; build_tooling forbids
  third-party notice; host_runtime restricted to provenance-only;
  mirrored_pack forces docs-pack manifest attribution;
  hold_pending_first_admission gated on admission state).

- `tools/governance/build_dependency_notice_seed.py` — first
  executable consumer. Emits a deterministic markdown draft at
  `artifacts/governance/build/dependency_notice_draft.md` and a JSON
  sidecar at `artifacts/governance/build/dependency_notice_draft.json`
  so the data model is executable rather than aspirational.

- `tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py`
  — unattended runner that replays the seed and emits the durable
  JSON capture.

## Named runtime consumers

- `docs/governance/m1_dependency_and_notice_seed.md` — reviewer-facing
  landing page that quotes the seeded rows verbatim so docs / help /
  release / support copy reads the same template and publication
  vocabulary as the seed.
- `tools/governance/build_dependency_notice_seed.py` — draft pipeline
  the seed is executable against; emits the markdown draft and JSON
  sidecar.
- `artifacts/governance/dependency_register.yaml` — upstream register
  of upstream choices the seed projects against without forking.
- `artifacts/governance/third_party_import_register.yaml` — upstream
  register of imported / mirrored bytes the seed projects against
  without forking.
- `artifacts/governance/release_notice_seed.yaml` — upstream
  publication-binding seed; the M1 critical register agrees with its
  template / publication choices.
- `tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py`
  — unattended CI / review validator.

## Live runtime consumers (read-only)

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/dependency_and_notice_seed_validation_capture.json`

## M1 register-row coverage

The seed exercises every closed `template_class`, every required
publication target, and every required release-notice action:

| `register_entry_id` | `template_class` | `release_notice_action_class` |
| --- | --- | --- |
| `cdr.runtime_dependency.renderer_wgpu` | `runtime_dependency` | `hold_pending_first_admission` |
| `cdr.runtime_dependency.shell_winit` | `runtime_dependency` | `emit_third_party_notice_and_sbom_entries` |
| `cdr.runtime_dependency.shell_softbuffer` | `runtime_dependency` | `hold_pending_first_admission` |
| `cdr.runtime_dependency.text_swash` | `runtime_dependency` | `hold_pending_first_admission` |
| `cdr.runtime_dependency.text_fontdb` | `runtime_dependency` | `hold_pending_first_admission` |
| `cdr.runtime_dependency.shaper_rustybuzz` | `runtime_dependency` | `hold_pending_first_admission` |
| `cdr.runtime_dependency.accessibility_accesskit` | `runtime_dependency` | `hold_pending_first_admission` |
| `cdr.bundled_asset.noto_font_family_source` | `bundled_asset` | `hold_pending_first_admission` |
| `cdr.bundled_asset.noto_fallback_font_subset` | `bundled_asset` | `hold_pending_first_admission` |
| `cdr.build_tooling.rust_toolchain` | `build_tooling` | `emit_build_tooling_provenance_record_only` |
| `cdr.host_runtime.git_cli` | `host_runtime` | `emit_host_runtime_environment_capture_only` |
| `cdr.host_runtime.rustup` | `host_runtime` | `emit_host_runtime_environment_capture_only` |
| `cdr.host_runtime.python3_benchmark_runtime` | `host_runtime` | `emit_host_runtime_environment_capture_only` |
| `cdr.mirrored_pack.docs_official_pack` | `mirrored_pack` | `emit_docs_pack_manifest_attribution` |

## Failure-drill coverage

Fourteen named drills, each reproducible under
`--force-drill <register_entry_id>:<drill_id>`:

| Row (`register_entry_id`) | Drill | Expected check id |
| --- | --- | --- |
| `cdr.runtime_dependency.renderer_wgpu` | `runtime_dependency_third_party_notice_dropped` | `critical_dependency_register.runtime_dependency_publication_targets_must_include_third_party_notice` |
| `cdr.runtime_dependency.shell_winit` | `runtime_dependency_release_notice_action_widened_to_build_tooling` | `critical_dependency_register.runtime_dependency_release_notice_action_must_match_template` |
| `cdr.runtime_dependency.shell_softbuffer` | `runtime_dependency_spdx_sbom_target_dropped` | `critical_dependency_register.runtime_dependency_publication_targets_must_include_spdx_sbom` |
| `cdr.runtime_dependency.text_swash` | `runtime_dependency_provenance_target_dropped` | `critical_dependency_register.runtime_dependency_publication_targets_must_include_provenance_statement` |
| `cdr.runtime_dependency.text_fontdb` | `runtime_dependency_protected_path_class_relaxed` | `critical_dependency_register.fork_or_replace_trigger_required` |
| `cdr.runtime_dependency.shaper_rustybuzz` | `protected_path_fork_or_replace_trigger_dropped` | `critical_dependency_register.fork_or_replace_trigger_required_for_protected_path` |
| `cdr.runtime_dependency.accessibility_accesskit` | `hold_pending_admission_relaxed_to_admitted_action` | `critical_dependency_register.hold_pending_first_admission_blocked_when_admitted` |
| `cdr.bundled_asset.noto_font_family_source` | `bundled_asset_family_release_notice_action_widened` | `critical_dependency_register.bundled_asset_release_notice_action_must_match_template` |
| `cdr.bundled_asset.noto_fallback_font_subset` | `bundled_asset_provenance_target_dropped` | `critical_dependency_register.bundled_asset_publication_targets_must_include_provenance_statement` |
| `cdr.build_tooling.rust_toolchain` | `build_tooling_widened_to_third_party_notice` | `critical_dependency_register.build_tooling_publication_targets_must_not_include_third_party_notice` |
| `cdr.host_runtime.git_cli` | `host_runtime_widened_to_sbom_publication` | `critical_dependency_register.host_runtime_publication_targets_must_be_provenance_only` |
| `cdr.host_runtime.rustup` | `host_runtime_widened_to_third_party_notice` | `critical_dependency_register.host_runtime_publication_targets_must_be_provenance_only` |
| `cdr.host_runtime.python3_benchmark_runtime` | `host_runtime_release_notice_action_widened_to_build_tooling` | `critical_dependency_register.host_runtime_release_notice_action_must_match_template` |
| `cdr.mirrored_pack.docs_official_pack` | `mirrored_pack_docs_pack_manifest_dropped` | `critical_dependency_register.mirrored_pack_publication_targets_must_include_docs_pack_manifest` |

A separate cross-register failure drill is built into the draft
pipeline itself: passing
`--force-drill 'omit:<register_entry_id>'` to
`tools/governance/build_dependency_notice_seed.py` simulates omitting
a seed row and proves the
`critical_dependency_register.companion_dependency_missing_seed_entry`
gate fires — the spec's "add or update a critical dependency without
register / update-notice changes -> automation flags the omission"
drill.

## Refresh

Re-run the draft pipeline AND the validation lane after a change to:

- the seed YAML,
- either schema (envelope or row),
- the reviewer-facing landing page,
- a companion register (`dependency_register.yaml`,
  `third_party_import_register.yaml`,
  `release_notice_seed.yaml`) whose row identity or critical posture
  changed,
- the draft pipeline tool, or
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the
governed proof root and every row reports PASS for closed-vocabulary
membership (template_class, criticality_class, protected_path_class,
license_class, provenance_status_class, admission_state_class,
publication_target_class, release_notice_action_class), the
conditional invariants (runtime_dependency forces notice + SBOM +
provenance; bundled_asset forces provenance; build_tooling forbids
third-party notice; host_runtime restricted to provenance-only;
mirrored_pack forces docs-pack manifest attribution; protected-path
rows must carry an owner DRI and a non-empty fork_or_replace_trigger;
hold_pending_first_admission gated on admission state), required
coverage (template classes, publication targets, release-notice
actions), cross-register critical coverage (every critical companion
row has a matching seed entry), named-runtime-consumer existence, the
draft-output existence rule (build_dependency_notice_seed.py has been
run on a clean tree), and its fourteen named failure drills.
