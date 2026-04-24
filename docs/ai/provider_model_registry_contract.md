# AI provider / model registry, local-model-pack, and external-tool gateway contract

This document is the **product-wide contract** for how AI provider
choice, model identity, local-model packs, and external AI tool
gateways are named, governed, and inspected before any turn runs
against them. It freezes one registry shape for providers, one for
models, and one for external-tool / MCP-server entries so that local,
BYOK (user-brought credential), enterprise-gateway-brokered, and
vendor-hosted routes all emit the same provider / model / execution-
locus identity on every evidence surface; so that external-tool
output is tainted by default and rides the same permission, policy,
and audit model as native tools; and so that provider selection is
inspectable at the point of intent and cannot silently widen disallowed
regions, retention classes, or feature classes.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / composer / tool-
gateway surface's mint of its own copy, this document wins and the
surface is non-conforming.

The companion artifacts are:

- [`/schemas/ai/provider_registry.schema.json`](../../schemas/ai/provider_registry.schema.json)
  — boundary schema for the `ai_provider_registry_entry_record`,
  `local_model_pack_entry_record`,
  `provider_selection_disclosure_record`, and
  `ai_provider_registry_audit_event_record` shapes.
- [`/schemas/ai/model_registry.schema.json`](../../schemas/ai/model_registry.schema.json)
  — boundary schema for the `model_registry_entry_record` and
  `model_registry_audit_event_record` shapes.
- [`/schemas/ai/external_tool_registry.schema.json`](../../schemas/ai/external_tool_registry.schema.json)
  — boundary schema for the `external_tool_registry_entry_record`,
  `external_tool_invocation_disclosure_record`, and
  `external_tool_registry_audit_event_record` shapes.
- [`/fixtures/ai/provider_tool_rows/`](../../fixtures/ai/provider_tool_rows/)
  — worked-example corpus covering at least one local-model pack,
  one BYOK provider, one enterprise-gateway route, and one each of
  a stdio / local-HTTP / remote-HTTP external-tool row.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/context_assembly_contract.md`](./context_assembly_contract.md) —
  provider class, route path class, cost visibility class, trust
  posture, tainted-fence strategy, tainted-usage constraints, and
  tool-call outcome are authored there and re-exported here without
  redefinition. Every AI turn resolves to one
  `ai_context_assembly_record`; this contract pins which
  provider / model / locus / pack / external-tool rows that record
  may cite.
- [`/docs/ai/evidence_replayability_contract.md`](./evidence_replayability_contract.md) —
  evidence packets quote the provider entry id, model entry id,
  execution-locus class, region posture, retention stance, quota
  family, and (for tool-using turns) the tool entry id. Graded
  replay posture keys off those ids.
- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md) —
  mutation-mode vocabulary, `ai_provider_surface` surface class,
  connected-provider record, browser-handoff packet, callback
  envelope, and publish-later queue are authored there. AI
  invocations that route through a connected provider reuse that
  contract's callback / queue shape for asynchronous returns.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state, `deployment_profile_class`, policy epoch,
  and trust state on every record.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md) —
  BYOK credentials are addressed through the secret broker; the
  broker-owned redaction pass runs before bytes reach any persistent
  or exportable sink; raw credentials never cross this boundary.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md) —
  admin policy MAY narrow which providers, models, packs, tools,
  loci, regions, retention stances, quota families, auth modes,
  side-effect classes, and data classes an invocation may reach;
  policy MAY NOT silently widen any axis.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  approval-ticket vocabulary. Every AI provider invocation above
  `allowed_without_prompt`, every local-pack admission, and every
  external-tool invocation above `allowed_without_prompt` MUST
  spend an approval ticket before dispatch.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  `redaction_class` is re-exported without modification.
- [`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md) —
  extension-provided providers and extension-provided tools ride
  the extension effective-permission surface; the registry row
  references the extension manifest by id and never re-authors the
  extension's authority.

If this document disagrees with those sources, those sources win and
this document plus the schemas are updated in the same change.

This document does not ship a live provider-arbitration service, a
live model-serving runtime, a live local-pack install / verify
pipeline, a live tool-gateway runtime, or any adapter-specific
wire code. It freezes the contract those implementations will read
and write. The eventual provider-arbitration and tool-gateway
crates' Rust types are the schema of record; the JSON Schema
exports are the cross-tool boundary every non-owning surface reads.

