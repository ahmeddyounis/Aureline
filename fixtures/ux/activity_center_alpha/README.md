# Activity-center alpha fixtures

Worked examples for the durable activity-center alpha contract:

- [`schemas/events/activity_row.schema.json`](../../../schemas/events/activity_row.schema.json)
- [`docs/ux/activity_center_alpha.md`](../../../docs/ux/activity_center_alpha.md)
- [`crates/aureline-shell/src/activity_center/alpha.rs`](../../../crates/aureline-shell/src/activity_center/alpha.rs)

The snapshot fixture covers indexing, restore, install/update, task, and
test job families in one durable row model. It proves stable row/job ids,
actor or subsystem identity, progress and chronology, exact reopen,
cancel/retry/open-detail actions, impact flags, collapse behavior, and
support-export posture.

The support-export fixture is the same row set projected into a
machine-readable support artifact so downstream support/review flows do
not scrape UI text.
