# Control-artifact index

This document is the human-readable overview of the control-artifact
index. It exists so that every control asset the foundations milestone
produces has one canonical home, one named owner, and one review path
— and so that engineering, design, QE, docs, support, and release can
all find that home from a single page.

Companion artifacts:

- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — machine-readable register. Tooling reads this file; the narrative
  below describes the same rows.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — seed routing table for public and private issue classes.
- [`./feature_flag_policy.md`](./feature_flag_policy.md)
  — normative policy for experiments, feature flags, Labs inventory,
  rollouts, policy disables, and kill switches.
- [`/artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml)
  — canonical register for every control row, including hidden
  developer toggles and reserved control-stack bindings.
- [`/artifacts/governance/labs_register.yaml`](../../artifacts/governance/labs_register.yaml)
  — contributor-visible Labs / prototype / preview inventory
  projected from the canonical register.
- [`./benchmark_council_charter.md`](./benchmark_council_charter.md)
  — charter seed for the benchmark council (roles, scope, cadence).
- [`./interface_inventory.md`](./interface_inventory.md) — outline of
  the interface-inventory categories that the machine-readable form
  will eventually cover.

**One home, one owner, one review path.** Every control asset — the
control-artifact graph, the interface inventory, benchmark governance,
the benchmark-publication pack, the qualification cadence, the
compatibility qualification seed, the decision dependency register, the
build-vs-buy register, the fork-review policy, the supply-chain
dependency and import registers, the experiment/Labs control registers,
the release-notice seed, the release-artifact graph, release evidence,
docs and help truth, route and build-truth artifacts,
accessibility review packets, surface-traceability artifacts, and
frozen-surface manifests — appears as exactly one row in the index
file. If two documents describe the same asset, the index names only
the canonical one and the other is either merged in or retired.

The index does not restate the content of the assets it points at. The
detailed governance workflow lives in
[`decision_workflow.md`](./decision_workflow.md); contract schemas live
in [`/schemas/`](../../schemas/); claim-manifest rules live in the
governance-packet template. The index only names, routes, and scopes.

## How each role uses the index

### Engineering

- Before introducing a new control asset — a register, manifest,
  review packet, or machine-readable surface definition — look for an
  existing row in the index. If the asset already has a canonical
  home, extend that home; do not mint a parallel document.
- Experiments, feature flags, rollout rows, and Labs inventory updates
  now route through `feature_flag_policy`, `experiments_register`, and
  `labs_register`. Do not hide a new prototype mode or developer
  toggle in script help text alone.
- Protected-path dependency choices and deliberate upstream divergence
  now route through `build_vs_buy_register`, `dependency_review_policy`,
  `critical_dependency_register`, and `fork_review_policy`. Do not keep
  build-vs-buy posture or fork rationale only in an ADR or PR comment.
- When a pull request changes anything under a canonical-location path
  named in the index, update the corresponding row's `status` or
  `notes` if the change moves the artifact between `outline_only`,
  `seeded`, and `not_yet_seeded`.
- Review cadence `each_change` means the asset must be re-consulted
  on every pull request that could affect it. Review cadence
  `per_milestone` means on milestone boundaries. Review cadence
  `each_release` means during release-evidence assembly.

### Design

- Use the `design_system_seeds` and `accessibility_review_packets`
  rows as the only canonical locations for design-system snapshots,
  token sources, component references, accessibility audits, input-
  method review packets, and reduced-motion / contrast artifacts.
- When proposing a new design-system artifact, extend the existing
  canonical location under `artifacts/ux/`. Do not create parallel
  homes in docs or in an external tool.

### Quality engineering

- The `benchmark_governance`, `benchmark_publication_pack`,
  `fitness_function_catalog`,
  `benchmark_corpus_manifest`, `compatibility_qualification_seed`,
  `qualification_cadence`, `decision_dependency_register`,
  `build_vs_buy_register`, `critical_dependency_register`,
  `fork_review_policy`, `feature_flag_policy`, and
  `experiments_register` rows are the anchor points for quality work.
  New fitness functions, benchmark corpora, and qualification gates
  land under the lanes named by those rows. Protected speed and
  safety claims MUST cite a row in the fitness-function catalog via
  the `fitness_function_snapshot` packet shape rather than invent a
  parallel metric.
- Public benchmark claims do not ship straight from the raw dashboard
  or a slide deck. They now route through
  [`docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  so exact command line, corpus revision, comparability, docs
  applicability, known limits, and competitor settings are frozen in
  one packet.
- Disputes about benchmark results route through the benchmark-council
  charter (see `benchmark_governance`); do not open ad-hoc comparison
  threads outside that forum. The `fitness_function_catalog` row is
  the source of truth for which protected fitness functions exist at
  a given catalog revision and which waiver authority covers each.

### Docs

- The `docs_public_truth` and `route_build_truth` rows are the
  canonical homes for every public-facing document, including the
  known-limits matrix, the support-window statement, and migration
  guides.
- The clean-room rebuild lane now lives beside the reproducible-build
  baseline under `route_build_truth`; build truth is no longer only a
  narrative baseline plus an implicit CI script.
- The Labs inventory is not free-form copy. Contributor-visible
  prototype or preview inventory must resolve back to
  `labs_register.yaml`, and policy/process guidance must resolve back
  to `feature_flag_policy.md`.
- Any claim made in docs that a downstream consumer might rely on must
  cite an evidence owner via the claim-manifest packet family
  (governance-packet template). The index does not duplicate the
  manifest itself — it only names where the manifest lives.

### Support

- The `support_export` and `accessibility_review_packets` rows are the
  canonical homes for supportability artifacts. Field runbooks, the
  crash-diagnostics corpus, and export-safe packet schemas all live
  there.
- The `record_class_registry` row is the canonical home for class-level
  retention, export, hold, delete, and offboarding posture that
  support bundles, issue handoff packets, and later managed support
  claims must quote instead of re-labelling privately.
- The deployment continuity drill catalog seed lives under
  [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  with narrative guidance in
  [`/docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md).
  Release and boundary lanes cite that catalog rather than minting
  separate control-plane/data-plane outage vocabulary.
- Private partner and support cases follow the private routes in
  [`issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml);
  public supportability defects route to the OSS lane. Both cases
  preserve the owning forum.

### Release

- The `release_artifact_graph`, `release_evidence`,
  `compatibility_qualification_seed`,
  `release_notice_seed`, `frozen_surface_manifests`,
  `route_build_truth`, and `cleanroom_rebuild_lane` rows are the
  anchor points for release assembly.
  `review_cadence: each_release` means the release-engineer DRI MUST
  re-consult the artifact's rules before cutting a release.
- Release completeness is no longer implicit. The canonical graph and
  machine-readable rules now live in
  [`docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  and
  [`artifacts/release/artifact_graph_rules.yaml`](../../artifacts/release/artifact_graph_rules.yaml),
  which bind binaries, debug manifests, docs/help truth, benchmark
  proof packets, advisories, and promotion evidence into one
  non-overlapping release graph.
- The release-evidence family is now seeded with a narrative packet
  template in [`docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md),
  a filled seed example, a release-waiver schema under
  [`schemas/release/`](../../schemas/release/), and a shared evidence
  metadata catalog under
  [`artifacts/evidence/`](../../artifacts/evidence/). Concrete release
  packets still land under `artifacts/release/` when a real candidate is
  assembled.
- The clean-room rebuild lane now has a canonical command
  ([`ci/cleanroom_rebuild.sh`](../../ci/cleanroom_rebuild.sh)), a public
  contract document
  ([`docs/build/cleanroom_rebuild_lane.md`](../build/cleanroom_rebuild_lane.md)),
  and a CI wrapper
  ([`/.github/workflows/cleanroom_rebuild.yml`](../../.github/workflows/cleanroom_rebuild.yml)).
  Its current limitations are intentionally named in emitted artifacts
  rather than being left as tribal CI knowledge.
- Frozen-surface manifests and stable-surface contract metadata remain
  explicitly out of scope at this milestone.

## Review cadence semantics

- **`each_change`** — the artifact is consulted on every pull request
  that could affect it. Decision-register rows, public-truth copy,
  and build-truth artifacts use this cadence.
- **`per_milestone`** — the artifact is reviewed at milestone
  boundaries, alongside the scorecard. Governance packets,
  benchmark-council outputs, and surface-traceability artifacts use
  this cadence.
- **`each_release`** — the artifact is consulted during release-
  evidence assembly. Release packets and frozen-surface manifests
  use this cadence.

## Visibility classes

- **`internal`** — the artifact is a repository-internal control
  record. It may still live in a public repository, but it is not a
  published surface and no downstream consumer is expected to
  integrate against it.
- **`public`** — the artifact is part of the project's public truth:
  it is linked from docs intended for downstream consumers and is
  governed by the `docs_public_truth` lane.

### Product

- The `boundary_manifest_strawman` row is the canonical home for the
  open-source core versus managed / commercial / service-plane
  boundary. The strawman narrative lives under `docs/product/` and
  conforms to `schemas/product/boundary_manifest.schema.json`. Every
  new product capability or managed service must either map to an
  existing row or land a new row in the same change; adding a
  managed service without a row is a governance error.
- Capability rows that introduce managed copies, support exports, AI
  evidence, usage exports, exit packets, or destruction receipts also
  resolve through the `record_class_registry` row so the managed claim
  does not hide record behavior.

## What this index is not

- It is **not** the detailed governance workflow. That lives in
  [`decision_workflow.md`](./decision_workflow.md).
- It is **not** the contract schema for any artifact. Schemas live
  under [`/schemas/`](../../schemas/).
- It is **not** the claim manifest. The claim-manifest family is
  defined by
  [`/artifacts/governance/governance_packet_template.yaml`](../../artifacts/governance/governance_packet_template.yaml)
  and instantiated per release.
- It is **not** a substitute for the ownership matrix. The matrix
  defines who owns each lane; this index defines which artifacts sit
  inside those lanes. Lane IDs in the index always resolve back to
  `ownership_matrix.scorecard_lane_index`.

## Change discipline

- Adding a new control asset requires: a row in
  [`control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml),
  a canonical location, a named owner DRI, a lane from the ownership
  matrix, a review cadence, a visibility class, and a next-milestone
  target. All in the same change.
- Retiring a control asset requires: setting its `status` to reflect
  the removal and leaving the row in place with a note in `notes`.
  Rows are not deleted, so the audit trail of "this home existed and
  was retired" survives.
- When this document and the YAML index disagree, the YAML index wins
  for tooling and this document must be updated in the same change.
