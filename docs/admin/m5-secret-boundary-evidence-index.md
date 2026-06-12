# M5 Secret Boundary Evidence Index

This page is the admin-docs projection for
`artifacts/security/m5/m5-secret-boundary-depth.json`.

- Matrix id: `m5.secret_boundary.depth.v1`
- Evidence index ref: `evidence_index:m5.secret_boundary.depth.v1`
- Canonical contract doc: `docs/security/m5/m5-secret-boundary-depth.md`
- Canonical artifact: `artifacts/security/m5/m5-secret-boundary-depth.json`

## Admin Projection Rules

- Admin docs MUST reuse the canonical `qualification_rows` and
  `consumer_projections`.
- Admin docs MUST preserve `qualification_row_id`, `matrix_row_id`,
  `deployment_profile`, `displayed_label`, `proof_freshness`, `narrow_reason`,
  and `rationale`.
- Admin docs MUST NOT widen a row beyond the packet's `displayed_label`.
- Admin docs MAY link to the checked `qualification_packet` and `proof_index_ref`
  for audit or support review.

## Checked Packet Counts

- 48 total qualification rows
- 30 `qualified_current`
- 8 `limited_local_continuity`
- 10 `support_review_only`

Rows that lack a current checked M5 proof packet remain admin-visible, but only
as `support_review_only` until current proof and repair/export parity are
present in the canonical packet.
