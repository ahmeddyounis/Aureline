# Docs / Help / About / service-health badge parity packet

This packet freezes the *cross-surface parity contract* for the badges and
disclosures that appear on:

- embedded docs panes and docs search results,
- Help / About,
- service-health,
- migration/help pivots,
- release center artifact rows, and
- support-bundle / support-summary exports.

The goal is to prevent “truth drift” where each surface invents its own
wording for the same condition (stale mirror, version mismatch, policy block,
browser-only route, community handoff, etc.). Surfaces remain free to change
layout, but **they MUST project the same badge fields and MUST render the same
export-safe wording classes for the same condition**.

This packet is pre-implementation: it defines the governed record shape, the
parity matrix, and the fixture corpus used by audits and release checks. It
does not implement docs publishing, service-health backends, or UI rendering.

## Companion artifacts

- [`/schemas/docs/help_badge_projection.schema.json`](../../schemas/docs/help_badge_projection.schema.json)
  — cross-surface projection schema. Any surface that renders a help/docs/About/service-health
  badge emits (or can be audited as if it emitted) one `help_badge_projection_record`.
- [`/artifacts/docs/help_parity_matrix.yaml`](../../artifacts/docs/help_parity_matrix.yaml)
  — parity matrix enumerating required fields, allowed disagreement rules, and the case set.
- [`/fixtures/docs/help_about_cases/`](../../fixtures/docs/help_about_cases)
  — worked parity cases (mirror stale, pack missing, version mismatch, provider outage, community
  handoff, browser-only, out-of-scope cached content, exact-build mismatch).
- [`/artifacts/docs/embedded_docs_help_parity_packet.md`](../../artifacts/docs/embedded_docs_help_parity_packet.md)
  — narrower parity packet for embedded docs/help panes (cache/offline/locale/stale-example parity). This packet
  layers a *badge/disclosure* parity envelope across additional surfaces (service-health, release center, support
  exports) without redefining the embedded-pane contract.

## Normative sources this packet composes

This packet does not mint new truth vocabularies. It composes and requires
field-for-field agreement on the existing contracts below:

- [`/docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — source class, version-match, freshness, client-scope, degraded-cause, and allowed
  browser-handoff reason subset.
- [`/docs/docs/help_about_service_health_routes.md`](./help_about_service_health_routes.md)
  and [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  — exact-build applicability, external-open policy, browser-handoff reason, community/support
  destination trust/boundary, and screenshot/export safety.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  — one build identity rendered by Help/About, release center, and support exports.
- [`/docs/release/release_center_object_model_contract.md`](../release/release_center_object_model_contract.md)
  and [`/schemas/release/release_center_object.schema.json`](../../schemas/release/release_center_object.schema.json)
  — exact-build backreferences and parity links that release center rows and headless automation
  export without translation.
- [`/docs/about/about_provenance_and_boundary_contract.md`](../about/about_provenance_and_boundary_contract.md)
  and [`/schemas/about/about_card.schema.json`](../../schemas/about/about_card.schema.json)
  — About/community handoff routes and export-safe disclosures.

If this packet conflicts with any source above, the source wins and this
packet MUST be updated in the same change.

## Parity packet fields (what must not drift)

For any *logically identical condition* (same underlying docs pack, same route,
same build identity, same client scope), the surfaces in scope MUST agree on
the fields below as a single projection record:

- `source_class`
- `exact_build_applicability`
- `running_build_identity_ref` (when build-bound)
- `version_match_state` (when build-bound)
- `support_class`
- `client_scopes`
- `freshness_class`
- `availability_state`
- `service_contract_state` (when the row is service-backed)
- `degraded_state_cause` (when non-green on freshness/version/service/scope)
- `external_open_policy`
- `browser_handoff_reason` (when an external/browser route exists)
- `community_handoff_route` (when the surface offers “community” routing)
- `release_center_event_ref` and `release_center_exact_build_identity_ref`
  (when the surface is release-bearing or cites release-center history)

Surfaces MAY add additional context (timestamps, owner refs, repair hooks) as
long as they do not change the meaning of the projection. Surfaces MUST NOT
replace these fields with surface-local free-form text.

## Screenshot-safe / export-safe wording classes

Surfaces render *copy* by mapping the projection fields to stable wording
classes (not by writing ad-hoc sentences per surface). The mapping must be
usable in screenshots and support-bundle exports, so it avoids raw URLs,
private endpoints, or surface-specific jargon.

### Browser-handoff reason copy (frozen subset)

When `external_open_policy != not_permitted`, the surface MUST quote exactly
one `browser_handoff_reason` from ADR 0013’s allowed subset, and MUST map it
to the same wording class everywhere:

| `browser_handoff_reason` | Export-safe meaning (must be equivalent everywhere) |
| --- | --- |
| `external_docs_or_runbook` | The authoritative content lives outside the product; opening in a browser is required or offered as a safe fallback. |
| `provider_consent_flow` | This action requires an explicit consent flow that cannot be completed safely in-product. |
| `provider_admin_delegation` | This requires an admin delegation step that must occur on the provider side. |
| `license_or_portal_acceptance` | This requires accepting license/portal terms outside the product boundary. |
| `admin_only_surface` | This route is admin-only and must be performed on an admin surface outside the product. |
| `step_up_required` | This route requires step-up authentication outside the product boundary. |
| `mutation_not_supported_in_product` | The product can inspect state, but the mutation must happen outside the product. |

### Client-scope wording rule

When `client_scopes` excludes the rendering client, the surface MUST preserve
the exclusion as a first-class disclosure (not a silent suppression) and MUST
set `degraded_state_cause = client_scope_excludes_surface`.

### Version applicability wording rule

When `exact_build_applicability != not_build_bound`, the surface MUST keep the
build applicability visible on the primary surface (not tooltip-only) and MUST
keep `version_match_state` and `running_build_identity_ref` mechanically
copyable/exportable.

## Case corpus

The worked case set lives under
[`/fixtures/docs/help_about_cases/`](../../fixtures/docs/help_about_cases).
The parity matrix
[`/artifacts/docs/help_parity_matrix.yaml`](../../artifacts/docs/help_parity_matrix.yaml)
defines which axes must match for each case and which axes may diverge with an
explicit rationale.

Adding a new state class requires adding:

1. a new parity-matrix row;
2. at least one new fixture case; and
3. (when new vocabulary is needed) a schema + ADR update in the same change.
