# Provenance and compliance baseline

This document is the authoritative description of what the repository
expects from contributors and CI for IP, provenance, and supply-chain
hygiene. It exists so that the project is launch-safe from day one
without committing to release-grade attestation infrastructure that is
not yet warranted.

It is intentionally narrow: the goal is a baseline that lets later
release-engineering work add real signing, SBOM emission, and
provenance attestations on top of a stable foundation. Full
SLSA-compliant signed builds are out of scope for this milestone.

Companion artifacts:

- [`/CONTRIBUTING.md`](../../CONTRIBUTING.md) — contributor-facing
  rules (DCO, REUSE/SPDX, third-party imports, generated code).
- [`/docs/governance/dependency_review_policy.md`](./dependency_review_policy.md)
  — lightweight admission policy for third-party dependencies,
  imported bytes, build-vs-buy linkage, automation posture, and
  notice/SBOM/provenance flow.
- [`/artifacts/governance/dependency_register.yaml`](../../artifacts/governance/dependency_register.yaml)
  — canonical register of selected and admitted third-party
  dependencies.
- [`/artifacts/governance/third_party_import_register.yaml`](../../artifacts/governance/third_party_import_register.yaml)
  — canonical register of copied, bundled, or mirrored third-party
  bytes.
- [`/artifacts/governance/release_notice_seed.yaml`](../../artifacts/governance/release_notice_seed.yaml)
  — third-party attribution seed keyed by stable dependency/import
  ids.
- [`/artifacts/governance/compliance_checklist.yaml`](../../artifacts/governance/compliance_checklist.yaml)
  — bridge artifact and sweep ledger that points tooling at the
  canonical registers above.
- [`/docs/build/reproducible_build_baseline.md`](../build/reproducible_build_baseline.md)
  — pinned toolchain, bootstrap command, and build-identity record
  that this baseline composes with.
- [`/docs/build/cleanroom_rebuild_lane.md`](../build/cleanroom_rebuild_lane.md)
  — first clean-room rebuild lane that records mirrors, trust
  assumptions, digest comparisons, and provenance-capture outputs
  explicitly.
- [`/ci/sbom_provenance.sh`](../../ci/sbom_provenance.sh) — placeholder
  CI command that the provenance lane will replace incrementally.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — private-security route for vulnerability reports.
- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  — boundary manifest that every release-evidence claim manifest
  cites. Release-grade SBOMs, attestations, and claim manifests
  compose over the boundary rows; the claim manifest never
  re-derives the boundary and never contradicts it.

## Goals

1. **Every merged commit is attributable.** A `Signed-off-by:` trailer
   on every commit certifies submission rights under the project's
   open-source license.
2. **Every maintained file declares its license.** REUSE-style SPDX
   metadata is present from the first source-bearing change.
3. **Every external input is reviewable in one canonical register.**
   New dependencies and imported bytes land in the dedicated
   dependency/import registers before merge; the compliance checklist
   remains only the bridge and sweep ledger.
4. **Every public surface follows a versioning policy.** Schemas, CLI
   surfaces, RPC envelopes, manifests, and saved artefacts have a
   declared SemVer-aligned policy that applies once they are declared
   stable.
5. **CI runs a placeholder provenance/SBOM step today.** The step is
   intentionally lightweight so the lane is wired and observable
   before the real generators are introduced.
6. **The clean-room lane records its assumptions instead of hiding
   them.** Mirror inputs, signing gaps, and known reproducibility
   limitations are emitted as first-class artifacts, not left implicit
   in CI scripts.

## DCO sign-off baseline

- Contribution rule: every commit on the merge target carries a
  `Signed-off-by:` trailer (see `CONTRIBUTING.md`).
- Enforcement: the placeholder CI lane verifies that every commit on
  the pull request has a sign-off trailer with a non-empty name and
  email. Replacing the placeholder with a richer DCO bot or a
  branch-protection rule is a later release-engineering change.
- Failure mode: a missing or malformed sign-off blocks merge; it does
  not break the workspace build. Contributors fix forward with
  amended commits or fix-up commits, never by rewriting published
  history without coordination with the lane DRI.
