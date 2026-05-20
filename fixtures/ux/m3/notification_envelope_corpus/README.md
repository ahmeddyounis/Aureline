# Durable-notification routing corpus

Reviewable fixtures for the comprehensive beta attention lane built on the
governed attention router in
[`crates/aureline-shell/src/attention_router/`](../../../../crates/aureline-shell/src/attention_router/).

Where the seeded corpus under
[`../notification_routing/`](../notification_routing/) is a representative
cross-surface coverage set, this corpus proves that *every beta job or alert
family that claims durable attention truth* routes correctly under look-away,
lock-screen, companion fanout, presentation/follow, focus, screen-reader,
quiet-hours, and stale-target conditions.

Each JSON file is a literal projection of the seeded
`NotificationEnvelopeCorpusPacket` produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_notification_envelope_corpus.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_notification_envelope_corpus.rs)).
The inspector is the only mint-from-truth path for these fixtures, so the
checked-in JSON cannot drift from the Rust types. Every routed outcome is a
`notification_route_outcome_record` that conforms to the boundary schema at
[`schemas/ux/notification_route_outcome.schema.json`](../../../../schemas/ux/notification_route_outcome.schema.json).

All records carry the shared contract ref
`shell:notification_envelope_corpus:v1` so shell rows, the headless CLI rows,
and the support-export rows pivot to the same `case_id` and `route_outcome_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`packet.json`](./packet.json) | The full packet: coverage summary, cases, drift drills, badge probes, overlay parity, and support export. |
| [`cases.json`](./cases.json) | One worked routing case per beta attention family (indexing, restore, install/update/download, AI approvals, provider sync, policy change, remote reconnect, managed alert, classroom/presentation overlays) with its live posture, resolved outcome, and reopen proof. |
| [`drift_drills.json`](./drift_drills.json) | Adversarial regressions — wrong-target reopen, lock-screen leakage, badge inflation, quiet-hours drift — each constructed from a real outcome and shown to fail the conformance lane, with a structured diff. |
| [`badge_probes.json`](./badge_probes.json) | Retry-storm badge-integrity probes proving the deduped durable count never inflates and the OS app-icon / lock-screen visibility honors quiet-hours posture. |
| [`overlay_parity.json`](./overlay_parity.json) | Presentation, follow, focus, and screen-reader overlays proving the claimed durable rows and reopen target never diverge from the baseline route. |
| [`support_export.json`](./support_export.json) | Support route/outcome export that reconstructs class, route, suppression, and resolution from stable enums. Raw user-facing copy is excluded by construction. |

## What the corpus proves

- **Family coverage.** Every beta attention family has a worked routing case
  with its expected route and actionability outcome.
- **One alert, every surface.** The same envelope resolves consistently across
  `durable_job_row`, `status_item`, `activity_center_digest_card`,
  `contextual_banner`, `toast`, `os_notification`, `lock_screen_summary`, and
  `companion_push`.
- **Exact-target reopen.** Every resolved surface keeps the single
  `reopen_target_ref`; a stale or missing target reopens a truthful placeholder
  (`truthful_placeholder`) or a revalidation requirement
  (`denied_requires_revalidation`) — never a generic home view.
- **Drift drills fail the lane.** Wrong-target reopen, lock-screen leakage,
  badge inflation, and quiet-hours drift each produce an actionable diff and a
  stable rejection reason token.
- **Overlay parity.** Presentation, follow, focus, and screen-reader postures
  dim audience-visible surfaces without changing the claimed durable rows or the
  reopen target.
- **Export-safe truth.** The support export carries stable enums and per-surface
  resolution rows, never raw user-facing message text.

## Fixture rules

- Regenerate the fixtures and the three artifacts only by the headless
  inspector:

  ```sh
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- packet > fixtures/ux/m3/notification_envelope_corpus/packet.json
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- cases > fixtures/ux/m3/notification_envelope_corpus/cases.json
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- drift-drills > fixtures/ux/m3/notification_envelope_corpus/drift_drills.json
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- badge-probes > fixtures/ux/m3/notification_envelope_corpus/badge_probes.json
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- overlay-parity > fixtures/ux/m3/notification_envelope_corpus/overlay_parity.json
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- support-export > fixtures/ux/m3/notification_envelope_corpus/support_export.json
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- audit-md > artifacts/ux/m3/notification_privacy_and_route_audit.md
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- export-report-md > artifacts/support/m3/notification_route_outcome_export_report.md
  cargo run -q -p aureline-shell --bin aureline_shell_notification_envelope_corpus -- conformance-md > docs/ux/m3/notification_route_conformance.md
  ```

- The replay test
  [`crates/aureline-shell/tests/notification_envelope_corpus_fixtures.rs`](../../../../crates/aureline-shell/tests/notification_envelope_corpus_fixtures.rs)
  fails if the JSON drifts from the seeded packet, if a family is dropped, if any
  outcome loses its reopen target or durable truth, if a drift drill stops
  failing the lane, if a badge probe inflates, if an overlay diverges, if the
  support export leaks summary copy, or if the rendered artifacts drift.
