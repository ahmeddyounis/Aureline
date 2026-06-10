# Statement-safety classifier, write-mode bar, and protected-target step-up flows

## Scope

This document describes the canonical M5 qualification packet for the statement-safety classifier, write-mode bar, and protected-target step-up flows in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/add_the_statement_safety_classifier_write_mode_bar_and_protected_target_step_up_flows/mod.rs`
- Schema: `schemas/data/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.schema.json`
- Checked-in packet: `artifacts/data/m5/add-the-statement-safety-classifier-write-mode-bar-and-protected-target-step-up-flows.json`
- Fixtures: `fixtures/data/m5/add_the_statement_safety_classifier_write_mode_bar_and_protected_target_step_up_flows/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Statement-safety classifier | stable | stable | Labels all 18 statement-safety classes before execution, discloses multi-statement posture, object impact, and ambiguity reason, and requires consent tickets for destructive DDL. |
| Write-mode bar | stable | stable | Discloses autocommit risk, explicit transaction scope, rollback posture, write-guarded state, affected schema, and open-mutation-review action before execution. |
| Protected-target step-up flow | stable | stable | Triggers on mutation-class statements, denies only the requested action, emits audit events, and requires consent tickets for destructive DDL. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.

## Redaction and privacy

- Classifier rows do not expose raw statement bodies, bind values, or object names.
- Write-mode bar disclosures are reviewable sentences; no raw SQL or secrets appear.
- Step-up flows do not expose raw credentials, session tokens, or biometric data.
- Consent ticket refs are opaque; raw user identities or approval details do not appear in qualification rows.

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
