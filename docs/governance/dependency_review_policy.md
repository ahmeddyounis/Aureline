# Dependency review policy

This document is the lightweight operating policy for admitting new
third-party dependencies and imported bytes into Aureline. It sits on
top of the provenance and compliance baseline and turns the architecture
doc's build-vs-buy posture into a repository-local review workflow.

Companion artifacts:

- [`/artifacts/governance/build_vs_buy_register.yaml`](../../artifacts/governance/build_vs_buy_register.yaml)
  — canonical machine-readable build-vs-buy domain register and scoring
  rubric.
- [`/artifacts/governance/dependency_register.yaml`](../../artifacts/governance/dependency_register.yaml)
  — canonical register of selected and admitted third-party
  dependencies.
- [`/artifacts/governance/third_party_import_register.yaml`](../../artifacts/governance/third_party_import_register.yaml)
  — canonical register of copied, bundled, or mirrored third-party
  bytes.
- [`/docs/governance/fork_review_policy.md`](./fork_review_policy.md)
  — required review bar for protected-path forks and long-lived patch
  stacks.
- [`/artifacts/governance/release_notice_seed.yaml`](../../artifacts/governance/release_notice_seed.yaml)
  — third-party attribution seed keyed by the stable ids from the two
  canonical registers above.
- [`/artifacts/governance/compliance_checklist.yaml`](../../artifacts/governance/compliance_checklist.yaml)
  — bridge artifact and sweep ledger; it no longer owns dependency
  rows directly.
- [`/docs/governance/maintainer_coverage_policy.md`](./maintainer_coverage_policy.md)
  — protected-path reviewer-depth and backup-owner policy that also
  links critical dependencies to maintainer durability and release
  quorum rules.
- [`/artifacts/governance/upstream_health_scorecard.yaml`](../../artifacts/governance/upstream_health_scorecard.yaml)
  — canonical health-score or provisional-risk register keyed by
  dependency row id for critical upstreams.
- [`/docs/governance/dependency_automation_and_release_notice.md`](./dependency_automation_and_release_notice.md)
  — automation contract for upstream ingest and release-notice generation that
  keeps stable ids, evidence markers, and human review checkpoints aligned.
- [`/docs/governance/provenance_and_compliance_baseline.md`](./provenance_and_compliance_baseline.md)
  — the broader DCO, REUSE/SPDX, and placeholder SBOM/provenance
  baseline this policy composes with.
- [`/docs/governance/provenance_badge_contract.md`](./provenance_badge_contract.md)
  — shared badge and row vocabulary dependency/import review surfaces
  use when they project provenance, license, notice, upstream health,
  support, freshness, advisory, and mirror/offline state.

## 1. One dependency row, one import row, one source id

Supply-chain review splits cleanly:

- `dependency_register.yaml` records upstream choices and operational
  fragility: owner, license class, provenance status, health status,
  criticality, update cadence, fork or replace trigger, release-notice
  class, and automation posture.
- `third_party_import_register.yaml` records copied or mirrored bytes:
  origin, local-path home, local modifications, provenance, and notice
  / SBOM flow.
- `release_notice_seed.yaml` does not invent its own ids. Every row
  keys off `source_id` from one of the canonical registers above.

If the same upstream appears in multiple files, the dependency row is
still the anchor. Imports point back to it with `source_dependency_id`
when applicable.

## 2. When a row is required

Add or extend a dependency-register row in the same change when a pull
request:

1. selects a third-party dependency for a protected path, even before
   the dependency lands in `Cargo.toml`;
2. adds a third-party crate, CLI tool, host runtime requirement, build
   tool, or generator runtime to repository workflows; or
3. changes the owner, criticality, release-notice class, fork trigger,
   or automation posture of an existing third-party dependency.

Add or extend an import-register row in the same change when a pull
request:

1. copies, bundles, or mirrors third-party source, fonts, docs packs,
   binary assets, or generated metadata;
2. changes the imported bytes, their local-path home, or their local
   modifications; or
3. changes how imported bytes flow into notices, SBOMs, pack manifests,
   or provenance statements.

If no third-party bytes are copied or mirrored, do not mint an import
row just because a dependency exists upstream.

## 3. When build-vs-buy linkage is mandatory

A dependency row MUST link to an existing build-vs-buy source when any
of the following are true:

- the dependency is on a protected path;
- the dependency is the chosen default for a user-visible product
  capability rather than a repo-only tool;
- the change proposes a local fork, long-lived patch stack, or mirrored
  upstream pack; or
- replacing the dependency would change the architecture doc's stated
  posture for that domain.

The link may point at an ADR, a structured tradeoff-row register, or
the canonical build-vs-buy register in
`artifacts/governance/build_vs_buy_register.yaml`. The originating ADR
or the build-vs-buy tables in
`.t2/docs/Aureline_Technical_Architecture_Document.md` may be cited as
supporting context when needed. Do not mint ad-hoc build-vs-buy ids
outside the canonical register, ADR set, or architecture sources.

Repo tooling and host runtimes that exist only to build or verify the
repository do not require a build-vs-buy row. Those rows instead cite
their selection basis in the reproducible-build baseline or the
benchmark/journey harness docs.

## 4. Minimum review fields

Every dependency row MUST declare:

