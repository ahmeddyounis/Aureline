# M5 Governance Dashboard

- Packet: `m5-governance-dashboard:stable:0001`
- Label: `M5 governance dashboard`
- Corpus: `m5-reference-corpus:0001` (M5 reference corpus)
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Fitness: 7 (7 passing, 0 warning, 0 stale, 0 waived, 0 waiver-expired, 0 blocked)
- Waivers: 0 (0 open, 0 expired)
- Services: 4 (4 clean) — Decisions: 4 (4 exercisable)
- Stable promotion: pass

## Deployment-profile overviews

| Profile | Effective posture | Gate | Qualification | Passing | Warning | Stale | Waived | Waiver-expired | Blocked | Open waivers |
|---------|-------------------|------|---------------|---------|---------|-------|--------|----------------|---------|--------------|
| `managed` | `managed` | `governed` | `stable` | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| `self_hosted` | `self_hosted` | `governed` | `stable` | 3 | 0 | 0 | 0 | 0 | 0 | 0 |
| `regulated` | `regulated` | `governed` | `stable` | 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| `sovereign` | `sovereign` | `governed` | `stable` | 7 | 0 | 0 | 0 | 0 | 0 | 0 |

## Fitness tiles

| Function | Service | Scope | Measure | Freshness | State | Gate | Owner | Forum |
|----------|---------|-------|---------|-----------|-------|------|-------|-------|
| `package_boundary_integrity` | `package_governance` | `managed` | `pass` | `current` | `passing` | `governed` | `package_governance_owner` | `architecture_forum` |
| `protected_path_review` | `package_governance` | `self_hosted` | `pass` | `current` | `passing` | `governed` | `package_governance_owner` | `architecture_forum` |
| `schema_example_parity` | `evidence_pipeline` | `managed` | `pass` | `current` | `passing` | `governed` | `evidence_pipeline_owner` | `governance_council` |
| `evidence_freshness_slo` | `evidence_pipeline` | `regulated` | `pass` | `current` | `passing` | `governed` | `evidence_pipeline_owner` | `governance_council` |
| `claim_no_overclaim` | `claim_publication` | `regulated` | `pass` | `current` | `passing` | `governed` | `claim_publication_owner` | `shiproom_forum` |
| `route_explainability` | `route_provenance` | `sovereign` | `pass` | `current` | `passing` | `governed` | `route_provenance_owner` | `governance_council` |
| `provenance_completeness` | `route_provenance` | `sovereign` | `pass` | `current` | `passing` | `governed` | `route_provenance_owner` | `governance_council` |

## Service ownership

| Service | Owner | Forum | Worst state | Gate | Open waivers | Expired |
|---------|-------|-------|-------------|------|--------------|---------|
| `package_governance` | `package_governance_owner` | `architecture_forum` | `passing` | `governed` | 0 | 0 |
| `evidence_pipeline` | `evidence_pipeline_owner` | `governance_council` | `passing` | `governed` | 0 | 0 |
| `claim_publication` | `claim_publication_owner` | `shiproom_forum` | `passing` | `governed` | 0 | 0 |
| `route_provenance` | `route_provenance_owner` | `governance_council` | `passing` | `governed` | 0 | 0 |

## Decision rights

| Decision | Forum | Owner | Posture | Worst state | Gate | Blocking |
|----------|-------|-------|---------|-------------|------|----------|
| `stable_promotion` | `shiproom_forum` | `release_owner` | `clear` | `passing` | `governed` | 0 |
| `waiver_acceptance` | `governance_council` | `governance_owner` | `clear` | `passing` | `governed` | 0 |
| `boundary_change` | `architecture_forum` | `architecture_owner` | `clear` | `passing` | `governed` | 0 |
| `exception_renewal` | `governance_council` | `governance_owner` | `clear` | `passing` | `governed` | 0 |
