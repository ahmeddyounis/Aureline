# Ship capability-envelope packets across the M5 execution surfaces

This document is the canonical contract for the M5 **capability-envelope
packet**: the export-safe runtime artifact issued against the frozen M5
runtime-authority matrix. Where the matrix states what *may* be granted per
executing surface, an envelope is the concrete record bound to one issued
execution — naming its actor, target identity, allowed roots/sinks/endpoints,
handle-only secret references, governing policy epoch, expiry, and audit
lineage. Desktop, command, policy, CLI/headless, diagnostics, support-export,
help/About, and release surfaces consume one envelope object instead of cloning
per-surface approval or capability prose.

- Implementation: `crates/aureline-policy/src/ship_capability_envelope_packets_with_actor_target_allowed_roots_or_sinks_or_endpoints_secret_handle_refs_policy_epoch_e/`
- Boundary schema: `schemas/execution-auth/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.schema.json`
- Support export (truth source): `artifacts/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/support_export.json`
- Markdown summary: `artifacts/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e.md`
- Narrowed fixtures: `fixtures/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/`
- Producer / validator: `cargo run -p aureline-policy --example dump_m5_capability_envelope_packets`

## Track invariant

No ambient privilege. No AI tool, extension, recipe, browser route, or remote
helper self-issues authority: every envelope minted for an untrusted-helper
actor carries an externally issued lineage and is flagged
`self_issued_by_executor: false`. Actor and target identity, allowed scope,
secret references, policy epoch, expiry, and audit lineage stay inspectable and
export-safe. Raw secret material, credential bodies, and live ticket signatures
stay outside the support boundary. If enforcement cannot be honored, the
envelope **narrows or fails closed** instead of silently widening.

## What an envelope carries

Each `M5CapabilityEnvelope` is issued against one matrix surface row and names:

| Field | Meaning |
| --- | --- |
| `envelope_id` | Stable id for this issued envelope. |
| `surface` | The matrix executing surface this envelope is issued against. |
| `actor` | The actor (`human_operator`, `system_automation`, `ai_tool`, `recipe`, `extension`, `browser_route`, `remote_helper`), an export-safe `actor_ref`, and any delegated `on_behalf_of`. |
| `target` | The target class and export-safe `target_identity`, whether it is `off_device`, and whether the identity is `identity_verified`. |
| `allowed_scope` | The allowed **roots, sinks, and endpoints** — each a `kind` (`filesystem_root` / `data_sink` / `network_endpoint`), export-safe `label`, and `access` mode. |
| `granted_capability_classes` | The capability classes this envelope exercises; always a subset of the matrix row. |
| `secret_handle_refs` | Handle-only broker references — never raw secret material. |
| `secret_scope` | `no_secret_access`, `handle_only_delegated`, or `scoped_brokered_secret`. |
| `sandbox_profile` | The sandbox profile the envelope ran under (the matrix default or fully inert). |
| `policy_epoch` | The governing policy-epoch id, sequence, and whether it has been superseded. |
| `expiry` | `issued_at`, `expires_at`, a non-zero `ttl_seconds`, and whether the envelope is `single_use`. |
| `audit_lineage` | The external issuer class and ref, the approval-ticket ref and posture, the ordered `decision_chain`, and `self_issued_by_executor: false`. |
| `degraded_fallback` | What the envelope narrows to when full authority cannot be honored. |
| `applied_downgrade_triggers` | Triggers applied to this concrete envelope; non-empty exactly when `narrowed_from_default` is true. |

## Issued surfaces

Every claimed M5 executing surface is issued an envelope: `request_api_send`,
`database_action`, `notebook_kernel`, `scaffold_hook`, `preview_server`,
`ai_tool`, `recipe`, `browser_routed_action`, `incident_flow`, and
`remote_mutation`. A packet missing any surface fails validation
(`required_surface_missing`).

`ai_tool`, `recipe`, `extension`, `browser_route`, and `remote_helper` actors
are **untrusted helpers**: they may never self-issue authority, so their
envelope lineage must be externally issued. Browser-routed and remote-mutation
envelopes run off-device in an isolated remote runtime and must still carry the
identical envelope shape — including a verified target identity — even when
execution is brokered by another runtime.

## Relationship to the frozen matrix

The envelope packet consumes the frozen runtime-authority matrix directly. For
every envelope:

- `granted_capability_classes` MUST be a subset of the matrix row's
  `allowed_capability_classes` (`capability_widens_beyond_matrix` otherwise).
- `sandbox_profile` MUST be the matrix row's `default_sandbox_profile` or the
  fully inert `inert_no_execution` fail-closed profile
  (`sandbox_profile_widens` otherwise).

An envelope therefore can only ever **narrow** what the matrix authorizes; it
can never widen it.

## Enforced invariants

`M5CapabilityEnvelopePacket::validate` returns stable violation tokens. A packet
is rejected (and the row cannot publish) when:

- `wrong_record_kind` / `wrong_schema_version` / `missing_identity` — packet
  header is malformed.
- `missing_source_contracts` — the schema, doc, and frozen matrix / issuer /
  approval-ticket / secret-handle contracts are not all referenced.
- `required_surface_missing` — an executing surface has no issued envelope.
- `envelope_incomplete` / `capability_envelope_empty` / `allowed_scope_missing`
  — an envelope is missing identity, capabilities, or allowed roots/sinks/endpoints.
- `capability_widens_beyond_matrix` / `sandbox_profile_widens` — an envelope
  grants more than its matrix row allows.
- `self_issued_authority_forbidden` — an untrusted-helper actor self-issues
  authority.
- `elevated_capability_without_ticket` — an envelope grants an elevated
  capability without an externally issued approval-ticket ref.
- `expiry_missing` / `policy_epoch_missing` / `audit_lineage_incomplete` — the
  time bound, governing epoch, or lineage is absent.
- `secret_scope_inconsistent` — secret references and the declared secret scope
  disagree, or a secret-projecting capability carries no handle ref.
- `off_device_target_unverified` — an off-device envelope binds to an unverified
  target identity.
- `narrowing_inconsistent` — the `narrowed_from_default` flag disagrees with the
  applied downgrade triggers.
- `trust_review_incomplete` / `consumer_projection_incomplete` /
  `proof_freshness_incomplete` — a required review block is unsatisfied.
- `raw_boundary_material_in_export` — the export carries forbidden secret
  material.

## Downgrade, expiry, and off-device fixtures

The narrowed fixtures exercise the failure/recovery and off-device paths and all
validate clean:

- `remote_mutation_off_device_brokered.json` — a remote mutation executed
  off-device through a remote broker runtime preserves the full envelope shape.
- `ai_tool_ticket_expired_narrowed.json` — an AI-tool envelope whose ticket
  expired narrows to a read-only sanitized preview (`approval_ticket_expired`,
  `narrowed_from_default: true`).
- `database_action_write_ticket_unavailable_read_only.json` — a database
  envelope whose write ticket could not be honored narrows to read-only.

## Consumer parity

The trust review and consumer projection blocks assert that desktop,
command/policy, CLI/headless, support-export, diagnostics, help/About, and
release-evidence surfaces all project the same envelopes, and that remote and
browser-routed surfaces preserve envelope semantics off-device. Downstream
surfaces ingest the support export directly rather than cloning envelope prose.