- a named `owner_dri`;
- at least one owning package or lane;
- `license_class`;
- `provenance_status`;
- `health_status`;
- `criticality`;
- `update_cadence_class`;
- `fork_or_replace_trigger`;
- `release_notice_class`; and
- `automation_refresh` with `mode`, `evidence_sources`,
  `machine_refresh_fields`, `manual_fields`, and a stale-threshold
  profile.

Every import row MUST additionally declare:

- its `import_kind`;
- `local_path_ref` (or `not_yet_seeded` until the home exists);
- `local_modifications`; and
- the `source_dependency_id` when the import is derived from a specific
  dependency row.

Rows missing these fields are review failures even if the code change is
otherwise correct.

## 5. Critical-upstream scorecard coverage

Every dependency row with `criticality` in one of these classes:

- `protected_path_release_critical`
- `release_engineering_critical`
- `benchmark_lab_required`
- `repo_operations_required`

MUST have a matching row in
[`/artifacts/governance/upstream_health_scorecard.yaml`](../../artifacts/governance/upstream_health_scorecard.yaml).

Minimum scorecard content:

- matching `dependency_id`;
- review cadence;
- either an explicit health score or a provisional risk class;
- one note for each health dimension (activity, responsiveness,
  license, bus factor, security posture, replacement feasibility);
- escalation triggers; and
- a named sponsor or owner for follow-up.

Rules:

- protected-path rows may begin with a provisional risk class while the
  first upstream review is still being assembled, but the row may not be
  absent;
- manifest admission or stable-claim work on a protected path may not
  pretend the upstream is healthy if the scorecard still carries
  provisional risk class `high` or `blocked`;
- changing a row's `criticality`, fork trigger, or sustainment posture
  without updating the scorecard is a review failure; and
- the scorecard is where supply-chain fragility becomes release and
  maintainer risk instead of remaining a comment in a dependency row.

## 6. Manual versus machine-refreshed fields

The registers intentionally separate human judgment from machine ingest.

- Manual fields are policy and ownership facts: owner, criticality,
  build-vs-buy linkage, provenance posture, fork trigger, and notice
  class.
- Machine-refreshed fields are evidence facts: observed tool/runtime
  version, manifest admission, release tag, source digest, mirror
  revision, or other upstream-health probes.

Automation MAY refresh only the fields each row names under
`machine_refresh_fields`. Everything else is reviewer-owned. A bot that
changes a manual field without an explicit human review is non-
conforming.

## 7. Stale-row thresholds

The seeded stale profiles are the minimum floor:

- `protected_path_selected` — manual review every 30 days while the
  dependency is selected but not yet manifested.
- `admitted_repo_tooling` — machine refresh every 14 days and manual
  review every 90 days for pinned build/release tooling.
- `required_host_runtime` — machine observation every 30 days and
  manual review every 120 days for unpinned host runtimes such as
  `bash`, `git`, or `python3`.
- `reserved_import` — manual review every 30 days while a copied or
  mirrored third-party asset is reserved but not yet imported.
- `imported_release_asset` — machine refresh every 14 days and manual
  review every 90 days once imported bytes are actually shipped.

These thresholds are seeds, not ceilings. A protected-path dependency
with repeated instability may tighten its cadence in the same row.

## 8. Notice, SBOM, and provenance flow

The third-party publication path is:

1. The dependency or import row carries the canonical source id.
2. `release_notice_seed.yaml` maps that source id to publication
   targets (`third_party_notice`, `spdx_sbom`, `cyclonedx_sbom`,
   `provenance_statement`, `docs_pack_manifest`) and a render gate.
3. The eventual generators read the source row directly. They do not
   maintain hand-written copy tables with separate ids.

Practical rules:

- `release_notice_class = build_input_only_no_notice` still requires
  provenance or build-input SBOM coverage; it only suppresses a binary
  notice row.
- Host runtimes that are not redistributed (`bash`, `git`, `python3`,
  `rustup`) are provenance-only unless a later distribution model
  embeds them.
- Imported assets and mirrored packs flow through the import register
  and may publish through non-binary targets such as `docs_pack_manifest`.

## 9. Local forks and mirrors

Local forks, long-lived upstream patch stacks, and mirrored upstream
packs raise the review bar:

- the dependency or import row MUST name why upstreaming is not enough;
- the relevant build-vs-buy row MUST still name the exit or fork
  posture for that domain;
- protected-path forks MUST satisfy
  `docs/governance/fork_review_policy.md`, including a named owner,
  divergence review cadence, and re-upstream plan;
- the row MUST name an exit path, not just a fork reason; and
- the row MUST tighten its review cadence to at least `per_release`
  until the divergence is gone or formally ratified.

If a fork remains divergent across more than one release family, open a
fresh architecture review instead of quietly carrying the patch stack
forward.

## 10. Relationship to the compliance checklist

`compliance_checklist.yaml` remains in the repository, but its role is
now narrow:

- it is the bridge artifact that points tooling and reviewers to the
  canonical registers above; and
- it remains the sweep ledger for repo-wide compliance chores such as
  REUSE/SPDX backfills.

It is no longer the canonical home for dependency, vendored-source, or
pending-notice rows. Adding a new dependency there without a canonical
row in `dependency_register.yaml` or `third_party_import_register.yaml`
is a governance error.
