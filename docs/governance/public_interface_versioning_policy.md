# Public-interface versioning and deprecation policy

This page is the first M1-bearing public-interface versioning and
deprecation policy. It is the canonical artifact for the
`public_interface_versioning` row of the contribution-governance seed at
[`/artifacts/governance/contribution_governance_seed.yaml`](../../artifacts/governance/contribution_governance_seed.yaml).
The reviewer-facing landing page for the seed is
[`./contribution_and_signoff.md`](./contribution_and_signoff.md).

The aim is narrow: declare which surfaces are versioned units, how
breaking changes are minted, and how a deprecation is reviewable. The
policy is intentionally conservative until any individual surface is
declared stable, because no public surface in this repository is yet
stable.

## Versioned surfaces

Once a public surface is declared stable, it follows
[Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html). The
surfaces below are the versioning units this policy governs. Each
surface either carries an integer `schema_version` or rides the crate's
semver, never both.

| Surface | Versioning unit | Notes |
|---|---|---|
| Schemas under `/schemas/` | The `schema_version` integer in each schema. | Backward-incompatible changes require a new `schema_version` and a deprecation packet (see below). |
| Manifests (workspace, package, governance) | Their `schema_version` field. | Adding an optional field is a minor bump; renaming or removing a field is a major bump. |
| Saved artifacts (release-evidence, support-bundle, claim-manifest, governance packets) | Their `schema_version` field. | Persistent on-disk forms remain readable across one major version unless explicitly noted in the deprecation packet. |
| CLI surfaces | Top-level command name plus its `--json` envelope. | Breaking changes to CLI flags or the `--json` envelope are major-version bumps. |
| RPC envelopes (`aureline-rpc`) | Crate semver plus an envelope `version` field. | Wire-level envelope additions are minor bumps; renames or removals are major bumps. |
| Command IDs (palette + scripted invocation) | The stable command id string. | Renames are major bumps and require a deprecation packet that names the redirect. |
| Public Rust API in SDK crates | Crate semver. | Until a crate is declared an SDK, internal cross-crate APIs follow [`/docs/repo/dependency_rules.md`](../repo/dependency_rules.md). |

## Pre-stable operating rules

Until any of the above is declared stable, the workspace is at
version `0.0.0` and the rules below apply. They keep the eventual
stabilisation gate cheap to clear.

- Pre-stable changes are still recorded. Bumping a `schema_version`
  MUST update the corresponding schema under `/schemas/` and every
  consumer of that schema in the same change.
- Adding a field is a minor bump and never requires a deprecation
  packet.
- Removing a field, renaming a field, or changing the meaning of a
  field MUST either (a) bump `schema_version` and land a deprecation
  packet, or (b) carry an ADR that explicitly waives the bump for the
  experimental surface.
- The decision to declare a surface stable is itself a decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  closed by an ADR. No surface becomes stable by drift.

## Deprecation packet shape

A deprecation packet is the one canonical packet format every schema,
command, lifecycle, or interface deprecation reuses instead of inventing
ad hoc release notes. The template lives at
[`./deprecation_packet_template.md`](./deprecation_packet_template.md).

A deprecation packet MUST declare:

- the affected surface and its versioning unit (e.g.
  `schemas/example.schema.json#schema_version`);
- the deprecated identifier (field path, command id, RPC envelope name,
  CLI flag, or crate item);
- the version the deprecation was introduced in (`deprecated_since`);
- the version the deprecated identifier will stop being writable in
  (`writer_sunset`);
- the version readers will stop accepting the deprecated identifier in
  (`reader_sunset`);
- a typed `downgrade_action` in
  `{drop_field_on_read, preserve_as_unknown, refuse_read, refuse_export}`
  that names how legacy bytes are handled during the deprecation window;
- the replacement identifier (if any) so consumers can migrate
  mechanically;
- the named runtime consumer (docs page, CI gate, support surface) that
  will read the deprecation packet to decide whether to honour the
  deprecation.

The deprecation window is at least one LTS cycle for writers and at
least one further LTS cycle for readers, unless the affected surface is
experimental and the corresponding ADR explicitly waives the window.

## Stabilisation prerequisites

Declaring a surface stable requires, at minimum:

- the surface's schema, manifest, or crate semver baseline checked in
  under `/schemas/` or the relevant `Cargo.toml`;
- a row in [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  whose ADR closes the stabilisation decision;
- a row in the contribution-governance seed pointing at this policy as
  the canonical artifact (the `public_interface_versioning` row);
- a deprecation packet template ready for the first deprecation event
  (see [`./deprecation_packet_template.md`](./deprecation_packet_template.md));
- a release-evidence pack ready to record the stabilisation milestone.

## How this policy is enforced

The contribution-governance seed's validation lane at
[`/tests/governance/m1_contribution_governance_seed_lane/`](../../tests/governance/m1_contribution_governance_seed_lane/)
re-parses the seed and asserts:

- the `public_interface_versioning` row's `canonical_artifact_ref`
  points at this file;
- this file contains the canonical-artifact-marker the row declares
  (so the seed cannot quietly point at the wrong document);
- this file is listed in the seed's named runtime consumer or supporting
  artifacts so it is reachable from the seed.

If this policy and the contribution-governance seed disagree, the seed
wins and this page MUST be updated in the same change.
