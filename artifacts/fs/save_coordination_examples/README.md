# Save-coordination examples

Compact reviewer-facing rows for the filesystem identity corpus.

These examples are intentionally smaller than the full save-plan export:

- the VFS prototype export (`artifacts/fs/save_plan_examples/`) records the
  full identity layers, token, manifest, watcher frames, and hook counters;
- this directory highlights the coordination judgment a later UI, support,
  or qualification packet would quote: exact vs degraded vs unsupported,
  what review is required, and what the client can still assert.

Every JSON file cites the same `corpus_case_id` used by
`fixtures/fs/identity_corpus_manifest.yaml` and by the VFS prototype's
scenario export.

Coverage in this seed:

- exact local atomic save
- alias-driven review for case-only and symlinked paths
- hardlink shared-authority disclosure
- Unicode-normalization alias disclosure
- compare-before-write mismatch on local and remote roots
- review-gated and read-only blocked writes
- watch degradation where save correctness still holds
- participant failure before compare-before-write
