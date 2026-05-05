# Transport Inspector, Effective-Route Explanation, And No-Direct-Fallback Denial Contract

This contract freezes the in-product transport inspector. It binds one
`effective_route_state_record` and one `transport_denial_record` shape to the
shared transport-governance vocabulary so a reviewer can answer two questions
about any network attempt:

1. Where did the request come from, where did it execute, and why did it take
   or not take a given route?
2. If the route was refused or rerouted, what blocked it, what is the typed
   remediation, and which inspector or export action is available on the
   current profile?

The contract is a projection layer only. It does not implement a network
stack, a proxy resolver, a PAC engine, a certificate store, an SSH
known-hosts store, a mirror service, or a repair runner.

## Companion Artifacts

- [`/docs/network/transport_governance_seed.md`](transport_governance_seed.md)
  defines the underlying component, permission, target, auth, route, proxy,
  trust, mirror, offline, and attribution vocabulary.
- [`/docs/network/transport_governance_packet_seed.md`](transport_governance_packet_seed.md)
  defines the transport decision record this inspector projects from.
- [`/docs/network/transport_explainability_surface_contract.md`](transport_explainability_surface_contract.md)
  defines the summary-strip, endpoint-row, certificate-card, and denied-attempt
  history surfaces that the inspector links into.
- [`/schemas/network/transport_decision.schema.json`](../../schemas/network/transport_decision.schema.json)
  is the source decision schema for route, trust, no-bypass, and repair hints.
- [`/schemas/network/network_attribution_record.schema.json`](../../schemas/network/network_attribution_record.schema.json)
  is the event-attribution spine for allowed, denied, failed, deferred, and
  inbound callback transport events.
- [`/schemas/network/transport_summary_strip.schema.json`](../../schemas/network/transport_summary_strip.schema.json),
  [`/schemas/network/endpoint_history_row.schema.json`](../../schemas/network/endpoint_history_row.schema.json),
  and
  [`/schemas/network/certificate_detail_card.schema.json`](../../schemas/network/certificate_detail_card.schema.json)
  define the projected explainability surfaces.
- [`/schemas/network/effective_route_state.schema.json`](../../schemas/network/effective_route_state.schema.json)
  defines one `effective_route_state_record` for the inspector.
- [`/schemas/network/transport_denial.schema.json`](../../schemas/network/transport_denial.schema.json)
  defines one `transport_denial_record` for typed denials.
- [`/fixtures/network/transport_inspector_cases/`](../../fixtures/network/transport_inspector_cases/)
  contains worked inspector cases for desktop, CLI/headless, remote target,
  extension host, AI broker, docs/update clients, managed-control path, and
  air-gapped or mirrored mode.

If this document and a schema disagree, the schema wins and this document
updates in the same change. If this document and the transport governance
seed disagree on enum values, the seed wins. If this contract disagrees with
the source specs below, the source specs win and all projection artifacts
update together.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` section 5.42, network, proxy, certificates, and
  transport-governance architecture.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix G, network
  and transport policy matrix.
- `.t2/docs/Aureline_Technical_Design_Document.md` transport governance
  section plus Appendices CX, CY, and CZ.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` section 18.14 and Appendix BS,
  transport, proxy, certificate, and mirror-state templates.

## Surface Boundary

Every transport inspector projects from a `transport_decision_record`, an
optional `network_attribution_record`, and the current `transport_posture`
object. The inspector MUST NOT derive private terms such as "marketplace
network error", "direct fallback", "AI gateway offline", "bad cert", or
"remote issue" when a shared proxy, trust, mirror, policy, auth, offline, or
deny-reason code is available.

The two records this contract introduces answer different questions:

| Record | Question answered | Required source |
|---|---|---|
| `effective_route_state_record` | What is the route for this attempt, where does it come from, where does it execute, and what does the canonical route mean? | One or more transport decisions plus the current transport posture |
| `transport_denial_record` | Why did the attempt fail, what is the typed remediation, and which inspector or export action is available? | One denied or deferred transport decision |

Both records are safe for product UI and metadata-oriented support export by
default. They carry opaque refs and fingerprints, never raw endpoints, raw
host names, raw IP addresses, raw proxy credentials, PAC bodies, certificate
PEM, private keys, raw SSH keys, tokens, cookies, request bodies, or response
bodies.

## Effective Route State

`effective_route_state_record` is the inspector's primary surface. It MUST
include:

