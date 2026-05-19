# Privileged-action lineage examples

Support-facing examples for the privileged-action lineage rows projected from
the runtime-authority issuer-boundary page. Each example shows what support
and admin-audit reviewers actually see — and what they do **not** see — when
they read a metadata-only lineage row from a support packet.

The canonical record kind is
`security_runtime_authority_lineage_packet_record`. Each row in the packet is
a `security_runtime_authority_lineage_row_record`. The packet projection lives
in
[`/crates/aureline-policy/src/runtime_authority_issuers/mod.rs`](../../../crates/aureline-policy/src/runtime_authority_issuers/mod.rs)
and the boundary schema in
[`/schemas/security/runtime_authority_issuer.schema.json`](../../../schemas/security/runtime_authority_issuer.schema.json).

The full sample corpus is in
[`/fixtures/security/m3/privileged_action_lineage/`](../../../fixtures/security/m3/privileged_action_lineage/).
The red/green release report is
[`/artifacts/security/m3/runtime_authority_conformance_report.md`](../../security/m3/runtime_authority_conformance_report.md).

## What every row carries — and what it never carries

Each lineage row preserves only metadata-class tokens, opaque refs, and the
export-safe explanation string that is identical to the UI/CLI message:

- `decision_id`, `request_id`;
- `requesting_surface_class_token`, opaque `requesting_surface_ref`;
- `issuer_class_token` (`shell` / `policy_service` / `supervisor`);
- `requested_ticket_class_token` (`local_mutation`,
  `external_provider_mutation`, `credential_projection`,
  `privileged_debug_attach`, `policy_trust_admin_change`);
- `actor_class_token`, `authority_source_class_token`;
- `decision_class_token` (`granted`, `remembered_decision_narrowed`,
  `refused`);
- closed `rejection_reason_tokens` (empty when admitted);
- optional `minted_authority_ticket_ref`, optional `renewed_from_rule_id`
  (opaque);
- export-safe `explanation`, `decided_at`, and `audit_event_refs`.

Reviewers never see:

- raw credentials, projected-secret contents, or plaintext secret material;
- raw signed authority bodies (policy bundles, trust-root rotation blobs,
  admin command payloads);
- raw policy or evaluator payloads beyond the policy-epoch ref carried inside
  the sandbox-binding's `policy_epoch_ref`;
- raw sandbox-profile or capability-envelope contents beyond their opaque
  refs;
- raw provider response bodies;
- hidden delegation artifacts (delegation chains, downstream session cookies,
  intermediate handshake material).

## Example: external provider mutation granted via shell

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_external_provider_mutation_granted.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_external_provider_mutation_granted.json).

- Issuer chain: `shell` minted a single-use ticket after the user approved
  the AI plan at the shell prompt.
- Actor class: `human_account`; authority source: `human_account` against a
  provider-linked session.
- Ticket class: `external_provider_mutation` against a `provider_object`
  target ref (`provider:github:owner/repo:release-draft:42`).
- Approval ref: `authority-ticket:external:provider-publish:0001`.
- Outcome: `granted`. The decision time is `2026-05-18T10:00:05Z`.
- Recovery posture: `local_editing_preserved=true`. The granted decision does
  not require a reprompt.

This row proves the AI/extension/CLI/companion path can still complete a
provider write **without** self-authorizing — the shell mediates the mint
under fresh user approval.

## Example: local mutation via remembered-decision narrowing

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_local_mutation_remembered_narrowed.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_local_mutation_remembered_narrowed.json).

- Issuer chain: `policy_service` narrowed the remembered local-format rule
  `remembered-rule:local-format:0001` into a fresh ten-minute ticket scoped
  to the current repository and policy epoch.
- Actor class: `human_account`; authority source: `human_account`.
- Ticket class: `local_mutation` against the local workspace target ref
  `workspace:aureline:current-repo`.
- Approval refs: minted ticket `authority-ticket:local:remembered-format:0002`
  and renewed-from rule `remembered-rule:local-format:0001`.
- Outcome: `remembered_decision_narrowed`. The renewal is bounded — the
  remembered rule never acts like an unlimited bearer credential.

This row proves the remembered-decision narrowing path is visible and
auditable. Reviewers can quote both the minted ticket ref and the renewed-from
rule id and walk back through the audit stream to the original approval.

## Example: credential projection granted via supervisor

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_credential_projection_granted.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_credential_projection_granted.json).

- Issuer chain: `supervisor` minted a session-only credential projection
  scoped to the registry publish consumer.
