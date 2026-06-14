# M5 stability verdicts, flaky-state badges, and quarantine records

This document is the contract for the **stability verdicts** and **quarantine
records** the M5 test-intelligence lane uses to turn repeated attempt outcomes
into governed, evidence-based truth. Where the session-plan / attempt-record
ledger makes the *execution* of a test selection attributable, this contract
makes its *stability* governable: a flaky state is no longer a single opaque
boolean label, and a quarantine is no longer a local mute that disappears from
release and support surfaces.

A row stays trustworthy only if its flaky state is a controlled token backed by a
visible evidence window, if a quarantine carries an owner, an expiry, and a
restore condition that survive export, and if an expired or stale row fails
readiness instead of silently persisting behind a generic green state. This
contract makes all three guarantees structural.

## Source of truth

- Packet type: `StabilityVerdictQuarantinePacket`
  (`crates/aureline-runtime/src/stability_verdicts_quarantines_and_release_visibility/`).
- Boundary schema:
  `schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json`.
- Checked support export:
  `artifacts/testing/m5/stability-verdicts-quarantines-and-release-visibility/support_export.json`.
- Markdown summary:
  `artifacts/testing/m5/stability-verdicts-quarantines-and-release-visibility.md`.
- Protected fixtures:
  `fixtures/testing/m5/stability-verdicts-quarantines-and-release-visibility/`.

Regenerate the canonical export, summary, and fixture after any shape change:

```bash
cargo run -p aureline-runtime --example dump_stability_verdict_quarantine
cargo run -p aureline-runtime --example dump_stability_verdict_quarantine summary
cargo run -p aureline-runtime --example dump_stability_verdict_quarantine fixture
```

## Stability verdicts

A `StabilityVerdictRecord` ties a stable `verdict_id` and a durable
`VerdictSubject` to:

- a controlled `StabilityVerdictState` — `stable`, `suspected_flaky`,
  `confirmed_flaky`, `quarantined`, `known_failing`, `imported_only_unverified`,
  `stale_evidence`, or `unknown_requires_review` — the badge vocabulary, never a
  single opaque boolean;
- a visible `EvidenceWindow` (`observed_attempts` plus the `passed`, `failed`, and
  `inconclusive` counts and the attempt refs that produced them);
- a `StabilityConfidenceClass` (`high`, `moderate`, `low`, `insufficient_evidence`);
- a `VerdictEvidenceProvenance` (`local_authoritative`, `remote_authoritative`,
  `notebook_authoritative`, or `imported_read_only`);
- an `owner_ref`, a `ReleaseVisibilityClass`, and a `ReadinessImpactClass`.

The packet validation requires the `stable`, a flaky (`suspected_flaky` or
`confirmed_flaky`), `quarantined`, and `imported_only_unverified` states to each
be represented, so the badge vocabulary is exercised, not merely declared.

Identity and evidence rules every verdict obeys (`StabilityVerdictRecord::is_valid`):

- **A subject's fingerprint is never its bare id.** Each `VerdictSubject` carries a
  `subject_fingerprint_token` distinct from its `subject_id`
  (`fingerprint_substitutes_identity`).
- **Templates stay distinct from invocations.** A subject carries a
  `DurableTestNodeKind`; the packet requires both `parameterized_template` and
  `concrete_invocation` to appear (`template_collapsed_with_invocation`).
- **Badges stay evidence-based.** The controlled state must agree with its window
  (`verdict_state_window_mismatch`): a `stable` verdict has no failures, a
  `confirmed_flaky` verdict has both passes and failures, a `known_failing`
  verdict has failures and no passes.
- **Imported never reads as local** (`imported_verdict_reads_as_local`): an
  imported verdict is `imported_only_unverified`, carries `imported_read_only`
  provenance and an `origin_provider_ref`, and may never be `stable`; a local
  verdict carries none of these markers.

## No green over stale or quarantine

Only a locally verified `stable` verdict may carry the `no_impact` readiness
class, and only when its confidence supports a green roll-up and its release
visibility is `informational_recovered`
(`StabilityVerdictRecord::readiness_impact_consistent`). Every flaky, quarantined,
known-failing, imported-only, stale, or unknown row must carry `fails_readiness`
or `narrows_claim` and stay visible as release debt
(`green_over_stale_or_quarantine`).

## Quarantine records

A `QuarantineRecord` ties a stable `quarantine_id` and a `verdict_ref` to:

- a `QuarantineTreatmentKind` (`mute` or `quarantine`);
- a `QuarantineState` (`active`, `expired_reopened`, `resolved`, `renewed`);
- a `QuarantineReason`, an `owner_ref`, a `created_at`, an `expires_at`, a
  `RestoreConditionClass`, a `ReleaseVisibilityClass`, and a `ReadinessImpactClass`.

Quarantines stay visible, filterable, countable, and exportable rather than
collapsing into local muting:

- **A suppressing record stays visible as debt** (`quarantine_silently_muted`): an
  `active`, `expired_reopened`, or `renewed` record may never carry
  `informational_recovered` visibility or `no_impact` readiness.
- **Expiry fails readiness** (`expired_quarantine_persists_silently`): an
  `expired_reopened` record carries a `reopened_attempt_ref`, `release_blocking`
  visibility, and `fails_readiness` impact.
  `QuarantineRecord::evaluated_at` performs that flip when an active record's
  `expires_at` has passed, reopening its scope so it cannot silently persist as
  local state.
- **Bindings resolve both ways**: every `quarantined` verdict binds a
  `quarantine_ref` to a record present in the packet whose subject matches
  (`quarantine_binding_unresolved`, `quarantine_subject_mismatch`), and every
  quarantine's `verdict_ref` resolves to a verdict in the packet
  (`quarantine_verdict_unresolved`).

The packet validation requires at least one `expired_reopened` record so the
expiry-fails-readiness drill is exercised.

## Release visibility and readiness

`ReleaseVisibilityClass` and `ReadinessImpactClass` travel with every verdict and
quarantine so release and support packets see the same truth the product showed
locally. `StabilityVerdictQuarantinePacket::readiness_blocked` is the gate the
release lane reads: it is blocked whenever any verdict or quarantine carries
`fails_readiness`. The `StabilityConsumerProjection` block records that flaky-state
badges, the quarantine UI, imported-run joins, and release / support exports all
normalize onto these records, and that the readiness gate reads the packet instead
of scraping UI text.

## Boundary discipline

The packet carries only typed class tokens, booleans, opaque ids, fingerprint
digests, and redaction-aware reviewable labels. Raw test source, raw provider
payloads, provider cursors, credentials, host names, and raw artifact bodies never
cross this boundary.
