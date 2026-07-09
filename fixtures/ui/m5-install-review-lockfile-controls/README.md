# M5 install-review sheet and lockfile-impact card fixtures

Protected fixtures for the `install_review_sheet` and `lockfile_impact_card`
components implemented in
`aureline_deps::implement_install_review_sheets_and_lockfile_impact_cards`.

Each fixture is an export-safe `InstallReviewLockfileControlsPacket` that
validates against
[`schemas/ui/m5-install-review-lockfile-controls.schema.json`](../../../schemas/ui/m5-install-review-lockfile-controls.schema.json)
and passes `InstallReviewLockfileControlsPacket::validate`.

- `broad_peer_conflict.json` — spotlights a broad remove that must resolve a peer
  conflict and regenerates several lockfiles; the sheet's change breadth is
  derived, so it can never read as a small isolated change.
- `regenerate_broad_churn.json` — a regenerate-from-source lockfile with broad
  churn answered from an offline snapshot; the write mode and rollback posture
  stay consistent and churn is never understated.

Regenerate with:

```
GEN_INSTALL_REVIEW_LOCKFILE_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  implement_install_review_sheets_and_lockfile_impact_cards::tests::generate_artifacts
```
