# Critical dependency / import register and release-notice draft pipeline

Reviewer entry point for the canonical critical-dependency / import
register seed published at
[`artifacts/governance/critical_dependency_register.yaml`](../../artifacts/governance/critical_dependency_register.yaml).

The seed exists so the M1 build, release, and provenance lanes can
reason about third-party provenance, ownership, and notice generation
from a single executable contract. It does not replace the two source
registers; it projects their stable ids into a release-notice draft
contract that downstream automation (release packets, SBOM writers,
provenance statements, docs-pack manifests, support exports) reads
without reformatting.

## What the seed is

A versioned register projection of the critical M1 third-party
inputs. Each row binds one **stable upstream choice or imported byte
set** to:

1. a closed `template_class` drawn from `runtime_dependency` /
   `bundled_asset` / `build_tooling` / `host_runtime` / `mirrored_pack`
   so the draft pipeline can pick a template without inventing one;
2. a closed `criticality_class` so the seed cannot quietly downgrade a
   protected-path dependency;
3. a closed `publication_targets` set drawn from `third_party_notice`,
   `spdx_sbom`, `cyclonedx_sbom`, `provenance_statement`, and
   `docs_pack_manifest`;
4. a closed `release_notice_action_class` that names what the draft
   pipeline emits for the row;
5. a named owner DRI and a reviewable `fork_or_replace_trigger` so
   protected-path rows can never be anonymous; and
6. a named failure drill the validation lane reproduces under
   `--force-drill`.

## Canonical sources

- [`artifacts/governance/critical_dependency_register.yaml`](../../artifacts/governance/critical_dependency_register.yaml)
  — seed rows the draft pipeline and the validation lane consume.
- [`schemas/governance/critical_dependency_register.schema.json`](../../schemas/governance/critical_dependency_register.schema.json)
  — envelope schema; freezes vocabularies, required coverage lists,
  named-consumer shape, companion-register paths, and matrix
  identity.
- [`schemas/governance/critical_dependency_register_entry.schema.json`](../../schemas/governance/critical_dependency_register_entry.schema.json)
  — row schema; freezes the closed template / criticality /
  protected-path / license / provenance-status / admission-state /
  publication-target / release-notice-action vocabularies and the
  conditional invariants the runner asserts independently with
  precise actionable check_ids.

## Companion registers (the seed projects against these, never forks them)

| Companion register | Role |
| --- | --- |
| [`artifacts/governance/dependency_register.yaml`](../../artifacts/governance/dependency_register.yaml) | Upstream choices the build relies on (`dep.*` stable ids). |
| [`artifacts/governance/third_party_import_register.yaml`](../../artifacts/governance/third_party_import_register.yaml) | Copied / mirrored third-party bytes that ship with a release (`import.*` stable ids). |
| [`artifacts/governance/release_notice_seed.yaml`](../../artifacts/governance/release_notice_seed.yaml) | Existing publication-binding seed; the M1 critical register agrees with its template/publication choices without re-minting ids. |

The seed and the draft pipeline only key on the stable `dep.*` /
`import.*` ids declared in the companion registers. No separate
identifier space is permitted; minting a new id space is a breaking
change to this contract.

## Draft notice / report pipeline

The first executable consumer of the seed is
[`tools/governance/build_dependency_notice_seed.py`](../../tools/governance/build_dependency_notice_seed.py).
It reads the seed, validates each row against the row schema, resolves
each `source_id` against the named companion register, and emits:

- a deterministic markdown draft at
  [`artifacts/governance/build/dependency_notice_draft.md`](../../artifacts/governance/build/dependency_notice_draft.md)
  with one section per `release_notice_action_class`; and
- a JSON sidecar at
  [`artifacts/governance/build/dependency_notice_draft.json`](../../artifacts/governance/build/dependency_notice_draft.json)
  so downstream automation can read the payload without re-parsing
  markdown.

The tool fails closed when:

- a seed row's `source_id` does not resolve in the named companion
  register;
- a companion register declares a row whose `criticality` is in
  `{protected_path_release_critical, release_engineering_critical,
  benchmark_lab_required}` but no seed entry cites that `source_id`
  (the spec's failure drill: "Add or update a critical dependency
  without register / update-notice changes -> automation flags the
  omission");
- the seed's vocabulary disagrees with the row schema `$defs`;
- a row violates the template / publication-target /
  release-notice-action invariants the row schema declares; or
- `--check` is passed and the on-disk draft outputs differ from what
  the tool would re-emit.

Run modes:

```sh
# regenerate the draft outputs (default)
python3 tools/governance/build_dependency_notice_seed.py --repo-root .

# CI-friendly drift check; exits non-zero on stale drafts
python3 tools/governance/build_dependency_notice_seed.py --repo-root . --check

# simulate omitting a row to prove the omission gate fires
python3 tools/governance/build_dependency_notice_seed.py --repo-root . \
  --force-drill 'omit:cdr.runtime_dependency.renderer_wgpu'
```

## Validation lane (CI / review)

[`tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py`](../../tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py)
is the unattended review-grade validator. It re-runs the row /
envelope vocabulary checks, asserts the closed-vocabulary agreement
with the row schema `$defs`, resolves the named-runtime-consumer set,
re-checks the cross-register critical-coverage gate, and writes a
durable capture at
[`artifacts/milestones/m1/captures/dependency_and_notice_seed_validation_capture.json`](../../artifacts/milestones/m1/captures/dependency_and_notice_seed_validation_capture.json).

```sh
python3 tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py \
  --repo-root .
```

Reproduce a named failure drill loudly with `--force-drill`:

```sh
python3 tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py \
  --repo-root . \
  --force-drill 'cdr.runtime_dependency.renderer_wgpu:critical_dependency_register_drill.runtime_dependency_third_party_notice_dropped'
```

The drill exits 0 only when the runner reproduces the row's declared
`expected_check_id`.

## M1 register-row coverage

The seed covers every closed `template_class` and exercises every
required publication target and release-notice action:

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

Each row's named drill is reproducible under `--force-drill` against
the validation lane:

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

## Refresh procedure

Re-run the draft pipeline AND the validation lane after any change
to:

- the seed YAML,
- either schema (envelope or row),
- this reviewer landing page,
- a companion register (`dependency_register.yaml`,
  `third_party_import_register.yaml`, `release_notice_seed.yaml`)
  whose row identity or critical posture changed, or
- the build-identity record the capture embeds.

```sh
python3 tools/governance/build_dependency_notice_seed.py --repo-root .
python3 tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py --repo-root .
```

## Out of scope (deliberately)

- a full software-composition-analysis platform,
- a vulnerability-management suite,
- a public notice portal,
- crawling upstream for license / activity facts; the optional
  best-effort probe lives at
  `tools/governance/dependency_ingest/refresh_upstream_observations.py`
  and is evidence for human review only.

This seed is a **register and draft pipeline**, not a release-notice
runtime. It freezes the data model the draft pipeline executes on so
the M1 release work can be planned and reviewed without ad-hoc
spreadsheets.
