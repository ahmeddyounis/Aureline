# Difficult save-target-review cases

Reviewer-facing fixtures for the hardest filesystem cases the
path-truth chip and the alias inspector must stay honest across.

These fixtures complement the existing corpora rather than
replacing them:

- `fixtures/fs/save_truth_cases/` describes full save inputs and
  expected save-pipeline outcomes;
- `fixtures/fs/alias_and_symlink_cases/` isolates the identity
  and disclosure facts a later editor/search surface reuses;
- this directory holds the **hardest fixtures** — the ones the
  chip and the inspector view must explain without relying on
  UI screenshots as the only explanation.

Every file here is one `save_target_review_case` that registers a
stable `case_id` in
[`../identity_corpus_manifest.yaml`](../identity_corpus_manifest.yaml)
under the `difficult_save_review_cases` section. Each case joins
to:

- a chip artifact at
  [`/artifacts/fs/path_truth_examples/`](../../../artifacts/fs/path_truth_examples/), and
- an alias-inspector artifact at
  [`/artifacts/fs/alias_inspector_examples/`](../../../artifacts/fs/alias_inspector_examples/).

The three files share the same `corpus_case_id`; reviewers join
on that id and never on filename patterns.

## Index

| Case id | Difficulty axis | Row truth | Save assertion | Expected review action |
|---|---|---|---|---|
| `corpus.fs.difficult.case_only_rename_with_collision` | `case_only_rename` | `exact` | `degraded` | `open_rename_preview` |
| `corpus.fs.difficult.symlink_target_shift_mid_session` | `symlink_target_shift` | `stale` | `degraded` | `review_and_approve` |
| `corpus.fs.difficult.managed_overlay_review_required` | `overlay_managed_root` | `imported` | `degraded` | `review_and_approve` |
| `corpus.fs.difficult.whole_file_rewrite_fallback` | `whole_file_rewrite_fallback` | `heuristic` | `degraded` | `save_with_compare_before_write` |
| `corpus.fs.difficult.archive_inner_alias_inspect_only` | `archive_inner_alias` | `exact` | `unsupported` | `save_as` |
| `corpus.fs.difficult.bind_mount_canonical_drift` | `bind_mount_canonical_drift` | `stale` | `degraded` | `review_and_approve` |

## Schema

Each fixture is one YAML file with the following top-level
fields (shape frozen in
[`/docs/fs/path_truth_packet.md`](../../../docs/fs/path_truth_packet.md)
§3):

- `schema_version: 1`
- `case_id`, `case_family: save_target_review_case`
- `title`, `summary`
- `difficulty_axis` — one of the six axes listed above
- `root_profile` — pass-through of the root capability class
- `presentation_path`, `canonical_filesystem_object`, `alias_set`
  — layer-1/3/4 identity records
- `row_truth_state`, `row_truth_reason`
- `degraded_save_hints` — chip-side hint block
- `save_target_review` — the review block the chip carries
- `expected_chip_artifact`, `expected_inspector_artifact` —
  file paths for the paired artifacts
- `client_can_still_assert`, `client_must_not_assert`
- `related_fixture_ids`

No wall-clock times are encoded; synthetic monotonic tokens
(`mono:HHMM:SS:SS.FRAC`) stand in so reruns are byte-stable.

## Adding a new case

1. Pick a new difficulty axis or add the case under an existing
   one; do not collapse two axes into one fixture.
2. Register the `case_id` in
   `../identity_corpus_manifest.yaml` under
   `difficult_save_review_cases` with the chip and inspector
   artifact paths.
3. Author both companion artifacts
   (`/artifacts/fs/path_truth_examples/...json` and
   `/artifacts/fs/alias_inspector_examples/...json`) carrying
   the same `corpus_case_id`.
4. Keep the `client_can_still_assert` / `client_must_not_assert`
   lists non-empty — they are the reviewer's honesty guardrails.
