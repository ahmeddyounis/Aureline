# Handoff & reproduction-packet corpus report

Reviewer-facing summary of the handoff & reproduction-packet audit lane — the
release-engineering / public-proof packet that proves the marketed beta handoff
rows cannot quietly leak context, lose reproduction data, or mislabel
support/community/security boundaries. Support, docs, and security teams consume
this report (and the parity packet) to validate issue templates, disclosure
flows, and community-handoff copy against actual product behavior. Regenerate the
machine-readable findings with:

```sh
scripts/ci/run_handoff_repro_corpus.sh \
  --report-json artifacts/public/m3/handoff_repro_corpus_report.json
```

- **Drill record:** `handoff_repro_drill_record`
- **Target schema:** `schemas/public/handoff_target_review.schema.json`
- **Packet schema:** `schemas/public/repro_packet_preview.schema.json`
- **Corpus:** `fixtures/public/m3/handoff_repro_corpus/`
- **Matrix:** `fixtures/public/m3/handoff_repro_corpus/corpus_matrix.json`
- **Parity packet:** `fixtures/public/m3/handoff_repro_corpus/export_parity_packet.json`
- **Validator:** `ci/check_handoff_repro_corpus.py`
- **Script:** `scripts/ci/run_handoff_repro_corpus.sh`
- **Contract:** `docs/public/m3/handoff_and_repro_boundary.md`

The validator is an independent Python port of the Rust `HandoffReviewSheet`
model in `crates/aureline-shell/src/handoff_review/`, so a regression in either
the model or a fixture fails the lane. CI and nightly runs catch share-boundary
regressions before beta widening or stable promotion.

## What the lane proves

1. **Exact target identity & visibility.** Every handoff names a typed route and
   one of the five visibility classes; a route can never coerce a disclosure onto
   a public target.
2. **Preview before share.** The system browser only opens after the
   reproduction-packet preview is confirmed.
3. **Redaction posture safe for the target.** A world-readable target may not
   carry a support- or security-scoped redaction posture.
4. **Exact-anchor / build-context continuity.** The redaction posture, target
   visibility, exact anchor / object identity, versioned build-context export
   blocks, and the previewed-shareable diagnostic / attachment set survive every
   prepare → preview → block → retry → export → reopen stage; a field omitted
   from the review sheet is never exported.
5. **Browser-blocked / offline preservation.** Blocked, offline, and
   policy-denied handoffs preserve the draft, attachments, target class, and
   redaction posture instead of losing them.
6. **Support-boundary copy honesty.** Reviewer-facing copy is derived from the
   target and can never describe a private / security route as world-readable.
7. **Export parity.** The support-bundle plaintext and CLI / headless index
   preserve the record semantics, so a reviewer can explain a handoff boundary
   from the exported packet alone — no screenshots or manual reconstruction.

## Accept-drill index (one per claimed beta row)

| Scenario | Claimed beta row | Route | Visibility | World-readable | Redaction | Outcome | Preview confirmed |
| -------- | ---------------- | ----- | ---------- | -------------- | --------- | ------- | ----------------- |
| `public_issue_after_preview` | `beta.row.public.issue_handoff` | public_issue | official_public | true | fully_redacted_public_safe | opened_in_system_browser | true |
| `docs_feedback_browser_blocked` | `beta.row.public.docs_feedback` | docs_feedback | official_public | true | fully_redacted_public_safe | browser_blocked | true |
| `community_support_offline_capture` | `beta.row.community.support_offline` | community_support | community | true | metadata_refs_only | offline | true |
| `private_support_intake_authenticated` | `beta.row.private.support_intake` | community_support | official_private | false | redacted_support_scoped | opened_in_system_browser | true |
| `security_disclosure_encrypted_channel` | `beta.row.security.disclosure` | security_disclosure | security_disclosure | false | security_channel_only | opened_in_system_browser | true |
| `rfc_discussion_policy_denied` | `beta.row.community.rfc_discussion` | rfc_discussion | community | true | metadata_refs_only | policy_denied | true |

## Reject-drill index (one per documented drift)

| Scenario | Expected typed reason | What it proves |
| -------- | --------------------- | -------------- |
| `security_route_coerced_to_public` | `route_visibility_mismatch` | A security route cannot be pointed at a world-readable target. |
| `public_target_with_security_redaction` | `redaction_posture_unsafe_for_visibility` | A public target cannot carry a security-scoped redaction posture. |
| `preview_omits_but_exports` | `continuity_preview_export_mismatch` | A field hidden from the preview is not exported. |
| `exact_anchor_loss_on_reopen` | `continuity_anchor_drift` | The exact anchor / object cannot drift on reopen. |
| `blocked_handoff_context_loss` | `continuity_blocked_context_loss` | A blocked handoff cannot drop the prepared draft. |
| `support_boundary_copy_drift` | `support_copy_world_readable_drift` | Support-boundary copy cannot mislabel a private / security route. |

## Beta scorecard

Every claimed beta handoff row maps to exactly one current accept drill, and
every accept drill maps back to exactly one claimed row (6 rows, 6 accept
drills). The matrix records the claimed-row → packet map and the reject cases;
the parity packet carries a self-contained support bundle per accept drill so a
support, docs, or security reviewer can explain the handoff boundary from the
exported packet alone.

## Coverage

The corpus proves every required axis: the four reachable target visibilities
(official public, official private, security disclosure, community); the
browser-blocked, offline, and policy-denied preservation outcomes; offline
capture-and-preview; and all six rejection drills (route mislabeling,
redaction-preview mismatch, preview/export omission, exact-anchor loss,
blocked-handoff context loss, and support-boundary copy drift). The enum-only
matrix and the export-parity packet are regenerated and drift-checked on every
run.
