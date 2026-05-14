# Help / About / Service-Health Destination Descriptor Contract

This document freezes the shared destination-descriptor contract used by
Help, About, service-health, docs-browser, migration-center, onboarding,
provenance, community-handoff, and support-export surfaces before they
offer a route, browser handoff, device-code fallback, or local export.
The machine-readable boundary is
[`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json);
worked seed descriptors live in
[`/artifacts/docs/destination_descriptor_seed.yaml`](../../artifacts/docs/destination_descriptor_seed.yaml).

The eventual docs-help-service-health crate's Rust types are the schema
of record. This document and the JSON Schema export are the cross-tool
boundary every non-owning surface reads; if this document and the ADRs
disagree, the ADRs win and this document MUST be updated in the same
change.

Related governance artifacts:

- [`/artifacts/governance/source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml)
  — canonical owner-routing map that tells downstream reviewers which
  artifact owns route-bearing truth versus claim-bearing truth.
- [`/docs/governance/drift_blocking_rules.md`](../governance/drift_blocking_rules.md)
  — same-change-set and severity rules this descriptor family follows
  when route truth, source version, support class, or freshness changes.

## Why freeze this now

ADR 0013 froze the truth-source badge contract for docs / Help / About /
service-health rows. ADR 0010 froze typed browser handoff packets.
ADR 0015 froze host-owned embedded boundary cards. What stayed reserved
but unimplemented was the shared object that sits between those layers:
the destination descriptor.

Without that shared descriptor:

- Help, About, service-health, migration, onboarding, and support-export
  surfaces would each invent their own trust labels for official,
  community, local-only, and authenticated destinations.
- "Open in browser" would drift into free-form copy with no stable
  reason code, no clear ownership/boundary disclosure, and no
  export-safe meaning.
- Local-only, offline-cached, mirrored, missing, and
  version-mismatched routes would collapse into vague "available" or
  "learn more" affordances that support exports could not reconstruct.
- Support class, freshness, client scope, and route posture would blur
  together even though the UI spec requires those axes to remain
  separately addressable.

The destination descriptor closes that gap by giving every route-bearing
surface one stable family for trust class, owner, boundary,
source/version, exact-build applicability, support class, client scope,
freshness, availability, locale availability, offline behavior, route
class, auth expectation, issue-template support, data-exit boundary,
and screenshot-safe handoff reasons.

## Scope

Frozen at this revision:

- One `destination_descriptor_record` shape with a closed set of
  purpose classes, trust classes, owner classes, boundary classes,
  exact-build applicability values, availability states,
  locale-availability classes, offline behaviors, route classes,
  external-open policies, auth expectations, issue-template support
  states, data-exit boundaries, and disclosure-safety classes.
- Product-bound fields surfaces can project directly instead of
  restating them in surface-local copy:
  `destination_trust_class`, `owner_class`, `boundary_class`,
  `source_descriptor_kind`, `source_ref`, `source_revision_ref`,
  `display_source_version`, `running_build_identity_ref`,
  `exact_build_applicability`, `version_match_state`, `support_class`,
  `client_scopes`, `freshness_class`, `availability_state`,
  `locale_availability_class`, `offline_behavior`,
  `preferred_route_class`, `fallback_route_classes`,
  `external_open_policy`, `auth_expectation`,
  `issue_template_support`, and `data_exit_boundary`.
- Explicit route rules for `system_browser`, `embedded_surface`,
  `device_code`, `local_only`, and `offline_cached_snapshot`.
- The requirement that any browser or device-code handoff reason quoted
  by this descriptor family comes from the ADR-0013 subset and remains
  screenshot-safe and export-safe.

Install topology is a local-only About/diagnostics destination while the
installer and update surfaces are still alpha contracts. Route-bearing surfaces
should point to
[`/docs/install/install_topology_alpha.md`](../install/install_topology_alpha.md)
and consume the `aureline-install` surface projection instead of inventing
separate install-mode, channel, updater-owner, state-root, handler-owner, or
rollback labels.

Out of scope until a superseding decision row opens:

- The full browser-handoff packet body. This contract points at the
  ADR-0010 destination class and reason subset; the packet itself stays
  in
  [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json).
