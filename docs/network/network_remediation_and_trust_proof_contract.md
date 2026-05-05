# Proxy/PAC/CA/SSH Remediation Card, Trust-Proof Packet, And Enterprise-Lab Export Contract

This contract freezes how network trust failures are explained and handed off
so enterprise diagnostics stay actionable without exposing secrets or
requiring a live vendor console. It binds two records to the shared
transport-governance vocabulary so every reviewer answers the same three
questions about a network trust failure:

1. What is the failure family, what endpoint class is failing, and which
   typed fix lane applies?
2. What was actually verified locally versus asserted by an upstream or
   mirror source, under which policy or entitlement epoch, against which
   trust root or host proof?
3. Can a separate admin or trust team review the failure offline, with
   redacted evidence, without a live vendor console?

The contract is a projection layer only. It does not implement an enterprise
proxy lab, a CA store, or an SSH stack.

## Companion Artifacts

- [`/docs/network/transport_governance_seed.md`](transport_governance_seed.md)
  defines the underlying component, permission, target, auth, route, proxy,
  trust, mirror, offline, and attribution vocabulary.
- [`/docs/network/transport_governance_packet_seed.md`](transport_governance_packet_seed.md)
  defines the source transport-decision record.
- [`/docs/network/transport_inspector_contract.md`](transport_inspector_contract.md)
  defines the effective-route and denial records that remediation cards
  project from.
- [`/docs/network/transport_explainability_surface_contract.md`](transport_explainability_surface_contract.md)
  defines the summary-strip, endpoint-row, certificate-card, and denied
  history surfaces.
- [`/schemas/network/network_remediation_card.schema.json`](../../schemas/network/network_remediation_card.schema.json)
  defines one `network_remediation_card_record`.
- [`/schemas/network/trust_proof_packet.schema.json`](../../schemas/network/trust_proof_packet.schema.json)
  defines one `trust_proof_packet_record`.
- [`/fixtures/network/network_trust_cases/`](../../fixtures/network/network_trust_cases/)
  contains worked cases for the enterprise proxy lab, sovereign mirror,
  offline bundle verification, and self-hosted target attach scenarios.

If this document and a schema disagree, the schema wins and this document
updates in the same change. If this document and the transport governance
seed disagree on enum values, the seed wins.

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

A `network_remediation_card_record` projects from a `transport_decision_record`
and the linked `effective_route_state_record` and (when applicable)
`transport_denial_record`. A `trust_proof_packet_record` projects from the
same chain plus the active `transport_posture` snapshot.

Both records are safe for product UI and for offline review by an enterprise
proxy/CA/SSH/mirror lab or by a separate admin team. They carry opaque refs,
fingerprints, and typed enums; they never carry raw URLs, raw host names, raw
IP addresses, raw proxy credentials, PAC script bodies, certificate PEM,
private keys, raw SSH keys, tokens, cookies, request bodies, response bodies,
or unrelated traffic.

| Record | Question answered | Required source |
|---|---|---|
| `network_remediation_card_record` | What is the failure family, what is the typed fix lane, and which export action is available on the current profile? | One transport decision plus the linked effective-route record (and a denial record when the decision was denied or deferred) |
| `trust_proof_packet_record` | What was verified locally versus asserted by an upstream or mirror source, under which policy or entitlement epoch, against which trust root or host proof, with what redaction state and export-safe summary? | The same transport decision plus the active transport-posture snapshot |

