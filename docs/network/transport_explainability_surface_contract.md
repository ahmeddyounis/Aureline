# Transport Explainability Surface Contract

This contract freezes the user-facing projection layer for Aureline's
transport governance model. It defines how the transport summary strip,
endpoint row, certificate/detail card, and denied-attempt history row render
proxy, trust, policy, mirror, degraded/offline, and repair truth without
inventing a second network vocabulary.

The contract is a projection layer only. It does not implement a network
stack, proxy resolver, certificate store, SSH known-hosts store, mirror
service, policy backend, or repair runner.

## Companion Artifacts

- [`/docs/network/transport_governance_seed.md`](transport_governance_seed.md)
  defines the underlying component, permission, target, auth, route, proxy,
  trust, mirror, offline, and attribution vocabulary.
- [`/docs/network/transport_governance_packet_seed.md`](transport_governance_packet_seed.md)
  defines the transport decision record that these surfaces project.
- [`/schemas/network/transport_decision.schema.json`](../../schemas/network/transport_decision.schema.json)
  remains the source decision schema for route, trust, no-bypass, and repair
  hints.
- [`/schemas/network/network_attribution_record.schema.json`](../../schemas/network/network_attribution_record.schema.json)
  remains the event-attribution spine for allowed, denied, failed, deferred,
  and inbound callback transport events.
- [`/schemas/network/transport_summary_strip.schema.json`](../../schemas/network/transport_summary_strip.schema.json)
  defines the summary strip record.
- [`/schemas/network/endpoint_history_row.schema.json`](../../schemas/network/endpoint_history_row.schema.json)
  defines endpoint rows and denied-attempt history rows.
- [`/schemas/network/certificate_detail_card.schema.json`](../../schemas/network/certificate_detail_card.schema.json)
  defines certificate, trust path, client-certificate, and SSH host-proof
  detail cards.
- [`/fixtures/network/transport_explainability_cases/`](../../fixtures/network/transport_explainability_cases/)
  contains worked projection cases for PAC proxy routing, custom CA trust,
  mirror unavailable, policy blocked, and SSH host-proof mismatch.
- [`/docs/network/transport_inspector_contract.md`](transport_inspector_contract.md),
  [`/schemas/network/effective_route_state.schema.json`](../../schemas/network/effective_route_state.schema.json),
  [`/schemas/network/transport_denial.schema.json`](../../schemas/network/transport_denial.schema.json),
  and
  [`/fixtures/network/transport_inspector_cases/`](../../fixtures/network/transport_inspector_cases/)
  freeze the transport inspector that projects effective-route state and
  typed denial records on top of the same shared transport vocabulary.

If this document and a schema disagree, the schema wins and this document
updates in the same change. If this document and the transport governance
seed disagree on enum values, the governance seed wins and this document plus
the explainability schemas update together. If this contract disagrees with
the source specs below, the source specs win and all projection artifacts
update together.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` section 5.42, network, proxy,
  certificates, and transport-governance architecture.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix G,
  network and transport policy matrix.
- `.t2/docs/Aureline_Technical_Design_Document.md` transport governance
  section plus Appendices CX, CY, and CZ.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` section 18.14 and Appendix BS,
  transport, proxy, certificate, and mirror-state templates.

## Surface Boundary

Every network-capable surface that explains transport state to users, admins,
CLI callers, AI tools, docs/help, support exports, or release evidence MUST
project from a `transport_decision_record`, a `network_attribution_record`, or
the current `transport_posture` object. A surface MUST NOT derive private
terms such as "marketplace network error", "AI gateway offline", "bad cert",
or "remote issue" when a shared proxy, trust, mirror, policy, auth, or offline
code is available.

The four surface records answer different questions:

| Surface | Question answered | Required source |
|---|---|---|
| Transport summary strip | What is the current connectivity posture? | current posture plus recent decisions |
| Endpoint row | What can this component reach, through which route, with which last outcome? | latest decision or attribution for one endpoint |
| Certificate/detail card | Why did trust succeed or fail for this endpoint? | decision trust evidence or trust descriptor |
| Denied-attempt history row | What was refused or failed, why, and what repair path is safe? | terminal denied, deferred, stale, or trust-failed decision |

