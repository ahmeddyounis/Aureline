# Artifact: Stabilize profile switch review and temporary profile lifecycle

The profile-switch lifecycle lane is now represented by the checked-in shared
contract `settings:profile_switch_review_lifecycle:v1`.

Evidence shipped in this change:

- Typed Rust model and validator for profile cards, switch review sheets,
  temporary-profile lifecycle state, portable artifact boundaries,
  import/sync conflict rows, rollback audit rows, sync fallback rows, and
  cross-surface truth rows.
- JSON Schema at `schemas/settings/profile-switch-review.schema.json`.
- Deterministic fixtures covering daily profile switching, temporary
  troubleshooting state, local-authoritative sync fallback, refused authority
  widening, and a narrowed missing-checkpoint drill.
- Fixture tests proving restart-delta truth, session-only state visibility,
  `Discard` / `Promote` / `Compare to durable profile` actions, secret-safe
  artifact boundaries, non-widening conflict review, rollback checkpoint
  creation, local-authoritative file portability, and shared surface truth.

Stable lanes must consume this record rather than cloning profile semantics in
settings, shell, sync, help, support, or export prose.
