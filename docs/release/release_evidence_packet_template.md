# Release-evidence packet template

<!--
Copy this template when assembling a release-evidence packet or a
pre-release seed packet.

Related control artifacts:
- docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md
- docs/release/release_artifact_graph.md
- docs/release/qualification_cadence.md
- docs/governance/maintainer_coverage_policy.md
- artifacts/release/artifact_graph_rules.yaml
- artifacts/release/artifact_family_map.yaml
- artifacts/release/qualification_schedule.yaml
- artifacts/release/evidence_ownership_map.yaml
- artifacts/release/promotion_gate_map.yaml
- docs/release/build_farm_and_remote_cache_policy.md
- artifacts/release/pipeline_lane_rules.yaml
- artifacts/release/cache_trust_classes.yaml
- docs/release/ring_progression_policy.md
- artifacts/release/ring_matrix.yaml
- artifacts/governance/signing_quorum.yaml
- artifacts/governance/upstream_health_scorecard.yaml
- artifacts/governance/evidence_id_conventions.md
- docs/governance/evidence_freshness_policy.md
- artifacts/governance/evidence_freshness_slos.yaml
- artifacts/governance/evidence_rerun_triggers.yaml
- schemas/governance/evidence_packet_header.schema.json
- schemas/release/ring_history_packet.schema.json
- docs/build/exact_build_identity_model.md
- docs/benchmarks/benchmark_publication_pack_template.md
- artifacts/bench/protected_metrics.yaml
- docs/benchmarks/fitness_function_catalog.md
- docs/compat/compatibility_row_seed.md
- docs/deployment/drill_catalog_seed.md
- schemas/release/waiver_packet.schema.json
- artifacts/evidence/evidence_metadata_fields.yaml

This packet is intentionally structured around stable refs:
exact_build_identity_ref, evidence_id, compat_row id, drill_id,
source_anchor_ref, and waiver packet id. Do not substitute free-text
"same build as..." naming where a stable ref exists.
-->

## Shared packet header

Every release-evidence packet SHOULD embed a header that conforms to
`schemas/governance/evidence_packet_header.schema.json` and SHOULD
follow `artifacts/governance/evidence_id_conventions.md` when it joins
benchmark packets, verification packets, support drills, known-limit
notes, or migration packets.

- **Packet id:** `<release-evidence-packet-id>`
- **Packet state:** `draft` | `in_review` | `accepted` | `blocked` | `superseded`
- **Readiness:** `releasable` | `preview_only` | `narrow_claims` | `blocked`
- **Candidate or scope:** `<candidate-or-snapshot-label>`
- **Candidate stage:** `preview_candidate` | `beta_candidate` | `rc_or_stable_candidate` | `lts_candidate` | `hotfix_candidate`
- **Opened on:** `YYYY-MM-DD`
- **Assembled on:** `YYYY-MM-DDTHH:MM:SSZ`
- **Release channel scope:** `nightly` | `preview` | `beta` | `stable` | `lts` | `hotfix`
- **Deployment profile scope:** deployment-profile ids from `artifacts/compat/qualification_matrix_seed.yaml`
- **Owner:** `@handle`
- **Evidence owner:** `@handle`
- **Review forums:** `release_council`, `shiproom_executive_scope_review`, `<other forum if required>`
- **Benchmark-governance revision:** `<protected-metrics-id>@<metrics-revision>`
- **Primary exact-build identity set:** list of `exact_build_identity_ref`
- **Active waiver packet refs:** waiver packet ids or `none`
- **Promotion gate refs:** list of `gate.*` ids from `artifacts/release/promotion_gate_map.yaml`
- **Ring history packet refs:** packet ids conforming to `schemas/release/ring_history_packet.schema.json` or `none`
- **Late-proof exception refs:** exception ids from `artifacts/release/promotion_gate_map.yaml` or `none`
- **Emergency transport flow:** `none` or `mirror_only_response`
- **Maintainer coverage source:** `artifacts/governance/ownership_matrix.yaml` + `docs/governance/maintainer_coverage_policy.md`
- **Quorum action ids used:** list of `signing_quorum.yaml#actions.id`
- **Critical upstream refs:** list of `dependency_id` values from `artifacts/governance/upstream_health_scorecard.yaml` or `none`

## Executive summary

Two or three sentences: what this packet is asserting, what is
explicitly out of scope, and whether the current state is releasable,
preview-only, or claim-narrowed.

