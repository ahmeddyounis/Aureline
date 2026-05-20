# handoff_and_repro_preview — M3 fixture corpus

Positive and negative fixtures for the public/private handoff-review and
reproduction-packet preview contracts published in M3. The corpus is loaded by
the `handoff_review_preview_fixtures` test in
`crates/aureline-shell/tests/`.

## Layout

```
positive/    Handoff-review sheets that MUST validate.
negative/    Handoff-review sheets that MUST fail validation with a typed reason.
```

Each fixture is a JSON serialization of the
`aureline_shell::handoff_review::HandoffReviewSheet` record, which bundles a
`handoff_target_review_record`, a `repro_packet_preview_record`, and the
draft-continuity block. Adding a new positive case adds a passing row; adding a
negative case asserts the contract still rejects that drift.

The two constituent records are governed by
`schemas/public/handoff_target_review.schema.json` and
`schemas/public/repro_packet_preview.schema.json`; the contract narrative is
`docs/public/m3/handoff_and_repro_boundary.md`.

## Drift axes covered

- **Target visibility** — `Official public`, `Official private`,
  `Security disclosure`, `Community`, `Third-party / vendor`. A route may only
  target a visibility from its allowed set, so the lanes never blur together or
  coerce the user into a public target by accident.
- **Handoff route** — public issue, security disclosure, docs feedback,
  RFC/discussion, community support.
- **Network/browser requirement** — offline capture/preview, public-browse and
  authenticated system-browser, encrypted security channel, vendor call.
- **Data-exit boundary** — reused from the About/help/community vocabulary;
  security disclosure carries `security_payloads_only`, public/community never
  carries a security- or support-scoped exit.
- **Redaction posture** — `fully_redacted_public_safe`, `redacted_support_scoped`,
  `security_channel_only`, `metadata_refs_only`. The posture must be safe for
  the chosen target visibility.
- **Preview-before-share** — a handoff only opens the system browser after the
  reproduction-packet preview is confirmed.
- **Draft continuity** — browser-blocked, offline, and policy-denied outcomes
  preserve the draft text, attachments, target class, and redaction posture with
  export and save actions instead of silent loss.

## Positive cases

- `public_issue_handoff` — public issue opened after preview confirmation.
- `security_disclosure_private_channel` — security disclosure routed to the
  private encrypted channel, never the public tracker.
- `offline_blocked_preserves_draft` — community-support handoff hits an offline
  failure and preserves everything with export/save/retry/discard.
- `docs_feedback_policy_denied` — managed policy denies the handoff; the draft
  and target class are preserved with export/save/copy-refs.

## Negative cases

- `security_route_coerced_to_public` — security route pointed at a public target.
- `public_target_with_security_redaction` — security-scoped redaction on a
  world-readable target.
- `blocked_handoff_discard_only` — a blocked handoff offers only discard.
- `shared_without_preview_confirmation` — browser opened before the preview was
  confirmed.
- `missing_safe_fallback` — handoff target offers no safe fallback route.
- `preserved_visibility_mismatch` — preserved draft records a different target
  class than the one chosen.
- `handoff_missing_build_context_export` — handoff lane attaches no versioned
  build-context export block.
- `unredacted_attachment` — a packet attachment is not redacted before share.