These records are safe for product UI and metadata-oriented support export by
default. They carry opaque refs and fingerprints, not raw endpoints, raw host
names, raw IP addresses, raw proxy credentials, PAC bodies, certificate PEM,
private keys, SSH host keys, tokens, cookies, request bodies, or response
bodies.

## Transport Summary Strip

The transport summary strip is a compact current-state record. It is used by
desktop status areas, CLI/headless explain output, diagnostics cards, docs and
update surfaces, admin inspection, AI/provider route previews, remote
connectors, and support exports.

The strip MUST include:

- `connectivity_mode`, using the same values as
  `offline_or_deny_all_state`.
- `effective_proxy_source`, including `proxy_resolution_mode`, a source class
  such as `system_proxy_settings`, `pac_script`, `environment_variables`,
  `manual_admin_policy`, `manual_user_setting`, `direct_no_proxy_selected`, or
  `proxy_source_unknown`, and opaque proxy/script refs where present.
- `trust_ca_state`, including `trust_store_source`, CA bundle or pinned root
  refs, SSH default provenance, and the current `trust_failure_state`.
- `policy_source`, including active policy bundle and epoch refs, freshness,
  and whether the source is local default, user/profile, signed org bundle,
  managed policy, offline bundle, or emergency policy.
- `mirror_posture`, including mirror route class, public-origin permission,
  mirror snapshot refs, and freshness.
- `degraded_offline_summary`, including the typed degraded or offline state,
  a reviewable sentence, and the time the state began when applicable.
- `no_bypass_projection`, stating whether direct or insecure fallback is
  allowed and which bypass classes are forbidden.
- `repair_actions`, using bounded action classes such as
  `review_proxy_settings`, `add_ca_bundle`, `refresh_policy`, `reauth`,
  `switch_mirror`, `refresh_mirror`, `review_ssh_host_proof`,
  `renew_client_certificate`, and `retry_current_context`.

The strip MAY summarize recent endpoint counts, but it MUST NOT hide a known
deny reason behind an aggregate "offline" or "network error" label.

## Endpoint Row

An endpoint row summarizes one inspectable target for one component. It is the
row used by endpoint inspectors, docs/update/registry panels, AI/provider
preflight, remote attach detail, CLI explain output, support export, and admin
audit views.

Each endpoint row MUST carry:

- `component_class` and optional component instance ref.
- `permission_class` and `egress_class`.
- `target_class` and opaque `target_identity_ref`.
- `auth_mode` and optional handle ref.
- route summary: `origin_scope`, `route_class`,
  `proxy_resolution_mode`, mirror/public label, mirror route class, and
  public-origin permission.
- `last_outcome`, including the transport decision code, outcome class, time,
  and whether the value is live, last-known-good, stale, current-denied, or
  in-flight.
- `reason_detail`, null on clean success and non-null on typed denial,
  trust failure, proxy failure, offline deferral, stale mirror, or unsupported
  auth.
- at least one repair path when the last outcome is denied, deferred, failed,
  or stale.

Rows MUST preserve mirror versus origin as a transport fact, not a provenance
substitute. A mirrored official docs pack is both official and mirrored; those
concepts do not collapse into one label.

## Certificate Detail Card

The certificate/detail card explains trust without leaking secret material. It
covers TLS server certificate chains, organization CA bundles, pinned
control-plane roots, offline trust roots, SSH host proofs, and client
certificate bindings.

Each card MUST include:

- a privacy-safe server or peer identity label and opaque identity ref;
- `trust_material_kind`, naming whether the card is about a TLS certificate
  chain, SSH host proof, client-certificate binding, pinned control-plane
  root, or offline bundle trust root;
- trust path fields: `trust_store_source`, CA source class, org CA bundle
  fingerprint, pinned root refs, and trust anchor refs;
- pinning or host-key posture: SSH host-key provenance, opaque fingerprint,
  and pinning state;
- certificate lifecycle fields: opaque chain and leaf fingerprints, issuer
  ref, validity window, expiry state, and client-certificate state where
  applicable;
- `trust_result`, including `trust_succeeded`, `trust_failure_state`, and a
  reviewable reason sentence;
