# M5 author-and-publish-preview matrix

This document describes the canonical packet that freezes the **M5 author-side and
publish-preview matrix** — one row per marketed M5 artifact family — covering the
local extension workspace, sideload review, sandbox/runtime inspection,
hot-reload/relaunch, and publish preview an author drives before a package reaches the
public registry. It is the user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-author-and-publish-preview.json` and the typed model in the
`aureline-ecosystem` crate (`m5_author_and_publish_preview`).

Where the
[install-governance matrix](m5-ecosystem-install-governance-matrix.md) speaks for the
*end-user install* of a marketed M5 artifact family, this packet is the canonical truth
for the *author lane*. It reuses the same closed artifact-family vocabulary so the new
M5 framework packs, docs packs, local-model packs, recipe packs, templates, and
bridge-backed or side-loaded packages are authored and shipped through one trust model
instead of a parallel synonym set.

## What this packet covers

The packet carries one row for every claimed M5 artifact family:

1. **`first_party_framework_pack`** — first-party framework pack.
2. **`docs_pack`** — documentation pack.
3. **`local_model_pack`** — local-model pack.
4. **`signed_recipe_pack`** — signed recipe pack.
5. **`template_artifact`** — template artifact.
6. **`bridge_backed_package`** — bridge-backed package.
7. **`side_loaded_package`** — side-loaded package.
8. **`mirrored_registry_variant`** — mirrored/private-registry variant.

Each row answers, for its family:

- **What shape does it run as?** A `runtime_class` of `passive_package`,
  `wasm_capability_sandbox`, `declarative_host_rendered_view`, `external_host`,
  `compatibility_bridge`, or `remote_side_component`, plus a `host_abi` execution locus
  of `no_code_execution`, `local_machine`, `managed_host`, `remote_target`,
  `external_process`, or `browser_runtime`.
- **What state is the local workspace in?** A `workspace_state` of
  `source_present_built`, `source_missing`, or `build_failed`.
- **How is it signed?** A `signature_state` of `signed_verified`, `signed_unverified`,
  `unsigned_local_dev`, `unsigned_sideload`, or `revoked_signature`.
- **What trust posture does it carry?** A `declared_trust_posture` the author requests
  and a `published_trust_posture` the gate allows, drawn from `unsigned_local_only`,
  `registry_bound`, `verified_publisher`, and `enterprise_approved`.
- **What does hot reload do?** A `hot_reload_posture` of `no_widening`,
  `relaunch_only`, `runtime_class_widened_pending_review`,
  `permissions_widened_pending_review`, or `external_executable_added_pending_review`.
- **What publish review does it require?** A `publish_review_requirement` of
  `full_registry_policy_review`, `standard_review`, `expedited_review`, or
  `not_publishable_from_local`.
- **How does it conform?** A `conformance_output` of `conformant`,
  `bridge_conformant`, `partial`, `failed`, `retest_pending`, or `not_run`.
- **What is its anti-abuse transparency?** An `anti_abuse_transparency` of
  `disclosed_clean`, `publisher_loss_history_disclosed`, `undisclosed`, or
  `quarantined`.

## The publish gate keeps blockers and warnings explicit

The gate recomputes three things from the observed states, and the packet stores them
only when they equal the recomputed values:

- **`published_trust_posture`** — the trust posture the family may publish, capped to
  the ceiling implied by its signing state. A `signed_verified` artifact may publish up
  to `enterprise_approved`; a `signed_unverified` artifact up to `registry_bound`; and
  an `unsigned_local_dev`, `unsigned_sideload`, or `revoked_signature` artifact is
  capped at `unsigned_local_only`.
- **`findings`** — a severity-tagged list. Each finding carries a closed `code`, a
  `severity` of `blocker` or `warning`, and a `domain` of `local_workspace`,
  `sideload_trust`, `hot_reload`, `conformance`, or `anti_abuse`. A blocker hard-stops
  publication; a warning publishes with disclosure. The publish preview is therefore a
  real review with explicit registry-policy consequences, not a pass/fail manifest
  lint.
- **`publish_readiness`** — `withheld_quarantined` when the family is quarantined,
  `blocked_from_publish` when any blocker is present, `publishable_with_warnings` when
  only warnings are present, and `ready_to_publish` when the family is genuinely clean.

A hot reload that would widen the runtime class, add an external executable, or expand
permissions raises a blocking `hot_reload_*` finding, so authority can never widen
without a fresh review step.

## Author-side packages never inherit end-user registry trust

The gate is non-inheriting by construction:

- An `unsigned_local_dev`, `unsigned_sideload`, or `revoked_signature` artifact must
  publish as `unsigned_local_only`. A locally-built or side-loaded artifact can never
  inherit a `verified_publisher` or `enterprise_approved` badge just because it was
  built on the same machine as a trusted user. The packet exercises this directly: the
  unsigned `local_model_pack` declares a verified-publisher badge and the revoked
  `mirrored_registry_variant` declares an enterprise-approved badge, yet both publish
  as `unsigned_local_only`.
- Each claimed family carries exactly one row, so no family inherits an author-lane
  posture from an adjacent one.

## How downstream surfaces consume it

Local authoring surfaces, package install/update flows, diagnostics, and certification
packets should ingest `export_projection()` from the typed model — including the
per-family `blocker_count`, `warning_count`, finding codes, published trust posture,
and publish-readiness verdict — rather than cloning author/publish status text.

## Validation

`M5AuthorPublishMatrix::validate()` returns a violation for any row whose published
trust posture exceeds its signing ceiling, whose local/side-loaded/revoked artifact
inherited a trusted badge, whose readiness or findings disagree with the gate, whose
finding severity or domain disagrees with its code, whose ready-to-publish row is not
genuinely clean, or whose summary counts disagree with the rows. The
`aureline-ecosystem` tests load the embedded packet, assert it validates, and prove the
gate, vocabulary coverage, and non-inheritance guardrail.
