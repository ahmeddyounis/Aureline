# Contributing to Aureline

Thanks for your interest in contributing. This document is the entry
point for contributors and reviewers. It pairs with:

- [`AGENTS.md`](./AGENTS.md) and [`CLAUDE.md`](./CLAUDE.md) — guidance
  for coding agents working in this repository.
- [`CODEOWNERS`](./CODEOWNERS) — pull-request review routing.
- [`docs/governance/dri_map.md`](./docs/governance/dri_map.md) —
  ownership, blocker aging, and escalation.
- [`docs/governance/decision_workflow.md`](./docs/governance/decision_workflow.md)
  — how architecture decisions open, close, supersede, and narrow.
- [`docs/repo/topology.md`](./docs/repo/topology.md),
  [`docs/repo/dependency_rules.md`](./docs/repo/dependency_rules.md),
  and
  [`artifacts/governance/package_inventory.yaml`](./artifacts/governance/package_inventory.yaml)
  — the package map, layering rules, and protected-path inventory.
- [`docs/build/reproducible_build_baseline.md`](./docs/build/reproducible_build_baseline.md)
  and
  [`docs/build/exact_build_identity_model.md`](./docs/build/exact_build_identity_model.md)
  — developer setup, supported hosts, and the exact-build identity
  every build, packet, and bug report should quote.
- [`docs/governance/dogfood_issue_taxonomy.md`](./docs/governance/dogfood_issue_taxonomy.md)
  — category, severity, and evidence-link rules for dogfood and
  supportability issues.
- [`docs/benchmarks/benchmark_lab_run_results.md`](./docs/benchmarks/benchmark_lab_run_results.md)
  and
  [`docs/benchmarks/benchmark_publication_pack_template.md`](./docs/benchmarks/benchmark_publication_pack_template.md)
  — benchmark-lab and public-proof packet rules for claim-bearing
  performance work.
- [`docs/governance/provenance_and_compliance_baseline.md`](./docs/governance/provenance_and_compliance_baseline.md)
  — provenance, SBOM, and supply-chain expectations this guide enforces.
- [`docs/governance/dependency_review_policy.md`](./docs/governance/dependency_review_policy.md)
  — the admission policy for third-party dependencies, imported bytes,
  build-vs-buy linkage, and notice/SBOM/provenance flow.
- [`artifacts/governance/dependency_register.yaml`](./artifacts/governance/dependency_register.yaml)
  and
  [`artifacts/governance/third_party_import_register.yaml`](./artifacts/governance/third_party_import_register.yaml)
  — the canonical machine-readable homes for third-party dependency and
  import rows.

Aureline is in its pre-implementation stage. The contribution rules
below apply from the first source-bearing change so that compliance
debt does not accumulate silently.

## Quick start

1. Read [`AGENTS.md`](./AGENTS.md) and the relevant design documents in
   `.t2/docs/` (use targeted Grep/Read; the files are large).
2. Run the bootstrap once on a clean clone:

   ```sh
   ./tools/build/bootstrap.sh
   ```

3. Build the workspace:

   ```sh
   ./tools/build/build.sh
   ```

4. For changes that affect a protected lane or a public surface, open
   or extend the appropriate decision row before broad implementation
   (see [`decision_workflow.md`](./docs/governance/decision_workflow.md)).

The repository already contains a Cargo workspace, prototype crates,
schemas, fixtures, and governance artifacts. After bootstrap and build,
run the narrowest affected checks for your change. There is not yet a
single repo-wide lint/test wrapper covering every lane, so PRs **MUST**
state the exact commands that were run. If your change affects a
protected metric or a claim-bearing surface, capture the relevant
evidence or attach an approved waiver before requesting review.

## Read The Architecture First

Non-trivial changes start from the authoritative specs in `.t2/docs/`.
Use targeted search/read rather than loading those files wholesale.

Start with:

- `.t2/docs/Aureline_PRD.md` for requirement families, verification
  classes, supportability posture, and dogfood-ring expectations.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` for system
  boundaries, supportability contracts, repair flows, exact-build
  linkage, and route-truth vocabulary.
- `.t2/docs/Aureline_Technical_Design_Document.md` for component-level
  contracts and dependency-marker behavior.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` for support-center,
  recovery-ladder, repair-preview, and public-truth UX obligations.

Then use the repo-local contracts under `docs/`:

- [`docs/README.md`](./docs/README.md) for the checked-in docs map.
- [`docs/repo/topology.md`](./docs/repo/topology.md) and
  [`docs/repo/dependency_rules.md`](./docs/repo/dependency_rules.md)
  for package placement and allowed internal edges.