- `endpoint_class_context`: the requesting `component_class`, optional
  `component_instance_ref`, `permission_class`, optional
  `permission_grant_ref`, `target_class`, `target_identity_ref`, optional
  `target_scope_ref`, `egress_class`, `auth_mode`, and optional
  `auth_handle_ref`. This block answers what is asking and what is being
  asked for.
- `origin_surface`: the `origin_scope` (`desktop_client`,
  `headless_runner`, `extension_host`, `remote_target`, `managed_service`,
  `system_browser`, or `local_helper`) and the inspector
  `inspector_surface_class` (`desktop_inspector`, `cli_explain_surface`,
  `headless_runner_explain`, `support_export_surface`, `admin_audit_surface`,
  `ai_broker_preflight`, `docs_update_inspector`, `remote_target_inspector`,
  `extension_host_inspector`, or `managed_control_plane_inspector`). A
  reviewer reading the record MUST be able to tell whether the request came
  from the client, a remote target, an extension, or a managed path.
- `execution_location`: the `execution_location_class` (`local_device`,
  `remote_target_workspace`, `managed_service_host`, `extension_sandbox`,
  `system_browser_helper`, `headless_runner_host`, `air_gap_local_only`, or
  `deny_all_offline_only`), optional `execution_target_ref`, the active
  `deployment_profile`, and the active `identity_mode`.
- `proxy_source`: the `proxy_resolution_mode`, `proxy_source_class`,
  `effective_proxy_ref`, optional `proxy_config_ref`, and a reviewable
  sentence. Records that ride a proxy MUST name the source rather than
  collapse it into a blank "system proxy" label.
- `pac_result`: the `pac_state_class` (including `pac_loaded_current`,
  `pac_loaded_stale`, `pac_resolution_error`, `pac_unreachable`,
  `pac_blocked_by_policy`, or `pac_disagrees_with_admin_policy`), opaque
  `pac_script_fingerprint`, `pac_decision_class`, optional
  `pac_resolved_proxy_ref`, and a reviewable sentence. PAC outcomes that did
  not run are labelled `pac_not_in_use` rather than omitted.
- `mirror_route_state`: the mirror `label_class`, `mirror_route_class`,
  `mirror_endpoint_ref`, `mirror_snapshot_ref`, `mirror_freshness_class`,
  and `public_origin_allowed`. A mirrored official source is both official
  and mirrored; those concepts do not collapse into one label.
- `direct_route_eligibility`: a boolean `direct_route_allowed` and one
  `direct_route_eligibility_class` from the closed nine-value set
  (`direct_disallowed_no_route`, `direct_disallowed_policy`,
  `direct_disallowed_mirror_only_profile`,
  `direct_disallowed_offline_or_air_gapped`,
  `direct_disallowed_trust_floor`, `direct_disallowed_managed_relay_required`,
  `direct_allowed_with_published_exception`,
  `direct_allowed_default_profile`, or `direct_not_applicable_local_only`).
  An `exception_ref` MUST be present when a direct route is allowed only via
  a published exception.
- `trust_posture`: `trust_store_source`, optional
  `org_ca_bundle_fingerprint`, optional `pinned_control_plane_trust_refs`
  and `trust_anchor_refs`, `ssh_host_key_provenance`, optional
  `ssh_host_key_fingerprint`, `client_certificate_state`, optional
  `client_certificate_handle_ref` and `client_certificate_fingerprint`, and
  the current `trust_failure_state`.
- `policy_constraints`: `policy_source_class`, `policy_freshness_class`,
  `active_policy_bundle_ref`, `policy_epoch_ref`, and a list of
  `policy_constraint_class` values from the closed ten-value set
  (`region_pinned_target`, `mirror_only_required`,
  `public_origin_forbidden`, `step_up_authentication_required`,
  `managed_handoff_only`, `no_egress_under_deny_all`,
  `direct_fallback_forbidden`, `offline_grace_window_only`,
  `approval_ticket_required`, or `no_active_constraint`).
- `mirror_constraints`: a list of `mirror_constraint_class` values from the
  closed eight-value set (`snapshot_signed_required`,
  `region_pinned_mirror`, `offline_bundle_only`,
  `mirror_freshness_floor_required`, `mirror_must_be_org_approved`,
  `mirror_only_no_direct`, `mirror_metadata_signed_evidence_required`, or
  `mirror_not_applicable`).
