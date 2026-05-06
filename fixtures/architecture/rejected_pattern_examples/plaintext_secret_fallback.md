# Example: plaintext secret fallback proposal (rejected)

## Proposal summary

When the OS keychain is unavailable, persist provider tokens in a plaintext config file so workflows keep working across restarts.

## Rejection anchor

- Rejected pattern: `rp.plaintext_secret_fallback`
- Ledger: `artifacts/architecture/rejected_pattern_rows.yaml`

## Why this is rejected (short)

Persisting long-lived credentials in plaintext expands compromise blast radius and violates the trust-store and redaction posture; the acceptable fallback is visible degradation and/or session-only in-memory tokens.

## Governing refs (starting points)

- `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
- `.t2/docs/Aureline_Technical_Architecture_Document.md` (rejected patterns table)

## What would be required to reopen

Name and satisfy a revisit trigger:

- Trigger: `rt.legal_or_policy_change_forces_storage_change` (or another trigger that explicitly authorizes durable credential storage changes)
  - Required artifacts: `adr` + `verification_packet`
  - Forums: `security_trust_review` + `release_council` + `architecture_council`

Concrete minimum packet expectations:

- Verification packet includes threat model delta, redaction posture, and recovery/rollback story for any durable credential record.
- ADR defines the new storage class, retention posture, and the explicit user-visible disclosures required.

