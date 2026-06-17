# M5 Supportability Qualification Review

This review packet certifies the Support Center, environment explainability,
precedence inspection, support-bundle consent, crash-intake/recovery, and
supportability-handoff surfaces for every claimed M5 profile across the desktop
and CLI/headless deployment modes, using one shared qualification index.

## Evidence

| Evidence | Path |
| --- | --- |
| Rust packet | `crates/aureline-support/src/m5_supportability_qualification/mod.rs` |
| Boundary schema | `schemas/support/m5-supportability-qualification.schema.json` |
| Reviewer doc | `docs/help/support/m5-supportability-qualification.md` |
| Canonical fixture | `fixtures/support/m5/m5-supportability-qualification/packet.json` |
| Shiproom claim packet | `artifacts/shiproom/m5-supportability-claim-packet/m5_supportability_claim_packet.md` |
| Support Center matrix | `schemas/support/m5-support-center-matrix.schema.json` |
| Environment explainability | `schemas/runtime/m5-environment-status-strip.schema.json` |
| Precedence inspection | `schemas/support/m5-precedence-inspector.schema.json` |
| Support-bundle consent | `schemas/support/m5-support-bundle-consent.schema.json` |
| Crash intake / recovery | `schemas/support/m5-crash-intake.schema.json` |
| Supportability handoff | `schemas/support/m5-supportability-handoff-packets.schema.json` |

## Review Findings

| Area | Result |
| --- | --- |
| Canonical qualification index | Every claimed supportability surface and profile binds one row across the desktop and CLI/headless deployment modes in one checked packet (30 rows). |
| Own-proof discipline | Each row cites its own surface's boundary schema, canonical artifact, fixture corpus, and record kind; a row can never borrow an adjacent crash or Doctor surface's proof, so a passing neighbor cannot keep a surface green. |
| Supportability drills | Diagnosis latency, consent-sheet accuracy, export-mode parity, crash-loop recovery, and exact-build intake are bound into the corpus and mapped to the surfaces they back. |
| Local-save first-class | Every send-capable surface (consent, crash-intake, handoff) keeps local-save first-class; a row may never demote local-save below a team-share or formal-support send. |
| Downgrade automation | Stale surface evidence, a stale drill, lost deployment-mode parity, a policy-blocked send, or a missing consumer binding can no longer keep a broad supportability claim green. |
| Shared consumer contract | Help/About, the desktop Support Center, CLI/headless, support export, the shiproom claim packet, and the release manifest all ingest the same packet id and preserve the same row fields verbatim. |
| Export safety | The qualification remains metadata-only and by-reference; raw artifact payloads, raw logs, raw transcripts, and secrets stay outside this boundary. |

## Current posture

- All six supportability surfaces qualify on every claimed M5 profile in the
  canonical index; each surface stands on its own checked-in lane proof rather
  than an adjacent surface's maturity.
- Send-capable surfaces (`support_bundle_consent`, `crash_intake_recovery`,
  `supportability_handoff`) carry `local_save_first = true` on every profile, so
  local-save and self-diagnosis stay first-class even where a profile would
  block a hosted send.
- Degraded fixtures demonstrate the auto-narrowing the claim depends on:
  - `consent_drill_stale_narrowed.json` — a stale consent-sheet-accuracy drill
    narrows the consent and handoff surfaces to `local_self_diagnosis_only`
    while local-save stays first-class.
  - `environment_evidence_stale_blocked.json` — stale environment-explainability
    evidence blocks that surface (`blocked_unverified`) and narrows the Support
    Center that binds it (`limited_profile_scoped`); the precedence, consent,
    crash-intake, and handoff surfaces keep their own fresh proof and stay green.

## Regenerating this evidence

```sh
cargo run -q -p aureline-support --example dump_m5_supportability_qualification_packet -- canonical \
  > fixtures/support/m5/m5-supportability-qualification/packet.json
cargo run -q -p aureline-support --example dump_m5_supportability_qualification_packet -- consent_drill_stale \
  > fixtures/support/m5/m5-supportability-qualification/consent_drill_stale_narrowed.json
cargo run -q -p aureline-support --example dump_m5_supportability_qualification_packet -- environment_evidence_stale \
  > fixtures/support/m5/m5-supportability-qualification/environment_evidence_stale_blocked.json
cargo test -p aureline-support m5_supportability_qualification
```