- The embedded-surface boundary packet body. Embedded routes quote the
  ADR-0015 boundary contract rather than re-embedding it here.
- Rendering details for Help/About/service-health pages. This document
  freezes the route vocabulary; layout and interaction details land in
  the consuming surfaces later.

## Record fields

The full field set lives in
[`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json).
The notable fields are:

- **Trust, owner, and boundary.**
  `destination_trust_class` answers whether the route is
  `official_public`, `official_authenticated`, `community`, or
  `local_only`.
  `owner_class` answers who owns the destination.
  `boundary_class` answers where bytes and authority live after the
  route is followed.
  `disclosure_mode` answers how that truth must be shown:
  inline for local/cached routes, the host-owned boundary card for
  embedded routes, and a pre-handoff review for browser or device-code
  routes.
- **Source and version.**
  `source_descriptor_kind`, `source_ref`, `source_revision_ref`, and
  `display_source_version` point back to the object that published the
  descriptor.
  `exact_build_applicability` tells consumers whether the route is
  `same_build_only`, a `compatible_minor_window`,
  a `release_channel_window`, or `not_build_bound`.
  When the route is build-bound, `running_build_identity_ref` and
  `version_match_state` are required and stay mechanically comparable to
  docs/help badge rows and exact-build evidence.
- **Support, scope, freshness, and availability.**
  `support_class`, `client_scopes`, `freshness_class`, and
  `availability_state` remain separate axes. A route may be official but
  stale, available but only on desktop, or community-supported but not
  installed locally.
- **Locale and offline posture.**
  `locale_availability_class`, `available_locales`, and
  `offline_behavior` let surfaces say whether a route is local,
  cached-only, mirror-verified offline, or unavailable when offline.
- **Route and handoff semantics.**
  `preferred_route_class` and `fallback_route_classes` make the primary
  route explicit.
  `external_open_policy` tells consumers whether browser open is
  forbidden, optional, required as the primary path, or required as a
  fallback when the in-product route stops being truthful.
  `auth_expectation`, `browser_handoff_reason`,
  `issue_template_support`, and `data_exit_boundary` make the route
  explainable before the user crosses a boundary.

## Route rules

### `local_only`

Use `local_only` when the destination never leaves the local product
boundary: build info, provenance, local diagnostics, or reviewed export
packets saved to disk.

Rules:

- ownership and local-only boundary MUST remain visible inline;
- `external_open_policy = not_permitted` is the default unless a
  separate optional browser route exists for the same logical object;
- no sign-in or browser redirect may be required to inspect core build
  facts.

### `offline_cached_snapshot`

Use `offline_cached_snapshot` when the route is fulfilled by a cached or
mirrored local copy whose freshness is narrower than live truth.

Rules:

- freshness, availability, and offline behavior MUST remain visible
  inline while the user reads;
- a cached row may offer `system_browser` as a fallback, but the
  descriptor MUST keep the cached or mirrored posture explicit instead of
  implying live authority;
- not-installed, stale, and version-mismatched states remain first-class
  route states, not tooltip-only caveats.

### `embedded_surface`

Use `embedded_surface` only when the host can render the ADR-0015
boundary card and keep owner/origin/boundary truth outside the embedded
body.

Rules:

- `disclosure_mode` MUST be `embedded_boundary_card`;
- the embedded route MUST not impersonate native trust, update, or
  approval chrome;
- if the embedded route narrows below its declared capability set,
  `external_open_policy = required_fallback_when_in_product_unavailable`
  is the truthful escape hatch.

### `system_browser`

Use `system_browser` when the destination truth lives outside the
product and the user is expected to cross into a browser route on this
device.

Rules:

- the route MUST quote one of the ADR-0013 handoff reasons and the
  corresponding ADR-0010 destination class;
- the reason and destination disclosure MUST remain screenshot-safe and
  export-safe;
- `required_primary_route` means the route is browser-first and the
  product may not imply in-product parity.

### `device_code`

Use `device_code` when authentication or gated portal access should not
fall back to an embedded password flow or an unlabeled browser jump.

Rules:

- device-code routes still use `pre_handoff_review`;
- only typed route metadata and the device code leave the boundary by
  default (`data_exit_boundary = device_code_only_no_payload`);
- the route reason remains one of the export-safe handoff reasons.

## Handoff reasons and disclosure safety

This descriptor family may quote only the browser-handoff reasons frozen
in ADR 0013:

- `external_docs_or_runbook`
- `provider_consent_flow`
- `provider_admin_delegation`
- `license_or_portal_acceptance`
- `admin_only_surface`
- `step_up_required`
- `mutation_not_supported_in_product`

These are the only reasons this contract allows because they remain safe
to show in screenshots, support exports, and evidence packets without
leaking raw URLs, secret-bearing payloads, or route-specific private
state.

Reasons outside that subset are non-conforming for Help, About,
service-health, docs-browser, migration, onboarding, and support-route
descriptors.

## Freshness and availability rules

The descriptor family intentionally separates:

- `freshness_class` for how current the route's source truth is,
- `availability_state` for whether the route can be used now,
- `offline_behavior` for what survives offline,
- `version_match_state` for build applicability,
- `support_class` for support commitment, and
- `client_scopes` for which clients may claim the route.

Rules:

- offline, cached, stale, mirror-only, not-installed, version-mismatched,
  unreachable, and policy-blocked routes MUST remain distinct states;
- a route that is `version_mismatched` MUST compute a non-green
  `version_match_state`;
- if the route is build-bound, `running_build_identity_ref` and
  `version_match_state` are mandatory;
- if the route is not build-bound, those fields stay null instead of
  inventing a fake version badge.

## Seed coverage

The worked seeds in
[`/artifacts/docs/destination_descriptor_seed.yaml`](../../artifacts/docs/destination_descriptor_seed.yaml)
cover the required families and degradations:

- packaged project docs (`local_only`);
- mirrored official docs (`offline_cached_snapshot` with optional
  browser fallback);
- local-only service-health continuity;
- embedded hosted service dashboard with mandatory browser fallback;
- external status feed (`system_browser`);
- local About/provenance packet;
- community handoff from About/help;
- local reviewed support-bundle export;
- official authenticated support portal;
- device-code support route;
- version-mismatched migration guidance; and
- onboarding/glossary content that is known but not installed.

Together these seeds exercise every required route class and the
material availability degradations the task calls for: cached, mirrored,
missing, local-only continuation, version mismatch, and browser/device
handoff.

## Related contracts

- [`/schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)
  — shared docs/help/About/service-health source, version, freshness,
  and browser-handoff-reason vocabulary.