- Out of scope here: contributor licence agreements (CLAs), notarised
  identity, or paid-publisher verification programs. The project
  uses DCO 1.1 by default.

## Per-file license metadata

- Convention: REUSE Specification with SPDX identifiers, applied
  per-file via in-file headers or sibling `.license` files.
- Default identifier: `Apache-2.0`, matching the workspace declaration
  in `Cargo.toml`. Any file using a different identifier must be
  inside a crate or directory whose declared license matches that
  identifier, and the discrepancy must be justified by an ADR.
- Coverage target for this milestone: 100 % of source-bearing files
  added or modified after this baseline lands. Pre-existing files are
  brought into compliance opportunistically; the compliance checklist
  records any deferred sweep.
- Tooling target: a future `tools/governance/reuse_lint.sh` will run
  the official REUSE linter and emit a report for the release evidence
  pack. Today the placeholder CI lane only checks that any *newly
  added* file under `crates/`, `tools/`, `ci/`, `schemas/`, or
  `docs/` carries an `SPDX-License-Identifier` line. Stronger
  enforcement lands with the real lint integration.

## Third-party imports and dependency review

- Third-party dependency rows live in
  `dependency_register.yaml`. Copied, bundled, or mirrored bytes live
  in `third_party_import_register.yaml`.
- The dependency register records the upstream choice and its
  fragility posture: owner, owning scope, license class, provenance
  status, health status, criticality, update cadence, build-vs-buy
  linkage when required, fork-or-replace trigger, release-notice
  class, and automation-refresh posture.
- The import register records the byte-level state: copied or mirrored
  source, local-path home, local modifications, provenance, and the
  publication targets that will eventually emit notices, SBOM entries,
  or provenance statements.
- The workspace still has zero external Cargo dependencies admitted in
  `Cargo.toml`; the seeded dependency rows are a mix of repo tooling
  already required today and protected-path choices selected by ADR
  but not yet manifested.
- Adding the first external Cargo dependency still requires a truthful
  `Cargo.lock` update in the same change.

## Generated code provenance

- Every generated file carries the standard SPDX header plus a
  single-line annotation identifying the generator name and version.
- External generators or generator runtimes live in
  `dependency_register.yaml` so their origin, license class, and
  provenance posture are tracked alongside other supply-chain inputs.
- Hand-edits to generated files are not accepted; either regenerate
  from source, or convert the file to hand-maintained in a deliberate
  change that removes the "generated by" annotation.

## Notice updates

- The repository does not yet ship a binary distribution, so there is
  no `NOTICE` file at this milestone.
- `release_notice_seed.yaml` is the canonical seed for third-party
  attribution. It keys off the stable dependency/import ids rather than
  a separate notice-id system.
- A dependency or import row that carries a notice-bearing
  `release_notice_class` must update `release_notice_seed.yaml` in the
  same change that introduces or changes the source row.
- Retiring a dependency or import does not justify deleting its source
  row; the audit trail of "this attribution once applied" survives in
  the canonical register and in the release-notice seed.

## SBOM and provenance commands

The repository ships a placeholder script that CI invokes today. Its
job is to make the lane observable and to fail loudly if the inputs
the real generators will need are missing — it does **not** itself
emit a release-grade SBOM or provenance attestation.

```sh
./ci/sbom_provenance.sh
```

The placeholder:

1. Resolves the build identity by invoking
   `tools/build/print_build_identity.sh`. The build identity is the
   anchor that real SBOM and provenance documents will reference.
2. Emits a deterministic, minimal SBOM stub describing the workspace
   crates declared in `Cargo.toml` to
   `target/ci-artifacts/sbom_workspace.json`. The stub deliberately
   does **not** claim SPDX or CycloneDX conformance; it is a
   structural placeholder that downstream lanes can extend.
3. Records the placeholder provenance summary to
   `target/ci-artifacts/provenance_summary.json`. The summary names
   the build identity, the toolchain pin, and the canonical
   dependency/import register revisions it consumed.
4. Exits zero on success. Failures of the underlying scripts surface
   as CI failures; the script does not swallow errors.

The replacement plan (out of scope here, named so the home is
reserved):

