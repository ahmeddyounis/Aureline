# Review Workspace Seed Alpha Fixtures

Protected fixtures for the local review-workspace seed projection.

- `local_diff_with_work_item_link.yaml` covers opening a diff inside a
  review workspace, assigning deterministic anchors to each protected
  row, and projecting a work-item relation with actor and command
  attribution.

The fixture uses local Git truth and opaque work-item refs only. Hosted
provider overlay fields can be attached later without changing the row
anchor IDs.
