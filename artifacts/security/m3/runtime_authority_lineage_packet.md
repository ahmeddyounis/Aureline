# Runtime authority lineage packet

Release-evidence artifact for the runtime-authority issuer-boundary projection.
The packet projects each `security_issuer_boundary_decision_record` in the
seeded `runtime_authority_issuer_page` into a metadata-only row that admin
audit, support exports, and release-evidence reviewers can replay without
seeing raw credentials, raw policy payloads, or plaintext secret material.

## Inputs

- `shared_contract_ref: security:runtime_authority_issuer_beta:v1`
- `schema_version: 1`
- Source contract owned by
  [`/crates/aureline-policy/src/runtime_authority_issuers/mod.rs`](../../../crates/aureline-policy/src/runtime_authority_issuers/mod.rs).
- Page boundary schema:
  [`/schemas/security/runtime_authority_issuer.schema.json`](../../../schemas/security/runtime_authority_issuer.schema.json).
- Remembered-rule boundary schema:
  [`/schemas/security/remembered_decision_rule.schema.json`](../../../schemas/security/remembered_decision_rule.schema.json).
- Reviewer-facing doc:
  [`/docs/security/m3/runtime_authority_issuer_boundaries.md`](../../../docs/security/m3/runtime_authority_issuer_boundaries.md).

## Lineage row shape

Each `runtime_authority_lineage_row` preserves these fields verbatim from the
page:

- `decision_id`
- `request_id`
- `requesting_surface_class_token` (`ai_tool`, `extension`, `recipe_runner`,
  `cli_script`, `browser_companion`, `remote_helper`, `admin_console`,
  `local_admin_tool`, `automation_scheduler`)
- `requesting_surface_ref` (opaque)
- `issuer_class_token` (`shell` / `policy_service` / `supervisor`)
- `requested_ticket_class_token` (`local_mutation`,
  `external_provider_mutation`, `credential_projection`,
  `privileged_debug_attach`, `policy_trust_admin_change`)
- `actor_class_token`
- `authority_source_class_token` (`human_account`, `installation_grant`,
  `delegated_credential`, `local_only_authority`)
- `decision_class_token` (`granted`, `remembered_decision_narrowed`, `refused`)
- `rejection_reason_tokens` (closed vocabulary; empty when admitted)
- `minted_authority_ticket_ref` (optional, opaque)
- `renewed_from_rule_id` (optional, opaque)
- `explanation` (export-safe, identical to the UI/CLI string)
- `decided_at`
- `audit_event_refs`

The packet wraps the rows with `rejection_reason_counts`,
`raw_credentials_excluded`, `provider_versus_local_distinguished`, and a
`redaction_summary` that documents exactly which fields are preserved and
which are excluded.

## Required coverage

For an honest M3 beta posture the packet must include at least one row for
each of:

- a `granted` decision against an `external_provider_mutation` request from a
  non-issuer surface (proves the AI/extension/CLI/companion path cannot
  self-authorize but can still proceed under shell mediation);
- a `remembered_decision_narrowed` decision (proves remembered decisions
  compile to fresh short-lived tickets rather than bearer credentials);
- a refusal carrying `self_authorization_by_non_issuer` (proves attempts to
  bypass the issuer boundary fail closed);
- a refusal carrying `ambient_privilege_inferred` (proves ambient privilege
  inferences are denied);
- a refusal carrying `remembered_decision_too_broad` and/or
  `remembered_decision_target_drift` (proves remembered rules cannot be
  silently broadened);
- a refusal carrying `authority_source_mismatch` and/or
  `authority_source_unreachable_target` (proves local-only authority cannot
  conflate with provider-linked authority); and
- a `granted` `policy_trust_admin_change` row whose request carries
  `root_authority_proof_present = true` (proves admin/root changes require an
  explicit signed proof).

The seeded `seeded_runtime_authority_issuer_page()` page covers all of these,
and the policy crate validator surfaces typed defects whenever one of them
goes missing.

## How to regenerate

The page and packet are pure functions of the source matrix in the
`runtime_authority_issuers` module. To rebuild the packet:

```sh
cargo test -p aureline-policy
```

The unit tests under `runtime_authority_issuers::tests` confirm:

- the seeded page validates with zero defects;
- the lineage packet preserves required rejection reasons; and
- the validator flips to a typed defect when an admitted decision is forged
  on top of a self-authorization attempt, when a remembered rule is broadened,
  when a `shell` issuer overreaches into root authority, when a refused
  decision lacks a reason, or when a local-only authority is admitted against
  a provider target.

## Redaction summary

The packet only carries opaque refs, closed-vocabulary tokens, the
export-safe explanation strings already shown in the UI and CLI, and opaque
audit event refs. The following are explicitly excluded:

- raw credentials and plaintext secret material;
- raw authority bodies (signed policy bundles, signed trust-root rotation
  blobs, signed admin command payloads);
- raw policy or evaluator payloads beyond the policy-epoch ref;
- the contents of the sandbox profile or capability envelope beyond their
  opaque ref and fingerprint ref; and
- raw provider response bodies.

Reviewers can therefore consume the packet in admin audit and support exports
without reintroducing credential material into shared channels.