- Real SPDX SBOM emission (workspace + transitive Cargo graph) lands
  via a future `tools/governance/spdx_sbom.sh` and replaces the stub.
- CycloneDX export is added alongside SPDX once a security-tooling
  consumer requires it.
- in-toto / SLSA-style provenance attestation lands once the release
  signing infrastructure exists. The placeholder summary file is the
  surface that the real attestation will replace.

CI invokes the placeholder via `ci/build.sh` (or its successor) so
that flipping the script over to the real generator is a single edit
to one file rather than a CI-config archaeology project.

## Public-interface versioning policy

The contributor-facing form of this policy lives in
`CONTRIBUTING.md`. The governance form below is the rulebook tooling
will eventually validate against:

- **Surfaces in scope.** Schemas under `/schemas/`, CLI commands plus
  their `--json` envelopes, RPC envelopes provided by `aureline-rpc`,
  on-disk manifests (workspace, package, governance), and saved
  artefacts (release evidence, support bundles, claim manifests,
  governance packets).
- **SemVer alignment.** Once a surface is declared stable, it follows
  Semantic Versioning 2.0.0. Adding a field is a minor bump; renaming
  or removing a field is a major bump; meaning-changing edits are
  major bumps regardless of textual diff size.
- **Schema-version anchors.** Surfaces with a `schema_version`
  integer use that integer as the wire-level versioning anchor. The
  schema file under `/schemas/` is the source of truth; tooling and
  documentation reference the schema file rather than restating
  field-level rules.
- **Stable declaration is itself a decision.** No surface becomes
  stable by drift. A surface graduates from experimental to stable
  via a decision row in the decision register, closed by an ADR.
- **Pre-stable discipline.** Even before a surface is stable, removing
  or renaming a field requires either a `schema_version` bump or an
  ADR that explicitly waives the bump. This keeps the eventual
  graduation cheap because no churn slips through unrecorded.
- **Cross-references.** The compatibility-report packet family in
  `governance_packet_template.yaml` will instantiate per release with
  the diff between two adjacent versions of each stable surface.

## Compliance checklist as bridge and sweep ledger

The compliance checklist (`compliance_checklist.yaml`) is no longer the
canonical home for dependency or import rows. Its role is narrow:

- it points reviewers and tooling at
  `dependency_register.yaml`, `third_party_import_register.yaml`, and
  `release_notice_seed.yaml`; and
- it records repository-wide compliance sweeps that are not themselves
  third-party dependency or import rows.

Change discipline:

- Adding a third-party dependency or import row requires updating the
  canonical register plus this bridge artifact in the same change.
- Retiring a sweep entry or closing a deferred sweep still happens
  here; those entries are not dependency rows and do not move into the
  canonical registers.
- When this document and the YAML artifacts disagree, the YAML
  artifacts are authoritative for tooling and this document must be
  updated in the same change.

## Solo-maintainer posture

Under the current solo-maintainer posture, every DRI handle resolves
to the sole maintainer (see `dri_map.md`). The single-maintainer
backup waiver covers the absence of a second named owner on the
provenance lane. The waiver expiry, escalation path, and reason are
recorded in the ownership matrix; the provenance lane closes its
backup-owner gap on the same milestone the waiver closes.

## What this baseline is not

- It is **not** a release-evidence pack. The release-evidence packet
  family lives under `artifacts/release/` and is governed by
  `release_evidence` in the control-artifact index.
- It is **not** a security-response policy. Security reports follow
  `private_security_channel` in `issue_routing.yaml`; the response
  policy is a later artifact.
- It is **not** a CLA. The project uses DCO 1.1 by default.
- It is **not** a substitute for ADRs. Every change that crosses into
  a protected lane's public surface still goes through the decision
  workflow, and licence-model decisions are themselves ADR-bound.

## Upgrade discipline

Bumping any of the rules above (replacing the placeholder SBOM script
with a real generator, declaring the first stable surface, switching
the workspace licence) shares the same discipline:

1. Open or extend a decision row in
   [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
2. Land an ADR that closes it.
3. Update this document, the dependency review policy, and whichever of
   the canonical dependency/import/notice registers or compliance
   bridge changed in the same commit so the baseline and the registers
   never disagree.
