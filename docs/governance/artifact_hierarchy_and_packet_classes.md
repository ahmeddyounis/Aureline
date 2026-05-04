# Artifact hierarchy and packet classes

This document is the human-readable overview of the canonical
control-artifact stack. It exists so reviewers can answer three
questions from one page:

1. **Which packet, report, or registry is authoritative for this
   surface?** Every governed artifact resolves to one canonical class,
   one owner lane, and one source-of-truth location.
2. **How do those classes link to one another?** Each class declares a
   minimum-backlink contract so requirement rows, ADRs, contract
   packets, verification corpora, claim rows, runbook/shiproom packets,
   and public-proof bundles compose without parallel identifiers.
3. **When is each class release-bearing?** A per-milestone release-use
   matrix names presence (required, required-if-class-used,
   recommended, not-required, retired) plus the missing and stale
   effects at every governed milestone.

Companion artifacts:

- [`/artifacts/governance/packet_class_registry.yaml`](../../artifacts/governance/packet_class_registry.yaml)
  — machine-readable registry. Tooling reads this file; the narrative
  below describes the same rows.
- [`/schemas/governance/artifact_class_row.schema.json`](../../schemas/governance/artifact_class_row.schema.json)
  — boundary schema for every row in the registry.
- [`/fixtures/governance/artifact_class_examples/`](../../fixtures/governance/artifact_class_examples/)
  — worked cross-class examples showing how one launch-bearing surface
  links across requirement, ADR, contract packet, verification corpus,
  claim manifest, shiproom packet, and public-proof bundle.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  and
  [`./control_artifact_index.md`](./control_artifact_index.md) —
  canonical home of every control asset. Each row in the packet-class
  registry resolves to exactly one row in the control-artifact index.
- [`/artifacts/governance/source_anchor_map.yaml`](../../artifacts/governance/source_anchor_map.yaml)
  and
  [`./canonical_reference_rules.md`](./canonical_reference_rules.md) —
  source-anchor map and linter that group rows by class.
- [`/artifacts/governance/mandatory_review_artifacts.yaml`](../../artifacts/governance/mandatory_review_artifacts.yaml)
  — minimum-contents catalog for ADRs, RFCs, design packets,
  verification packets, compatibility reports, benchmark reports, and
  waiver packets.

**One class, one owner, one release use.** The packet-class registry
does not restate ADR prose, schema fields, evidence-id grammar, claim
posture, or shiproom go/no-go vocabulary. It only freezes which class
governs which artifact family, which classes link to which, and what
happens at each milestone when an instance of the class is missing or
stale.

## Artifact classes

The registry covers ten canonical classes. The list mirrors
`schemas/governance/artifact_class_row.schema.json` and is closed:
every governed artifact in the repository resolves to exactly one of
these classes.

### `requirement_register_and_alias_crosswalk`

The canonical requirement register and the alias crosswalk. Owner lane
`governance_packets`. Public visibility. Freshness `each_change`. Every
shipped obligation, waiver, scorecard call, fitness row, and release
packet cites a row here rather than minting a new id. Required at every
governed milestone — missing rows block milestone close at
foundations/prototype, narrow claims at alpha, and block release
promotion or claim publication at first beta and stable.

### `architecture_decision_record`

Durable architecture decisions under `docs/adr/`. Owner lane
`governance_packets`. Public visibility. Freshness `each_change`. ADRs
authorize protected-lane contracts, runtime boundaries, storage or
schema shape, trust-model changes, or process placement. Required at
every governed milestone — missing ADRs block protected-path widening
through alpha and block release promotion at first beta and stable.

### `request_for_comments`

Broad subsystem proposals under `docs/rfc/`. Owner lane
`governance_packets`. Public visibility. Freshness `each_change`.
Required when broad changes need open review before an ADR can land;
otherwise the row is `required_if_class_used` so reviewers do not have
to mint an empty RFC at every milestone.

### `design_packet`

Launch-critical UX, accessibility, design-token, and interaction
packets under `docs/ux/`, `artifacts/ux/`, `docs/accessibility/`, and
`artifacts/accessibility/`. Owner lane `design_system_seeds`. Public
visibility. Freshness `each_change`. Recommended at foundations,
required-if-class-used at prototype, required at alpha and later.
Missing or stale design packets narrow claims to implemented or seed
only and block release promotion at first beta and stable.

### `contract_or_schema_or_interface_packet`

