# Deprecation packet template

This page is the canonical deprecation packet template for any schema,
command, lifecycle, or interface deprecation. It is the canonical
artifact for the `deprecation_packet_template` row of the
contribution-governance seed at
[`/artifacts/governance/contribution_governance_seed.yaml`](../../artifacts/governance/contribution_governance_seed.yaml).
The reviewer-facing landing page for the seed is
[`./contribution_and_signoff.md`](./contribution_and_signoff.md), and the
versioning rules a deprecation event lives inside are at
[`./public_interface_versioning_policy.md`](./public_interface_versioning_policy.md).

The template is short on purpose. Future governed work reuses it
instead of inventing ad hoc release notes when a field, command,
manifest entry, or interface is being deprecated.

## When to use this template

Open a deprecation packet when:

- a schema bumps `schema_version` because a field is being renamed,
  retyped, or removed;
- a CLI flag, command id, or `--json` envelope field is being renamed
  or retired;
- an RPC envelope adds, renames, or removes a field that crosses the
  wire;
- a saved-artifact shape (release-evidence pack, support-bundle
  manifest, claim-manifest, governance packet) changes in a way that
  breaks readers; or
- a public Rust API in an SDK crate is being deprecated.

Do not open a deprecation packet for additive changes (a new optional
field, a new optional flag, a new command id). Those are minor bumps
and live in normal release notes.

## Required sections

A deprecation packet MUST contain the following sections, in this order.
The validation lane for the contribution-governance seed re-parses
this file and asserts every required section heading is present, so
the template cannot quietly lose a section.

### 1. Summary

One paragraph. Names the affected surface, the deprecated identifier,
the version it was introduced in, the planned writer-sunset and
reader-sunset versions, and the replacement (if any).

### 2. Affected surface

Names the versioning unit the deprecation lives inside, taken from
[`./public_interface_versioning_policy.md`](./public_interface_versioning_policy.md):

- `schemas/<path>.schema.json#schema_version` for a schema field;
- `commands.<command_id>` for a command id;
- `rpc.<envelope_name>.<field_path>` for an RPC envelope field;
- `cli.<command_name>.<flag_or_envelope_field>` for a CLI surface;
- `manifests.<manifest_name>#<field_path>` for a manifest;
- `artifacts.<artifact_kind>#<field_path>` for a saved artifact;
- `crate.<crate_name>.<item_path>` for a Rust SDK item.

### 3. Deprecation window

Names the integer versions involved:

- `deprecated_since`: the version the deprecation was introduced in;
- `writer_sunset`: the version the deprecated identifier stops being
  writable in;
- `reader_sunset`: the version readers stop accepting the deprecated
  identifier in.

The default deprecation window is at least one LTS cycle for writers
and at least one further LTS cycle for readers. A shorter window
requires an ADR that names the affected surface and the reason the
window is being shortened.

### 4. Downgrade action

Names a typed `downgrade_action` in
`{drop_field_on_read, preserve_as_unknown, refuse_read, refuse_export}`
that describes how legacy bytes are handled during the deprecation
window. The four values map to:

- `drop_field_on_read` — readers silently drop the deprecated field.
- `preserve_as_unknown` — readers keep the deprecated field as opaque
  unknown bytes so historical artifacts remain legible.
- `refuse_read` — readers refuse the deprecated identifier and surface
  an actionable error.
- `refuse_export` — writers refuse to emit the deprecated identifier
  on new exports.

### 5. Replacement

Names the replacement identifier and links to the schema or doc that
introduces it. If there is no replacement, the section MUST say so
explicitly and link to the ADR explaining why.

### 6. Consumer notice

Names the runtime consumers (docs page, CI gate, support surface,
release-evidence pack) that will read the deprecation packet to
decide whether to honour the deprecation. At least one consumer
MUST be named so the deprecation cannot ship without a real reader.

### 7. Failure drill

Names the failure-drill check id the lane should reproduce if the
deprecation packet drifts (for example,
`contribution_governance.deprecation_packet_template_artifact_required`).
This keeps the deprecation packet honest the same way the
contribution-governance seed itself stays honest.

## Cross-references

- [`./contribution_and_signoff.md`](./contribution_and_signoff.md)
  — reviewer-facing landing page for the contribution-governance seed.
- [`./public_interface_versioning_policy.md`](./public_interface_versioning_policy.md)
  — versioning rules the deprecation packet lives inside.
- [`./repo_hygiene_scaffolding.md`](./repo_hygiene_scaffolding.md)
  — repo-hygiene scaffolding that names the canonical home for this
  template.

If this template and the contribution-governance seed disagree, the
seed wins and this page MUST be updated in the same change.
