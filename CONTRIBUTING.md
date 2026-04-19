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
- [`docs/governance/provenance_and_compliance_baseline.md`](./docs/governance/provenance_and_compliance_baseline.md)
  — provenance, SBOM, and supply-chain expectations this guide enforces.
- [`artifacts/governance/compliance_checklist.yaml`](./artifacts/governance/compliance_checklist.yaml)
  — machine-readable checklist for new dependencies, third-party
  imports, and generated code.

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

## Third-party imports and dependencies

Aureline minimises external dependencies on protected paths. Every new
external dependency or vendored third-party file is a deliberate act
that must be reviewable in one place.

When you add or update a third-party dependency:

1. Open or extend the corresponding row in
   [`artifacts/governance/compliance_checklist.yaml`](./artifacts/governance/compliance_checklist.yaml)
   in the same pull request. Each row records origin, license,
   upstream version, local modifications (if any), and the named
   update owner.
2. Confirm the dependency's license is compatible with the importing
   crate's declared license. If it is not, the change is blocked
   until either the importing crate's license is reconsidered (ADR
   required) or a different dependency is selected.
3. Keep `Cargo.lock` truthful: do not regenerate it as a drive-by
   change. The lockfile is a pinned input to the determinism
   contract (see the reproducible-build baseline).
4. If the dependency triggers a notice update (for example a license
   that requires attribution in distributed binaries), update the
   pending notice register in the compliance checklist; the notice
   file itself lands once the first binary distribution channel is
   stood up.

Vendored source files (third-party code copied into the repository
rather than fetched as a Cargo dependency) follow the same rules and,
in addition, **MUST** preserve upstream copyright headers verbatim.
Local modifications are recorded in the checklist row, not by removing
the upstream header.

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

If the generator is itself an external tool, add a row in the
compliance checklist so the generator's origin and license are
reviewable alongside the generated artefacts.

Hand-edits to a file marked generated are not accepted; either
regenerate from source, or convert the file to hand-maintained in a
deliberate change that removes the "generated by" line.

## Notice updates

The repository does not yet ship a binary distribution, so there is no
`NOTICE` file today. The compliance checklist tracks every
attribution that will need to land in `NOTICE` (or its equivalent in
the eventual distribution format) before the first binary release. If
you add a dependency or vendored file whose license requires
attribution, mark its checklist row `notice_required: true` so the
release-engineering DRI sees it.

## Pull-request hygiene

- Every commit is signed off (DCO).
- Every new file carries an SPDX header (or a sibling `.license` file).
- Every new external dependency or vendored file has a checklist row
  in the same change.
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
