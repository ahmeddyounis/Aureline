# Fixtures: Stabilize review workspace anchors, stale-base labels, approval invalidation, and mergeability truth

These fixtures exercise the review-stabilization packet that binds workspace anchors, landing-candidate truths, review-pack digest, base/head identity, required-check vocabulary, and ownership signals into a single coherent stabilization view.

## Files

| Fixture | Scenario |
|---|---|
| `stabilized_current.json` | Everything green: anchors bound exact, base current, mergeable, no ownership conflict, provider-authoritative. |
| `stabilized_stale_pack.json` | Stale-pack downgrade: anchors bound to stale pack digest, checks stale within grace, mergeable pending eligibility. |
| `approval_invalidated_with_replay_evidence.json` | Approval invalidated by CI check failure after approval, with replayable local-CI and AI-review evidence. |
| `ownership_conflict_with_offline_handoff.json` | Advisory vs enforceable ownership conflict on `src/critical/`, with bundle export and offline handoff preserving review-pack version and divergence labels. |

## Fixture format

Each fixture is a JSON object with:

- `record_kind`: `"review_stabilization_case"`
- `schema_version`: `1`
- `case_name`: descriptive name
- `seed_fixture_ref`: path to the alpha seed fixture
- `beta_workspace_input`: input to `ReviewWorkspaceBetaPacket::from_seed_packet`
- `landing_input`: input to `LandingCandidatePacket::from_workspace_packet`
- `stabilization_input`: input to `ReviewStabilizationPacket::from_workspace_and_landing_packets`
- `expected`: boolean and count assertions for the inspection record
