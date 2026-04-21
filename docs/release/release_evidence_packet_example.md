# Release-evidence packet example

<!--
This is a filled seed example assembled from checked-in repository
artifacts. It demonstrates packet shape and stable refs; it is not a
release-ready packet.
-->

- **Packet id:** `release-evidence.seed.current-repository-baseline`
- **Packet state:** `draft`
- **Readiness:** `narrow_claims`
- **Candidate or scope:** `seeded repository baseline snapshot`
- **Opened on:** `2026-04-21`
- **Assembled on:** `2026-04-21T20:45:00Z`
- **Release channel scope:** `stable`
- **Deployment profile scope:** `individual_local`, `self_hosted`, `air_gapped`
- **Owner:** `@ahmedyounis`
- **Evidence owner:** `@ahmedyounis`
- **Review forums:** `release_council`, `shiproom_executive_scope_review`
- **Benchmark-governance revision:** `aureline.protected_fitness_function_catalog@1`
- **Primary exact-build identity set:**
  - `build-id:aureline:stable:0.7.3:x86_64-unknown-linux-gnu:release:a4d1c3f0e27b`
  - `build-id:aureline:stable:0.7.3:x86_64-unknown-linux-gnu:release:a4d1c3f0e27b:debug-symbols`
  - `build-id:aureline:stable:0.7.3:docs-pack:a4d1c3f0e27b`
- **Active waiver packet refs:** `none`

## Executive summary

This packet shows that the repository can assemble release evidence from
stable refs rather than prose-only naming. Exact-build propagation,
benchmark-governance revisioning, compatibility row ids, and continuity
drill ids are all present in one packet. The packet is not releasable:
the benchmark evidence is a self-capture seed, certification rows are
not current claim-bearing reports, and no concrete signed release packet
artifact exists yet.

## Architecture anchors

- **Source anchor refs:**
  - `docs/build/exact_build_identity_model.md#public-evidence-packet` — release evidence aggregates one coordinated exact-build identity set.
  - `.t2/docs/Aureline_Milestones_Document.md §3.36` — exact-build, reproducibility, and publication parity are release-bearing.
  - `.t2/docs/Aureline_Milestones_Document.md §3.21` — continuity and locality truth must remain explicit in release proof.
  - `.t2/docs/Aureline_PRD.md §4.4` — waivers must stay explicit, time-bounded, and visible in shiproom evidence.
- **Protected rows or requirement refs:**
  - `compat_row:release_identity.exact_build_propagation`
  - `compat_row:desktop_benchmark_lab.exact_build_identity`
  - `ff.buffer_operations`
  - `ff.vfs_save_conflict_handling`
  - `ff.benchmark_lab_health`

## Exact-build identity set

| Artifact family | `exact_build_identity_ref` | Evidence linkage | Source |
|---|---|---|---|
| `ide_binary` | `build-id:aureline:stable:0.7.3:x86_64-unknown-linux-gnu:release:a4d1c3f0e27b` | `build-log:ci:release:stable:0.7.3:linux`, `artifact-graph-node:aureline:stable:0.7.3:ide_binary:linux`, `release-note:aureline:stable:0.7.3` | `fixtures/build/exact_build_examples/ci_release_stable_linux_ide_binary.json` |
| `ide_debug_symbols` | `build-id:aureline:stable:0.7.3:x86_64-unknown-linux-gnu:release:a4d1c3f0e27b:debug-symbols` | `build-log:ci:release:stable:0.7.3:linux:debug-symbols`, `artifact-graph-node:aureline:stable:0.7.3:ide_debug_symbols:linux` | `fixtures/build/exact_build_examples/ci_release_stable_linux_ide_debug_symbols.json` |
| `docs_pack` | `build-id:aureline:stable:0.7.3:docs-pack:a4d1c3f0e27b` | `build-log:ci:release:stable:0.7.3:docs-pack`, `artifact-graph-node:aureline:stable:0.7.3:docs_pack`, `release-note:aureline:stable:0.7.3` | `fixtures/build/exact_build_examples/ci_release_stable_docs_pack.json` |

## Benchmark and fitness evidence

- **Catalog:** `artifacts/bench/fitness_function_catalog.yaml` (`catalog_revision: 1`)
- **Runs or dashboards cited:** `benchmark_run.seed.self_capture`, `aureline.benchmark_lab_dashboard`
- **Freshness and comparability:** the captured run is `self_capture` and `not_yet_comparable`; it demonstrates packet shape and row linkage, but it is not admissible as stable-claim evidence.

