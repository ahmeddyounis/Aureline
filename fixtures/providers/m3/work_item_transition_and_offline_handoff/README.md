# Provider work-item transition and offline-handoff fixtures

This directory covers provider-backed work-item rows, status-transition
review, and offline handoff replay drills for the beta provider lanes.

The executable fixture source is the seeded page emitted by:

```sh
cargo run -p aureline-provider --bin aureline_provider_work_item_transition_beta -- page
```

The compact corpus matrix in `corpus_matrix.json` pins the drill classes
that the seeded page validates: stale target identity, revoked credentials,
conflicting provider updates, read-only sessions, offline capture, and
publish-later replay.