## Why freeze this now

The product has to answer the same set of questions for every AI
turn and every AI-driven tool call, long before a live
provider-arbitration service exists:

1. *Which provider entry, which model entry, which execution locus,
   which region, which retention stance, which quota family, and
   which feature classes did this invocation run against?*
2. *Was that identity inspectable at the point of intent, or was it
   selected silently behind a header?*
3. *Did the invocation run locally (in-process, sandbox, companion
   service), through a BYOK vendor route, through an enterprise
   gateway, through a vendor-hosted first-party-managed route, or
   through an extension-mediated route?*
4. *If the invocation returned bytes that a future turn will read,
   did those bytes ride the tainted fence by default, or did the
   surface allow them to masquerade as trusted instruction?*
5. *If the invocation spent a side effect (wrote a comment, called a
   tool, mutated provider state, widened a capability), did it
   spend an approval ticket through the same path native tools use,
   or did it try to bypass that path?*
6. *For a local-model pack — is the pack signed? Is the manifest
   chain intact? Is the mirror continuity intact? Is it quarantined
   or withdrawn?*

Without one frozen contract the product is free to invent a local
"Use built-in model" shortcut, a BYOK "Custom API" shortcut, a
hosted-vendor "Managed" shortcut, an enterprise-gateway "Company
proxy" shortcut, and a tool / MCP-server shortcut that each disclose
different identity axes, classify retention differently, apply
taint differently, and audit under different stream ids. Support
exports see five incompatible shapes, the replay harness cannot
correlate a local invocation with a hosted invocation of the same
model family, and a silent widening (policy-forbidden region, wrong
retention stance, un-approved feature class) becomes invisible.

This contract closes that gap with **three governed registries
(provider, model, external tool), one per-invocation disclosure
shape each, and one set of const audit-event ids** every
AI-adjacent surface reads.

## Who reads this document

- **AI / prompt-composer / context-resolver authors** resolving a
  turn against one governed provider entry, one governed model
  entry, optionally one governed local-model pack, and zero or
  more governed external-tool entries.
- **Tool-gateway / tool-invocation-surface authors** (AI tool
  callers, palette / command surfaces that invoke external tools,
  automation recipes) minting one
  `external_tool_invocation_disclosure_record` before every
  invocation and one audit event after.
- **Admin / policy / settings surface authors** narrowing which
  provider classes, execution loci, transports, auth modes,
  retention stances, region postures, feature classes, data
  classes, quota families, and external-tool classes are admitted
  per deployment profile.
- **Evidence / replay / support / parity-audit authors** quoting
  provider entry id, model entry id, execution-locus class, region
  posture, retention stance, quota family, feature class, and tool
  entry id mechanically, regardless of whether the invocation ran
  locally, BYOK, through an enterprise gateway, or vendor-hosted.
- **Security / emergency-action authors** quarantining or
  withdrawing a provider entry, a local pack, or an external-tool
  entry through the audit event stream without re-authoring the
  registry shape.

## 1. One contract, three registries, one identity spine

### 1.1 Frozen registries

| Registry                                        | Primary record kind                            | Source of truth                                        |
|-------------------------------------------------|------------------------------------------------|--------------------------------------------------------|
| AI provider registry                            | `ai_provider_registry_entry_record`            | `schemas/ai/provider_registry.schema.json`             |
| Local-model pack registry                       | `local_model_pack_entry_record`                | `schemas/ai/provider_registry.schema.json`             |
| AI model registry                               | `model_registry_entry_record`                  | `schemas/ai/model_registry.schema.json`                |
| External-tool / MCP-server gateway registry     | `external_tool_registry_entry_record`          | `schemas/ai/external_tool_registry.schema.json`        |

Local-model packs live alongside providers in the provider-registry
schema because every pack is served by exactly one provider entry
whose `execution_locus_class` is one of the three local loci. A pack
that no provider references cannot be invoked.

### 1.2 The identity spine

Every AI invocation (inline-composer turn, background branch-agent
turn, review-handoff turn, palette-triggered AI command, CLI / SDK
invocation) MUST carry at least:

