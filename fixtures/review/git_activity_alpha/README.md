# Git/review activity alpha fixtures

Worked examples for the Git/review activity event family:

- [`schemas/support/git_review_event_alpha.schema.json`](../../../schemas/support/git_review_event_alpha.schema.json)
- [`crates/aureline-shell/src/activity_center/git_review.rs`](../../../crates/aureline-shell/src/activity_center/git_review.rs)
- [`crates/aureline-shell/src/support_seed/mod.rs`](../../../crates/aureline-shell/src/support_seed/mod.rs)

The snapshot fixture covers a local Git mutation, a failed publish review, and
a local review workspace. It proves branch/head context, target identity,
command-backed action identity, exact reopen links, durable activity-row
projection, and structured support-export posture.

The support-export fixture is the same event family projected into a
machine-readable support artifact, so support bundles can include Git/review
events without scraping rendered activity-center text.
