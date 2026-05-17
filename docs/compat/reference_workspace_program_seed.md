# Reference-workspace program seed

This document is the narrative companion to the reference-workspace
program. Tooling reads the machine-readable rubric and inventory; this
file gives reviewers the support-class meaning, the evidence burden per
class, and the graduation/demotion mechanics that bind reference
fixtures, the protected benchmark corpus, compatibility reports, and
claim-manifest rows together.

Companion artifacts:

- [`/artifacts/compat/archetype_rubric.yaml`](../../artifacts/compat/archetype_rubric.yaml)
  — closed support-class taxonomy, evidence-burden vocabulary, owner
  allocation, release-treatment posture, graduation paths, and demotion
  rules. If this document and the rubric disagree, the rubric is
  authoritative and this document is updated in the same change.
- [`/artifacts/compat/reference_workspace_rows.yaml`](../../artifacts/compat/reference_workspace_rows.yaml)
  — per-archetype seed rows for the v1.0 inventory plus explicit
  exclusion rows for archetypes intentionally outside the foundations
  scope.
- [`/artifacts/compat/m3/reference_workspace_register.yaml`](../../artifacts/compat/m3/reference_workspace_register.yaml)
  — beta reference-workspace corpus register with owners, toolchain
  pins, privacy/license posture, workflow harnesses, and consumer refs.
- [`/fixtures/compat/archetype_seed_notes/`](../../fixtures/compat/archetype_seed_notes/)
  — one short notes file per archetype row recording the representative
  stack, the required-mode rationale, and any open evidence questions.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  — every archetype row binds to
  `compat_row:certification.launch_archetype_matrix`.
