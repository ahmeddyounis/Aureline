# Proof packet: external alpha proof artifact index

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T17:31:25Z
stale_after: P14D
source_revision: git:7ef49d38b543d94113d56e1b3aa289eea9e62c2e
trigger_revision: alpha_truth_workflow_contract_set@2026-05-15
validator: ci/check_alpha_proof_artifact_index.py
validation_capture: artifacts/milestones/m2/captures/artifact_index_validation_capture.json
claim_change_state: no_claim_widening
```

Index: `artifacts/milestones/m2/artifact_index.yaml`  
Review template: `docs/review/m2_review_packet_template.md`  
Truth workflow: `docs/governance/m2_truth_update_workflow.md`  
Validator: `ci/check_alpha_proof_artifact_index.py`  
Latest capture: `artifacts/milestones/m2/captures/artifact_index_validation_capture.json`

This packet anchors the alpha proof registry. It proves that each current alpha
planning row has one registered proof lane, that review packets carry owner,
freshness, and exact-build identity fields, and that claim-bearing rows carry
same-change-set docs, migration, help, known-limit, and support-export refs.

## Protected Proof Path

Run:

`python3 ci/check_alpha_proof_artifact_index.py --repo-root . --report artifacts/milestones/m2/captures/artifact_index_validation_capture.json`

The validator checks that:

- the index consumes the existing alpha wedge matrix, scoreboard, and
  dependency graph;
- every current alpha plan ref is represented by at least one proof lane;
- each proof lane cites owner, freshness metadata, exact-build identity, and an
  owning packet;
- same-change-set truth refs cover docs, migration notes, Help/About truth,
  known limits, and support export; and
- the review packet template requires freshness date, owner, and exact-build
  identity instead of free-form notes.