## Architecture anchors

- **Source anchor refs:**
  - `<path>#<anchor>` or `<doc> §<section>` — short note on why this
    anchor matters to the packet.
  - ...
- **Protected rows or requirement refs:**
  - `<requirement-id>` / `<fitness-row-id>` / `<compat-row-id>`
  - ...

## Promotion posture

- **Artifact families in scope:** `artifact_family_class` values from `schemas/build/exact_build_identity.schema.json`
- **Release posture classes in scope:** `primary_payload` | `debug_retention_sidecar` | `public_truth_payload` | `supply_chain_proof` | `supportability_payload` | `release_control_packet`
- **Same-change-set groups in scope:** `exact_build_and_publication_bundle` | `claim_docs_known_limit_bundle` | `supportability_and_symbolication_bundle` | `advisory_and_revocation_bundle` | `mirror_and_offline_bundle`
- **Rollback atom posture:** `coordinated_release_family` or narrowed explanation for why the omitted family is truly not in scope
- **Mirror/offline publication parity:** `not_required` | `required_when_claimed` | `required_for_supported_release_lines`
- **Manual-import or mirror manifest refs:** `<manifest refs or none>`

## Ring progression and reset posture

- **Validation ring scope:** `core_team_canary` | `broad_internal_dogfood` | `design_partner_preview` | `public_preview_or_beta` | `stable_candidate_or_ga`
- **Ring matrix source:** `artifacts/release/ring_matrix.yaml`
- **Ring history packet refs:** packet ids or `none`
- **Minimum soak observed:** one sentence with duration, cycle class, and whether the expectation is satisfied
- **Active hold or reset trigger refs:** `reset.*` ids from `artifacts/release/ring_matrix.yaml` or `none`
- **Reset-sensitive change families touched in this packet:** `version_skew_behavior` | `provider_mutation_authority` | `install_topology` | `schema_migration` | `protected_dependency_posture` | `none`
- **Owner acknowledgements:** lane refs or packet refs showing release, docs, support, and rollback review state

## Exact-build identity set

| Artifact family | `exact_build_identity_ref` | Evidence linkage | Source |
|---|---|---|---|
| `<ide_binary>` | `<exact_build_identity_ref>` | `build_log_ref`, `artifact_graph_node_ref`, `public_release_note_ref` | `<repo-relative path>` |
| `<docs_pack>` | `<exact_build_identity_ref>` | `provenance_statement_ref`, `sbom_document_ref` | `<repo-relative path>` |
| ... | ... | ... | ... |

## Benchmark and fitness evidence

- **Protected metrics file:** `artifacts/bench/protected_metrics.yaml` (`metrics_file_revision: <n>`)
- **Catalog:** `artifacts/bench/fitness_function_catalog.yaml` (`catalog_revision: <n>`)
- **Protected-path ledger:** `artifacts/perf/protected_path_ledger.yaml` (`ledger_revision: <n>`)
- **Latency-budget ledger:** `artifacts/perf/latency_budget_ledger.yaml` (`ledger_revision: <n>`)
- **Evidence-linkage seed:** `artifacts/perf/evidence_linkage_seed.yaml` (`seed_revision: <n>`)
- **Protected path ids cited:** list of `path.*` ids when the packet is making path-level claims
- **Runs or dashboards cited:** `benchmark_run_id`, `dashboard_id`, or both
- **Freshness and comparability:** one sentence that says whether the
  evidence is claim-bearing, seed-only, stale, or not yet comparable.
- **Publication-pack refs:** cite `benchmark-publication-pack-id` when a
  public performance claim leaves the raw dashboard or shiproom packet.

| `evidence_id` | Row or run ref | Captured at | `stale_after` | Comparability note | Source |
|---|---|---|---|---|---|
| `<evidence-id>` | `<fitness-row-id or run-id>` | `YYYY-MM-DDTHH:MM:SSZ` | `<duration or null>` | `<why comparable or not>` | `<repo-relative path>` |
| ... | ... | ... | ... | ... | ... |

## Qualification and compatibility

- **Qualification row refs:**
  - `<compat-row-id>` — verdict, evidence status, and source path.
  - ...
- **Accessibility acceptance-pack refs:** acceptance-pack family ids or
  task ids from `fixtures/accessibility/task_corpus_manifest.yaml`, or
  `none`
- **Accessibility known-limit refs:** `known_limit.accessibility.*`
  refs, or `none`
