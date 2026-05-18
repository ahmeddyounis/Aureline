# Authority-ticket credential and privileged-action fixtures

Reviewer-facing fixtures for the beta projection that extends authority
tickets to credential projection, privileged debug attach, and policy, trust,
or root-authority admin changes.

The canonical page record kind is
`security_authority_ticket_page_record`. The schemas live at
[`/schemas/security/authority_ticket.schema.json`](../../../../schemas/security/authority_ticket.schema.json),
[`/schemas/security/credential_projection.schema.json`](../../../../schemas/security/credential_projection.schema.json),
and
[`/schemas/security/root_authority_change.schema.json`](../../../../schemas/security/root_authority_change.schema.json).
The policy module lives at
[`/crates/aureline-policy/src/authority/mod.rs`](../../../../crates/aureline-policy/src/authority/mod.rs),
and the reviewer-facing landing page is
[`/docs/security/m3/authority_ticket_and_root_authority.md`](../../../../docs/security/m3/authority_ticket_and_root_authority.md).

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

## Files

| File | Purpose |
| --- | --- |
| `page.json` | Full authority-ticket page: tickets, credential projections, root-authority changes, spend attempts, defects, and summary. |
| `credential_projections.json` | Projection rows covering delegated-handle, session-only, and sign-only modes. |
| `root_authority_changes.json` | Signed root-authority change rows with admin actor, target, policy epoch, proof refs, rollback ref, and audit refs. |
| `spend_attempts.json` | Spend attempts spanning admitted credential projection, privileged attach, root-authority change, missing ticket, policy-epoch drift, and authority-source mismatch. |
| `defects.json` | Defect array; empty on the seeded page. |
| `summary.json` | Aggregate counts, class coverage, projection-mode coverage, root-change coverage, spend outcomes, and defect counts. |
| `support_export.json` | Redaction-safe support export preserving privileged-action lineage without raw secrets. |
| `drill_raw_secret_projection.json` | Drill: a projection flips `plaintext_secret_present=true`; surfaces `raw_secret_material_present`. |
| `drill_unsigned_root_change.json` | Drill: the root-authority proof becomes `missing_or_unverified`; surfaces `root_authority_proof_missing`. |
| `drill_admitted_without_ticket.json` | Drill: a missing-ticket spend is flipped to `admitted`; surfaces `spend_admitted_without_ticket`. |

## Protected states covered

- Credential projection is admitted only through a current matching ticket
  whose actor, target, sandbox, policy epoch, authority source, expiry, and
  projection row still match.
- Privileged debug attach and root-authority admin changes fail closed when a
  spend lacks a current matching ticket, drifts policy epoch, or mismatches
  the lineage authority source.
- Remembered decisions narrow into a reusable rule plus short-lived renewed
  local-mutation tickets. Credential projection, privileged attach, and
  policy/trust/admin tickets reject remembered reusable authority.
- Root-authority changes remain attributable to the admin actor and exportable
  through signed or local-authority proof refs, audit refs, policy epoch,
  target fingerprint, and rollback ref.
- Support exports preserve ticket, spend, credential projection,
  root-authority proof, actor, target, sandbox, policy, and audit lineage
  while excluding raw credentials and plaintext secret material.