- [`artifacts/governance/package_inventory.yaml`](./artifacts/governance/package_inventory.yaml)
  for protected-path status and allowed internal dependencies.
- [`docs/product/boundary_manifest_strawman.md`](./docs/product/boundary_manifest_strawman.md)
  when a change affects the open-source core versus managed/service
  boundary.

## Development Setup And Workflow

The contributor baseline is defined by
[`docs/build/reproducible_build_baseline.md`](./docs/build/reproducible_build_baseline.md).
Do not invent alternate setup instructions in PR comments or issue
threads when the checked-in build docs can be updated instead.

Contributor expectations:

- Use the pinned toolchain from [`rust-toolchain.toml`](./rust-toolchain.toml).
- Treat `./tools/build/bootstrap.sh` and `./tools/build/build.sh` as the
  canonical setup and build entry points.
- If you change toolchain pins, build inputs, or artifact identity
  semantics, update the build baseline and exact-build identity docs in
  the same change.
- If you touch a benchmark, support bundle, crash artifact, docs/help
  surface, or claim packet, quote the exact-build identity instead of a
  loose version string.

## Package Boundaries And Protected Paths

The package map is a contract, not a suggestion.

- New crates, renamed crates, or new internal dependency edges **MUST**
  update:
  [`docs/repo/topology.md`](./docs/repo/topology.md),
  [`docs/repo/dependency_rules.md`](./docs/repo/dependency_rules.md),
  and
  [`artifacts/governance/package_inventory.yaml`](./artifacts/governance/package_inventory.yaml)
  in the same change.
- Production crates must not grow dependencies on spike or benchmark
  crates. `aureline-shell-spike` is disposable; `aureline-bench` is off
  the production cone.
- Protected-path crates and governance lanes need higher review
  discipline. If `protected_path: true` in the package inventory, assume
  that evidence, routing, and decision-record rules apply.
- Changes that introduce or widen a managed/service-plane dependency
  should update the boundary manifest so the dependency is declared,
  reviewable, and not left as a hidden product assumption.

## ADR, RFC, And Review Artifact Workflow

Use the decision workflow before broad implementation when a change:

- changes protected-lane-visible behavior;
- adds or changes a shared contract, schema, envelope, or stable public
  surface;
- changes package boundaries or lifecycle/dependency-marker semantics;
- narrows a committed protected path, supportability promise, or public
  claim.

Primary references:

- [`docs/adr/README.md`](./docs/adr/README.md)
- [`docs/rfc/README.md`](./docs/rfc/README.md)
- [`docs/governance/decision_workflow.md`](./docs/governance/decision_workflow.md)
- [`docs/governance/templates/verification_packet_template.md`](./docs/governance/templates/verification_packet_template.md)
- [`docs/governance/templates/waiver_template.md`](./docs/governance/templates/waiver_template.md)
- [`docs/governance/templates/freeze_exception_template.md`](./docs/governance/templates/freeze_exception_template.md)

Rules:

- ADRs are required before broad implementation of protected-lane or
  contract-shaping changes.
- RFCs are for changes that span multiple packages, teams, or design
  forums and need a written proposal before the ADR closes the decision.
- Do not quietly land architecture in code, schemas, or docs without the
  matching decision-row updates.
- If a change cannot yet meet the required evidence bar, attach a waiver
  or freeze exception with scope, risk, mitigation, owner, expiry, and
  planned exit. "We will follow up later" is not a waiver.

## Developer Certificate of Origin (DCO)

