# Admin controls for provider allowlists, retention classes, region gates, and model deprecation

This contract carries operator-set governance over AI providers and models in one
export-safe truth packet whose unit of truth is an admin-control row. Shell, docs,
support export, diagnostics, and release tooling consume the packet directly
instead of re-describing an allowlist decision, retention floor, region gate, or
model deprecation by hand.

- Packet type: `aureline_ai::ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation::AdminControlPacket`
- Schema: [`schemas/ai/ship-admin-controls-for-provider-allowlists-retention-classes-region-gates-and-model-deprecation.schema.json`](../../../schemas/ai/ship-admin-controls-for-provider-allowlists-retention-classes-region-gates-and-model-deprecation.schema.json)
- Support export: [`artifacts/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/support_export.json`](../../../artifacts/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/support_export.json)
- Fixtures: [`fixtures/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/`](../../../fixtures/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/)

This lane reuses the execution-mode, region, and retention vocabularies frozen by
the provider/model registry lane, the approval vocabulary frozen by the
tool-gateway baseline, and the frozen M5 qualification, downgrade-trigger, and
rollback-posture vocabularies — it does not fork a parallel set of terms. Signed
and shared automation, external-tool connectors, and admin controls all follow the
same gate, audit, and downgrade discipline.

## The admin-control row

Each `AdminControlRow` binds, for one admin control:

| Field | Meaning |
| --- | --- |
| `control_id` / `control_label` / `control_family_label` | Stable id and review-safe labels. |
| `target_provider_id` / `target_provider_label` | The provider the control governs. |
| `target_model_id` | The governed model; required for a model-deprecation control and empty otherwise. |
| `governed_execution_mode` | `local`, `byok`, or `managed` — the mode the control governs the provider under. |
| `directive` | The typed control directive; its `kind` is the control family. |
| `enforcement_scope` | `organisation_wide`, `tenant_scoped`, `workspace_scoped`, or `deployment_profile_scoped`. |
| `enforcement_state` | Whether the control is live, staged, rolled back, superseded, draft, or blocked by higher policy. |
| `admin_authority` | The approval posture required to set or change the control. |
| `admin_identity_ref` | Opaque ref to the admin identity that set the control. |
| `audited` | Whether changes to the control are durably audited. |
| `claimed_qualification` | The control's claimed qualification. |
| `downgrade_rules` | Triggers that narrow the claim, including proof-stale and provider-unavailable. |
| `rollback_posture` / `rollback_verified` | Whether and how a control change reverses. |
| `evidence_packet_refs` | Evidence backing a claimed control. |
| `explanation_label` | Review-safe explanation of the posture. |

## Control families

The directive's `kind` tag is the control family. Each variant carries only the
fields its family needs:

- **`provider_allowlist`** — `decision` is `provider_allowed`,
  `provider_allowed_with_conditions`, `provider_denied_by_policy`, or
  `provider_pending_review`. A denied provider is a denial control; a pending
  review may not claim Stable.
- **`retention_floor`** — `required_floor` is the minimum disclosed retention
  posture a route may carry, and `denies_below_floor` says whether the control
  rejects routes below it. The floor must name a concretely disclosed retention
  class — never an unverified or policy-blocked posture.
- **`region_gate`** — `allowed_region_posture` is the disclosed region posture a
  route may run in, `allowed_region_tags` names the concrete regions when the
  posture is pinned, and `denies_outside_gate` says whether the control rejects
  routes outside the gate. A pinned posture names its regions; an unpinned posture
  names none.
- **`model_deprecation`** — `lifecycle_stage` moves a named model from
  `generally_available` through `deprecation_announced`,
  `deprecated_sunset_scheduled`, `blocked_new_sessions`, to `retired_removed`. A
  scheduled sunset names a `sunset_date`; any deprecation that has begun names a
  `migration_path_ref` or a `replacement_model_ref` so no user is stranded.

## Invariants

`AdminControlPacket::validate` enforces, among others:

- A model-deprecation control names its model; the other families carry no model id.
- A denial control — a denied provider, a retention floor that rejects routes
  below it, a region gate that rejects routes outside it, or a deprecation that
  blocks or retires a model — carries a real admin gate, is audited, and is
  actually live when it claims a public qualification rather than sitting in a
  silent draft.
- A control overridden by a higher-tier policy, or a provider allowlist still
  pending admin review, may not keep a public qualification.
- A claimed control carries evidence, and a claimed control whose change can be
  reversed has had that reversal verified.
- Every control carries the proof-stale and provider-unavailable downgrade
  triggers, and each rule narrows to a strictly weaker qualification than the
  control claims.
- Provider endpoints, credential bodies, raw API keys, OAuth tokens, and raw
  provider payloads never cross the support boundary; the packet carries opaque
  ids, classes, region tags, content addresses, and review-safe labels only.

## Consumers

`AdminControlPacket::is_provider_admin_blocked` projects the live allowlist denial
that routing and shell surfaces honor before dispatching to a provider.
`live_controls`, `family_count`, `denial_control_count`, `claimed_control_count`,
and `narrowed_qualification` give the deterministic projections support, docs,
diagnostics, and release tooling render instead of re-deriving admin posture
locally. `render_inspector` and `render_markdown_summary` produce review-safe
cards for support and review handoff.