- **Qualification packet or report refs:**
  - `<packet-id or stable ref>` — `present`, `seed_only`, or `not_yet_available`.
  - ...
- **Future conformance or compatibility rows to refresh before widening claim:**
  - `<compat-row-id>` — reason it is still future or narrowed.
  - ...

## Locality and continuity truth

- **Deployment context:** `<profiles, install posture, or ring scope>`
- **Continuity drill refs:** `drill_id` values from `artifacts/support/deployment_drill_catalog_seed.yaml`
- **Locality / region / tenant / key posture:** plain truth for the
  claimed path; do not hide `not_applicable` or `local_only` states.
- **Linked release inputs:** install-topology, state-root, or mirror
  packet refs that explain the deployment envelope.

## Maintainer coverage and approval quorum

- **Protected lane refs:** package names or governance-lane ids touched
  by this packet.
- **Coverage posture:** `covered` | `waived_seed_only` | `blocked`
- **Quorum action ids:** ids from `artifacts/governance/signing_quorum.yaml`
- **Break-glass refs:** audit-log or governance-packet refs, or `none`
- **Critical upstream status:** dependency ids plus score or provisional
  risk from `artifacts/governance/upstream_health_scorecard.yaml`

## Active waivers

List every active waiver packet that narrows or temporarily bypasses a
claimed row. A packet with no active waivers should say `none` here,
not leave the section blank.

- **Waiver packet:** `<waiver-packet-id>`
  - **Workflow state:** `draft` | `submitted_for_review` | `active` | `renewal_requested` | `renewed` | `rejected` | `closed` | `expired`
  - **Affected requirement / protected path:** `<requirement-id>` / `<fitness-row-id>` / `<protected-path-id>`
  - **Mitigation now:** `<short mitigation>`
  - **Compensating evidence:** `<evidence-id>` values
  - **Owner and expiry:** `@handle`, `YYYY-MM-DDTHH:MM:SSZ`

## Waiver workflow

1. Open a packet that conforms to `schemas/release/waiver_packet.schema.json`.
2. Move `workflow_state` from `draft` to `submitted_for_review` once
   the exact gap, owner, mitigation, compensating evidence, and expiry
   are present.
3. Only list the waiver in **Active waivers** after the packet reaches
   `active` or `renewed`.
4. Renewals must cite a new `source_revision`, updated compensating
   evidence freshness, and a new expiry; silent roll-forward is
   non-conforming.
5. Close or let the waiver expire before widening the next release
   gate. Expired waivers stay visible in the packet until replaced or
   closed.

## Risks and disclosure

- User-visible or enterprise-relevant risks that still remain.
- Release-note disclosures required by active waivers or narrowed claims.
- Evidence that is stale, seed-only, or pending re-baseline.
- If the packet uses `Emergency transport flow != none`, describe the
  hosted-path gap and confirm the same exact-build, claim-row,
  known-limit, and release-note truth moved in the same refresh.

## Evidence index

Every evidence item listed here should use the shared field names from
`artifacts/evidence/evidence_metadata_fields.yaml`.

- **`evidence_id:`** `<evidence-id>`
  - **Artifact family:** `<artifact-family>`
  - **Packet id:** `<release-evidence-packet-id>`
  - **Evidence ref:** `<repo-relative path or opaque ref>`
  - **Captured at:** `YYYY-MM-DDTHH:MM:SSZ`
  - **Stale after:** `<ISO-8601 duration or null>`
  - **Source revision:** `<commit, schema revision, or document revision>`
  - **Trigger revision:** `<catalog/schema/anchor revision or null>`
  - **Channel context:** `<channel>`
  - **Deployment context:** `<profile ids>`
  - **Comparability note:** `<one sentence>`
  - **Exact-build identity ref:** `<exact_build_identity_ref or null>`
  - **Source anchor refs:** `<anchor refs or none>`
  - **Qualification row refs:** `<compat-row ids or none>`
  - **Continuity drill refs:** `<drill ids or none>`
  - **Waiver packet refs:** `<waiver ids or none>`

## Signoff and next action

- **Decision:** `accept packet` | `keep preview-only` | `narrow claims` | `block release`
- **Named next action:** one sentence that says what evidence refresh,
  waiver closure, or compatibility recheck is required next. Prefer a
  trigger id from `artifacts/governance/evidence_rerun_triggers.yaml`
  when the refresh is driven by a named invalidation rule.