Every commit merged to this repository **MUST** carry a
`Signed-off-by:` trailer attesting to the
[Developer Certificate of Origin 1.1](https://developercertificate.org/).
A sign-off certifies that you wrote the change, or have the right to
submit it under the project's open-source license, and that you accept
the certificate's terms.

Add the trailer with `git commit -s` (or `git commit --signoff`):

```text
Signed-off-by: Random Developer <random@developer.example.org>
```

The full text of the certificate, reproduced for convenience:

> Developer Certificate of Origin
> Version 1.1
>
> Copyright (C) 2004, 2006 The Linux Foundation and its contributors.
>
> Everyone is permitted to copy and distribute verbatim copies of
> this license document, but changing it is not allowed.
>
>
> Developer's Certificate of Origin 1.1
>
> By making a contribution to this project, I certify that:
>
> (a) The contribution was created in whole or in part by me and I
>     have the right to submit it under the open source license
>     indicated in the file; or
>
> (b) The contribution is based upon previous work that, to the best
>     of my knowledge, is covered under an appropriate open source
>     license and I have the right under that license to submit that
>     work with modifications, whether created in whole or in part by
>     me, under the same open source license (unless I am permitted
>     to submit under a different license), as indicated in the file;
>     or
>
> (c) The contribution is provided directly to me by some other
>     person who certified (a), (b) or (c) and I have not modified
>     it.
>
> (d) I understand and agree that this project and the contribution
>     are public and that a record of the contribution (including all
>     personal information I submit with it, including my sign-off)
>     is maintained indefinitely and may be redistributed consistent
>     with this project and the open source license(s) involved.

Use a real name and a reachable email. Anonymous or pseudonymous
sign-offs are not accepted; aliases tied to a stable identity (for
example, GitHub `noreply` addresses) are accepted only when the
upstream identity is verifiable.

If you forget the trailer, amend the most recent commit with
`git commit --amend -s` (only safe before push) or, after push, add
fix-up commits each carrying their own sign-off and ask a maintainer
to merge with the sign-offs preserved. **Never** rewrite published
history to retro-add sign-offs without coordinating with the lane DRI
named in [`docs/governance/dri_map.md`](./docs/governance/dri_map.md).

CI enforces the sign-off rule on protected branches; pull requests
without sign-off on every commit are not merged.

## Per-file licensing hygiene (REUSE + SPDX)

Every maintained source file in the repository **MUST** declare its
licensing in a machine-readable way using SPDX identifiers, following
the [REUSE Specification](https://reuse.software/spec/) so that
copyright and licensing metadata are inspectable per file.

The two preferred forms are interchangeable:

1. An in-file header at the top of the file:

   ```rust
   // SPDX-FileCopyrightText: 2026 Aureline contributors
   // SPDX-License-Identifier: Apache-2.0
   ```

   ```sh
   # SPDX-FileCopyrightText: 2026 Aureline contributors
   # SPDX-License-Identifier: Apache-2.0
   ```

   ```toml
   # SPDX-FileCopyrightText: 2026 Aureline contributors
   # SPDX-License-Identifier: Apache-2.0
   ```

2. For files that cannot carry a comment (binaries, fixtures,
   golden traces, certain JSON or schema files), record the same
   metadata in a sibling `.license` file or in the repository's
   `.reuse/dep5` register once it is introduced.

Rules:

- The SPDX license identifier **MUST** be one of the licenses listed
  in the per-crate `Cargo.toml` (`license = "..."`) or in the
  documentation license declared by the `Docs / public truth` lane.
  Do not introduce a new identifier without an ADR.
- The copyright line **MUST** be `Aureline contributors` for original
  contributions. Vendored or imported third-party files keep their
  upstream copyright lines unchanged and add an `SPDX-FileCopyrightText`
  line for the importing project only when modifications warrant it.
- The crate manifest's `license` field is the source of truth for the
  default license of files inside that crate. A file whose SPDX header
  disagrees with its crate's `license` field is a hygiene defect and
  blocks merge.
- Generated files **MUST** carry the same SPDX header as a hand-written
  file, plus a one-line note identifying the generator and version
  (see "Generated code provenance" below).
- Documentation under `docs/` and `README.md` use the documentation
  license declared by the `Docs / public truth` lane. Until that
  declaration lands, documentation files inherit the workspace
  `Apache-2.0` identifier.

The release-evidence pack (see the provenance baseline) will eventually
include a REUSE-lint report. Land your changes REUSE-clean from day
one to keep the eventual gate cheap.

## Workspace license declaration

The Cargo workspace currently declares `license = "Apache-2.0"` in
[`Cargo.toml`](./Cargo.toml). That declaration is what every
workspace crate inherits via `license.workspace = true` and is what
the SPDX header above must match.

The eventual core/SDK/docs licensing model (per the recommendation in
the product requirements doc — MPL 2.0 for the core, Apache 2.0 for
SDKs and protocol definitions, a docs-friendly license for
documentation) is governed by an explicit decision record. Until that
ADR closes, the workspace `Apache-2.0` declaration stands. Do not
change a crate's `license` field as a drive-by edit.

## Public-interface versioning policy

Once a public surface is declared stable, it follows
[Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).
Today the workspace is at version `0.0.0` and **no public surface is
declared stable**. The list below names the surfaces the policy will
cover so that a change author can recognise when the policy applies to
them, even before any of these surfaces leaves experimental status.

Surfaces governed by SemVer:

| Surface                       | Versioning unit                                    | Notes                                                                                          |
|-------------------------------|----------------------------------------------------|------------------------------------------------------------------------------------------------|
| Schemas under `/schemas/`     | The `schema_version` integer in each schema.       | Backward-incompatible changes require a new `schema_version` and migration notes.              |
| CLI surfaces                  | Top-level command name plus its `--json` envelope. | Breaking changes to CLI flags or the JSON envelope are major-version bumps.                    |
| RPC envelopes (`aureline-rpc`)| Crate semver, plus an envelope `version` field.    | Wire-level envelope additions are minor bumps; renames or removals are major bumps.            |
| Manifests (workspace, package, governance) | Their `schema_version` field.         | Adding a field is a minor bump; renaming or removing a field is a major bump.                  |
| Saved artifacts (release evidence, support bundles, claim manifests, governance packets) | Their `schema_version` field.                   | Persistent on-disk forms must remain readable across one major version unless explicitly noted.|

Operating rules until any of the above is declared stable:

- Pre-stable changes are still recorded. Bumping a `schema_version`
  must update the corresponding schema under `/schemas/` and any
  consumer of that schema in the same change.
- Removing a field, renaming a field, or changing the meaning of a
  field requires either a `schema_version` bump or an ADR that
  explicitly waives the bump for the experimental surface.
- The decision to declare a surface stable is itself a decision row
  in [`artifacts/governance/decision_index.yaml`](./artifacts/governance/decision_index.yaml)
  closed by an ADR. No surface becomes stable by drift.

The same rules apply to public Rust API in any crate that is later
declared a public SDK. Until then, internal cross-crate APIs follow
the dependency rules in [`docs/repo/dependency_rules.md`](./docs/repo/dependency_rules.md).

## Filing Bugs, Dogfood Issues, And Supportability Defects

Use the routing table in
[`artifacts/governance/issue_routing.yaml`](./artifacts/governance/issue_routing.yaml)
and the intake rules in
[`docs/governance/dogfood_issue_taxonomy.md`](./docs/governance/dogfood_issue_taxonomy.md).

Routing defaults:

- `perf_regression` for measured protected-metric regressions.
- `supportability_issue` for blocked-user recovery, Doctor, bundle,
  route, or safe-mode failures.
- `docs_truth_defect` when public docs, known limits, help surfaces, or
  claim language disagree with product behavior.
- `design_system_defect` for accessibility and design-system defects when
  the issue is primarily on a UI contract rather than subsystem logic.
- `security_issue` for vulnerabilities or trust bugs that should stay on
  the private route.

Every non-trivial bug report should include, where applicable:

- the exact-build identity ref or the `build_identity.json` produced by
  the build you are using;
- OS, arch, workspace archetype, and workspace size class;
- enabled extensions or runtime hosts involved;
- whether the issue reproduces in safe mode, restricted mode, or
  headless mode;
- command, invocation-session, route, or target identifiers if the bug
  crossed an execution or transport boundary;
- evidence refs such as journey traces, benchmark runs, support-bundle
  manifests, crash IDs, Doctor finding codes, or checkpoint IDs;
- known-limit refs, dependency-marker refs, or compatibility/public-proof
  packet refs when the issue is really a truth-surface mismatch.

Dogfood reports that only say "it felt broken" are not actionable on
protected paths. Attach the best evidence you have and let the issue
state what remains unknown.

## Benchmark, Compatibility, And Public-Proof Expectations

Protected performance work and claim-bearing evidence follow the checked-
in benchmark/public-proof contracts:

- [`docs/benchmarks/fitness_function_catalog.md`](./docs/benchmarks/fitness_function_catalog.md)
- [`docs/benchmarks/benchmark_lab_run_results.md`](./docs/benchmarks/benchmark_lab_run_results.md)
- [`docs/benchmarks/benchmark_publication_pack_template.md`](./docs/benchmarks/benchmark_publication_pack_template.md)
- [`docs/release/release_evidence_packet_template.md`](./docs/release/release_evidence_packet_template.md)

Use the benchmark-lab entry point:

```sh
./tools/benchmark_lab.sh
```

Rules:

- Developer captures default to `run_context=self_capture`; they are for
  local comparison and debugging, not for release or public-proof claims.
- Claim-bearing benchmark or public-proof work should preserve the exact
  command line, corpus revision, comparability class, exact-build
  identity refs, docs/help version-match state, and known limits.
- Protected-metric regressions above the published threshold need either
  passing fresh evidence or an explicit approved waiver before merge.
- Public-facing benchmark, compatibility, or replacement-grade language
  must never get ahead of the checked-in packet/report state.

## Third-party imports and dependencies

Aureline minimises external dependencies on protected paths. Every new
external dependency or vendored third-party file is a deliberate act
that must be reviewable in one place.

When you add or update a third-party dependency:

1. Open or extend the corresponding row in
   [`artifacts/governance/dependency_register.yaml`](./artifacts/governance/dependency_register.yaml)
   in the same pull request.
2. Confirm the dependency's license is compatible with the importing
   crate's declared license. If it is not, the change is blocked
   until either the importing crate's license is reconsidered (ADR
   required) or a different dependency is selected.
3. Keep `Cargo.lock` truthful: do not regenerate it as a drive-by
   change. The lockfile is a pinned input to the determinism
   contract (see the reproducible-build baseline).
4. If the dependency touches a protected path, a local fork, or a
   mirrored/bundled asset source, add the build-vs-buy reference named
   by the dependency-review policy in the same row.
5. If the dependency changes attribution, SBOM, or provenance
   publication, update
   [`artifacts/governance/release_notice_seed.yaml`](./artifacts/governance/release_notice_seed.yaml)
   in the same change.

Vendored source files (third-party code copied into the repository
rather than fetched as a Cargo dependency) follow the same rules and,
in addition, **MUST** preserve upstream copyright headers verbatim.
Local modifications are recorded in
[`artifacts/governance/third_party_import_register.yaml`](./artifacts/governance/third_party_import_register.yaml),
not by removing the upstream header.

## Generated code provenance

Code that is generated rather than hand-written **MUST** carry both:

- the standard SPDX header (license + copyright); and
- a single-line annotation identifying the generator and the exact
  version used.

For example:

```rust
// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0
// Generated by aureline-codegen 0.1.2 from schemas/example.schema.json — do not edit by hand.
```

If the generator is itself an external tool or host runtime, add or
update its row in `dependency_register.yaml` so the origin, license
class, and provenance posture are reviewable alongside the generated
artefacts.

Hand-edits to a file marked generated are not accepted; either
regenerate from source, or convert the file to hand-maintained in a
deliberate change that removes the "generated by" line.

## Notice updates

The repository does not yet ship a binary distribution, so there is no
`NOTICE` file today. The canonical seed for future third-party
attribution is
[`artifacts/governance/release_notice_seed.yaml`](./artifacts/governance/release_notice_seed.yaml),
keyed by the stable dependency/import ids from the canonical registers.
If you add a dependency or imported asset whose release-notice class
requires publication, update the seed in the same change.

## Protected-Path Evidence And Waiver Discipline

Protected-path and claim-bearing changes need evidence, not optimism.

At minimum, a PR touching a protected path or truth surface should link
to the relevant evidence:

- build/setup changes: build baseline or exact-build identity updates;
- performance changes: benchmark-lab output or benchmark packet updates;
- recovery/supportability changes: Doctor findings, recovery-ladder
  drills, support-bundle or repair-preview evidence;
- compatibility or migration changes: updated compatibility report,
  known-limits note, or migration packet;
- docs/public-truth changes: docs-pack or claim-manifest parity updates.

When evidence is not yet green, attach a waiver instead of leaving the
reviewer to infer intent. Waivers must be time-bounded and should cite:

- the affected protected path or claim family;
- scope and current user impact;
- risk and mitigation;
- owner and expiry;
- the concrete exit path back to verified status.

Repeated waivers on the same protected path are a correction signal, not
business as usual.

## Pull-request hygiene

- Every commit is signed off (DCO).
- Every new file carries an SPDX header (or a sibling `.license` file).
- Every new external dependency or vendored file has a canonical
  dependency/import row
  in the same change.
- Every protected-path or claim-bearing change attaches fresh evidence or
  an approved waiver/freeze exception in the same change.
- Every change to a public-interface surface bumps the relevant
  `schema_version` (or attaches an ADR waiving the bump).
- Every change to a protected lane links to the appropriate decision
  row before broad implementation (see decision workflow).
- Commit messages and PR titles describe **what** changed and **why**;
  they do not surface internal milestone or task identifiers (see
  [`AGENTS.md`](./AGENTS.md) on planning metadata).

## Reporting security issues

Security reports follow the private route in
[`artifacts/governance/issue_routing.yaml`](./artifacts/governance/issue_routing.yaml)
under `private_security_channel`. Do **not** open a public issue for a
suspected vulnerability. The disclosure expectation for that route is
`public_on_advisory`: a public advisory is the eventual disclosure
surface, while the raw report stays private until the advisory lands.

## Questions

For governance, ownership, or scope questions, the lane DRI named in
[`dri_map.md`](./docs/governance/dri_map.md) is the first point of
contact. For routing questions about whether to open an issue, an RFC,
or an ADR, see [`docs/governance/decision_workflow.md`](./docs/governance/decision_workflow.md).
