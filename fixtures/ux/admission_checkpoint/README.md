# Admission Checkpoint Route Fixtures

These fixtures exercise the post-entry admission checkpoint and first-useful-work
route record consumed by `aureline-workspace::admission::checkpoint` and projected
by `aureline-shell::start_center::first_useful_work`.

Each fixture preserves:

- source-labeled archetype or prerequisite evidence;
- distinct `blocking_now`, `recommended_soon`, and `optional_later` buckets;
- same-weight `set_up_later`, `open_minimal`, and `dismiss_recommendation`
  bypasses when setup is proposed;
- no automatic install and no automatic trust;
- entry intent through the first landing route.

The mixed-workspace fixture also proves explicit boundary choices:
`open_whole_repo`, `open_probable_project`, `open_current_folder_only`, and
`create_workset_or_slice`.
