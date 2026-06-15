# M5 publish-preview sheet set

This document describes the canonical packet that freezes the **M5 publish-preview
sheets** — the reviewed publish action an author drives before a package reaches the
public registry. It is the user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-publish-preview.json` and the typed model in the
`aureline-ecosystem` crate (`m5_publish_preview`).

Where the [author-and-publish-preview matrix](m5-author-and-publish-preview.md) freezes
the whole author lane as one row per family and carries a single `publish_preview_ref`,
this packet **materializes that reference** into a first-class publish-preview sheet. The
matrix answers "may this family publish at all?"; the sheet answers "what exactly is being
published, and which gate said no?". It reuses the same closed artifact-family,
runtime-class, host/ABI, signing-state, trust-posture, hot-reload, finding-severity, and
publish-readiness vocabulary, so the sheet and the matrix describe the same artifact rather
than a parallel synonym set.

## What each sheet carries

The packet carries one sheet for every claimed M5 artifact family. Each sheet answers, for
its publish action:

- **What is being published?** A `package_identity`, the `current_version` and
  `proposed_version`, a `version_bump` of `no_bump`, `patch`, `minor`, `major`,
  `downgrade`, or `invalid`, and a `release_channel` of `stable`, `beta`, `edge`, `canary`,
  or `internal`.
- **What changed in the manifest?** A `manifest_deltas` list, one entry per changed section
  with a `path`, a `kind` (metadata, dependency, API addition/break, permission
  added/removed, runtime-class change, external executable added, namespace rebind, signer
  rotation), and a short `detail`. The largest [change impact](m5-author-and-publish-preview.md)
  in the diff sets the minimum version bump the sheet must carry.
- **Who is the signer and what namespace?** A `signature_state`, an opaque
  `signer_identity_ref`, a `namespace_state` of `publisher_owned`, `publisher_verified`,
  `enterprise_managed`, `namespace_transfer_pending`, `namespace_mismatch`, or
  `namespace_unclaimed`, and an opaque `namespace_ref`.
- **What did each gate say?** A `checks` list with one `CheckResult` per named publish gate —
  `schema_validation`, `conformance_kit`, `accessibility_smoke`, `performance_smoke`,
  `docs_completeness`, `template_sample_completeness`, and `registry_policy` — each with an
  outcome of `passed`, `warning`, `blocked`, `not_applicable`, or `not_run`.
- **What are the findings?** A `findings` list, each tagged with a `source` (which gate it
  came from), a `reason`, a `severity` of `blocker` or `warning`, and an opaque
  `detail_ref`.
- **What is the verdict?** A `published_trust_posture` and a `publish_readiness` of
  `ready_to_publish`, `publishable_with_warnings`, `blocked_from_publish`, or
  `withheld_quarantined`.

## A publish preview is a review, not a manifest linter

The sheet recomputes its verdict from the observed facts, so it cannot be reduced to a
pass/fail lint:

- **Blockers versus warnings stay explicit, and each names its source.** A blocked or
  warned named gate raises a `check_failed`, `check_warning`, or `check_not_run` finding
  attributed to that gate, so a reviewer always sees whether a blocker came from schema
  validation, the conformance kit, the accessibility or performance smoke, docs
  completeness, template/sample completeness, or registry policy — versus the structural
  manifest, version, signer, namespace, channel, or hot-reload facts.
- **The version bump must cover the change.** The bump must be at least as large as the
  largest change impact in the diff; an undersized bump, a missing bump, a downgrade, or an
  invalid version each block publication.
- **Widening requires a fresh review.** A manifest delta that widens permissions, the
  runtime class, or an external executable — or a hot reload that would do the same — raises
  a blocking finding until `widening_reviewed` clears it, so widening never reaches the
  registry through a hot reload alone.
- **Signer and namespace truth cap the badge.** The published trust posture is the minimum
  of the signing-state ceiling and the namespace ceiling, so a locally-built artifact, an
  unsigned or revoked signature, or an unclaimed or mismatched namespace can never inherit a
  verified-publisher or enterprise-approved badge.
- **The channel carries its own consequences.** A channel that requires a signed release
  blocks an unsigned local-only artifact; a channel that requires a clean release blocks a
  release that still carries warnings. Namespace, signer, and channel consequences are
  never hidden from the review surface.

## Narrowing / cross-check

- The typed model recomputes the effective trust posture, the readiness, and the full
  finding set (in canonical source/reason order) from the observed facts; a checked-in
  packet that drifts fails `M5PublishPreviewSheetSet::validate`.
- `M5PublishPreviewSheetSet::cross_check_matrix` proves no sheet publishes a stronger badge
  than the author-lane publish gate would grant the same family, so the publish preview and
  the author lane project one trust truth.
- Downstream surfaces — authoring chrome, install/update flows, diagnostics, support
  exports, and release packets — consume `export_projection()` rather than cloning publish
  status text. Each sheet carries a `release_packet_ref` so release packets reference the
  preview object directly.
