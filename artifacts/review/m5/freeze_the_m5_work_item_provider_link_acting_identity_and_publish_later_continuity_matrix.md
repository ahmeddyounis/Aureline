# M5 Provider-Work-Item Governance Matrix

- Packet: `m5-provider-workitem-governance:stable:0001`
- Schema: `schemas/review/freeze-the-m5-work-item-provider-link-acting-identity-and-publish-later-continuity-matrix.schema.json`
- Support export: `artifacts/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix/support_export.json`
- Contract doc: `docs/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix.md`
- Fixtures: `fixtures/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix/`

## Coverage

- The canonical object vocabulary is qualified Stable: provider-backed work items, review-linked change intent, typed browser handoff packets, deferred publish packets, imported snapshots, and provider-event envelopes reuse one governed object model.
- Provider-linked mutation is qualified Stable: preview hash, target object, acting identity, effective scope, and downgrade posture remain explicit before any hosted mutation.
- Acting identity and effective scope are qualified Stable: human account, installation grant, delegated credential, browser-only fallback, denied scope, and publish-later local draft remain separate inspectable rows.
- Browser handoff continuity is qualified Stable: handoff remains typed, reasoned, attributable, and return-anchor safe.
- Deferred publish continuity is qualified Stable: local drafts, queued publishes, retry, cancel, and export-safe packet flows stay explicit across restart and reconnect.
- Provider event reconciliation is qualified Beta: callback/webhook import, replay ledger, and deny/audit paths are governed and typed, but remain narrowed until every claimed provider family proves current reconciliation evidence.
- Every lane carries required evidence packet refs, downgrade triggers, rollback posture, and consumer-surface parity.
- Proof freshness SLO is 168 hours with automatic narrowing on stale authority, stale reconciliation proof, and stale publish-later continuity proof.

## Trust guardrails

The matrix proves that provider-owned objects never masquerade as local canonical truth, imported snapshots never claim provider-committed freshness, acting identity and effective scope stay visible on claimed lanes, and local drafts, queued publishes, provider-committed state, and callback-denied audit state remain visibly distinct. Browser handoff preserves reason and return anchor, deferred publish survives restart and reconnect, and callback events only mutate through typed, deduplicated reconciliation pathways. Stale authority, stale reconciliation proof, or stale publish-later proof narrows the claim rather than hiding the lane.
