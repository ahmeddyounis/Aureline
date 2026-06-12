# M5 Secret Boundary Evidence Index

This page is the Help/About truth-source projection for
`artifacts/security/m5/m5-secret-boundary-depth.json`.

- Matrix id: `m5.secret_boundary.depth.v1`
- Evidence index ref: `evidence_index:m5.secret_boundary.depth.v1`
- Canonical contract doc: `docs/security/m5/m5-secret-boundary-depth.md`
- Canonical artifact: `artifacts/security/m5/m5-secret-boundary-depth.json`

## Consumer Contract

- Help/About MUST reuse the packet's `qualification_rows` verbatim.
- Help/About MUST display `displayed_label`, never `claimed_label`.
- Help/About MUST preserve `qualification_row_id`, `matrix_row_id`,
  `deployment_profile`, `proof_freshness`, `narrow_reason`, and `rationale`.
- Help/About MUST NOT widen `limited_local_continuity` or
  `support_review_only` rows into a generic connected state.

## Checked Packet Counts

- 48 total qualification rows
- 30 `qualified_current`
- 8 `limited_local_continuity`
- 10 `support_review_only`

The current checked packet narrows rows for two reasons only:

- `profile_local_continuity_only` for `mirror_offline` continuity rows
- `proof_packet_missing` for rows that do not yet carry a current checked M5
  proof packet
