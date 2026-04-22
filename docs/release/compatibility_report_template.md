# Compatibility-report template

<!--
Copy this template when assembling a release-time compatibility report,
an out-of-band compatibility refresh, or a narrowed-claim correction.

Related control artifacts:
- artifacts/compat/qualification_matrix_seed.yaml
- artifacts/compat/version_skew_register.yaml
- docs/compat/compatibility_row_seed.md
- artifacts/governance/governance_packet_template.yaml
- schemas/governance/evidence_packet_header.schema.json
- schemas/release/compatibility_row.schema.json
- docs/release/certified_archetype_report_template.md
- docs/release/release_evidence_packet_template.md
- docs/governance/dogfood_issue_taxonomy.md

Compatibility reports extend seeded `row_id` values by reference. They
do not rename boundary families, collapse supported / best_effort /
untested into one vague "partial" label, or replace stable ids with
release-note prose.
-->

This template is the reviewer-facing companion for release-time
compatibility publication. It turns the seeded rows in
`artifacts/compat/qualification_matrix_seed.yaml` into a reusable report
format that release, docs, support, migration, and qualification-cadence
work can all read without inventing a second compatibility dialect.

## Shared packet shell

Every compatibility report SHOULD:

- use the `compatibility_report` packet family from
  `artifacts/governance/governance_packet_template.yaml`;
- embed a shared header that conforms to
  `schemas/governance/evidence_packet_header.schema.json`;
- serialize each report row to
  `schemas/release/compatibility_row.schema.json`;
- quote `row_id`, `artifact_or_protocol_boundary_label`, and
  `version_skew_register_ref` exactly as they appear in
  `artifacts/compat/qualification_matrix_seed.yaml`; and
- link any certified-workspace or launch-language claims back to
  `docs/release/certified_archetype_report_template.md` instead of
  restating them locally.

## Report metadata

- **Report id:** `<compatibility-report-id>`
- **Packet state:** `draft` | `in_review` | `accepted` | `narrowed` |
  `blocked` | `superseded`
- **Scope:** channel, build family, deployment envelope, or correction
  window being described
- **Release channel scope:** `nightly` | `preview` | `beta` |
  `stable` | `lts` | `hotfix`
- **Claimed deployment profiles:** ids from
  `artifacts/compat/qualification_matrix_seed.yaml`
- **Owner:** `@handle`
- **Evidence owner:** `@handle`
- **Opened on:** `YYYY-MM-DD`
- **Generated on:** `YYYY-MM-DDTHH:MM:SSZ`
- **Row schema:** `schemas/release/compatibility_row.schema.json`
- **Qualification seed revision:** path or commit for
  `artifacts/compat/qualification_matrix_seed.yaml`
- **Version-skew register revision:** path or commit for
  `artifacts/compat/version_skew_register.yaml`
- **Current release packet refs:** packet ids or repo-relative paths
- **Freshness summary:** one sentence that says whether the report is
  current, caveated, retest-pending, or stale

## Status model

Keep support tier, current report state, and out-of-window posture
separate.

| Field | Vocabulary | Meaning |
|---|---|---|
| `support_class` | `certified`, `supported`, `community`, `experimental` | What class of claim the row belongs to when evidence is current. |
| `current_state` | `supported`, `best_effort`, `untested`, `degraded`, `unsupported` | What this report is actually asserting today. |
| `out_of_window_posture` | `fail_closed`, `read_only`, `degraded`, `explicitly_unsupported` | What happens once the row falls outside its declared window. |

Required interpretation:

- `supported` is claim-bearing inside the declared skew and freshness
  window.
- `best_effort` is usable but not promotion-grade; caveats and
  follow-up actions must stay attached to the row.
- `untested` is reserved explicitly; it is not a soft synonym for
  supported.
- `degraded` means the row stays inspectable while narrowing scope,
  fidelity, or authority.
- `unsupported` means the report must refuse the claim-bearing path or
  surface a typed denial.

## Row record

Each row SHOULD be represented as a machine-readable record that
conforms to `schemas/release/compatibility_row.schema.json`.

