# Managed Metering, Quota, and Usage-Export Explainability Contract

This contract freezes the vocabulary and row shapes for optional
managed-service metering, quota display, and customer-visible usage
exports. It exists so future AI, collaboration, remote-workspace,
registry, support, and entitlement surfaces cannot invent hidden
billing semantics, treat stale meters as current truth, or make users
and support infer the scope or time basis of exported usage.

The contract is normative for product surfaces that display quota,
usage, spend attribution, stale-meter state, or export rows. Where this
document disagrees with the source product and architecture specs, the
source specs win and this document must be updated in the same change.
Where a downstream surface invents a conflicting label, this document
wins and that surface is non-conforming.

Companion artifacts:

- [`/schemas/managed/quota_state.schema.json`](../../schemas/managed/quota_state.schema.json)
  - boundary schema for one `quota_state_record`, the object every
  quota or usage display reads before claiming a current amount.
- [`/schemas/governance/usage_export_record.schema.json`](../../schemas/governance/usage_export_record.schema.json)
  - boundary schema for one `usage_export_packet_record`, the packet
  envelope that carries one export window's rows plus explicit
  availability and offboarding posture.
- [`/schemas/managed/usage_export_row.schema.json`](../../schemas/managed/usage_export_row.schema.json)
  - boundary schema for one `usage_export_row_record`, the row shape
  for monthly or bounded customer exports.
- [`/fixtures/managed/metering_cases/`](../../fixtures/managed/metering_cases/)
  - worked cases covering authoritative, cached, estimated,
  unavailable, and policy-suppressed usage states plus export rows.
- [`/fixtures/governance/usage_export_cases/`](../../fixtures/governance/usage_export_cases/)
  - worked cases covering packet-level export availability, entitlement-loss
  review, renewal-window posture, local-only absence, and non-coercive
  downgrade narrowing.

Inherited contracts:

- [`/docs/service/managed_service_seed.md`](../service/managed_service_seed.md)
  provides managed-service SLO, degradation, retention, deletion, and
  local-core non-dependence vocabulary. This contract narrows
  `managed_entitlement_usage`; it does not replace the service row.
- [`/docs/governance/data_portability_and_exit_matrix.md`](../governance/data_portability_and_exit_matrix.md)
  defines the portability and offboarding posture for usage and billing
  exports.
- [`/docs/ai/model_graduation_and_budget_contract.md`](../ai/model_graduation_and_budget_contract.md)
  defines AI budget-routing and route-selection disclosure. AI spend
  receipts may cite quota states and usage export rows, but AI surfaces
  must keep route-selection truth separate from export accounting.