- `provider_entry_id` — the governed row from the provider registry;
- `model_entry_id` — the governed row from the model registry;
- `execution_locus_class` — where inference actually ran;
- `region_posture_class` — where bytes landed geographically;
- `retention_stance_class` — what the endpoint does with bodies;
- `quota_family_class` — how capacity and cost are budgeted;
- the subset of `supported_feature_class` values actually exercised;
- the `data_class` allowlist the assembly filtered under.

These fields are declared on the provider entry and echoed verbatim
on every `provider_selection_disclosure_record`,
`ai_context_assembly_record`, `ai_route_plan_record`,
`ai_route_receipt_record`, `ai_spend_receipt_record`, and
`ai_evidence_packet_record` the invocation produces. A local,
BYOK, enterprise-gateway, or vendor-hosted instance of the same
model family resolves to the same `model_entry_id` (when the
parity lab has verified the model is identical) or to distinct
`model_entry_id`s (when the instances are known to differ in
behaviour); the row chosen MUST match the actual execution locus.

### 1.3 One governed row per route class

Every registry entry names exactly one of the frozen
`execution_locus_class` values. A provider selection that cannot
name its locus MUST route to a `disabled_no_provider` entry and
deny with `execution_locus_unresolved`; it MUST NOT fabricate a
locus or silently default.

| Route class the user sees              | Execution locus class                          | Canonical example                                                |
|----------------------------------------|------------------------------------------------|------------------------------------------------------------------|
| Built-in local model                   | `local_in_process`                             | First-party signed completion pack loaded in the shell's process |
| Local model (sandboxed)                | `local_sandbox_process`                        | A signed pack running in a separate sandboxed process / container |
| Local companion service                | `local_companion_service`                      | A loopback model server (llama.cpp, local Ollama, etc.)          |
| Bring-your-own-key (vendor)            | `byok_remote_vendor_direct`                    | User pastes a vendor API key; calls land on the vendor directly  |
| Bring-your-own-key (self-hosted)       | `byok_remote_self_hosted_direct`               | User registers a self-hosted endpoint; calls land there directly |
| Enterprise gateway                     | `enterprise_gateway_brokered`                  | All calls ride a customer-operated gateway                       |
| Vendor-hosted (first-party managed)    | `vendor_hosted_first_party_managed`            | First-party product operates the vendor contract                 |
| Extension-provided                     | `extension_provided_locus`                     | An extension manifest authors the locus                          |
| Mocked / test                          | `mocked_test_locus`                            | Parity / record-replay fixtures only                             |
| Disabled / withdrawn                   | `disabled_no_locus`                            | Row exists but is not admitted                                   |

## 2. Inspectability at the point of intent

### 2.1 Pre-invocation disclosure

Every AI invocation MUST mint a
`provider_selection_disclosure_record` before dispatch. The record
MUST surface at minimum the six disclosure kinds:

1. `provider_identity_chip`
2. `model_identity_chip`
3. `execution_locus_chip`
4. `region_posture_chip`
5. `retention_stance_chip`
6. `data_class_allowlist_readout`

Missing any required kind denies with
`inspectability_disclosure_missing_pre_invocation`. Additional
disclosure kinds (`quota_family_chip`, `transport_class_chip`,
`auth_mode_chip`, `cost_visibility_chip`, `deployment_profile_chip`,
`policy_epoch_chip`, `workspace_trust_chip`, `feature_class_badge`,
`tainted_return_posture_chip`) narrow; they do not widen.

### 2.2 No silent widening

A disclosure record names the admitted sets (feature classes, data
classes) and the single admitted values (execution locus, region
posture, retention stance, quota family, transport, auth mode,
approval posture, taint posture). The subsequent invocation is
bounded by the disclosure:

- An invocation whose requested feature class is not in the
  disclosure's `supported_feature_classes` MUST deny with
  `provider_exceeds_feature_class_policy`.
- An invocation whose source data class (from the assembly) is not
  in the disclosure's `data_class_allowlist` MUST deny with
  `provider_data_class_not_in_allowlist`.
- An invocation whose region, retention stance, quota family,
  transport, auth mode, or approval posture does not match the
  disclosure MUST deny with the corresponding typed reason
  (`provider_not_approved_for_region`,
  `provider_not_approved_for_retention_stance`,
  `provider_quota_family_forbidden`,
  `provider_transport_forbidden`,
  `provider_auth_mode_forbidden`, or deny via the approval-ticket
  path).