```yaml
schema_version: 1
record_kind: compatibility_row
report_family: compatibility_report
row_id: compat_row:<family>.<row>
artifact_or_protocol_boundary_label: boundary.<family>.<boundary>
claimed_surface: <surface-id>
source_family: <schema_family | build_identity | profile_schema | platform_profile | ...>
source_version: <semver | schema_epoch | manifest_revision | profile_revision>
support_class: supported
current_state: supported
qualification_evidence_status: schema_and_fixture_seeded
claimed_deployment_profiles:
  - individual_local
version_skew_register_ref: skew_register:<family>.<row>
skew_window_class: same_schema_epoch_additive_only
skew_window_summary: >
  One sentence describing the currently defended compatibility window.
current_skew_case_ref: skew_case:<case-id>
out_of_window_posture: fail_closed
review_cadence: each_change
retained_support_window: >
  Support promise retained until the published replacement or overlap
  rule is met.
deprecation_posture:
  state: none
  replacement_refs: []
  earliest_removal_scope: null
  disclosure_refs: []
  note: No active deprecation notice on this row.
migration_guidance_refs:
  - docs/<migration-guide>.md
issue_template_refs:
  - docs/governance/dogfood_issue_taxonomy.md
export_refs:
  - support_bundle.semantic_readiness_packet
owner: "@handle"
evidence_owner: "@handle"
evidence_packet_refs:
  - compatibility.<report-id>
supporting_evidence_refs:
  - <repo-relative path or evidence id>
known_deviations:
  - deviation_id: compat_deviation:<token>
    summary: <short deviation>
    impact: <user-visible or reviewer-visible effect>
    workaround: <current workaround or narrower path>
    owner: "@handle"
    issue_template_ref: docs/governance/dogfood_issue_taxonomy.md
    export_ref: support_bundle.semantic_readiness_packet
    tracking_ref: <issue, packet, or note ref>
freshness:
  captured_at: 2026-04-21T23:45:00Z
  stale_after: P14D
  freshness_state: current
  next_review_target: stable-rc-1
notes: >
  Any extra report-local guidance that does not belong in the seeded row.
```

## Required per-row questions

Reviewers should be able to answer the following from each row without
opening a second spreadsheet:

1. Which seeded boundary row is this report extending?
2. What source family and version or revision were actually checked?
3. Is the row currently supported, best-effort, untested, degraded, or
   unsupported?
4. What evidence packet or fixture backs that verdict?
5. When does the row go stale, and who owns refreshing it?
6. What migration path or deprecation notice applies if the row narrows?
7. Which issue-intake path and export surface receive this row when it
   fails in the field?

## Seed example rows

These are illustrative seed rows built from the current repository
artifacts. They are examples of report shape, not live launch claims.

### Benchmark hardware and exact-build linkage

- **Row id:** `compat_row:desktop_benchmark_lab.exact_build_identity`
- **Source family/version:** `reference_laptop_matrix`,
  `artifacts/perf/reference_laptop_matrix.yaml@schema_version=1`
- **Support class / current state:** `supported` / `supported`
- **Skew case:** `skew_case:desktop_benchmark_lab.same_identity`
- **Evidence packet refs:** `compatibility.seed.desktop_benchmark_lab`
- **Supporting evidence refs:**
  `artifacts/perf/reference_laptop_matrix.yaml`,
  `artifacts/benchmarks/dashboard_seed/dashboard.json`,
  `fixtures/build/exact_build_examples/ci_release_stable_linux_ide_binary.json`
- **Freshness:** `current`, refresh whenever the reference hardware
  image, exact-build identity, or benchmark dashboard revision changes
- **Issue template / export refs:**
  `docs/governance/dogfood_issue_taxonomy.md`,
  `support_bundle.performance_summary`
- **Notes:** Use this row when benchmark hardware or exact-build
  comparability becomes part of a compatibility claim. Cross-channel
  same-commit comparison stays out of window until the register widens.

### Command-plane schema parity

- **Row id:** `compat_row:command_plane.command_descriptor_schema`
- **Source family/version:** `command_descriptor_schema`,
  `schemas/commands/command_descriptor.schema.json@schema_version=1`
- **Support class / current state:** `supported` / `supported`
- **Skew case:** `skew_case:command_plane.same_schema_epoch`
- **Evidence packet refs:** `compatibility.seed.command_plane`
- **Supporting evidence refs:**
  `docs/commands/command_descriptor_contract.md`,
  `schemas/commands/command_descriptor.schema.json`,
  `fixtures/commands/command_descriptor_examples/README.md`
