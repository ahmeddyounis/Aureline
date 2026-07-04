# M5 handler-ownership primitive

The **handler-ownership primitive** is the reusable handler-ownership / precedence disclosure
card, the set of channel-association review rows for file / protocol / recent-item /
notification handlers, and the recovery-alignment block that About, diagnostics, install-review,
support, notification-center, and docs surfaces ingest instead of cloning a bespoke
desktop-integration panel or forcing users to inspect installer state by hand. One
handler-ownership context resolves into all three surfaces and they share one ownership
identity, so current owner, proposed owner, precedence, user-facing impact, and rollback
identity never blur across them.

It **narrows** the handler-ownership and channel-association truth already claimed by the frozen
[deployment/continuity component matrix](../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
— the `channel_association_review_row` family and the install-profile handler-ownership
descriptors — into one working resolver (`resolve_handler_ownership`) rather than restating
install / integration truth in feature-local prose. It reuses the frozen matrix's operating-mode
and downgrade-trigger vocabulary; it adds only the minted vocabulary the resolver needs (handler
channel class, owner class, precedence state, change state, user-facing impact, bounded
association actions, export field, and the parity surface families).

- **Boundary schema:**
  [`schemas/ui/m5-handler-ownership-primitive.schema.json`](../../schemas/ui/m5-handler-ownership-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-deployment-continuity-component-matrix.schema.json`](../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-handler-ownership-primitive-proof/support_export.json`](../../artifacts/release/m5-handler-ownership-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-handler-ownership-primitive/`](../../fixtures/ui/m5-handler-ownership-primitive/)
- **Implementation:**
  `crates/aureline-install/src/implement_the_m5_handler_ownership_disclosure_and_channel_association_review_primitive/`

## What the resolver projects

`resolve_handler_ownership(&M5HandlerOwnershipInput)` returns a `M5ResolvedHandlerOwnership` with
three surfaces that all carry the same `ownership_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Ownership / precedence disclosure card | `M5ResolvedHandlerOwnershipCard` | the owning install identity, its owner class, the operating mode, the precedence state, the precise ownership reason, the rollback identity, and the guarantee that it stays inspectable without manual installer inspection |
| Channel-association review rows | `Vec<M5ResolvedChannelAssociationReviewRow>` | each channel's class, current owner, proposed owner, change state, user-facing impact, bounded keep / reassign / cancel actions, and — for a proposed change — a preview action |
| Recovery-alignment block | `M5ResolvedRecoveryAlignment` | one recovery path per system-open / deep-link / recent-item / notification channel, each resolving to the disclosed owner and carrying the rollback identity |

## One shared precedence vocabulary

The primitive keeps one vocabulary for handler precedence across UI, docs / help, and support
exports — `Sole owner`, `Primary among installs`, `Shared contested`, `Superseded`, and
`Not registered` — so a side-by-side install can always explain which build currently owns file
associations and why. The disclosure card names the owner class and the precedence state
together with a precise, non-generic ownership reason.

## Acceptance criteria the resolver proves

- **AC1 — a side-by-side install can explain which build owns file associations and why.** The
  disclosure card names the owning install, its owner class, the precedence state, and a precise
  ownership reason, and every channel row discloses its current owner. A card that hides the
  current owner is rejected as `OwnerNotDisclosed`; a missing reason is rejected as
  `OwnershipReasonMissing`; a card that requires manual installer inspection is rejected as
  `RequiresManualInstallerInspection`.
- **AC2 — handler changes are previewable and reversible instead of silent takeovers.** Every
  review row keeps bounded keep / reassign / cancel actions, names the proposed owner and
  user-facing impact, and — when a change is proposed — is previewable and reversible. A row that
  silently captures a default handler is rejected as `SilentTakeover`; an unreviewed change as
  `ChannelChangeNotReviewed`; a change that is not previewable or reversible as
  `ChangeNotPreviewable` / `ChangeNotReversible`.
- **AC3 — support packets preserve handler ownership and precedence truth.** The card keeps a
  rollback identity, and every system-open / deep-link / recent-item / notification recovery path
  resolves to the disclosed channel owner and carries the rollback identity. A recovery path that
  routes away from the disclosed owner is rejected as `RecoveryPathMisaligned`; a change forced
  without a rollback identity as `EmptyRollbackIdentityRef`.

## Honesty guarantees

- Raw config bytes, credentials, license keys, handler URIs, registry paths, and device
  identifiers never cross this boundary; the resolver carries only opaque refs, typed class
  tokens, booleans, and redacted labels.
- A degraded input must carry a precise, non-generic label; a generic non-answer (`unavailable`,
  `error`, `contested`… when reduced to a bare word) is rejected.
- The support / export packet reconstructs exactly what each surface would have shown: every
  worked case stores both its input and its resolved projection, and validation re-runs the
  resolver so a stored projection can never drift from the live resolver.

## Parity matrix

The `M5HandlerOwnershipPrimitivePacket` binds each of the six handler surface families (About
integration, diagnostics handlers, install review, support / export replay, notification center,
docs handler reference) to the shared contract with worked resolution cases, a frozen
controlled-vocabulary set, governance-review and consumer-projection blocks, and a release /
support parity posture. See the
[matrix CSV](../../artifacts/release/m5-handler-ownership-primitive-proof/matrix.csv) and
[report](../../artifacts/release/m5-handler-ownership-primitive-proof/report.md) for the
per-surface summary.