### 2.3 Policy-epoch invalidation

A policy-epoch roll between disclosure and invocation invalidates
the disclosure and the invocation MUST deny with
`policy_epoch_rolled_invalidations`. Re-disclosure is required
before any subsequent invocation; silent re-use is forbidden.

## 3. Cross-locus identity parity

A local, BYOK, enterprise-gateway, or vendor-hosted instance of the
same model family MUST emit the same identity spine on every
evidence packet, route receipt, and spend receipt. The
`evidence_identity_fields` set declared on every
`ai_provider_registry_entry_record` lists the required fields; an
emission missing any of them denies with
`provider_identity_mismatch_on_evidence` or
`execution_locus_mismatch_on_evidence`.

### 3.1 Minimum evidence identity fields

Every row MUST include at least:

- `provider_entry_id`
- `model_entry_id`
- `execution_locus_class`
- `region_posture_class`
- `retention_stance_class`

Additional fields (`transport_class`, `quota_family_class`,
`auth_mode_class`) narrow; they do not widen.

### 3.2 Correlation rule

Two invocations whose `model_family_label` and
`model_capability_version` match but whose `execution_locus_class`
differs MAY resolve to the same `model_entry_id` only when the
parity lab has verified the two instances behave identically under
the parity harness. Otherwise the rows resolve to distinct
`model_entry_id`s and a reviewer who wants to correlate them does
so through the shared `model_family_label` chip on the model-picker
readout — not through an ad-hoc identity collapse at the assembly
layer.

## 4. Local-model-pack integrity

### 4.1 Verification state rules

A local-model pack row MUST carry a
`pack_verification_state_class`. A pack whose state is not one of
`verified_signature_valid`, `verified_manifest_chain_valid`, or
`verified_mirror_continuity_intact` MUST NOT serve turns. The
failing state surfaces on every pack chooser, settings, and
evidence surface; the repair hook points at the admin / security-
review path, not at a silent retry.

### 4.2 Trust posture rules

A pack's `pack_trust_posture_class` MAY NOT silently promote to
any `trusted_*` posture. A user-imported pack stays at
`reviewed_user_imported` until an admin / security-review decision
promotes it; a pack whose manifest is unsigned or whose mirror
continuity is broken stays at `policy_quarantined_pack` or
`unreviewed_user_imported`.

### 4.3 Quarantine and withdrawal

A pack in the `quarantined_pending_review`,
`verification_failed_signature_invalid`,
`verification_failed_manifest_chain_broken`,
`verification_failed_mirror_continuity_broken`, or
`withdrawn_by_operator` state MUST deny every invocation with the
matching typed reason. Silent fallback to another pack is
forbidden; the surface MUST surface the denial and the repair hook.

### 4.4 Offline posture

A pack whose locus is local and whose auth mode is
`no_auth_local_only` / `signed_manifest_only_local_pack` MUST
declare `offline_only_no_network_egress`. An offline-only pack
that later requires network egress is a contract violation and
denies with `provider_transport_forbidden`.

## 5. External tools are tainted by default

### 5.1 Taint invariant

Every external-tool row declares a
`tool_output_trust_posture_class`. All values except
`trusted_first_party_local_tool_output` and
`trusted_signed_extension_local_tool_output` treat the tool's
return bytes as **tainted input** for any future AI turn that
reads them. Tainted bytes ride the
`schemas/ai/context_assembly.schema.json` tainted-fence strategy
with the default usage-constraint set
(`must_not_gain_tool_permission`, `must_not_escalate_scope`,
`must_not_mint_citations`, `must_not_override_instruction_bundle`,
`must_not_publish_externally`, `must_not_commit_to_repo`,
`must_not_dispatch_branch_agent`,
`must_not_route_to_higher_cost_tier`,
`must_preserve_fence_in_downstream_packet`).

A surface that lowers a tainted return to trusted authority without
a `tool_output_trust_posture_class` in the trusted set and without
the corresponding signed-manifest verification is non-conforming
and denies with `tool_output_untainted_attempted`.

### 5.2 Same-as-native permission / policy / audit model