Every remediation card MUST link exactly one trust-proof packet so the
narrative ("what to do next") and the verifiable evidence ("what was actually
proved") travel together. The two records use the same enum values for the
shared transport-governance vocabulary.

## Remediation Card Families

`remediation_card_family` is drawn from the closed seven-value vocabulary:

- `missing_proxy_configuration` — system, environment, or manual proxy is
  required but unreachable, unauthenticated, or absent. Fix lane drawn from
  `review_proxy_settings_lane`, `switch_to_system_proxy_lane`,
  `switch_to_manual_admin_policy_lane`, or `request_admin_proxy_review_lane`.
  Failure class drawn from `proxy_unreachable`,
  `proxy_auth_required_no_handle`, `proxy_auth_denied`,
  `manual_user_setting_disagrees_with_admin_policy`, or
  `no_effective_proxy_resolved`.
- `pac_evaluation_error` — PAC script is unreachable, errored, blocked by
  policy, disagrees with admin policy, or is stale beyond freshness floor.
  Fix lane drawn from `review_pac_resolution_lane`, `refresh_pac_script_lane`,
  `request_admin_proxy_review_lane`, or
  `switch_to_manual_admin_policy_lane`. Failure class drawn from
  `pac_resolution_error`, `pac_unreachable`, `pac_blocked_by_policy`,
  `pac_disagrees_with_admin_policy`, or `pac_loaded_stale`.
- `direct_route_disallowed` — a direct public fallback was attempted or
  implied where policy, mirror-only profile, offline mode, trust floor, or
  managed-relay routing forbids it. Fix lane drawn from
  `request_admin_published_exception_lane`, `switch_mirror_lane`,
  `managed_service_handoff_lane`, `request_admin_proxy_review_lane`, or
  `no_repair_supported_lane`. Failure class drawn from the typed
  `direct_disallowed_*` set. Cards in this family MUST mark
  `endpoint_class_context.public_origin_allowed=false`.
- `ca_mismatch` — TLS chain validation failed against the active trust
  store, organization CA bundle, or pinned control-plane root. Fix lane
  drawn from `add_org_ca_bundle_lane`, `refresh_org_ca_bundle_lane`,
  `refresh_pinned_root_lane`, `rotate_trust_anchor_lane`, or
  `request_admin_trust_review_lane`. Failure class drawn from `ca_untrusted`,
  `hostname_mismatch`, `bundle_stale`, `pin_mismatch`, `rotation_required`,
  `trust_store_verification_failed`, `org_ca_bundle_not_loaded`, or
  `pinned_control_plane_trust_mismatch`.
- `ssh_host_proof_mismatch` — SSH host-key verification failed under
  strict mode, advisory mode, or first-use pin pending. Fix lane drawn
  from `review_ssh_host_proof_lane`, `request_admin_to_publish_pin_lane`,
  `refresh_known_hosts_from_policy_lane`, or
  `rebind_to_new_host_proof_with_admin_lane`. Failure class drawn from
  `ssh_host_key_unknown`, `ssh_host_key_mismatch`, or
  `ssh_host_key_strict_mode_rejected`.
- `client_certificate_absence` — required client certificate is missing,
  expired, has no usable handle, or is policy-blocked. Fix lane drawn from
  `renew_client_certificate_lane`, `request_managed_handle_lane`,
  `request_admin_to_provision_handle_lane`, or
  `switch_to_supported_auth_path_lane`. Failure class drawn from
  `required_missing`, `expired`, `handle_unavailable`, or `policy_blocked`.
- `mirror_freshness_or_signer_mismatch` — mirror is past its extended
  freshness window, has unknown freshness, fails signer verification, has
  unsigned metadata, or is unreachable. Fix lane drawn from
  `switch_mirror_lane`, `refresh_mirror_lane`,
  `request_admin_mirror_review_lane`, `retry_after_mirror_refresh_lane`, or
  `request_signed_offline_bundle_import_lane`. Failure class drawn from
  `mirror_past_extended_window`, `mirror_freshness_unknown`,
  `mirror_signer_mismatch`, `mirror_metadata_unsigned`, or
  `mirror_unreachable`.

Every card MUST also carry `endpoint_class_context` (component, permission,
target, target scope, tenant scope, egress, auth, origin, deployment
profile, and `public_origin_allowed`), `failure_summary` (typed failure
class plus the trust/client-cert/mirror-freshness/conflict/offline state
markers from the shared seed), `repair_boundary` (admin-versus-user with
optional secondary boundaries), `evidence_refs` (active policy bundle and
epoch, linked trust-proof packet, optional decision/denial/support refs),
`no_bypass_projection`, and one `export_actions` entry per action class.

## Trust-Proof Packet

`trust_proof_packet_record` is the verifiable side of the same failure. It
MUST include:

- `endpoint_class` — the same component, permission, egress, target, auth,
  origin, deployment-profile, and `public_origin_allowed` view that appears
  on the linked card and effective-route record.
- `trust_failure_class` — `trust_failure_family_class` drawn from the
  closed seven-value set (`ca_or_certificate_chain_failure`,
  `ssh_host_proof_failure`, `client_certificate_absence`,
  `mirror_signer_or_freshness_failure`,
  `proxy_or_pac_resolution_failure`, `policy_or_entitlement_block`, or
  `no_trust_failure`), the shared `trust_failure_state`, and an optional
  `verifiability_class` from the closed seven-value provenance set
  (`verified_locally_against_pinned_root`,
  `verified_locally_against_org_ca_bundle`,
  `verified_locally_against_offline_bundle`,
  `asserted_by_upstream_only_not_locally_verifiable`,
  `asserted_by_mirror_only_not_locally_verifiable`,
  `verification_pending_admin_handoff`, or
  `verification_not_applicable`).
- `policy_or_entitlement_epoch` — the active policy bundle ref, policy
  epoch ref, optional entitlement epoch ref, optional policy rule ref,
  policy source class, policy freshness class, and capture timestamp so
  the packet pins the rule the failure was observed under.
- `trust_root_or_host_proof_metadata` — `trust_material_kind` from the
  closed eight-value set (`tls_certificate_chain`,
  `pinned_control_plane_root`, `org_ca_bundle`, `ssh_host_key_pin`,
  `client_certificate_handle`, `mirror_signing_root`,
  `offline_bundle_signing_root`, `trust_material_not_applicable`) plus the
  matching family-specific metadata block: `trust_root_metadata` (trust
  store source, anchor refs, optional fingerprints and lifecycle dates),
  `ssh_host_proof_metadata` (host-key provenance, observed and pinned
  fingerprints, optional policy pin ref), `client_certificate_metadata`
  (state, handle ref, fingerprint, expiry), or `mirror_metadata` (endpoint
  ref, snapshot ref, freshness class, signer state class, optional signing
  root ref and signature fingerprint, snapshot capture timestamp).
- `target_scope` — the target class, target identity ref, and target scope
  ref the attempt rode under.
- `tenant_scope` — the tenant scope ref, tenant kind class, and identity
  mode so a sovereign-mirror or air-gapped tenant cannot collapse into the
  same scope as a managed-cloud tenant.
- `verification_provenance` — the closed eight-value
  `locally_verified` vocabulary, the closed six-value
  `upstream_or_mirror_asserted` vocabulary, the closed six-value
  `verification_handoff_pending` vocabulary, and a reviewable sentence so
  the packet is precise about what was proved locally versus what was
  asserted by an upstream or mirror source and what is awaiting an admin or
  managed-service handoff.
- `secret_material_attestation` — `raw_material_excluded=true` and a
  non-empty list of forbidden material classes drawn from the closed
  fourteen-value set (raw certificate PEM, raw CA bundle bytes, raw private
  key, raw SSH private key, raw SSH public key text, raw endpoint name, raw
  full URL, raw IP address, raw proxy credentials, raw PAC body, raw token
  or cookie, raw request body, raw response body, unrelated traffic
  payloads).
- `enterprise_lab_export` — `lab_export_class` from the closed five-value
  set (`exportable_to_enterprise_lab`, `exportable_redaction_restricted`,
  `lab_export_blocked_managed_policy`, `lab_export_blocked_air_gapped`,
  `lab_export_not_applicable`), `review_team_class` from the closed
  six-value set (`no_review_team_required`, `admin_team`,
  `trust_or_pki_team`, `ssh_or_remote_target_team`,
  `mirror_or_release_team`, `support_assisted_team`), an optional
  `lab_packet_ref`, and an optional `lab_export_audience_class` so the
  packet names which separate review team owns the offline review.
- `no_bypass_projection` — every packet pins
  `no_bypass_status=no_bypass_enforced`, `direct_fallback_allowed=false`,
  and `insecure_fallback_allowed=false`. A trust-proof packet MUST NOT
  permit a silent direct or insecure fallback regardless of family.
- `redaction_state` — `redaction_class` plus boolean flags asserting that
  raw endpoint, proxy credentials, PAC body, certificate PEM, private key,
  SSH key, token/cookie, request/response body, and unrelated traffic are
  all excluded. The default for all flags is `true`.
- `export_safe_remediation_summary` — one privacy-safe sentence naming
  the failure family and fix lane that the offline reviewer reads first.

The schema enforces family-specific consistency:

| Trust failure family | Required pinning |
|---|---|
| `ca_or_certificate_chain_failure` | `trust_material_kind` is `tls_certificate_chain`, `pinned_control_plane_root`, or `org_ca_bundle`; `trust_root_metadata` is populated |
| `ssh_host_proof_failure` | `trust_material_kind=ssh_host_key_pin`; `ssh_host_proof_metadata` is populated |
| `client_certificate_absence` | `client_certificate_metadata` is populated |
| `mirror_signer_or_freshness_failure` | `trust_material_kind=mirror_signing_root`; `mirror_metadata` is populated |

## Enterprise-Lab Export Behavior

A network trust failure is reviewable by a separate enterprise proxy/CA/SSH
or mirror lab when, and only when, all of the following hold on the linked
records:

1. `secret_material_attestation.raw_material_excluded=true` and the
   forbidden material list covers raw certificate PEM, raw CA bundle bytes,
   raw private key, raw SSH private key, raw SSH public key text, raw
   endpoint name, raw full URL, raw IP address, raw proxy credentials, raw
   PAC body, raw token or cookie, raw request body, raw response body, and
   unrelated traffic payloads.
2. `redaction_state` flags every raw-material exclusion as `true` for the
   chosen `redaction_class`.
3. `no_bypass_projection` is `no_bypass_enforced` with `direct_fallback`
   and `insecure_fallback` both `false`.
4. `enterprise_lab_export.lab_export_class` is
   `exportable_to_enterprise_lab` or `exportable_redaction_restricted`. A
   `lab_export_blocked_managed_policy` or `lab_export_blocked_air_gapped`
   packet MUST be reviewed in place; it MUST NOT cross the boundary as a
   redacted-but-still-secret bundle.
5. `review_team_class` names the separate review team that owns the
   offline review (`admin_team`, `trust_or_pki_team`,
   `ssh_or_remote_target_team`, `mirror_or_release_team`, or
   `support_assisted_team`). `no_review_team_required` MUST only appear
   when `lab_export_class=lab_export_not_applicable`.

The remediation card carries a parallel `enterprise_lab_export_posture`
block with the same `lab_export_class`, the same `review_team_handoff_class`,
and the audience class so a desktop, CLI, support, and lab inspector all
read the same routing.

When `enterprise_lab_export.lab_export_class` is
`lab_export_blocked_managed_policy` or `lab_export_blocked_air_gapped`, the
matching `export_actions[export_packet]` entry on the remediation card MUST
be `unavailable_managed_policy_blocked` or `unavailable_air_gapped_no_export`
respectively. The two surfaces MUST agree.

## Verification Provenance Rule

A trust-proof packet MUST be honest about the difference between locally
proved and upstream-asserted state:

- A `ca_or_certificate_chain_failure` packet that names a managed-service
  pinned root MUST list `tls_chain_verified_against_pinned_root` under
  `locally_verified`. If the pin itself was distributed by the managed
  service without a locally rotatable anchor, the packet MUST also list
  `trust_anchor_asserted_by_upstream_only` under
  `upstream_or_mirror_asserted`.
- A `mirror_signer_or_freshness_failure` packet MAY list
  `mirror_signature_verified_against_org_pin` under `locally_verified` only
  when the org pin is materialised on the device. Otherwise the packet MUST
  list `mirror_freshness_asserted_by_mirror_only` under
  `upstream_or_mirror_asserted`.
- A `ssh_host_proof_failure` packet MUST list
  `ssh_host_key_verified_against_admin_pin` only when an admin policy pin
  exists locally. A `first_use_pending_admin_pin` posture MUST instead list
  `awaiting_admin_pin_publication` under `verification_handoff_pending`.

This rule prevents the packet from rendering managed-service or
mirror-asserted state as locally proved when it is not.

## No-Direct-Fallback Rule

Both records reassert the no-direct-fallback rule:

1. `network_remediation_card_record.no_bypass_projection` carries
   `no_bypass_enforced`, `direct_fallback_allowed=false`, and
   `insecure_fallback_allowed=false` regardless of family. A
   `direct_route_disallowed` card MUST also pin
   `endpoint_class_context.public_origin_allowed=false`.
2. `trust_proof_packet_record.no_bypass_projection` enforces the same
   pinning. A trust failure cannot be silently upgraded to a direct or
   insecure attempt under any family, profile, or audience.

## Cross-Surface Consistency

Desktop, CLI/headless, AI broker preflight, docs/update inspectors,
extension host inspectors, remote target inspectors, managed-control plane
inspectors, support exports, admin audit surfaces, and enterprise-lab review
surfaces read the same record fields. They MUST preserve:

- the same enum values for family, failure class, fix lane, repair
  boundary, trust-material kind, verifiability class, and forbidden
  material;
- the same opaque refs and fingerprints;
- the same redaction posture and `export_safe` value;
- the same `no_bypass_projection`; and
- the same per-profile export availability.

User-facing labels MAY be localized or shortened. Underlying record values
must remain stable for automation, CLI explain output, support export, admin
audit, release evidence, and offline enterprise-lab review.

## Per-Scenario Coverage

The fixture corpus includes worked cases for the four scenarios called out
by the spec:

| Fixture | Scenario | Card family | Trust failure family |
|---|---|---|---|
| `enterprise_proxy_lab_pac_ca_failure.yaml` | Enterprise proxy lab evaluating a PAC + custom-CA failure on a managed-cloud tenant | `pac_evaluation_error` (with linked CA mismatch) | `proxy_or_pac_resolution_failure` |
| `sovereign_mirror_signer_mismatch.yaml` | Sovereign mirror tenant rejecting a mirror snapshot whose signer does not match the org pin | `mirror_freshness_or_signer_mismatch` | `mirror_signer_or_freshness_failure` |
| `offline_bundle_verification_failure.yaml` | Air-gapped tenant verifying an imported offline bundle that fails signature verification | `mirror_freshness_or_signer_mismatch` | `mirror_signer_or_freshness_failure` |
| `self_hosted_target_attach_ssh_host_proof.yaml` | Self-hosted SSH workspace attach failing strict-host verification with admin handoff | `ssh_host_proof_mismatch` | `ssh_host_proof_failure` |

Each fixture binds a remediation card to a trust-proof packet and pins the
shared decision/effective-route refs so a reviewer can read the chain end to
end.

## Conformance Checklist

A reviewer auditing a remediation-card or trust-proof-packet change MUST be
able to confirm:

- every `network_remediation_card_record` names one family from the closed
  seven-value set, one fix lane consistent with that family, one typed
  failure class, one repair boundary, and one entry per export action class;
- every card carries a single linked trust-proof packet ref so narrative and
  evidence travel together;
- every `trust_proof_packet_record` names one trust-failure family from the
  closed seven-value set, one trust-material kind consistent with that
  family, the policy or entitlement epoch the failure was observed under,
  the target scope, the tenant scope, and the verification provenance;
- every packet attests `raw_material_excluded=true` and lists the full set
  of forbidden material classes the redaction class entails;
- every packet pins `no_bypass_enforced` with `direct_fallback_allowed=false`
  and `insecure_fallback_allowed=false`;
- the card's `enterprise_lab_export_posture` and the packet's
  `enterprise_lab_export` agree on `lab_export_class` and the responsible
  review team class; and
- a desktop, CLI, support, and enterprise-lab reviewer all read the same
  failure family, fix lane, repair boundary, redaction state, and export
  availability without semantic drift.