- [`/docs/docs/docs_help_about_service_health_parity.md`](./docs_help_about_service_health_parity.md)
  and
  [`/schemas/docs/help_badge_projection.schema.json`](../../schemas/docs/help_badge_projection.schema.json)
  — cross-surface parity packet and projection schema that keep badge fields and handoff reasons identical across surfaces.
- [`/schemas/docs/docs_pack_manifest.schema.json`](../../schemas/docs/docs_pack_manifest.schema.json)
  — docs-pack source/version/locale/offline truth that docs-bearing
  destinations point into.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — typed browser-handoff packet envelope and destination classes.
- [`/schemas/ux/embedded_surface_boundary.schema.json`](../../schemas/ux/embedded_surface_boundary.schema.json)
  — host-owned embedded boundary card and downgrade states.
- [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  — exact-build identity resolved by build-bound descriptors.
- [`/schemas/migration/migration_session.schema.json`](../../schemas/migration/migration_session.schema.json)
  — migration/session refs that migration-help and issue-template routes
  may point into.

## Source anchors

- [`docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — source-of-truth ownership, shared badge parity fields, safe
  browser-handoff reason subset, and the reserved help destination
  descriptor this contract closes.
- [`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — browser-handoff packet and destination class vocabulary.
- [`docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  — host-owned embedded boundary card and system-browser-first auth
  rules.
- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md)
  — normative launch and supportability language for local-first support
  bundles, exact-build truth, and browser-handoff honesty.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  — About packet, service-health event, help destination descriptor, and
  community/support destination manifest requirements.
- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — rule that support class, freshness, and client scope stay separate
  cues and that browser handoff preserves object identity and return
  path.
