# Support Bundle Preview Contract

This document freezes the local review surface and post-export manifest
contract for support bundles. A support bundle is not shareable until
the user or admin can inspect stable item ids, risk classes, redaction
states, deselection rules, policy locks, and the exact manifest that
will travel with the archive.

Companion artifacts:

- [`/schemas/support/support_bundle_preview_item.schema.json`](../../schemas/support/support_bundle_preview_item.schema.json)
  - one row shown in local bundle preview.
- [`/schemas/support/support_bundle_manifest.schema.json`](../../schemas/support/support_bundle_manifest.schema.json)
  - preview and post-export manifest record.
- [`/fixtures/support/support_bundle_preview_cases/`](../../fixtures/support/support_bundle_preview_cases/)
  - seeded preview cases for metadata-only, code-adjacent opt-in,
  prohibited high-risk, and policy-locked exports.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  - governed support-bundle packet and redaction profile.
- [`/docs/support/diagnostic_artifact_matrix.md`](./diagnostic_artifact_matrix.md)
  and
  [`/artifacts/support/support_evidence_pack_matrix.yaml`](../../artifacts/support/support_evidence_pack_matrix.yaml)
  - stable support-pack item ids and default inclusion policy.
- [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  - representation-labeled export behavior and metadata-only fallback.
- [`/artifacts/security/redaction_posture_matrix.yaml`](../../artifacts/security/redaction_posture_matrix.yaml)
  - support-export redaction defaults.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` section 10.15 and section 10.22.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections
  21.3, 21.5, 21.8, 24.4, 24.5, and Appendix I.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 5.4, 9.6,
  11.5, and the support/export review templates.

If this document disagrees with the machine-readable schemas, the
schemas control shape and this document must be corrected in the same
change.

## Scope

Frozen at this revision:

- a preview-item record with stable item id, file or section identity,
  estimated size, shared diagnostic data class, visible redaction
  state, collected/generated/imported/source class, deselectability
  posture, policy lock state, and first-actionable-diagnosis impact;
- a manifest record with `collection_schema_version`, build id,
  exact-build refs, active redaction profile, policy notes, preview
  items, review decisions, excluded classes, redaction report,
  actionability warnings, reopen-after-export path, and parity digests;
- rules for item deselection and stronger redaction where diagnostic
  integrity permits;
- preview/export parity fields that let a post-export reader
  reconstruct exactly what was shared, omitted, redacted, or retained
  locally.

Out of scope:

- live collectors, upload transport, hosted intake, or ticket routing;
- byte-level redaction implementation;
- final support-center UI layout.

## Shared Risk Classes

Preview items reuse the diagnostic data classes already defined for
support artifacts:

| Class | Meaning in preview | Default posture |
|---|---|---|
| `metadata_only` | build ids, version, policy fingerprints, extension ids, docs-pack identity, summary counters | eligible for default inclusion when the user/admin requests a bundle |
| `environment_adjacent` | toolchain versions, target classes, connection classes, proxy posture, route summaries | eligible for metadata-only inclusion after clear preview |
| `code_adjacent` | filenames, stack traces, selected snippets, notebook cells, bounded mutation excerpts, command-argument summaries | excluded or redacted until exact item-level opt-in |
| `high_risk` | secret-bearing material, raw dumps, full shell history, raw traces, full transcripts, token-bearing payloads | prohibited or retained local-only unless a separate high-friction policy path exists |

High-risk preview rows must also name a `high_risk_content_class` such
as `secret_bearing`, `raw_dump_or_memory`, or
`full_shell_history`. A preview that says only "extra diagnostics" is
non-conforming.

## Preview Item Contract

Every row rendered in a support-bundle preview is a
`support_bundle_preview_item_record`.

| Field group | Required meaning |
|---|---|
| `preview_item_id` | Stable id copied from preview to export manifest and support intake. |
| `parity_binding.support_pack_item_id` | Stable item id from the diagnostic artifact matrix. |
| `file_section_identity` | Bundle section, artifact kind, preview label, manifest path ref, optional member path, and source refs. |
| `size_estimate` | Estimated byte size, confidence, display label, and source of the estimate. |
| `redaction` | Shared data class, high-risk subtype, support redaction class, visible redaction state, rule refs, and summary ref. |
| `materialization` | Whether the body is embedded, by reference, redacted, omitted, retained local-only, optional upload, or intentionally excluded. |
| `deselectability` | Whether the reviewer can remove the row, whether stronger redaction is available, and why a row is locked. |
| `actionability_impact` | Whether removing or further redacting the row reduces the chance of first actionable diagnosis. |
| `policy_lock` | Policy source and explicit reason whenever policy narrows or blocks export. |

Rows must preserve collected/generated/imported/source distinction with
`materialization.collection_source_class`. A generated omission marker,
an imported evidence-packet ref, and a user-selected code snippet are
different rows even when they share a support-pack item id.

## Manifest Contract

Every local preview and exported archive carries one
`support_bundle_manifest_record`.

Required manifest fields:

- `collection_schema_version` so readers know which collection contract
  produced the manifest.
- `build_identity.build_id` and `build_identity.exact_build_refs[]` so
  crash, symbol, docs, and release evidence can join without fuzzy
  version matching.
- `collection_context.policy_notes[]` for policy narrowing, local-only
  constraints, or admin-required reasons.
- `preview_items[]` with the exact rows shown to the reviewer.
- `review_decisions[]` with the selected inclusion, omission, opt-in,
  local-only, or stronger-redaction decision for each reviewed row.
- `excluded_classes[]` for classes omitted by user choice, policy,
  source absence, or prohibition.
- `redaction_report` with applied rules, redaction states present,
  high-risk items, prohibited items confirmed absent, and secret-scan
  outcome.
- `actionability_warnings[]` for any deselection or stronger redaction
  that reduces diagnosis potential.
- `reopen_after_export_path` so the exact preview can be reopened from
  the exported archive or local export history.
- `preview_export_parity` so post-export intake can reconstruct the
  preview order, item decisions, redaction rules, excluded classes, and
  unknown-field policy.

The manifest is the truth for what left the machine. The archive may
contain bytes, refs, digests, local-retention markers, or omission
markers, but the manifest must make those states distinguishable.

## Deselectability And Stronger Redaction

Preview rows fall into five deselection postures:

| Posture | Rule |
|---|---|
| `deselect_allowed_without_warning` | The row can be removed without reducing the first-actionable diagnosis path. |
| `deselect_allowed_with_actionability_warning` | The row can be removed only after a warning is recorded in `actionability_warnings[]`. |
| `blocked_required_for_diagnosis` | The row is required core metadata, exact-build truth, policy truth, or other minimum support context. |
| `blocked_by_policy` | Policy has narrowed the row; the policy note and lock reason must be visible. |
| `blocked_forbidden_marker_required` | The body is prohibited, but the omission marker must remain so absence is auditable. |

Stronger redaction is always narrowing. A reviewer may move a row from
embedded to redacted, by-reference, retained local-only, or omitted
only when the row advertises that state in
`allowed_stronger_redaction_states[]`. Policy may narrow further, but
policy must not silently expand collection beyond the documented class
rules.

If deselection or stronger redaction changes the diagnosis value, the
preview must record a warning before export. Required warnings name the
specific item id, impact class, and consequence; generic "bundle may be
less useful" copy is not enough.

## Preview And Export Parity

Preview/export parity has two goals:

1. The archive manifest can reconstruct exactly what was shared.
2. A reviewer can compare the manifest to other export surfaces using
   the same item ids and risk classes.

To satisfy parity, every manifest records:

- the preview snapshot ref and item-order digest;
- one review decision per included, omitted, prohibited, or retained
  local-only item;
- a digest or omission marker for every row that could have carried a
  body;
- exact-build refs used during collection;
- active redaction profile and applied rule refs;
- excluded class rows with explicit reasons;
- unknown-field handling policy;
- a reopen-after-export path.

Post-export readers must not infer payload meaning from archive member
names alone. They read the manifest first, then resolve embedded
members, managed refs, local-retention refs, optional-upload tickets,
or omission markers by stable preview item id.

## Fixture Expectations

The preview cases under
[`/fixtures/support/support_bundle_preview_cases/`](../../fixtures/support/support_bundle_preview_cases/)
cover:

- metadata-only bundle preview with exact-build and policy truth;
- code-adjacent item included only after exact item-level opt-in;
- secret-bearing high-risk item that is prohibited and represented by
  an omission marker;
- policy-locked preview with an explicit reason and diagnosis impact.

Each case is a machine-readable manifest. Reviewers should be able to
compare `preview_items[]`, `review_decisions[]`, `excluded_classes[]`,
and `redaction_report` without reading prose or assuming hidden
defaults.