| `evidence_id` | Row or run ref | Captured at | `stale_after` | Comparability note | Source |
|---|---|---|---|---|---|
| `evidence.seed.benchmark.dashboard` | `aureline.benchmark_lab_dashboard` | `1970-01-01T00:00:00Z` | `P0D` | Seed timestamp only; immediately stale for release proof. | `artifacts/benchmarks/dashboard_seed/dashboard.json` |
| `evidence.seed.benchmark.self_capture` | `benchmark_run.seed.self_capture` | `1970-01-01T00:00:00Z` | `P0D` | `self_capture` and `not_yet_comparable`; useful for assembly, not claim widening. | `artifacts/benchmarks/dashboard_seed/raw/benchmark_run.seed.self_capture.json` |
| `evidence.seed.benchmark.buffer_gate` | `ff.buffer_operations` | `1970-01-01T00:00:00Z` | `P0D` | Passes a boolean gate in the seed run, but still inherits `self_capture` limitations. | `artifacts/benchmarks/dashboard_seed/raw/benchmark_run.seed.self_capture.json` |

## Qualification and compatibility

- **Qualification row refs:**
  - `compat_row:release_identity.exact_build_propagation` — `schema_and_fixture_seeded`, supported, fail-closed when the coordinated artifact set diverges. Source: `artifacts/compat/qualification_matrix_seed.yaml`.
  - `compat_row:desktop_benchmark_lab.exact_build_identity` — `benchmark_seed_available`, supported, fail-closed when benchmark evidence is not tied to exact-build identity. Source: `artifacts/compat/qualification_matrix_seed.yaml`.
- **Qualification packet or report refs:**
  - `artifacts/compat/qualification_matrix_seed.yaml` — `seed_only`
  - `fixtures/workspaces/reference/` — supporting reference-workspace inputs exist, but there is no current release-qualification report packet yet.
- **Future conformance or compatibility rows to refresh before widening claim:**
  - `compat_row:certification.launch_archetype_matrix` — certified claim wording still requires a current report.
  - `compat_row:remote.attach_envelope_and_drift` — remote attach remains future-facing compatibility work.
  - `compat_row:provider.service_api_and_browser_handoff` — provider-linked and browser-handoff claims remain narrower than core local release truth.
  - `compat_row:launcher.local_helper_contracts` — helper-process compatibility is reserved but not yet claim-bearing.

## Locality and continuity truth

- **Deployment context:** local-core benchmark capture with release-install topology references present but not yet tied to a concrete public candidate.
- **Continuity drill refs:** `benchmark_lab_local_only_capture`
- **Locality / region / tenant / key posture:** `device_local`, `not_applicable`, `single_user_local`, `os_store`
- **Linked release inputs:**
  - `artifacts/support/deployment_drill_catalog_seed.yaml`
  - `docs/deployment/drill_catalog_seed.md`
  - `docs/release/install_topology_plan.md`
  - `artifacts/release/install_topology_matrix.yaml`
  - `artifacts/release/state_root_map.yaml`

The current continuity truth is intentionally narrow: benchmark capture
continues locally and remains exportable, while hosted compare and
cohort refresh behavior are absent by design.

## Active waivers

`none`

## Waiver workflow

If a future packet needs a waiver, it must open a
`release_waiver_packet`, attach compensating evidence with explicit
freshness fields, move through `draft -> submitted_for_review -> active`
before it affects release readiness, and either close or renew with new
evidence before the next claim-widening gate.

## Risks and disclosure

- Benchmark evidence is seed-only and immediately stale for stable-claim use.
- Qualification rows exist, but no current beta/RC or stable qualification packet is attached.
- The continuity evidence proves local-only baseline behavior, not broader managed or mirrored release posture.
- No waiver packets are active, so there is no release-note disclosure requirement yet; if a protected row later ships waived, that disclosure must be added before promotion.

## Evidence index