- [`/docs/governance/record_class_governance.md`](../governance/record_class_governance.md)
  and [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  define the `entitlement_usage_export_packet` record class.
- [`/docs/governance/usage_export_and_offboarding_contract.md`](../governance/usage_export_and_offboarding_contract.md)
  defines the packet-level usage export envelope, default posture matrix,
  and the reference-without-embedding rule between usage export and
  offboarding exit packets.
- [`/docs/governance/time_semantics.md`](../governance/time_semantics.md)
  and [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  provide shared timestamp, freshness, and degraded-state language.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` sections 5.23, 5.24, 5.33, 5.34,
  13.15, and Appendix AN.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections
  9.6, 18.10, and 21.9.

## Scope

Frozen at this revision:

- the authority classes a meter or quota display may claim:
  `authoritative`, `cached`, `estimated`, `unavailable`, and
  `policy_suppressed`;
- the source classes behind those claims:
  `service_authoritative`, `service_cached`, `local_estimate`,
  `service_unavailable`, and `policy_suppressed`;
- stale-meter label classes, reset-window classes, quota-status
  classes, measurement units, time-basis classes, and spend-
  attribution dimensions;
- the `quota_state_record` payload that UI, CLI/headless, support,
  offboarding, and AI surfaces consume before presenting quota or
  usage state;
- the `usage_export_row_record` payload that monthly or bounded
  exports emit so customer finance, support, and admins do not infer
  scope, source, time basis, or caveats;
- downgrade rules for missing, stale, or policy-suppressed service
  usage.

Out of scope:

- a billing engine, pricing model, taxation, invoicing, rating
  ledger, or managed admin console;
- raw vendor invoices, raw payment processor records, raw prompt
  bodies, raw code payloads, raw directory attributes, raw URLs, raw
  user emails, or raw tenant names;
- legal interpretation of contract-specific billing terms.

## Core Rule

Quota and usage are not a number. They are a claim with authority,
source, scope, time basis, unit, reset window, freshness, caveats, and
downgrade behavior.

Any surface that renders a usage or quota value must carry the
matching authority class and must narrow its claim when the service
meter is stale, unavailable, estimated locally, or suppressed by
policy. A surface that lacks a current authoritative meter may not
display precise current usage as if the service had supplied it.

Local-core workflows remain non-blocking. Metering failures may narrow
the specific managed action whose spend cannot be bounded, but they
must not block opening, editing, saving, searching, local Git, local
tasks, direct local/BYOK AI, or already-authorized local automation.

## Authority And Source Classes

| Authority class | Source class | Meaning | Required surface behavior |
|---|---|---|---|
| `authoritative` | `service_authoritative` | Current service meter or quota state was fetched from the managed authority within the declared freshness window. | May show the value with authoritative confidence and cite `last_refreshed_at`. |
| `cached` | `service_cached` | Last-known service state is being reused. It may still be inside a freshness floor or beyond it. | Must show a stale-meter label and last-refresh timestamp. Claims are narrower than authoritative. |
| `estimated` | `local_estimate` | Client or edge counter estimates usage before service reconciliation. | Must show estimate language. May not be used as invoice truth, final support truth, or precise quota truth. |
| `unavailable` | `service_unavailable` | The service meter cannot currently supply a value. | Must show unavailable state or a gap marker. May not substitute zero or hide the row. |
| `policy_suppressed` | `policy_suppressed` | Policy allows the surface to acknowledge usage exists but forbids displaying one or more fields. | Must show suppression and the policy ref. May not imply usage is zero, free, or unknown due to outage. |

## Quota State Record

A `quota_state_record` is the current explainability object for quota,
usage, or spend status. It is not a bill. It can be quoted by:

- AI pre-invocation budget chips and route fallback explainers;
- managed-service dashboards and admin summaries;
- CLI/headless status output;
- support packets and support-bundle previews;
- offboarding and usage-export manifests.

Minimum fields:

| Field | Purpose |
|---|---|
| `quota_state_id` | Stable opaque id safe for logs, exports, and support packets. |
| `scope` | Tenant/org/user/workspace/repo/feature/provider/model/action scope and parent scope refs. |
| `quota_family_class` | What budget family is being measured, such as AI tokens, tool invocations, workspace-hours, sync bytes, or registry downloads. |
| `quota_unit_class` | The measurement unit. The unit is explicit even when the amount is suppressed or unavailable. |
| `authority` | Authority class, source class, meter reading class, and confidence label. |
| `numeric_readout` | Optional amount/limit/remaining readout plus precision class. Values are nullable because unavailable and policy-suppressed states are explicit states, not numbers. |
| `reset_window` | Reset policy, current window, next reset, and source ref. |
| `stale_meter` | Label class, last refresh, freshness floor, and required disclosure text. |
| `downgrade_behavior` | Product behavior for AI, dashboard, support, and offboarding consumers. |
| `spend_attribution` | Dimensions that explain who or what the usage is attributed to. |
| `caveats` | Typed caveats such as late events, rounded amounts, cache staleness, local estimate, unavailable service, or policy suppression. |

### Quota status classes

Quota status is separate from authority:

- `within_quota` - current source says the action is within quota.
- `soft_limit_warning` - current source says warning threshold is
  crossed but managed action may continue.
- `hard_limit_exhausted` - current source says no quota remains.
- `unavailable_cannot_bound` - service data is unavailable and the
  managed action cannot be bounded safely.
- `blocked_policy_suppressed` - policy prevents enough disclosure or
  authority to proceed.
- `disabled_not_applicable` - this quota family is disabled or not
  applicable for the scope.

A `hard_limit_exhausted` state must name the reset window or state
that the reset window is unavailable. An `unavailable_cannot_bound`
state must not display a precise remaining amount.

### Reset windows

Supported reset-window classes:

- `calendar_month_utc`
- `rolling_24h`
- `rolling_7d`
- `rolling_30d`
- `contract_term`
- `manual_admin_reset`
- `no_reset_window`
- `unknown_unavailable`
- `policy_suppressed`

Every rendered reset window must name its time basis. If the service
cannot provide reset timing, the UI must render `unknown_unavailable`
or `policy_suppressed`; it may not infer a local calendar reset.

## Spend Attribution

Every quota state and export row must include at least one spend
attribution dimension and must include all dimensions available for
the source. Supported dimensions:

- `tenant`
- `organization`
- `user`
- `workspace`
- `repository`
- `feature`
- `provider`
- `model`
- `action`
- `execution_locus`
- `region`
- `quota_family`
- `managed_service`
- `source_class`

Attribution refs are opaque. Raw usernames, emails, billing account
ids, invoice numbers, provider URLs, and prompt or code bodies never
appear in the metering boundary.

## Stale-Meter Labels

| Label class | When used | Required claim narrowing |
|---|---|---|
| `no_stale_label` | Fresh authoritative state. | May show authoritative confidence. |
| `cached_within_freshness_floor` | Cached service state is still inside a declared freshness floor. | Show last refresh and that the value is cached. |
| `cached_beyond_freshness_floor` | Cached service state is beyond the declared floor. | Stop precision claims; managed actions that require bounded spend must narrow or block. |
| `estimate_pending_reconciliation` | Local estimate has not been reconciled with the service. | Show estimate language and exclude from final usage truth. |
| `unavailable_meter` | No current or usable service meter exists. | Show unavailable/gap marker and block only actions requiring bounded spend. |
| `policy_suppressed_meter` | Policy suppresses amount, scope detail, spend, or model/provider disclosure. | Show policy suppression; do not imply zero usage or free usage. |

Stale labels must propagate to product UI, CLI/headless output,
support packets, usage exports, and offboarding packets. A stale meter
may not be hidden behind an indefinite spinner.

## Usage Export Rows

A `usage_export_row_record` is the atomic row for a monthly or bounded
usage export. Exports may be CSV or JSON, but the JSON row shape below
is the canonical boundary. CSV columns must preserve the same fields or
include a manifest mapping CSV column names back to these fields.

Minimum fields:

| Field | Purpose |
|---|---|
| `export_job_ref` | Export job or packet ref, safe for support and offboarding. |
| `row_sequence` | Stable row sequence inside the export job. |
| `time_basis` | Usage-event, service-ingest, rating-close, export-generation, or policy-effective basis plus window start/end. |
| `scope` | Scope class and opaque scope ref. Parent scopes prevent finance and support from inferring hierarchy. |
| `service_family_class` | Managed service family, such as AI gateway or remote workspace. |
| `quota_family_class` / `quota_unit_class` | What was measured and in which unit. |
| `source_class` | Authoritative service meter, cached service meter, client estimate, mixed reconciliation, unavailable gap marker, policy-suppressed marker, BYOK external reference, or admin adjustment. |
| `authority_class` | `authoritative`, `cached`, `estimated`, `unavailable`, or `policy_suppressed`. |
| `usage_quantity` | Nullable quantity plus precision and aggregation method. |
| `usage_limit_snapshot` | Nullable limit snapshot and reset window at export time. |
| `spend_attribution` | Attribution dimensions used for chargeback, support, and policy review. |
| `caveats` | Typed caveats. Non-authoritative rows must carry at least one caveat. |
| `export_visibility` | Redaction class, customer visibility, support-export safety, and suppressed fields. |
| `offboarding_posture` | Whether this row is included in offboarding and whether it remains available after seat end. |

Missing data is represented by explicit rows. A bounded export that has
a meter gap emits an `unavailable_gap_marker` row with nullable
quantity and a caveat; it does not silently omit the period. A policy-
suppressed row emits `policy_suppressed_marker` and names suppressed
fields; it does not render zero, blank, or "not applicable" as if no
usage existed.

## UI And Support Rules

### AI surfaces

- Pre-invocation chips must distinguish authoritative, cached,
  estimated, unavailable, and policy-suppressed quota state.
- Cached or estimated state may allow low-risk local/BYOK/manual
  routes only if the route's budget policy admits that downgrade.
- Vendor-hosted or managed AI routes that cannot bound spend safely
  must deny with a typed reason and show the configured fallback
  route. They must not retry silently.
- Long-running agent dispatch must display estimate versus
  authoritative status before dispatch and record the state ref in
  the route-selection disclosure.

### Managed-service dashboards

- Dashboard summary tiles may aggregate rows only when unit, scope,
  time basis, and authority class are compatible.
- A cached tile must show the stale label and last refresh. A stale
  aggregate may not be merged into an authoritative aggregate without
  a mixed-source caveat.
- Policy-suppressed fields must show the suppression class and policy
  ref rather than empty cells.

### Support packets

- Support packets quote `quota_state_id` or `usage_export_row_id`,
  not free-text copies of amounts.
- Support copy must say whether the row is authoritative, cached,
  estimated, unavailable, or policy-suppressed.
- Stale or missing service data narrows the support claim. Support may
  say "current service meter unavailable" or "local estimate pending
  reconciliation"; it may not claim the customer was definitively
  under or over quota from an estimate alone.

### Offboarding and export paths

- A managed customer must be able to obtain the promised usage export
  during the access-end window without a support ticket.
- Exports remain open-format and scriptable. CSV exports must ship
  with a schema or manifest that maps each column to the row fields.
- Offboarding packets include usage export rows, entitlement snapshots,
  policy/entitlement snapshot refs, and caveats for held, suppressed,
  unavailable, or local-only data.
- After access ends, local-core workflows and already exported local
  files remain user-controlled. Managed aggregate retention follows the
  record-class and retention-row policies.

## Downgrade Behavior

When service usage data is missing or stale:

| Consumer | Cached within floor | Cached beyond floor | Local estimate | Service unavailable | Policy suppressed |
|---|---|---|---|---|---|
| AI surface | Allow only routes whose budget policy admits cached state; show cached chip. | Narrow or deny managed route requiring bounded spend; show stale chip. | Allow only estimate-admitted routes; show estimate chip. | Deny managed route requiring bounded spend; offer local/BYOK/manual fallback where allowed. | Hide suppressed fields and deny or narrow according to policy. |
| Dashboard | Show cached aggregate with last refresh. | Move tile to stale section and stop precision claims. | Show estimate section only, excluded from authoritative totals. | Show unavailable marker or gap row. | Show suppressed marker and policy ref. |
| Support packet | Quote cached state and freshness floor. | Quote stale state and caveat. | Quote estimate with reconciliation pending caveat. | Quote unavailable gap; no precision claim. | Quote suppression and policy ref; no amount. |
| Usage export | Emit `service_meter_cached` row with caveat. | Emit cached row with stale caveat or unavailable gap if outside export confidence. | Emit `client_local_estimate` row with caveat. | Emit `unavailable_gap_marker` row. | Emit `policy_suppressed_marker` row with suppressed fields. |

## Forbidden Patterns

The following are non-conforming:

- displaying `0`, blank, "free", or "unlimited" when state is
  unavailable or policy-suppressed;
- treating a local estimate as invoice truth, final support truth, or
  authoritative quota truth;
- hiding stale-meter state behind a spinner or generic "try again";
- blocking local edit/save/search/Git/tasks because metering,
  rating, export, or billing services are unreachable;
- exporting usage without scope, time basis, source class, authority
  class, unit, and caveats;
- merging cached, estimated, unavailable, or policy-suppressed rows
  into authoritative totals without preserving row-level caveats;
- using raw prompts, code payloads, user emails, tenant names, billing
  ids, invoice ids, URLs, or provider account identifiers in metering
  payloads.

## Evolution Rules

- Adding a new authority class, source class, quota family, unit,
  reset-window class, caveat class, or spend-attribution dimension is
  additive-minor and requires updating both schemas and at least one
  fixture.
- Repurposing an existing class is breaking and requires a new
  governance decision row plus a migration note for support/export
  consumers.
- Any new usage or quota surface must cite this contract, one schema,
  and at least one fixture case before it may claim customer-visible
  quota or export behavior.
- Any schema that feeds customer-visible usage export must remain
  scriptable, partiality-aware, and independent from invoice rendering.