- Actor class: `organization_admin`; authority source: `human_account`.
- Ticket class: `credential_projection` against the credential-consumer
  target ref `consumer:registry:publish-session`.
- Approval ref: `authority-ticket:credential-projection:registry-publish:0001`.
- Outcome: `granted`.

The row carries the broker handle ref only — no raw secret material is
exported, and no projected-secret contents leak into support packets.

## Example: privileged debug attach granted via shell

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_privileged_debug_attach_granted.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_privileged_debug_attach_granted.json).

- Issuer chain: `shell` minted a single-use privileged debug attach ticket
  after the local admin stepped up at the shell prompt.
- Actor class: `local_admin`; authority source: `human_account`.
- Ticket class: `privileged_debug_attach` against the debug-attach target ref
  `debug-attach:workspace:aureline:editor-process:42`.
- Approval ref:
  `authority-ticket:privileged-debug-attach:editor-process:0001`.
- Outcome: `granted`.

This row proves privileged attach lineage is precise enough for incident
review: actor, target, sandbox profile, and policy epoch are all explicit on
the row.

## Example: root-of-authority / admin change granted via supervisor

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_admin_root_authority_change_granted.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_admin_root_authority_change_granted.json).

- Issuer chain: `supervisor` minted a trust-root rotation ticket after
  verifying the signed root-authority proof and the step-up admin actor.
- Actor class: `organization_admin`; authority source: `human_account`.
- Ticket class: `policy_trust_admin_change` against the trust-store target
  `trust-root:2026-primary`.
- Approval ref: `authority-ticket:admin:trust-root-rotation:0001`.
- Outcome: `granted`.

Admin-audit reviewers can quote this row to prove that root-authority changes
required an explicit signed proof — the lineage row never inherits ambient
privilege from a non-issuer surface.

## Example: extension self-authorization refused

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_credential_projection_self_authorization_refused.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_credential_projection_self_authorization_refused.json).

- Issuer chain: no issuer minted authority. The extension attempted to mint a
  credential projection on its own.
- Outcome: `refused`. Rejection reason tokens:
  `self_authorization_by_non_issuer`, `missing_issuer_binding`.
- Recovery posture: `local_editing_preserved=true`, `reprompt_required=true`.
  The shell or policy service must reprompt before the request can retry.

This row proves the extension/recipe/CLI/companion surfaces cannot
self-authorize even when they hold an installation grant.

## Example: browser companion ambient privilege refused

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_browser_companion_ambient_privilege_refused.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_browser_companion_ambient_privilege_refused.json).

- Issuer chain: `shell` rejected an ambient-privilege inference from the
  browser companion's host session.
- Outcome: `refused`. Rejection reason token: `ambient_privilege_inferred`.
- Recovery posture: `local_editing_preserved=true`, `reprompt_required=true`.

## Example: remote helper local-only source refused

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_remote_helper_local_only_source_refused.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_remote_helper_local_only_source_refused.json).

- Issuer chain: `supervisor` refused a remote helper using a local-only
  authority against a provider object.
- Outcome: `refused`. Rejection reason tokens: `authority_source_mismatch`,
  `authority_source_unreachable_target`.
- The recovery guidance asks the operator to reroute through a human-account
  or installation-grant authority.

## Example: recipe remembered-decision drift refused

Fixture: [`fixtures/security/m3/privileged_action_lineage/lineage_row_recipe_remembered_decision_drift_refused.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_row_recipe_remembered_decision_drift_refused.json).

- Issuer chain: `shell` refused a recipe renewal that tried to broaden the
  remembered local-format rule onto a different repository.
- Outcome: `refused`. Rejection reason tokens:
  `remembered_decision_too_broad`, `remembered_decision_target_drift`.
- The recovery guidance asks the user to reprompt for the other repository.

## Walking back from a row to the source projection

Each row resolves back to the same projection module:

1. Quote the `decision_id` and `request_id` from the lineage row.
2. Open `/fixtures/security/m3/runtime_authority_issuer/page.json` and locate
   the matching request and decision; the `audit_event_refs` chain back to
   the broader audit stream.
3. Open `/crates/aureline-policy/src/runtime_authority_issuers/mod.rs` and
   read the seeded scenario; the live evaluator uses the same vocabulary, so
   the lineage row matches the runtime behavior.

No row depends on tribal knowledge or out-of-band lookups. Reviewers in
support, admin-audit, and release-evidence consume the same metadata.