- `canonical_route`: see [Canonical Route](#canonical-route).
- `no_bypass_projection`: the same projection used elsewhere in the
  transport explainability surface contract. Mirror-only and air-gapped
  records MUST forbid both direct and insecure fallback.
- `inspector_actions`: the bounded inspector actions and per-profile
  availability — see [Inspector Actions](#inspector-actions).
- `remediation_hints`: typed hints — see [Remediation Hints](#remediation-hints).

The record MAY also link a `route_history_summary` covering recent decision
and attribution refs, with a `history_window_class` from the closed set
(`session_recent`, `workspace_recent`, `support_export_selected`,
`admin_audit_retained`, or `history_unavailable_in_profile`).

### Canonical Route

`canonical_route` resolves the chosen path to one canonical
`route_class` from the shared eleven-value vocabulary plus four typed fields:

| Field | Vocabulary |
|---|---|
| `audience_class` | `end_user_only`, `admin_only`, `support_export_only`, `signing_evidence_only`, `end_user_and_admin`, `all_inspectors` |
| `ttl_class` | `ephemeral_session_only`, `short_window`, `policy_window`, `persistent_until_revocation`, `no_ttl_applicable` |
| `approval_posture_class` | `end_user_implicit`, `user_consent_recorded`, `admin_policy_approved`, `signed_org_bundle_approved`, `managed_service_default`, `exception_required`, `approval_not_applicable` |
| `revocation_class` | `revocable_end_user`, `revocable_admin_policy`, `revocable_managed_service`, `revocable_via_exception_only`, `not_revocable_immutable`, `revocation_not_applicable` |

The `ttl_seconds` field is optional and machine-readable; the
`ttl_class` is required. An effective route MUST NOT carry a free-form path
description in place of a canonical route class.

### Inspector Actions

The record MUST include one `inspector_action_projection` per action class
in the closed five-value set:

- `copy_summary` — copy a privacy-safe sentence summary of the route.
- `open_policy_details` — open the policy bundle, epoch, and constraint
  detail behind the route.
- `open_trust_proof` — open the certificate, trust path, or SSH host-proof
  detail card.
- `open_route_history` — open the recent decision/attribution history for
  the same target and component.
- `export_packet` — export the metadata-safe inspector packet (decision,
  effective route, denial when applicable, summary strip, endpoint row,
  certificate card, history rows).

Each action carries an `availability_class` from the closed eight-value set
(`available`, `unavailable_redaction_restricted`,
`unavailable_air_gapped_no_export`, `unavailable_managed_policy_blocked`,
`unavailable_offline_packet_only`,
`unavailable_remote_target_no_local_state`,
`unavailable_admin_only_for_user`, or
`unavailable_unsupported_for_profile`) and an
`unavailable_reason_class` from the closed ten-value set
(`profile_air_gapped_no_export`, `managed_policy_disallows_export`,
`offline_packet_only_no_live_export`,
`redaction_restricted_above_default`, `remote_target_no_local_state`,
`extension_host_no_route_history`, `cli_headless_no_ui_action`,
`admin_only_action_not_available_to_user`,
`ai_broker_preflight_no_history_yet`, or
`not_applicable_no_unavailability`). When an action is `available`, the
unavailable reason MUST be `not_applicable_no_unavailability`. When an
action is unavailable, the reason MUST NOT be
`not_applicable_no_unavailability`.

Profiles MUST observe the following availability floors:

| Profile | Forced unavailability |
|---|---|
| `air_gapped` | `export_packet` and any open-action that requires a live network MUST be `unavailable_air_gapped_no_export` |
| `managed_cloud` with managed export disallowed | `export_packet` MUST be `unavailable_managed_policy_blocked` |
| `cli_explain_surface` | UI-only actions (`open_policy_details`, `open_trust_proof`, `open_route_history`) MUST be `unavailable` with reason `cli_headless_no_ui_action` |
| `extension_host_inspector` | `open_route_history` MUST be `unavailable` with reason `extension_host_no_route_history` |
| `ai_broker_preflight` | `open_route_history` MUST be `unavailable` with reason `ai_broker_preflight_no_history_yet` when no history has been written |
| `remote_target_inspector` | Actions that require local state (e.g. `open_route_history` against the workstation cache) MUST be `unavailable` with reason `remote_target_no_local_state` |

### Remediation Hints

`remediation_hints` is a list keyed by `remediation_hint_kind_class` from
the closed six-value set:

- `proxy_pac_mismatch` — `hint_class` drawn from
  `review_proxy_settings`, `review_pac_resolution`, `switch_to_system_proxy`,
  `switch_to_manual_admin_policy`, `refresh_pac_script`,
  `request_admin_proxy_review`, or `no_proxy_repair_required`.
- `certificate_trust_failure` — `hint_class` drawn from `add_ca_bundle`,
  `refresh_ca_bundle`, `refresh_pinned_root`, `request_admin_trust_review`,
  `rotate_trust_anchor`, or `no_trust_repair_required`.
- `client_certificate_absence` — `hint_class` drawn from
  `renew_client_certificate`, `request_managed_handle`,
  `request_admin_to_provision_handle`, `switch_to_supported_auth_path`, or
  `no_client_cert_repair_required`.
- `ssh_host_key_drift` — `hint_class` drawn from `review_ssh_host_proof`,
  `request_admin_to_publish_pin`, `refresh_known_hosts_from_policy`,
  `rebind_to_new_host_proof_with_admin`, or `no_ssh_repair_required`.
- `dns_or_mirror_issue` — `hint_class` drawn from `switch_mirror`,
  `refresh_mirror`, `request_admin_mirror_review`,
  `retry_after_mirror_refresh`, `request_dns_review`, or
  `no_mirror_repair_required`.
- `admin_vs_user_repair_boundary` — `hint_class` drawn from
  `end_user_can_repair`, `admin_must_repair`, `support_assisted_repair`,
  `managed_service_must_repair`, or `repair_unavailable_in_profile`.

Every hint also carries a `repair_boundary_class` from the same admin-vs-user
vocabulary so the user, admin, support, and release-evidence reviewer all
read the same routing.

## Transport Denial Record

`transport_denial_record` is the inspector's typed denial surface. Every
denial record MUST set:

- `denial_record_category` from the closed eight-value set
  (`blocked_public_fallback`, `conflicting_proxy_state`,
  `expired_mirror_metadata`, `certificate_trust_failure`,
  `ssh_host_proof_failure`, `policy_prohibition`, `offline_only_mode`, or
  `client_certificate_absent`).
- `denial_category` from the existing transport denial-category vocabulary
  (`proxy`, `certificate`, `policy`, `offline`, `auth`,
  `mirror_unavailable`, `ssh_host_proof`, `egress`, `deny_all`, or
  `unknown_requires_review`).
- `transport_decision_code` from the denial-only set (`deny_policy`,
  `deny_proxy_resolution`, `deny_trust`, `offline_deferred`, `stale_mirror`,
  `unsupported_auth_mode`).
- `outcome_class` from the denial-only outcome subset.
- `denial_subject` covering component, permission, egress, target, auth,
  origin, route, mirror label, mirror route, and `public_origin_allowed`.
- `denial_evidence` covering `deny_reason_class`, `trust_failure_state`,
  `client_certificate_state`, `mirror_freshness_class`,
  `conflict_state_class`, `offline_state_class`, optional `policy_rule_ref`,
  `user_setting_ref`, and `approval_ticket_ref`.
- `no_bypass_projection`. Every denial MUST set
  `no_bypass_status=no_bypass_enforced`, `direct_fallback_allowed=false`,
  and `insecure_fallback_allowed=false`. A denial MUST NOT permit the
  attempt to silently upgrade to a direct public fallback.
- `required_remediation`: at least one typed action drawn from the closed
  fourteen-value `remediation_action_class` set, paired with the
  `remediation_kind_class` and `repair_boundary_class`.
- `export_actions`: one entry per `export_action_class` from the closed
  five-value set (`copy_summary`, `open_policy_details`, `open_trust_proof`,
  `open_route_history`, `export_packet`) with the same per-profile
  availability rules as the inspector actions on the effective-route
  record. The two surfaces MUST agree: if the effective-route record marks
  `export_packet` as unavailable for a profile, the linked denial record
  MUST do the same.
- `repair_boundary`: a primary `admin_or_user_repair_boundary_class` value
  plus optional secondary boundary classes when the denial routes through
  more than one repair authority.

The schema enforces category-specific consistency:

| Category | Required pinning |
|---|---|
| `blocked_public_fallback` | `public_origin_allowed=false`; `silent_direct_origin_fallback` listed in `forbidden_bypass_classes` |
| `conflicting_proxy_state` | `conflict_state_class` is non-`proxy_conflict_not_applicable`; `transport_decision_code=deny_proxy_resolution` |
| `expired_mirror_metadata` | `mirror_freshness_class` is `mirror_past_extended_window` or `mirror_freshness_unknown`; `public_origin_allowed=false`; `transport_decision_code=stale_mirror` |
| `certificate_trust_failure` | `trust_failure_state` is non-`none`; `transport_decision_code=deny_trust` |
| `ssh_host_proof_failure` | `trust_failure_state` is `ssh_host_key_unknown` or `ssh_host_key_mismatch`; `transport_decision_code=deny_trust` |
| `policy_prohibition` | `deny_reason_class` is one of the typed policy/permission/approval values; `transport_decision_code` is `deny_policy` or `unsupported_auth_mode` |
| `offline_only_mode` | `offline_state_class` names an active offline state; `public_origin_allowed=false`; `transport_decision_code` is `deny_policy` or `offline_deferred` |
| `client_certificate_absent` | `client_certificate_state` is `required_missing`, `expired`, `handle_unavailable`, or `policy_blocked` |

## No-Direct-Fallback Rule

A network-capable surface MUST NOT silently upgrade from a mirror, proxy, or
policy-bound route to a direct public fallback when policy or trust forbids
it. The inspector enforces the rule at three layers:

1. The `effective_route_state_record` carries `direct_route_eligibility`. A
   `direct_route_allowed=false` value MUST coincide with
   `no_bypass_projection.direct_fallback_allowed=false` and
   `no_bypass_projection.insecure_fallback_allowed=false`.
2. Mirror-only routes (`mirror_only_no_direct_allowed`) MUST set
   `public_origin_allowed=false` and forbid both direct and insecure
   fallback.
3. Air-gapped, deny-all, and offline execution locations MUST forbid public
   origin.

The denial record reasserts the same rule. A `blocked_public_fallback`
record names `silent_direct_origin_fallback` in `forbidden_bypass_classes`
and refuses to mark `public_origin_allowed=true`.

## Cross-Profile Consistency

Desktop, CLI/headless, AI broker preflight, docs/update inspectors,
extension host inspectors, remote target inspectors, managed-control plane
inspectors, support exports, and admin audit surfaces read the same record
fields. They MUST preserve:

- the same enum values;
- the same opaque refs and fingerprints;
- the same canonical route, audience, TTL, approval posture, and revocation
  class;
- the same redaction posture and `export_safe` value;
- the same `no_bypass_projection`; and
- the same inspector or export action availability per profile.

User-facing labels MAY be localized or shortened. Underlying record values
must remain stable for automation, CLI explain output, support export, admin
audit, and release evidence.

## Per-Profile Coverage

The fixture corpus includes worked inspector cases for the eight profiles
called out by the spec:

| Fixture | Origin surface | Execution location | Outcome |
|---|---|---|---|
| `desktop_pac_proxy.yaml` | `desktop_inspector` | `local_device` | `allow` via PAC-resolved org-CA route |
| `cli_headless_manual_proxy.yaml` | `cli_explain_surface` | `headless_runner_host` | `allow` via manual proxy with parity to the desktop inspector |
| `remote_target_ssh_workspace.yaml` | `remote_target_inspector` | `remote_target_workspace` | `deny_trust` for an SSH host-key mismatch |
| `extension_host_ai_tool_blocked.yaml` | `extension_host_inspector` | `extension_sandbox` | `deny_policy` for an AI tool egress block |
| `ai_broker_route_preview.yaml` | `ai_broker_preflight` | `local_device` | `allow_mirror` preview into a vendor-published mirror |
| `docs_update_mirror_only.yaml` | `docs_update_inspector` | `local_device` | `stale_mirror` denial for an expired docs mirror |
| `managed_control_plane.yaml` | `managed_control_plane_inspector` | `managed_service_host` | `allow` via managed-relay route with admin-only export |
| `air_gapped_offline_bundle.yaml` | `support_export_surface` | `air_gap_local_only` | `offline_deferred` packet-only review with no live export |

Each fixture binds an `effective_route_state_record` to the underlying
`transport_decision_record` and pins the inspector actions, remediation
hints, and (where applicable) `transport_denial_record` so a reviewer can
read the chain end to end.

## Conformance Checklist

A reviewer auditing a transport-inspector change MUST be able to confirm:

- every `effective_route_state_record` names one origin scope, inspector
  surface class, and execution location;
- every record names a canonical route class with audience, TTL, approval
  posture, and revocation semantics;
- direct-route eligibility, mirror route state, and the no-bypass projection
  agree on whether a direct public fallback is permitted;
- every record carries one entry per inspector action class with a typed
  per-profile availability and unavailable reason;
- every applicable failure family (proxy/PAC, certificate, client
  certificate, SSH host key, DNS or mirror, admin-vs-user) has a typed
  remediation hint with a repair-boundary class;
- every `transport_denial_record` enforces no-bypass posture and forbids
  direct or insecure fallback;
- every denial category is paired with the typed evidence the schema
  requires; and
- effective-route and denial records agree on per-profile export
  availability so a desktop-only or CLI-only happy path cannot drop a
  surface that the other inspector advertises.
