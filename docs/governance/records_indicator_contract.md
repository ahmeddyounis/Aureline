# Records-governance indicator scoreboard, evidence-source contract, and failure-response rules

This contract freezes one shared vocabulary for the **records-governance
indicator scoreboard**. It exists so reviewers can measure and steer
records-governance health *before* managed-service launch: boundary-manifest
coverage, deletion/hold visibility, export completeness, record-class freshness,
dependency-marker debt, and public-proof link coverage.

The contract is pre-implementation. It defines the reusable indicator-row record
shape, the evidence-source contract each row must carry, and the typed failure
response that narrows shiproom posture, claim publication, and docs/help
language **without changing the underlying retention/deletion/hold vocabulary**
already frozen elsewhere.

## Companion artifacts

- [`/schemas/governance/indicator_row.schema.json`](../../schemas/governance/indicator_row.schema.json)
  — boundary schema for one `indicator_row_record`.
- [`/artifacts/governance/records_governance_indicator_scoreboard.yaml`](../../artifacts/governance/records_governance_indicator_scoreboard.yaml)
  — seeded indicator register. Every row conforms to the schema above.

Related upstream contracts the indicator rows MUST cite (instead of rewording):

- [`/docs/governance/record_class_governance.md`](./record_class_governance.md) and
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — canonical record-class posture (retention/export/delete/hold/offboarding).
- [`/docs/governance/retention_deletion_matrix_contract.md`](./retention_deletion_matrix_contract.md) and
  [`/schemas/governance/delete_request_state.schema.json`](../../schemas/governance/delete_request_state.schema.json)
  — deletion honesty, hold visibility, and delete-request state vocabulary.
- [`/docs/governance/claim_manifest_contract.md`](./claim_manifest_contract.md) and
  [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — claim-row posture and downgrade rules that docs/help and release packets
  must project instead of inventing local wording.
- [`/docs/release/shiproom_runbook.md`](../release/shiproom_runbook.md) and
  [`/docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md)
  — shiproom status vocabulary and release readiness caps the indicator failures
  map onto.
- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  — public boundary-manifest rows whose coverage is score-boarded here.
- [`/docs/qe/public_proof_scoreboards.md`](../qe/public_proof_scoreboards.md) and
  [`/artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml)
  — public-proof scoreboards and packet shape bindings used by the
  public-proof-link indicators.

## Normative sources projected here

- `.t2/docs/Aureline_Milestones_Document.md` records governance, deletion honesty,
  and monthly leading-indicator guidance.
- `.t2/docs/Aureline_PRD.md` retention/deletion/evidence boundary rules for
  collaboration, AI evidence, audit events, and support bundles.
- `.t2/docs/Aureline_Technical_Design_Document.md` records-governance stewardship
  architecture (record-class registry, hold resolver, request case manager,
  archive/export pipeline, destruction receipts).

If this contract disagrees with those sources, those sources win and this
contract, schema, and scoreboard update in the same change.

## Why an indicator scoreboard exists

1. **Records-governance drift is slow and silent.** Without a monthly scoreboard,
   deletion honesty, hold visibility, export/offboarding completeness, and
   public-proof link coverage become “we’ll fix it later” promises that only
   surface when enterprise trust is already on the line.
2. **Governance must bind to concrete record-bearing surfaces.** Indicators are
   required to cite record classes, boundary-manifest rows, claim rows, and
   export/delete/offboarding contracts. Abstract “ops metrics” that do not point
   back to governed surfaces are non-conforming.
3. **Failures must narrow truth, not rename it.** Indicator failures change
   *posture* (shiproom status floor, claim-posture cap, required disclosure) but
   MUST NOT mint new deletion/hold vocabulary. Deletion honesty and hold
   semantics remain governed by the retention/deletion matrix and record-state
   model.

## 1. Indicator row shape

Each `indicator_row_record` carries:

- Stable identity and scope:
  - `indicator_id`, `indicator_kind`, `indicator_family_id`
  - `subject` refs to record classes, claim rows, boundary-manifest rows, and
    relevant contracts/schemas/artifacts.
- Evidence-source contract:
  - `collection_cadence` (monthly)
  - `automation` (`scriptable`, `scriptable_with_manual_review`, or `manual_only`)
  - `evidence_sources[]` with typed `source_kind` plus a concrete `ref` and a
    one-sentence note explaining what is extracted.
- Ownership, waiver, and escalation:
  - `owner_lane` resolving through `artifacts/governance/ownership_matrix.yaml`
  - `waiver_policy` naming the waiver authority lane and the waiver register
    contract reference
  - `escalation_policy` naming the owning forum reference plus an SLA for
    converting a failed indicator into an owned correction (narrowing, waiver,
    or publication fix).
- Failure response:
  - `failure_response.shiproom_status_floor` uses the shiproom status vocabulary.
  - `failure_response.release_readiness_cap` uses the release readiness
    vocabulary.
  - `failure_response.claim_posture_cap` uses the claim-manifest posture
    vocabulary.
  - `failure_response.followthrough_actions[]` is a typed list of required
    follow-through actions.

Indicators may be computed by tooling, but consuming surfaces MUST be able to
act on the row without tooling: the evidence sources, pass condition, failure
response, and escalation route are required fields.

## 2. Evidence-source contract

Every indicator row MUST list evidence sources that can be inspected without
private vendor tooling. Sources are typed as one of:

- `repo_artifact` — YAML/JSON artifacts under `artifacts/` or `fixtures/`.
- `repo_schema` — JSON schemas under `schemas/`.
- `repo_doc` — normative narrative contracts under `docs/`.
- `generated_report` — deterministic generated outputs (when seeded later).
- `manual_review` — a temporary stopgap when an upstream source is not yet
  machine-readable; the row MUST still point to concrete refs and MUST declare
  an escalation path to remove the manual-only dependency.

## 3. Failure-response contract (narrowing without renaming)

Indicator failures MUST be expressed only as:

- a shiproom status floor (`hold_for_refresh` or `no_go` as appropriate),
- a release readiness cap (`narrow_claims` or `blocked`),
- a claim-posture cap (`experimental`, `limited`, etc), and
- a typed follow-through plan.

No indicator row is allowed to “fix” a deletion/hold semantics issue by renaming
delete states, introducing a new hold label, or hiding partial outcomes. The
row may require follow-through actions such as publishing a known-limit note,
adding an explicit dependency marker, or narrowing a claim row — but it must
not invent replacement vocabulary for governed record state.

## 4. Change discipline

- Adding a new indicator row is additive.
- Adding a new enum value in `indicator_row.schema.json` is additive-minor and
  requires this contract and the scoreboard seed to update in the same change.
- Repurposing an enum value, changing the meaning of an indicator kind, or
  reusing an `indicator_id` for a different meaning is breaking and requires a
  governance decision.
