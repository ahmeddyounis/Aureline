# Human-readable durable artifact registry

This document freezes the canonical inventory of durable artifacts that are:

- **human-owned** (user/profile/workspace), and therefore must remain text-first and reviewable, or
- **generated but explicitly reviewable** (lockfiles, import reports, manifests), or
- **managed-only but still inspectable** (policy-authority artifacts).

The goal is to ensure every loader, writer, migration tool, and support/export
flow can answer the same questions from one place instead of re-deriving format
and editability rules per surface.

Companion artifacts:

- [`/artifacts/governance/human_readable_artifacts.yaml`](../../artifacts/governance/human_readable_artifacts.yaml)
  — machine-readable registry rows (authoritative for tooling).
- [`/schemas/governance/human_readable_artifact_family.schema.json`](../../schemas/governance/human_readable_artifact_family.schema.json)
  — boundary schema for the registry file and its worked examples.
- [`/fixtures/governance/human_readable_artifact_examples/`](../../fixtures/governance/human_readable_artifact_examples/)
  — worked examples covering JSONC, JSON, YAML, Markdown, notebook, and mixed-manifest families.

Related policies and contracts:

- [`/docs/config/artifact_format_and_migration_policy.md`](../config/artifact_format_and_migration_policy.md)
  — shared rules for comment preservation, unknown-field handling, ordering,
  and rewrite disclosures.
- [`/docs/config/human_edited_artifact_contract.md`](../config/human_edited_artifact_contract.md)
  — additional contract detail for the core schema-backed human-edited config
  family (settings, keybindings, tasks, launch, workspace manifest).
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  — portability class vocabulary and export exclusions.

## How to use this registry

When a surface needs to read or write a durable artifact family, it consults the
registry row and treats the row as the policy input for:

1. **Format selection** (`format_class`) and whether the format can carry
   comments.
2. **Schema location** (`schema_home`) when schema-backed validation and
   migration apply.
3. **Preservation postures** (`comment_policy`, `unknown_field_policy`,
   `ordering_policy`).
4. **Write gating** (`editability_class`) — whether the surface may write
   directly, must show a preview first, should regenerate instead of preserving
   hand edits, or must refuse writes and offer inspection only.
5. **Review posture** (`vcs_review_posture`) and **portability posture**
   (`portability_expectation`) for export/import, support, and recovery flows.

If a surface needs a behavior that contradicts the row (for example, it wants to
canonicalize order for a class marked `stable_order_preserved`), the surface is
non-conforming until the registry and upstream format policy are updated in the
same change.

## Editability classes

Editability classes are the cross-surface decision point for when Aureline may
write an artifact without user review.

The canonical definitions and write posture flags are in
`artifacts/governance/human_readable_artifacts.yaml#editability_classes`.

At a high level:

- `freely_editable` — free-form text edited directly by humans; no background
  normalization.
- `structured_safe_editable` — schema-backed human-edited artifacts; writers
  preserve comments, unknown fields, and ordering per the row.
- `review_before_write` — writes require a preview that discloses lossiness,
  canonicalization, or broader-than-requested edits.
- `regenerate_preferred` — generated-but-reviewable artifacts; preferred path is
  regenerate-from-source with preview and rollback, not hand edits.
- `inspect_only` — readable/exportable, but not a general-purpose edit surface;
  writes route through the owning authority path (policy, signing, controlled
  tool).

## Worked examples

See `fixtures/governance/human_readable_artifact_examples/` for:

- a JSONC example showing comment + unknown-field preservation expectations:
  [`jsonc_user_settings_file.yaml`](../../fixtures/governance/human_readable_artifact_examples/jsonc_user_settings_file.yaml)
- a JSON example showing canonical ordering and portability exclusions:
  [`json_portable_profile_export.yaml`](../../fixtures/governance/human_readable_artifact_examples/json_portable_profile_export.yaml)
- a YAML example showing schema-attached governance YAML conventions:
  [`yaml_registry_row.yaml`](../../fixtures/governance/human_readable_artifact_examples/yaml_registry_row.yaml)
- a Markdown example covering repo instruction bundles consumed by assist flows:
  [`markdown_repo_instruction_bundle.yaml`](../../fixtures/governance/human_readable_artifact_examples/markdown_repo_instruction_bundle.yaml)
- a notebook example showing round-trip preview requirements for `.ipynb`:
  [`notebook_ipynb_roundtrip.yaml`](../../fixtures/governance/human_readable_artifact_examples/notebook_ipynb_roundtrip.yaml)
- a mixed-manifest example showing a package that carries a manifest plus
  multiple payload members (export + redaction + integrity):
  [`multi_format_portable_state_package.yaml`](../../fixtures/governance/human_readable_artifact_examples/multi_format_portable_state_package.yaml)
