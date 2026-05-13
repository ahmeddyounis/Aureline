# Saved Query Privacy Fixtures

These fixtures protect the query-session ledger, saved-query privacy controls,
and export-safe search packet projection in `aureline-search`.

Covered paths:

- `support_redacted_selected_workset.json` proves support export stores query
  hash material only, keeps the live selected-workset scope label, and narrows
  reopen from a wider current workspace back to the captured scope.
- `local_private_replay.json` proves local-only saved queries may retain raw
  query text while staying non-shareable.
- `policy_withheld_docs_handoff.json` proves policy-withheld queries omit raw
  text and hash material and refuse rerun with an explicit denial reason.
