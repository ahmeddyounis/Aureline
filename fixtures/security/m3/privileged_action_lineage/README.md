# Privileged-action lineage corpus

Reviewer-facing fixtures for the support-safe privileged-action lineage packet
projected from the seeded runtime-authority-issuer page. The packet preserves
the issuer chain, requesting-surface class, actor class, target identity (as a
ticket-class token plus an opaque target ref), approval refs (minted ticket and
renewed-from rule), decision outcome, and audit-event refs — and explicitly
excludes raw credentials, projected-secret contents, hidden delegation
artifacts, raw authority bodies, and plaintext secret material.

The lineage packet schema is the `lineage_packet` projection in
[`/schemas/security/runtime_authority_issuer.schema.json`](../../../../schemas/security/runtime_authority_issuer.schema.json).
The reviewer-facing landing page is
[`/docs/security/m3/runtime_authority_issuer_boundaries.md`](../../../../docs/security/m3/runtime_authority_issuer_boundaries.md).
The support-facing example walk-through is
[`/artifacts/support/m3/privileged_action_lineage_examples.md`](../../../../artifacts/support/m3/privileged_action_lineage_examples.md).

The lineage packet itself is regenerated alongside the full conformance corpus
under [`/fixtures/security/m3/runtime_authority_issuer/`](../runtime_authority_issuer/).
This corpus is a per-row excerpt of that packet, sliced one fixture per high-risk
flow class so support reviewers can quote a single row without exporting the
full page.

## Files

| File | Purpose |
| --- | --- |
| `lineage_packet.json` | Full lineage packet (`security_runtime_authority_lineage_packet_record`). Bit-for-bit identical to `fixtures/security/m3/runtime_authority_issuer/lineage_packet.json`. |
| `lineage_row_local_mutation_remembered_narrowed.json` | One `runtime_authority_lineage_row`: CLI script renews a remembered local-format rule into a short-lived ticket via the policy service. |
| `lineage_row_external_provider_mutation_granted.json` | AI tool plan asks the shell to mint a provider-publish ticket; granted via shell. |
| `lineage_row_credential_projection_granted.json` | Admin console asks the supervisor to project a session-only handle to a registry consumer; granted. The row carries the broker handle ref only — no raw secret material. |
| `lineage_row_credential_projection_self_authorization_refused.json` | Extension tries to project a credential without an issuer; refused with `self_authorization_by_non_issuer` and `missing_issuer_binding`. |
| `lineage_row_privileged_debug_attach_granted.json` | Local admin steps up at the shell to attach to an editor process; granted via shell. |
| `lineage_row_admin_root_authority_change_granted.json` | Admin console rotates the trust root via the supervisor with a recorded signed root-authority proof. |
| `lineage_row_browser_companion_ambient_privilege_refused.json` | Browser companion infers ambient privilege from its host session; refused with `ambient_privilege_inferred`. |
| `lineage_row_remote_helper_local_only_source_refused.json` | Remote helper uses a local-only authority against a provider target; refused with `authority_source_mismatch` and `authority_source_unreachable_target`. |
| `lineage_row_recipe_remembered_decision_drift_refused.json` | Recipe renewal asks to broaden a remembered local-format rule onto a different repository; refused with `remembered_decision_too_broad` and `remembered_decision_target_drift`. |
| `redaction_summary.json` | Packet-level metadata: rejection-reason counts, `raw_credentials_excluded`, `provider_versus_local_distinguished`, and the export-safe `redaction_summary` string. |

## Redaction posture

Every row preserves only metadata-class tokens, opaque refs, and the
export-safe explanation string that is identical to the UI/CLI message. The
following are explicitly excluded:

- raw credentials, projected-secret contents, plaintext secret material;
- raw authority bodies (signed policy bundles, signed trust-root rotation
  blobs, signed admin command payloads);
- raw policy or evaluator payloads beyond the policy-epoch ref;
- raw sandbox-profile or capability-envelope contents beyond their opaque ref
  and fingerprint ref;
- raw provider response bodies; and
- any hidden delegation artifacts (delegation chains, downstream session
  cookies, intermediate handshake material).

Each lineage row is reproducible from
[`crates/aureline-policy::seeded_runtime_authority_issuer_page()`](../../../../crates/aureline-policy/src/runtime_authority_issuers/mod.rs)
followed by `RuntimeAuthorityLineagePacket::from_page(...)`. The fixture-replay
test under
[`crates/aureline-policy/tests/runtime_authority_issuer_cases.rs`](../../../../crates/aureline-policy/tests/runtime_authority_issuer_cases.rs)
re-derives the packet and asserts the checked-in JSON has not drifted from the
projection.