External tools MUST ride the same permission, policy, and audit
model as native tools. Specifically:

1. Every invocation above `allowed_without_prompt` spends an
   approval ticket (ADR-0010 `schemas/integration/approval_ticket.schema.json`)
   before invocation. A missing ticket denies with
   `tool_bypass_attempted_approval_ticket_missing`.
2. Every invocation runs through the admin-policy resolver and the
   workspace-trust evaluator. A skipped evaluation denies with
   `tool_bypass_attempted_policy_evaluation_skipped`.
3. Every invocation emits on the `ai_external_tool_registry`
   audit stream (`ai_external_tool_invocation_admitted`,
   `ai_external_tool_invocation_denied`,
   `ai_external_tool_output_tainted_by_default`) and, when the
   invocation originated from an AI turn, also on the
   `ai_context` audit stream
   (`ai_context_tool_call_invoked`,
   `ai_context_tool_call_denied`,
   `ai_context_tainted_return_observed`). A suppressed event
   denies with `tool_bypass_attempted_audit_event_suppressed`.

### 5.3 Side-effect and data-class containment

Every tool row names the side-effect classes the tool may predict
(`allowed_side_effect_classes`) and the data classes the tool may
receive as input (`allowed_data_classes`). An invocation whose
predicted side effect is outside the row's allowlist denies with
`tool_side_effect_class_not_in_allowlist`; an invocation whose
data class is outside the row's allowlist denies with
`tool_data_class_not_in_allowlist`. Credential handles and secret
projections are on the `denied_always_data_classes` set and are
never admitted.

### 5.4 Transport and locus visibility

A stdio-spawned local tool (`local_stdio_spawn` /
`local_subprocess_same_device`), a loopback HTTP tool
(`local_http_loopback`), a unix-domain-socket tool, a remote HTTPS
tool (`remote_https`), and each of the three MCP-server variants
(`remote_mcp_over_stdio`, `remote_mcp_over_local_http`,
`remote_mcp_over_streamable_http`) are mutually distinguishable on
the invocation disclosure chip and in the audit event payload. No
surface may collapse `local_subprocess_same_device` and
`remote_https` under one "external tool" label.

## 6. Approval-ticket and browser-handoff composition

An AI invocation that routes through a connected provider
(`connected_provider_vendor`, `connected_provider_self_hosted`,
`extension_provided_provider`) inherits the ADR-0010 browser-handoff
and approval-ticket vocabulary. The provider-selection disclosure
cites the admitting approval ticket; asynchronous provider returns
(if any) ride the `provider_callback_envelope_record` shape
(`docs/providers/provider_mode_contract.md`) with `callback_class =
provider_event` or `provider_webhook`. A callback-bearing AI
provider entry MUST declare its actor class, replay posture, and
dedup policy through the provider-mode contract; this document
does not re-mint those vocabularies.

## 7. Redaction posture

Every provider entry, local-pack entry, model entry, external-tool
entry, selection disclosure, invocation disclosure, and audit event
declares a `redaction_class` from the ADR-0011 set
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw URLs,
raw endpoint hostnames, raw spawn commands, raw environment
variables, raw API keys, raw OAuth tokens, raw mTLS material, raw
request / response bodies, raw stdio frames, raw model weights,
and raw pack bytes never cross this boundary on any surface.
Exports, support bundles, mutation-journal entries, evidence
packets, replay captures, and AI context captures carry opaque
refs and structured fields only.

Narrowing is permitted: admin policy MAY remove a provider class,
a model entry, a local pack, an external-tool class, an execution
locus, a transport, an auth mode, a retention stance, a region
posture, a feature class, a data class, a quota family, or an
approval posture from a deployment profile. Widening beyond the
frozen rules is forbidden.

## 8. Audit-event reuse

Every registry admission / update / disable / supersede event,
every selection disclosure, every pack admission / verification /
quarantine / withdrawal event, and every tool invocation admission
/ denial / bypass-attempt / taint-application event fires on the
dedicated audit streams named in the schemas:

- `ai_provider_registry` — events frozen in
  `schemas/ai/provider_registry.schema.json`.
- `ai_model_registry` — events frozen in
  `schemas/ai/model_registry.schema.json`.
- `ai_external_tool_registry` — events frozen in
  `schemas/ai/external_tool_registry.schema.json`.