Surface-contract packets covering JSON schema exports, WIT worlds,
OpenAPI families, field registries, event envelopes, and mixed-version
envelopes. Owner lane `governance_packets`. Public visibility.
Freshness `each_change`. Required at every governed milestone — missing
or stale contract packets block protected-path widening, narrow the
compatibility window, and block release promotion or claim publication
at first beta and stable.

### `verification_corpus_and_benchmark_plan`

Verification packets, benchmark publication packs, fixture corpora,
reference hardware and lab-image manifests, the protected-metrics file,
the fitness-function catalog, and protected-path evidence. Owner lane
`benchmark_lab`. Public visibility. Freshness `each_release`. Required
at every governed milestone — missing or stale corpora block milestone
close, narrow claims to implemented or seed only, and block release
promotion or claim publication at first beta and stable.

### `compatibility_or_certified_archetype_report`

Reports under `artifacts/compat/`, `docs/state/`, and `docs/release/`
that bind public interface diffs, SDK/CLI/schema changes, retained
support windows, deprecation posture, and certified reference
workspaces or archetypes back to the surface contract packets and
verification corpora that prove them. Owner lane
`compatibility_ecosystem_review`. Public visibility. Freshness
`each_release`. Not required at foundations,
`required_if_class_used` through alpha, required at first beta and
stable.

### `claim_manifest_and_docs_migration_pack`

The public-truth claim manifest and its companion docs, help,
migration, known-limit, and release-note packets. Owner lane
`docs_public_truth`. Public visibility. Freshness `each_release`.
Required at every governed milestone — missing or stale rows block
claim publication and, at first beta and stable, block release
promotion.

### `runbook_support_or_shiproom_packet`

Operational packets that carry shiproom go/no-go review, release
readiness, support-export and crash-diagnostics handoff, field
runbooks, deployment drill catalog, and incident or emergency
response. Owner lane `support_export`. Internal visibility. Freshness
`each_release`. Required-if-class-used through prototype, required at
alpha and later.

### `public_proof_publication_bundle`

Reviewer-facing bundles under `docs/program/`, `docs/benchmarks/`, and
`artifacts/release/` that publish reproducibility inputs for benchmark,
compatibility, migration, accessibility, security, and claim-bearing
surfaces. Owner lane `docs_public_truth`. Public visibility. Freshness
`on_promotion`. Not required at foundations, recommended at prototype,
`required_if_class_used` at alpha, required at first beta and stable.

## Minimum-backlink graph

The registry freezes a directed back-edge graph so every instance
resolves to canonical ids upstream rather than restating prose. The
required edges are:

- `requirement_register_and_alias_crosswalk` →
  `architecture_decision_record`
- `architecture_decision_record` →
  `requirement_register_and_alias_crosswalk`
- `request_for_comments` →
  `requirement_register_and_alias_crosswalk`
- `design_packet` →
  `requirement_register_and_alias_crosswalk`,
  `verification_corpus_and_benchmark_plan`
- `contract_or_schema_or_interface_packet` →
  `requirement_register_and_alias_crosswalk`,
  `architecture_decision_record`
- `verification_corpus_and_benchmark_plan` →
  `requirement_register_and_alias_crosswalk`,
  `contract_or_schema_or_interface_packet`
- `compatibility_or_certified_archetype_report` →
  `contract_or_schema_or_interface_packet`,
  `verification_corpus_and_benchmark_plan`
- `claim_manifest_and_docs_migration_pack` →
  `requirement_register_and_alias_crosswalk`,
  `verification_corpus_and_benchmark_plan`
- `runbook_support_or_shiproom_packet` →
  `verification_corpus_and_benchmark_plan`,
  `claim_manifest_and_docs_migration_pack`
- `public_proof_publication_bundle` →
  `claim_manifest_and_docs_migration_pack`,
  `verification_corpus_and_benchmark_plan`

Optional edges are listed per row in the registry. Optional links never
substitute for required backlinks.

## Release-use matrix

The registry's per-row `release_use_matrix` is the canonical contract;
the table below is a reviewer-facing summary. Cells use the closed
presence vocabulary `required`, `required_if_class_used`,
`recommended`, `not_required`, `retired`. The missing-or-stale outcome
column lists the dominant effects across the milestone column;
per-milestone detail lives in the row.

