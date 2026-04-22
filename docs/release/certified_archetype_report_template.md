# Certified-archetype report template

<!--
Copy this template when assembling a certified-archetype report, a
launch-language qualification packet, or a narrower archetype refresh.

Related control artifacts:
- artifacts/compat/qualification_matrix_seed.yaml
- artifacts/compat/version_skew_register.yaml
- schemas/release/compatibility_row.schema.json
- schemas/governance/evidence_packet_header.schema.json
- artifacts/governance/governance_packet_template.yaml
- fixtures/benchmarks/corpus_manifest.yaml
- fixtures/workspaces/reference/
- artifacts/perf/reference_laptop_matrix.yaml
- artifacts/platform/claimed_desktop_profiles.yaml
- docs/platform/desktop_platform_conformance_matrix.md
- docs/release/compatibility_report_template.md
- docs/benchmarks/benchmark_publication_pack_template.md
- docs/governance/dogfood_issue_taxonomy.md

Certified-archetype reports reuse the same seeded `row_id` and
compatibility-row schema as compatibility reports. They add the
hardware, toolchain, reference-workspace, workflow, and platform-profile
rows needed to defend a "Certified" or "Supported" archetype claim.
-->

This template is the reviewer-facing packet for archetype qualification.
It keeps benchmark hardware, toolchain posture, reference workspace
shape, workflow outcomes, platform profile narrowings, and evidence
freshness on one report surface.

## Shared packet shell

Every certified-archetype report SHOULD:

- use the `compatibility_report` packet family until a dedicated
  archetype packet family exists;
- embed a shared header that conforms to
  `schemas/governance/evidence_packet_header.schema.json`;
- represent each governed row with
  `schemas/release/compatibility_row.schema.json` using
  `report_family: certified_archetype_report`;
- cite `compat_row:certification.launch_archetype_matrix` for the
  archetype claim itself and then link any prerequisite compatibility
  rows that bound the claim; and
- point back to `docs/release/compatibility_report_template.md` for the
  shared state model, migration posture, and downgrade rules.

## Required report front matter

- **Report id:** `<certified-archetype-report-id>`
- **Packet state:** `draft` | `in_review` | `accepted` | `narrowed` |
  `blocked` | `superseded`
- **Archetype ids in scope:** one or more canonical archetype ids
- **Release/build scope:** channel, exact-build identity set, or
  benchmark-publication packet id
- **Deployment profiles:** ids from
  `artifacts/compat/qualification_matrix_seed.yaml`
- **Owner:** `@handle`
- **Evidence owner:** `@handle`
- **Generated on:** `YYYY-MM-DDTHH:MM:SSZ`
- **Freshness summary:** one sentence naming whether the archetype rows
  are current, caveated, retest-pending, or stale
- **Linked compatibility report refs:** packet ids or repo-relative
  paths

## Required matrices

Every archetype report SHOULD make the following rows explicit:

| Matrix | What it names | Minimum required fields |
|---|---|---|
| Hardware row | benchmark or qualification host profile | profile ref, OS/arch, evidence ref, freshness, owner |
| Toolchain row | required or declared-only toolchain posture | toolchain ref, version or assumption, evidence ref, freshness, owner |
| Reference-workspace row | repo slice or synthetic workspace shape | corpus/reference-workspace ref, repo or slice scope, evidence ref, freshness, owner |
| Workflow row | one workflow or benchmark scenario the claim depends on | workflow ref, verdict, caveat, evidence ref, freshness, owner |
| Platform-profile row | desktop-profile narrowing or OS claim | claimed profile ref, evidence ref, freshness, owner |

## Row record

Use the shared row schema with the archetype extension filled in.

