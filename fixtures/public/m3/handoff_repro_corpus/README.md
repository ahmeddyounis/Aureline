# handoff_repro_corpus — M3 handoff & reproduction-packet proof lane

Release-engineering / public-proof corpus that makes Aureline's public/private
handoff behavior **measurable**: it proves the marketed beta handoff rows cannot
quietly leak context, lose reproduction data, or mislabel
support/community/security boundaries. Each drill is one
`handoff_repro_drill_record` that embeds a real
`aureline_shell::handoff_review::HandoffReviewSheet` plus the continuity-flow and
support-boundary-copy projections this lane audits.

The lane is driven by `ci/check_handoff_repro_corpus.py` (run it via
`scripts/ci/run_handoff_repro_corpus.sh`). The validator is an independent Python
port of the Rust `HandoffReviewSheet` model, so a regression in either the model
(`crates/aureline-shell/src/handoff_review/mod.rs`) or a fixture fails the lane.

## Layout

```
accept_*.json   Handoff drills that MUST validate end-to-end. One per claimed beta row.
reject_*.json   Handoff drills that MUST be rejected, each with a typed expected reason.
corpus_matrix.json        Enum-only matrix: claimed-row → packet map, accept/reject cases.
export_parity_packet.json Support-bundle + CLI/headless projections per accept drill.
```

The two constituent records inside each `review_sheet` are governed by
`schemas/public/handoff_target_review.schema.json` and
`schemas/public/repro_packet_preview.schema.json`; the contract narrative is
`docs/public/m3/handoff_and_repro_boundary.md`.

## What every drill proves

For each **accept** drill the validator schema-validates both constituent
records, re-runs the model port, re-derives the continuity carried-identity and
the support-boundary copy from the review sheet and drift-checks the stored
values, then proves export parity. For each **reject** drill it proves the
documented drift is actually caught, with the expected typed reason.

- **Exact target identity & visibility.** Every handoff pins a typed route and
  one of the five visibility classes (`official_public`, `official_private`,
  `security_disclosure`, `community`, `third_party_vendor`). A route may only
  target a visibility from its allowed set.
- **Preview before share.** The system browser only opens after the
  reproduction-packet preview is confirmed.
- **Redaction posture safe for the target.** A world-readable target may not
  carry a support- or security-scoped redaction posture.
- **Exact-anchor / build-context continuity.** The redaction posture, target
  visibility, exact anchor / object identity, versioned build-context export
  blocks, and the previewed-shareable diagnostic / attachment set survive every
  `prepared → preview_confirmed → block → retry → export → reopen` stage. A field
  the preview omitted is never exported.
- **Browser-blocked / offline preservation.** Blocked, offline, and
  policy-denied handoffs preserve the draft, attachments, target class, and
  redaction posture with export / save actions — never silent loss.
- **Support-boundary copy honesty.** The reviewer-facing copy is derived from the
  target and can never describe a private / security route as world-readable.

## Accept drills (one per claimed beta row)

- `public_issue_after_preview` — public issue opened only after preview
  confirmation; the exact anchor and redacted export survive export and reopen.
- `docs_feedback_browser_blocked` — a blocked browser preserves the prepared
  docs-feedback context across retry, export, and reopen.
- `community_support_offline_capture` — the packet is built and previewed offline
  without leaving the product, then preserved for later publish.
- `private_support_intake_authenticated` — a private support request routes to
  the authenticated plane and is never world-readable.
- `security_disclosure_encrypted_channel` — a security report routes only to the
  encrypted private channel and can never be coerced to a public target.
- `rfc_discussion_policy_denied` — a managed-policy denial preserves the RFC
  proposal context with export and save.

## Reject drills (one per documented drift)

- `security_route_coerced_to_public` — `route_visibility_mismatch`.
- `public_target_with_security_redaction` — `redaction_posture_unsafe_for_visibility`.
- `preview_omits_but_exports` — `continuity_preview_export_mismatch` (a field
  hidden from the review sheet is still exported).
- `exact_anchor_loss_on_reopen` — `continuity_anchor_drift`.
- `blocked_handoff_context_loss` — `continuity_blocked_context_loss`.
- `support_boundary_copy_drift` — `support_copy_world_readable_drift` (and
  `support_copy_visibility_drift`, `support_copy_label_drift`).

## Regenerating

```sh
python3 ci/check_handoff_repro_corpus.py --repo-root . --write
```

This re-mints every `accept_*.json` / `reject_*.json` drill, `corpus_matrix.json`,
and `export_parity_packet.json` from the model. Keep fixtures privacy-cleared and
synthetic — no real customer or user content rides this corpus.
