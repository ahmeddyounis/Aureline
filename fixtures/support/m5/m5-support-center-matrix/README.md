# Fixtures: M5 Support Center matrix

This directory contains fixture metadata for the `m5_support_center_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/support/m5/m5-support-center-matrix.json`

It is the one authoritative Support Center contract; the typed model and fail-closed gate live in the
`aureline-support` crate (`m5_support_center_matrix`).

## Coverage

- All twelve Support Center modules — `doctor`, `safe_mode`, `bisect`, `performance`, `language`,
  `index`, `ai_usage`, `crash`, `network`, `artifacts`, `issue_report_crash_intake`, and
  `support_bundle_export_preview` — carry exactly one row. No module inherits a posture from an
  adjacent one.
- Each row binds the module to a subset of the one canonical inspector vocabulary
  (`environment_status`, `precedence_inspector`, `crash_intake`, `install_advisory_state`,
  `credential_state`, `export_consent`), declares its support data classes (`metadata_only`,
  `environment_adjacent`, `code_adjacent`, `high_risk`), a redaction default, and the export modes
  (`local_save`, `team_share`, `formal_support`) it offers.
- Published readiness covers `operational` (Doctor, Crash, issue-report/crash-intake, support-bundle
  export preview), `degraded` (Safe mode, Bisect, Language), `inspect_only` (Performance published;
  Index and AI-usage narrowed), and `unavailable` (Network, Artifacts). The publication decision
  covers `published`, `narrowed`, and `withheld`.
- Evidence freshness covers `current`, `aging` (Safe mode), `expired` (Index), and `missing`
  (Artifacts). Inspector availability covers `available`, `degraded` (Bisect's install/advisory), and
  `unavailable` (Network's environment-status). Consent state covers `granted`, `required_not_granted`
  (Language's formal-support), and `blocked` (AI-usage's formal-support).
- The four downgrade reasons — `evidence_stale`, `inspector_degraded`, `inspector_unavailable`, and
  `consent_unsatisfied` — are each exercised, and the five recovery paths — `refresh_evidence`,
  `restore_inspector`, `resolve_consent`, `withhold_module`, and `none` — are each exercised.
- Redaction defaults cover all five postures, with `excluded_always` guarding the two high-risk
  modules (AI-usage and the support-bundle export preview).
- The gate is exercised in every direction: four modules publish a full `operational` claim with
  current evidence, available inspectors, and granted consent, proving the gate is not a blanket
  downgrade; the performance inspector publishes cleanly at its designed `inspect_only` level; Safe
  mode narrows on aging evidence; Bisect narrows on a degraded inspector; Language and AI-usage narrow
  on unsatisfied consent; Index narrows on expired evidence; and Network and Artifacts are withheld
  with no offered actions. Each row's `published_readiness`, `module_publication`,
  `downgrade_reasons`, and `downgrade_path` equal the recomputed gate, so the desktop-shell,
  CLI/headless, Help/About, shiproom, and formal-support-handoff surfaces ingest one packet and a
  narrowed module cannot stay green by inertia.