```yaml
schema_version: 1
record_kind: compatibility_row
report_family: certified_archetype_report
row_id: compat_row:certification.launch_archetype_matrix
artifact_or_protocol_boundary_label: boundary.certification.launch_archetype_matrix
claimed_surface: certified_archetype_and_launch_language_claims
source_family: reference_workspace_report
source_version: fixtures/benchmarks/corpus_manifest.yaml@manifest_revision=1
support_class: certified
current_state: supported
qualification_evidence_status: benchmark_seed_available
claimed_deployment_profiles:
  - individual_local
version_skew_register_ref: skew_register:certification.launch_archetype_matrix
skew_window_class: certified_report_freshness_window
skew_window_summary: >
  Certified wording remains valid only while the linked reference
  workspace report, workflow evidence, and caveat set stay fresh for
  the named build and claimed profile set.
current_skew_case_ref: skew_case:certification.current_report_and_row_ids
out_of_window_posture: degraded
review_cadence: each_release
retained_support_window: >
  Keep the archetype on the current report until a fresher report or an
  explicit downgrade notice replaces it.
deprecation_posture:
  state: none
  replacement_refs: []
  earliest_removal_scope: null
  disclosure_refs: []
  note: No active deprecation notice on this archetype row.
migration_guidance_refs:
  - docs/release/compatibility_report_template.md
issue_template_refs:
  - docs/governance/dogfood_issue_taxonomy.md
export_refs:
  - release_evidence.claim_manifest
owner: "@handle"
evidence_owner: "@handle"
evidence_packet_refs:
  - certified_archetype.<report-id>
supporting_evidence_refs:
  - fixtures/benchmarks/corpus_manifest.yaml
  - artifacts/perf/reference_laptop_matrix.yaml
  - artifacts/platform/claimed_desktop_profiles.yaml
known_deviations: []
freshness:
  captured_at: 2026-04-21T23:45:00Z
  stale_after: P14D
  freshness_state: current
  next_review_target: stable-rc-1
archetype_matrix:
  archetype_id: rust_workspace_self_host
  hardware_profile_ref: ref.arm64.macos15.apple_silicon_14in
  toolchain_ref: rust_toolchain_pinned_v1
  reference_workspace_ref: refws.small_rust_self_host_slice
  workflow_refs:
    - workflow.first_useful_edit_rust_self_host
  platform_profile_refs:
    - macos_15_plus_universal
notes: >
  The archetype row stays claim-bearing only while the linked workflow
  and desktop-profile evidence remain current.
```

## State use

Certified-archetype packets use the same `current_state` vocabulary as
compatibility reports:

- `supported` means the named hardware, toolchain, repo/workspace, and
  workflow rows are currently backed by fresh evidence.
- `best_effort` means the archetype is still useful but one or more rows
  carry explicit caveats and cannot be marketed as certified.
- `untested` means the archetype id, hardware row, or workflow appears
  before a fresh report exists.
- `degraded` means the packet remains inspectable while narrowing the
  claim to a subset of workflows, platforms, or toolchain assumptions.
- `unsupported` means the report must not render certified or supported
  wording for that tuple.

## Seed example rows

These are illustrative seed rows built from the repository's current
benchmark corpus, reference workspaces, and platform-conformance assets.
They are examples of packet shape, not live launch claims.

### Rust self-host slice on the macOS ARM64 reference laptop

- **Archetype row:** `rust_workspace_self_host`
- **Hardware row:** `ref.arm64.macos15.apple_silicon_14in` from
  `artifacts/perf/reference_laptop_matrix.yaml`
- **Toolchain row:** `rust_toolchain_pinned_v1` from
  `fixtures/workspaces/reference/small_rust_self_host_slice.json`
- **Reference-workspace row:** `refws.small_rust_self_host_slice`
- **Workflow rows:** `workflow.startup_rust_self_host_slice`,
  `workflow.first_useful_edit_rust_self_host`
- **Platform-profile row:** `macos_15_plus_universal`
- **Support class / current state:** `certified` / `supported`
- **Evidence packet refs:** `certified_archetype.seed.rust_self_host`
- **Supporting evidence refs:**
  `fixtures/workspaces/reference/small_rust_self_host_slice.json`,
  `fixtures/benchmarks/corpus_manifest.yaml`,
  `docs/platform/desktop_platform_conformance_matrix.md`