- repair actions such as add CA bundle, refresh policy, reauth, review SSH
  host proof, renew client certificate, or retry with the current context;
- an attestation that raw certificate material, private keys, raw SSH keys,
  and raw endpoint names are excluded.

When trust fails because of CA, hostname, pin, SSH host proof, client
certificate, bundle freshness, or policy, the card MUST name that state. It
MUST NOT offer insecure retry, ignore-certificate, trust-any-host, or direct
origin bypass actions.

## Denied-Attempt History Row

Denied-attempt history is the durable local-first record of transport denials,
safe offline deferrals, stale-mirror refusals, and trust failures. It is not a
raw log viewer. It is searchable, exportable, and consistent with desktop,
CLI, support, admin, AI, docs, update, registry, package, and remote
connector surfaces.

Each row MUST carry:

- timestamp and source decision ref;
- component, permission, egress, target, auth, origin, and route classes;
- mirror versus origin label and direct-public fallback posture;
- transport decision code and outcome class;
- controlled denial category: `proxy`, `certificate`, `policy`, `offline`,
  `auth`, `mirror_unavailable`, `ssh_host_proof`, `egress`, `deny_all`, or
  `unknown_requires_review`;
- typed deny reason and trust failure state when applicable;
- a repair path and retry posture;
- freshness and retention fields;
- the same no-bypass projection that prevented unsafe fallback.

The history row SHOULD link to a certificate/detail card or endpoint row when
one exists. It MUST remain useful without raw logs or provider-specific error
strings.

## Repair And No-Bypass Rules

Repair actions are projections of backend decisions, not overrides. A surface
MAY offer only actions that preserve policy, trust, and route boundaries:

| Repair action class | Allowed use |
|---|---|
| `review_proxy_settings` | Inspect or update the configured proxy source for the active profile |
| `add_ca_bundle` | Load or select an approved org CA bundle or offline trust root |
| `refresh_policy` | Request a signed policy or trust-root refresh |
| `reauth` | Refresh a credential handle without exposing raw secrets |
| `switch_mirror` | Switch to an approved alternate mirror when policy permits |
| `refresh_mirror` | Import or refresh an approved mirror snapshot |
| `review_ssh_host_proof` | Review changed SSH host proof or known-hosts policy |
| `renew_client_certificate` | Renew or replace an mTLS handle |
| `retry_current_context` | Retry the same target, origin, route, and auth context after repair |
| `inspect_only_no_action` | Open details without mutating state |

The following actions are forbidden on these surfaces unless an explicit
transport exception and waiver is active and visible on the decision record:

- direct origin fallback from a mirror-only profile;
- disabling certificate validation;
- ignoring a pin, host-key mismatch, or CA failure;
- bypassing proxy resolution;
- hidden CLI/desktop divergence;
- extension ambient egress without a grant;
- destructive offline deferral;
- feature-specific retry loops under deny-all posture;
- generic "try again" when a typed reason is known.

## Cross-Surface Projection

Desktop, CLI/headless, AI, docs, updates, registries, remote connectors,
support exports, and admin views read the same record fields. Surfaces may
render different amounts of detail, but they MUST preserve:

- the same enum values;
- the same opaque refs;
- the same decision or attribution ids;
- the same redaction posture;
- the same no-bypass posture; and
- the same repair action class.

User-facing labels may be localized or shortened. The underlying record values
must remain stable for automation, CLI output, support export, admin audit,
and release evidence.

## Fixture Coverage

The fixture directory includes a manifest and worked bundles that demonstrate:

- PAC-sourced proxy with org CA trust and no direct fallback;
- custom CA / organization CA bundle trust success;
- mirror unavailable or stale mirror refusal without public fallback;
- policy-blocked egress with refresh-policy and narrower-scope guidance;
- SSH host-proof mismatch under strict mode with no trust-on-first-use
  promotion.

Fixtures are projection examples. They intentionally reuse opaque refs from
the existing transport decision cases where useful, and they avoid raw
customer endpoints, raw host names, raw IPs, raw certificate bodies, raw keys,
tokens, payloads, and provider-specific jargon.