- **Freshness:** `current`, refresh on any schema-epoch change or
  command-surface meaning change
- **Issue template / export refs:**
  `docs/governance/dogfood_issue_taxonomy.md`,
  `support_bundle.semantic_readiness_packet`
- **Notes:** Additive UI-slot drift may remain `best_effort`; required
  field or meaning drift is `unsupported` and fails closed.

### Build-truth propagation

- **Row id:** `compat_row:release_identity.exact_build_propagation`
- **Source family/version:** `exact_build_identity`,
  `schemas/build/exact_build_identity.schema.json@schema_version=1`
- **Support class / current state:** `supported` / `supported`
- **Skew case:** `skew_case:release_identity.same_artifact_set`
- **Evidence packet refs:** `compatibility.seed.release_identity`
- **Supporting evidence refs:**
  `docs/build/exact_build_identity_model.md`,
  `schemas/build/exact_build_identity.schema.json`,
  `fixtures/build/exact_build_examples/README.md`
- **Freshness:** `current`, refresh on any producer-lane, docs/help, or
  support-export linkage change
- **Issue template / export refs:**
  `docs/governance/dogfood_issue_taxonomy.md`,
  `release_evidence.claim_manifest`
- **Notes:** Same-version fallback without exact-build identity stays
  outside the contract even when the semver string matches.

### Profile and layout schema drift

- **Row id:** `compat_row:state.profile_layout_schema`
- **Source family/version:** `portable_profile_schema`,
  `schemas/profile/portable_profile.schema.json@schema_version=1`
- **Support class / current state:** `supported` / `best_effort`
- **Skew case:** `skew_case:state.profile_layout_only_downgrade`
- **Evidence packet refs:** `compatibility.seed.profile_restore`
- **Supporting evidence refs:**
  `docs/state/profile_and_state_map.md`,
  `schemas/profile/portable_profile.schema.json`,
  `fixtures/profile/restore_provenance_examples/restore_provenance_compatible.json`
- **Freshness:** `known_issues`, refresh before widening restore wording
  beyond `compatible` or `layout_only`
- **Issue template / export refs:**
  `docs/governance/dogfood_issue_taxonomy.md`,
  `support_bundle.managed_workspace_summary`
- **Notes:** Layout-only or manual-review restore is a legitimate
  best-effort state and must stay explicit in the report.

### Desktop shell and input conformance

- **Row id:** `compat_row:desktop.platform_conformance_profiles`
- **Source family/version:** `claimed_desktop_profiles`,
  `artifacts/platform/claimed_desktop_profiles.yaml@schema_version=1`
- **Support class / current state:** `supported` / `degraded`
- **Skew case:** `skew_case:desktop.platform.current_claimed_profile_revision`
- **Evidence packet refs:** `compatibility.seed.desktop_platform`
- **Supporting evidence refs:**
  `docs/platform/desktop_platform_conformance_matrix.md`,
  `artifacts/platform/claimed_desktop_profiles.yaml`,
  `artifacts/qa/window_display_matrix.yaml`,
  `docs/qa/multi_window_verification.md`
- **Freshness:** `retest_pending`, refresh before broadening shell,
  window, IME, or secret-store claims beyond the named profiles
- **Issue template / export refs:**
  `docs/governance/dogfood_issue_taxonomy.md`,
  `support_bundle.first_run_summary`
- **Notes:** Report rows for shell/input conformance must name the
  claimed desktop profile, not only an OS family. New Linux desktop
  environments or Windows ARM64 remain explicit `untested` or
  `unsupported` cases until a profile row exists.

## Publication and refresh rules

- Keep report ids stable across freshness refreshes. Update row
  freshness, evidence refs, and notes; do not rename the report because
  a retest happened.
- If a row narrows from `supported` to `best_effort` or `degraded`,
  carry the old and new states in the change note and update the linked
  migration or known-limit refs in the same change.
- Do not widen a row to `supported` unless its evidence packet refs,
  freshness fields, and issue/export refs are all populated.
- A stale report may remain inspectable, but it must not keep
  `supported` wording after the freshness window expires.