- [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
  — every archetype row extends
  `skew_register:certification.launch_archetype_matrix`.
- [`/docs/release/certified_archetype_report_template.md`](../release/certified_archetype_report_template.md)
  — the report template the certified support class requires.
- [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — corpus-scenario ids the rows reference (or reserve).
- [`/docs/product/launch_language_bundle_rubric.md`](../product/launch_language_bundle_rubric.md)
  — launch-language bundle and framework-pack rubric. Bundles bind to
  archetype rows by `archetype_row_id`; the rubric is the bridge
  between an archetype row and the top-level claim wording that cites
  a `launch_bundle:` id.

## Why this exists before launch

The PRD already establishes the principle: "supports Python" or
"supports React" is not a precise product claim. Every top-level
support claim should resolve through a named **archetype row** that
carries a reproducible reference workspace, a benchmark corpus link,
a compatibility report, and a published claim-manifest row.

What was missing before this seed:

- a closed support-class vocabulary shared by archetype rows,
  compatibility rows, and claim-manifest rows;
- per-class evidence-burden rules that say what must be true before
  any one of `Certified`, `Supported`, `Community`, or `Experimental`
  is admissible;
- one inventory of which archetypes the program is committing to and
  which archetypes are intentionally excluded.

This document and its companion artifacts close those gaps without
certifying any archetype. The certification work itself stays the job
of later milestones; this seed makes those milestones inspectable from
day one.

## Support-class taxonomy

The program ships with exactly four support classes. The wording below
mirrors the PRD §5.38 and the technical-design §7.13.7 tables; any
future tightening lands in the rubric and propagates here in the same
change.

| Support class | Public meaning | Top-level claim admitted | Evidence burden (closed set) | Owners (selection / evidence / publication) | Release treatment |
|---|---|---|---|---|---|
| **Certified** | Release-blocking reference workspace with named owner and current results on the claimed matrix. | Yes — top-level launch and evaluation copy. | reference_workspace_required, benchmark_corpus_required, compatibility_report_required, certified_archetype_report_required, claim_manifest_row_required | `lane:compatibility_ecosystem_review` / `lane:benchmark_lab` / `lane:release_evidence` | May appear in headline launch and evaluation copy. |
| **Supported** | First-party tested and documented, but not on every release-blocking permutation. | No — docs and scoped roadmap only. | reference_workspace_required, benchmark_corpus_required, compatibility_report_required, claim_manifest_row_required | `lane:compatibility_ecosystem_review` / `lane:benchmark_lab` / `lane:docs_public_truth` | Docs and scoped roadmap copy with explicit caveats. |
| **Community** | Known path exists via extensions or community tooling. | No — community/extension surfaces only. | reference_workspace_required, design_partner_input_optional | `lane:compatibility_ecosystem_review` / `lane:open_community_sync` / `lane:docs_public_truth` | Discoverable on community surfaces; not marketed as replacement-grade. |
| **Experimental** | Valuable direction under active iteration. | No — preview/experimental label required. | design_partner_input_optional, none_required | `lane:product_scope_review` / `lane:compatibility_ecosystem_review` / `lane:docs_public_truth` | Preview-only language and no stable claim wording. |

The closed evidence-burden vocabulary lives in
`artifacts/compat/archetype_rubric.yaml` under
`evidence_burden_class_vocabulary`. Adding a class is breaking;
tightening the wording on an existing class is additive-minor.

## What top-level claims may say

A "supports X" claim is admissible only when it quotes one
`archetype_row_id` from
`artifacts/compat/reference_workspace_rows.yaml` at the named
`claim_admission_class`. Quoting "supports Python" or "supports Java"
in launch copy without an `archetype_row_id` reference is
non-conforming.

That mechanical bridge — claim → archetype row → reference workspace,
benchmark corpus, compatibility report, claim-manifest row — is the
whole point of the program.

## Seed inventory

The reference-workspace inventory commits to the following active
archetype rows. Each row binds to
`compat_row:certification.launch_archetype_matrix` and extends
`skew_register:certification.launch_archetype_matrix`.

| Archetype row id | Archetype id | Initial class | Target class | Inclusion target | Reference-workspace ref(s) |
|---|---|---|---|---|---|
| `archetype_row:ts_web_app_or_service` | `ts_web_app` | `experimental` | `certified` | `first_stable` | `refws.ts_web_app_archetype_seed` |
| `archetype_row:python_service_or_data_app` | `python_data_app` | `experimental` | `certified` | `first_stable` | `refws.python_data_app_archetype_seed` |
| `archetype_row:java_or_kotlin_service` | `java_or_kotlin_service` | `experimental` | `certified` | `first_stable` | `refws.java_kotlin_service_archetype_seed` |
| `archetype_row:rust_workspace` | `rust_workspace_self_host` | `supported` | `certified` | `foundations` | `refws.small_rust_self_host_slice` |
| `archetype_row:go_service_or_monorepo_slice` | `go_service_or_monorepo_slice` | `experimental` | `certified` | `first_stable` | `refws.go_service_archetype_seed` |
| `archetype_row:c_or_cpp_native_project` | `c_or_cpp_native_project` | `experimental` | `certified` | `first_stable` | `refws.c_cpp_native_archetype_seed` |
| `archetype_row:dotnet_service_or_app` | `dotnet_service_or_app` | `experimental` | `supported` | `first_beta` | reservation under `fixtures/workspaces/reference/` |
| `archetype_row:notebook_first_data_workflow` | `notebook_first_data_workflow` | `experimental` | `supported` | `first_beta` | reservation under `fixtures/workspaces/reference/` |

The inventory also carries one explicit exclusion row:

| Archetype row id | Archetype id | Exclusion reason | Inclusion target |
|---|---|---|---|
| `archetype_row:misc_local_folder_no_archetype` | `misc_folder` | `not_a_first_party_support_archetype` | `post_stable` |

The plain local-folder row is a corpus fixture for the
unrecognised-archetype path. It is intentionally outside the support
ladder; the row is recorded here so the inventory is complete and so
no surface promotes it to a support class by accident.

### Why .NET and the notebook-first row land at supported

The seed inventory deliberately includes a .NET service/app row and a
notebook-first data workflow row even though both target `supported`
rather than `certified` for the first stable cut. They are present so
the program is honest about its v1.0 reach: the work to make them
first-party is named, the evidence burden is the same as any other
supported row, and promotion to `certified` runs through the same
graduation path the launch-wedge archetypes use.

The notebook-first row also keeps the inventory consistent with the
structured-artifact review surface, which already requires cell-aware
compare and explicit notebook-output handling for any notebook surface.

## Evidence burden in detail

Every active row must carry at least the closed-set evidence the
support class demands. The required references are:

- **reference_workspace_required** — at least one entry in
  `reference_workspace_refs` resolves to a fixture in
  `fixtures/workspaces/reference/` or to a reservation slot under
  that directory. Reservation slots use the `reservation:<path>`
  prefix so reviewers can see the slot is named but not yet
  materialised.
- **benchmark_corpus_required** — at least one entry in
  `benchmark_corpus_refs` resolves to a corpus-scenario id in
  `fixtures/benchmarks/corpus_manifest.yaml` or a reserved scenario
  id (`reservation:archetype.<id>` / `reservation:workflow.<id>`).
- **compatibility_report_required** — the row has a current
  compatibility-report row that conforms to
  `schemas/release/compatibility_row.schema.json` and uses
  `report_family: compatibility_report`.
- **certified_archetype_report_required** — the row has a current
  certified-archetype-report row that conforms to the same row schema
  with `report_family: certified_archetype_report` and a populated
  `archetype_matrix` block.
- **claim_manifest_row_required** — the row is referenced by at least
  one `claim_row_id` in
  `artifacts/governance/claim_manifest_seed.yaml` (or the published
  claim manifest that supersedes it).
- **design_partner_input_optional** — design-partner repos are
  admissible inputs but never required. A row may carry sanitised
  design-partner evidence in addition to its synthetic or live-slice
  workspaces.
- **none_required** — only admissible at experimental and on the
  exclusion row.

A row that loses any required item demotes automatically; see the
demotion rules below.

## Graduation path

A row may graduate by exactly one step per change. The rubric defines
four steps:

1. **`graduation:experimental_to_community`** — admit the row as
   community once at least one reviewable reference workspace exists.
   Approver: `lane:compatibility_ecosystem_review`, co-required with
   `lane:open_community_sync`.
2. **`graduation:community_to_supported`** — promote to first-party
   supported once the row carries a reference workspace, a corpus
   link, a current compatibility-report row, and a published
   claim-manifest row. Approver:
   `lane:compatibility_ecosystem_review`, co-required with
   `lane:benchmark_lab` and `lane:docs_public_truth`.
3. **`graduation:supported_to_certified`** — promote to certified once
   the row also carries a current certified-archetype-report row and
   a release-evidence-owned claim-manifest row. Approver:
   `lane:compatibility_ecosystem_review`, co-required with
   `lane:benchmark_lab`, `lane:release_evidence`, and
   `lane:shiproom_executive_scope_review`.
4. **`graduation:hold_at_current_class`** — record an explicit hold so
   the row does not appear silently stale while evidence work
   continues. Approver: `lane:compatibility_ecosystem_review`.

Direct promotion from `experimental` to `certified` in one change is
non-conforming. The intermediate steps are the program's mechanism for
making "we now support X" a reviewable transition rather than an
implicit launch-day surprise.

### Design-partner repos in the graduation path

A design-partner repo never enters the program in raw form. It enters
as a sanitised reference workspace once the partner has signed off on
the sanitisation profile, the workspace has a stable
`reference_workspace_id`, and the corpus manifest has a scenario row
for the partner-derived workflow. The
`design_partner_input_class_vocabulary` in
`artifacts/compat/reference_workspace_rows.yaml` records what each row
admits (`none_yet`, `sanitised_repo_admissible`,
`synthetic_only_until_partner_signed`, or
`existing_repo_slice_self_host`).

## Demotion rules

Demotion is automatic. When a trigger fires, the row's effective
class drops to the resulting class until the remediation owner clears
the trigger and runs the relevant graduation step again.

| Trigger | Applies to | Resulting class | Visible reason | Remediation owner |
|---|---|---|---|---|
| `reference_workspace_missing_or_unreviewable` | certified, supported, community | experimental | `required_evidence_missing` | `lane:compatibility_ecosystem_review` |
| `benchmark_corpus_evidence_missing_or_stale` | certified, supported | community | `required_evidence_narrower_than_claim` | `lane:benchmark_lab` |
| `compatibility_report_stale_or_blocking` | certified, supported | community | `compatibility_row_degraded` | `lane:release_evidence` |
| `certified_archetype_report_stale_or_blocking` | certified | supported | `certified_archetype_report_stale` | `lane:release_evidence` |
| `claim_manifest_row_missing_or_narrowed` | certified, supported | experimental | `claim_manifest_row_missing` | `lane:release_evidence` |
| `design_partner_signal_withdrawn` | community | experimental | `community_signal_withdrawn` | `lane:open_community_sync` |
| `support_window_freshness_expired` | certified, supported | community | `support_window_expired` | `lane:release_evidence` |
| `regression_severity_threshold_exceeded` | certified, supported | experimental | `blocking_regression_open` | `lane:performance_council` |
| `protected_metric_regression_unresolved` | certified, supported | experimental | `protected_metric_regression` | `lane:performance_council` |
| `upstream_ecosystem_breakage_unresolved` | certified, supported, community | experimental | `upstream_ecosystem_blocked` | `lane:compatibility_ecosystem_review` |

The full per-rule details, including which classes a rule applies to
and the visible reason the surface renders, live in
`artifacts/compat/archetype_rubric.yaml` under `demotion_rules`.

## Use across other artifacts

### Compatibility reports

Compatibility-report rows that reference an archetype use the shared
row schema at `schemas/release/compatibility_row.schema.json`,
quote `compat_row:certification.launch_archetype_matrix` as the
`row_id`, and cite the `archetype_row_id` in the report's notes or
`claimed_surface` so the report row resolves back to the inventory
without aliasing.

### Certified-archetype reports

Certified-archetype reports use the same row schema with
`report_family: certified_archetype_report` and a populated
`archetype_matrix` block. The `archetype_id` field on
`archetype_matrix` mirrors the inventory's `archetype_id`. The report
is the only artifact that admits `support_class: certified` on a row.

### Claim manifests

Claim-manifest rows that publish a top-level support claim include the
`archetype_row_id` in `certified_archetype_refs` and quote
`compat_row:certification.launch_archetype_matrix` in
`compatibility_row_refs`. The row is the published bridge between the
public claim and the inventory.

### Benchmark corpus and reference workspaces

Reference-workspace fixtures keep their own README and metadata; the
inventory cites their stable `reference_workspace_id`. Corpus-scenario
ids are the inventory's link to the protected benchmark corpus; the
corpus continues to govern its own change rules through
`docs/benchmarks/fixture_classes.md` and
`docs/benchmarks/corpus_governance.md`.

## Change discipline

- **Add a new row** when a new archetype enters scope. The row must
  carry an initial support class, an inclusion target, at least one
  reference-workspace ref or reservation, and a seed-notes file under
  `fixtures/compat/archetype_seed_notes/`.
- **Update an existing row** when its evidence, owner allocation, or
  matrix dimensions change. The change updates the inventory and the
  rubric in the same commit if the rubric's vocabulary needs new
  values.
- **Promote or demote** through the rubric's graduation paths and
  demotion rules. Both forms of transition are reviewable artifacts;
  silent promotion is non-conforming.
- **Exclude an archetype** by adding a row to
  `excluded_archetype_rows` with an `exclusion_reason` and an
  `inclusion_target`. Omitting the row is non-conforming; the
  inventory is meant to be complete.