Every AI-turn-originated tool invocation also fires on the
`ai_context` audit stream (`schemas/ai/context_assembly.schema.json`)
so the turn lineage is legible without cross-schema join.

No new audit-event id is introduced by this contract on the
ADR-0010 `provider_handoff` stream; callback / approval / queue
events stay on their existing ids (the browser-handoff packet,
approval-ticket, and publish-later schemas).

## 9. Acceptance-criteria cross-walk

| Acceptance criterion                                                                                                                    | Where enforced                                                                                                                                                                                 |
|-----------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Provider selection is inspectable before invocation and cannot silently widen disallowed regions, retention classes, or feature classes. | Section 2 (disclosure record + minimum six disclosure kinds + no-widening rules + policy-epoch invalidation). Schema: `provider_selection_disclosure_record.disclosure_kinds_surfaced` allOf. |
| Local, remote, and hosted execution of external tools remains visually distinguishable in fixtures and schemas.                         | Section 1.3 (execution-locus matrix) + section 5.4 (transport / locus disclosure). Schema: `tool_execution_locus_class` enum is mutually distinct for local / remote / gateway variants.      |
| At least one example each exists for a local model pack, a BYOK provider, an enterprise-gateway route, and a stdio/local-HTTP/remote-HTTP external tool row. | Fixtures under `/fixtures/ai/provider_tool_rows/` (see README).                                                                                                                               |
| Local, BYOK, enterprise-gateway, and vendor-hosted routes emit the same provider / model / execution-locus identity fields.             | Section 3 (identity spine + `evidence_identity_fields` allOf) + schema invariants on the provider-registry entry record.                                                                      |
| External tool output is tainted input by default and MAY NOT bypass the permission, policy, and audit model used by native tools.       | Section 5 (taint invariant + same-as-native model). Schema: `tool_output_trust_posture_class` allOf on `external_tool_registry_entry_record`; denial reasons `tool_output_untainted_attempted`, `tool_bypass_attempted_*`. |
| Local-model packs cannot serve turns until verified; quarantined or withdrawn packs deny with typed reasons.                            | Section 4 (pack verification / trust posture rules). Schema: allOf on `local_model_pack_entry_record`; denial reasons `local_model_pack_unverified`, `local_model_pack_quarantined`, `local_model_pack_withdrawn`. |

## 10. Schema-of-record posture

Rust types in the eventual provider-arbitration and tool-gateway
crates are the source of truth. The JSON Schema exports at
`schemas/ai/provider_registry.schema.json`,
`schemas/ai/model_registry.schema.json`, and
`schemas/ai/external_tool_registry.schema.json` are the cross-tool
boundary every non-owning surface reads.

Adding a new `provider_class`, `execution_locus_class`,
`transport_class`, `auth_mode_class`, `retention_stance_class`,
`region_posture_class`, `supported_feature_class`,
`quota_family_class`, `data_class`,
`pre_invocation_disclosure_kind`, `approval_posture_class`,
`pack_origin_class`, `pack_verification_state_class`,
`pack_trust_posture_class`, `capability_posture_class`,
`tokeniser_posture_class`, `context_window_class`,
`deprecation_lane_class`, `model_lifecycle_state_class`,
`determinism_posture_class`, `instruction_following_profile_class`,
`modality_class`, `tool_class`, `tool_transport_class`,
`tool_execution_locus_class`, `tool_auth_mode_class`,
`tool_output_trust_posture_class`, `tool_lifecycle_state_class`,
`tool_output_invocation_disclosure_kind`, `audit_event_id`, or
`denial_reason` is additive-minor and requires the relevant
`*_schema_version` bump; repurposing an existing value is breaking
and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, ADR 0010, ADR 0011, and the existing AI
context-assembly contract.

## 11. Out of scope at this revision

- Live provider arbitration (the service that picks which entry to
  invoke).
- Model serving (the runtime that actually runs inference locally
  or routes remote calls).
- Local-model-pack install, download, verification, or quarantine
  pipelines. This document freezes the row shape those pipelines
  will read and write.
- External-tool invocation routing, stdio / MCP transport
  implementation, and connection management.
- Cost / spend / metering runtime. Cost-visibility class is
  declared on each row; the metering lane is separate.
