# Project Entry Review Fixtures

This fixture set exercises the workspace-level entry review builder and
schema at `schemas/workspace/entry_review.schema.json`.

The cases cover:

- open folder and open workspace as distinct reviews;
- clone review with mirror, auth, ref, depth, LFS, submodule, destination,
  and post-clone action disclosures;
- duplicate clone destination collision review;
- add-root review for active workspace widening;
- inspect-only portable-state import review;
- restore review with no side-effect rerun.

Every case builds a `project_entry_review_record` in Rust and asserts that
the review sheet kind, source vocabulary, admission checkpoint route, primary
handoff action, and collision class match the expected values.