- **`evidence_id:`** `evidence.seed.exact_build.ide_binary`
  - **Artifact family:** `exact_build_identity`
  - **Packet id:** `release-evidence.seed.current-repository-baseline`
  - **Evidence ref:** `fixtures/build/exact_build_examples/ci_release_stable_linux_ide_binary.json`
  - **Captured at:** `2026-04-13T14:05:12Z`
  - **Stale after:** `null`
  - **Source revision:** `commit:a4d1c3f0e27b6f91d8e2c1a4b7f3e0c5d9a8b2e1`
  - **Trigger revision:** `exact_build_identity_schema:1`
  - **Channel context:** `stable`
  - **Deployment context:** `individual_local`, `self_hosted`, `enterprise_online`, `air_gapped`, `managed_cloud`
  - **Comparability note:** Same coordinated artifact set as the paired debug-symbol and docs-pack identities.
  - **Exact-build identity ref:** `build-id:aureline:stable:0.7.3:x86_64-unknown-linux-gnu:release:a4d1c3f0e27b`
  - **Source anchor refs:** `docs/build/exact_build_identity_model.md#public-evidence-packet`
  - **Qualification row refs:** `compat_row:release_identity.exact_build_propagation`
  - **Continuity drill refs:** `none`
  - **Waiver packet refs:** `none`

- **`evidence_id:`** `evidence.seed.benchmark.self_capture`
  - **Artifact family:** `benchmark_run_result`
  - **Packet id:** `release-evidence.seed.current-repository-baseline`
  - **Evidence ref:** `artifacts/benchmarks/dashboard_seed/raw/benchmark_run.seed.self_capture.json`
  - **Captured at:** `1970-01-01T00:00:00Z`
  - **Stale after:** `P0D`
  - **Source revision:** `benchmark_run.seed.self_capture`
  - **Trigger revision:** `aureline.protected_fitness_function_catalog@1`
  - **Channel context:** `dev_local`
  - **Deployment context:** `individual_local`, `self_hosted`, `air_gapped`
  - **Comparability note:** Seed-only self-capture; not eligible for claim widening.
  - **Exact-build identity ref:** `exact_build_identity.seed.self_capture`
  - **Source anchor refs:** `docs/benchmarks/fitness_function_catalog.md#10-export-format-for-shiproom-review-packets`
  - **Qualification row refs:** `compat_row:desktop_benchmark_lab.exact_build_identity`
  - **Continuity drill refs:** `benchmark_lab_local_only_capture`
  - **Waiver packet refs:** `none`

- **`evidence_id:`** `evidence.seed.compat.release_identity`
  - **Artifact family:** `compatibility_row`
  - **Packet id:** `release-evidence.seed.current-repository-baseline`
  - **Evidence ref:** `artifacts/compat/qualification_matrix_seed.yaml`
  - **Captured at:** `2026-04-21T20:45:00Z`
  - **Stale after:** `P30D`
  - **Source revision:** `qualification_matrix_seed@1`
  - **Trigger revision:** `version_skew_register:release_identity.exact_build_propagation`
  - **Channel context:** `mixed`
  - **Deployment context:** `individual_local`, `self_hosted`, `enterprise_online`, `air_gapped`, `managed_cloud`
  - **Comparability note:** Seeded compatibility row; authoritative for ids and posture, not yet a current release report.
  - **Exact-build identity ref:** `null`
  - **Source anchor refs:** `docs/compat/compatibility_row_seed.md`
  - **Qualification row refs:** `compat_row:release_identity.exact_build_propagation`
  - **Continuity drill refs:** `none`
  - **Waiver packet refs:** `none`

- **`evidence_id:`** `evidence.seed.continuity.benchmark_lab_local_only_capture`
  - **Artifact family:** `continuity_drill`
  - **Packet id:** `release-evidence.seed.current-repository-baseline`
  - **Evidence ref:** `fixtures/deployment/impairment_cases/benchmark_lab_local_only_capture.json`
  - **Captured at:** `2026-04-21T20:45:00Z`
  - **Stale after:** `P30D`
  - **Source revision:** `deployment_drill_catalog_seed@1`
  - **Trigger revision:** `drill:benchmark_lab_local_only_capture`
  - **Channel context:** `not_applicable`
  - **Deployment context:** `individual_local`, `self_hosted`, `air_gapped`
  - **Comparability note:** Demonstrates continuity truth for local-only benchmark capture; does not prove hosted compare or managed release posture.
  - **Exact-build identity ref:** `null`
  - **Source anchor refs:** `docs/deployment/drill_catalog_seed.md`
  - **Qualification row refs:** `none`
  - **Continuity drill refs:** `benchmark_lab_local_only_capture`
  - **Waiver packet refs:** `none`

## Signoff and next action

- **Decision:** `narrow claims`
- **Named next action:** refresh this packet with a current claim-bearing benchmark capture, a current qualification report, and a concrete release packet artifact before any stable-facing promotion decision.
