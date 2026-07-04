# M5 handler-ownership primitive fixtures

Protected fixtures for the reusable **handler-ownership primitive** — the ownership / precedence
disclosure card, the channel-association review rows for file / protocol / recent-item /
notification handlers, and the recovery-alignment block that resolve from one handler-ownership
context and share one ownership identity (task M05-832).

The primitive *narrows* the handler-ownership and channel-association truth of the frozen
[deployment/continuity component matrix](../../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
(`channel_association_review_row` and the install-profile handler-ownership descriptors) into one
working resolver:

- **AC1** — a side-by-side install can always explain which build owns file associations and why:
  the disclosure card names the owning install, its owner class, the precedence state, and a
  precise ownership reason.
- **AC2** — handler changes stay previewable and reversible instead of silent takeovers: every
  review row keeps bounded keep / reassign / cancel actions and a preview for proposed changes.
- **AC3** — support packets preserve handler ownership and precedence truth: every system-open /
  deep-link / recent-item / notification recovery path resolves to the disclosed owner and carries
  the rollback identity.

## Files

- `support_export.json` — byte-identical copy of the canonical release proof at
  `artifacts/release/m5-handler-ownership-primitive-proof/support_export.json`.
- `matrix.csv` — one row per handler surface family.

## Source of truth

Both files are emitted from the in-crate seeded builder `seeded_m5_handler_ownership_packet()` in
`crates/aureline-install/src/implement_the_m5_handler_ownership_disclosure_and_channel_association_review_primitive/`.
Do not hand-edit; regenerate from the builder so the packet, the checked-in release proof, and
these fixtures stay byte-aligned. The boundary carries only opaque refs and typed class tokens —
never raw config bytes, credentials, handler URIs, or registry paths.