| Class | Foundations | Prototype | Alpha | First beta | Stable |
| --- | --- | --- | --- | --- | --- |
| Requirement register and alias crosswalk | required | required | required | required | required |
| Architecture decision record | required | required | required | required | required |
| Request for comments | required-if-class-used | required-if-class-used | required-if-class-used | required-if-class-used | required-if-class-used |
| Design packet | recommended | required-if-class-used | required | required | required |
| Contract / schema / interface packet | required | required | required | required | required |
| Verification corpus and benchmark plan | required | required | required | required | required |
| Compatibility or certified-archetype report | not required | required-if-class-used | required-if-class-used | required | required |
| Claim manifest and docs/migration pack | required | required | required | required | required |
| Runbook / support / shiproom packet | required-if-class-used | required-if-class-used | required | required | required |
| Public-proof publication bundle | not required | recommended | required-if-class-used | required | required |

When a required class is missing or stale, the dominant effects at each
milestone are:

- **Foundations and prototype**: missing or stale instances block
  milestone close (requirement register, ADR, contract packet,
  verification corpus, claim manifest), block protected-path widening
  (ADR, contract packet), or narrow claims to implemented or seed only
  (design packet, verification corpus, claim manifest, public-proof
  bundle).
- **Alpha**: missing instances block milestone close on every required
  class. Stale instances narrow claims to implemented or seed only or
  force a waiver packet.
- **First beta and stable**: missing instances block release promotion
  on every required class. Stale instances block claim publication,
  narrow the compatibility window (contract packet, compatibility
  report), or downgrade the support window (runbook/support/shiproom
  packet).

The full per-milestone outcomes are quoted verbatim in the registry's
`release_use_matrix` so tooling can read the contract without
interpreting this prose.

## How to use the registry

### When proposing a new control artifact

1. Identify which existing class governs the artifact. If none does,
   open a decision row before minting a parallel id.
2. Confirm the canonical id namespace named in the registry row and
   reuse it. Aliases land in
   `docs/governance/requirement_alias_crosswalk.md` (for requirement
   ids) or in the source-of-truth document for the class.
3. Honor the row's minimum-backlink contract. Cite canonical ids by id,
   not by prose.
4. Update the row in
   `artifacts/governance/control_artifact_index.yaml` if the new
   artifact extends an existing canonical home, or land a new row in
   the same change.

### When updating a public surface

Walk the back-edge graph in reverse. A change to a public surface
typically requires updates in:

1. The contract packet that owns the surface id.
2. The compatibility or certified-archetype report (when the change
   affects a stable-facing surface).
3. The verification corpus or benchmark plan that proves the surface.
4. The claim manifest and docs/migration pack that publish the surface.
5. The runbook/support/shiproom packet that handles the rollout.
6. The public-proof bundle that publishes reproducibility inputs.

The cross-class fixtures under
[`/fixtures/governance/artifact_class_examples/`](../../fixtures/governance/artifact_class_examples/)
walk this path end-to-end for two launch-bearing surfaces.

### When a downstream packet looks stale

Read the upstream row's `freshness` block and the rerun-trigger refs
named in the registry. A stale instance flips the row's
`stale_effect` for every milestone at or after the current one;
release-bearing claims downgrade automatically rather than waiting for
a milestone-close review.

## What this document is not

- It is **not** the contract schema for an artifact-class row. The
  schema lives in
  [`/schemas/governance/artifact_class_row.schema.json`](../../schemas/governance/artifact_class_row.schema.json).
- It is **not** the source-of-truth for any individual artifact. Every
  registry row points at the canonical home; instances live there.
- It is **not** the mandatory-review artifacts catalog. The
  minimum-contents catalog for ADRs, RFCs, design packets, verification
  packets, compatibility reports, benchmark reports, and waiver
  packets lives in
  [`/artifacts/governance/mandatory_review_artifacts.yaml`](../../artifacts/governance/mandatory_review_artifacts.yaml).
- It is **not** the claim manifest. The claim-row contract lives in
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json).

## Change discipline

- Adding a class requires a new entry in the schema's
  `artifact_class_id_value` enum, a new row in the registry, a row in
  the control-artifact index, and a paired entry in the source-anchor
  map's `artifact_classes`. All in the same change.
- Retiring a class requires marking the row's release-use matrix entries
  `retired` for the affected milestones and leaving the row in place.
  Rows are not deleted, so the audit trail of "this class existed and
  was retired" survives.
- When this document and the YAML registry disagree, the YAML registry
  wins for tooling and this document must be updated in the same change.
