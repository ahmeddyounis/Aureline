# Authority tickets, credential projection, and root authority

This document is the reviewer-facing landing page for the beta authority
projection owned by
[`/crates/aureline-policy/src/authority/mod.rs`](../../../crates/aureline-policy/src/authority/mod.rs).
It extends the shell/policy authority-ticket model to credential projection,
privileged debug attach, and policy, trust, or root-authority admin changes.

The machine-readable contracts live at:

- [`/schemas/security/authority_ticket.schema.json`](../../../schemas/security/authority_ticket.schema.json)
  for the full page, ticket, spend-attempt, defect, summary, and support
  export shapes.
- [`/schemas/security/credential_projection.schema.json`](../../../schemas/security/credential_projection.schema.json)
  for standalone credential projection rows.
- [`/schemas/security/root_authority_change.schema.json`](../../../schemas/security/root_authority_change.schema.json)
  for signed or local-root-authorized admin changes.

The source matrix is
[`/artifacts/security/m3/authority_ticket/authority_ticket_matrix.yaml`](../../../artifacts/security/m3/authority_ticket/authority_ticket_matrix.yaml),
and the fixtures are under
[`/fixtures/security/m3/credential_projection_and_privileged_attach/`](../../../fixtures/security/m3/credential_projection_and_privileged_attach/).

## What the projection covers

Every `security_authority_ticket_page_record` carries:

- `security_authority_ticket_record` rows for local mutation, external
  provider mutation, credential projection, privileged debug attach, and
  policy/trust/admin authority classes. Each ticket binds issuer, request
  origin, actor, target, sandbox profile, capability envelope, policy epoch,
  issuance window, use posture, revocation hook, lineage, audit refs, and
  no-secret guardrails.
- `security_credential_projection_record` rows for credential projection
  modes. The seeded page covers delegated-handle, session-only, and sign-only
  projections, each tied back to its admitting ticket and consumer identity.
- `security_root_authority_change_record` rows for signed root-authority
  changes. Each row carries admin actor, target fingerprint, policy epoch,
  source proof, rollback ref, audit refs, and exportability.
- `security_authority_ticket_spend_attempt_record` rows for current-context
  spend evaluation. A spend can admit only when the presented ticket, actor,
  target, sandbox, policy epoch, authority source, expiry, credential
  projection, root proof, and lineage all still match.

## Acceptance posture

- **No ambient credential projection.** Credential projection spends fail
  closed unless a current ticket points to a projection row whose ticket ref,
  consumer identity, projection mode, target, expiry, revocation path, and
  guardrails are still valid. Projection rows carry opaque credential refs
  only; raw credential bytes and plaintext secret material are invalid.
- **Privileged attach requires current authority.** Debug attach and privileged
  inspection spends are admitted only through a current
  `privileged_debug_attach` ticket. Missing ticket, expiry, target drift,
  sandbox drift, policy-epoch drift, authority-source mismatch, and broken
  lineage all deny and require reapproval before replay.
- **Remembered decisions stay narrow.** A remembered local decision compiles to
  a narrow reusable rule plus a renewable short-lived ticket. Credential
  projection, privileged attach, and policy/trust/admin tickets reject
  remembered reusable authority because those classes must be reissued with
  fresh context.
- **Root/admin changes are stronger than settings approvals.** Trust-store,
  signing-root, policy-admin, and governance authority changes require an
  authority ticket with root-change lineage plus signed or local root-authority
  source proof. The root-change row must remain attributable, rollbackable,
  audited, and exportable.
- **Support exports reconstruct lineage without secrets.** The support export
  wrapper preserves ticket ids, actor bindings, target fingerprints, sandbox
  and policy refs, credential projection modes, consumer identities, root proof
  refs, spend outcomes, and audit refs. It excludes raw credentials, raw
  authority bodies, raw policy payloads, raw evidence bodies, and plaintext
  secret material.

## Failure-mode drills

The seed example regenerates the fixtures and drill cases:

- `drill_raw_secret_projection.json` flips a projection guardrail to claim
  plaintext secret material was present. The validator surfaces
  `raw_secret_material_present`.
- `drill_unsigned_root_change.json` replaces a signed root-authority proof
  with `missing_or_unverified`. The validator surfaces
  `root_authority_proof_missing`.
- `drill_admitted_without_ticket.json` flips a missing-ticket privileged
  attach denial to `admitted`. The validator surfaces
  `spend_admitted_without_ticket`.

## Regeneration

```sh
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- page > fixtures/security/m3/credential_projection_and_privileged_attach/page.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- tickets > fixtures/security/m3/credential_projection_and_privileged_attach/ticket_rows.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- credential-projections > fixtures/security/m3/credential_projection_and_privileged_attach/credential_projections.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- root-authority-changes > fixtures/security/m3/credential_projection_and_privileged_attach/root_authority_changes.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- spend-attempts > fixtures/security/m3/credential_projection_and_privileged_attach/spend_attempts.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- defects > fixtures/security/m3/credential_projection_and_privileged_attach/defects.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- summary > fixtures/security/m3/credential_projection_and_privileged_attach/summary.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- support-export > fixtures/security/m3/credential_projection_and_privileged_attach/support_export.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- drill-raw-secret-projection > fixtures/security/m3/credential_projection_and_privileged_attach/drill_raw_secret_projection.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- drill-unsigned-root-change > fixtures/security/m3/credential_projection_and_privileged_attach/drill_unsigned_root_change.json
cargo run -q -p aureline-policy --example dump_authority_ticket_fixtures -- drill-admitted-without-ticket > fixtures/security/m3/credential_projection_and_privileged_attach/drill_admitted_without_ticket.json
```

## Verification

```sh
cargo fmt -p aureline-policy -- --check
cargo test -p aureline-policy
```

The unit tests cover seeded validation and typed failure drills. The
fixture-driven tests parse the JSON fixtures, verify class and projection-mode
coverage, preserve support-export lineage, and recompute a clean audit for the
seeded page.
