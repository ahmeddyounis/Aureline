# Repair Preview Alpha Fixtures

This fixture manifest binds the guided-repair protected path to the checked-in
repair seed cases. The support crate consumes the manifest in tests and proves
that each seed case can produce a transaction, preview, outcome, journal entry,
and metadata-safe support packet without inventing private tokens outside the
schemas.

The cases cover disposable cache rebuild, extension quarantine, toolchain
re-resolution, remote runtime rollback, policy approval refresh, and
escalation-only refusal when no safe local repair exists.