- **Freshness:** `current`, refresh when the reference laptop image, the
  live repo slice, or the macOS claimed-profile row changes
- **Issue template / export refs:**
  `docs/governance/dogfood_issue_taxonomy.md`,
  `release_evidence.claim_manifest`
- **Notes:** This row stays claim-bearing only while the slice
  resolution and the macOS profile remain stable. Portable or unmanaged
  macOS narrowings stay outside this exact row unless added explicitly.

### TS web-app seed on the Ubuntu GNOME Wayland reference laptop

- **Archetype row:** `ts_web_app`
- **Hardware row:** `ref.x86_64.ubuntu24_04.framework13` from
  `artifacts/perf/reference_laptop_matrix.yaml`
- **Toolchain row:** `node_js_declared_only` placeholder until a
  dedicated toolchain packet lands
- **Reference-workspace row:** `refws.ts_web_app_archetype_seed`
- **Workflow rows:** `archetype.ts_web_app_first_open_certified`,
  `workflow.first_useful_edit_ts_web_app`
- **Platform-profile row:** `linux_ubuntu_24_04_gnome_wayland_x86_64`
- **Support class / current state:** `supported` / `best_effort`
- **Evidence packet refs:** `certified_archetype.seed.ts_web_app`
- **Supporting evidence refs:**
  `fixtures/workspaces/reference/ts_web_app_archetype_seed.json`,
  `fixtures/benchmarks/corpus_manifest.yaml`,
  `artifacts/platform/claimed_desktop_profiles.yaml`
- **Freshness:** `known_issues`, refresh before widening the row from
  seeded archetype detection to full certified workflow language
- **Issue template / export refs:**
  `docs/governance/dogfood_issue_taxonomy.md`,
  `support_bundle.first_run_summary`
- **Notes:** The fixture intentionally omits a lockfile and installed
  dependency tree. Those caveats must remain attached to the row until a
  fuller workflow packet replaces the seed.

### Python data-app seed waiting on a broader workflow packet

- **Archetype row:** `python_data_app`
- **Hardware row:** `ref.x86_64.ubuntu24_04.framework13`
- **Toolchain row:** `python_interpreter_declared_only` placeholder
- **Reference-workspace row:** `refws.python_data_app_archetype_seed`
- **Workflow rows:** `archetype.python_data_app_first_open_certified`,
  `workflow.first_useful_edit_python_data_app`
- **Platform-profile row:** `linux_ubuntu_24_04_gnome_wayland_x86_64`
- **Support class / current state:** `supported` / `untested`
- **Evidence packet refs:** `certified_archetype.seed.python_data_app`
- **Supporting evidence refs:**
  `fixtures/workspaces/reference/python_data_app_archetype_seed.json`,
  `fixtures/benchmarks/corpus_manifest.yaml`
- **Freshness:** `retest_pending`, refresh before using this row in
  certified or launch-language copy
- **Issue template / export refs:**
  `docs/governance/dogfood_issue_taxonomy.md`,
  `support_bundle.first_run_summary`
- **Notes:** This is the explicit reserved state for a named archetype
  that has corpus coverage but not yet a broad enough workflow packet to
  justify stronger wording.

## Publication and downgrade rules

- Only call a row `certified` when the hardware, toolchain,
  reference-workspace, workflow, and platform-profile rows are all
  present and fresh.
- If any one of those rows goes stale, narrow the archetype row to
  `best_effort` or `degraded` first; do not leave the stale row hidden
  behind the archetype label.
- If a new archetype id appears before a corpus or reference-workspace
  row exists, keep it `untested` or `unsupported` and cite
  `skew_case:certification_new_archetype_not_in_corpus`.
- Keep exact issue-template refs and export refs attached to each row so
  support, docs, and release can route the same failure without
  translating the archetype name into a new local label.
