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
- [`/artifacts/governance/compliance_checklist.yaml`](../../artifacts/governance/compliance_checklist.yaml)
  — machine-readable register of dependencies, vendored files,
  generators, and pending notice-file rows.
- [`/docs/build/reproducible_build_baseline.md`](../build/reproducible_build_baseline.md)
  — pinned toolchain, bootstrap command, and build-identity record
  that this baseline composes with.
- [`/ci/sbom_provenance.sh`](../../ci/sbom_provenance.sh) — placeholder
  CI command that the provenance lane will replace incrementally.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — private-security route for vulnerability reports.

## Goals

1. **Every merged commit is attributable.** A `Signed-off-by:` trailer
   on every commit certifies submission rights under the project's
   open-source license.
2. **Every maintained file declares its license.** REUSE-style SPDX
   metadata is present from the first source-bearing change.
3. **Every external input is reviewable in one place.** New
   dependencies, vendored files, and generators land with a row in the
   compliance checklist before merge.
4. **Every public surface follows a versioning policy.** Schemas, CLI
   surfaces, RPC envelopes, manifests, and saved artefacts have a
   declared SemVer-aligned policy that applies once they are declared
   stable.
5. **CI runs a placeholder provenance/SBOM step today.** The step is
   intentionally lightweight so the lane is wired and observable
   before the real generators are introduced.

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

- Every external Cargo dependency or vendored third-party file is
  represented by exactly one row in `compliance_checklist.yaml`.
- Each row records:
  - `id` — short stable identifier;
  - `kind` — `cargo_dependency`, `vendored_source`, `generator`, or
    `pending_notice`;
  - `name` and `version` — exact upstream identity;
  - `origin` — upstream URL or registry reference;
  - `license` — declared SPDX identifier;
  - `local_modifications` — `none` or a description;
  - `notice_required` — boolean; controls inclusion in the eventual
    `NOTICE` file;
  - `update_owner` — DRI handle from the ownership matrix;
  - `status` — `proposed`, `accepted`, `under_review`, or `retired`;
  - `notes` — free form, including any ADR or RFC references.
- The workspace currently has zero external Cargo dependencies, so
  the dependency portion of the register is intentionally empty. The
  generator section is seeded with the rustc / clippy / rustfmt
  toolchain pinned by `rust-toolchain.toml`, since the toolchain is
  the one external generator already present.
- Adding the first external Cargo dependency must update both this
  register and `Cargo.lock` in the same change.

## Generated code provenance

- Every generated file carries the standard SPDX header plus a
  single-line annotation identifying the generator name and version.
- Generators that are external tools also have a `generator` row in
  the compliance checklist so their origin and licence are tracked
  alongside other supply-chain inputs.
- Hand-edits to generated files are not accepted; either regenerate
  from source, or convert the file to hand-maintained in a deliberate
  change that removes the "generated by" annotation.

## Notice updates

- The repository does not yet ship a binary distribution, so there is
  no `NOTICE` file at this milestone.
- Every dependency or vendored file whose licence requires
  attribution sets `notice_required: true` in its checklist row. The
  release-engineering DRI consumes that flag when standing up the
  first distribution channel and producing the initial `NOTICE`
  contents.
- Removing a dependency that previously required notice does **not**
  retire its row; instead, the row's `status` moves to `retired` so
  the audit trail of "this attribution was once required" survives.

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
   the build identity, the toolchain pin, and the compliance
   checklist version it consumed.
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

## Compliance checklist as the single register

The compliance checklist (`compliance_checklist.yaml`) is the only
canonical home for new dependencies, vendored files, generators, and
pending notice rows. Adding a parallel register in another part of
the repository is a governance error.

Change discipline:

- Adding a row requires the same fields listed under "Third-party
  imports and dependency review" above. Missing fields are validation
  failures.
- Retiring a row sets its `status` to `retired` and leaves the row in
  place with a closing note in `notes`. Rows are not deleted.
- When this document and the YAML checklist disagree, the YAML is
  authoritative for tooling and this document must be updated in the
  same change.

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
3. Update this document and the compliance checklist in the same
   change so the baseline and the registers never disagree.
