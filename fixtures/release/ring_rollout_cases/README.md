# Ring rollout fixture cases

These fixture cases point the rollout validator at representative beta
promotion and rollback evidence. The canonical positive case is the checked-in
packet under `artifacts/release/m3/ring_rollout/`; failure drills are modeled as
small mutations in the manifest so the validator can keep the rejection classes
stable without duplicating large release packets.

Covered rejection classes:

- managed and self-serve lanes using different ring vocabularies;
- promotion hiding the prior package instead of preserving rollback visibility;
- rollback leaving more than one active package state for a channel;
- state-root audit rows drifting from install diagnostics.
